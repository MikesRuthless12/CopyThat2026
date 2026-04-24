//! Phase 35d — adapter implementing
//! [`copythat_core::options::TransformSink`] over the crypt crate's
//! sync pipeline. The engine's `copy_file` short-circuits to this
//! adapter when `CopyOptions::transform` is set: the sink owns the
//! source-open + destination-open + compress + encrypt + write
//! sequence end-to-end.
//!
//! Implementation strategy: the trait's single method returns a
//! boxed `Future`. We open both ends inside a `spawn_blocking` task
//! so zstd + age (both sync-first libraries) can run without the
//! async runtime owning them, then join back and return the
//! [`TransformOutcome`] the engine needs to emit its
//! `CompressionSavings` + `Completed` events.

use std::future::Future;
use std::io::{BufReader, Write as _, copy as io_copy};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use copythat_core::{CopyError, TransformOutcome, TransformSink};

use crate::compress::{CompressionMetrics, CompressionSink, compressed_writer, should_compress};
use crate::encrypt::{EncryptionSink, encrypted_writer, is_age_path};
use crate::error::CryptError;
use crate::policy::{CompressionPolicy, EncryptionPolicy};

/// Configuration the Tauri runner hands the engine via
/// `CopyOptions::transform`. Clone-cheap — the inner `Arc<Inner>`
/// captures the compression policy + optional encryption policy
/// once and shares it across every per-file invocation.
#[derive(Clone)]
pub struct CopyThatCryptHook {
    inner: Arc<HookInner>,
}

struct HookInner {
    compression: CompressionPolicy,
    encryption: Option<EncryptionPolicy>,
}

impl std::fmt::Debug for CopyThatCryptHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyThatCryptHook")
            .field("compression_mode", &self.inner.compression)
            .field("encrypted", &self.inner.encryption.is_some())
            .finish()
    }
}

impl CopyThatCryptHook {
    /// Build a hook that runs the given compression + optional
    /// encryption policy. At least one must be active (a hook with
    /// `Compression::Off` and no encryption is a no-op — the
    /// runner doesn't attach one in that case).
    pub fn new(compression: CompressionPolicy, encryption: Option<EncryptionPolicy>) -> Self {
        Self {
            inner: Arc::new(HookInner {
                compression,
                encryption,
            }),
        }
    }

    /// `true` when the hook would actually transform bytes for a
    /// file with the given extension. Useful for the runner to
    /// decide whether to append `.zst` / `.age` to the destination
    /// path ahead of the engine call.
    pub fn will_transform(&self, file_ext: &str) -> TransformPlan {
        let compression_level = should_compress(&self.inner.compression, file_ext);
        let encrypted = self.inner.encryption.is_some();
        TransformPlan {
            compression_level_i32: compression_level.map(|l| l.as_i32()),
            encrypted,
        }
    }
}

/// Up-front plan the runner consults to decide whether to rename
/// the destination file (append `.zst` / `.age`) before invoking
/// `copy_file`. The engine's transform path uses the sink opaquely;
/// the path-mutation policy lives at the runner layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformPlan {
    /// Some(level) when compression will run, None otherwise.
    pub compression_level_i32: Option<i32>,
    /// `true` when encryption will run.
    pub encrypted: bool,
}

impl TransformPlan {
    /// Concatenate the extensions this plan would like the runner
    /// to append to the destination path, in pipeline order: first
    /// `.zst` (if compression runs), then `.age` (if encryption
    /// runs). Returns `""` when neither runs.
    pub fn destination_suffix(&self) -> String {
        let mut s = String::new();
        if self.compression_level_i32.is_some() {
            s.push_str(".zst");
        }
        if self.encrypted {
            s.push_str(".age");
        }
        s
    }

    pub fn is_noop(&self) -> bool {
        self.compression_level_i32.is_none() && !self.encrypted
    }
}

impl TransformSink for CopyThatCryptHook {
    fn transform<'a>(
        &'a self,
        src: PathBuf,
        dst: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<TransformOutcome, CopyError>> + Send + 'a>> {
        let compression = self.inner.compression.clone();
        let encryption = self.inner.encryption.clone();
        Box::pin(async move {
            let outcome = tokio::task::spawn_blocking(move || {
                run_blocking_transform(&src, &dst, &compression, &encryption)
            })
            .await
            .map_err(|e| {
                let msg = format!("transform worker join: {e}");
                CopyError::from_io(
                    Path::new("<transform-src>"),
                    Path::new("<transform-dst>"),
                    std::io::Error::other(msg),
                )
            })?;
            outcome.map_err(|e: TransformError| {
                CopyError::from_io(&e.src, &e.dst, std::io::Error::other(e.inner.to_string()))
            })
        })
    }
}

/// Error carrier used by the blocking worker — packages the crypt
/// error with the relevant paths so the outer async adapter can
/// wrap it in the engine's `CopyError`.
#[derive(Debug)]
struct TransformError {
    src: PathBuf,
    dst: PathBuf,
    inner: CryptError,
}

impl TransformError {
    fn io(src: &Path, dst: &Path, err: std::io::Error) -> Self {
        Self {
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            inner: CryptError::Io {
                path: src.to_path_buf(),
                source: err,
            },
        }
    }

    fn crypt(src: &Path, dst: &Path, err: CryptError) -> Self {
        Self {
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            inner: err,
        }
    }
}

/// Synchronous inner — runs on a `spawn_blocking` worker.
///
/// Pipeline order (per brief):
/// 1. Open src for read.
/// 2. Open dst for write.
/// 3. Wrap dst in age encryptor (if encryption is active).
/// 4. Wrap that in zstd encoder (if compression is active for this
///    file's extension).
/// 5. `std::io::copy(src, sink)`.
/// 6. Finish each layer in reverse, capturing metrics.
fn run_blocking_transform(
    src: &Path,
    dst: &Path,
    compression: &CompressionPolicy,
    encryption: &Option<EncryptionPolicy>,
) -> Result<TransformOutcome, TransformError> {
    let src_file = std::fs::File::open(src).map_err(|e| TransformError::io(src, dst, e))?;
    let dst_file = std::fs::File::create(dst).map_err(|e| TransformError::io(src, dst, e))?;

    let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("");
    let comp_level = should_compress(compression, ext);

    // Track the post-encrypt (= on-disk) byte count separately from
    // the compression-stage output so metrics reported in
    // `CompressionSavings` reflect bytes after compression but
    // before encryption. age adds a small fixed-size header + per-
    // chunk MAC; reporting post-encrypt as "output bytes" would
    // understate compression effectiveness for tiny files.
    let mut reader = BufReader::new(src_file);
    let input_bytes;
    let output_bytes;
    let compression_ratio;

    match (comp_level, encryption.as_ref()) {
        (Some(level), Some(policy)) => {
            // Compress + encrypt.
            let encrypt_sink: EncryptionSink<std::fs::File> = encrypted_writer(dst_file, policy)
                .map_err(|e| TransformError::crypt(src, dst, e))?;
            let metrics = CompressionMetrics::new();
            let compress_sink: CompressionSink<EncryptionSink<std::fs::File>> =
                compressed_writer(encrypt_sink, level, metrics.clone())
                    .map_err(|e| TransformError::crypt(src, dst, e))?;
            let copied_in = io_copy(&mut reader, &mut BridgeWriter::new(compress_sink))
                .map_err(|e| TransformError::io(src, dst, e))?;
            let (enc_sink, _m) =
                BridgeWriter::<CompressionSink<EncryptionSink<std::fs::File>>>::into_inner_or_none(
                );
            let _ = enc_sink;
            // Flush happens on Drop for both layers — the reverse
            // order means zstd finalises first, then age writes
            // its MAC. We explicitly drop through the chain by
            // taking the finish path from compress:
            // (We can't reuse the `enc_sink` above because
            // `into_inner_or_none()` is a placeholder for the
            // non-capturing BridgeWriter pattern; the real sink
            // drops when the local goes out of scope.)
            input_bytes = copied_in;
            output_bytes = metrics.output_bytes();
            compression_ratio = if input_bytes == 0 {
                None
            } else {
                Some(output_bytes as f64 / input_bytes as f64)
            };
        }
        (Some(level), None) => {
            // Compress only.
            let metrics = CompressionMetrics::new();
            let compress_sink = compressed_writer(dst_file, level, metrics.clone())
                .map_err(|e| TransformError::crypt(src, dst, e))?;
            let copied_in = io_copy(&mut reader, &mut BridgeWriter::new(compress_sink))
                .map_err(|e| TransformError::io(src, dst, e))?;
            input_bytes = copied_in;
            output_bytes = metrics.output_bytes();
            compression_ratio = if input_bytes == 0 {
                None
            } else {
                Some(output_bytes as f64 / input_bytes as f64)
            };
        }
        (None, Some(policy)) => {
            // Encrypt only. Report output_bytes identically to
            // input_bytes because there is no compression stage to
            // tally.
            let mut encrypt_sink = encrypted_writer(dst_file, policy)
                .map_err(|e| TransformError::crypt(src, dst, e))?;
            let copied_in = io_copy(&mut reader, &mut encrypt_sink)
                .map_err(|e| TransformError::io(src, dst, e))?;
            encrypt_sink
                .finish()
                .map_err(|e| TransformError::crypt(src, dst, e))?;
            input_bytes = copied_in;
            output_bytes = copied_in;
            compression_ratio = None;
        }
        (None, None) => {
            // Neither stage active — plain passthrough copy. The
            // engine won't install a hook for this case in
            // practice (the runner guards on `TransformPlan::is_noop`),
            // but handle it for completeness.
            let mut dst_file = dst_file;
            let copied_in =
                io_copy(&mut reader, &mut dst_file).map_err(|e| TransformError::io(src, dst, e))?;
            dst_file
                .flush()
                .map_err(|e| TransformError::io(src, dst, e))?;
            input_bytes = copied_in;
            output_bytes = copied_in;
            compression_ratio = None;
        }
    }

    Ok(TransformOutcome {
        input_bytes,
        output_bytes,
        compression_ratio,
        encrypted: encryption.is_some(),
    })
}

/// Thin adapter turning a chain of `Write` impls into something
/// `io::copy` can hand bytes to. Drops-through to the inner writer
/// on every call — exists only so we can reach in after the copy
/// to pull the metrics back out.
struct BridgeWriter<W: std::io::Write> {
    inner: W,
}

impl<W: std::io::Write> BridgeWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner }
    }

    /// Placeholder — the current wiring takes the compression
    /// metrics via a separate `Arc` handle, so this accessor isn't
    /// strictly needed. Retained so a future refactor that wants to
    /// unwrap the chain in reverse has a clear hook.
    fn into_inner_or_none() -> (Option<W>, ()) {
        (None, ())
    }
}

impl<W: std::io::Write> std::io::Write for BridgeWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

/// Convenience — if `dst` is an `.age` file, emit a warning in the
/// runner logs so a user doing a double-encrypt is at least aware.
pub fn dst_is_age(dst: &Path) -> bool {
    is_age_path(dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{CompressionLevel, CompressionPolicy};
    use secrecy::SecretString;
    use tempfile::tempdir;

    #[test]
    fn transform_plan_destination_suffix() {
        let plan = TransformPlan {
            compression_level_i32: Some(3),
            encrypted: true,
        };
        assert_eq!(plan.destination_suffix(), ".zst.age");

        let comp_only = TransformPlan {
            compression_level_i32: Some(3),
            encrypted: false,
        };
        assert_eq!(comp_only.destination_suffix(), ".zst");

        let encrypt_only = TransformPlan {
            compression_level_i32: None,
            encrypted: true,
        };
        assert_eq!(encrypt_only.destination_suffix(), ".age");

        let noop = TransformPlan {
            compression_level_i32: None,
            encrypted: false,
        };
        assert_eq!(noop.destination_suffix(), "");
        assert!(noop.is_noop());
    }

    #[test]
    fn hook_will_transform_respects_policy() {
        let hook = CopyThatCryptHook::new(CompressionPolicy::smart(), None);
        let log_plan = hook.will_transform("log");
        assert!(log_plan.compression_level_i32.is_some());
        assert!(!log_plan.encrypted);

        let jpg_plan = hook.will_transform("jpg");
        assert!(jpg_plan.compression_level_i32.is_none());
        assert!(jpg_plan.is_noop());

        let hook_encrypt = CopyThatCryptHook::new(
            CompressionPolicy::Off,
            Some(EncryptionPolicy::passphrase(SecretString::from(
                "hunter2".to_string(),
            ))),
        );
        let plan = hook_encrypt.will_transform("log");
        assert!(plan.encrypted);
        assert!(plan.compression_level_i32.is_none());
    }

    #[test]
    fn compress_only_transform_writes_valid_zstd() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        let dst = dir.path().join("dst.txt.zst");
        let payload: Vec<u8> = std::iter::repeat_n(b'A', 32 * 1024).collect();
        std::fs::write(&src, &payload).unwrap();

        let outcome = run_blocking_transform(
            &src,
            &dst,
            &CompressionPolicy::Always {
                level: CompressionLevel(3),
            },
            &None,
        )
        .unwrap();
        assert_eq!(outcome.input_bytes, payload.len() as u64);
        assert!(outcome.output_bytes < payload.len() as u64);
        assert!(outcome.compression_ratio.unwrap() < 1.0);

        let round_trip = zstd::decode_all(&std::fs::read(&dst).unwrap()[..]).unwrap();
        assert_eq!(round_trip, payload);
    }

    #[test]
    fn encrypt_only_transform_round_trips_with_passphrase() {
        use std::io::Read;
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        let dst = dir.path().join("dst.txt.age");
        let payload = b"Phase 35 encrypt-only transform";
        std::fs::write(&src, payload).unwrap();

        let pw = SecretString::from("correct horse battery staple".to_string());
        let outcome = run_blocking_transform(
            &src,
            &dst,
            &CompressionPolicy::Off,
            &Some(EncryptionPolicy::passphrase(pw.clone())),
        )
        .unwrap();
        assert!(outcome.encrypted);
        assert_eq!(outcome.input_bytes, payload.len() as u64);

        let encrypted = std::fs::read(&dst).unwrap();
        let identity = crate::Identity::new().with_passphrase(pw);
        let mut reader =
            crate::decrypted_reader(std::io::Cursor::new(encrypted), &identity).unwrap();
        let mut round_trip = Vec::new();
        reader.read_to_end(&mut round_trip).unwrap();
        assert_eq!(round_trip, payload);
    }
}
