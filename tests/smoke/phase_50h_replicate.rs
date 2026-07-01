//! Phase 50h smoke — `Repository::replicate_to`: a dedup-aware push of one
//! repo's snapshots into another. Proves (1) only missing chunks transfer,
//! (2) re-running is idempotent, (3) an incremental snapshot ships only its
//! novel chunks, and (4) the destination restores byte-identically even when
//! it runs a *different* compression policy than the source (the copied
//! manifest's per-chunk codec is re-stamped to match the destination).

use copythat_chunk::{CompressionLevel, RepoCompression, Repository, SnapshotKind, VerifyLevel};

#[test]
fn replicate_to_dedups_is_idempotent_and_restores() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();

    // Source (default = uncompressed): two snapshots sharing a file so the
    // second snapshot's chunks fully dedup against the first.
    let src = Repository::open(src_dir.path()).unwrap();
    let a = vec![7u8; 300_000];
    let b = vec![9u8; 300_000];
    src.snapshot_bytes(
        SnapshotKind::Backup,
        "s1",
        1_000,
        &[("/data/a.bin", a.as_slice()), ("/data/b.bin", b.as_slice())],
    )
    .unwrap();
    src.snapshot_bytes(
        SnapshotKind::Backup,
        "s2",
        2_000,
        &[("/data/a.bin", a.as_slice())],
    )
    .unwrap();

    // Destination runs a DIFFERENT compression policy than the source — the
    // codec re-stamp must keep the replicated manifest self-consistent.
    let dst = Repository::open_with_compression(
        dst_dir.path(),
        RepoCompression::Always {
            level: CompressionLevel::default(),
        },
    )
    .unwrap();

    // First push: both snapshots + their (deduped) chunks land.
    let r1 = src.replicate_to(&dst).unwrap();
    assert_eq!(r1.snapshots_copied, 2, "both snapshots pushed");
    assert_eq!(r1.snapshots_skipped, 0);
    assert!(r1.chunks_copied > 0, "chunks transferred");
    assert_eq!(dst.snapshots().unwrap().len(), 2);

    // Destination reconstructs byte-identically (proves the cross-policy
    // chunks decode) and passes a deep verify.
    assert!(dst.verify(None, VerifyLevel::ReadData).unwrap().is_clean());
    let fs = dst
        .snapshot_at("/data/b.bin", 2_000)
        .unwrap()
        .expect("b.bin present in destination");
    let dst_file = out.path().join("b.bin");
    dst.restore(&fs, &dst_file).unwrap();
    assert_eq!(
        std::fs::read(&dst_file).unwrap(),
        b,
        "byte-identical restore"
    );

    // Re-running ships nothing new (content fingerprints already present).
    let r2 = src.replicate_to(&dst).unwrap();
    assert_eq!(r2.snapshots_copied, 0);
    assert_eq!(r2.snapshots_skipped, 2);
    assert_eq!(r2.chunks_copied, 0);
    assert_eq!(dst.snapshots().unwrap().len(), 2, "no duplicate snapshots");

    // Incremental: a new source snapshot pushes only its novel chunks.
    let c = vec![5u8; 300_000];
    src.snapshot_bytes(
        SnapshotKind::Backup,
        "s3",
        3_000,
        &[("/data/c.bin", c.as_slice())],
    )
    .unwrap();
    let r3 = src.replicate_to(&dst).unwrap();
    assert_eq!(r3.snapshots_copied, 1, "only the new snapshot");
    assert_eq!(r3.snapshots_skipped, 2);
    assert!(r3.chunks_copied > 0);
    assert_eq!(dst.snapshots().unwrap().len(), 3);
    assert!(dst.verify(None, VerifyLevel::ReadData).unwrap().is_clean());
}
