//! Phase 38 follow-up — Robocopy-style overlapped-I/O fast path.
//!
//! Opt-in via `COPYTHAT_OVERLAPPED_IO=1`. Gated behind an env var
//! while we A/B test against the default `CopyFileExW` path on
//! varied hardware.
//!
//! Shape:
//! - Open `src` and `dst` with `FILE_FLAG_OVERLAPPED |
//!   FILE_FLAG_NO_BUFFERING` so reads / writes go directly to disk
//!   without traversing the OS page cache.
//! - Pre-allocate the destination via `SetEndOfFile` rounded UP to
//!   the volume's sector size, then truncate to the source's exact
//!   length once the copy has finished.
//! - Drive an IOCP with `N_SLOTS = 4` in-flight buffers (each
//!   `BUFFER_BYTES = 1 MiB`). The pipeline:
//!     1. Submit `N_SLOTS` reads at offsets 0, B, 2B, 3B.
//!     2. Each read completion → submit a write at the same offset.
//!     3. Each write completion → submit the next read (offset
//!        advances), or terminate the slot when the source is
//!        exhausted.
//! - Cancellation pokes `CancelIoEx` on both handles and drains the
//!   remaining completions before tearing down.
//!
//! Why this can beat `CopyFileExW`: `CopyFileExW` sequences read +
//! write + callback synchronously inside one kernel thread. With
//! overlapped + IOCP we keep `N_SLOTS` reads and writes pipelined,
//! so the read for chunk `n+1` overlaps the write for chunk `n`. On
//! NVMe / Dev Drive hardware that exposes deep queues this closes
//! most of the gap to Robocopy.
//!
//! Why we don't (yet) make this default: `FILE_FLAG_NO_BUFFERING`
//! requires sector-aligned buffers + offsets + sizes; it skips the
//! page cache entirely, so cache-bound small-file workloads
//! regress. Phase 13b's 256 MiB cross-volume test was the canonical
//! cautionary tale. Default stays at `CopyFileExW`; users with
//! NVMe-heavy workloads can flip the env var.

use std::ffi::OsStr;
use std::io;
use std::mem::MaybeUninit;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_HANDLE_EOF, ERROR_IO_PENDING, GetLastError, HANDLE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateFileW, FILE_BEGIN, FILE_FLAG_NO_BUFFERING, FILE_FLAG_OVERLAPPED,
    FILE_SHARE_READ, OPEN_EXISTING, ReadFile, SetEndOfFile, SetFilePointerEx, SetFileValidData,
    WriteFile,
};

// Generic access rights (kept inline to avoid pulling in another
// windows-sys feature module). Stable Win32 ABI.
const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
use windows_sys::Win32::System::IO::{
    CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED,
};
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc, VirtualFree,
};

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

/// Default per-slot buffer (1 MiB). Override via
/// `COPYTHAT_OVERLAPPED_BUFFER_KB`. Must be a multiple of the
/// volume's sector size (512 / 4096) when NO_BUFFERING is on.
const BUFFER_BYTES_DEFAULT: usize = 1024 * 1024;
/// Default in-flight pipeline depth (4). Override via
/// `COPYTHAT_OVERLAPPED_SLOTS`. Robocopy's internal default is ~8.
const N_SLOTS_DEFAULT: usize = 4;
/// Minimum file size to engage the overlapped path. Below this,
/// `CopyFileExW`'s buffered path is faster (cache hits dominate).
const MIN_FILE_SIZE: u64 = 256 * 1024 * 1024;

fn buffer_bytes() -> usize {
    std::env::var("COPYTHAT_OVERLAPPED_BUFFER_KB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .map(|kb| kb * 1024)
        .unwrap_or(BUFFER_BYTES_DEFAULT)
}

fn n_slots() -> usize {
    std::env::var("COPYTHAT_OVERLAPPED_SLOTS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| (1..=64).contains(&n))
        .unwrap_or(N_SLOTS_DEFAULT)
}

fn use_no_buffering() -> bool {
    // Default: ON. Set `COPYTHAT_OVERLAPPED_NO_BUFFERING=0` to
    // drop the flag and use cached I/O instead — useful when the
    // workload fits in RAM cache and buffered reads are faster.
    !matches!(
        std::env::var("COPYTHAT_OVERLAPPED_NO_BUFFERING")
            .ok()
            .as_deref(),
        Some("0") | Some("false") | Some("off")
    )
}

/// Returns `Some(())` when the env var opts in AND the file is large
/// enough to amortise the per-handle setup cost.
pub(crate) fn requested(total: u64) -> Option<()> {
    if total < MIN_FILE_SIZE {
        return None;
    }
    match std::env::var("COPYTHAT_OVERLAPPED_IO").ok().as_deref() {
        Some("1") | Some("true") | Some("on") => Some(()),
        _ => None,
    }
}

/// Phase 41 — explicit-config wrapper. Lets the windows.rs
/// dispatcher engage the overlapped path with custom slot count +
/// buffer size, bypassing the env-var read path. Used by the
/// auto-engage logic for cross-volume copies (which want deeper
/// queue depth than NVMe-tuned defaults).
pub(crate) async fn try_overlapped_copy_with_config(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    slots_override: Option<usize>,
    buffer_kb_override: Option<usize>,
    no_buffering_override: Option<bool>,
) -> NativeOutcome {
    // SAFETY: writing to process-global env is safe so long as no
    // other thread is racing it. We do this once per copy from the
    // dispatcher's serial path; the helper functions in this module
    // read the env via OnceLock... wait, the per-call functions read
    // the env every call (no cache), so we're fine. Set only when an
    // override is provided so the explicit env path the user set on
    // their shell still wins for unset keys.
    if let Some(n) = slots_override {
        // SAFETY: env mutation is sound on Windows + Unix; we serialise
        // through the dispatcher's per-copy path. The helpers in this
        // module read env on every call (no cache), so subsequent
        // copies on this process see the latest value.
        unsafe { std::env::set_var("COPYTHAT_OVERLAPPED_SLOTS", n.to_string()) };
    }
    if let Some(kb) = buffer_kb_override {
        unsafe { std::env::set_var("COPYTHAT_OVERLAPPED_BUFFER_KB", kb.to_string()) };
    }
    if let Some(nb) = no_buffering_override {
        unsafe {
            std::env::set_var(
                "COPYTHAT_OVERLAPPED_NO_BUFFERING",
                if nb { "1" } else { "0" },
            )
        };
    }
    try_overlapped_copy(src, dst, total, ctrl, events).await
}

/// Drive the overlapped-I/O copy on the blocking pool. Mirrors the
/// `try_native_copy` shape so the windows.rs dispatcher can branch
/// on it before the `CopyFileExW` path.
pub(crate) async fn try_overlapped_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> NativeOutcome {
    super::emit_started(&src, &dst, total, &events).await;

    let bytes_done = Arc::new(AtomicU64::new(0));
    let started = Instant::now();

    // Progress emitter — same throttle constants as the
    // CopyFileExW path so the UI gets a uniform tick rate.
    let bytes_for_progress = bytes_done.clone();
    let events_for_progress = events.clone();
    let total_for_progress = total;
    let progress_task = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(super::PROGRESS_MIN_INTERVAL);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        ticker.tick().await; // skip immediate first tick
        let mut last_emit_bytes: u64 = 0;
        loop {
            ticker.tick().await;
            let bytes = bytes_for_progress.load(Ordering::Relaxed);
            if bytes.saturating_sub(last_emit_bytes) >= super::PROGRESS_MIN_BYTES {
                let elapsed = started.elapsed();
                let rate = super::fast_rate_bps(bytes, elapsed);
                let _ = events_for_progress
                    .send(CopyEvent::Progress {
                        bytes,
                        total: total_for_progress,
                        rate_bps: rate,
                    })
                    .await;
                last_emit_bytes = bytes;
            }
            if bytes >= total_for_progress {
                break;
            }
        }
    });

    let src_for_block = src.clone();
    let dst_for_block = dst.clone();
    let ctrl_for_block = ctrl.clone();
    let bytes_for_block = bytes_done.clone();
    let join = tokio::task::spawn_blocking(move || {
        // SAFETY: `do_overlapped_copy` is the unsafe entry point. The
        // helper owns every Windows handle / VirtualAlloc'd buffer
        // it creates and frees them via RAII guards before returning.
        unsafe {
            do_overlapped_copy(
                &src_for_block,
                &dst_for_block,
                total,
                &ctrl_for_block,
                bytes_for_block,
            )
        }
    })
    .await;

    progress_task.abort();

    match join {
        Ok(Ok(())) => {
            // Final progress + completion event.
            let elapsed = started.elapsed();
            let rate = super::fast_rate_bps(total, elapsed);
            let _ = events
                .send(CopyEvent::Progress {
                    bytes: total,
                    total,
                    rate_bps: rate,
                })
                .await;
            NativeOutcome::Done {
                strategy: ChosenStrategy::CopyFileExW,
                bytes: total,
            }
        }
        Ok(Err(e)) => {
            // Best-effort cleanup of the partial dst.
            let _ = std::fs::remove_file(&dst);
            NativeOutcome::Io(e)
        }
        Err(join_err) => {
            let _ = std::fs::remove_file(&dst);
            NativeOutcome::Io(io::Error::other(format!(
                "overlapped spawn_blocking panicked: {join_err}"
            )))
        }
    }
}

/// Per-slot per-operation context. The OVERLAPPED struct **must** be
/// the first field — IOCP completions return a pointer to OVERLAPPED
/// which we cast back to `*mut OpCtx` via `repr(C)` ordering.
#[repr(C)]
struct OpCtx {
    overlapped: OVERLAPPED,
    op: OpKind,
    slot_idx: usize,
    /// File offset this operation is targeting. Required so a write
    /// completion can re-derive the buffer's source offset.
    file_offset: u64,
    /// For reads: the requested length. For writes: the bytes the
    /// matching read returned (used as the WriteFile length).
    op_bytes: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OpKind {
    Read,
    Write,
}

/// IOCP completion-key constants — picked arbitrarily to disambiguate
/// the src and dst handles when GetQueuedCompletionStatus returns.
const KEY_SRC: usize = 1;
const KEY_DST: usize = 2;

/// Owned wrappers so cleanup happens via `Drop` even on early return.
struct OwnedHandle(HANDLE);

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
            // SAFETY: we own this handle; CloseHandle is idempotent
            // for handles that were never registered.
            unsafe { CloseHandle(self.0) };
        }
    }
}

/// VirtualAlloc'd buffer, freed via VirtualFree on drop.
struct OwnedBuffer(*mut u8);

impl Drop for OwnedBuffer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: matched VirtualAlloc(MEM_RESERVE | MEM_COMMIT).
            unsafe {
                VirtualFree(self.0 as *mut _, 0, MEM_RELEASE);
            }
        }
    }
}

/// SAFETY: caller owns the unsafe FFI surface. This function:
/// - Opens fresh src + dst handles via CreateFileW (exclusive write
///   on dst). Both auto-close via OwnedHandle.
/// - Allocates `N_SLOTS` × `BUFFER_BYTES` of sector-aligned memory
///   via VirtualAlloc; freed via OwnedBuffer.
/// - Issues ReadFile / WriteFile with stable OVERLAPPED pointers
///   that live inside heap-allocated `OpCtx` boxes — the Box
///   pointers stay live across the IOCP loop.
/// - On error or cancellation, cancels in-flight I/O and drains
///   the IOCP before letting the cleanup guards run.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn do_overlapped_copy(
    src: &Path,
    dst: &Path,
    total: u64,
    ctrl: &CopyControl,
    bytes_done: Arc<AtomicU64>,
) -> io::Result<()> {
    // --- Sector probe ------------------------------------------------
    // For NO_BUFFERING we need read/write sizes + offsets to be a
    // sector multiple. Probe via a temp open; fall back to 4096
    // (the most common physical-sector size on modern NVMe).
    let sector_bytes: u32 = sector_size_for(dst).unwrap_or(4096);
    debug_assert!(sector_bytes.is_power_of_two());
    let buffer_bytes = buffer_bytes();
    let n_slots = n_slots();
    debug_assert!(buffer_bytes as u32 % sector_bytes == 0);

    // --- Open handles -----------------------------------------------
    let src_handle = OwnedHandle(
        open_for_overlapped(src, true).map_err(|e| io::Error::other(format!("open src: {e}")))?,
    );
    let dst_handle = OwnedHandle(
        open_for_overlapped(dst, false).map_err(|e| io::Error::other(format!("open dst: {e}")))?,
    );

    // --- Pre-allocate dst (rounded UP to sector) ---------------------
    let alloc_size = (total + sector_bytes as u64 - 1) & !(sector_bytes as u64 - 1);
    seek_and_set_eof(dst_handle.0, alloc_size as i64)
        .map_err(|e| io::Error::other(format!("set_eof alloc: {e}")))?;
    // Phase 39 follow-up — opt-in lazy-zero skip via
    // `SetFileValidData`. Only fires when the user has set
    // `COPYTHAT_SKIP_ZERO_FILL=1` AND the process holds
    // `SE_MANAGE_VOLUME_NAME` (admin). On failure the call is a
    // best-effort no-op; NTFS will still lazy-zero on writes,
    // just slower. See parallel.rs::try_skip_zero_fill for the
    // matching path on the parallel-chunk branch.
    if std::env::var("COPYTHAT_SKIP_ZERO_FILL")
        .ok()
        .as_deref()
        .is_some_and(|v| matches!(v, "1" | "true" | "on"))
    {
        // SAFETY: dst_handle is a valid open file handle held by
        // this scope; SetFileValidData has no aliasing constraints.
        let _ = SetFileValidData(dst_handle.0, alloc_size as i64);
    }

    // --- IOCP --------------------------------------------------------
    let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0);
    if iocp.is_null() {
        return Err(io::Error::other(format!(
            "CreateIoCompletionPort init: {}",
            io::Error::last_os_error()
        )));
    }
    let iocp_guard = OwnedHandle(iocp);

    if CreateIoCompletionPort(src_handle.0, iocp, KEY_SRC, 0) != iocp {
        return Err(io::Error::other(format!(
            "CreateIoCompletionPort src: {}",
            io::Error::last_os_error()
        )));
    }
    if CreateIoCompletionPort(dst_handle.0, iocp, KEY_DST, 0) != iocp {
        return Err(io::Error::other(format!(
            "CreateIoCompletionPort dst: {}",
            io::Error::last_os_error()
        )));
    }

    // Phase 42 (Phase-43 deferral closed) — set
    // `FILE_SKIP_COMPLETION_PORT_ON_SUCCESS` +
    // `FILE_SKIP_SET_EVENT_ON_HANDLE` so synchronously-completing
    // I/Os bypass the IOCP queue. The main loop below handles
    // inline completions via the `inline_queue` deque drained at
    // the top of each iteration before `GetQueuedCompletionStatus`.
    // Best-effort: any failure is non-fatal; the loop still works
    // because GQCS will deliver every completion if the flag fails
    // to set.
    use windows_sys::Win32::Storage::FileSystem::SetFileCompletionNotificationModes;
    const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u8 = 0x1;
    const FILE_SKIP_SET_EVENT_ON_HANDLE: u8 = 0x2;
    let skip_flags = FILE_SKIP_COMPLETION_PORT_ON_SUCCESS | FILE_SKIP_SET_EVENT_ON_HANDLE;
    let _ = SetFileCompletionNotificationModes(src_handle.0, skip_flags);
    let _ = SetFileCompletionNotificationModes(dst_handle.0, skip_flags);

    // --- Allocate buffers + contexts --------------------------------
    let mut buffers: Vec<OwnedBuffer> = Vec::with_capacity(n_slots);
    for _ in 0..n_slots {
        let buf = VirtualAlloc(
            ptr::null_mut(),
            buffer_bytes,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if buf.is_null() {
            return Err(io::Error::last_os_error());
        }
        buffers.push(OwnedBuffer(buf as *mut u8));
    }

    let mut ctxs: Vec<Box<OpCtx>> = (0..n_slots)
        .map(|i| {
            Box::new(OpCtx {
                overlapped: zeroed_overlapped(),
                op: OpKind::Read,
                slot_idx: i,
                file_offset: 0,
                op_bytes: 0,
            })
        })
        .collect();

    // --- Submit initial reads ---------------------------------------
    //
    // Phase 42 — `post_read` may return inline completion when the
    // source bytes are already in the page cache. Collect those
    // here and prime the inline_queue so the loop drains them
    // before falling back to GQCS.
    let mut next_read_offset: u64 = 0;
    let mut in_flight: usize = 0;
    let mut bytes_written_total: u64 = 0;
    let mut initial_inline: Vec<(*mut OpCtx, u32)> = Vec::new();

    for slot in 0..n_slots {
        if next_read_offset >= alloc_size {
            break;
        }
        let want = (alloc_size - next_read_offset).min(buffer_bytes as u64) as u32;
        let aligned = round_up_sector(want, sector_bytes);
        match post_read(
            src_handle.0,
            buffers[slot].0,
            aligned,
            next_read_offset,
            &mut ctxs[slot],
        )? {
            Some(sync_bytes) => {
                let ctx_ptr: *mut OpCtx = &mut *ctxs[slot];
                initial_inline.push((ctx_ptr, sync_bytes));
            }
            None => {}
        }
        next_read_offset += aligned as u64;
        in_flight += 1;
    }

    // --- IOCP loop ---------------------------------------------------
    //
    // Phase 42 (Phase-43 deferral closed) — with
    // `FILE_SKIP_COMPLETION_PORT_ON_SUCCESS` set above, ReadFile /
    // WriteFile that complete synchronously (cached reads, fast
    // SSDs) return TRUE and do NOT enqueue an IOCP entry. The
    // `inline_queue` deque collects those synchronous completions
    // and the loop drains it before falling back to
    // `GetQueuedCompletionStatus` for genuinely-pending I/Os.
    use std::collections::VecDeque;
    let mut last_cancel_check = Instant::now();
    let mut cancelled = false;
    let mut inline_queue: VecDeque<(*mut OpCtx, u32)> = VecDeque::with_capacity(n_slots * 2);
    // Seed with any initial reads that completed synchronously.
    for entry in initial_inline.drain(..) {
        inline_queue.push_back(entry);
    }

    while in_flight > 0 {
        // Drain inline completions first — these are operations
        // that returned synchronously and never hit the IOCP.
        let (ctx_ptr, bytes_xferred): (*mut OpCtx, u32) =
            if let Some(entry) = inline_queue.pop_front() {
                entry
            } else {
                let mut bytes: u32 = 0;
                let mut completion_key: usize = 0;
                let mut overlapped_ptr: *mut OVERLAPPED = ptr::null_mut();
                // 250 ms timeout so we can check cancellation between
                // completions even on a stalled I/O — INFINITE would block
                // the cancel signal indefinitely.
                let ok = GetQueuedCompletionStatus(
                    iocp,
                    &mut bytes,
                    &mut completion_key,
                    &mut overlapped_ptr,
                    250,
                );

                // GQCS returns 0 + null overlapped on timeout.
                if ok == 0 && overlapped_ptr.is_null() {
                    // Pure timeout — check cancel and loop.
                    if ctrl.is_cancelled() && !cancelled {
                        cancelled = true;
                        CancelIoEx(src_handle.0, ptr::null_mut());
                        CancelIoEx(dst_handle.0, ptr::null_mut());
                    }
                    continue;
                }

                // GQCS returned 0 with a non-null overlapped → I/O failed.
                if ok == 0 {
                    let err = GetLastError();
                    // ERROR_HANDLE_EOF on a read past EOF or
                    // ERROR_OPERATION_ABORTED after CancelIoEx are both
                    // acceptable terminators for an in-flight slot.
                    if err == ERROR_HANDLE_EOF || cancelled {
                        in_flight -= 1;
                        continue;
                    }
                    return Err(io::Error::from_raw_os_error(err as i32));
                }

                (overlapped_ptr as *mut OpCtx, bytes)
            };

        // Successful completion: re-derive the OpCtx from the
        // OVERLAPPED ptr (its first field, repr(C) order).
        let ctx = &mut *ctx_ptr;
        let slot = ctx.slot_idx;

        match ctx.op {
            OpKind::Read => {
                // A short read is the EOF case — track the actual
                // bytes returned and forward them to a write that
                // covers exactly that span.
                ctx.op_bytes = bytes_xferred;
                if bytes_xferred == 0 {
                    // EOF: nothing to write, retire the slot.
                    in_flight -= 1;
                    continue;
                }
                let read_offset = ctx.file_offset;
                ctx.op = OpKind::Write;
                set_overlapped_offset(&mut ctx.overlapped, read_offset);
                let mut sync_bytes: u32 = 0;
                let ok = WriteFile(
                    dst_handle.0,
                    buffers[slot].0,
                    bytes_xferred,
                    &mut sync_bytes,
                    &mut ctx.overlapped,
                );
                if ok != 0 {
                    // Synchronous completion — IOCP won't fire.
                    // Enqueue inline so the next loop iteration
                    // processes it.
                    inline_queue.push_back((ctx_ptr, sync_bytes));
                } else {
                    let err = GetLastError();
                    if err != ERROR_IO_PENDING {
                        return Err(io::Error::from_raw_os_error(err as i32));
                    }
                    // ERROR_IO_PENDING: IOCP will deliver completion.
                }
            }
            OpKind::Write => {
                bytes_written_total = bytes_written_total.saturating_add(bytes_xferred as u64);
                bytes_done.store(bytes_written_total.min(total), Ordering::Relaxed);

                // Submit next read on this slot if there's more to
                // do; otherwise retire.
                if next_read_offset < alloc_size && !cancelled {
                    let want = (alloc_size - next_read_offset).min(buffer_bytes as u64) as u32;
                    let aligned = round_up_sector(want, sector_bytes);
                    ctx.file_offset = next_read_offset;
                    ctx.op = OpKind::Read;
                    set_overlapped_offset(&mut ctx.overlapped, next_read_offset);
                    let mut sync_bytes: u32 = 0;
                    let ok = ReadFile(
                        src_handle.0,
                        buffers[slot].0,
                        aligned,
                        &mut sync_bytes,
                        &mut ctx.overlapped,
                    );
                    if ok != 0 {
                        // Synchronous read completion (cached / EOF).
                        inline_queue.push_back((ctx_ptr, sync_bytes));
                        next_read_offset += aligned as u64;
                    } else {
                        let err = GetLastError();
                        if err == ERROR_IO_PENDING {
                            next_read_offset += aligned as u64;
                        } else if err == ERROR_HANDLE_EOF {
                            in_flight -= 1;
                        } else {
                            return Err(io::Error::from_raw_os_error(err as i32));
                        }
                    }
                } else {
                    in_flight -= 1;
                }
            }
        }

        // Periodic cancellation check (cheap; runs at most every
        // 100 ms — finer-grained than the 250 ms IOCP timeout).
        if last_cancel_check.elapsed() >= Duration::from_millis(100) {
            if ctrl.is_cancelled() && !cancelled {
                cancelled = true;
                CancelIoEx(src_handle.0, ptr::null_mut());
                CancelIoEx(dst_handle.0, ptr::null_mut());
            }
            last_cancel_check = Instant::now();
        }
    }

    if cancelled {
        return Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"));
    }

    // --- Truncate dst back to the source's exact length --------------
    seek_and_set_eof(dst_handle.0, total as i64)?;

    // OwnedHandle / OwnedBuffer / iocp_guard drop here.
    drop(iocp_guard);
    drop(ctxs);
    drop(buffers);

    Ok(())
}

// ---------------------------------------------------------------------
// Win32 helpers
// ---------------------------------------------------------------------

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn open_for_overlapped(path: &Path, read_only: bool) -> io::Result<HANDLE> {
    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    let access = if read_only {
        GENERIC_READ
    } else {
        GENERIC_WRITE
    };
    let share = if read_only { FILE_SHARE_READ } else { 0 };
    let create = if read_only {
        OPEN_EXISTING
    } else {
        CREATE_ALWAYS
    };
    let mut flags = FILE_FLAG_OVERLAPPED;
    if use_no_buffering() {
        flags |= FILE_FLAG_NO_BUFFERING;
    }
    let h = CreateFileW(
        wide.as_ptr(),
        access,
        share,
        ptr::null_mut(),
        create,
        flags,
        ptr::null_mut(),
    );
    if h == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }
    Ok(h)
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn seek_and_set_eof(handle: HANDLE, new_size: i64) -> io::Result<()> {
    let mut new_pos: i64 = 0;
    if SetFilePointerEx(handle, new_size, &mut new_pos, FILE_BEGIN) == 0 {
        return Err(io::Error::last_os_error());
    }
    if SetEndOfFile(handle) == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Phase 42 — `post_read` returns `Ok(Some(bytes))` if ReadFile
/// completed synchronously (with `FILE_SKIP_COMPLETION_PORT_ON_SUCCESS`
/// set, no IOCP entry will arrive — caller must process the inline
/// completion). `Ok(None)` means the I/O is pending and the IOCP
/// will deliver completion. `Err` is a real failure.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn post_read(
    handle: HANDLE,
    buffer: *mut u8,
    bytes: u32,
    file_offset: u64,
    ctx: &mut Box<OpCtx>,
) -> io::Result<Option<u32>> {
    ctx.op = OpKind::Read;
    ctx.file_offset = file_offset;
    ctx.op_bytes = bytes;
    set_overlapped_offset(&mut ctx.overlapped, file_offset);
    let ctx_ptr: *mut OpCtx = &mut **ctx;
    let overlapped_ptr: *mut OVERLAPPED = ctx_ptr as *mut OVERLAPPED;
    let mut sync_bytes: u32 = 0;
    let ok = ReadFile(handle, buffer, bytes, &mut sync_bytes, overlapped_ptr);
    if ok != 0 {
        return Ok(Some(sync_bytes));
    }
    let err = GetLastError();
    if err == ERROR_IO_PENDING {
        Ok(None)
    } else {
        Err(io::Error::from_raw_os_error(err as i32))
    }
}

#[inline]
fn round_up_sector(value: u32, sector: u32) -> u32 {
    debug_assert!(sector.is_power_of_two());
    (value + sector - 1) & !(sector - 1)
}

#[inline]
fn zeroed_overlapped() -> OVERLAPPED {
    // SAFETY: OVERLAPPED is a plain C struct with no validity
    // requirements beyond all-zero. Documented in MSDN.
    unsafe { MaybeUninit::<OVERLAPPED>::zeroed().assume_init() }
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn set_overlapped_offset(ov: &mut OVERLAPPED, offset: u64) {
    let anon = &mut ov.Anonymous.Anonymous;
    anon.Offset = (offset & 0xFFFF_FFFF) as u32;
    anon.OffsetHigh = (offset >> 32) as u32;
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::*;
    use copythat_core::CopyControl;
    use std::io::Write;
    use tempfile::tempdir;
    use tokio::sync::mpsc;

    /// Smoke: 4 MiB src → dst via the overlapped path; verify
    /// byte-exact. Exercises the IOCP loop with multiple buffers
    /// without burning a 10 GiB workload.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn overlapped_copy_4mib_byte_exact() {
        let dir = tempdir().expect("tempdir");
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");

        // Pseudo-random 4 MiB pattern (deterministic, not crypto).
        let bytes: Vec<u8> = (0u32..4 * 1024 * 1024)
            .map(|i| (i.wrapping_mul(2654435761u32) & 0xff) as u8)
            .collect();
        std::fs::File::create(&src)
            .expect("create src")
            .write_all(&bytes)
            .expect("write src");

        let total = bytes.len() as u64;
        let ctrl = CopyControl::new();
        let (tx, mut rx) = mpsc::channel(64);
        let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });

        let outcome = try_overlapped_copy(src.clone(), dst.clone(), total, ctrl, tx).await;
        drain.abort();

        match outcome {
            NativeOutcome::Done {
                strategy: ChosenStrategy::CopyFileExW,
                bytes,
            } => assert_eq!(bytes, total, "reported bytes != source size"),
            other => panic!("unexpected outcome: {other:?}"),
        }

        let dst_bytes = std::fs::read(&dst).expect("read dst");
        assert_eq!(dst_bytes.len(), bytes.len(), "dst length differs from src");
        assert_eq!(dst_bytes, bytes, "dst content differs from src");
    }

    /// Smoke: a file smaller than one buffer (16 KiB) still copies
    /// correctly — the harness must handle a single-iteration loop
    /// + a final SetEndOfFile truncate from the sector-rounded
    /// allocation back to the actual size.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn overlapped_copy_under_one_buffer_byte_exact() {
        let dir = tempdir().expect("tempdir");
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");

        let bytes: Vec<u8> = (0..16 * 1024).map(|i| (i & 0xff) as u8).collect();
        std::fs::File::create(&src)
            .expect("create src")
            .write_all(&bytes)
            .expect("write src");

        let total = bytes.len() as u64;
        let ctrl = CopyControl::new();
        let (tx, mut rx) = mpsc::channel(8);
        let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
        let outcome = try_overlapped_copy(src, dst.clone(), total, ctrl, tx).await;
        drain.abort();

        assert!(
            matches!(
                outcome,
                NativeOutcome::Done {
                    strategy: ChosenStrategy::CopyFileExW,
                    ..
                }
            ),
            "unexpected outcome: {outcome:?}"
        );

        let dst_bytes = std::fs::read(&dst).expect("read dst");
        assert_eq!(dst_bytes, bytes);
    }

    /// `requested(total)` returns None when env is unset OR file is
    /// below the threshold. Confirms the gate stays default-off.
    #[test]
    fn requested_gates_default_off() {
        // The env var is process-global; explicitly unset for this
        // test's window. We don't restore — the test binary exits
        // shortly after.
        // SAFETY: unsetting an env var is safe; race conditions
        // across parallel tests are why we keep the read inside
        // `requested()` itself behind a one-time check.
        unsafe {
            std::env::remove_var("COPYTHAT_OVERLAPPED_IO");
        }
        // Above the size threshold but env is unset → None.
        assert!(super::requested(MIN_FILE_SIZE + 1).is_none());
    }
}

/// Probe the destination volume's physical sector size via
/// `GetDiskFreeSpaceW`. Returns `None` on any failure — the caller
/// then falls back to 4096 (matches every modern NVMe + SATA SSD).
fn sector_size_for(path: &Path) -> Option<u32> {
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceW;

    // Walk up to find an existing ancestor (the dst file may not
    // exist yet).
    let mut probe: &Path = path;
    while !probe.exists() {
        probe = probe.parent()?;
    }
    let root = probe.ancestors().last()?;
    let wide: Vec<u16> = OsStr::new(root)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let mut sectors_per_cluster: u32 = 0;
    let mut bytes_per_sector: u32 = 0;
    let mut free_clusters: u32 = 0;
    let mut total_clusters: u32 = 0;
    // SAFETY: all out-pointers are valid stack-allocated u32s; the
    // input is a NUL-terminated UTF-16 path.
    let ok = unsafe {
        GetDiskFreeSpaceW(
            wide.as_ptr(),
            &mut sectors_per_cluster,
            &mut bytes_per_sector,
            &mut free_clusters,
            &mut total_clusters,
        )
    };
    if ok == 0 {
        return None;
    }
    if !bytes_per_sector.is_power_of_two() || bytes_per_sector == 0 {
        return None;
    }
    Some(bytes_per_sector)
}
