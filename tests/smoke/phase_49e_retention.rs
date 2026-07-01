//! Phase 49e smoke — retention & GFS prune over `Repository::prune_source`.
//!
//! Records several `Backup` snapshots tagged with a source key, applies a
//! retention policy, and asserts: the policy-selected losers are removed,
//! the freshest is always kept, the follow-up `gc()` sweeps a chunk unique
//! to a dropped snapshot while a surviving snapshot still restores
//! byte-for-byte, and snapshots of other sources (or no source) are
//! untouched. The retention math itself is the audited
//! `copythat_core::versioning::select_for_pruning` (unit-tested there); this
//! verifies the `Repository` plumbing + the gc sweep + source isolation.

use std::collections::HashSet;

use copythat_chunk::{Chunker, GfsPolicy, Repository, RetentionPolicy, SnapshotKind};

/// Deterministic pseudo-random bytes (same LCG as the repository unit
/// tests) so the fixture is reproducible without a `rand` dependency.
fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    let mut s = seed;
    for b in &mut out {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *b = (s >> 33) as u8;
    }
    out
}

const MIB: usize = 1024 * 1024;

/// File contents for snapshot `i`: a shared 4 MiB base (identical across
/// all snapshots → its interior chunks dedup) followed by a 2 MiB block
/// unique to `i`, guaranteeing at least one chunk only this snapshot
/// references.
fn file_for(base: &[u8], i: u64) -> Vec<u8> {
    let mut v = base.to_vec();
    v.extend_from_slice(&lcg_bytes(1000 + i, 2 * MIB));
    v
}

#[test]
fn last_n_prunes_oldest_sweeps_unique_keeps_survivors() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let base = lcg_bytes(1, 4 * MIB);

    // Six backup snapshots of one source, oldest → newest.
    for i in 0..6u64 {
        let f = file_for(&base, i);
        repo.snapshot_bytes_with_source(
            SnapshotKind::Backup,
            "Documents",
            "src-A",
            1000 + (i as i64) * 1000,
            &[("/A/file", &f)],
        )
        .unwrap();
    }
    assert_eq!(repo.snapshots().unwrap().len(), 6);

    // A chunk unique to the OLDEST snapshot (from its unique block, not the
    // shared base) — it must be swept once that snapshot is pruned.
    let chunker = Chunker::default();
    let base_hashes: HashSet<_> = chunker.chunk_bytes(&base).iter().map(|c| c.hash).collect();
    let witness = chunker
        .chunk_bytes(&file_for(&base, 0))
        .into_iter()
        .map(|c| c.hash)
        .find(|h| !base_hashes.contains(h))
        .expect("oldest snapshot must have a chunk unique to it");
    assert!(repo.store().has(&witness).unwrap());

    // Keep the newest 2 → drop the 4 oldest.
    let report = repo
        .prune_source("src-A", &RetentionPolicy::LastN(2), 100_000)
        .unwrap();
    assert_eq!(report.snapshots_removed, 4);
    assert!(
        report.chunks_swept >= 1,
        "expected the dropped unique chunk to sweep"
    );

    // The catalog now holds the 2 newest snapshots only.
    let left = repo.snapshots().unwrap();
    assert_eq!(left.len(), 2);
    assert!(
        left.iter()
            .all(|s| s.source_key.as_deref() == Some("src-A"))
    );
    assert!(left.iter().all(|s| s.created_at_ms >= 5000));

    // The pruned snapshot's unique chunk is gone; the newest snapshot still
    // restores byte-for-byte (its chunks survive).
    assert!(
        !repo.store().has(&witness).unwrap(),
        "orphan chunk should be swept"
    );
    let newest = repo.snapshot_at("/A/file", 100_000).unwrap().unwrap();
    let dst = tmp.path().join("restored.bin");
    repo.restore(&newest, &dst).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), file_for(&base, 5));
}

#[test]
fn gfs_keeps_freshest_and_prune_is_source_scoped() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let base = lcg_bytes(2, 4 * MIB);

    const DAY_MS: i64 = 24 * 3_600_000;
    let day0: i64 = 1_000_000_000_000;

    // Source A: one snapshot per day across 4 distinct UTC days.
    for d in 0..4u64 {
        let f = file_for(&base, d);
        repo.snapshot_bytes_with_source(
            SnapshotKind::Backup,
            "Photos",
            "src-A",
            day0 + (d as i64) * DAY_MS,
            &[("/A/p", &f)],
        )
        .unwrap();
    }
    // Source B: one snapshot under a different source key.
    let fb = file_for(&base, 99);
    let b_id = repo
        .snapshot_bytes_with_source(
            SnapshotKind::Backup,
            "Music",
            "src-B",
            day0,
            &[("/B/m", &fb)],
        )
        .unwrap();
    // An ad-hoc snapshot with no source key at all.
    let fc = file_for(&base, 77);
    repo.snapshot_bytes(SnapshotKind::Copy, "adhoc", day0, &[("/C/c", &fc)])
        .unwrap();
    assert_eq!(repo.snapshots().unwrap().len(), 6);

    // GFS daily=2 over source A's four daily snapshots → keep the two
    // most-recent days (one snapshot each); the freshest is always kept.
    let now = day0 + 4 * DAY_MS;
    let report = repo
        .prune_source(
            "src-A",
            &RetentionPolicy::Gfs(GfsPolicy {
                keep_daily: 2,
                ..Default::default()
            }),
            now,
        )
        .unwrap();
    assert_eq!(report.snapshots_removed, 2);

    let left = repo.snapshots().unwrap();
    // 2 source-A survivors + source-B + the ad-hoc = 4.
    assert_eq!(left.len(), 4);
    // The freshest source-A snapshot survived.
    assert!(
        left.iter()
            .any(|s| s.source_key.as_deref() == Some("src-A")
                && s.created_at_ms == day0 + 3 * DAY_MS)
    );
    // Source B + the no-source snapshot are untouched.
    assert!(
        left.iter()
            .any(|s| s.source_key.as_deref() == Some("src-B"))
    );
    assert!(left.iter().any(|s| s.source_key.is_none()));
    // Source B's snapshot still resolves + restores (isolation held).
    let fs = repo.snapshot_at("/B/m", now).unwrap().unwrap();
    assert_eq!(fs.snapshot_id, b_id.as_u64());
}

#[test]
fn keep_all_and_empty_source_are_no_ops() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let f = lcg_bytes(3, 2 * MIB);
    repo.snapshot_bytes_with_source(SnapshotKind::Backup, "Docs", "src-A", 1000, &[("/A/f", &f)])
        .unwrap();

    // `None` retention keeps everything and skips gc → zeroed report.
    let report = repo
        .prune_source("src-A", &RetentionPolicy::None, 9999)
        .unwrap();
    assert_eq!(report.snapshots_removed, 0);
    assert_eq!(report.chunks_swept, 0);

    // An unknown / empty source prunes nothing.
    let report = repo
        .prune_source("src-does-not-exist", &RetentionPolicy::LastN(1), 9999)
        .unwrap();
    assert_eq!(report.snapshots_removed, 0);
    assert_eq!(repo.snapshots().unwrap().len(), 1);
}
