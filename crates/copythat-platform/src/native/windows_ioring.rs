//! Phase 47 — Windows 11 IoRing fast path.
//!
//! IoRing is the Win 11 22000+ analogue of Linux's io_uring. Compared
//! to the existing IOCP overlapped path in `windows_overlapped.rs` it:
//!
//! - Submits batches of read/write SQEs through a shared submission
//!   ring instead of one syscall per `ReadFile`/`WriteFile`. On
//!   high-op-rate workloads (mixed-tree, USB, SMB) this saves real
//!   syscall overhead.
//! - Pre-registers buffers (one-time DMA mapping) and file handles
//!   (one-time handle lookup), which the IOCP path does per-op.
//! - Drives completions through a shared completion ring polled with
//!   `PopIoRingCompletion` — no per-op `OVERLAPPED` allocation.
//!
//! This path is **opt-in** (`COPYTHAT_IORING_IO=1`) until A/B benches
//! confirm it wins on production hardware. Promotion to default
//! is gated on the same kind of head-to-head bench gate the IOCP
//! path went through (Phase 41 → Phase 46).
//!
//! Compatibility: gracefully no-ops on:
//! - Windows 10 / Server 2019 (no IoRing kernel support).
//! - Win 11 22000 builds where `api-ms-win-core-ioring-l1-1-0.dll` is
//!   unavailable (older 22H2 SKUs / Insider channels).
//! - Hosts where `CreateIoRing` succeeds but the requested ops fail
//!   `IsIoRingOpSupported`.
//!
//! In every fallback case the dispatcher's existing routing to
//! `CopyFileExW` / `windows_overlapped` takes over with no observable
//! behaviour change other than missing the env-var-gated win.

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
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, S_OK};
use windows_sys::Win32::Storage::FileSystem::{
    BuildIoRingCancelRequest, BuildIoRingReadFile, BuildIoRingRegisterBuffers,
    BuildIoRingRegisterFileHandles, BuildIoRingWriteFile, CREATE_ALWAYS, CloseIoRing, CreateFileW,
    CreateIoRing, FILE_BEGIN, FILE_FLAG_NO_BUFFERING, FILE_FLAG_OVERLAPPED, FILE_SHARE_READ,
    FILE_WRITE_FLAGS_NONE, HIORING, IORING_BUFFER_INFO, IORING_BUFFER_REF, IORING_BUFFER_REF_0,
    IORING_CQE, IORING_CREATE_FLAGS, IORING_HANDLE_REF, IORING_HANDLE_REF_0, IORING_OP_READ,
    IORING_OP_WRITE, IORING_REF_REGISTERED, IORING_REGISTERED_BUFFER, IORING_VERSION_3,
    IOSQE_FLAGS_NONE, IsIoRingOpSupported, OPEN_EXISTING, PopIoRingCompletion,
    QueryIoRingCapabilities, SetEndOfFile, SetFilePointerEx, SetFileValidData, SubmitIoRing,
};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, LoadLibraryW};
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc, VirtualFree,
};

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

/// Default per-slot buffer (1 MiB). Override via
/// `COPYTHAT_IORING_BUFFER_KB`.
const BUFFER_BYTES_DEFAULT: usize = 1024 * 1024;
/// Default in-flight pipeline depth (8). Override via
/// `COPYTHAT_IORING_SLOTS`. IoRing's submission queue is sized to
/// `2 × slots` so each slot can have a read AND a write queued.
const N_SLOTS_DEFAULT: usize = 8;
/// Minimum file size to engage the IoRing path. Below this,
/// `CopyFileExW`'s buffered path wins (cache hits dominate).
const MIN_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// HRESULT return codes we care about. `S_FALSE` is normal: it means
/// "no completion was popped from the ring on this attempt."
const S_FALSE: i32 = 1;

fn buffer_bytes() -> usize {
    std::env::var("COPYTHAT_IORING_BUFFER_KB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .map(|kb| kb * 1024)
        .unwrap_or(BUFFER_BYTES_DEFAULT)
}

fn n_slots() -> usize {
    std::env::var("COPYTHAT_IORING_SLOTS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| (1..=64).contains(&n))
        .unwrap_or(N_SLOTS_DEFAULT)
}

/// Returns `Some(())` when the env var opts in AND the file is large
/// enough to amortise the per-handle setup cost.
pub(crate) fn requested(total: u64) -> Option<()> {
    if total < MIN_FILE_SIZE {
        return None;
    }
    match std::env::var("COPYTHAT_IORING_IO").ok().as_deref() {
        Some("1") | Some("true") | Some("on") => Some(()),
        _ => None,
    }
}

/// Probe whether `api-ms-win-core-ioring-l1-1-0.dll` is loaded /
/// loadable in this process. Cached per-process.
///
/// On Win 10 / Server 2019 the API set DLL doesn't exist, so any
/// IoRing function call would unwind via the loader and fail the
/// process. `LoadLibraryW` lets us discover the absence cheaply
/// without trying to actually call into the table.
pub(crate) fn is_available() -> bool {
    use std::sync::OnceLock;
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        let dll_name: Vec<u16> = "api-ms-win-core-ioring-l1-1-0.dll\0"
            .encode_utf16()
            .collect();
        // Try GetModuleHandle first — if something already loaded it,
        // we don't need to LoadLibrary and bump the refcount.
        // SAFETY: dll_name is a NUL-terminated UTF-16 string with
        // 'static-equivalent lifetime within this scope.
        let handle = unsafe { GetModuleHandleW(dll_name.as_ptr()) };
        if !handle.is_null() {
            return true;
        }
        // SAFETY: same as above. LoadLibraryW returns NULL on
        // failure; non-NULL is a leaked module handle which is fine
        // for a per-process probe (we want the loader to keep the
        // module pinned for subsequent calls).
        let loaded = unsafe { LoadLibraryW(dll_name.as_ptr()) };
        !loaded.is_null()
    })
}

/// Probe IoRing capabilities. Returns the best version this kernel
/// supports, or `None` if the API isn't available at all. Cached.
fn capabilities() -> Option<IoRingCaps> {
    use std::sync::OnceLock;
    static CACHED: OnceLock<Option<IoRingCaps>> = OnceLock::new();
    *CACHED.get_or_init(|| {
        if !is_available() {
            return None;
        }
        let mut caps: MaybeUninit<windows_sys::Win32::Storage::FileSystem::IORING_CAPABILITIES> =
            MaybeUninit::zeroed();
        // SAFETY: caps is a properly-sized IORING_CAPABILITIES;
        // QueryIoRingCapabilities populates it fully on success.
        let hr = unsafe { QueryIoRingCapabilities(caps.as_mut_ptr()) };
        if hr != S_OK {
            return None;
        }
        let caps = unsafe { caps.assume_init() };
        Some(IoRingCaps {
            max_version: caps.MaxVersion,
            max_sq_size: caps.MaxSubmissionQueueSize,
            max_cq_size: caps.MaxCompletionQueueSize,
        })
    })
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // max_version + max_cq_size are forward-compat surface
struct IoRingCaps {
    max_version: i32,
    max_sq_size: u32,
    max_cq_size: u32,
}

/// Drive the IoRing copy on the blocking pool. Mirrors
/// `windows_overlapped::try_overlapped_copy` so the windows.rs
/// dispatcher can branch on it before the `CopyFileExW` path.
pub(crate) async fn try_ioring_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> NativeOutcome {
    // Capability probe — if IoRing isn't available, propagate
    // `Unsupported` so the dispatcher routes to the next strategy
    // (overlapped IOCP / CopyFileExW). No event noise.
    if capabilities().is_none() {
        return NativeOutcome::Unsupported;
    }

    super::emit_started(&src, &dst, total, &events).await;

    let bytes_done = Arc::new(AtomicU64::new(0));
    let started = Instant::now();

    // Progress emitter — same throttle as the IOCP path.
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
        // SAFETY: `do_ioring_copy` is the unsafe FFI entry point.
        // It owns every kernel handle / buffer it allocates and
        // frees them via RAII guards before returning.
        unsafe {
            do_ioring_copy(
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
                "ioring spawn_blocking panicked: {join_err}"
            )))
        }
    }
}

/// Owned IoRing handle, closed via `CloseIoRing` on drop.
struct OwnedRing(HIORING);

impl Drop for OwnedRing {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: we own this ring; CloseIoRing frees the
            // submission/completion queues and any pinned resources.
            unsafe {
                let _ = CloseIoRing(self.0);
            }
        }
    }
}

struct OwnedHandle(HANDLE);

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
            // SAFETY: we own this handle; CloseHandle is idempotent.
            unsafe { CloseHandle(self.0) };
        }
    }
}

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

/// Per-slot state machine. Tracks whether the slot is currently
/// reading, writing, or has finished its last write.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SlotState {
    Idle,
    Reading { offset: u64, len: u32 },
    Writing { offset: u64, len: u32 },
    Done,
}

/// Userdata tag layout: pack slot index + op kind into a `usize` so
/// the completion handler can recover both. Slot index occupies the
/// low 32 bits; op kind occupies bit 32 (1 = write, 0 = read).
const USERDATA_OP_BIT: usize = 1 << 32;

fn pack_userdata(slot: usize, is_write: bool) -> usize {
    slot | if is_write { USERDATA_OP_BIT } else { 0 }
}

fn unpack_userdata(ud: usize) -> (usize, bool) {
    (ud & 0xFFFF_FFFF, (ud & USERDATA_OP_BIT) != 0)
}

/// SAFETY: caller owns the unsafe FFI surface. This function:
/// - Opens fresh src + dst handles via CreateFileW (exclusive write
///   on dst). Both auto-close via OwnedHandle.
/// - Allocates `N_SLOTS` × `BUFFER_BYTES` of sector-aligned memory
///   via VirtualAlloc; freed via OwnedBuffer.
/// - Creates an IoRing, registers buffers + handles, runs a
///   submit/drain loop until `total` bytes are written.
/// - On error or cancellation, attempts `BuildIoRingCancelRequest`
///   on each in-flight slot, drains the ring, and lets RAII guards
///   tear everything down.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn do_ioring_copy(
    src: &Path,
    dst: &Path,
    total: u64,
    ctrl: &CopyControl,
    bytes_done: Arc<AtomicU64>,
) -> io::Result<()> {
    let sector_bytes: u32 = super::windows_overlapped::sector_size_for(dst).unwrap_or(4096);
    debug_assert!(sector_bytes.is_power_of_two());
    let buffer_bytes = buffer_bytes();
    let n_slots = n_slots();
    debug_assert!(buffer_bytes as u32 % sector_bytes == 0);

    // --- Open handles -----------------------------------------------
    let src_handle = OwnedHandle(open_for_ioring(src, true)?);
    let dst_handle = OwnedHandle(open_for_ioring(dst, false)?);

    // --- Pre-allocate dst (rounded UP to sector) ---------------------
    let alloc_size = (total + sector_bytes as u64 - 1) & !(sector_bytes as u64 - 1);
    seek_and_set_eof(dst_handle.0, alloc_size as i64)?;
    // Phase 47 — same lazy-zero default as the IOCP path: ON when
    // running elevated (SE_MANAGE_VOLUME_NAME held). Env override
    // via `COPYTHAT_SKIP_ZERO_FILL`.
    let env_val = std::env::var("COPYTHAT_SKIP_ZERO_FILL").ok();
    let should_skip_zero = match env_val.as_deref() {
        Some("0") | Some("false") | Some("off") => false,
        Some("1") | Some("true") | Some("on") => true,
        _ => super::windows::has_manage_volume_privilege(),
    };
    if should_skip_zero {
        let _ = SetFileValidData(dst_handle.0, alloc_size as i64);
    }

    // --- Create IoRing ----------------------------------------------
    let caps = capabilities().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "IoRing capabilities probe returned None — kernel doesn't support it",
        )
    })?;
    // We need at least 2*n_slots SQ entries (one read + one write
    // per slot can be queued). CQ same size. Clamp to caps.
    let sq_size = ((n_slots * 2) as u32).min(caps.max_sq_size).max(8);
    let cq_size = sq_size; // 1:1 sizing is the common pattern
    let create_flags = IORING_CREATE_FLAGS {
        Required: 0,
        Advisory: 0,
    };
    let mut ring: HIORING = ptr::null_mut();
    let hr = CreateIoRing(IORING_VERSION_3, create_flags, sq_size, cq_size, &mut ring);
    if hr != S_OK {
        return Err(io::Error::other(format!(
            "CreateIoRing v3 failed: HRESULT 0x{hr:08X}"
        )));
    }
    let _ring_guard = OwnedRing(ring);

    // Verify the ops we need are supported. On exotic SKUs / future
    // versions, IsIoRingOpSupported is the canonical capability check.
    if IsIoRingOpSupported(ring, IORING_OP_READ) == 0
        || IsIoRingOpSupported(ring, IORING_OP_WRITE) == 0
    {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "IoRing is available but READ/WRITE ops are unsupported on this kernel",
        ));
    }

    // --- Allocate + register buffers --------------------------------
    let buffers: Vec<OwnedBuffer> = (0..n_slots)
        .map(|_| {
            // SAFETY: VirtualAlloc returns NULL on failure.
            let ptr = VirtualAlloc(
                ptr::null(),
                buffer_bytes,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            ) as *mut u8;
            OwnedBuffer(ptr)
        })
        .collect();
    if buffers.iter().any(|b| b.0.is_null()) {
        return Err(io::Error::other(format!(
            "VirtualAlloc failed: {}",
            io::Error::last_os_error()
        )));
    }
    let buf_infos: Vec<IORING_BUFFER_INFO> = buffers
        .iter()
        .map(|b| IORING_BUFFER_INFO {
            Address: b.0 as *mut _,
            Length: buffer_bytes as u32,
        })
        .collect();
    let hr = BuildIoRingRegisterBuffers(ring, n_slots as u32, buf_infos.as_ptr(), 0);
    if hr != S_OK {
        return Err(io::Error::other(format!(
            "BuildIoRingRegisterBuffers failed: HRESULT 0x{hr:08X}"
        )));
    }

    // --- Register file handles --------------------------------------
    let handles: [HANDLE; 2] = [src_handle.0, dst_handle.0];
    let hr = BuildIoRingRegisterFileHandles(ring, 2, handles.as_ptr(), 0);
    if hr != S_OK {
        return Err(io::Error::other(format!(
            "BuildIoRingRegisterFileHandles failed: HRESULT 0x{hr:08X}"
        )));
    }

    // Submit the two registration ops + wait for both to complete.
    // Each register op produces one CQE. Submit-and-wait pattern.
    let mut submitted: u32 = 0;
    let hr = SubmitIoRing(ring, 2, u32::MAX, &mut submitted);
    if hr != S_OK {
        return Err(io::Error::other(format!(
            "SubmitIoRing register failed: HRESULT 0x{hr:08X}"
        )));
    }
    drain_completions(ring, 2)?;

    // --- Pipeline: submit N initial reads ---------------------------
    let mut next_offset: u64 = 0;
    let mut bytes_remaining = alloc_size; // sector-aligned upper bound
    let mut in_flight: usize = 0;
    let mut slot_states: Vec<SlotState> = vec![SlotState::Idle; n_slots];

    let src_ref = make_handle_ref_index(0);
    let dst_ref = make_handle_ref_index(1);

    for slot in 0..n_slots {
        if bytes_remaining == 0 {
            break;
        }
        let to_read = std::cmp::min(buffer_bytes as u64, bytes_remaining) as u32;
        let buf_ref = make_buffer_ref_registered(slot as u32, 0);
        let hr = BuildIoRingReadFile(
            ring,
            src_ref,
            buf_ref,
            to_read,
            next_offset,
            pack_userdata(slot, false),
            IOSQE_FLAGS_NONE,
        );
        if hr != S_OK {
            return Err(io::Error::other(format!(
                "BuildIoRingReadFile init slot {slot}: HRESULT 0x{hr:08X}"
            )));
        }
        slot_states[slot] = SlotState::Reading {
            offset: next_offset,
            len: to_read,
        };
        next_offset += to_read as u64;
        bytes_remaining -= to_read as u64;
        in_flight += 1;
    }

    // Submit batch (don't wait — we'll drain in the loop below).
    let hr = SubmitIoRing(ring, 0, 0, &mut submitted);
    if hr != S_OK {
        return Err(io::Error::other(format!(
            "SubmitIoRing initial reads: HRESULT 0x{hr:08X}"
        )));
    }

    // --- Drain loop -------------------------------------------------
    while in_flight > 0 {
        // Cancel-pending check between completions.
        if ctrl.is_cancelled() {
            // Best-effort cancel of every in-flight slot's last op.
            for slot in 0..n_slots {
                let is_write = matches!(slot_states[slot], SlotState::Writing { .. });
                let target = if is_write { dst_ref } else { src_ref };
                if matches!(
                    slot_states[slot],
                    SlotState::Reading { .. } | SlotState::Writing { .. }
                ) {
                    let _ = BuildIoRingCancelRequest(
                        ring,
                        target,
                        pack_userdata(slot, is_write),
                        usize::MAX,
                    );
                }
            }
            let _ = SubmitIoRing(ring, in_flight as u32, u32::MAX, &mut submitted);
            // Drain whatever lands; ignore errors (cancellation
            // races are expected to surface as ERROR_OPERATION_ABORTED).
            let _ = drain_completions(ring, in_flight);
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "ioring copy cancelled",
            ));
        }
        while ctrl.is_paused() {
            std::thread::sleep(Duration::from_millis(20));
            if ctrl.is_cancelled() {
                break;
            }
        }

        // Pop one completion. If S_FALSE (no completion ready),
        // submit + wait for at least one. Otherwise process it.
        let mut cqe: IORING_CQE = std::mem::zeroed();
        let hr = PopIoRingCompletion(ring, &mut cqe);
        if hr == S_FALSE {
            let hr = SubmitIoRing(ring, 1, u32::MAX, &mut submitted);
            if hr != S_OK {
                return Err(io::Error::other(format!(
                    "SubmitIoRing wait: HRESULT 0x{hr:08X}"
                )));
            }
            continue;
        }
        if hr != S_OK {
            return Err(io::Error::other(format!(
                "PopIoRingCompletion failed: HRESULT 0x{hr:08X}"
            )));
        }
        if cqe.ResultCode != S_OK {
            return Err(io::Error::other(format!(
                "ioring op failed: HRESULT 0x{:08X}",
                cqe.ResultCode
            )));
        }

        let (slot, was_write) = unpack_userdata(cqe.UserData);
        let bytes_transferred = cqe.Information as u32;

        if !was_write {
            // Read completion → submit a write at the same offset.
            let SlotState::Reading { offset, len } = slot_states[slot] else {
                return Err(io::Error::other(format!(
                    "slot {slot} read completion in unexpected state {:?}",
                    slot_states[slot]
                )));
            };
            // Sanity: kernel returned the bytes we asked for (or
            // EOF, but we sized the request to alloc_size which is
            // sector-aligned ≥ total).
            if bytes_transferred == 0 {
                // Treat as logical EOF on this slot.
                slot_states[slot] = SlotState::Done;
                in_flight -= 1;
                continue;
            }
            let buf_ref = make_buffer_ref_registered(slot as u32, 0);
            let hr = BuildIoRingWriteFile(
                ring,
                dst_ref,
                buf_ref,
                bytes_transferred,
                offset,
                FILE_WRITE_FLAGS_NONE,
                pack_userdata(slot, true),
                IOSQE_FLAGS_NONE,
            );
            if hr != S_OK {
                return Err(io::Error::other(format!(
                    "BuildIoRingWriteFile slot {slot}: HRESULT 0x{hr:08X}"
                )));
            }
            slot_states[slot] = SlotState::Writing {
                offset,
                len: bytes_transferred,
            };
            // Submit immediately so the kernel can start.
            let _ = SubmitIoRing(ring, 0, 0, &mut submitted);
            // Don't decrement in_flight here — slot is still busy.
            let _ = len; // suppress unused
        } else {
            // Write completion. Update progress, queue next read.
            let SlotState::Writing { offset, len } = slot_states[slot] else {
                return Err(io::Error::other(format!(
                    "slot {slot} write completion in unexpected state {:?}",
                    slot_states[slot]
                )));
            };
            let _ = offset;
            bytes_done.fetch_add(len as u64, Ordering::Relaxed);

            if bytes_remaining > 0 {
                let to_read = std::cmp::min(buffer_bytes as u64, bytes_remaining) as u32;
                let buf_ref = make_buffer_ref_registered(slot as u32, 0);
                let hr = BuildIoRingReadFile(
                    ring,
                    src_ref,
                    buf_ref,
                    to_read,
                    next_offset,
                    pack_userdata(slot, false),
                    IOSQE_FLAGS_NONE,
                );
                if hr != S_OK {
                    return Err(io::Error::other(format!(
                        "BuildIoRingReadFile cont slot {slot}: HRESULT 0x{hr:08X}"
                    )));
                }
                slot_states[slot] = SlotState::Reading {
                    offset: next_offset,
                    len: to_read,
                };
                next_offset += to_read as u64;
                bytes_remaining -= to_read as u64;
                let _ = SubmitIoRing(ring, 0, 0, &mut submitted);
            } else {
                slot_states[slot] = SlotState::Done;
                in_flight -= 1;
            }
        }
    }

    // --- Truncate dst to exact total --------------------------------
    seek_and_set_eof(dst_handle.0, total as i64)?;

    Ok(())
}

/// Drain exactly `count` completions out of the ring. Returns Ok on
/// the first non-S_OK ResultCode (the caller maps it to an io::Error
/// at the call site if needed, but for the registration ops we treat
/// any failure as fatal).
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn drain_completions(ring: HIORING, count: usize) -> io::Result<()> {
    let mut popped = 0usize;
    let mut spins = 0u32;
    while popped < count {
        let mut cqe: IORING_CQE = std::mem::zeroed();
        let hr = PopIoRingCompletion(ring, &mut cqe);
        if hr == S_FALSE {
            // No completion ready yet. Submit + wait for one.
            let mut submitted: u32 = 0;
            let hr = SubmitIoRing(ring, 1, u32::MAX, &mut submitted);
            if hr != S_OK {
                return Err(io::Error::other(format!(
                    "SubmitIoRing drain wait: HRESULT 0x{hr:08X}"
                )));
            }
            spins += 1;
            if spins > 1000 {
                return Err(io::Error::other(
                    "drain_completions: spun 1000 times without progress",
                ));
            }
            continue;
        }
        if hr != S_OK {
            return Err(io::Error::other(format!(
                "PopIoRingCompletion drain: HRESULT 0x{hr:08X}"
            )));
        }
        if cqe.ResultCode != S_OK {
            return Err(io::Error::other(format!(
                "ioring drain op failed: HRESULT 0x{:08X}",
                cqe.ResultCode
            )));
        }
        popped += 1;
        spins = 0;
    }
    Ok(())
}

fn make_handle_ref_index(idx: u32) -> IORING_HANDLE_REF {
    IORING_HANDLE_REF {
        Kind: IORING_REF_REGISTERED,
        Handle: IORING_HANDLE_REF_0 { Index: idx },
    }
}

fn make_buffer_ref_registered(buffer_idx: u32, offset: u32) -> IORING_BUFFER_REF {
    IORING_BUFFER_REF {
        Kind: IORING_REF_REGISTERED,
        Buffer: IORING_BUFFER_REF_0 {
            IndexAndOffset: IORING_REGISTERED_BUFFER {
                BufferIndex: buffer_idx,
                Offset: offset,
            },
        },
    }
}

fn open_for_ioring(path: &Path, read: bool) -> io::Result<HANDLE> {
    let wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let (access, share, disposition) = if read {
        (GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING)
    } else {
        (GENERIC_WRITE, 0, CREATE_ALWAYS)
    };
    // SAFETY: `wide` is a NUL-terminated UTF-16 path; CreateFileW
    // accepts NULL security_attributes and template_handle.
    let handle = unsafe {
        CreateFileW(
            wide.as_ptr(),
            access,
            share,
            ptr::null(),
            disposition,
            FILE_FLAG_OVERLAPPED | FILE_FLAG_NO_BUFFERING,
            ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE || handle.is_null() {
        return Err(io::Error::last_os_error());
    }
    Ok(handle)
}

fn seek_and_set_eof(handle: HANDLE, size: i64) -> io::Result<()> {
    let mut new_pos: i64 = 0;
    // SAFETY: handle is owned by the caller, valid for SetFilePointerEx.
    let ok = unsafe { SetFilePointerEx(handle, size, &mut new_pos, FILE_BEGIN) };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: handle is owned by the caller, valid for SetEndOfFile.
    let ok = unsafe { SetEndOfFile(handle) };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
