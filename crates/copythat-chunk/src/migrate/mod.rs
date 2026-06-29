//! Phase 50 (moonshot) — cross-tool repository migration for CDR-0.
//!
//! The CDR-0 promise (see [`docs/spec/CDR-0.md`]) is that migrating *in*
//! from another deduplicating-backup tool reuses the source's chunks
//! rather than re-ingesting terabytes. This module is the migration
//! entry point, the source detector, and the per-tool importers.
//!
//! # Status
//!
//! - **CDR-0 → CDR-0** ([`RepoFormat::Cdr`]) and **restic → CDR-0**
//!   ([`RepoFormat::Restic`]) are implemented + tested (restic against a
//!   real `restic 0.17.3` v2 fixture). restic chunk IDs aren't portable
//!   (per-repo polynomial + SHA-256 of plaintext), so the importer
//!   reconstructs file bytes and lets the CDR store re-chunk with its own
//!   FastCDC + BLAKE3 — a faithful content migration.
//! - **[`RepoFormat::detect`]** recognises restic / Borg / Kopia / CDR-0
//!   from on-disk markers (no decryption).
//! - **Borg / Kopia → CDR-0** return a typed
//!   [`MigrateError::SourceUnsupported`]. Kopia's crypto is in-tree but
//!   its keyed-hash index + object format is a separate build-out; Borg
//!   additionally needs a MessagePack codec (the one genuinely-new
//!   dependency). See `docs/spec/CDR-0.md §11`.

mod restic;

use std::path::{Path, PathBuf};

use crate::cdr::{CDR_ALGO, CDR_SPEC_VERSION};
use crate::error::ChunkStoreError;
use crate::repository::{Repository, SnapshotId};

/// A recognised (or unrecognised) source repository format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoFormat {
    /// A CDR-0 repository (this crate's [`Repository`]).
    Cdr,
    /// A restic repository.
    Restic,
    /// A Borg (borgbackup) 1.x repository.
    Borg,
    /// A Kopia repository.
    Kopia,
    /// Nothing recognisable at the path.
    Unknown,
}

impl RepoFormat {
    /// Stable lowercase tag, also the CLI selector.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cdr => "cdr",
            Self::Restic => "restic",
            Self::Borg => "borg",
            Self::Kopia => "kopia",
            Self::Unknown => "unknown",
        }
    }

    /// Parse a CLI selector (`cdr` / `restic` / `borg` / `kopia`).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "cdr" => Some(Self::Cdr),
            "restic" => Some(Self::Restic),
            "borg" => Some(Self::Borg),
            "kopia" => Some(Self::Kopia),
            _ => None,
        }
    }

    /// Sniff a repository's format from its on-disk marker files. Reads
    /// no secrets and decrypts nothing — purely a layout probe.
    #[must_use]
    pub fn detect(root: &Path) -> Self {
        if root.join("cdr.toml").is_file() || root.join("repository.redb").is_file() {
            return Self::Cdr;
        }
        if root.join("kopia.repository").is_file() {
            return Self::Kopia;
        }
        if root.join("config").is_file()
            && root.join("data").is_dir()
            && root.join("snapshots").is_dir()
        {
            return Self::Restic;
        }
        // Borg 1.x: `README` + `config` (INI) + `data/`, and — unlike
        // restic — no `snapshots/` directory.
        if root.join("README").is_file()
            && root.join("config").is_file()
            && root.join("data").is_dir()
        {
            return Self::Borg;
        }
        Self::Unknown
    }
}

/// Errors from migration / detection.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    /// Filesystem error.
    #[error("I/O error at {path}: {source}")]
    Io {
        /// Offending path.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },

    /// The underlying chunk store / repository failed.
    #[error(transparent)]
    Store(#[from] ChunkStoreError),

    /// Nothing recognisable at the source path.
    #[error("no recognised backup repository found at {0}")]
    Unrecognized(PathBuf),

    /// The detected source format differs from the one requested.
    #[error("requested source format {requested} but detected {detected} at {path}")]
    FormatMismatch {
        /// What the caller asked for.
        requested: &'static str,
        /// What the layout actually looks like.
        detected: &'static str,
        /// The source path.
        path: PathBuf,
    },

    /// This (encrypted) source tool needs a passphrase that wasn't given.
    #[error("{tool} repositories are encrypted — pass the repository passphrase to migrate")]
    NeedsPassphrase {
        /// The source tool.
        tool: &'static str,
    },

    /// Decryption / MAC verification failed (wrong passphrase or
    /// corruption).
    #[error("decryption failed: {0}")]
    Decrypt(String),

    /// A base64 / JSON / zstd / structural decode failed.
    #[error("decode error: {0}")]
    Decode(String),

    /// The source repository's structure was not as expected.
    #[error("unexpected repository structure: {0}")]
    Format(String),

    /// A real importer for this tool is not implemented; the message
    /// names the concrete blocker.
    #[error("migrating from {tool} is not yet supported: {reason}")]
    SourceUnsupported {
        /// The source tool.
        tool: &'static str,
        /// Why — the specific missing capability.
        reason: String,
    },
}

/// Summary of a completed migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MigrateReport {
    /// Snapshots written to the destination.
    pub snapshots: u64,
    /// File entries copied across all snapshots.
    pub files: u64,
    /// Non-content nodes (symlinks, devices, FIFOs, empty dirs) that
    /// carry no chunk data — counted, not silently dropped, so the
    /// caller can surface "N skipped" rather than reporting clean.
    pub skipped: u64,
}

/// Write the CDR-0 repository descriptor (`cdr.toml`, spec §9) into
/// `root`. Hand-rolled (3 fields) so the chunk crate needs no TOML
/// dependency.
pub fn write_cdr_descriptor(root: &Path) -> std::result::Result<(), MigrateError> {
    let body = format!(
        "spec_version = {CDR_SPEC_VERSION}\nalgo = \"{CDR_ALGO}\"\ncreated_by = \"copythat-chunk {}\"\n",
        env!("CARGO_PKG_VERSION"),
    );
    let path = root.join("cdr.toml");
    std::fs::write(&path, body).map_err(|e| MigrateError::Io { path, source: e })
}

/// Migrate a source repository at `src` into a CDR-0 repository at
/// `dst_root` (created if absent). `from` must match the format detected
/// at `src`. `passphrase` is required for encrypted sources (restic).
pub fn migrate(
    from: RepoFormat,
    src: &Path,
    dst_root: &Path,
    passphrase: Option<&str>,
) -> std::result::Result<MigrateReport, MigrateError> {
    let detected = RepoFormat::detect(src);
    if detected == RepoFormat::Unknown {
        return Err(MigrateError::Unrecognized(src.to_path_buf()));
    }
    if detected != from {
        return Err(MigrateError::FormatMismatch {
            requested: from.as_str(),
            detected: detected.as_str(),
            path: src.to_path_buf(),
        });
    }
    match from {
        RepoFormat::Cdr => migrate_cdr_to_cdr(src, dst_root),
        RepoFormat::Restic => {
            let pw = passphrase.ok_or(MigrateError::NeedsPassphrase { tool: "restic" })?;
            restic::import_restic(src, pw, dst_root)
        }
        RepoFormat::Borg => Err(MigrateError::SourceUnsupported {
            tool: "borg",
            reason: "Borg needs a MessagePack parser for its archive/item streams (new \
                     dep), and default repokey/keyfile modes are encrypted (AES-256-CTR + \
                     HMAC + PBKDF2, or 2.0 AEAD + Argon2). See docs/spec/CDR-0.md §11."
                .to_string(),
        }),
        RepoFormat::Kopia => Err(MigrateError::SourceUnsupported {
            tool: "kopia",
            reason: "Kopia's crypto is in-tree, but its keyed-hash index (v1/v2 binary) + \
                     object/indirect format is a separate build-out. See docs/spec/CDR-0.md §11."
                .to_string(),
        }),
        RepoFormat::Unknown => Err(MigrateError::Unrecognized(src.to_path_buf())),
    }
}

/// Copy every snapshot from one CDR-0 repository into another,
/// re-ingesting file bytes (so the destination's chunk store is built
/// fresh + self-consistent). Returns the counts.
fn migrate_cdr_to_cdr(
    src: &Path,
    dst_root: &Path,
) -> std::result::Result<MigrateReport, MigrateError> {
    // Refuse migrating a repository onto itself — opening the same redb
    // files twice would otherwise fail with an opaque "Database already
    // open".
    if let (Ok(s), Ok(d)) = (src.canonicalize(), dst_root.canonicalize()) {
        if s == d {
            return Err(MigrateError::Format(
                "source and destination are the same repository".into(),
            ));
        }
    }

    let source = Repository::open(src)?;
    let dest = Repository::open(dst_root)?;
    write_cdr_descriptor(dst_root)?;
    let chunker = crate::chunker::Chunker::default();

    let mut report = MigrateReport::default();
    for summary in source.snapshots()? {
        let Some(snap) = source.snapshot(SnapshotId(summary.id))? else {
            continue;
        };
        // Re-ingest each file independently so peak memory is one file,
        // not the whole snapshot.
        let mut entries: Vec<crate::repository::FileEntry> = Vec::with_capacity(snap.files.len());
        for entry in &snap.files {
            let mut bytes = Vec::with_capacity(entry.manifest.size as usize);
            for chunk in &entry.manifest.chunks {
                let data = source.store().get(&chunk.hash)?.ok_or_else(|| {
                    ChunkStoreError::MissingChunk {
                        hash: crate::types::hex_of(&chunk.hash),
                    }
                })?;
                bytes.extend_from_slice(&data);
            }
            let manifest = crate::manifest::chunk_into_store(dest.store(), &chunker, &bytes)?.1;
            entries.push(crate::repository::FileEntry {
                path: entry.path.clone(),
                manifest,
            });
            report.files += 1;
        }
        dest.record(snap.kind, &snap.label, snap.created_at_ms, entries)?;
        report.snapshots += 1;
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::SnapshotKind;

    #[test]
    fn parse_and_as_str_round_trip() {
        for tool in ["cdr", "restic", "borg", "kopia"] {
            assert_eq!(RepoFormat::parse(tool).unwrap().as_str(), tool);
        }
        assert!(RepoFormat::parse("zip").is_none());
    }

    #[test]
    fn detect_cdr_repository() {
        let tmp = tempfile::tempdir().unwrap();
        let _repo = Repository::open(tmp.path()).unwrap();
        assert_eq!(RepoFormat::detect(tmp.path()), RepoFormat::Cdr);
    }

    #[test]
    fn detect_restic_borg_kopia_layouts() {
        let tmp = tempfile::tempdir().unwrap();

        let restic = tmp.path().join("restic");
        std::fs::create_dir_all(restic.join("data")).unwrap();
        std::fs::create_dir_all(restic.join("snapshots")).unwrap();
        std::fs::write(restic.join("config"), b"x").unwrap();
        assert_eq!(RepoFormat::detect(&restic), RepoFormat::Restic);

        let borg = tmp.path().join("borg");
        std::fs::create_dir_all(borg.join("data")).unwrap();
        std::fs::write(borg.join("README"), b"borg").unwrap();
        std::fs::write(borg.join("config"), b"[repository]").unwrap();
        assert_eq!(RepoFormat::detect(&borg), RepoFormat::Borg);

        let kopia = tmp.path().join("kopia");
        std::fs::create_dir_all(&kopia).unwrap();
        std::fs::write(kopia.join("kopia.repository"), b"{}").unwrap();
        assert_eq!(RepoFormat::detect(&kopia), RepoFormat::Kopia);

        let empty = tmp.path().join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        assert_eq!(RepoFormat::detect(&empty), RepoFormat::Unknown);
    }

    #[test]
    fn migrate_cdr_to_cdr_round_trips() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");

        let bytes = {
            let repo = Repository::open(&src).unwrap();
            let mut b = vec![0u8; 1024 * 1024];
            let mut s = 0xABCDu64;
            for x in &mut b {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
                *x = (s >> 33) as u8;
            }
            repo.snapshot_bytes(SnapshotKind::Backup, "src snap", 1000, &[("/f.bin", &b)])
                .unwrap();
            b
        };

        let report = migrate(RepoFormat::Cdr, &src, &dst, None).unwrap();
        assert_eq!(report.snapshots, 1);
        assert_eq!(report.files, 1);
        assert!(dst.join("cdr.toml").is_file());

        let dest = Repository::open(&dst).unwrap();
        let list = dest.snapshots().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].kind, SnapshotKind::Backup);
        let fs = dest.snapshot_at("/f.bin", i64::MAX).unwrap().unwrap();
        let out = tmp.path().join("restored.bin");
        dest.restore(&fs, &out).unwrap();
        assert_eq!(std::fs::read(&out).unwrap(), bytes);
    }

    #[test]
    fn migrate_errors_are_typed() {
        let tmp = tempfile::tempdir().unwrap();
        // restic layout, no passphrase → NeedsPassphrase.
        let restic = tmp.path().join("restic");
        std::fs::create_dir_all(restic.join("data")).unwrap();
        std::fs::create_dir_all(restic.join("snapshots")).unwrap();
        std::fs::write(restic.join("config"), b"x").unwrap();
        let err = migrate(RepoFormat::Restic, &restic, &tmp.path().join("o1"), None).unwrap_err();
        assert!(matches!(
            err,
            MigrateError::NeedsPassphrase { tool: "restic" }
        ));

        // borg layout still unsupported.
        let borg = tmp.path().join("borg");
        std::fs::create_dir_all(borg.join("data")).unwrap();
        std::fs::write(borg.join("README"), b"borg").unwrap();
        std::fs::write(borg.join("config"), b"[repository]").unwrap();
        let err = migrate(RepoFormat::Borg, &borg, &tmp.path().join("o2"), None).unwrap_err();
        assert!(matches!(
            err,
            MigrateError::SourceUnsupported { tool: "borg", .. }
        ));

        // Format mismatch: ask for borg, point at restic.
        let err = migrate(RepoFormat::Borg, &restic, &tmp.path().join("o3"), None).unwrap_err();
        assert!(matches!(err, MigrateError::FormatMismatch { .. }));

        // Nothing there.
        let empty = tmp.path().join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        let err = migrate(RepoFormat::Cdr, &empty, &tmp.path().join("o4"), None).unwrap_err();
        assert!(matches!(err, MigrateError::Unrecognized(_)));
    }
}
