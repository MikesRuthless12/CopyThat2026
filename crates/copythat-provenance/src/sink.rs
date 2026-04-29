//! [`CopyThatProvenanceSink`] — the
//! [`copythat_core::ProvenanceSink`] implementation the engine talks
//! to.
//!
//! Lifecycle:
//! 1. Caller constructs the sink with [`SinkConfig`] (src_root,
//!    dst_root, signing key, encoder mode).
//! 2. Caller wraps the sink in `Arc::new` and stashes it in
//!    [`copythat_core::ProvenancePolicy`], which lives on
//!    [`copythat_core::CopyOptions::provenance`].
//! 3. The engine, per file, calls
//!    [`CopyThatProvenanceSink::make_encoder`] +
//!    [`CopyThatProvenanceSink::record_file`] to feed records into
//!    the per-job state.
//! 4. After the tree-walk, the caller invokes
//!    [`CopyThatProvenanceSink::finalize_to_path`] to build the
//!    canonical CBOR manifest, sign it (when a key was provided),
//!    optionally request a TSA timestamp (when the `tsa` feature is
//!    enabled), and write the result to disk.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use copythat_core::{OutboardEncoder, ProvenanceSink};
use ed25519_dalek::Signer;

use crate::encoder::{BaoOutboardEncoder, RootOnlyEncoder};
use crate::error::{ProvenanceError, ProvenanceErrorKind};
use crate::manifest::{
    FileRecord, ProvenanceManifest, Rfc3161Token, Signature, manifest_root_blake3,
    manifest_signing_bytes, validate_rel_path, write_manifest_cbor,
};
use crate::sign::SigningKey;

/// Per-job sink configuration. Built by the caller (CLI / UI / test)
/// and passed to [`CopyThatProvenanceSink::new`].
pub struct SinkConfig {
    /// Source root the job is copying from. Used to derive each
    /// [`FileRecord`]'s `rel_path`.
    pub src_root: PathBuf,
    /// Destination root the job is copying to. Recorded in the
    /// manifest verbatim.
    pub dst_root: PathBuf,
    /// Optional signing key. When `Some`, the sink signs the
    /// manifest with this key during `finalize_to_path`.
    pub signing_key: Option<SigningKey>,
    /// Optional TSA URL. Set to request an RFC 3161 timestamp from
    /// this URL during `finalize_to_path`. The `tsa` Cargo feature
    /// must be enabled at compile time for the request to actually
    /// fire; otherwise `finalize_to_path` returns
    /// [`ProvenanceErrorKind::TsaFeatureDisabled`].
    pub tsa_url: Option<String>,
    /// Encoder mode for per-file outboards. `true` (the default)
    /// uses [`BaoOutboardEncoder`]; `false` uses [`RootOnlyEncoder`]
    /// (skipping the outboard saves memory at the cost of partial-
    /// byte-range verification support).
    pub bao_outboards: bool,
    /// Hostname recorded in the manifest. Defaults to
    /// `hostname::get` at sink construction; callers can override
    /// (useful for offline test fixtures).
    pub host: String,
    /// Username recorded in the manifest. Defaults to
    /// `whoami::username`; callers can override.
    pub user: String,
    /// Copy That version recorded in the manifest. Defaults to the
    /// current crate's `CARGO_PKG_VERSION`.
    pub copythat_version: String,
}

impl SinkConfig {
    /// Construct a default config for the given roots, with no
    /// signing key, no TSA, Bao outboards on, and host/user/version
    /// auto-probed.
    pub fn new(src_root: PathBuf, dst_root: PathBuf) -> Self {
        Self {
            src_root,
            dst_root,
            signing_key: None,
            tsa_url: None,
            bao_outboards: true,
            host: probe_host(),
            user: probe_user(),
            copythat_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Per-job sink. `Arc::new` it once, stash in
/// `CopyOptions::provenance`, run the copy, then call
/// [`Self::finalize_to_path`].
pub struct CopyThatProvenanceSink {
    config: SinkConfig,
    state: Mutex<SinkState>,
}

struct SinkState {
    files: Vec<FileRecord>,
    started_at: chrono::DateTime<Utc>,
}

impl CopyThatProvenanceSink {
    /// Wrap a [`SinkConfig`] into a sink. Records the wall-clock
    /// time as the manifest's `started_at`.
    pub fn new(config: SinkConfig) -> Arc<Self> {
        Arc::new(Self {
            config,
            state: Mutex::new(SinkState {
                files: Vec::new(),
                started_at: Utc::now(),
            }),
        })
    }

    /// Number of files recorded so far. Cheap read; exposed so the
    /// CLI / UI can show a live count.
    pub fn file_count(&self) -> usize {
        self.state.lock().map(|s| s.files.len()).unwrap_or_default()
    }

    /// Drain the accumulated state into a finalised
    /// [`ProvenanceManifest`]. Sorts files by `rel_path`, computes
    /// the Merkle root, signs (when configured), optionally requests
    /// a TSA timestamp, and writes canonical CBOR to `manifest_path`.
    ///
    /// Idempotent at the `Arc<Self>` level: a second call with the
    /// same accumulated state produces a manifest whose
    /// `merkle_root` matches the first (the per-record bytes don't
    /// change). The `job_id` and `signature.sig` fields are fresh
    /// each call though — UUIDs and ed25519 signatures are
    /// non-deterministic by construction.
    pub fn finalize_to_path(
        self: &Arc<Self>,
        manifest_path: &Path,
    ) -> Result<ProvenanceManifest, ProvenanceError> {
        let (files, started_at) = {
            let mut state = self.state.lock().expect("provenance sink state poisoned");
            (std::mem::take(&mut state.files), state.started_at)
        };

        let mut manifest = ProvenanceManifest::new(
            self.config.src_root.clone(),
            self.config.dst_root.clone(),
            started_at,
            Utc::now(),
            self.config.host.clone(),
            self.config.user.clone(),
            self.config.copythat_version.clone(),
            files,
        );

        if let Some(key) = self.config.signing_key.as_ref() {
            let signing_bytes = manifest_signing_bytes(&manifest)?;
            let sig = key.sign(&signing_bytes);
            manifest.signature = Some(Signature {
                public_key: key.verifying_key().to_bytes(),
                sig: sig.to_bytes(),
            });
        }

        if let Some(tsa_url) = self.config.tsa_url.as_ref() {
            let token = request_timestamp(tsa_url, &manifest)?;
            manifest.timestamp = Some(token);
        }

        write_manifest_cbor(&manifest, manifest_path)?;
        Ok(manifest)
    }

    /// Re-compute the Merkle root over the currently-buffered files
    /// without finalizing the manifest. Cheap; useful for live UI
    /// updates ("manifest will hash to: 0xab12…").
    pub fn live_merkle_root(&self) -> [u8; 32] {
        let state = self.state.lock().expect("provenance sink state poisoned");
        let mut sorted = state.files.clone();
        sorted.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
        manifest_root_blake3(&sorted)
    }
}

impl std::fmt::Debug for CopyThatProvenanceSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyThatProvenanceSink")
            .field("src_root", &self.config.src_root)
            .field("dst_root", &self.config.dst_root)
            .field("signing", &self.config.signing_key.is_some())
            .field("tsa_url", &self.config.tsa_url)
            .field("bao_outboards", &self.config.bao_outboards)
            .field("file_count", &self.file_count())
            .finish_non_exhaustive()
    }
}

impl ProvenanceSink for CopyThatProvenanceSink {
    fn make_encoder(&self) -> Box<dyn OutboardEncoder> {
        if self.config.bao_outboards {
            Box::new(BaoOutboardEncoder::new())
        } else {
            Box::new(RootOnlyEncoder::new())
        }
    }

    fn record_file(
        &self,
        src: &Path,
        _dst: &Path,
        size: u64,
        blake3_root: [u8; 32],
        bao_outboard: Vec<u8>,
    ) {
        let rel_path = derive_rel_path(&self.config.src_root, src);
        // Phase 43 post-review — defense-in-depth: refuse to record a
        // rel_path that would later round-trip through `verify_manifest`
        // and trigger the path-traversal guard. A hostile engine adapter
        // could otherwise poison the manifest with `../../../etc/passwd`
        // entries; the verify-side guard catches it on read, but
        // failing fast at write time gives a clearer error.
        if validate_rel_path(&rel_path).is_err() {
            eprintln!(
                "copythat-provenance: rejected rel_path {rel_path:?} (traversal check) for src {}",
                src.display()
            );
            return;
        }
        let record = FileRecord {
            rel_path,
            size,
            blake3_root,
            bao_outboard,
        };
        if let Ok(mut state) = self.state.lock() {
            state.files.push(record);
        }
    }
}

/// Derive the manifest-friendly rel_path for `src` against
/// `src_root`. Forward-slash separated; only emits `Component::Normal`
/// segments (root / drive / parent components are filtered out so the
/// manifest stays portable and the verify-side traversal guard never
/// has to reject a path produced by this function).
///
/// Phase 43 post-review (P43 #6): when `src.strip_prefix(src_root)`
/// fails — caller drove the engine on an absolute path from outside
/// `src_root` — we previously fell back to `src` itself, which on
/// long absolute paths produced a multi-component rel_path that COULD
/// collide between two source files sharing the same suffix tail.
/// New behavior: fall back to just the filename. Multiple files at
/// different absolute paths sharing a basename will collide on the
/// rel_path, but the alternative (silently encoding the absolute
/// path into the manifest) is worse — the manifest's rel_path is
/// supposed to be relative to `dst_root` for the verify side.
/// Callers that drive the engine outside `src_root` are off-policy
/// and get a best-effort name-only record.
fn derive_rel_path(src_root: &Path, src: &Path) -> String {
    let rel = src.strip_prefix(src_root).unwrap_or_else(|_| {
        // Out-of-tree caller — emit only the filename.
        std::path::Path::new(
            src.file_name()
                .map(|s| s.to_str().unwrap_or("?"))
                .unwrap_or("?"),
        )
    });
    let parts: Vec<String> = rel
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            // ParentDir, RootDir, Prefix, CurDir are dropped so the
            // emitted rel_path always satisfies `validate_rel_path`.
            _ => None,
        })
        .collect();
    if parts.is_empty() {
        // Nothing usable produced — fall back to the basename (or
        // `?` if even that's not available).
        if let Some(name) = src.file_name() {
            return name.to_string_lossy().into_owned();
        }
        return "?".into();
    }
    parts.join("/")
}

/// Best-effort hostname probe. Falls back to "unknown" when the
/// host crate can't read the OS hostname (sandboxed env, etc.).
fn probe_host() -> String {
    // Avoid a cross-platform hostname dependency; read the standard
    // OS env var instead. `COMPUTERNAME` on Windows, `HOSTNAME` on
    // *nix shells, "unknown" otherwise.
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".into())
}

/// Best-effort username probe. Mirrors `probe_host`.
fn probe_user() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown".into())
}

#[cfg(feature = "tsa")]
fn request_timestamp(
    tsa_url: &str,
    manifest: &ProvenanceManifest,
) -> Result<Rfc3161Token, ProvenanceError> {
    crate::timestamp::request(tsa_url, manifest)
}

#[cfg(not(feature = "tsa"))]
fn request_timestamp(
    _tsa_url: &str,
    _manifest: &ProvenanceManifest,
) -> Result<Rfc3161Token, ProvenanceError> {
    Err(ProvenanceError::classify(
        ProvenanceErrorKind::TsaFeatureDisabled,
        "build was compiled without the `tsa` feature; rebuild with `--features tsa` to request RFC 3161 timestamps",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::DEFAULT_MANIFEST_FILENAME;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn fixture_sink() -> (Arc<CopyThatProvenanceSink>, TempDir) {
        let tmp = TempDir::new().unwrap();
        let src_root = tmp.path().join("src");
        let dst_root = tmp.path().join("dst");
        std::fs::create_dir_all(&src_root).unwrap();
        std::fs::create_dir_all(&dst_root).unwrap();
        let mut config = SinkConfig::new(src_root, dst_root);
        config.signing_key = Some(crate::sign::generate_signing_key());
        let sink = CopyThatProvenanceSink::new(config);
        (sink, tmp)
    }

    #[test]
    fn record_file_derives_rel_path_from_src_root() {
        let (sink, tmp) = fixture_sink();
        let src_root = tmp.path().join("src");
        let nested = src_root.join("a").join("b.txt");
        sink.record_file(
            &nested,
            &tmp.path().join("dst").join("a").join("b.txt"),
            16,
            [0u8; 32],
            Vec::new(),
        );
        assert_eq!(sink.file_count(), 1);
        let state = sink.state.lock().unwrap();
        assert_eq!(state.files[0].rel_path, "a/b.txt");
    }

    #[test]
    fn finalize_signs_when_signing_key_present() {
        let (sink, tmp) = fixture_sink();
        let src_root = tmp.path().join("src");
        sink.record_file(
            &src_root.join("only.txt"),
            &tmp.path().join("dst").join("only.txt"),
            8,
            [0xAB; 32],
            Vec::new(),
        );
        let manifest_path = tmp.path().join("dst").join(DEFAULT_MANIFEST_FILENAME);
        let manifest = sink.finalize_to_path(&manifest_path).unwrap();
        assert!(manifest.signature.is_some());
        assert_eq!(manifest.files.len(), 1);
        assert!(manifest_path.exists());
    }

    #[test]
    fn finalize_without_key_produces_unsigned_manifest() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::create_dir_all(tmp.path().join("dst")).unwrap();
        let config = SinkConfig::new(tmp.path().join("src"), tmp.path().join("dst"));
        let sink = CopyThatProvenanceSink::new(config);
        sink.record_file(
            &tmp.path().join("src").join("a.txt"),
            &tmp.path().join("dst").join("a.txt"),
            4,
            [1u8; 32],
            Vec::new(),
        );
        let manifest_path = tmp.path().join("dst").join(DEFAULT_MANIFEST_FILENAME);
        let m = sink.finalize_to_path(&manifest_path).unwrap();
        assert!(m.signature.is_none());
    }

    #[cfg(not(feature = "tsa"))]
    #[test]
    fn tsa_url_without_feature_returns_classified_error() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::create_dir_all(tmp.path().join("dst")).unwrap();
        let mut config = SinkConfig::new(tmp.path().join("src"), tmp.path().join("dst"));
        config.tsa_url = Some(crate::DEFAULT_TSA_URL.to_string());
        let sink = CopyThatProvenanceSink::new(config);
        sink.record_file(
            &tmp.path().join("src").join("a.txt"),
            &tmp.path().join("dst").join("a.txt"),
            4,
            [1u8; 32],
            Vec::new(),
        );
        let manifest_path = tmp.path().join("dst").join(DEFAULT_MANIFEST_FILENAME);
        let err = sink.finalize_to_path(&manifest_path).unwrap_err();
        assert_eq!(err.kind(), ProvenanceErrorKind::TsaFeatureDisabled);
    }

    #[test]
    fn rel_path_uses_forward_slashes() {
        let root = PathBuf::from("/a/b");
        let nested = PathBuf::from("/a/b/c/d.txt");
        assert_eq!(derive_rel_path(&root, &nested), "c/d.txt");
    }
}
