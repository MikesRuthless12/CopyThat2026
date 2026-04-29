//! Phase 43 — forensic chain-of-custody manifests for Copy That.
//!
//! `copythat-provenance` ships:
//!
//! - [`ProvenanceManifest`] — the public document a copy job emits
//!   when [`copythat_core::CopyOptions::provenance`] is set. Carries
//!   the per-file BLAKE3 root, the Bao verified-streaming outboard,
//!   the Merkle root over all per-file roots (sorted by relative
//!   path), an optional ed25519 detached signature, and an optional
//!   RFC 3161 timestamp token.
//! - [`FileRecord`] — one row per copied file: `(rel_path, size,
//!   blake3_root, bao_outboard)`.
//! - [`CopyThatProvenanceSink`] — the
//!   [`copythat_core::ProvenanceSink`] implementation the engine talks
//!   to. Owns a `Mutex<Vec<FileRecord>>` and a `BaoOutboardEncoder`
//!   factory; the caller (CLI / UI / test) calls
//!   [`CopyThatProvenanceSink::finalize_to_path`] after the tree-walk
//!   to build the manifest, sign it, optionally request a TSA
//!   timestamp, and write the canonical CBOR to disk.
//! - [`SigningKey`] / [`VerifyingKey`] / [`generate_signing_key`] —
//!   ed25519 keypair management with PKCS#8 PEM import/export so the
//!   UI can store keys in the OS keyring without learning curve25519.
//! - [`verify_manifest`] — the verify command's worker: re-hashes
//!   each file at the path it claims, compares BLAKE3 roots, verifies
//!   the Merkle root, validates the detached signature, and (when the
//!   `tsa` feature is on) checks the RFC 3161 timestamp.
//!
//! # What's NOT here
//!
//! - The Tauri command surface for "generate / import / export
//!   signing key from settings UI" lives in `apps/copythat-ui/
//!   src-tauri/src/commands/provenance.rs`. This crate stays
//!   keyring-agnostic so it builds clean for the CLI, the smoke
//!   test, and downstream embedding.
//! - The default TSA URL (`https://freetsa.org/tsr`) is referenced
//!   but the actual HTTP call lives behind the `tsa` feature flag
//!   (off by default — `reqwest` + `rfc3161-client` add a heavy
//!   TLS+async dep tree and the smoke test runs offline per Phase 43
//!   spec).
//!
//! # Manifest format (CBOR)
//!
//! The on-disk layout is canonical CBOR (RFC 8949) of the
//! [`ProvenanceManifest`] struct — see the `serde` derives on the
//! type for field ordering. The `signature` and `timestamp` fields
//! are `Option<...>`: an unsigned-untimestamped manifest is still
//! valid, with the verify command reporting "Manifest valid for N
//! files; SIGNATURE NOT PRESENT" so users opting in to provenance
//! get integrity (digest matching) without forcing every job into a
//! signing flow.
//!
//! # Threat model
//!
//! - The manifest authenticates that **every byte at the recorded
//!   `rel_path` matches the `blake3_root` taken at copy time**. It
//!   does NOT authenticate that the source bytes were genuine — only
//!   that the destination bytes match what was hashed. Combined with
//!   the optional RFC 3161 timestamp, the manifest establishes a
//!   "this content existed in this form at this point in time"
//!   claim.
//! - A tampered destination (one byte flipped) re-hashes to a
//!   different root, and the verify command flags the file. The
//!   Merkle root also changes; an attacker cannot edit one file and
//!   re-sign the manifest without the private key.
//! - A tampered manifest itself (file paths, sizes, roots, anything)
//!   invalidates the detached ed25519 signature — assuming the
//!   verifier trusts the public key. Public-key distribution is
//!   out-of-band (the user's responsibility).
//! - The sink stores Bao outboards in memory until
//!   `finalize_to_path` writes the manifest. For a tree that totals
//!   ~64 GiB of source bytes, expect ~1 GiB of outboard memory peak.
//!   Future work: stream outboards to per-file sidecars instead of
//!   buffering. Tracked as a follow-up; the manifest schema already
//!   permits an empty `bao_outboard` (verify falls back to a full
//!   re-hash).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod encoder;
mod error;
mod manifest;
mod sign;
mod sink;
#[cfg(feature = "tsa")]
mod timestamp;
mod verify;

pub use encoder::{BaoOutboardEncoder, RootOnlyEncoder};
pub use error::{ProvenanceError, ProvenanceErrorKind};
pub use manifest::{
    DEFAULT_MANIFEST_FILENAME, FileRecord, ProvenanceManifest, Rfc3161Token, Signature,
    canonical_cbor_bytes, manifest_root_blake3, manifest_signing_bytes, parse_manifest_cbor,
    validate_rel_path, write_manifest_cbor,
};
pub use sign::{
    SigningKey, VerifyingKey, generate_signing_key, signing_key_from_pem, signing_key_to_pem,
    verifying_key_from_pem, verifying_key_to_pem,
};
pub use sink::{CopyThatProvenanceSink, SinkConfig};
pub use verify::{VerificationOutcome, VerifyReport, verify_manifest};

/// Default TSA URL — free, publicly trusted RFC 3161 service.
/// Opt-in only; the `tsa` feature must be enabled to actually request
/// timestamps. Documented here so the Settings UI can pre-fill the
/// "Default TSA URL" input with a sensible value.
pub const DEFAULT_TSA_URL: &str = "https://freetsa.org/tsr";
