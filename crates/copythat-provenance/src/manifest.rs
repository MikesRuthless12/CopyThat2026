//! [`ProvenanceManifest`] schema + canonical-CBOR helpers.
//!
//! The manifest is the public document Phase 43 ships. Its on-disk
//! shape is canonical CBOR (RFC 8949) of the [`ProvenanceManifest`]
//! struct. The signing contract is "ed25519 over the
//! `manifest_signing_bytes` view" — that's a CBOR encoding of the
//! same struct with `signature = None` and `timestamp = None`. This
//! way the signature itself doesn't need to round-trip through the
//! signed bytes.
//!
//! The verify command uses [`parse_manifest_cbor`] to load the file,
//! [`manifest_signing_bytes`] to reconstruct the bytes the signature
//! covered, and [`manifest_root_blake3`] to verify the Merkle root
//! over per-file roots.

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ProvenanceError;

/// Default filename Copy That writes inside `<dst-root>/` after a
/// provenance-enabled job. Surfaces in the Settings UI ("Show
/// manifest after each job") and the CLI.
pub const DEFAULT_MANIFEST_FILENAME: &str = ".copythat-provenance.cbor";

/// One file's contribution to the manifest. Records the relative
/// path (sorted, see [`ProvenanceManifest::files`]), the size in
/// bytes, the BLAKE3 root, and the Bao verified-streaming outboard.
///
/// `bao_outboard` may be empty when the manifest was produced by a
/// root-only encoder; in that mode the verify command falls back to
/// a full BLAKE3 re-hash of the destination file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRecord {
    /// Relative path within the destination tree, joined with
    /// forward slashes regardless of host OS so the manifest is
    /// portable across Windows / Linux / macOS.
    pub rel_path: String,
    /// File size in bytes — matches the byte count the engine
    /// streamed through the encoder.
    pub size: u64,
    /// 32-byte BLAKE3 root over the file contents.
    #[serde(with = "serde_bytes_array_32")]
    pub blake3_root: [u8; 32],
    /// Bao verified-streaming outboard. May be empty (root-only
    /// mode); see [`crate::RootOnlyEncoder`].
    #[serde(with = "serde_bytes")]
    pub bao_outboard: Vec<u8>,
}

/// Detached ed25519 signature over [`manifest_signing_bytes`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    /// 32-byte SubjectPublicKeyInfo-shaped ed25519 public key (the
    /// raw 32-byte point, not the SPKI/PEM wrapper). The verify
    /// command compares this against the user's trusted-key list and
    /// fails with [`crate::ProvenanceErrorKind::BadSignature`] on
    /// mismatch.
    #[serde(with = "serde_bytes_array_32")]
    pub public_key: [u8; 32],
    /// 64-byte detached signature. RFC 8032 ed25519 (PureEd25519,
    /// not pre-hashed).
    #[serde(with = "serde_bytes_array_64")]
    pub sig: [u8; 64],
}

/// RFC 3161 timestamp token. The opaque DER bytes the TSA returned;
/// kept verbatim so the verify pass can hand them to the same
/// `rfc3161-client` (or another RFC 3161 verifier) without
/// re-encoding.
///
/// Phase 43 ships the schema; the actual TSA call lives behind the
/// `tsa` Cargo feature. A manifest produced without `tsa` will have
/// `timestamp = None`; consumers that need the proof can re-stamp
/// the manifest later (the TSA stamps a hash of the manifest, not
/// its raw bytes — see RFC 3161 §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rfc3161Token {
    /// TSA URL the token came from. Surfaced by the verify command
    /// so the user can audit the trust path.
    pub tsa_url: String,
    /// Opaque DER-encoded `TimeStampToken` (RFC 3161 §2.4.2).
    #[serde(with = "serde_bytes")]
    pub token_der: Vec<u8>,
    /// Wall-clock time the TSA reported in the token. Decoded from
    /// `token_der` at request time and surfaced here for quick UI
    /// rendering without re-parsing DER.
    pub stamped_at: DateTime<Utc>,
}

/// The manifest. One per provenance-enabled copy job. Serialized as
/// canonical CBOR per RFC 8949.
///
/// Field ordering matters: `signature` and `timestamp` come last so
/// the [`manifest_signing_bytes`] view can simply zero them out
/// before re-encoding. New schema fields go BEFORE `signature` and
/// `timestamp` to preserve that property — failure to do so breaks
/// signature compatibility across versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceManifest {
    /// Schema version. Bumped on breaking format changes; readers
    /// reject manifests with a version they don't recognise.
    pub schema_version: u32,
    /// Stable per-job UUIDv4. Matches the Phase 12 history `jobs.id`
    /// when the job was driven through the queue; a free-standing
    /// CLI invocation gets a fresh UUID.
    pub job_id: Uuid,
    /// Source root the job copied from.
    pub src_root: PathBuf,
    /// Destination root the job copied to.
    pub dst_root: PathBuf,
    /// Job start time (UTC, millisecond precision).
    pub started_at: DateTime<Utc>,
    /// Job finish time (UTC, millisecond precision). Set by the
    /// sink's `finalize_to_path` — until that call the field carries
    /// `started_at` as a placeholder.
    pub finished_at: DateTime<Utc>,
    /// Hostname the job ran on (`hostname::get` at finalise time;
    /// "unknown" if probing fails).
    pub host: String,
    /// User account the job ran as (`whoami::username`; "unknown" on
    /// failure).
    pub user: String,
    /// Copy That version that produced the manifest. Matches
    /// `env!("CARGO_PKG_VERSION")` of the crate that called
    /// `finalize_to_path`.
    pub copythat_version: String,
    /// 32-byte BLAKE3 root over the per-file `blake3_root`s after
    /// the files have been sorted by `rel_path`. Computed by
    /// [`manifest_root_blake3`].
    #[serde(with = "serde_bytes_array_32")]
    pub merkle_root: [u8; 32],
    /// One [`FileRecord`] per file copied, sorted by `rel_path`
    /// (ASCII). Sort order is part of the manifest contract — the
    /// Merkle root is path-sorted, so deterministic ordering is
    /// required.
    pub files: Vec<FileRecord>,
    /// Optional detached ed25519 signature over the bytes returned
    /// by [`manifest_signing_bytes`]. `None` for unsigned manifests
    /// (still useful for integrity but no authenticity claim).
    pub signature: Option<Signature>,
    /// Optional RFC 3161 timestamp token. `None` when the job did
    /// not request a timestamp, or when the build does not include
    /// the `tsa` feature.
    pub timestamp: Option<Rfc3161Token>,
}

impl ProvenanceManifest {
    /// Current schema version. Bump on any format change that
    /// breaks signature compatibility.
    pub const SCHEMA_VERSION: u32 = 1;

    /// Construct a fresh manifest skeleton with a UUIDv4 `job_id`,
    /// the provided roots + files, and no signature / timestamp.
    /// The `merkle_root` is computed from the supplied files (which
    /// are sorted in place by `rel_path`).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        src_root: PathBuf,
        dst_root: PathBuf,
        started_at: DateTime<Utc>,
        finished_at: DateTime<Utc>,
        host: String,
        user: String,
        copythat_version: String,
        mut files: Vec<FileRecord>,
    ) -> Self {
        files.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
        let merkle_root = manifest_root_blake3(&files);
        Self {
            schema_version: Self::SCHEMA_VERSION,
            job_id: Uuid::new_v4(),
            src_root,
            dst_root,
            started_at,
            finished_at,
            host,
            user,
            copythat_version,
            merkle_root,
            files,
            signature: None,
            timestamp: None,
        }
    }
}

/// Compute the BLAKE3 Merkle root over a sorted [`FileRecord`]
/// slice. Each per-file `blake3_root` is fed in declaration order;
/// the result is what gets stored in
/// [`ProvenanceManifest::merkle_root`].
///
/// Caller is responsible for sorting `files` by `rel_path` first —
/// the [`ProvenanceManifest::new`] constructor enforces this.
pub fn manifest_root_blake3(files: &[FileRecord]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    for record in files {
        hasher.update(&record.blake3_root);
    }
    *hasher.finalize().as_bytes()
}

/// Bytes the signature covers. Equivalent to canonical CBOR of the
/// manifest with `signature` and `timestamp` cleared. The verify
/// pass calls this on the parsed manifest to reconstruct exactly the
/// bytes the signer signed.
pub fn manifest_signing_bytes(m: &ProvenanceManifest) -> Result<Vec<u8>, ProvenanceError> {
    let stripped = ProvenanceManifest {
        signature: None,
        timestamp: None,
        ..m.clone()
    };
    canonical_cbor_bytes(&stripped)
}

/// Canonical CBOR encoding of `value`. Wraps `ciborium::ser::into_writer`
/// to centralise the error mapping.
pub fn canonical_cbor_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ProvenanceError> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(value, &mut out)?;
    Ok(out)
}

/// Parse a manifest from canonical CBOR bytes.
pub fn parse_manifest_cbor(bytes: &[u8]) -> Result<ProvenanceManifest, ProvenanceError> {
    let m: ProvenanceManifest = ciborium::de::from_reader(bytes)?;
    if m.schema_version != ProvenanceManifest::SCHEMA_VERSION {
        return Err(ProvenanceError::classify(
            crate::error::ProvenanceErrorKind::Protocol,
            format!(
                "schema version {} not supported (this build expects {})",
                m.schema_version,
                ProvenanceManifest::SCHEMA_VERSION
            ),
        ));
    }
    Ok(m)
}

/// Write `manifest` to `path` as canonical CBOR. Overwrites the file
/// if it exists.
pub fn write_manifest_cbor(
    manifest: &ProvenanceManifest,
    path: &Path,
) -> Result<(), ProvenanceError> {
    let bytes = canonical_cbor_bytes(manifest)?;
    std::fs::write(path, bytes).map_err(|e| ProvenanceError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    Ok(())
}

// `serde_bytes` only handles `Vec<u8>` / `&[u8]`; for fixed-size
// arrays we hand-roll a tiny adapter so the on-the-wire encoding is
// a CBOR byte string rather than a CBOR array of integers.
mod serde_bytes_array_32 {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(bytes: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let buf: serde_bytes::ByteBuf = serde_bytes::ByteBuf::deserialize(d)?;
        let v = buf.into_vec();
        if v.len() != 32 {
            return Err(D::Error::custom(format!(
                "expected 32-byte array, got {}",
                v.len()
            )));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&v);
        Ok(out)
    }
}

mod serde_bytes_array_64 {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(bytes: &[u8; 64], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 64], D::Error> {
        let buf: serde_bytes::ByteBuf = serde_bytes::ByteBuf::deserialize(d)?;
        let v = buf.into_vec();
        if v.len() != 64 {
            return Err(D::Error::custom(format!(
                "expected 64-byte array, got {}",
                v.len()
            )));
        }
        let mut out = [0u8; 64];
        out.copy_from_slice(&v);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(path: &str, root: u8) -> FileRecord {
        FileRecord {
            rel_path: path.to_string(),
            size: 16,
            blake3_root: [root; 32],
            bao_outboard: Vec::new(),
        }
    }

    #[test]
    fn manifest_roundtrips_through_canonical_cbor() {
        let now = Utc::now();
        let m = ProvenanceManifest::new(
            PathBuf::from("/src"),
            PathBuf::from("/dst"),
            now,
            now,
            "host".into(),
            "user".into(),
            "1.0.0".into(),
            vec![record("a.txt", 0x11), record("b.txt", 0x22)],
        );
        let bytes = canonical_cbor_bytes(&m).unwrap();
        let parsed = parse_manifest_cbor(&bytes).unwrap();
        assert_eq!(parsed, m);
    }

    #[test]
    fn merkle_root_changes_when_a_record_changes() {
        let r1 = manifest_root_blake3(&[record("a", 1), record("b", 2)]);
        let r2 = manifest_root_blake3(&[record("a", 1), record("b", 3)]);
        assert_ne!(r1, r2);
    }

    #[test]
    fn merkle_root_depends_on_order() {
        // Caller MUST sort by rel_path; passing them out of order
        // produces a different root. We verify the sensitivity here
        // so a future refactor that drops the sort fails loudly.
        let sorted = manifest_root_blake3(&[record("a", 1), record("b", 2)]);
        let reversed = manifest_root_blake3(&[record("b", 2), record("a", 1)]);
        assert_ne!(sorted, reversed);
    }

    #[test]
    fn signing_bytes_strip_signature_and_timestamp() {
        let now = Utc::now();
        let mut m = ProvenanceManifest::new(
            PathBuf::from("/s"),
            PathBuf::from("/d"),
            now,
            now,
            "h".into(),
            "u".into(),
            "v".into(),
            vec![record("x", 7)],
        );
        let bytes_before = manifest_signing_bytes(&m).unwrap();

        // Inject a sig + timestamp; signing bytes should be
        // unchanged.
        m.signature = Some(Signature {
            public_key: [0; 32],
            sig: [0; 64],
        });
        m.timestamp = Some(Rfc3161Token {
            tsa_url: "https://example".into(),
            token_der: vec![1, 2, 3],
            stamped_at: now,
        });
        let bytes_after = manifest_signing_bytes(&m).unwrap();

        assert_eq!(
            bytes_before, bytes_after,
            "signing-bytes view should ignore signature + timestamp fields"
        );
    }

    #[test]
    fn parse_rejects_unknown_schema_version() {
        let now = Utc::now();
        let mut m = ProvenanceManifest::new(
            PathBuf::from("/s"),
            PathBuf::from("/d"),
            now,
            now,
            "h".into(),
            "u".into(),
            "v".into(),
            vec![],
        );
        m.schema_version = 999;
        let bytes = canonical_cbor_bytes(&m).unwrap();
        let err = parse_manifest_cbor(&bytes).unwrap_err();
        assert_eq!(
            err.kind(),
            crate::error::ProvenanceErrorKind::Protocol,
            "unknown schema version should classify as Protocol"
        );
    }
}
