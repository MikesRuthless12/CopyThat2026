//! Phase 49h smoke — repository chunk compression.
//!
//! Asserts that an `Always`/`Auto` compression policy stores chunks
//! smaller on disk yet restores them byte-for-byte (the hash + logical
//! length are always of the plaintext), that incompressible data falls
//! back to raw under `Auto`, and that a dedup hit keeps the FIRST writer's
//! codec (so the manifest records the truth). The on-disk v0→v1 locator
//! back-compat is unit-tested inside `store.rs`.

use copythat_chunk::{ChunkCodec, CompressionLevel, RepoCompression, Repository, SnapshotKind};

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

fn always() -> RepoCompression {
    RepoCompression::Always {
        level: CompressionLevel::default(),
    }
}

#[test]
fn compressed_chunks_restore_byte_identical_and_shrink() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open_with_compression(tmp.path(), always()).unwrap();
    // Highly compressible content.
    let data = vec![42u8; 3 * MIB];
    let id = repo
        .snapshot_bytes(SnapshotKind::Backup, "c", 1000, &[("/a", &data)])
        .unwrap();

    // The manifest records Zstd for every chunk.
    let snap = repo.snapshot(id).unwrap().unwrap();
    assert!(
        snap.files[0]
            .manifest
            .chunks
            .iter()
            .all(|c| c.codec == ChunkCodec::Zstd),
        "all chunks should be zstd"
    );

    // Restore returns plaintext byte-for-byte.
    let fs = repo.snapshot_at("/a", 9999).unwrap().unwrap();
    let dst = tmp.path().join("out.bin");
    repo.restore(&fs, &dst).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), data);

    // And it really shrank on disk.
    let stats = repo.stats().unwrap();
    assert!(
        stats.physical_unique_bytes < stats.unique_bytes,
        "physical {} should be < logical {}",
        stats.physical_unique_bytes,
        stats.unique_bytes
    );
    assert!(stats.compression_ratio() > 0.0);
}

#[test]
fn incompressible_stays_raw_under_auto() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open_with_compression(
        tmp.path(),
        RepoCompression::Auto {
            level: CompressionLevel::default(),
        },
    )
    .unwrap();
    let data = lcg_bytes(7, 2 * MIB); // pseudo-random → zstd can't win
    let id = repo
        .snapshot_bytes(SnapshotKind::Backup, "r", 1000, &[("/a", &data)])
        .unwrap();
    let snap = repo.snapshot(id).unwrap().unwrap();
    assert!(
        snap.files[0]
            .manifest
            .chunks
            .iter()
            .all(|c| c.codec == ChunkCodec::None),
        "random data should stay raw"
    );
    let fs = repo.snapshot_at("/a", 9999).unwrap().unwrap();
    let dst = tmp.path().join("out.bin");
    repo.restore(&fs, &dst).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), data);
}

#[test]
fn dedup_hit_keeps_first_writers_codec() {
    let tmp = tempfile::tempdir().unwrap();
    let data = vec![5u8; 2 * MIB]; // compressible
    let mut repo = Repository::open_with_compression(tmp.path(), always()).unwrap();

    let id1 = repo
        .snapshot_bytes(SnapshotKind::Backup, "first", 1000, &[("/a", &data)])
        .unwrap();
    assert!(
        repo.snapshot(id1).unwrap().unwrap().files[0]
            .manifest
            .chunks
            .iter()
            .all(|c| c.codec == ChunkCodec::Zstd)
    );

    // Re-snapshot the SAME content with compression OFF → dedup hit → the
    // manifest must record the EXISTING (Zstd) codec, not None.
    repo.set_compression(RepoCompression::Off);
    let id2 = repo
        .snapshot_bytes(SnapshotKind::Backup, "second", 2000, &[("/b", &data)])
        .unwrap();
    assert!(
        repo.snapshot(id2).unwrap().unwrap().files[0]
            .manifest
            .chunks
            .iter()
            .all(|c| c.codec == ChunkCodec::Zstd),
        "dedup hit must keep the first writer's codec"
    );

    // Still restores byte-for-byte.
    let fs = repo.snapshot_at("/b", 9999).unwrap().unwrap();
    let dst = tmp.path().join("out.bin");
    repo.restore(&fs, &dst).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), data);
}
