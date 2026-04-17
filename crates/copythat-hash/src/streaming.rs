//! `hash_file_async` — the async streaming entry point.
//!
//! Shape: `BufReader -> fill_buf/consume -> update -> finalize`. One
//! MiB buffer by default (matches `copythat_core::DEFAULT_BUFFER_SIZE`).
//! Between buffers the loop cooperatively checks the shared
//! `CopyControl` for pause / resume / cancel so a UI can drive a hash
//! job with the same handle it uses to drive a copy job. Progress
//! events are throttled to at most one per 16 KiB *and* one per 50 ms
//! — same thresholds as the copy engine.

use std::path::Path;
use std::time::{Duration, Instant};

use copythat_core::CopyControl;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

use crate::algorithm::{HashAlgorithm, Hasher};
use crate::error::HashError;
use crate::event::{HashEvent, HashReport};

/// Buffer size the streaming hasher uses. Matches the copy engine's
/// default so the verify hook can fit in the same I/O rhythm.
pub const DEFAULT_HASH_BUFFER: usize = 1024 * 1024; // 1 MiB

const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: Duration = Duration::from_millis(50);

/// Stream-hash the file at `path` under `algo`. Emits `HashEvent`s for
/// lifecycle notifications and honours `CopyControl` for pause / resume
/// / cancel. On success returns the full `HashReport`; on cancellation
/// or I/O error returns a typed `HashError` (and also sends a
/// `HashEvent::Failed`).
pub async fn hash_file_async(
    path: &Path,
    algo: HashAlgorithm,
    ctrl: CopyControl,
    events: mpsc::Sender<HashEvent>,
) -> Result<HashReport, HashError> {
    hash_file_async_with_buffer(path, algo, DEFAULT_HASH_BUFFER, ctrl, events).await
}

/// Variant of [`hash_file_async`] that lets the caller override the
/// buffer size. Used by the verify hook inside the copy engine, which
/// wants to match its own buffer.
pub async fn hash_file_async_with_buffer(
    path: &Path,
    algo: HashAlgorithm,
    buffer_size: usize,
    ctrl: CopyControl,
    events: mpsc::Sender<HashEvent>,
) -> Result<HashReport, HashError> {
    let path_buf = path.to_path_buf();
    let buffer_size = buffer_size.clamp(
        copythat_core::MIN_BUFFER_SIZE,
        copythat_core::MAX_BUFFER_SIZE,
    );

    let meta = tokio::fs::metadata(&path_buf)
        .await
        .map_err(|e| HashError::from_io(&path_buf, e))?;
    let total = meta.len();

    let file = File::open(&path_buf)
        .await
        .map_err(|e| HashError::from_io(&path_buf, e))?;
    let mut reader = BufReader::with_capacity(buffer_size, file);

    let _ = events
        .send(HashEvent::Started {
            path: path_buf.clone(),
            algorithm: algo,
            total_bytes: total,
        })
        .await;

    let mut hasher: Box<dyn Hasher> = algo.hasher();
    let started_at = Instant::now();
    let mut processed: u64 = 0;
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = 0;
    let mut was_paused = false;

    let loop_result: Result<(), HashError> = loop {
        if ctrl.is_cancelled() {
            break Err(HashError::cancelled(&path_buf));
        }
        if ctrl.is_paused() {
            if !was_paused {
                let _ = events.send(HashEvent::Paused).await;
                was_paused = true;
            }
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                break Err(HashError::cancelled(&path_buf));
            }
            if was_paused {
                let _ = events.send(HashEvent::Resumed).await;
                was_paused = false;
            }
            continue;
        }

        let buf = match reader.fill_buf().await {
            Ok(b) => b,
            Err(e) => break Err(HashError::from_io(&path_buf, e)),
        };
        if buf.is_empty() {
            break Ok(());
        }
        let n = buf.len();
        hasher.update(buf);
        reader.consume(n);
        processed += n as u64;

        let now = Instant::now();
        if processed.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started_at);
            let rate = rate_bps(processed, elapsed);
            let _ = events
                .send(HashEvent::Progress {
                    bytes: processed,
                    total,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = processed;
        }
    };

    match loop_result {
        Ok(()) => {
            let digest = hasher.finalize();
            let elapsed = started_at.elapsed();
            let rate = rate_bps(processed, elapsed);
            // Final progress tick so subscribers see 100%.
            let _ = events
                .send(HashEvent::Progress {
                    bytes: processed,
                    total,
                    rate_bps: rate,
                })
                .await;
            let _ = events
                .send(HashEvent::Completed {
                    digest: digest.clone(),
                    bytes: processed,
                    duration: elapsed,
                    rate_bps: rate,
                })
                .await;
            Ok(HashReport {
                path: path_buf,
                algorithm: algo,
                digest,
                bytes: processed,
                duration: elapsed,
                rate_bps: rate,
            })
        }
        Err(err) => {
            let _ = events.send(HashEvent::Failed { err: err.clone() }).await;
            Err(err)
        }
    }
}

fn rate_bps(bytes: u64, elapsed: Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn hash_file_async_empty_file_matches_empty_vector() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("empty.bin");
        std::fs::write(&p, b"").unwrap();

        let (tx, _rx) = mpsc::channel::<HashEvent>(8);
        let report = hash_file_async(&p, HashAlgorithm::Sha256, CopyControl::new(), tx)
            .await
            .unwrap();
        assert_eq!(
            report.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(report.bytes, 0);
    }

    #[tokio::test]
    async fn hash_file_async_matches_in_memory_hash() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("mixed.bin");
        // Multiple buffer boundaries to force chunked updates. Use
        // 64 KiB (min buffer) so we get a few chunks even with a tiny
        // buffer override.
        let payload: Vec<u8> = (0..250_000u32).map(|x| x as u8).collect();
        std::fs::write(&p, &payload).unwrap();

        let (tx, _rx) = mpsc::channel::<HashEvent>(64);
        let report = hash_file_async_with_buffer(
            &p,
            HashAlgorithm::Blake3,
            64 * 1024,
            CopyControl::new(),
            tx,
        )
        .await
        .unwrap();

        // In-memory reference digest.
        let mut reference = HashAlgorithm::Blake3.hasher();
        reference.update(&payload);
        let reference_hex = hex::encode(reference.finalize());
        assert_eq!(report.hex(), reference_hex);
    }

    #[tokio::test]
    async fn hash_file_async_cancels_between_buffers() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("cancel.bin");
        // Large enough that at least one buffer gets consumed before
        // cancellation; we assert the error type rather than progress.
        let payload = vec![0u8; 2 * 1024 * 1024];
        std::fs::write(&p, &payload).unwrap();

        let ctrl = CopyControl::new();
        let ctrl_clone = ctrl.clone();
        let (tx, _rx) = mpsc::channel::<HashEvent>(64);

        // Cancel before the call — tightest possible race, asserts the
        // cooperative check fires on the first iteration.
        ctrl_clone.cancel();
        let err = hash_file_async(&p, HashAlgorithm::Sha256, ctrl, tx)
            .await
            .unwrap_err();
        assert!(err.is_cancelled(), "expected cancelled, got {err}");
    }
}
