//! Phase 49r — repository statistics / reports.
//!
//! Analytics beyond the dedup hero: a per-kind breakdown, cumulative
//! storage growth over time, and the most-versioned files — all derived
//! from the snapshot catalog in a single pass (no chunk reads), plus a
//! hand-rolled markdown rendering for export.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::repository::{RepoStats, Repository, SnapshotKind};
use crate::types::Blake3Hash;

/// One day's worth of milliseconds (UTC growth bucketing).
const MS_PER_DAY: i64 = 86_400_000;

/// Snapshot count + effective bytes for one [`SnapshotKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindBreakdown {
    /// The snapshot kind.
    pub kind: SnapshotKind,
    /// How many snapshots of this kind exist.
    pub count: u64,
    /// Sum of logical file sizes across those snapshots (pre-dedup).
    pub effective_bytes: u64,
}

/// A point on the cumulative-unique-bytes growth curve (one per UTC day
/// that has at least one snapshot).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthPoint {
    /// UTC-day floor, milliseconds since epoch.
    pub ts_ms: i64,
    /// Distinct chunk bytes accumulated up to and including this day.
    pub cumulative_unique_bytes: u64,
    /// Snapshots recorded up to and including this day.
    pub snapshot_count: u64,
}

/// A file that appears across multiple snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopFile {
    /// Logical path.
    pub path: String,
    /// How many snapshots contain this path.
    pub versions: u64,
    /// Largest logical size this path reached.
    pub max_size: u64,
}

/// The full repository report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoReport {
    /// The dedup hero numbers.
    pub stats: RepoStats,
    /// Per-kind snapshot breakdown (ordered by kind).
    pub by_kind: Vec<KindBreakdown>,
    /// Cumulative storage growth, one point per active UTC day.
    pub growth: Vec<GrowthPoint>,
    /// The most-versioned files (descending), capped at `top_n`.
    pub top_files: Vec<TopFile>,
    /// Fraction of effective bytes saved by dedup, `0.0..=1.0`.
    pub dedup_ratio: f64,
}

impl Repository {
    /// Compute the repository report. One pass over the snapshot catalog
    /// (no chunk bytes are read); growth is bucketed by UTC day so the
    /// point count stays bounded regardless of snapshot frequency.
    pub fn report(&self, top_n: usize) -> Result<RepoReport> {
        let stats = self.stats()?;
        let mut snaps = self.load_all_snapshots()?;
        snaps.sort_by_key(|s| (s.created_at_ms, s.id));

        let mut by_kind: HashMap<SnapshotKind, (u64, u64)> = HashMap::new();
        let mut files: HashMap<String, (u64, u64)> = HashMap::new();
        let mut seen: HashSet<Blake3Hash> = HashSet::new();
        let mut cumulative_unique_bytes = 0u64;
        let mut snapshot_count = 0u64;
        let mut growth: Vec<GrowthPoint> = Vec::new();

        for s in &snaps {
            // Per-kind breakdown.
            let effective: u64 = s.files.iter().map(|f| f.manifest.size).sum();
            let k = by_kind.entry(s.kind).or_insert((0, 0));
            k.0 += 1;
            k.1 += effective;

            // Cumulative unique chunk bytes (growth curve numerator).
            for f in &s.files {
                for c in &f.manifest.chunks {
                    if seen.insert(c.hash) {
                        cumulative_unique_bytes += u64::from(c.len);
                    }
                }
                // Top-files accounting.
                let entry = files.entry(f.path.clone()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 = entry.1.max(f.manifest.size);
            }
            snapshot_count += 1;

            // One growth point per UTC day; update the day's point in place
            // as later snapshots that day add unique bytes.
            let day_floor = s.created_at_ms.div_euclid(MS_PER_DAY) * MS_PER_DAY;
            match growth.last_mut() {
                Some(last) if last.ts_ms == day_floor => {
                    last.cumulative_unique_bytes = cumulative_unique_bytes;
                    last.snapshot_count = snapshot_count;
                }
                _ => growth.push(GrowthPoint {
                    ts_ms: day_floor,
                    cumulative_unique_bytes,
                    snapshot_count,
                }),
            }
        }

        let mut by_kind: Vec<KindBreakdown> = by_kind
            .into_iter()
            .map(|(kind, (count, effective_bytes))| KindBreakdown {
                kind,
                count,
                effective_bytes,
            })
            .collect();
        by_kind.sort_by_key(|k| k.kind.as_str());

        let mut top_files: Vec<TopFile> = files
            .into_iter()
            .map(|(path, (versions, max_size))| TopFile {
                path,
                versions,
                max_size,
            })
            .collect();
        top_files.sort_by(|a, b| {
            b.versions
                .cmp(&a.versions)
                .then(b.max_size.cmp(&a.max_size))
                .then(a.path.cmp(&b.path))
        });
        top_files.truncate(top_n);

        Ok(RepoReport {
            dedup_ratio: stats.saved_ratio(),
            stats,
            by_kind,
            growth,
            top_files,
        })
    }

    /// Render [`Self::report`] as a self-contained markdown document — no
    /// new dependency, just `String` formatting. Suitable for export.
    pub fn report_markdown(&self, top_n: usize) -> Result<String> {
        let r = self.report(top_n)?;
        let mut s = String::new();
        s.push_str("# Repository report\n\n");
        s.push_str(&format!(
            "- Snapshots: {}\n- Distinct chunks: {}\n- Stored on disk: {} bytes\n- Dedup saved: {:.1}%\n\n",
            r.stats.snapshot_count,
            r.stats.chunk_count,
            r.stats.stored_bytes,
            r.dedup_ratio * 100.0,
        ));
        s.push_str("## By kind\n\n");
        for k in &r.by_kind {
            s.push_str(&format!(
                "- {}: {} snapshots · {} effective bytes\n",
                k.kind.as_str(),
                k.count,
                k.effective_bytes,
            ));
        }
        s.push_str("\n## Top files\n\n");
        for f in &r.top_files {
            s.push_str(&format!(
                "- {} — {} versions · {} bytes max\n",
                f.path, f.versions, f.max_size,
            ));
        }
        Ok(s)
    }
}
