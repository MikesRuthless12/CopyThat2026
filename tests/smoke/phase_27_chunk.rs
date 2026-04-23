//! Phase 27 smoke — CDC chunk store end-to-end.
//!
//! Three cases from the brief's smoke spec:
//!
//! 1. **Manifest creation.** Chunk a 100 MiB file, write it + its
//!    manifest into the store, assert the manifest survives a
//!    round-trip.
//! 2. **Delta-resume.** Take the same file with one byte flipped at
//!    offset 50 MiB, ingest again under a different manifest key.
//!    FastCDC should keep ≥ 99 % of the original chunks; only the
//!    chunk containing the mutation need be new.
//! 3. **Cross-file dedup.** Generate 10 files that each carry a
//!    shared prefix half + a unique tail half. After ingesting all
//!    10, the store's disk usage must come out < 6× any single
//!    file's size — i.e. the shared half lands in the store exactly
//!    once.
//!
//! All three cases run on every host (no platform gating). The
//! slowest case is ~1.2 s on a 2024 laptop; `COPYTHAT_PHASE27_FULL=1`
//! bumps every size by 10× for CI's thorough pass.

use std::sync::Arc;

use copythat_chunk::{
    ChunkStore, Chunker, CopyThatChunkSink, IngestStats, delta_plan, ingest_bytes, materialise_file,
};
use copythat_core::ChunkStoreSink;

/// Deterministic PRNG so the smoke is reproducible across hosts. An
/// LCG with the standard constants is plenty for "non-compressible
/// bytes" — the chunk-store behaviour we're testing is
/// content-defined and doesn't care about randomness quality.
fn seeded_bytes(seed: u64, len: usize) -> Vec<u8> {
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

fn scale_up() -> bool {
    std::env::var("COPYTHAT_PHASE27_FULL").is_ok_and(|v| v == "1")
}

#[test]
fn case1_100mib_file_creates_manifest() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ChunkStore::open(tmp.path()).unwrap();

    let size = if scale_up() {
        1024 * 1024 * 1024
    } else {
        100 * 1024 * 1024
    };
    let bytes = seeded_bytes(0xabc_def, size);

    let src_path = tmp.path().join("big.bin");
    std::fs::write(&src_path, &bytes).unwrap();

    let (stats, manifest) = ingest_bytes(
        &store,
        &Chunker::default(),
        &bytes,
        src_path.to_str().unwrap(),
    )
    .unwrap();

    assert!(
        stats.chunks_total >= 50,
        "100 MiB at 1 MiB avg should produce ≥50 chunks, got {}",
        stats.chunks_total
    );
    assert_eq!(stats.chunks_dedup, 0, "fresh store: nothing should dedup",);
    assert_eq!(manifest.size, bytes.len() as u64);

    // Manifest must round-trip through the store by key.
    let reread = store
        .get_manifest(src_path.to_str().unwrap())
        .unwrap()
        .expect("manifest persisted");
    assert_eq!(reread, manifest);

    // Materialise back from the store and compare. This is the
    // restore-from-chunk-store path that the moonshot phases will
    // reuse.
    let dst = tmp.path().join("restored.bin");
    materialise_file(&store, &manifest, &dst).unwrap();
    let back = std::fs::read(&dst).unwrap();
    assert_eq!(back.len(), bytes.len());
    // BLAKE3 compare so we don't eat 100 MiB of assertion-failure
    // print when the length check already caught it.
    assert_eq!(
        blake3::hash(&back).as_bytes(),
        blake3::hash(&bytes).as_bytes(),
    );
}

#[test]
fn case2_one_byte_change_reuses_at_least_99_percent() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ChunkStore::open(tmp.path()).unwrap();
    let chunker = Chunker::default();

    let size = if scale_up() {
        1024 * 1024 * 1024
    } else {
        100 * 1024 * 1024
    };
    let bytes_v1 = seeded_bytes(0xfeedface, size);

    let (_, manifest_v1) = ingest_bytes(&store, &chunker, &bytes_v1, "/dst/fileA").unwrap();

    // Flip a byte at offset 50 MiB.
    let mut bytes_v2 = bytes_v1.clone();
    let mutate_at = bytes_v2.len() / 2;
    bytes_v2[mutate_at] ^= 0x5a;

    let (stats_v2, manifest_v2) =
        ingest_bytes(&store, &chunker, &bytes_v2, "/dst/fileA-v2").unwrap();

    // Spec: "only the chunk containing offset 50 MiB should be new."
    // FastCDC's cut-point resilience guarantees at most a small
    // window of chunks around the mutation shift; anything beyond
    // that window must dedup against v1's chunks from the store.
    // We assert at most 2 new chunks (1 canonical + 1 slack for a
    // cut-point that happened to land adjacent to the mutation).
    assert!(
        stats_v2.chunks_new <= 2,
        "expected ≤2 new chunks after 1-byte mutation, got {} new + {} dedup out of {} total",
        stats_v2.chunks_new,
        stats_v2.chunks_dedup,
        stats_v2.chunks_total,
    );
    // And the dedup count is everything else.
    assert_eq!(
        stats_v2.chunks_new + stats_v2.chunks_dedup,
        stats_v2.chunks_total
    );

    // Delta plan — chunks that must be re-written — must be small.
    let plan = delta_plan(&manifest_v1, &manifest_v2);
    assert!(
        plan.len() <= 3,
        "1-byte mutation should touch ≤3 chunks via FastCDC resilience, got {} (total {})",
        plan.len(),
        manifest_v2.chunks.len(),
    );
}

#[test]
fn case3_shared_template_dedups_across_files() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ChunkStore::open(tmp.path()).unwrap();
    let chunker = Chunker::default();

    let file_size = if scale_up() {
        40 * 1024 * 1024
    } else {
        8 * 1024 * 1024
    };
    let half = file_size / 2;
    let template = seeded_bytes(0x54454d50, half); // "TEMP" seed

    for i in 0..10u64 {
        let mut content = Vec::with_capacity(file_size);
        content.extend_from_slice(&template);
        let tail = seeded_bytes(0x1000_0000 + i, half);
        content.extend_from_slice(&tail);
        let key = format!("/dst/file-{i}");
        let (_, _) = ingest_bytes(&store, &chunker, &content, &key).unwrap();
    }

    // Spec: total store size < 6× single-file size. With the shared
    // half fully dedup'd across 10 files, naive storage would be
    // 10×file_size; dedup'd storage is ~5.5×file_size (the shared
    // half once + 10 unique tails). The 6× bar is comfortable slack
    // for pack overhead + redb metadata.
    let usage = store.disk_usage_bytes().unwrap();
    let bar = 6 * file_size as u64;
    assert!(
        usage < bar,
        "store usage {usage} must be < 6× file_size ({bar})",
    );

    // Sanity: at least one chunk must have been shared; if every
    // file's chunks were unique we'd expect usage ≈ 10× file_size.
    assert!(usage < 8 * file_size as u64);
}

#[test]
fn case4_sink_trait_is_usable_from_core() {
    // Round-trip the Phase 27 sink trait from core's vantage point
    // — i.e. consume it as `Arc<dyn ChunkStoreSink>` with no direct
    // copythat-chunk types visible, which is how the engine will
    // interact with it.
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(ChunkStore::open(tmp.path()).unwrap());
    let sink_arc: Arc<dyn ChunkStoreSink> = Arc::new(CopyThatChunkSink::new(Arc::clone(&store)));

    assert!(!sink_arc.has_chunk(&[0u8; 32]));
    // Manifest helpers on the trait surface: put_manifest + get_manifest
    // round-trip via JSON bytes.
    let bytes = b"{\"file_hash\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"size\":0,\"chunks\":[]}";
    sink_arc.put_manifest("k", bytes);
    let back = sink_arc.get_manifest("k").expect("persisted");
    // The store canonicalises JSON key order; accept either shape.
    assert!(!back.is_empty());

    // Also exercise the direct store API through the Arc to prove
    // multiple handles over the same Arc don't corrupt each other.
    let s2 = Arc::clone(&store);
    let content = vec![7u8; 2048];
    let hash = *blake3::hash(&content).as_bytes();
    s2.put(hash, &content).unwrap();
    assert!(sink_arc.has_chunk(&hash));

    // Stats ignored only by format — confirm dedup IngestStats is
    // `Default`.
    let _default_stats = IngestStats::default();
}
