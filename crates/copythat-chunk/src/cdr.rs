//! Phase 50 (moonshot) — Common Dedup Repository, version 0 (CDR-0).
//!
//! CDR-0 is a small, public-domain (CC0-1.0) interchange format so any
//! compliant tool can read any compliant deduplicated repository. The
//! normative specification lives in [`docs/spec/CDR-0.md`]; this module
//! is its **Rust reference implementation** for the layer that makes
//! cross-tool migration near-instant: the *manifest* — the list of
//! content-addressed chunks that reconstruct a file.
//!
//! # What this module covers
//!
//! - [`CdrManifest`] / [`CdrChunkRef`] — the canonical, tool-neutral
//!   manifest types, encoded as **deterministic CBOR** (definite-length,
//!   stable struct-field order, via `ciborium`).
//! - [`CdrManifest::validate`] — the invariants a reader must check
//!   before trusting a manifest (algorithm tag, 32-byte BLAKE3 digests,
//!   contiguous chunk tiling, declared size).
//! - [`ensure_readable`] — the spec's hard rule: a reader **must refuse
//!   to read a repository whose `spec_version` is newer than the
//!   version it understands**, rather than silently mis-parse it.
//! - Lossless conversion to/from the internal [`Manifest`] so a CDR-0
//!   manifest can be turned straight back into something
//!   [`crate::materialise_file`] can restore.
//!
//! # What is deferred (documented, not built here)
//!
//! The full Phase 50 vision also includes a `copythat migrate
//! <restic|borg|kopia>` / `copythat export` CLI, a sibling `cdr-py`
//! PyPI package, and upstream adoption PRs. Those need third-party repo
//! binaries, a separate published crate, and community engagement, so
//! they are tracked as follow-ups in `docs/ROADMAP.md` — this module
//! ships the on-disk **format contract** they all build on.
//!
//! # Canonical encoding
//!
//! v0 uses CBOR (RFC 8949) with definite-length arrays/maps and stable
//! struct-field order, which makes the encoder **deterministic**
//! (re-encoding the same manifest yields byte-identical output — the
//! property that lets two tools agree on a manifest's identity). Strict
//! RFC 8949 §4.2 core-deterministic *map-key sorting* is a v1 hardening
//! noted in the spec; it does not affect v0 round-trip fidelity.
//!
//! ```
//! use copythat_chunk::cdr::{CdrManifest, ensure_readable, CDR_SPEC_VERSION};
//! use copythat_chunk::Manifest;
//!
//! let internal = Manifest { file_hash: [0u8; 32], size: 0, chunks: vec![] };
//! let cdr = CdrManifest::from_manifest(&internal);
//! let bytes = cdr.to_cbor()?;
//! let back = CdrManifest::from_cbor(&bytes)?;
//! back.validate()?;
//! assert_eq!(back.to_manifest()?, internal);
//! assert!(ensure_readable(CDR_SPEC_VERSION).is_ok());
//! # Ok::<(), copythat_chunk::cdr::CdrError>(())
//! ```

use serde::{Deserialize, Serialize};

use crate::compress::ChunkCodec;
use crate::types::{Blake3Hash, ChunkRef, Manifest};

/// The CDR specification version this implementation understands.
///
/// Per the spec, a reader **must refuse** any repository whose
/// `spec_version` exceeds this (see [`ensure_readable`]).
pub const CDR_SPEC_VERSION: u32 = 0;

/// Canonical chunk-algorithm tag for CDR-0: FastCDC v2020 with the
/// spec's mandated parameters, BLAKE3-256 keying. Every CDR-0 manifest
/// carries this so a reader can reject a repo it cannot rechunk.
pub const CDR_ALGO: &str = "fastcdc-2020;min=524288;avg=1048576;max=4194304;hash=blake3-256";

/// The length, in bytes, of a BLAKE3-256 digest.
const HASH_LEN: usize = 32;

/// Errors from the CDR-0 reference implementation.
#[derive(Debug, thiserror::Error)]
pub enum CdrError {
    /// The repository's `spec_version` is newer than this reader
    /// understands. The spec mandates refusing rather than guessing.
    #[error("CDR repo spec version {repo} is newer than this reader supports ({reader})")]
    UnsupportedVersion {
        /// Version found in the repository.
        repo: u32,
        /// Highest version this reader implements ([`CDR_SPEC_VERSION`]).
        reader: u32,
    },

    /// CBOR serialisation failed.
    #[error("CDR CBOR encode error: {0}")]
    Encode(String),

    /// CBOR deserialisation failed.
    #[error("CDR CBOR decode error: {0}")]
    Decode(String),

    /// A manifest violated a CDR-0 invariant.
    #[error("CDR manifest invalid: {0}")]
    Invalid(String),
}

/// A chunk reference in canonical CDR-0 form. The digest is a CBOR byte
/// string (via `serde_bytes`) rather than an array of integers, so the
/// encoding is compact and tool-neutral.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdrChunkRef {
    /// 32-byte BLAKE3 digest of the chunk's bytes.
    #[serde(with = "serde_bytes")]
    pub hash: Vec<u8>,
    /// Byte offset of this chunk in the reconstructed file.
    pub offset: u64,
    /// Byte length of the chunk (logical / plaintext).
    pub len: u32,
    /// Phase 49h — codec the chunk's stored bytes use. `#[serde(default)]`
    /// (CBOR is field-name-keyed) → a manifest written before 49h decodes
    /// as `None`. A v0-compatible additive field; no `CDR_SPEC_VERSION`
    /// bump (the chunk hash + logical len are unchanged).
    #[serde(default)]
    pub codec: ChunkCodec,
}

/// A file manifest in canonical CDR-0 form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdrManifest {
    /// CDR spec version this manifest was written under.
    pub spec_version: u32,
    /// Chunk-algorithm tag — must match [`CDR_ALGO`] for a v0 reader to
    /// be able to re-chunk source data identically.
    pub algo: String,
    /// 32-byte BLAKE3 digest of the whole file.
    #[serde(with = "serde_bytes")]
    pub file_hash: Vec<u8>,
    /// Total file size in bytes (`== sum(chunk.len)`).
    pub size: u64,
    /// Ordered chunks. Concatenating their bytes reconstructs the file.
    pub chunks: Vec<CdrChunkRef>,
}

impl CdrManifest {
    /// Project an internal [`Manifest`] into canonical CDR-0 form.
    #[must_use]
    pub fn from_manifest(m: &Manifest) -> Self {
        Self {
            spec_version: CDR_SPEC_VERSION,
            algo: CDR_ALGO.to_string(),
            file_hash: m.file_hash.to_vec(),
            size: m.size,
            chunks: m
                .chunks
                .iter()
                .map(|c| CdrChunkRef {
                    hash: c.hash.to_vec(),
                    offset: c.offset,
                    len: c.len,
                    codec: c.codec,
                })
                .collect(),
        }
    }

    /// Convert back to an internal [`Manifest`]. Validates first, so a
    /// malformed or too-new manifest is rejected rather than silently
    /// truncated.
    pub fn to_manifest(&self) -> std::result::Result<Manifest, CdrError> {
        self.validate()?;
        let chunks = self
            .chunks
            .iter()
            .map(|c| {
                Ok(ChunkRef {
                    hash: vec_to_hash(&c.hash)?,
                    offset: c.offset,
                    len: c.len,
                    codec: c.codec,
                })
            })
            .collect::<std::result::Result<Vec<_>, CdrError>>()?;
        Ok(Manifest {
            file_hash: vec_to_hash(&self.file_hash)?,
            size: self.size,
            chunks,
        })
    }

    /// Encode to deterministic CBOR.
    pub fn to_cbor(&self) -> std::result::Result<Vec<u8>, CdrError> {
        let mut out = Vec::new();
        ciborium::into_writer(self, &mut out).map_err(|e| CdrError::Encode(e.to_string()))?;
        Ok(out)
    }

    /// Decode from CBOR **and validate**. This is the entry point a
    /// CDR-0 reader should use: the spec (§5, §13) requires a reader to
    /// enforce every manifest invariant, so validation is not optional.
    /// A too-new `spec_version`, a bad digest length, broken chunk
    /// tiling, or a size mismatch is rejected here rather than silently
    /// trusted.
    pub fn from_cbor(bytes: &[u8]) -> std::result::Result<Self, CdrError> {
        let m = Self::from_cbor_unchecked(bytes)?;
        m.validate()?;
        Ok(m)
    }

    /// Low-level decode that **skips** validation. Prefer
    /// [`Self::from_cbor`]; use this only when you deliberately need the
    /// raw decoded fields (e.g. to inspect an intentionally-malformed
    /// manifest in a test or diagnostic).
    pub fn from_cbor_unchecked(bytes: &[u8]) -> std::result::Result<Self, CdrError> {
        ciborium::from_reader(bytes).map_err(|e| CdrError::Decode(e.to_string()))
    }

    /// Check every CDR-0 invariant a reader must enforce:
    ///
    /// - `spec_version` is readable ([`ensure_readable`]),
    /// - the algorithm tag matches [`CDR_ALGO`],
    /// - the file digest and every chunk digest are 32 bytes,
    /// - chunks tile the file contiguously from offset 0, and
    /// - the summed chunk lengths equal the declared `size`.
    pub fn validate(&self) -> std::result::Result<(), CdrError> {
        ensure_readable(self.spec_version)?;
        if self.algo != CDR_ALGO {
            return Err(CdrError::Invalid(format!(
                "unsupported chunk algorithm {:?}; expected {CDR_ALGO:?}",
                self.algo
            )));
        }
        if self.file_hash.len() != HASH_LEN {
            return Err(CdrError::Invalid(format!(
                "file_hash must be {HASH_LEN} bytes, got {}",
                self.file_hash.len()
            )));
        }
        let mut cursor: u64 = 0;
        for (i, c) in self.chunks.iter().enumerate() {
            if c.hash.len() != HASH_LEN {
                return Err(CdrError::Invalid(format!(
                    "chunk {i} hash must be {HASH_LEN} bytes, got {}",
                    c.hash.len()
                )));
            }
            if c.offset != cursor {
                return Err(CdrError::Invalid(format!(
                    "chunk {i} offset {} breaks contiguous tiling (expected {cursor})",
                    c.offset
                )));
            }
            cursor += u64::from(c.len);
        }
        if cursor != self.size {
            return Err(CdrError::Invalid(format!(
                "chunk lengths sum to {cursor} but declared size is {}",
                self.size
            )));
        }
        Ok(())
    }
}

/// The spec's mandatory forward-compatibility gate: a reader must
/// refuse a repository whose `spec_version` is newer than the version it
/// implements ([`CDR_SPEC_VERSION`]).
pub fn ensure_readable(repo_version: u32) -> std::result::Result<(), CdrError> {
    if repo_version > CDR_SPEC_VERSION {
        return Err(CdrError::UnsupportedVersion {
            repo: repo_version,
            reader: CDR_SPEC_VERSION,
        });
    }
    Ok(())
}

/// Map a CDR byte-string digest to a fixed 32-byte [`Blake3Hash`].
fn vec_to_hash(v: &[u8]) -> std::result::Result<Blake3Hash, CdrError> {
    if v.len() != HASH_LEN {
        return Err(CdrError::Invalid(format!(
            "digest must be {HASH_LEN} bytes, got {}",
            v.len()
        )));
    }
    let mut h = [0u8; HASH_LEN];
    h.copy_from_slice(v);
    Ok(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> Manifest {
        Manifest {
            file_hash: [9u8; 32],
            size: 300,
            chunks: vec![
                ChunkRef {
                    hash: [1u8; 32],
                    offset: 0,
                    len: 100,
                    codec: ChunkCodec::None,
                },
                ChunkRef {
                    hash: [2u8; 32],
                    offset: 100,
                    len: 200,
                    codec: ChunkCodec::None,
                },
            ],
        }
    }

    #[test]
    fn round_trip_internal_to_cdr_to_internal() {
        let m = sample_manifest();
        let cdr = CdrManifest::from_manifest(&m);
        cdr.validate().unwrap();
        let bytes = cdr.to_cbor().unwrap();
        let back = CdrManifest::from_cbor(&bytes).unwrap();
        assert_eq!(back, cdr);
        assert_eq!(back.to_manifest().unwrap(), m);
    }

    #[test]
    fn cbor_encoding_is_deterministic() {
        let cdr = CdrManifest::from_manifest(&sample_manifest());
        assert_eq!(cdr.to_cbor().unwrap(), cdr.to_cbor().unwrap());
    }

    #[test]
    fn version_guard_refuses_newer_repos() {
        assert!(ensure_readable(CDR_SPEC_VERSION).is_ok());
        let err = ensure_readable(CDR_SPEC_VERSION + 1).unwrap_err();
        assert!(matches!(err, CdrError::UnsupportedVersion { .. }));
    }

    #[test]
    fn validate_rejects_bad_hash_length() {
        let mut cdr = CdrManifest::from_manifest(&sample_manifest());
        cdr.chunks[0].hash = vec![0u8; 31];
        assert!(matches!(cdr.validate(), Err(CdrError::Invalid(_))));
    }

    #[test]
    fn validate_rejects_noncontiguous_tiling() {
        let mut cdr = CdrManifest::from_manifest(&sample_manifest());
        cdr.chunks[1].offset = 999;
        assert!(matches!(cdr.validate(), Err(CdrError::Invalid(_))));
    }

    #[test]
    fn validate_rejects_size_mismatch() {
        let mut cdr = CdrManifest::from_manifest(&sample_manifest());
        cdr.size = 12345;
        assert!(matches!(cdr.validate(), Err(CdrError::Invalid(_))));
    }

    #[test]
    fn validate_rejects_wrong_algo() {
        let mut cdr = CdrManifest::from_manifest(&sample_manifest());
        cdr.algo = "gzip".to_string();
        assert!(matches!(cdr.validate(), Err(CdrError::Invalid(_))));
    }
}
