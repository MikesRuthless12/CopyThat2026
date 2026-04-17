//! `FastCopyHook` adapter — the bridge into [`copythat_core`].
//!
//! Drop a [`PlatformFastCopyHook`] into [`copythat_core::CopyOptions::fast_copy_hook`]
//! and every per-file `copy_file` (and therefore every per-file leaf
//! of `copy_tree`) routes through [`crate::fast_copy`] before the
//! engine's own async loop runs.

use std::path::PathBuf;
use std::pin::Pin;

use copythat_core::{
    CopyControl, CopyError, CopyEvent, CopyOptions, CopyReport, FastCopyHook, FastCopyHookOutcome,
};
use tokio::sync::mpsc;

use crate::dispatcher::fast_copy;

/// `FastCopyHook` implementation backed by [`crate::fast_copy`].
///
/// Stateless and cheap to clone — wrap once in `Arc::new` at app
/// startup and reuse forever.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformFastCopyHook;

impl FastCopyHook for PlatformFastCopyHook {
    fn try_copy<'a>(
        &'a self,
        src: PathBuf,
        dst: PathBuf,
        opts: CopyOptions,
        ctrl: CopyControl,
        events: mpsc::Sender<CopyEvent>,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<FastCopyHookOutcome, CopyError>> + Send + 'a>,
    > {
        Box::pin(async move {
            // The dispatcher runs its own async fallback when both
            // reflink and the OS-native path decline. Whichever path
            // wins, the bytes are already moved — we always report
            // Done back to the engine so it doesn't run its own loop
            // and double-copy.
            let outcome = fast_copy(&src, &dst, opts, ctrl, events).await?;
            Ok(FastCopyHookOutcome::Done(CopyReport {
                src,
                dst,
                bytes: outcome.bytes,
                duration: outcome.duration,
                rate_bps: outcome.rate_bps,
            }))
        })
    }
}
