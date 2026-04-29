//! Error types for the provenance crate.

use std::path::PathBuf;

use thiserror::Error;

/// One error type for the whole crate. Variant body holds the
/// machine-readable detail; the `Display` impl returns
/// human-readable text suitable for CLI / UI output.
#[derive(Debug, Error)]
pub enum ProvenanceError {
    /// Sub-classification — kept on a sibling field so callers can
    /// `match err.kind()` without parsing the `Display` string.
    #[error("{kind}: {detail}")]
    Classified {
        /// Semantic class.
        kind: ProvenanceErrorKind,
        /// Free-form detail (path, hash, byte position, etc.).
        detail: String,
    },

    /// IO failure with the path that triggered it (so the verify
    /// command can print "tampered: <path>" rather than dropping the
    /// path on the floor).
    #[error("io error on {path}: {source}")]
    Io {
        /// Path the error referred to.
        path: PathBuf,
        /// Underlying `std::io::Error`.
        #[source]
        source: std::io::Error,
    },

    /// CBOR serialisation failure. Distinct from I/O so a corrupt
    /// manifest can be diagnosed independently of disk problems.
    #[error("cbor serialise failed: {0}")]
    CborSer(#[from] ciborium::ser::Error<std::io::Error>),

    /// CBOR deserialisation failure.
    #[error("cbor deserialise failed: {0}")]
    CborDe(#[from] ciborium::de::Error<std::io::Error>),

    /// PKCS#8 / PEM key parsing failure.
    #[error("ed25519 key error: {0}")]
    Ed25519Key(String),

    /// Signature verification failure.
    #[error("ed25519 signature mismatch")]
    SignatureMismatch,
}

/// Semantic classifier for [`ProvenanceError::Classified`]. The
/// verify command branches on this to render exit codes and the UI
/// branches on this for which icon to show beside each row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceErrorKind {
    /// A file's on-disk bytes hash to a different BLAKE3 root than
    /// the manifest claims.
    Tampered,
    /// The manifest's `merkle_root` doesn't match the freshly
    /// computed Merkle root over per-file roots.
    MerkleMismatch,
    /// The manifest's signature didn't validate.
    BadSignature,
    /// The manifest's RFC 3161 timestamp didn't validate.
    BadTimestamp,
    /// The TSA feature was requested at runtime but the build does
    /// not include it (the `tsa` Cargo feature was off).
    TsaFeatureDisabled,
    /// The manifest references a file that's missing from the
    /// destination.
    Missing,
    /// Generic protocol / parsing error.
    Protocol,
}

impl std::fmt::Display for ProvenanceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ProvenanceErrorKind::Tampered => "tampered",
            ProvenanceErrorKind::MerkleMismatch => "merkle-mismatch",
            ProvenanceErrorKind::BadSignature => "bad-signature",
            ProvenanceErrorKind::BadTimestamp => "bad-timestamp",
            ProvenanceErrorKind::TsaFeatureDisabled => "tsa-feature-disabled",
            ProvenanceErrorKind::Missing => "missing",
            ProvenanceErrorKind::Protocol => "protocol",
        })
    }
}

impl ProvenanceError {
    /// Construct a [`ProvenanceError::Classified`] without typing
    /// the verbose struct literal at every call site.
    pub fn classify(kind: ProvenanceErrorKind, detail: impl Into<String>) -> Self {
        Self::Classified {
            kind,
            detail: detail.into(),
        }
    }

    /// Pull out the semantic kind for `match`-on-the-kind code paths.
    /// Returns `None` for non-classified variants (raw I/O, raw CBOR,
    /// etc.) — the caller should treat those as `Protocol`.
    pub fn kind(&self) -> ProvenanceErrorKind {
        match self {
            ProvenanceError::Classified { kind, .. } => *kind,
            ProvenanceError::SignatureMismatch => ProvenanceErrorKind::BadSignature,
            _ => ProvenanceErrorKind::Protocol,
        }
    }
}
