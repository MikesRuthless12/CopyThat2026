//! Phase 49o — snapshot diff / compare via manifests.
//!
//! Pure manifest math (reads no chunk bytes): compare two snapshots and
//! report each file as Added / Removed / Modified / Unchanged plus the
//! chunk-level incremental cost — how many *new* distinct chunk bytes the
//! `to` snapshot adds over `from`. Mirrors the delta-resume
//! [`delta_plan`](crate::delta_plan) primitive, so the "what did this
//! backup actually cost?" answer is cheap and exact.

use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::error::{ChunkStoreError, Result};
use crate::repository::{Repository, SnapshotId};
use crate::types::{Blake3Hash, Manifest};

/// How a file changed between two snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FileChange {
    /// Present in `to`, absent in `from`.
    Added,
    /// Present in `from`, absent in `to`.
    Removed,
    /// Present in both, different content (file hash differs).
    Modified,
    /// Present in both, identical content.
    Unchanged,
}

/// One file's diff between two snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileDiff {
    /// Logical path.
    pub path: String,
    /// Change classification.
    pub change: FileChange,
    /// Size in `from` (`None` if Added).
    pub old_size: Option<u64>,
    /// Size in `to` (`None` if Removed).
    pub new_size: Option<u64>,
    /// Chunks in `to`'s version already present in `from` (dedup reuse).
    pub chunks_shared: u64,
    /// Chunks in `to`'s version that `from` did not have.
    pub chunks_changed: u64,
    /// Sum of the lengths of this file's changed chunks.
    pub bytes_added: u64,
}

/// Diff between two snapshots — per-file detail plus repo-level totals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotDiff {
    /// The baseline snapshot id.
    pub from_id: u64,
    /// The compared snapshot id.
    pub to_id: u64,
    /// Per-file diffs, ordered by path.
    pub files: Vec<FileDiff>,
    /// Number of Added files.
    pub added: u64,
    /// Number of Removed files.
    pub removed: u64,
    /// Number of Modified files.
    pub modified: u64,
    /// Number of Unchanged files.
    pub unchanged: u64,
    /// Distinct chunk bytes present in `to` but not in `from` — the true
    /// incremental storage cost of `to` over `from`.
    pub bytes_added: u64,
}

impl Repository {
    /// Compare snapshot `from` to snapshot `to`: per-file
    /// Added/Removed/Modified/Unchanged plus the chunk-level incremental
    /// cost. Pure manifest math — no chunk bytes are read.
    ///
    /// Errors with [`ChunkStoreError::SnapshotNotFound`] if either id is
    /// not in the catalog.
    pub fn diff_snapshots(&self, from: SnapshotId, to: SnapshotId) -> Result<SnapshotDiff> {
        let from_snap = self
            .snapshot(from)?
            .ok_or(ChunkStoreError::SnapshotNotFound(from.as_u64()))?;
        let to_snap = self
            .snapshot(to)?
            .ok_or(ChunkStoreError::SnapshotNotFound(to.as_u64()))?;

        let from_files: HashMap<&str, &Manifest> = from_snap
            .files
            .iter()
            .map(|f| (f.path.as_str(), &f.manifest))
            .collect();
        let to_files: HashMap<&str, &Manifest> = to_snap
            .files
            .iter()
            .map(|f| (f.path.as_str(), &f.manifest))
            .collect();

        // Distinct chunks already present in `from` — the baseline against
        // which "new" chunks (for Added files + the repo total) are judged.
        let from_chunks: HashSet<Blake3Hash> = from_snap
            .files
            .iter()
            .flat_map(|f| f.manifest.chunks.iter().map(|c| c.hash))
            .collect();

        let mut paths: BTreeSet<&str> = BTreeSet::new();
        paths.extend(from_files.keys().copied());
        paths.extend(to_files.keys().copied());

        let mut files = Vec::with_capacity(paths.len());
        let (mut added, mut removed, mut modified, mut unchanged) = (0u64, 0u64, 0u64, 0u64);

        for path in paths {
            let diff = match (from_files.get(path), to_files.get(path)) {
                (None, Some(new_m)) => {
                    added += 1;
                    let changed: Vec<_> = new_m
                        .chunks
                        .iter()
                        .filter(|c| !from_chunks.contains(&c.hash))
                        .collect();
                    FileDiff {
                        path: path.to_string(),
                        change: FileChange::Added,
                        old_size: None,
                        new_size: Some(new_m.size),
                        chunks_shared: (new_m.chunks.len() - changed.len()) as u64,
                        chunks_changed: changed.len() as u64,
                        bytes_added: changed.iter().map(|c| u64::from(c.len)).sum(),
                    }
                }
                (Some(old_m), None) => {
                    removed += 1;
                    FileDiff {
                        path: path.to_string(),
                        change: FileChange::Removed,
                        old_size: Some(old_m.size),
                        new_size: None,
                        chunks_shared: 0,
                        chunks_changed: 0,
                        bytes_added: 0,
                    }
                }
                (Some(old_m), Some(new_m)) if old_m.file_hash == new_m.file_hash => {
                    unchanged += 1;
                    FileDiff {
                        path: path.to_string(),
                        change: FileChange::Unchanged,
                        old_size: Some(old_m.size),
                        new_size: Some(new_m.size),
                        chunks_shared: new_m.chunks.len() as u64,
                        chunks_changed: 0,
                        bytes_added: 0,
                    }
                }
                (Some(old_m), Some(new_m)) => {
                    modified += 1;
                    // Measure new chunks against the ENTIRE `from` snapshot —
                    // the same baseline the Added arm and the repo-level
                    // `bytes_added` use — not just this file's own prior
                    // version. A chunk whose bytes already exist elsewhere in
                    // `from` is not new storage, so charging it here (via
                    // delta_plan against only old_m) over-reported the
                    // incremental cost.
                    let changed: Vec<_> = new_m
                        .chunks
                        .iter()
                        .filter(|c| !from_chunks.contains(&c.hash))
                        .collect();
                    let changed_n = changed.len() as u64;
                    FileDiff {
                        path: path.to_string(),
                        change: FileChange::Modified,
                        old_size: Some(old_m.size),
                        new_size: Some(new_m.size),
                        chunks_shared: new_m.chunks.len() as u64 - changed_n,
                        chunks_changed: changed_n,
                        bytes_added: changed.iter().map(|c| u64::from(c.len)).sum(),
                    }
                }
                (None, None) => unreachable!("path came from the union of both snapshots"),
            };
            files.push(diff);
        }

        // Repo-level incremental cost: distinct chunks in `to` not in `from`.
        let mut to_only: HashSet<Blake3Hash> = HashSet::new();
        let mut bytes_added = 0u64;
        for f in &to_snap.files {
            for c in &f.manifest.chunks {
                if !from_chunks.contains(&c.hash) && to_only.insert(c.hash) {
                    bytes_added += u64::from(c.len);
                }
            }
        }

        Ok(SnapshotDiff {
            from_id: from.as_u64(),
            to_id: to.as_u64(),
            files,
            added,
            removed,
            modified,
            unchanged,
            bytes_added,
        })
    }
}
