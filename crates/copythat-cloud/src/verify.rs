//! Phase 32f — verify-on-remote.
//!
//! After a successful cloud upload, compute the client-side hash of
//! the local source + compare against whatever the remote backend
//! exposes — either a server-reported ETag/content-md5 on S3-class
//! backends, or (fall-back) a round-trip `get` + re-hash on the
//! client.
//!
//! # Why the round-trip fallback?
//!
//! Not every OpenDAL backend returns a content checksum in `stat`
//! metadata. SFTP, WebDAV, FTP, OneDrive, Google Drive, Dropbox all
//! lack a standard checksum header. For those the only honest way
//! to verify is to pull the blob back and hash it locally — which
//! doubles network bandwidth but catches corruption that any other
//! path would miss.
//!
//! # What this module ships
//!
//! - [`VerifyAlgorithm`] — algorithm selector (`Blake3` / `Sha256`).
//!   Matches the Phase 3 verify pipeline's algorithm list where
//!   possible; remote-side ETags + MD5s are coerced through the
//!   same comparator.
//! - [`verify_upload`] — public entry point. Takes a local source
//!   path + the `Arc<dyn CopyTarget>` + the dst_key + the chosen
//!   algorithm + an `on_progress` callback + a
//!   `round_trip_allowed` flag. Returns a [`VerifyOutcome`]
//!   describing what happened (matched, mismatched, or unverified
//!   because the backend doesn't expose a comparable checksum and
//!   round-trip was disabled).

use std::path::Path;
use std::sync::Arc;

use bytes::Bytes;
use thiserror::Error;
use tokio::io::AsyncReadExt;

use crate::error::BackendError;
use crate::target::CopyTarget;

/// Verify algorithm. [`VerifyAlgorithm::Blake3`] is the default —
/// faster than SHA-256 and matches Phase 3's preferred verify
/// algo. [`VerifyAlgorithm::Sha256`] is kept for backends whose
/// server-side checksum is SHA-256 (GCS default, Azure Blob when
/// MD5 is disabled). [`VerifyAlgorithm::Md5`] (Phase 32h) is
/// specifically for the S3 / Azure ETag server-side-checksum
/// fast-path; not cryptographically safe on its own, only safe
/// here because corruption detection is the goal, not security.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyAlgorithm {
    Blake3,
    Sha256,
    Md5,
}

impl VerifyAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blake3 => "blake3",
            Self::Sha256 => "sha256",
            Self::Md5 => "md5",
        }
    }
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("local source read failed: {0}")]
    SourceRead(String),
    #[error("remote get failed: {0}")]
    RemoteGet(#[from] BackendError),
    #[error("hash mismatch: local={local_hex}, remote={remote_hex}")]
    Mismatch { local_hex: String, remote_hex: String },
}

/// What the verify step actually produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// Client-side hash matched the remote checksum / round-tripped
    /// bytes. `source` identifies which comparator path succeeded so
    /// the UI can render "verified via ETag" vs "verified via
    /// round-trip" badges distinctly.
    Matched {
        algorithm: VerifyAlgorithm,
        local_hex: String,
        source: VerifySource,
    },
    /// Backend doesn't expose a server-side checksum in `stat`, and
    /// round-trip verification was disabled. The upload's
    /// integrity past the local-network transport layer is
    /// untested.
    Unverified {
        algorithm: VerifyAlgorithm,
        reason: String,
    },
}

/// Which comparator leg confirmed the match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifySource {
    /// Server-reported ETag (S3-class / Azure Blob).
    Etag,
    /// Server-reported `content-md5` header.
    ContentMd5,
    /// Round-trip fetch + client-side re-hash.
    RoundTrip,
}

/// Verify that `src_path`'s contents match the `dst_key` object on
/// `target`. Runs the algorithm over the local source while
/// streaming it through the reader; then either compares against
/// the server-side checksum from `stat`, or (if
/// `round_trip_allowed` is true and stat has no checksum) fetches
/// the object back and re-hashes it client-side.
///
/// Progress callback fires per-chunk with the running local-hash
/// byte count.
pub async fn verify_upload(
    src_path: &Path,
    target: &Arc<dyn CopyTarget>,
    dst_key: &str,
    algorithm: VerifyAlgorithm,
    round_trip_allowed: bool,
    on_progress: &(dyn Fn(u64) + Send + Sync),
) -> Result<VerifyOutcome, VerifyError> {
    // 1. Hash the local source.
    let local_hex = hash_local(src_path, algorithm, on_progress).await?;

    // 2. Phase 32h — server-side checksum fast-path. Fires when
    //    `algorithm == Md5` AND the backend's `stat` exposed a
    //    server-side ETag or content-md5 header. S3 single-part
    //    uploads write `etag = "<hex md5>"`; Azure Blob writes
    //    `content-md5 = "<base64 md5>"`. Multipart S3 uploads
    //    produce an `md5-of-md5s-<nparts>` suffix we can't
    //    reconcile client-side — we detect the hyphen and fall
    //    through to the round-trip path in that case.
    let meta = target.stat(dst_key).await?;
    if matches!(algorithm, VerifyAlgorithm::Md5)
        && let Some(meta) = meta.as_ref()
    {
        // Prefer the content-md5 header when both are set — it's
        // backend-independent (S3 sometimes reports a non-MD5
        // ETag for encrypted objects).
        let candidate = meta.content_md5.clone().or_else(|| meta.etag.clone());
        if let Some(raw) = candidate {
            let normalized = raw.trim().trim_matches('"');
            // MPU suffix bails.
            if !normalized.contains('-') && !normalized.is_empty() {
                // Normalise to lowercase hex. Azure reports
                // base64 MD5; S3 reports hex. If it's 32 chars
                // and alphanumeric, it's already hex. Otherwise
                // try base64 → hex.
                let candidate_hex = if normalized.len() == 32
                    && normalized.chars().all(|c| c.is_ascii_hexdigit())
                {
                    normalized.to_ascii_lowercase()
                } else {
                    use base64::Engine;
                    match base64::engine::general_purpose::STANDARD.decode(normalized) {
                        Ok(bytes) if bytes.len() == 16 => hex::encode(bytes),
                        _ => String::new(),
                    }
                };
                if !candidate_hex.is_empty() && candidate_hex == local_hex {
                    let source = if meta.content_md5.is_some() {
                        VerifySource::ContentMd5
                    } else {
                        VerifySource::Etag
                    };
                    return Ok(VerifyOutcome::Matched {
                        algorithm,
                        local_hex,
                        source,
                    });
                } else if !candidate_hex.is_empty() {
                    // Authoritative mismatch — server reported a
                    // concrete MD5 and it doesn't match ours.
                    return Err(VerifyError::Mismatch {
                        local_hex,
                        remote_hex: candidate_hex,
                    });
                }
                // Couldn't decode the candidate as MD5 — fall
                // through to round-trip.
            }
        }
    }

    // 3. Fallback: round-trip the bytes and re-hash.
    if !round_trip_allowed {
        return Ok(VerifyOutcome::Unverified {
            algorithm,
            reason: "remote exposes no comparable checksum; round-trip disabled".into(),
        });
    }

    let pulled = target.get(dst_key).await?;
    let remote_hex = hash_bytes(&pulled, algorithm);
    if remote_hex == local_hex {
        Ok(VerifyOutcome::Matched {
            algorithm,
            local_hex,
            source: VerifySource::RoundTrip,
        })
    } else {
        Err(VerifyError::Mismatch {
            local_hex,
            remote_hex,
        })
    }
}

/// Stream `src_path` through the chosen algorithm, reporting
/// progress after each chunk.
async fn hash_local(
    src_path: &Path,
    algorithm: VerifyAlgorithm,
    on_progress: &(dyn Fn(u64) + Send + Sync),
) -> Result<String, VerifyError> {
    use sha2::Digest;
    let mut file = tokio::fs::File::open(src_path)
        .await
        .map_err(|e| VerifyError::SourceRead(format!("open {}: {e}", src_path.display())))?;
    const BUF_SIZE: usize = 1024 * 1024;
    let mut buf = vec![0u8; BUF_SIZE];
    let mut total: u64 = 0;

    match algorithm {
        VerifyAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let n = file
                    .read(&mut buf)
                    .await
                    .map_err(|e| VerifyError::SourceRead(format!("read: {e}")))?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
                total += n as u64;
                on_progress(total);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }
        VerifyAlgorithm::Sha256 => {
            // Phase 32g — real SHA-256 via the `sha2` crate. Matches
            // the server-side checksum algorithm GCS reports in
            // `Metadata::content_md5` on CRC32C-negotiated buckets
            // (actually a SHA-256 for some object classes; content-
            // md5 is the common field name).
            let mut hasher = sha2::Sha256::new();
            loop {
                let n = file
                    .read(&mut buf)
                    .await
                    .map_err(|e| VerifyError::SourceRead(format!("read: {e}")))?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
                total += n as u64;
                on_progress(total);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        VerifyAlgorithm::Md5 => {
            // Phase 32h — MD5 for the ETag server-side-checksum
            // fast-path. S3 single-part objects report ETag as hex
            // MD5 of the object bytes; Azure Blob reports
            // `content-md5` as base64 MD5. Either way, the
            // comparator needs a client-side MD5.
            use md5::Digest;
            let mut hasher = md5::Md5::new();
            loop {
                let n = file
                    .read(&mut buf)
                    .await
                    .map_err(|e| VerifyError::SourceRead(format!("read: {e}")))?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
                total += n as u64;
                on_progress(total);
            }
            Ok(hex::encode(hasher.finalize()))
        }
    }
}

fn hash_bytes(bytes: &Bytes, algorithm: VerifyAlgorithm) -> String {
    match algorithm {
        VerifyAlgorithm::Blake3 => blake3::hash(bytes).to_hex().to_string(),
        VerifyAlgorithm::Sha256 => {
            use sha2::Digest;
            let mut hasher = sha2::Sha256::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        VerifyAlgorithm::Md5 => {
            use md5::Digest;
            let mut hasher = md5::Md5::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{Backend, BackendConfig, BackendKind, LocalFsConfig, make_operator};
    use crate::target::OperatorTarget;

    fn local_target(root: &Path) -> Arc<dyn CopyTarget> {
        let backend = Backend {
            name: "local".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: root.to_string_lossy().into_owned(),
            }),
        };
        let op = make_operator(&backend, None).expect("operator");
        Arc::new(OperatorTarget::new("local", op))
    }

    #[tokio::test]
    async fn verify_matched_round_trip() {
        let src_dir = tempfile::tempdir().expect("src");
        let remote = tempfile::tempdir().expect("remote");
        let src = src_dir.path().join("a.bin");
        let payload: Vec<u8> = (0..4096).map(|i| (i % 251) as u8).collect();
        tokio::fs::write(&src, &payload).await.expect("seed");

        let target = local_target(remote.path());
        target
            .put("a.bin", Bytes::from(payload.clone()))
            .await
            .expect("upload");

        let outcome = verify_upload(
            &src,
            &target,
            "a.bin",
            VerifyAlgorithm::Blake3,
            true,
            &|_| {},
        )
        .await
        .expect("verify");
        assert!(matches!(outcome, VerifyOutcome::Matched { .. }));
    }

    #[tokio::test]
    async fn verify_mismatch_returns_typed_error() {
        let src_dir = tempfile::tempdir().expect("src");
        let remote = tempfile::tempdir().expect("remote");
        let src = src_dir.path().join("a.bin");
        tokio::fs::write(&src, b"local bytes").await.expect("seed");

        // Upload different bytes to the same key.
        let target = local_target(remote.path());
        target
            .put("a.bin", Bytes::from_static(b"remote differs"))
            .await
            .expect("upload");

        let err = verify_upload(
            &src,
            &target,
            "a.bin",
            VerifyAlgorithm::Blake3,
            true,
            &|_| {},
        )
        .await
        .expect_err("must mismatch");
        assert!(matches!(err, VerifyError::Mismatch { .. }));
    }

    #[tokio::test]
    async fn verify_unverified_when_round_trip_disabled() {
        let src_dir = tempfile::tempdir().expect("src");
        let remote = tempfile::tempdir().expect("remote");
        let src = src_dir.path().join("a.bin");
        tokio::fs::write(&src, b"hello").await.expect("seed");
        let target = local_target(remote.path());
        target
            .put("a.bin", Bytes::from_static(b"hello"))
            .await
            .expect("upload");
        let outcome = verify_upload(
            &src,
            &target,
            "a.bin",
            VerifyAlgorithm::Blake3,
            false,
            &|_| {},
        )
        .await
        .expect("verify");
        assert!(matches!(outcome, VerifyOutcome::Unverified { .. }));
    }

    #[test]
    fn algorithm_wire_strings_stable() {
        assert_eq!(VerifyAlgorithm::Blake3.as_str(), "blake3");
        assert_eq!(VerifyAlgorithm::Sha256.as_str(), "sha256");
    }
}
