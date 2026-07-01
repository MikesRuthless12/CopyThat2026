//! Phase 49o smoke — snapshot diff / compare via manifests.
//!
//! Records a baseline, then a snapshot that modifies one file (a single
//! flipped byte), adds one, and drops one; asserts the diff classifies
//! each correctly and that a modified file's incremental cost is only the
//! changed chunks (most chunks shared). Also checks the self-diff is
//! all-unchanged and a missing snapshot id errors.

use copythat_chunk::{FileChange, Repository, SnapshotId, SnapshotKind};

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

#[test]
fn diff_classifies_added_removed_modified() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let a = lcg_bytes(1, 6 * MIB);
    let b = lcg_bytes(2, MIB);
    let s1 = repo
        .snapshot_bytes(SnapshotKind::Backup, "s1", 1000, &[("/a", &a), ("/b", &b)])
        .unwrap();

    let mut a2 = a.clone();
    a2[0] ^= 0xff; // flip one byte → the first chunk changes, the rest don't
    let c = lcg_bytes(3, MIB);
    let s2 = repo
        .snapshot_bytes(SnapshotKind::Backup, "s2", 2000, &[("/a", &a2), ("/c", &c)])
        .unwrap();

    let diff = repo.diff_snapshots(s1, s2).unwrap();
    assert_eq!(diff.added, 1, "/c");
    assert_eq!(diff.removed, 1, "/b");
    assert_eq!(diff.modified, 1, "/a");
    assert_eq!(diff.unchanged, 0);

    let a_diff = diff.files.iter().find(|f| f.path == "/a").unwrap();
    assert_eq!(a_diff.change, FileChange::Modified);
    assert!(
        a_diff.chunks_shared > 0,
        "most chunks unchanged: {a_diff:?}"
    );
    assert!(a_diff.chunks_changed >= 1);
    assert!(
        a_diff.bytes_added < a.len() as u64,
        "only changed chunks counted, not the whole file"
    );

    let c_diff = diff.files.iter().find(|f| f.path == "/c").unwrap();
    assert_eq!(c_diff.change, FileChange::Added);
    assert_eq!(c_diff.old_size, None);

    let b_diff = diff.files.iter().find(|f| f.path == "/b").unwrap();
    assert_eq!(b_diff.change, FileChange::Removed);
    assert_eq!(b_diff.new_size, None);

    // Repo-level incremental cost: /a's changed chunk(s) + all of /c → > 0,
    // and far less than re-storing both files whole.
    assert!(diff.bytes_added > 0);
    assert!(diff.bytes_added < (a.len() + c.len()) as u64);
}

#[test]
fn diff_of_a_snapshot_with_itself_is_all_unchanged() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let a = lcg_bytes(7, 2 * MIB);
    let s1 = repo
        .snapshot_bytes(SnapshotKind::Backup, "s1", 1000, &[("/a", &a)])
        .unwrap();
    let diff = repo.diff_snapshots(s1, s1).unwrap();
    assert_eq!(diff.unchanged, 1);
    assert_eq!(diff.added + diff.removed + diff.modified, 0);
    assert_eq!(diff.bytes_added, 0);
    assert_eq!(diff.files[0].change, FileChange::Unchanged);
}

#[test]
fn diff_missing_snapshot_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let a = lcg_bytes(9, MIB);
    let s1 = repo
        .snapshot_bytes(SnapshotKind::Backup, "s1", 1000, &[("/a", &a)])
        .unwrap();
    assert!(repo.diff_snapshots(s1, SnapshotId(999)).is_err());
}
