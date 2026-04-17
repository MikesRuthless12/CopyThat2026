//! The `fast_copy` dispatcher.
//!
//! Tries strategies in order — reflink → OS-native → async fallback —
//! and reports back which one actually moved the bytes. Honours
//! [`CopyOptions::strategy`] so callers can scope the attempt window
//! (e.g. `NoReflink` for filesystems where reflink has known bugs).

use std::path::{Path, PathBuf};
use std::time::Instant;

use copythat_core::{CopyControl, CopyError, CopyEvent, CopyOptions, CopyStrategy, copy_file};
use tokio::sync::mpsc;

use crate::native::{self, NativeOutcome};
use crate::outcome::{ChosenStrategy, FastCopyOutcome};
use crate::reflink_path::{self, ReflinkOutcome};

/// Attempt the fastest available copy from `src` to `dst`.
///
/// Returns the final [`FastCopyOutcome`] on success. On failure, the
/// dispatcher emits `CopyEvent::Failed` and returns `Err`. The
/// destination is not unlinked automatically — callers using
/// [`copythat_core::copy_file`] get cleanup via the engine's own
/// `keep_partial` path.
pub async fn fast_copy(
    src: &Path,
    dst: &Path,
    opts: CopyOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<FastCopyOutcome, CopyError> {
    let src_owned: PathBuf = src.to_path_buf();
    let dst_owned: PathBuf = dst.to_path_buf();

    // Resolve total byte count up front so each strategy can emit
    // accurate Started / Progress events. A symlink source with
    // follow=false is forwarded straight to the async engine — the
    // platform layer doesn't have a generic symlink-clone primitive.
    let metadata_result = if opts.follow_symlinks {
        tokio::fs::metadata(&src_owned).await
    } else {
        tokio::fs::symlink_metadata(&src_owned).await
    };
    let src_meta = match metadata_result {
        Ok(m) => m,
        Err(e) => {
            let err = CopyError::from_io(&src_owned, &dst_owned, e);
            let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
            return Err(err);
        }
    };
    if !opts.follow_symlinks && src_meta.file_type().is_symlink() {
        return run_async_fallback(&src_owned, &dst_owned, opts, ctrl, events).await;
    }
    let total = src_meta.len();

    let started = Instant::now();

    // ---------- 1. Reflink ----------
    if matches!(opts.strategy, CopyStrategy::Auto | CopyStrategy::AlwaysFast) {
        match reflink_path::try_reflink(src_owned.clone(), dst_owned.clone()).await {
            ReflinkOutcome::Cloned => {
                let elapsed = started.elapsed();
                let rate = rate_bps(total, elapsed);
                emit_started(&src_owned, &dst_owned, total, &events).await;
                let _ = events
                    .send(CopyEvent::Progress {
                        bytes: total,
                        total,
                        rate_bps: rate,
                    })
                    .await;
                let _ = events
                    .send(CopyEvent::Completed {
                        bytes: total,
                        duration: elapsed,
                        rate_bps: rate,
                    })
                    .await;
                return Ok(FastCopyOutcome {
                    strategy: ChosenStrategy::Reflink,
                    bytes: total,
                    duration: elapsed,
                    rate_bps: rate,
                });
            }
            ReflinkOutcome::NotSupported => {
                // Fall through to the next strategy.
            }
            ReflinkOutcome::Io(e) => {
                let err = CopyError::from_io(&src_owned, &dst_owned, e);
                let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
                return Err(err);
            }
        }
    }

    // ---------- 2. OS-native accelerated path ----------
    if !matches!(opts.strategy, CopyStrategy::AlwaysAsync) {
        let started_native = Instant::now();
        match native::try_native_copy(
            src_owned.clone(),
            dst_owned.clone(),
            total,
            ctrl.clone(),
            events.clone(),
        )
        .await
        {
            NativeOutcome::Done { strategy, bytes } => {
                let elapsed = started_native.elapsed();
                let rate = rate_bps(bytes, elapsed);
                let _ = events
                    .send(CopyEvent::Completed {
                        bytes,
                        duration: elapsed,
                        rate_bps: rate,
                    })
                    .await;
                return Ok(FastCopyOutcome {
                    strategy,
                    bytes,
                    duration: elapsed,
                    rate_bps: rate,
                });
            }
            NativeOutcome::Cancelled => {
                let err = CopyError::from_io(
                    &src_owned,
                    &dst_owned,
                    std::io::Error::new(std::io::ErrorKind::Interrupted, "copy cancelled"),
                );
                let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
                return Err(err);
            }
            NativeOutcome::Unsupported => {
                // fall through to async fallback (or strict-fail).
            }
            NativeOutcome::Io(e) => {
                let err = CopyError::from_io(&src_owned, &dst_owned, e);
                let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
                return Err(err);
            }
        }
    }

    // ---------- 3. AlwaysFast hard-fails here ----------
    if matches!(opts.strategy, CopyStrategy::AlwaysFast) {
        let err = CopyError::from_io(
            &src_owned,
            &dst_owned,
            std::io::Error::other(
                "no fast path available (CopyStrategy::AlwaysFast); reflink and OS-native both reported NotSupported",
            ),
        );
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // ---------- 4. Async fallback ----------
    run_async_fallback(&src_owned, &dst_owned, opts, ctrl, events).await
}

async fn run_async_fallback(
    src: &Path,
    dst: &Path,
    opts: CopyOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<FastCopyOutcome, CopyError> {
    // Prevent infinite recursion: strip the hook from the options we
    // pass to copy_file so it doesn't re-enter the dispatcher.
    let mut downgraded = opts;
    downgraded.fast_copy_hook = None;
    downgraded.strategy = CopyStrategy::AlwaysAsync;
    let report = copy_file(src, dst, downgraded, ctrl, events).await?;
    Ok(FastCopyOutcome {
        strategy: ChosenStrategy::AsyncFallback,
        bytes: report.bytes,
        duration: report.duration,
        rate_bps: report.rate_bps,
    })
}

async fn emit_started(src: &Path, dst: &Path, total: u64, events: &mpsc::Sender<CopyEvent>) {
    let _ = events
        .send(CopyEvent::Started {
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            total_bytes: total,
        })
        .await;
}

fn rate_bps(bytes: u64, elapsed: std::time::Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}
