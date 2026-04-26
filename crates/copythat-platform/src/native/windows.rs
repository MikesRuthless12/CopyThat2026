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
const NO_BUFFERING_THRESHOLD_DEFAULT: u64 = 256 * 1024 * 1024;

fn no_buffering_threshold() -> u64 {
    static CACHED: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *CACHED.get_or_init(|| {
        std::env::var("COPYTHAT_NO_BUFFERING_THRESHOLD_MB")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(|mb| mb.saturating_mul(1024 * 1024))
            .unwrap_or(NO_BUFFERING_THRESHOLD_DEFAULT)
    })
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

    // Phase 13c — opt-in parallel multi-chunk copy for large files.
    // Gate behind `COPYTHAT_PARALLEL_CHUNKS=<N>` env var so users
    // can A/B test it against the single-stream `CopyFileExW` path
    // without risking default-path regressions until the numbers
    // prove it's universally better.
    if let Some(n) = super::parallel::requested_chunks(total) {
        return super::parallel::parallel_chunk_copy(src, dst, total, n, ctrl, events).await;
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
    let started = Instant::now();
    let events_for_progress = events.clone();
    let total_for_progress = total;
    let ctx_for_poll = ctx.clone();
    let progress_task = tokio::spawn(async move {
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
    });

    let src_w = wide(&src);
    let dst_w = wide(&dst);

    let flags: u32 = if total >= no_buffering_threshold() {
        COPY_FILE_NO_BUFFERING
    } else {
        0
    };

    let ctx_for_block = ctx.clone();
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
                Some(progress_routine),
                Arc::as_ptr(&ctx_for_block) as *const core::ffi::c_void,
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
    let _ = progress_task.await;

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

fn wide(path: &Path) -> Vec<u16> {
    let mut v: Vec<u16> = OsStr::new(path).encode_wide().collect();
    v.push(0);
    v
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

pub(crate) fn filesystem_name(path: &Path) -> Option<String> {
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
         (Get-Volume -DriveLetter '{letter}' | Select-Object -ExpandProperty FileSystem)"
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
    if token.is_empty() { None } else { Some(token) }
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
}
