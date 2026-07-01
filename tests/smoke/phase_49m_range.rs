//! Phase 49m smoke — chunk-range read (`materialise_range`) backing the
//! snapshot-mount read callback. Kernel-free: exercises the range math +
//! chunk assembly directly.

use copythat_chunk::{ChunkStore, Chunker, ingest_bytes, materialise_range};

#[test]
fn range_reads_match_the_original() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ChunkStore::open(tmp.path()).unwrap();
    let chunker = Chunker::default();

    // A multi-chunk file (> the chunker minimum) with pseudo-random content.
    let mut data = vec![0u8; 3 * 1024 * 1024];
    let mut s = 99u64;
    for b in &mut data {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        *b = (s >> 33) as u8;
    }
    let (_stats, manifest) = ingest_bytes(&store, &chunker, &data, "/f").unwrap();
    assert!(
        manifest.chunks.len() > 1,
        "file should span multiple chunks"
    );

    // Whole-file range reconstructs the original byte-for-byte.
    assert_eq!(
        materialise_range(&store, &manifest, 0, data.len()).unwrap(),
        data
    );

    // Arbitrary sub-ranges, including ones straddling chunk boundaries.
    for &(off, len) in &[
        (0u64, 10usize),
        (1_000_000, 500_000),
        (data.len() as u64 - 5, 5),
    ] {
        let got = materialise_range(&store, &manifest, off, len).unwrap();
        let end = (off as usize + len).min(data.len());
        assert_eq!(got, &data[off as usize..end], "range {off}+{len}");
    }

    // Read at/after EOF → empty; read past EOF is clamped to the tail.
    assert!(
        materialise_range(&store, &manifest, data.len() as u64, 100)
            .unwrap()
            .is_empty()
    );
    let tail = materialise_range(&store, &manifest, data.len() as u64 - 3, 100).unwrap();
    assert_eq!(tail, &data[data.len() - 3..]);
}
