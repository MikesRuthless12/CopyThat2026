//! Side-by-side verification helpers.
//!
//! `verify_pair` hashes two files with the same algorithm and reports
//! whether they match. The copy engine's verify hook uses a more
//! targeted path (it hashes the source during the copy itself to avoid
//! a re-read), but this helper is what a caller uses when they already
//! have a source and destination on disk — e.g. verifying an older
//! copy, or a "re-verify from sidecar" user action.

use std::path::Path;

use copythat_core::CopyControl;
use tokio::sync::mpsc;

use crate::algorithm::HashAlgorithm;
use crate::error::HashError;
use crate::event::HashEvent;
use crate::streaming::hash_file_async;

/// Result of a paired hash comparison.
#[derive(Debug, Clone)]
pub struct VerifyOutcome {
    pub algorithm: HashAlgorithm,
    pub src_digest: Vec<u8>,
    pub dst_digest: Vec<u8>,
    pub matched: bool,
}

impl VerifyOutcome {
    pub fn src_hex(&self) -> String {
        hex::encode(&self.src_digest)
    }
    pub fn dst_hex(&self) -> String {
        hex::encode(&self.dst_digest)
    }
}

/// Hash `src` and `dst` in parallel, return the two digests and
/// whether they match. Events from both streams are multiplexed onto
/// `events`. Cancellation on either stream aborts both (via the shared
/// `CopyControl`).
pub async fn verify_pair(
    src: &Path,
    dst: &Path,
    algorithm: HashAlgorithm,
    ctrl: CopyControl,
    events: mpsc::Sender<HashEvent>,
) -> Result<VerifyOutcome, HashError> {
    let src_path = src.to_path_buf();
    let dst_path = dst.to_path_buf();
    let ctrl_src = ctrl.clone();
    let ctrl_dst = ctrl.clone();
    let events_src = events.clone();
    let events_dst = events;

    let src_task =
        tokio::spawn(
            async move { hash_file_async(&src_path, algorithm, ctrl_src, events_src).await },
        );
    let dst_task =
        tokio::spawn(
            async move { hash_file_async(&dst_path, algorithm, ctrl_dst, events_dst).await },
        );

    let (src_res, dst_res) = tokio::join!(src_task, dst_task);
    // Panics in the spawned tasks should bubble up as an IoOther-style
    // error; unwrapping would swallow the cancellation path.
    let src_report = match src_res {
        Ok(inner) => inner,
        Err(_) => {
            // If one side panicked, cancel the other.
            ctrl.cancel();
            return Err(HashError::cancelled(src));
        }
    }?;
    let dst_report = match dst_res {
        Ok(inner) => inner,
        Err(_) => {
            ctrl.cancel();
            return Err(HashError::cancelled(dst));
        }
    }?;

    let matched = src_report.digest == dst_report.digest;
    Ok(VerifyOutcome {
        algorithm,
        src_digest: src_report.digest,
        dst_digest: dst_report.digest,
        matched,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn verify_pair_matches_identical_files() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.bin");
        let b = dir.path().join("b.bin");
        let payload = vec![0xABu8; 4096];
        tokio::fs::write(&a, &payload).await.unwrap();
        tokio::fs::write(&b, &payload).await.unwrap();

        let (tx, _) = mpsc::channel::<HashEvent>(64);
        let outcome = verify_pair(&a, &b, HashAlgorithm::Sha256, CopyControl::new(), tx)
            .await
            .unwrap();
        assert!(outcome.matched);
        assert_eq!(outcome.src_hex(), outcome.dst_hex());
    }

    #[tokio::test]
    async fn verify_pair_detects_mismatch() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.bin");
        let b = dir.path().join("b.bin");
        tokio::fs::write(&a, b"one").await.unwrap();
        tokio::fs::write(&b, b"two").await.unwrap();

        let (tx, _) = mpsc::channel::<HashEvent>(64);
        let outcome = verify_pair(&a, &b, HashAlgorithm::Sha256, CopyControl::new(), tx)
            .await
            .unwrap();
        assert!(!outcome.matched);
        assert_ne!(outcome.src_hex(), outcome.dst_hex());
    }
}
