//! Phase 49p smoke — snapshot pinning / labels / description / tags +
//! pin-protected global prune.
//!
//! Asserts a pinned snapshot survives a `keep_last`-style prune even when
//! it isn't the newest, that label/description/tags/pinned edits sync into
//! the summary index immediately and persist across reopen, and that an
//! empty policy + a missing id are inert.

use copythat_chunk::{PrunePolicy, Repository, SnapshotId, SnapshotKind};

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

#[test]
fn pinned_snapshot_survives_prune() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let d = lcg_bytes(1, 1024 * 1024);
    let s1 = repo
        .snapshot_bytes(SnapshotKind::Backup, "old", 1000, &[("/a", &d)])
        .unwrap();
    let s2 = repo
        .snapshot_bytes(SnapshotKind::Backup, "mid", 2000, &[("/b", &d)])
        .unwrap();
    let s3 = repo
        .snapshot_bytes(SnapshotKind::Backup, "new", 3000, &[("/c", &d)])
        .unwrap();

    // Pin the OLDEST.
    assert!(repo.set_pinned(s1, true).unwrap());

    // keep_last=1 would normally keep only s3; the pin rescues s1.
    let removed = repo
        .prune(
            &PrunePolicy {
                keep_last: Some(1),
                keep_within_ms: None,
            },
            10_000,
        )
        .unwrap();
    assert_eq!(removed, vec![s2], "only the unpinned non-newest drops");

    let ids: Vec<u64> = repo.snapshots().unwrap().iter().map(|s| s.id).collect();
    assert!(ids.contains(&s1.as_u64()), "pinned survives");
    assert!(ids.contains(&s3.as_u64()), "newest survives");
    assert!(!ids.contains(&s2.as_u64()));
}

#[test]
fn metadata_edits_persist_and_sync_summary() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().to_path_buf();
    let d = lcg_bytes(2, 512 * 1024);
    let id;
    {
        let repo = Repository::open(&path).unwrap();
        id = repo
            .snapshot_bytes(SnapshotKind::Copy, "orig", 1000, &[("/x", &d)])
            .unwrap();
        assert!(repo.set_label(id, "renamed").unwrap());
        assert!(repo.set_description(id, "quarterly archive").unwrap());
        assert!(repo.set_tags(id, vec!["keep".into(), "q3".into()]).unwrap());
        assert!(repo.set_pinned(id, true).unwrap());

        // The lightweight summary reflects edits immediately.
        let summaries = repo.snapshots().unwrap();
        let s = summaries.iter().find(|s| s.id == id.as_u64()).unwrap();
        assert_eq!(s.label, "renamed");
        assert_eq!(s.description, "quarterly archive");
        assert_eq!(s.tags, vec!["keep".to_string(), "q3".to_string()]);
        assert!(s.pinned);
    }

    // Reopen → the full snapshot row persisted the edits too.
    let repo = Repository::open(&path).unwrap();
    let full = repo.snapshot(id).unwrap().unwrap();
    assert_eq!(full.label, "renamed");
    assert_eq!(full.description, "quarterly archive");
    assert_eq!(full.tags, vec!["keep".to_string(), "q3".to_string()]);
    assert!(full.pinned);
}

#[test]
fn empty_policy_and_missing_id_are_inert() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let d = lcg_bytes(3, 256 * 1024);
    repo.snapshot_bytes(SnapshotKind::Backup, "s", 1000, &[("/a", &d)])
        .unwrap();
    assert!(
        repo.prune(&PrunePolicy::default(), 9999)
            .unwrap()
            .is_empty(),
        "all-None policy prunes nothing"
    );
    assert!(!repo.set_pinned(SnapshotId(999), true).unwrap());
}
