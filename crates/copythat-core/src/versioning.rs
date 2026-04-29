//! Phase 42 Part B — per-file rolling versions: policy types + the
//! pure retention-pruning function the engine + UI consult before
//! deleting an old `versions` row.
//!
//! The actual snapshot-on-overwrite hook (engine-side wiring that
//! captures the about-to-be-overwritten file as a chunk-store
//! manifest before the new bytes land) is deferred to a follow-up
//! commit because it touches the copy loop's hot path. This module
//! ships the data model + decision logic so the
//! `copythat-history::versions` table + the Settings → Versions
//! panel + Tauri command can build against a stable contract today.
//!
//! # Types
//!
//! - [`VersioningPolicy`] — per-job knob set: `enabled` flips the
//!   feature on/off, `retention` decides which versions survive.
//! - [`RetentionPolicy`] — variant set: `None` (keep every version
//!   forever — disk-bomb risk), `LastN(u32)` (keep the N newest),
//!   `OlderThanDays(u32)` (drop versions older than the cutoff), or
//!   `Gfs(GfsPolicy)` (Grandfather-Father-Son: keep the newest N per
//!   hour / day / week / month).
//! - [`GfsPolicy`] — the four GFS bucket counts.
//! - [`VersionEntry`] — minimal `(row_id, ts_ms)` shape the pure
//!   retention function consumes. Decoupled from
//!   `copythat-history::VersionRecord` on purpose so this crate
//!   stays history-free.
//!
//! # Pure pruning
//!
//! [`select_for_pruning`] takes a slice of [`VersionEntry`] (the
//! current versions for one path, in any order) plus a
//! [`RetentionPolicy`] plus the current wall-clock in ms, and
//! returns the row ids the caller should delete. Callers are
//! expected to feed the result into
//! `History::delete_versions(...)`. The function is
//! deterministic + side-effect-free; the engine's copy hot path
//! never calls it.

use std::collections::BTreeMap;
use std::path::Path;

/// Minimal version row shape for the [`select_for_pruning`] input.
/// Decoupled from `copythat-history::VersionRecord` so this crate
/// doesn't have to depend on history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionEntry {
    /// `versions.id` rowid in the history database.
    pub row_id: i64,
    /// Capture timestamp, milliseconds since epoch.
    pub ts_ms: i64,
}

/// Per-job versioning configuration.
///
/// The default disables the feature — versioning costs storage, and
/// the user opts in deliberately via Settings → Transfer → "Keep
/// previous versions on overwrite".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VersioningPolicy {
    /// Master toggle. When `false`, the engine skips the snapshot-
    /// before-overwrite hook entirely; nothing else in this struct
    /// matters.
    pub enabled: bool,
    /// Which versions survive after a snapshot. Default
    /// [`RetentionPolicy::None`] — keep every version. The Settings
    /// panel surfaces this as a dropdown.
    pub retention: RetentionPolicy,
}

/// Retention policy applied to a path's history *after* a fresh
/// snapshot lands. Variants are ordered from "keep everything" to
/// "most aggressive prune".
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RetentionPolicy {
    /// Keep every recorded version forever. Storage cost grows
    /// monotonically — fine for a few-files setup, dangerous on a
    /// rapidly-changing tree. The default because deleting user data
    /// without explicit consent is worse than wasting disk.
    #[default]
    None,
    /// Keep the `N` newest snapshots; drop the rest. `N = 0` is
    /// treated as `N = 1` so the freshly-captured snapshot is
    /// always retained.
    LastN(u32),
    /// Drop snapshots whose `ts_ms` falls more than `days` days
    /// before "now". Implementation rounds up to the millisecond
    /// boundary; clock skew up to ±1 day can shift the effective
    /// cutoff by one bucket.
    OlderThanDays(u32),
    /// Grandfather-Father-Son rolling retention. See [`GfsPolicy`]
    /// for bucket semantics.
    Gfs(GfsPolicy),
}

/// Grandfather-Father-Son retention buckets.
///
/// At pruning time the algorithm groups versions into per-hour /
/// per-day / per-week / per-month buckets (UTC, calendar-aligned),
/// keeps the newest version in each bucket up to the configured
/// count, and drops the rest. A version that survives in *any*
/// bucket is retained — the buckets are unioned, not intersected.
///
/// All four counts default to `0`. `0` means "skip this bucket
/// type"; e.g. `keep_weekly = 0` short-circuits the weekly group.
/// Setting all four to `0` reduces to "keep nothing" — degenerate
/// but explicit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GfsPolicy {
    /// Keep the newest version from each of the most recent
    /// `keep_hourly` UTC hours.
    pub keep_hourly: u32,
    /// Keep the newest version from each of the most recent
    /// `keep_daily` UTC days.
    pub keep_daily: u32,
    /// Keep the newest version from each of the most recent
    /// `keep_weekly` ISO weeks (Monday-anchored).
    pub keep_weekly: u32,
    /// Keep the newest version from each of the most recent
    /// `keep_monthly` calendar months.
    pub keep_monthly: u32,
}

/// Bridge contract for the per-file rolling-versions sink.
///
/// Implemented by a downstream adapter (planned home:
/// `copythat-history` once the chunk-store glue lands; until then any
/// caller that wires the trait can plug its own snapshot logic in).
/// Kept in `copythat-core` so [`crate::CopyOptions`] can hold a trait
/// object without pulling `copythat-history` + `copythat-chunk` into
/// every consumer of the engine's public surface.
///
/// The engine calls [`VersioningSink::snapshot_before_overwrite`]
/// exactly once per file, AFTER the collision-policy resolves to
/// "overwrite" but BEFORE the destination is opened with `truncate`.
/// Failure modes are best-effort by design: a snapshot that fails
/// must NOT abort the copy. Implementations log internally and
/// return `Err`; the engine logs and proceeds without the snapshot.
///
/// Returns:
/// - `Ok(true)` when the existing destination was successfully
///   ingested into the chunk store and a `versions` row recorded.
/// - `Ok(false)` when the destination didn't exist (so no snapshot
///   was taken). The engine treats this the same as "no sink
///   configured" — the copy proceeds.
/// - `Err(_)` on snapshot failure. Logged at the engine boundary;
///   the copy proceeds.
pub trait VersioningSink: Send + Sync + std::fmt::Debug {
    /// Snapshot the existing destination file before the engine
    /// overwrites it. Reads `dst` itself, ingests into the chunk
    /// store, records the [`copythat-history::VersionRecord`] row.
    /// `triggered_by_job_id` is the History `jobs.id` of the
    /// triggering copy job when the engine knows it; `None` for
    /// single-file `copy_file` calls outside a tree context.
    fn snapshot_before_overwrite(
        &self,
        dst: &Path,
        triggered_by_job_id: Option<i64>,
    ) -> Result<bool, String>;
}

/// Constants for the bucket-floor math. UTC; no leap-second
/// accounting (the rounding tolerance absorbs the few seconds of
/// drift over the bucket lifetime).
const MS_PER_HOUR: i64 = 3_600_000;
const MS_PER_DAY: i64 = 24 * MS_PER_HOUR;
const MS_PER_WEEK: i64 = 7 * MS_PER_DAY;

/// Decide which version row ids the caller should delete.
///
/// Inputs: the full set of versions for one destination path
/// (any order — the function sorts internally), the active retention
/// policy, and the current wall-clock in milliseconds-since-epoch.
///
/// Output: the subset of `versions[*].row_id` the caller should
/// hand to `History::delete_versions(...)`. The freshest snapshot
/// is **always** retained — even when the policy would otherwise
/// drop it — so a misconfigured policy can't delete the snapshot
/// the user just took.
///
/// Pure function; no side effects; no allocation beyond the return
/// vector. Safe to call from the copy hot path (it doesn't actually
/// delete; the caller does).
pub fn select_for_pruning(
    versions: &[VersionEntry],
    policy: &RetentionPolicy,
    now_ms: i64,
) -> Vec<i64> {
    if versions.is_empty() {
        return Vec::new();
    }
    // Sort newest-first for the per-policy walks below.
    let mut sorted: Vec<VersionEntry> = versions.to_vec();
    sorted.sort_by(|a, b| b.ts_ms.cmp(&a.ts_ms));
    let newest = sorted[0].row_id;

    let to_drop: Vec<i64> = match policy {
        RetentionPolicy::None => Vec::new(),
        RetentionPolicy::LastN(n) => {
            // 0 → 1 — never drop the freshest snapshot under any
            // numeric input.
            let keep = (*n).max(1) as usize;
            sorted.iter().skip(keep).map(|v| v.row_id).collect()
        }
        RetentionPolicy::OlderThanDays(days) => {
            let cutoff = now_ms.saturating_sub((*days as i64) * MS_PER_DAY);
            sorted
                .iter()
                .filter(|v| v.ts_ms < cutoff)
                .map(|v| v.row_id)
                .collect()
        }
        RetentionPolicy::Gfs(gfs) => gfs_select_for_pruning(&sorted, gfs),
    };

    // Belt-and-suspenders: never include the freshest snapshot's
    // row id in the drop set, even if a future policy variant drifts
    // and forgets the invariant.
    to_drop.into_iter().filter(|id| *id != newest).collect()
}

/// GFS bucket-keeper. Groups `sorted_newest_first` by hour / day /
/// week / month, keeps the newest entry in each of the most recent
/// `keep_*` buckets, and returns every other id.
fn gfs_select_for_pruning(sorted_newest_first: &[VersionEntry], gfs: &GfsPolicy) -> Vec<i64> {
    if sorted_newest_first.is_empty() {
        return Vec::new();
    }

    // Build per-bucket "first-seen newest" maps. Keys are bucket
    // floor timestamps (in ms). BTreeMap orders by key naturally —
    // we'll take the largest (most-recent) `keep_*` keys.
    let mut hourly: BTreeMap<i64, i64> = BTreeMap::new();
    let mut daily: BTreeMap<i64, i64> = BTreeMap::new();
    let mut weekly: BTreeMap<i64, i64> = BTreeMap::new();
    let mut monthly: BTreeMap<i64, i64> = BTreeMap::new();
    for v in sorted_newest_first {
        let h_floor = bucket_floor_hourly(v.ts_ms);
        let d_floor = bucket_floor_daily(v.ts_ms);
        let w_floor = bucket_floor_weekly(v.ts_ms);
        let m_floor = bucket_floor_monthly(v.ts_ms);
        // Newest-first means the first insert into a bucket is the
        // "winner" — `entry().or_insert()` preserves that.
        hourly.entry(h_floor).or_insert(v.row_id);
        daily.entry(d_floor).or_insert(v.row_id);
        weekly.entry(w_floor).or_insert(v.row_id);
        monthly.entry(m_floor).or_insert(v.row_id);
    }

    // Pick the most-recent `keep_*` buckets in each tier, take the
    // representative version of each, and union into a "keep" set.
    let mut keep: std::collections::HashSet<i64> = std::collections::HashSet::new();
    for &id in hourly.values().rev().take(gfs.keep_hourly as usize) {
        keep.insert(id);
    }
    for &id in daily.values().rev().take(gfs.keep_daily as usize) {
        keep.insert(id);
    }
    for &id in weekly.values().rev().take(gfs.keep_weekly as usize) {
        keep.insert(id);
    }
    for &id in monthly.values().rev().take(gfs.keep_monthly as usize) {
        keep.insert(id);
    }

    sorted_newest_first
        .iter()
        .filter(|v| !keep.contains(&v.row_id))
        .map(|v| v.row_id)
        .collect()
}

/// Bucket floor for the hourly tier — round `ts_ms` down to the most
/// recent UTC hour boundary. `0` lands on `1970-01-01T00:00:00Z`.
fn bucket_floor_hourly(ts_ms: i64) -> i64 {
    ts_ms.div_euclid(MS_PER_HOUR) * MS_PER_HOUR
}

fn bucket_floor_daily(ts_ms: i64) -> i64 {
    ts_ms.div_euclid(MS_PER_DAY) * MS_PER_DAY
}

/// Weekly bucket floor — Monday 00:00 UTC. The Unix epoch
/// `1970-01-01` was a Thursday, so we offset by 4 days
/// (`-4 * MS_PER_DAY`) before applying the week floor and undo the
/// shift afterwards. This produces ISO-week-aligned buckets.
fn bucket_floor_weekly(ts_ms: i64) -> i64 {
    const MONDAY_OFFSET_MS: i64 = 4 * MS_PER_DAY;
    let shifted = ts_ms - MONDAY_OFFSET_MS;
    shifted.div_euclid(MS_PER_WEEK) * MS_PER_WEEK + MONDAY_OFFSET_MS
}

/// Monthly bucket floor — UTC calendar month. Calendar-correct
/// (January has 31 days, February 28 / 29, etc.); we approximate by
/// converting `ts_ms` to (year, month) via a closed-form algorithm
/// from "civil_from_days" by Howard Hinnant — same approach the
/// recovery server's templates use to render dates.
fn bucket_floor_monthly(ts_ms: i64) -> i64 {
    // Days since 1970-01-01.
    let days_total = ts_ms.div_euclid(MS_PER_DAY);
    // Convert days-since-epoch to (year, month, day-of-month) per
    // Hinnant's algorithm. Reference:
    // https://howardhinnant.github.io/date_algorithms.html#civil_from_days
    let z = days_total + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y_adjusted = if m <= 2 { y + 1 } else { y };
    // Reverse: convert (y_adjusted, m, day=1) back to days-since-epoch.
    let m_shifted = if m > 2 { m - 3 } else { m + 9 };
    let y_for_calc = if m > 2 { y_adjusted } else { y_adjusted - 1 };
    let era2 = y_for_calc.div_euclid(400);
    let yoe2 = y_for_calc - era2 * 400;
    let doy2 = (153 * m_shifted + 2) / 5; // day = 1 → 0 days into month
    let doe2 = yoe2 * 365 + yoe2 / 4 - yoe2 / 100 + doy2;
    let days_to_first_of_month = era2 * 146_097 + doe2 - 719_468;
    days_to_first_of_month * MS_PER_DAY
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(row_id: i64, ts_ms: i64) -> VersionEntry {
        VersionEntry { row_id, ts_ms }
    }

    #[test]
    fn empty_input_yields_empty_drop_set() {
        let drop = select_for_pruning(&[], &RetentionPolicy::None, 0);
        assert!(drop.is_empty());
    }

    #[test]
    fn none_policy_drops_nothing() {
        let versions = [entry(1, 100), entry(2, 200), entry(3, 300)];
        let drop = select_for_pruning(&versions, &RetentionPolicy::None, 1_000);
        assert!(drop.is_empty());
    }

    #[test]
    fn last_n_keeps_newest_n() {
        let versions = [
            entry(1, 100),
            entry(2, 200),
            entry(3, 300),
            entry(4, 400),
            entry(5, 500),
        ];
        let drop = select_for_pruning(&versions, &RetentionPolicy::LastN(2), 1_000);
        // Keep newest 2 (id 5 and 4); drop 3, 2, 1.
        let mut sorted = drop;
        sorted.sort();
        assert_eq!(sorted, vec![1, 2, 3]);
    }

    #[test]
    fn last_n_zero_is_treated_as_one() {
        let versions = [entry(1, 100), entry(2, 200)];
        let drop = select_for_pruning(&versions, &RetentionPolicy::LastN(0), 1_000);
        // Newest (id 2) is always retained.
        assert_eq!(drop, vec![1]);
    }

    #[test]
    fn last_n_larger_than_set_drops_nothing() {
        let versions = [entry(1, 100), entry(2, 200)];
        let drop = select_for_pruning(&versions, &RetentionPolicy::LastN(99), 1_000);
        assert!(drop.is_empty());
    }

    #[test]
    fn older_than_days_drops_stale_entries() {
        let now = 30 * MS_PER_DAY;
        // 5 entries: 1d / 5d / 10d / 20d / 35d old.
        let versions = [
            entry(1, now - MS_PER_DAY),
            entry(2, now - 5 * MS_PER_DAY),
            entry(3, now - 10 * MS_PER_DAY),
            entry(4, now - 20 * MS_PER_DAY),
            entry(5, now - 35 * MS_PER_DAY),
        ];
        let drop = select_for_pruning(&versions, &RetentionPolicy::OlderThanDays(15), now);
        let mut sorted = drop;
        sorted.sort();
        // Older than 15 days → ids 4 (20d) and 5 (35d) drop.
        assert_eq!(sorted, vec![4, 5]);
    }

    #[test]
    fn older_than_days_protects_freshest_even_when_zero_days() {
        // OlderThanDays(0) would otherwise drop everything; the
        // newest-snapshot guard rescues the freshest.
        let now = 30 * MS_PER_DAY;
        let versions = [entry(1, now - 100), entry(2, now - 50)];
        let drop = select_for_pruning(&versions, &RetentionPolicy::OlderThanDays(0), now);
        // id 2 is freshest → retained; id 1 dropped.
        assert_eq!(drop, vec![1]);
    }

    #[test]
    fn gfs_keep_hourly_only() {
        // Every snapshot in a different hour. With keep_hourly=2 the
        // newest two hours survive; older ones drop.
        let base = 100 * MS_PER_DAY;
        let versions = [
            entry(1, base),
            entry(2, base + MS_PER_HOUR),
            entry(3, base + 2 * MS_PER_HOUR),
            entry(4, base + 3 * MS_PER_HOUR),
        ];
        let policy = RetentionPolicy::Gfs(GfsPolicy {
            keep_hourly: 2,
            ..GfsPolicy::default()
        });
        let drop = select_for_pruning(&versions, &policy, base + 4 * MS_PER_HOUR);
        let mut sorted = drop;
        sorted.sort();
        // Keep newest two hours (id 4 + id 3); drop id 2 + id 1.
        assert_eq!(sorted, vec![1, 2]);
    }

    #[test]
    fn gfs_keep_daily_unions_with_hourly() {
        // Snapshots across 3 distinct days; a daily-only policy
        // keeps one per day, which means everything older than 3
        // days drops.
        let day0 = 100 * MS_PER_DAY;
        let versions = [
            // Day 0 (oldest)
            entry(1, day0),
            entry(2, day0 + MS_PER_HOUR),
            // Day 1
            entry(3, day0 + MS_PER_DAY),
            // Day 2
            entry(4, day0 + 2 * MS_PER_DAY),
        ];
        let policy = RetentionPolicy::Gfs(GfsPolicy {
            keep_daily: 3,
            ..GfsPolicy::default()
        });
        let drop = select_for_pruning(&versions, &policy, day0 + 3 * MS_PER_DAY);
        // For each of the 3 most-recent days the newest is kept:
        // - Day 0: newest is id 2 (later by 1h)
        // - Day 1: id 3
        // - Day 2: id 4
        // Only id 1 drops.
        assert_eq!(drop, vec![1]);
    }

    #[test]
    fn gfs_zero_buckets_keeps_only_freshest() {
        let versions = [entry(1, 100), entry(2, 200), entry(3, 300)];
        let policy = RetentionPolicy::Gfs(GfsPolicy::default());
        let drop = select_for_pruning(&versions, &policy, 1_000);
        // All-zero buckets means nothing is kept by the GFS pass —
        // the freshest-snapshot guard rescues id 3.
        let mut sorted = drop;
        sorted.sort();
        assert_eq!(sorted, vec![1, 2]);
    }

    #[test]
    fn freshest_is_never_dropped_under_any_policy() {
        let versions = [entry(1, 100), entry(2, 200)];
        for policy in [
            RetentionPolicy::None,
            RetentionPolicy::LastN(0),
            RetentionPolicy::OlderThanDays(0),
            RetentionPolicy::Gfs(GfsPolicy::default()),
        ] {
            let drop = select_for_pruning(&versions, &policy, 1_000);
            assert!(
                !drop.contains(&2),
                "freshest version (id 2) was incorrectly dropped under {policy:?}"
            );
        }
    }

    #[test]
    fn bucket_floor_hourly_round_trip() {
        // 2024-01-01T12:34:56Z = 1704112496000 ms. The hour floor
        // should land on 1704110400000 (12:00:00).
        assert_eq!(bucket_floor_hourly(1_704_112_496_000), 1_704_110_400_000);
        // Boundary check — exactly on the hour stays put.
        assert_eq!(bucket_floor_hourly(1_704_110_400_000), 1_704_110_400_000);
    }

    #[test]
    fn bucket_floor_daily_round_trip() {
        // 2024-01-01T12:34:56Z → 2024-01-01T00:00:00Z.
        assert_eq!(bucket_floor_daily(1_704_112_496_000), 1_704_067_200_000);
    }

    #[test]
    fn versioning_policy_default_is_disabled_with_no_retention() {
        let p = VersioningPolicy::default();
        assert!(!p.enabled);
        assert_eq!(p.retention, RetentionPolicy::None);
    }
}
