//! Manifest verification — the "did this tree get tampered with?"
//! pass.
//!
//! Public entrypoint: [`verify_manifest`]. Re-hashes each file at
//! the path it claims, compares against the recorded `blake3_root`,
//! verifies the Merkle root over per-file roots, validates the
//! detached ed25519 signature (when one is present), and validates
//! the RFC 3161 timestamp (when the `tsa` feature is on AND the
//! manifest carries a timestamp).
//!
//! Returns a [`VerifyReport`] with a per-file outcome list so the
//! verify command can print "Manifest valid for N files; tampered:
//! M (paths…)." without re-walking the tree.

use std::path::{Path, PathBuf};

use ed25519_dalek::Verifier;

use crate::error::{ProvenanceError, ProvenanceErrorKind};
use crate::manifest::{
    FileRecord, ProvenanceManifest, manifest_root_blake3, manifest_signing_bytes,
    parse_manifest_cbor,
};

/// Per-file verification outcome. The verify command surfaces a list
/// of these, then prints aggregate counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationOutcome {
    /// File on disk matches the manifest's BLAKE3 root.
    Ok,
    /// File on disk hashes to a different root than the manifest
    /// claims. Carries the `(expected, actual)` pair so a verbose
    /// CLI mode can print the diff.
    Tampered {
        /// What the manifest said the BLAKE3 root should be.
        expected: [u8; 32],
        /// What we got when re-hashing the destination.
        actual: [u8; 32],
    },
    /// Manifest references a path that doesn't exist (or isn't
    /// readable) at verify time. Likely a moved or deleted file.
    Missing {
        /// I/O error (often `NotFound` / `PermissionDenied`).
        error: String,
    },
}

/// Aggregate report from [`verify_manifest`].
#[derive(Debug, Clone)]
pub struct VerifyReport {
    /// One outcome per [`FileRecord`] in the manifest, in the same
    /// order.
    pub per_file: Vec<(PathBuf, VerificationOutcome)>,
    /// Whether the manifest's `merkle_root` matched the freshly
    /// computed Merkle root over the per-file BLAKE3 roots in the
    /// manifest. (This is *manifest-internal* consistency; the
    /// per-file outcomes above check on-disk vs. manifest.)
    pub merkle_root_ok: bool,
    /// Signature outcome:
    /// - `Some(true)`: signature present and valid.
    /// - `Some(false)`: signature present but invalid.
    /// - `None`: no signature was attached to the manifest.
    pub signature_ok: Option<bool>,
    /// Timestamp outcome — same Tri-state as `signature_ok`.
    /// `Some(false)` when the `tsa` feature is off but a timestamp
    /// is attached (we can't verify it without the feature).
    pub timestamp_ok: Option<bool>,
    /// Number of files matching `Ok`.
    pub ok_count: usize,
    /// Number of files matching `Tampered`.
    pub tampered_count: usize,
    /// Number of files matching `Missing`.
    pub missing_count: usize,
}

impl VerifyReport {
    /// "Everything verified" — Merkle root good, no signature
    /// failure, no timestamp failure, no per-file mismatch, no
    /// missing files. Helper for CLI exit-code branching.
    pub fn all_clean(&self) -> bool {
        self.merkle_root_ok
            && self.signature_ok != Some(false)
            && self.timestamp_ok != Some(false)
            && self.tampered_count == 0
            && self.missing_count == 0
    }
}

/// Run the verify pass against a manifest at `manifest_path`. The
/// destination tree is `manifest.dst_root` from the parsed manifest;
/// the verify pass re-hashes `<dst_root>/<rel_path>` for each
/// [`FileRecord`].
///
/// `expected_public_key` lets the caller pin a trusted public key —
/// when `Some`, a signature whose `public_key` field doesn't match
/// is treated as an invalid signature even if the cryptographic
/// check would otherwise pass. Setting it to `None` accepts any
/// signature that validates against its embedded public key (TOFU
/// model — fine for ad-hoc verification, weak for adversarial
/// settings).
pub fn verify_manifest(
    manifest_path: &Path,
    expected_public_key: Option<&[u8; 32]>,
) -> Result<VerifyReport, ProvenanceError> {
    let bytes = std::fs::read(manifest_path).map_err(|e| ProvenanceError::Io {
        path: manifest_path.to_path_buf(),
        source: e,
    })?;
    let manifest = parse_manifest_cbor(&bytes)?;

    let merkle_root_ok = manifest.merkle_root == manifest_root_blake3(&manifest.files);

    let signature_ok = check_signature(&manifest, expected_public_key)?;
    let timestamp_ok = check_timestamp(&manifest);

    let mut per_file = Vec::with_capacity(manifest.files.len());
    let mut ok_count = 0usize;
    let mut tampered_count = 0usize;
    let mut missing_count = 0usize;

    for record in &manifest.files {
        let dst = manifest.dst_root.join(rel_to_native(&record.rel_path));
        let outcome = verify_one(&dst, record);
        match &outcome {
            VerificationOutcome::Ok => ok_count += 1,
            VerificationOutcome::Tampered { .. } => tampered_count += 1,
            VerificationOutcome::Missing { .. } => missing_count += 1,
        }
        per_file.push((dst, outcome));
    }

    Ok(VerifyReport {
        per_file,
        merkle_root_ok,
        signature_ok,
        timestamp_ok,
        ok_count,
        tampered_count,
        missing_count,
    })
}

fn verify_one(path: &Path, record: &FileRecord) -> VerificationOutcome {
    match std::fs::File::open(path) {
        Ok(mut f) => {
            let mut hasher = blake3::Hasher::new();
            // Use a 1 MiB read buffer to match the engine's default;
            // smaller buffers lose throughput on big files.
            let mut buf = vec![0u8; 1024 * 1024];
            loop {
                use std::io::Read;
                match f.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        hasher.update(&buf[..n]);
                    }
                    Err(e) => {
                        return VerificationOutcome::Missing {
                            error: e.to_string(),
                        };
                    }
                }
            }
            let actual: [u8; 32] = *hasher.finalize().as_bytes();
            if actual == record.blake3_root {
                VerificationOutcome::Ok
            } else {
                VerificationOutcome::Tampered {
                    expected: record.blake3_root,
                    actual,
                }
            }
        }
        Err(e) => VerificationOutcome::Missing {
            error: e.to_string(),
        },
    }
}

fn check_signature(
    manifest: &ProvenanceManifest,
    expected_public_key: Option<&[u8; 32]>,
) -> Result<Option<bool>, ProvenanceError> {
    let Some(sig) = manifest.signature.as_ref() else {
        return Ok(None);
    };

    if let Some(expected) = expected_public_key {
        if expected != &sig.public_key {
            return Ok(Some(false));
        }
    }

    let vk = ed25519_dalek::VerifyingKey::from_bytes(&sig.public_key)
        .map_err(|e| ProvenanceError::Ed25519Key(format!("invalid public key: {e}")))?;
    let signature = ed25519_dalek::Signature::from_bytes(&sig.sig);
    let signing_bytes = manifest_signing_bytes(manifest)?;
    let ok = vk.verify(&signing_bytes, &signature).is_ok();
    Ok(Some(ok))
}

#[cfg(feature = "tsa")]
fn check_timestamp(manifest: &ProvenanceManifest) -> Option<bool> {
    let Some(token) = manifest.timestamp.as_ref() else {
        return None;
    };
    Some(crate::timestamp::verify(token, manifest).is_ok())
}

#[cfg(not(feature = "tsa"))]
fn check_timestamp(manifest: &ProvenanceManifest) -> Option<bool> {
    // Token present but we can't verify without the feature → flag
    // as failed so the caller's `all_clean` check fires.
    if manifest.timestamp.is_some() {
        Some(false)
    } else {
        None
    }
}

/// Translate a manifest's forward-slash `rel_path` into the host's
/// native path representation. Idempotent on Unix (no separator
/// change); on Windows, replaces `/` with `\`.
fn rel_to_native(rel: &str) -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(rel.replace('/', "\\"))
    } else {
        PathBuf::from(rel)
    }
}

/// Convenience predicate the CLI uses to map `VerifyReport` →
/// classified error for the user.
#[allow(dead_code)]
pub(crate) fn report_to_classified_error(report: &VerifyReport) -> Option<ProvenanceError> {
    if !report.merkle_root_ok {
        return Some(ProvenanceError::classify(
            ProvenanceErrorKind::MerkleMismatch,
            "manifest merkle_root does not match per-file roots",
        ));
    }
    if report.signature_ok == Some(false) {
        return Some(ProvenanceError::classify(
            ProvenanceErrorKind::BadSignature,
            "ed25519 signature did not validate",
        ));
    }
    if report.timestamp_ok == Some(false) {
        return Some(ProvenanceError::classify(
            ProvenanceErrorKind::BadTimestamp,
            "RFC 3161 timestamp did not validate",
        ));
    }
    if report.tampered_count > 0 {
        return Some(ProvenanceError::classify(
            ProvenanceErrorKind::Tampered,
            format!("{} file(s) tampered", report.tampered_count),
        ));
    }
    if report.missing_count > 0 {
        return Some(ProvenanceError::classify(
            ProvenanceErrorKind::Missing,
            format!("{} file(s) missing", report.missing_count),
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::DEFAULT_MANIFEST_FILENAME;
    use crate::sign::generate_signing_key;
    use crate::sink::{CopyThatProvenanceSink, SinkConfig};
    use copythat_core::ProvenanceSink;
    use tempfile::TempDir;

    fn write_file(path: &Path, bytes: &[u8]) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, bytes).unwrap();
    }

    /// End-to-end fixture: build a sink, pretend the engine fed two
    /// files through it (we drive the encoder ourselves to keep
    /// the test off the engine), finalise to a manifest, run
    /// `verify_manifest` against the on-disk manifest.
    fn fixture_with_two_files(
        tamper: bool,
        sign: bool,
    ) -> (TempDir, std::path::PathBuf, VerifyReport) {
        let tmp = TempDir::new().unwrap();
        let src_root = tmp.path().join("src");
        let dst_root = tmp.path().join("dst");
        std::fs::create_dir_all(&src_root).unwrap();
        std::fs::create_dir_all(&dst_root).unwrap();

        // Two files at known relative paths, with known content.
        let files = [
            ("a.txt", b"alpha bytes" as &[u8]),
            ("nested/b.txt", b"beta bytes"),
        ];
        for (rel, bytes) in &files {
            write_file(&src_root.join(rel), bytes);
            write_file(&dst_root.join(rel), bytes);
        }

        let mut config = SinkConfig::new(src_root.clone(), dst_root.clone());
        if sign {
            config.signing_key = Some(generate_signing_key());
        }
        let sink = CopyThatProvenanceSink::new(config);

        for (rel, bytes) in &files {
            let mut enc = sink.make_encoder();
            enc.update(bytes);
            let (root, outboard) = enc.finalize();
            sink.record_file(
                &src_root.join(rel),
                &dst_root.join(rel),
                bytes.len() as u64,
                root,
                outboard,
            );
        }

        let manifest_path = dst_root.join(DEFAULT_MANIFEST_FILENAME);
        sink.finalize_to_path(&manifest_path).unwrap();

        if tamper {
            // Flip one byte in the second file (the manifest will
            // hash to the original; the on-disk content now diverges).
            let target = dst_root.join("nested/b.txt");
            let mut bytes = std::fs::read(&target).unwrap();
            bytes[0] ^= 0xFF;
            std::fs::write(&target, bytes).unwrap();
        }

        let report = verify_manifest(&manifest_path, None).unwrap();
        (tmp, manifest_path, report)
    }

    #[test]
    fn clean_manifest_verifies() {
        let (_tmp, _path, report) = fixture_with_two_files(false, false);
        assert!(report.all_clean(), "{report:?}");
        assert_eq!(report.ok_count, 2);
        assert_eq!(report.tampered_count, 0);
        assert_eq!(report.signature_ok, None);
    }

    #[test]
    fn tamper_one_byte_flags_only_the_tampered_file() {
        let (_tmp, _path, report) = fixture_with_two_files(true, false);
        assert!(!report.all_clean());
        assert_eq!(report.ok_count, 1);
        assert_eq!(report.tampered_count, 1);
        let tampered_path = &report
            .per_file
            .iter()
            .find(|(_, o)| matches!(o, VerificationOutcome::Tampered { .. }))
            .unwrap()
            .0;
        assert!(tampered_path.ends_with("b.txt"));
    }

    #[test]
    fn signed_manifest_validates() {
        let (_tmp, _path, report) = fixture_with_two_files(false, true);
        assert_eq!(report.signature_ok, Some(true));
        assert!(report.all_clean());
    }
}
