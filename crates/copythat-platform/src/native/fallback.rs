//! Catch-all native backend for OSes other than Linux / macOS / Windows.
//!
//! We have no syscall to wrap, so every attempt reports
//! [`NativeOutcome::Unsupported`] and the dispatcher falls through to
//! the async loop.

use std::path::{Path, PathBuf};

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;

use super::NativeOutcome;

#[allow(clippy::too_many_arguments)]
pub(crate) async fn try_native_copy(
    _src: PathBuf,
    _dst: PathBuf,
    _total: u64,
    _ctrl: CopyControl,
    _events: mpsc::Sender<CopyEvent>,
    _disable_callback: bool,
) -> NativeOutcome {
    NativeOutcome::Unsupported
}

pub(crate) fn is_ssd(_path: &Path) -> Option<bool> {
    None
}

pub(crate) fn filesystem_name(_path: &Path) -> Option<String> {
    None
}
