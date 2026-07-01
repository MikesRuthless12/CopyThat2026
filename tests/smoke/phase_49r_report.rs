//! Phase 49r smoke — repository statistics / reports.
//!
//! Records snapshots of varying kinds across several UTC days and asserts
//! the report's per-kind breakdown, the (non-decreasing) growth curve, the
//! top-files ranking + `top_n` cap, the dedup ratio bounds, and that the
//! markdown rendering carries its section headers.

use copythat_chunk::{Repository, SnapshotKind};

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
const DAY: i64 = 86_400_000;

#[test]
fn report_aggregates_kinds_growth_and_top_files() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let a = lcg_bytes(1, 3 * MIB);
    let mut a2 = a.clone();
    a2[0] ^= 0xff;
    let b = lcg_bytes(2, MIB);

    repo.snapshot_bytes(SnapshotKind::Copy, "c", 0, &[("/a", &a)])
        .unwrap();
    repo.snapshot_bytes(SnapshotKind::Sync, "s", DAY, &[("/a", &a)])
        .unwrap(); // same content → dedups
    repo.snapshot_bytes(SnapshotKind::Version, "v", 2 * DAY, &[("/a", &a2)])
        .unwrap(); // modified
    repo.snapshot_bytes(SnapshotKind::Backup, "bk", 2 * DAY + 1000, &[("/b", &b)])
        .unwrap();

    let r = repo.report(10).unwrap();

    // by_kind: four kinds, one snapshot each.
    assert_eq!(r.by_kind.len(), 4);
    assert!(r.by_kind.iter().all(|k| k.count == 1));

    // growth: cumulative non-decreasing; final point counts all snapshots.
    assert!(!r.growth.is_empty());
    let mut prev = 0u64;
    for g in &r.growth {
        assert!(g.cumulative_unique_bytes >= prev, "growth must not shrink");
        prev = g.cumulative_unique_bytes;
    }
    assert_eq!(r.growth.last().unwrap().snapshot_count, 4);

    // top_files: /a appears in 3 snapshots, /b in 1.
    assert_eq!(r.top_files[0].path, "/a");
    assert_eq!(r.top_files[0].versions, 3);
    let b_file = r.top_files.iter().find(|f| f.path == "/b").unwrap();
    assert_eq!(b_file.versions, 1);

    // dedup_ratio in [0,1]; copy+sync share /a content so it's positive.
    assert!((0.0..=1.0).contains(&r.dedup_ratio));
    assert!(r.dedup_ratio > 0.0, "shared content should dedup");

    let md = repo.report_markdown(10).unwrap();
    assert!(md.contains("# Repository report"));
    assert!(md.contains("## By kind"));
    assert!(md.contains("## Top files"));
}

#[test]
fn report_top_n_caps_files() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let paths: Vec<String> = (0..5).map(|i| format!("/f{i}")).collect();
    for (i, p) in paths.iter().enumerate() {
        let d = lcg_bytes(100 + i as u64, 256 * 1024);
        repo.snapshot_bytes(SnapshotKind::Backup, "s", i as i64, &[(p.as_str(), &d)])
            .unwrap();
    }
    let r = repo.report(3).unwrap();
    assert_eq!(r.top_files.len(), 3);
}
