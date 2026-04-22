//! Per-OS native fast paths.
//!
//! Each backing module exposes the same three entry points so the
//! dispatcher doesn't have to `cfg`-branch:
//!
//! - `try_native_copy(src, dst, total, ctrl, events) -> NativeOutcome`
//! - `is_ssd(path) -> Option<bool>`
//! - `filesystem_name(path) -> Option<String>`
//!
//! `NativeOutcome` distinguishes:
//! - `Done(strategy, bytes)` — the syscall reported success.
//! - `Cancelled` — the caller cancelled mid-copy via `CopyControl`.
//! - `Unsupported` — the syscall reported the operation isn't
//!   available (Linux `EXDEV` / `ENOSYS`, macOS Rosetta quirks, etc.).
//!   Caller should fall through to the async loop.
//! - `Io(io::Error)` — a real failure to surface.

use std::io;
use std::path::PathBuf;

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;

use crate::outcome::ChosenStrategy;

/// Outcome returned by a per-OS native attempt.
#[derive(Debug)]
#[allow(dead_code)] // each backend uses a different subset
pub(crate) enum NativeOutcome {
    Done {
        strategy: ChosenStrategy,
        bytes: u64,
    },
    Cancelled,
    Unsupported,
    Io(io::Error),
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::{filesystem_name, is_ssd, try_native_copy};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos::{filesystem_name, is_ssd, try_native_copy};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub(crate) use windows::{filesystem_name, is_ssd, try_native_copy};
#[cfg(target_os = "windows")]
pub(crate) mod parallel;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod fallback;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub(crate) use fallback::{filesystem_name, is_ssd, try_native_copy};

// --- Shared helpers usable by every backend ----------------------------

#[allow(dead_code)] // not all backends call this
pub(crate) async fn emit_started(
    src: &std::path::Path,
    dst: &std::path::Path,
    total: u64,
    events: &mpsc::Sender<CopyEvent>,
) {
    let _ = events
        .send(CopyEvent::Started {
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            total_bytes: total,
        })
        .await;
}

#[allow(dead_code)] // not all backends call this
pub(crate) fn fast_rate_bps(bytes: u64, elapsed: std::time::Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}

/// Shared progress-throttle constants. Every backend that emits
/// `CopyEvent::Progress` polls its own byte counter and re-emits
/// at most once per interval and per delta, preventing us from
/// drowning the event channel on fast media.
#[allow(dead_code)]
pub(crate) const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
#[allow(dead_code)]
pub(crate) const PROGRESS_MIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

#[allow(dead_code)]
pub(crate) async fn drain_events_into_void(_events: &mpsc::Sender<CopyEvent>) {
    // intentional: keep the API parameter symmetric with backends that
    // emit events directly.
}

#[allow(dead_code)]
pub(crate) fn unused_args(_src: PathBuf, _dst: PathBuf, _ctrl: CopyControl) {
    // Helper to silence "unused" warnings on backends that don't yet
    // observe every parameter (the fallback target_os is the main user).
}
