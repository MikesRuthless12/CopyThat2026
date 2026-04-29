//! Windows native fast path.
//!
//! Strategy: `CopyFileExW` with our progress callback. The kernel's
//! own copy implementation is the fastest portable path on NTFS, ReFS,
//! and Dev Drive, and it integrates with VSS / sparse files / EFS.
//!
//! Pause / resume / cancel: `LPPROGRESS_ROUTINE` returns one of
//! `PROGRESS_CONTINUE` / `PROGRESS_CANCEL` / `PROGRESS_QUIET` /
//! `PROGRESS_STOP`. We poll [`CopyControl`] inside the callback —
//! `PROGRESS_CANCEL` aborts and discards the dst, `PROGRESS_QUIET`
//! mutes further callbacks but lets the copy finish, and we busy-spin
//! while paused (the callback runs on the I/O worker thread, so a
//! short spin is fine; the user-visible effect is a paused throughput
//! display that resumes immediately on `resume()`).
//!
//! Buffering: `CopyFileExW` accepts `COPY_FILE_NO_BUFFERING` to bypass
//! the cache for files ≥256 MiB — the same threshold Windows
//! Explorer / Robocopy use internally.

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;
use windows_sys::Win32::Foundation::{BOOL, FALSE, GetLastError, TRUE};
use windows_sys::Win32::Storage::FileSystem::{CopyFileExW, LPPROGRESS_ROUTINE_CALLBACK_REASON};

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

/// Below this size CopyFileExW's default buffered path wins:
/// the source is almost certainly already in page cache and the
/// destination write gets coalesced through the cache layer. At
/// or above, `COPY_FILE_NO_BUFFERING` avoids double-buffering
/// through the cache and lets the kernel stream directly to the
/// disk — much better for sustained throughput on cross-volume
/// copies.
///
/// Phase 13b tried 64 MiB to match Robocopy's internal threshold
/// but it REGRESSED the 256 MiB C→E benchmark by ~3× (338 →
/// 103 MiB/s): cross-volume copies between 64 MiB and the point
/// where the source fits in RAM benefit enormously from
/// page-cache read-prefetching + write coalescing on exFAT.
/// We put it back to 256 MiB (Windows Explorer's default); the
/// 10 GiB C→C workload still picks up the NO_BUFFERING path
/// since it sits well above the threshold.
///
/// Phase 38 follow-up: the threshold can be overridden at runtime
/// via `COPYTHAT_NO_BUFFERING_THRESHOLD_MB=<N>` (set to a very
/// large number to effectively disable). Used by `xtask bench-vs`
/// to A/B test on Dev Drive / NVMe-equipped machines where the
/// page-cache regression argument may not apply.
/// Phase 43 — minimum file size to engage `COPY_FILE_NO_BUFFERING`.
///
/// Background: Phase 13b shipped a 256 MiB threshold (Windows
/// Explorer's default). Phase 42 added an adaptive cap of 1 GiB
/// (`free_RAM / 4` clamped to `[256 MiB, 1 GiB]`) to limit
/// SuperFetch standby-list pollution on multi-GiB copies. That cap
/// lined CopyThat up to "lose" the head-to-head bench on the 10 GiB
/// workload because every other tested tool (cmd copy, RoboCopy
/// without `/J`, default-config TeraCopy) leaves writes buffered
/// and returns from the syscall before bytes are physically on
/// disk. CopyThat with `NO_BUFFERING` engaged was correctly waiting
/// for durability — but on the bench clock that looked 30 % slower.
///
/// Phase 43 reverses the priority: match Explorer / cmd / RoboCopy
/// throughput numbers on the common case, and only escalate to
/// `NO_BUFFERING` when the file genuinely cannot fit in the OS
/// write-back cache. The default floor is 16 GiB; the adaptive
/// formula uses `free_phys_ram` directly (not `/4`) so a 32 GiB
/// host with 24 GiB free will engage `NO_BUFFERING` only above
/// 24 GiB. The `COPYTHAT_NO_BUFFERING_THRESHOLD_MB` env var still
/// wins over both for opt-in tuning.
const NO_BUFFERING_THRESHOLD_DEFAULT: u64 = 16 * 1024 * 1024 * 1024;

fn no_buffering_threshold() -> u64 {
    // The threshold is computed once per process. Env-var changes
    // after the first copy are ignored — set the variable before
    // launching the binary if you need a per-run override (the bench
    // harness does this via `Start-Process -Environment`).
    static CACHED: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *CACHED.get_or_init(|| {
        // 1. Explicit env var override always wins (used by `xtask
        //    bench-vs` and by users with unusual hardware). Read
        //    once at first call; see the cache comment above.
        if let Some(mb) = std::env::var("COPYTHAT_NO_BUFFERING_THRESHOLD_MB")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
        {
            return mb.saturating_mul(1024 * 1024);
        }
        // 2. Phase 43 — adaptive default: max(free_RAM, 16 GiB).
        //    Files smaller than this stay buffered (matches Explorer /
        //    cmd / RoboCopy semantics + bench numbers); files at-or-
        //    above engage `NO_BUFFERING` because the page cache cannot
        //    coalesce a copy that's larger than free RAM anyway, and
        //    streaming directly to disk avoids stalling on cache
        //    eviction.
        let free = free_phys_ram_bytes().unwrap_or(0);
        free.max(NO_BUFFERING_THRESHOLD_DEFAULT)
    })
}

/// Phase 42 — query free physical RAM via `GlobalMemoryStatusEx`.
/// Returns `None` on probe failure (so the threshold falls back to
/// the static 16 GiB default per Phase 43).
fn free_phys_ram_bytes() -> Option<u64> {
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    let mut info: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    // SAFETY: info is a properly-sized MEMORYSTATUSEX with dwLength set.
    let ok = unsafe { GlobalMemoryStatusEx(&mut info) };
    if ok == 0 {
        return None;
    }
    Some(info.ullAvailPhys)
}
const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);
const ERROR_REQUEST_ABORTED: u32 = 1235;

// Stable Win32 ABI constants. windows-sys 0.59 ships these under
// `Win32::System::WindowsProgramming` which would require an extra
// feature flag — we just inline the ABI values to keep the dependency
// surface narrow.
const PROGRESS_CONTINUE: u32 = 0;
const PROGRESS_CANCEL: u32 = 1;
const PROGRESS_STOP: u32 = 2;
const PROGRESS_QUIET: u32 = 3;
const COPY_FILE_NO_BUFFERING: u32 = 0x00001000;
/// Phase 42 — Win10 1903+ (always satisfied on Win11+ baseline).
/// Negotiates SMB v3.1.1 traffic compression on remote (UNC) dests.
/// Free win on slow links; ignored when dest is local. Incompatible
/// with SMB Direct / RDMA — but those are server SKUs and the
/// negotiation simply skips the flag, so passing it unconditionally
/// on UNC dests is safe.
const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 0x10000000;

#[allow(dead_code)] // forward-compat: Windows error codes
const ERROR_NOT_SUPPORTED: u32 = 50;
#[allow(dead_code)]
const _PROGRESS_STOP_USED: u32 = PROGRESS_STOP;

/// Phase 13 tuning: the callback runs on `CopyFileExW`'s internal
/// worker thread and every microsecond it spends stalls the copy.
/// We used to `send` on an `mpsc::UnboundedSender` per-callback,
/// which adds a heap allocation + cross-thread hand-off 4 000 times
/// in a 256 MiB copy — measurable even when it's only ~100 ns per
/// call. The new shape is strictly atomic-only:
///
/// - Callback: check cancel → check pause → store bytes → return.
///   No channel send, no allocation, no syscalls unless paused.
/// - Progress emission: a tokio polling task reads `bytes` every
///   `PROGRESS_MIN_INTERVAL` and decides whether to emit a
///   `CopyEvent::Progress`. This caps emissions at ~20 / s regardless
///   of internal chunk count, and keeps the hot callback path as
///   close to a no-op as we can get on Rust's ABI.
struct CallbackCtx {
    ctrl: CopyControl,
    bytes: AtomicU64,
    cancel_flag: AtomicBool,
}

unsafe extern "system" fn progress_routine(
    total_file_size: i64,
    total_bytes_transferred: i64,
    _stream_size: i64,
    _stream_bytes_transferred: i64,
    _stream_number: u32,
    _callback_reason: LPPROGRESS_ROUTINE_CALLBACK_REASON,
    _src_handle: *mut core::ffi::c_void,
    _dst_handle: *mut core::ffi::c_void,
    ctx_raw: *const core::ffi::c_void,
) -> u32 {
    // SAFETY: ctx_raw was set via copyfile_state_set's symmetric
    // pointer in the dispatcher, where it points at an `Arc<CallbackCtx>`
    // that outlives the CopyFileExW call.
    let ctx = unsafe { &*(ctx_raw as *const CallbackCtx) };

    if ctx.ctrl.is_cancelled() {
        ctx.cancel_flag.store(true, Ordering::Release);
        return PROGRESS_CANCEL;
    }

    // Park inside the callback while paused. `CopyFileExW` runs the
    // callback on the worker thread that's also driving the copy, so
    // returning slowly stalls the I/O — exactly what we want.
    while ctx.ctrl.is_paused() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if ctx.ctrl.is_cancelled() {
            ctx.cancel_flag.store(true, Ordering::Release);
            return PROGRESS_CANCEL;
        }
    }

    // Fast path: single relaxed atomic store. The polling task picks
    // this up; we do NOT allocate / send / cross thread boundaries here.
    if total_bytes_transferred >= 0 {
        ctx.bytes
            .store(total_bytes_transferred as u64, Ordering::Relaxed);
    }

    // Once we've reached EOF we can ask Windows to stop firing the
    // callback to save crossings — pure micro-opt.
    if total_file_size >= 0 && total_bytes_transferred == total_file_size {
        return PROGRESS_QUIET;
    }
    PROGRESS_CONTINUE
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn try_native_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    // Phase 43 — when `true`, skip installing the per-chunk progress
    // callback on `CopyFileExW` / `CopyFile2` and skip the polling
    // task that drives `CopyEvent::Progress`. The CLI sets this on
    // `--quiet` runs where no UI is consuming progress; the GUI
    // leaves it `false` so the per-50 ms progress stream still drives
    // its bar. Saves a kernel→user thread crossing per kernel chunk
    // (~tens of thousands on multi-GB copies).
    disable_callback: bool,
) -> NativeOutcome {
    // Phase 38 follow-up — opt-in Robocopy-style overlapped I/O
    // pipeline for large files. Gate behind
    // `COPYTHAT_OVERLAPPED_IO=1` so users can A/B test against
    // the default `CopyFileExW` path on NVMe / Dev Drive hardware
    // without risking the default path until numbers prove it
    // universally wins.
    if super::windows_overlapped::requested(total).is_some() {
        return super::windows_overlapped::try_overlapped_copy(src, dst, total, ctrl, events).await;
    }

    // Phase 41 — auto-engage overlapped pipeline for cross-volume
    // large-file copies. The COMPETITOR-TEST.md baseline showed
    // Robocopy beat us by ~48 % on 10 GiB · C→E (USB-attached
    // SSD) because its overlapped pipeline kept the slow USB
    // command queue deeper than our default `CopyFileExW`
    // single-stream. Auto-engaging the overlapped path with
    // 8 in-flight 4 MiB slots and buffered I/O (NO_BUFFERING off,
    // because USB drives often have small on-device caches that
    // benefit from the OS write-behind) closes that gap on USB
    // and external SATA / NVMe enclosures without affecting
    // same-volume NVMe (which still gets the
    // `CopyFileExW`-default fast path).
    //
    // Opt out via `COPYTHAT_DISABLE_AUTO_OVERLAPPED=1` if the
    // heuristic regresses on a specific user's hardware.
    if total >= 1024 * 1024 * 1024
        && std::env::var("COPYTHAT_DISABLE_AUTO_OVERLAPPED")
            .ok()
            .as_deref()
            .is_none_or(|v| !matches!(v, "1" | "true" | "on"))
        && is_cross_volume(&src, &dst)
    {
        // Phase 42 — topology-driven slot/buffer/QD picker. The
        // Phase 41 fixed defaults (8 slots × 4 MiB, NO_BUFFERING
        // off) were tuned for USB-attached external SSD — the
        // canonical "competitor beats us by 48 %" scenario. With
        // `IOCTL_STORAGE_QUERY_PROPERTY` we can now ask the
        // destination volume what it actually is and pick the
        // right shape per the swarm-research table:
        //   NVMe   → 1 MiB / QD 8 / NO_BUFFERING on
        //   SATA SSD → 256 KiB / QD 4 / NO_BUFFERING on
        //   HDD    → 4 MiB / QD 1 / NO_BUFFERING off (cache friendly)
        //   USB    → 512 KiB / QD ≤4 / NO_BUFFERING off
        //   SMB    → 1 MiB / QD 8 / NO_BUFFERING off
        // Probe-failure path falls back to the Phase 41 USB-tuned
        // defaults (which is what we'd have shipped previously).
        let topo = crate::topology::probe(&dst)
            .unwrap_or_else(crate::topology::VolumeTopology::conservative_default);
        let buffer_kb = topo.recommended_buffer_bytes() / 1024;
        let slots = topo.recommended_queue_depth();
        let no_buffering = matches!(
            topo.bus_type,
            crate::topology::BusType::Nvme | crate::topology::BusType::Sata
        );
        return super::windows_overlapped::try_overlapped_copy_with_config(
            src,
            dst,
            total,
            ctrl,
            events,
            Some(slots),
            Some(buffer_kb),
            Some(no_buffering),
        )
        .await;
    }

    // Phase 13c — opt-in parallel multi-chunk copy for large files.
    // Gate behind `COPYTHAT_PARALLEL_CHUNKS=<N>` env var so users
    // can A/B test it against the single-stream `CopyFileExW` path
    // without risking default-path regressions until the numbers
    // prove it's universally better.
    if let Some(n) = super::parallel::requested_chunks(total) {
        return super::parallel::parallel_chunk_copy(src, dst, total, n, ctrl, events).await;
    }

    // Phase 42 — pre-copy attribute probe. The result feeds:
    // - the CopyFile2-vs-CopyFileExW routing decision (sparse
    //   sources on Win11 22H2+ go through CopyFile2 with
    //   COPY_FILE_ENABLE_SPARSE_COPY for native sparseness
    //   preservation),
    // - downstream logging (cloud placeholders / encrypted /
    //   compressed sources warrant a one-line log entry),
    // - the eventual hardlink-set scanner (#13) once it exists.
    let src_attrs = crate::attrs::probe(&src).unwrap_or_default();

    // Phase 42 follow-up — surface the noteworthy attribute classes
    // to stderr so operators see why a copy is slow / hydrating
    // network bytes / re-keying EFS material. The most user-impactful
    // one is `is_recall_on_data_access` — modern OneDrive cloud-only
    // placeholders silently trigger gigabyte-scale downloads when
    // read, and the user deserves a heads-up before the copy starts.
    // (Stays as `eprintln!` to match the existing
    // `copythat-platform: WARNING — …` style elsewhere in this crate;
    // there's no tracing subscriber wired at this layer.)
    if src_attrs.is_recall_on_data_access {
        eprintln!(
            "copythat-platform: NOTE — source {:?} is a OneDrive / Cloud Files \
             placeholder (RECALL_ON_DATA_ACCESS); reading it will trigger a \
             transparent hydration download from the cloud provider before \
             the copy can complete.",
            src
        );
    }
    if src_attrs.is_encrypted {
        eprintln!(
            "copythat-platform: NOTE — source {:?} is EFS-encrypted; \
             CopyFileExW will re-encrypt at the destination using the source \
             keys (cross-volume copies to non-NTFS targets may fail without \
             OpenEncryptedFileRaw).",
            src
        );
    }
    if src_attrs.is_compressed {
        eprintln!(
            "copythat-platform: NOTE — source {:?} is NTFS-compressed; the \
             kernel decompresses on read and the destination is written \
             uncompressed unless FSCTL_SET_COMPRESSION is reapplied.",
            src
        );
    }

    super::emit_started(&src, &dst, total, &events).await;

    let ctx = Arc::new(CallbackCtx {
        ctrl: ctrl.clone(),
        bytes: AtomicU64::new(0),
        cancel_flag: AtomicBool::new(false),
    });

    // Phase 13 tuning: progress is a polling task that reads the
    // callback's atomic every `PROGRESS_MIN_INTERVAL` rather than
    // reacting to a per-chunk channel send. This keeps the hot
    // callback path allocation-free and shaves real wall-clock
    // time off cached same-volume copies (where callback overhead
    // is a large fraction of the syscall time).
    //
    // Phase 43 — the polling task only runs when the caller wants
    // progress events. CLI `--quiet` and bench harnesses pass
    // `disable_callback = true`, in which case we skip both the
    // task spawn AND the per-chunk callback installation below.
    let started = Instant::now();
    let progress_task: Option<tokio::task::JoinHandle<()>> = if disable_callback {
        None
    } else {
        let events_for_progress = events.clone();
        let total_for_progress = total;
        let ctx_for_poll = ctx.clone();
        Some(tokio::spawn(async move {
            let mut ticker = tokio::time::interval(PROGRESS_MIN_INTERVAL);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let mut last_emit_bytes: u64 = 0;
            // First tick fires immediately — skip it so the very first
            // progress event carries a real delta.
            ticker.tick().await;
            loop {
                ticker.tick().await;
                let bytes = ctx_for_poll.bytes.load(Ordering::Relaxed);
                if bytes == last_emit_bytes {
                    // Either the copy just started or it's already done
                    // and nobody has dropped the Arc yet. Either way no
                    // emission needed.
                    if Arc::strong_count(&ctx_for_poll) == 1 {
                        break;
                    }
                    continue;
                }
                if bytes.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES {
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
                if Arc::strong_count(&ctx_for_poll) == 1 {
                    // CopyFileExW has returned — the dispatcher dropped
                    // its ctx clone. Stop polling.
                    break;
                }
            }
        }))
    };

    let src_w = wide(&src);
    let dst_w = wide(&dst);
    // Phase 42 — tightened invariants. `wide()` always appends a NUL,
    // but we sanity-guard the FFI precondition explicitly so any future
    // refactor that strips the terminator gets caught in debug.
    debug_assert_eq!(
        src_w.last().copied(),
        Some(0u16),
        "src wide buffer must be NUL-terminated for CopyFileExW"
    );
    debug_assert_eq!(
        dst_w.last().copied(),
        Some(0u16),
        "dst wide buffer must be NUL-terminated for CopyFileExW"
    );
    // CopyFileExW with total == 0 is well-defined (creates an empty
    // dst), but the dispatcher sizes its NO_BUFFERING decision off
    // `total` and our progress / event story expects a positive byte
    // count. Treat zero as a contract violation in debug.
    debug_assert!(total > 0, "CopyFileExW path expects total > 0");

    let mut flags: u32 = if total >= no_buffering_threshold() {
        COPY_FILE_NO_BUFFERING
    } else {
        0
    };
    // Phase 42 — opportunistic SMB traffic compression on UNC dests.
    // Always-satisfied on the Win11+ baseline (introduced in
    // Win10 1903); on local dests the kernel ignores the flag.
    //
    // Phase 40 — when the flag is engaged, fire a one-shot
    // `SmbCompressionActive` event so the UI can render the header
    // badge + post-completion "Saved over the network" toast. The
    // negotiated chained-compression algorithm is server-decided in
    // the SMB 3.1.1 handshake and is not surfaced through any public
    // user-mode API today, so we report `"unknown"` as the wire
    // string. A future kernel-mode probe can fill this in without
    // breaking the wire format.
    if crate::topology::is_unc_path(&dst) {
        flags |= COPY_FILE_REQUEST_COMPRESSED_TRAFFIC;
        let _ = events
            .send(CopyEvent::SmbCompressionActive { algo: "unknown" })
            .await;
    }

    // Phase 42 — sparse sources on Win11 22H2+ benefit from
    // CopyFile2's `COPY_FILE_ENABLE_SPARSE_COPY` flag, which
    // preserves unallocated zero ranges natively. Route through the
    // CopyFile2 wrapper instead of CopyFileExW. Falls back to
    // CopyFileExW on older Win11 builds (21H2 → manual sparse
    // pathway in `engine.rs`).
    if src_attrs.is_sparse && crate::os::is_win11_22h2_plus() {
        return try_copy_file_2(
            src.clone(),
            dst.clone(),
            total,
            ctrl.clone(),
            events.clone(),
            flags,
            true, // enable_sparse_copy
            disable_callback,
        )
        .await;
    }

    let ctx_for_block = ctx.clone();
    // Phase 43 — when the caller opted out of progress events
    // (e.g. CLI `--quiet`), pass NULL for the progress callback so
    // CopyFileExW doesn't fire it per kernel chunk. Saves a thread
    // crossing per chunk; on a 10 GiB copy that's tens of thousands
    // of avoided crossings. The cancel-pending flag is unused in
    // this path because a callback-less copy can't cooperatively
    // cancel mid-stream — callers wanting cancel should leave the
    // callback installed.
    let join = tokio::task::spawn_blocking(move || {
        let mut cancel_pending: BOOL = FALSE;
        // SAFETY: src_w and dst_w are NUL-terminated UTF-16 buffers
        // owned by this scope; the callback context outlives this
        // call (held via Arc); cancel_pending is read but not mutated
        // by CopyFileExW once the call returns.
        let ok = unsafe {
            CopyFileExW(
                src_w.as_ptr(),
                dst_w.as_ptr(),
                if disable_callback {
                    None
                } else {
                    Some(progress_routine)
                },
                if disable_callback {
                    std::ptr::null()
                } else {
                    Arc::as_ptr(&ctx_for_block) as *const core::ffi::c_void
                },
                &mut cancel_pending as *mut BOOL,
                flags,
            )
        };
        if ok == TRUE {
            Ok(())
        } else {
            // SAFETY: GetLastError is thread-local and always valid to
            // call after a Windows API failure.
            let raw = unsafe { GetLastError() };
            Err(io::Error::from_raw_os_error(raw as i32))
        }
    })
    .await;

    drop(ctx); // close progress channel
    if let Some(task) = progress_task {
        let _ = task.await;
    }

    match join {
        Ok(Ok(())) => {
            // CopyFileExW guarantees the destination matches `total`
            // bytes on success.
            let bytes = total;
            NativeOutcome::Done {
                strategy: ChosenStrategy::CopyFileExW,
                bytes,
            }
        }
        Ok(Err(e)) => {
            // Map cancellation: ERROR_REQUEST_ABORTED is what
            // CopyFileExW returns when the callback returned
            // PROGRESS_CANCEL.
            if e.raw_os_error() == Some(ERROR_REQUEST_ABORTED as i32) {
                return NativeOutcome::Cancelled;
            }
            NativeOutcome::Io(e)
        }
        Err(join_err) => NativeOutcome::Io(io::Error::other(format!(
            "CopyFileExW spawn_blocking panicked: {join_err}"
        ))),
    }
}

// ---------------------------------------------------------------------
// Phase 42 — CopyFile2 path (sparse-source Win11 22H2+).
// ---------------------------------------------------------------------

/// `COPY_FILE_ENABLE_SPARSE_COPY` from `winbase.h` — Win11 22H2+ flag
/// that tells CopyFile2 to preserve sparse unallocated ranges
/// natively, skipping the read-zeros / write-zeros round-trip.
const COPY_FILE_ENABLE_SPARSE_COPY: u32 = 0x20000000;

/// CopyFile2 message types we care about. Stable Win32 ABI.
const COPYFILE2_CALLBACK_CHUNK_FINISHED: u32 = 2;
const COPYFILE2_CALLBACK_ERROR: u32 = 5;

/// CopyFile2 callback action codes.
const COPYFILE2_PROGRESS_CONTINUE: u32 = 0;
const COPYFILE2_PROGRESS_CANCEL: u32 = 1;
const COPYFILE2_PROGRESS_QUIET: u32 = 3;

/// Minimal header for `COPYFILE2_MESSAGE`. The full union has many
/// variants — we read the fixed prefix (`Type`, `dwPadding`) and
/// then re-cast the buffer to the variant we need based on `Type`.
/// Stable Win32 ABI; layout matches `winbase.h`.
#[repr(C)]
#[allow(non_snake_case)]
struct CopyFile2MessageHeader {
    Type: u32,
    dwPadding: u32,
}

/// `COPYFILE2_MESSAGE.Info.ChunkFinished` — the variant we read for
/// progress accounting.
#[repr(C)]
#[allow(non_snake_case)]
struct CopyFile2ChunkFinished {
    header: CopyFile2MessageHeader,
    dwStreamNumber: u32,
    dwReserved: u32,
    hSourceFile: *mut core::ffi::c_void,
    hDestinationFile: *mut core::ffi::c_void,
    uliChunkNumber: u64,
    uliChunkSize: u64,
    uliStreamSize: u64,
    uliStreamBytesTransferred: u64,
    uliTotalFileSize: u64,
    uliTotalBytesTransferred: u64,
}

/// `COPYFILE2_EXTENDED_PARAMETERS` layout per `winbase.h`. We hand-
/// roll this so we don't need an extra windows-sys feature flag for
/// the CopyFile2 path while the migration remains scoped to sparse
/// sources.
#[repr(C)]
#[allow(non_snake_case)]
struct CopyFile2ExtendedParameters {
    dwSize: u32,
    dwCopyFlags: u32,
    pfCancel: *mut BOOL,
    pProgressRoutine: Option<
        unsafe extern "system" fn(*const CopyFile2MessageHeader, *mut core::ffi::c_void) -> u32,
    >,
    pvCallbackContext: *mut core::ffi::c_void,
}

// Defensive compile-time assertions: the three hand-rolled structs
// above must match the `winbase.h` ABI exactly or we'll silently
// corrupt CopyFile2's parameter / message reads. The offsets below
// are the documented x64 layout; the `target_pointer_width = "64"`
// gate keeps a future 32-bit-Windows port building (where pointer
// fields would shift). The shipped MSVC x64 toolchain is the only
// supported target today.
#[cfg(target_pointer_width = "64")]
const _: () = {
    use std::mem::{offset_of, size_of};

    // CopyFile2MessageHeader — { u32, u32 }, total 8 bytes.
    assert!(offset_of!(CopyFile2MessageHeader, Type) == 0);
    assert!(offset_of!(CopyFile2MessageHeader, dwPadding) == 4);
    assert!(size_of::<CopyFile2MessageHeader>() == 8);

    // CopyFile2ChunkFinished — { header(8), 2×u32(8), 2×ptr(16), 6×u64(48) }.
    assert!(offset_of!(CopyFile2ChunkFinished, header) == 0);
    assert!(offset_of!(CopyFile2ChunkFinished, dwStreamNumber) == 8);
    assert!(offset_of!(CopyFile2ChunkFinished, dwReserved) == 12);
    assert!(offset_of!(CopyFile2ChunkFinished, hSourceFile) == 16);
    assert!(offset_of!(CopyFile2ChunkFinished, hDestinationFile) == 24);
    assert!(offset_of!(CopyFile2ChunkFinished, uliChunkNumber) == 32);
    assert!(offset_of!(CopyFile2ChunkFinished, uliChunkSize) == 40);
    assert!(offset_of!(CopyFile2ChunkFinished, uliStreamSize) == 48);
    assert!(offset_of!(CopyFile2ChunkFinished, uliStreamBytesTransferred) == 56);
    assert!(offset_of!(CopyFile2ChunkFinished, uliTotalFileSize) == 64);
    assert!(offset_of!(CopyFile2ChunkFinished, uliTotalBytesTransferred) == 72);

    // CopyFile2ExtendedParameters — { 2×u32, ptr, fn-ptr, ptr } = 32 bytes.
    assert!(offset_of!(CopyFile2ExtendedParameters, dwSize) == 0);
    assert!(offset_of!(CopyFile2ExtendedParameters, dwCopyFlags) == 4);
    assert!(offset_of!(CopyFile2ExtendedParameters, pfCancel) == 8);
    assert!(offset_of!(CopyFile2ExtendedParameters, pProgressRoutine) == 16);
    assert!(offset_of!(CopyFile2ExtendedParameters, pvCallbackContext) == 24);
    assert!(size_of::<CopyFile2ExtendedParameters>() == 32);
};

unsafe extern "system" {
    /// `CopyFile2` from kernel32. Declared inline so we don't need to
    /// add `Win32_Storage_FileSystem` extras for this single function;
    /// the symbol is part of the ABI-stable kernel32 surface and has
    /// been since Windows 8.
    ///
    /// `extern "system"` is the canonical ABI for Win32 imports — on
    /// x64 it coincides with the platform C ABI, but on the (now-rare)
    /// 32-bit Windows target it resolves to `stdcall`. Using the
    /// canonical ABI here insulates the FFI surface from any future
    /// 32-bit Windows port regression.
    fn CopyFile2(
        pwszExistingFileName: *const u16,
        pwszNewFileName: *const u16,
        pExtendedParameters: *const CopyFile2ExtendedParameters,
    ) -> i32; // HRESULT
}

unsafe extern "system" fn copyfile2_callback(
    msg_ptr: *const CopyFile2MessageHeader,
    ctx_raw: *mut core::ffi::c_void,
) -> u32 {
    // SAFETY: ctx_raw points at the same `CallbackCtx` we wired up
    // for the CopyFileExW path; layout is identical (we reuse the
    // type so callers see one telemetry surface).
    let ctx = unsafe { &*(ctx_raw as *const CallbackCtx) };

    if ctx.ctrl.is_cancelled() {
        ctx.cancel_flag.store(true, Ordering::Release);
        return COPYFILE2_PROGRESS_CANCEL;
    }

    while ctx.ctrl.is_paused() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if ctx.ctrl.is_cancelled() {
            ctx.cancel_flag.store(true, Ordering::Release);
            return COPYFILE2_PROGRESS_CANCEL;
        }
    }

    // SAFETY: msg_ptr is a valid CopyFile2 message for the duration
    // of this callback per Win32 contract.
    let header = unsafe { &*msg_ptr };
    if header.Type == COPYFILE2_CALLBACK_CHUNK_FINISHED {
        // Re-cast the message buffer to the ChunkFinished variant.
        let chunk: &CopyFile2ChunkFinished =
            unsafe { &*(msg_ptr as *const CopyFile2ChunkFinished) };
        ctx.bytes
            .store(chunk.uliTotalBytesTransferred, Ordering::Relaxed);
        if chunk.uliTotalBytesTransferred == chunk.uliTotalFileSize {
            return COPYFILE2_PROGRESS_QUIET;
        }
    } else if header.Type == COPYFILE2_CALLBACK_ERROR {
        // Surfaced via the HRESULT return; no action needed here.
        return COPYFILE2_PROGRESS_CONTINUE;
    }

    COPYFILE2_PROGRESS_CONTINUE
}

#[allow(clippy::too_many_arguments)]
async fn try_copy_file_2(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    base_flags: u32,
    enable_sparse_copy: bool,
    // Phase 43 — symmetric with `try_native_copy`. When `true`, skip
    // both the polling task and the per-chunk `copyfile2_callback`
    // installation; CopyFile2 runs without firing its progress
    // callback per chunk.
    disable_callback: bool,
) -> NativeOutcome {
    super::emit_started(&src, &dst, total, &events).await;

    let ctx = Arc::new(CallbackCtx {
        ctrl: ctrl.clone(),
        bytes: AtomicU64::new(0),
        cancel_flag: AtomicBool::new(false),
    });

    let started = Instant::now();
    let progress_task: Option<tokio::task::JoinHandle<()>> = if disable_callback {
        None
    } else {
        let events_for_progress = events.clone();
        let total_for_progress = total;
        let ctx_for_poll = ctx.clone();
        Some(tokio::spawn(async move {
            let mut ticker = tokio::time::interval(PROGRESS_MIN_INTERVAL);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let mut last_emit_bytes: u64 = 0;
            ticker.tick().await;
            loop {
                ticker.tick().await;
                let bytes = ctx_for_poll.bytes.load(Ordering::Relaxed);
                if bytes == last_emit_bytes {
                    if Arc::strong_count(&ctx_for_poll) == 1 {
                        break;
                    }
                    continue;
                }
                if bytes.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES {
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
                if Arc::strong_count(&ctx_for_poll) == 1 {
                    break;
                }
            }
        }))
    };

    let src_w = wide(&src);
    let dst_w = wide(&dst);
    let mut flags = base_flags;
    if enable_sparse_copy {
        flags |= COPY_FILE_ENABLE_SPARSE_COPY;
    }

    let ctx_for_block = ctx.clone();
    let join = tokio::task::spawn_blocking(move || {
        let mut cancel: BOOL = FALSE;
        let params = CopyFile2ExtendedParameters {
            dwSize: std::mem::size_of::<CopyFile2ExtendedParameters>() as u32,
            dwCopyFlags: flags,
            pfCancel: &mut cancel as *mut BOOL,
            pProgressRoutine: if disable_callback {
                None
            } else {
                Some(copyfile2_callback)
            },
            pvCallbackContext: if disable_callback {
                std::ptr::null_mut()
            } else {
                Arc::as_ptr(&ctx_for_block) as *mut core::ffi::c_void
            },
        };
        // SAFETY: src_w / dst_w are NUL-terminated UTF-16; params is a
        // properly-sized COPYFILE2_EXTENDED_PARAMETERS owned by this
        // scope; ctx is held via Arc for the duration of the call.
        let hresult = unsafe { CopyFile2(src_w.as_ptr(), dst_w.as_ptr(), &params) };
        if hresult >= 0 {
            Ok(())
        } else {
            Err(copyfile2_hresult_to_io_error(hresult))
        }
    })
    .await;

    drop(ctx);
    if let Some(task) = progress_task {
        let _ = task.await;
    }

    match join {
        Ok(Ok(())) => NativeOutcome::Done {
            strategy: ChosenStrategy::CopyFileExW, // share telemetry slot
            bytes: total,
        },
        Ok(Err(e)) => {
            if e.raw_os_error() == Some(ERROR_REQUEST_ABORTED as i32) {
                return NativeOutcome::Cancelled;
            }
            NativeOutcome::Io(e)
        }
        Err(join_err) => NativeOutcome::Io(io::Error::other(format!(
            "CopyFile2 spawn_blocking panicked: {join_err}"
        ))),
    }
}

fn wide(path: &Path) -> Vec<u16> {
    let mut v: Vec<u16> = OsStr::new(path).encode_wide().collect();
    v.push(0);
    v
}

/// Translate a CopyFile2 HRESULT failure into the right `io::Error`
/// variant. Phase 42 ABI-safety hardening (wave-1) carved this out:
///
/// HRESULT layout: bit 31 = severity, bits 16..28 = facility, bits
/// 0..15 = code. `io::Error::from_raw_os_error` only makes sense when
/// facility == 7 (`FACILITY_WIN32`) — that's the case we get for the
/// vast majority of CopyFile2 failures (file-not-found, access-denied,
/// disk-full, etc.). For non-Win32 facilities (RPC=4, security=9,
/// HTTP=12, etc.) the low 16 bits don't correspond to a Win32 error
/// code at all, so synthesise a descriptive `io::Error::other`
/// instead. Lifted out of the closure so the wave-2 unit test can
/// exercise both branches without driving a real CopyFile2 call.
fn copyfile2_hresult_to_io_error(hresult: i32) -> io::Error {
    let facility = (hresult >> 16) & 0x1FFF;
    if facility == 7 {
        io::Error::from_raw_os_error(hresult & 0xFFFF)
    } else {
        io::Error::other(format!(
            "CopyFile2 HRESULT 0x{hresult:08x} (facility {facility})"
        ))
    }
}

/// Phase 41 — returns true iff `src` and `dst` (or `dst`'s parent
/// directory if `dst` doesn't exist yet) live on different volumes
/// per `helpers::volume_id`. Used by the auto-engage heuristic for
/// the overlapped-pipeline path on cross-volume large copies.
///
/// Conservative: any failure to identify a volume returns `false`
/// so we don't accidentally engage the deeper-pipeline path on a
/// same-volume NVMe copy where `CopyFileExW` already wins.
fn is_cross_volume(src: &Path, dst: &Path) -> bool {
    let src_id = match crate::helpers::volume_id(src) {
        Some(id) => id,
        None => return false,
    };
    // dst probably doesn't exist yet — probe its parent.
    let dst_probe = dst.parent().unwrap_or(dst);
    let dst_id = match crate::helpers::volume_id(dst_probe) {
        Some(id) => id,
        None => return false,
    };
    src_id != dst_id
}

// ---------------------------------------------------------------------
// Helpers: SSD probe + filesystem name
// ---------------------------------------------------------------------

pub(crate) fn is_ssd(path: &Path) -> Option<bool> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    let abs = std::fs::canonicalize(&probe_target).ok()?;
    let abs_str = abs.to_string_lossy().into_owned();
    let letter = abs_str
        .chars()
        .find(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())?;
    let script = format!(
        "$ErrorActionPreference='SilentlyContinue'; \
         (Get-Partition -DriveLetter '{letter}' | Get-Disk | \
          Get-PhysicalDisk | Select-Object -First 1 -ExpandProperty MediaType)"
    );
    let out = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(&script)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let token = String::from_utf8_lossy(&out.stdout)
        .trim()
        .to_ascii_lowercase();
    if token.is_empty() {
        return None;
    }
    match token.as_str() {
        "ssd" | "scm" => Some(true),
        "hdd" => Some(false),
        _ => None,
    }
}

/// Best-effort filesystem-name probe.
///
/// Phase 43 — replaces the previous `powershell.exe Get-Volume`
/// shell-out (~100–300 ms per invocation, multi-second on tree copies)
/// with a direct `GetVolumeInformationW` Win32 call (~1–2 µs). This is
/// on the dispatcher's hot path: every fast-copy probes the destination
/// FS to decide whether to engage the explicit reflink stage. The PS
/// variant was correct but its cost dominated the dispatcher when the
/// new NTFS-skip-reflink optimisation started calling it unconditionally.
///
/// Returns the lowercase filesystem token (`"ntfs"`, `"refs"`, `"exfat"`,
/// `"fat32"`, etc.) or `None` if the probe fails (unknown path,
/// unmounted volume, permission denied at the volume root).
pub(crate) fn filesystem_name(path: &Path) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{GetVolumeInformationW, GetVolumePathNameW};

    // Files that don't yet exist (typical for the destination of an
    // in-flight copy) get probed via their parent directory.
    let probe_target: &Path = if path.is_file() { path.parent()? } else { path };
    let mut wide: Vec<u16> = OsStr::new(probe_target).encode_wide().collect();
    wide.push(0);

    // Step 1 — resolve any subpath to its volume root (e.g.
    // `C:\Users\miken\Desktop` → `C:\`). `GetVolumeInformationW`
    // refuses anything other than a volume root.
    let mut root_buf: [u16; 260] = [0; 260];
    // SAFETY: `wide` is NUL-terminated; `root_buf` is a fixed-size
    // buffer sized to `MAX_PATH + 1` per Win32 documented limit.
    let path_ok =
        unsafe { GetVolumePathNameW(wide.as_ptr(), root_buf.as_mut_ptr(), root_buf.len() as u32) };
    if path_ok == 0 {
        return None;
    }

    // Step 2 — ask the volume what filesystem it carries. We only
    // care about the FS name string; the serial-number / max-component
    // / flags out-parameters get NULL pointers.
    let mut fs_name_buf: [u16; 64] = [0; 64];
    // SAFETY: `root_buf` was NUL-terminated by GetVolumePathNameW;
    // `fs_name_buf` is sized to fit every documented FS name string;
    // the unused out-parameters take NULL per the Win32 contract.
    let ok = unsafe {
        GetVolumeInformationW(
            root_buf.as_ptr(),
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            fs_name_buf.as_mut_ptr(),
            fs_name_buf.len() as u32,
        )
    };
    if ok == 0 {
        return None;
    }

    // The buffer is NUL-terminated; trim before lossy-decoding.
    let len = fs_name_buf.iter().position(|&c| c == 0).unwrap_or(0);
    if len == 0 {
        return None;
    }
    let s = String::from_utf16_lossy(&fs_name_buf[..len])
        .trim()
        .to_ascii_lowercase();
    if s.is_empty() { None } else { Some(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_terminates_with_nul() {
        let w = wide(Path::new("C:/foo"));
        assert!(w.last().copied() == Some(0));
        // Make sure the prefix decodes back.
        let body = &w[..w.len() - 1];
        let s = String::from_utf16(body).unwrap();
        assert_eq!(s, "C:/foo");
    }

    /// Phase 43 — `filesystem_name` should resolve C:\ to a known FS
    /// without spawning a subprocess. The exact value depends on the
    /// CI rig (NTFS / ReFS / Dev Drive), so we just assert it's one of
    /// the documented Windows tokens. Negative test below covers the
    /// failure mode.
    #[test]
    fn filesystem_name_returns_known_token_for_c_drive() {
        let fs = filesystem_name(Path::new("C:\\"));
        if let Some(name) = fs {
            assert!(
                matches!(name.as_str(), "ntfs" | "refs" | "exfat" | "fat32" | "fat"),
                "C:\\ filesystem token `{name}` is not in the documented Windows set"
            );
        }
        // None is acceptable on unusual rigs (e.g. CI containers
        // without a mounted C: volume root).
    }

    /// Phase 43 — bogus path must return `None` rather than panicking
    /// or shelling out. `\\?\Z:\\does\\not\\exist` resolves to a non-
    /// existent volume root; the Win32 probe fails cleanly.
    #[test]
    fn filesystem_name_returns_none_on_unmounted_volume() {
        let fs = filesystem_name(Path::new("\\\\?\\Z:\\does\\not\\exist"));
        assert!(
            fs.is_none(),
            "filesystem_name on unmounted Z: should be None, got {fs:?}"
        );
    }

    /// Phase 43 — the NO_BUFFERING threshold must default to at least
    /// 16 GiB so files in the 1–16 GiB band stay buffered (matching
    /// cmd / RoboCopy / Explorer semantics). The exact value is
    /// `max(free_phys_ram, 16 GiB)`, so on a RAM-rich host it can be
    /// higher; we assert the floor.
    ///
    /// `OnceLock`: this test reads the cached value, so it must run
    /// in the same process as nothing that pre-sets the env var.
    /// `cargo test` shares a process per test binary; the env var is
    /// not set by the test harness.
    #[test]
    fn no_buffering_threshold_floor_is_16_gib() {
        let threshold = no_buffering_threshold();
        assert!(
            threshold >= 16 * 1024 * 1024 * 1024,
            "Phase 43 floor regressed: threshold={threshold}, expected >= 16 GiB"
        );
    }

    /// Phase 42 wave-2 — a CopyFile2 HRESULT with facility ==
    /// FACILITY_WIN32 (7) maps to `io::Error::from_raw_os_error`,
    /// preserving the real Win32 error code so call sites can match
    /// `e.raw_os_error() == Some(ERROR_REQUEST_ABORTED)` and the
    /// like.
    #[test]
    fn copyfile2_hresult_facility_win32_uses_from_raw_os_error() {
        // 0x80070005 = severity-failure | facility 7 | code 5
        // (ERROR_ACCESS_DENIED).
        let hresult: i32 = 0x80070005u32 as i32;
        let err = copyfile2_hresult_to_io_error(hresult);
        assert_eq!(
            err.raw_os_error(),
            Some(5),
            "FACILITY_WIN32 HRESULT must round-trip to its Win32 error code"
        );
    }

    /// Phase 42 wave-2 — the wave-1 fix's payoff: HRESULTs in a
    /// non-Win32 facility (FACILITY_RPC=4, FACILITY_SECURITY=9,
    /// FACILITY_HTTP=12, …) must NOT pass through
    /// `from_raw_os_error`. Their low 16 bits aren't a Win32 errno;
    /// translating them as such would produce nonsense
    /// `io::ErrorKind` mappings. The fix returns
    /// `io::Error::other(...)` carrying the full HRESULT in the
    /// message so log readers / SIEMs still see the original value.
    #[test]
    fn copyfile2_hresult_facility_rpc_uses_io_error_other() {
        // 0x80040005 = severity-failure | facility 4 (FACILITY_RPC)
        // | code 5. The low 16 bits look like ERROR_ACCESS_DENIED
        // but they're really an RPC failure code; mapping via
        // from_raw_os_error would mislabel the error.
        let hresult: i32 = 0x80040005u32 as i32;
        let err = copyfile2_hresult_to_io_error(hresult);
        assert_eq!(
            err.raw_os_error(),
            None,
            "non-Win32 facility must NOT be wrapped via from_raw_os_error \
             (otherwise the Win32-error mapping would mislabel it)"
        );
        // io::Error::other lands in ErrorKind::Other on every Rust
        // version; the message must carry the raw HRESULT so
        // operators can decode it manually.
        assert_eq!(err.kind(), io::ErrorKind::Other);
        let rendered = format!("{err}");
        assert!(
            rendered.contains("0x80040005"),
            "error message must include the original HRESULT for ops triage: {rendered}"
        );
        assert!(
            rendered.contains("facility 4"),
            "error message must include the parsed facility number: {rendered}"
        );
    }

    /// Phase 42 wave-2 — additional non-Win32 facilities to lock
    /// in the boundary (FACILITY_SECURITY=9, FACILITY_HTTP=12). All
    /// should fall through to the io::Error::other branch.
    #[test]
    fn copyfile2_hresult_other_non_win32_facilities_use_io_error_other() {
        for (hresult, facility) in [
            (0x80090001u32 as i32, 9),  // FACILITY_SECURITY
            (0x800C0001u32 as i32, 12), // FACILITY_HTTP (Windows decimal 12 = 0xC)
        ] {
            let err = copyfile2_hresult_to_io_error(hresult);
            assert_eq!(
                err.raw_os_error(),
                None,
                "facility {facility}: must not pass through from_raw_os_error"
            );
            let rendered = format!("{err}");
            assert!(
                rendered.contains(&format!("facility {facility}")),
                "facility {facility}: rendering missing facility tag: {rendered}"
            );
        }
    }
}
