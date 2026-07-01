//! Phase 49i smoke — pack compaction.
//!
//! Forces exactly two 48 KiB files per pack (via `open_with_rollover`),
//! removes snapshots so one pack goes fully dead and one goes half dead,
//! then asserts: quick `gc` drops the fully-dead pack, `compact` rewrites
//! the half-dead pack to reclaim its dead bytes, and every surviving file
//! — including the one physically moved by compaction — still restores
//! byte-for-byte.

use std::sync::Arc;

use copythat_chunk::{ChunkStore, CompactOptions, MaintenanceProgress, Repository, SnapshotKind};

const F: usize = 48 * 1024;

fn data(seed: u8) -> Vec<u8> {
    // Distinct, incompressible-ish content per seed so chunks never dedup.
    let mut v = vec![0u8; F];
    let mut s = u64::from(seed) + 1;
    for b in &mut v {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        *b = (s >> 33) as u8;
    }
    v
}

#[test]
fn compact_reclaims_partial_packs_and_survivors_restore() {
    let tmp = tempfile::tempdir().unwrap();
    // 64 KiB rollover → exactly two 48 KiB files (one chunk each, since
    // 48 KiB < the 512 KiB chunker minimum) per pack.
    let store = Arc::new(ChunkStore::open_with_rollover(tmp.path(), 64 * 1024).unwrap());
    let repo = Repository::with_store(store).unwrap();

    let mut ids = Vec::new();
    let mut blobs = Vec::new();
    for i in 0..6u8 {
        let d = data(i);
        let path = format!("/f{i}");
        let id = repo
            .snapshot_bytes(
                SnapshotKind::Backup,
                &format!("s{i}"),
                1000 + i64::from(i),
                &[(path.as_str(), &d)],
            )
            .unwrap();
        ids.push(id);
        blobs.push((path, d));
    }

    // Remove f0 (→ half-dead pack0), f2 + f3 (→ fully-dead pack1).
    // Survivors: f1 (in pack0), f4 + f5 (in pack2).
    repo.remove_snapshots(&[ids[0], ids[2], ids[3]]).unwrap();

    // Quick gc drops the fully-dead pack, but leaves the half-dead one.
    let gc = repo.gc().unwrap();
    assert!(gc.packs_removed >= 1, "gc should drop the fully-dead pack");

    // Full compact rewrites the half-dead pack, reclaiming its dead bytes.
    let before = repo.stats().unwrap().stored_bytes;
    let never = || false;
    let mut noop = |_: MaintenanceProgress| {};
    let (_gc2, cr) = repo
        .compact(CompactOptions::default(), &never, &mut noop)
        .unwrap();
    assert!(cr.packs_compacted >= 1, "the half-dead pack should compact");
    assert!(cr.chunks_moved >= 1);
    assert!(cr.bytes_reclaimed > 0);
    assert!(
        repo.stats().unwrap().stored_bytes < before,
        "compaction should shrink on-disk size"
    );

    // Every surviving file still restores byte-for-byte — including f1,
    // which compaction physically moved into the active pack.
    for (path, want) in [&blobs[1], &blobs[4], &blobs[5]] {
        let fs = repo.snapshot_at(path, 9_999).unwrap().unwrap();
        let dst = tmp.path().join("out.bin");
        repo.restore(&fs, &dst).unwrap();
        assert_eq!(&std::fs::read(&dst).unwrap(), want);
    }
}
