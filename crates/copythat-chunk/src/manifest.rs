//! File-level operations: chunk a file, write chunks + manifest to a
//! store, reassemble a file from a manifest.
//!
//! This is the glue layer. `Chunker` knows how to cut bytes,
//! `ChunkStore` knows how to persist them — `ingest_file` and
//! `materialise_file` wire the two together into the API the engine
//! actually calls.
//!
//! `ingest_file` is the side effect of "run a copy with delta
//! tracking on": for every source file, slice it into chunks, put
//! each chunk into the store (dedup wins already show up here), and
//! write a `Manifest` keyed by the caller's choice (usually the
//! destination absolute path).
//!
//! `materialise_file` is the re-read path — given a manifest key, it
//! streams the chunks back into a destination file, which lets the
//! chunk store double as a restore target for the moonshot phases.

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use crate::chunker::Chunker;
use crate::error::{ChunkStoreError, Result};
use crate::store::ChunkStore;
use crate::types::{Blake3Hash, ChunkRef, Manifest};

/// Summary returned from `ingest_file`. Lets the engine emit the
/// `ChunkStoreSavings` event with real numbers instead of a
/// pessimistic guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IngestStats {
    /// Total chunks in the file.
    pub chunks_total: u64,
    /// Chunks that were new to the store (had to be appended to a
    /// pack file).
    pub chunks_new: u64,
    /// Chunks that dedup'd against the store (no pack write).
    pub chunks_dedup: u64,
    /// Bytes that were new to the store.
    pub bytes_new: u64,
    /// Bytes that dedup'd. This is what `ChunkStoreSavings` reports.
    pub bytes_dedup: u64,
}

/// Chunk `path`, put every chunk into `store`, persist a manifest
/// under `manifest_key`, and return an `(IngestStats, Manifest)`.
///
/// Reads `path` exactly once. The manifest is written via
/// `store.put_manifest` before return, so a crash after this call
/// leaves the store in a valid state.
pub fn ingest_file(
    store: &ChunkStore,
    chunker: &Chunker,
    path: &Path,
    manifest_key: &str,
) -> Result<(IngestStats, Manifest)> {
    let mut buf = Vec::new();
    BufReader::new(File::open(path).map_err(|e| ChunkStoreError::Io {
        path: path.to_path_buf(),
        source: e,
    })?)
    .read_to_end(&mut buf)
    .map_err(|e| ChunkStoreError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    ingest_bytes(store, chunker, &buf, manifest_key)
}

/// In-memory ingest — used by the smoke test and by callers that
/// already have the bytes in a buffer (e.g. the engine when it's
/// reading via `CopyFileExW` chunk callbacks).
pub fn ingest_bytes(
    store: &ChunkStore,
    chunker: &Chunker,
    bytes: &[u8],
    manifest_key: &str,
) -> Result<(IngestStats, Manifest)> {
    let file_hash: Blake3Hash = *blake3::hash(bytes).as_bytes();
    let cuts = chunker.chunk_bytes(bytes);
    let mut stats = IngestStats::default();
    stats.chunks_total = cuts.len() as u64;
    let mut chunks = Vec::with_capacity(cuts.len());
    for c in &cuts {
        let slice = &bytes[c.offset as usize..(c.offset as usize + c.len as usize)];
        if store.has(&c.hash)? {
            stats.chunks_dedup += 1;
            stats.bytes_dedup += u64::from(c.len);
        } else {
            store.put(c.hash, slice)?;
            stats.chunks_new += 1;
            stats.bytes_new += u64::from(c.len);
        }
        chunks.push(ChunkRef {
            hash: c.hash,
            offset: c.offset,
            len: c.len,
        });
    }
    let manifest = Manifest {
        file_hash,
        size: bytes.len() as u64,
        chunks,
    };
    store.put_manifest(manifest_key, &manifest)?;
    Ok((stats, manifest))
}

/// Reassemble a file from its manifest.
///
/// `dst` must be openable for writing (either created anew or
/// truncated). Each chunk is pulled from the store in manifest order;
/// a `MissingChunk` error surfaces if a referenced chunk has been
/// garbage collected, and `CorruptChunk` surfaces if a pack failed
/// the BLAKE3 verification on read-back.
pub fn materialise_file(store: &ChunkStore, manifest: &Manifest, dst: &Path) -> Result<()> {
    let mut out = File::create(dst).map_err(|e| ChunkStoreError::Io {
        path: dst.to_path_buf(),
        source: e,
    })?;
    for c in &manifest.chunks {
        let bytes = store
            .get(&c.hash)?
            .ok_or_else(|| ChunkStoreError::MissingChunk {
                hash: crate::types::hex_of(&c.hash),
            })?;
        if bytes.len() != c.len as usize {
            return Err(ChunkStoreError::CorruptChunk {
                hash: crate::types::hex_of(&c.hash),
            });
        }
        out.write_all(&bytes).map_err(|e| ChunkStoreError::Io {
            path: dst.to_path_buf(),
            source: e,
        })?;
    }
    out.flush().map_err(|e| ChunkStoreError::Io {
        path: dst.to_path_buf(),
        source: e,
    })?;
    Ok(())
}

/// Planning helper for delta-resume. Given an `old` manifest (what
/// the store last saw for this destination) and a `new` manifest
/// (just computed from the current source), return the ordered list
/// of chunk hashes that are new in `new` and must be re-written to
/// the destination.
///
/// Chunks that appear in both are skipped — their bytes on-disk in
/// the destination are already correct modulo the engine's usual
/// post-copy verify.
#[must_use]
pub fn delta_plan(old: &Manifest, new: &Manifest) -> Vec<ChunkRef> {
    let old_hashes: std::collections::HashSet<Blake3Hash> =
        old.chunks.iter().map(|c| c.hash).collect();
    new.chunks
        .iter()
        .filter(|c| !old_hashes.contains(&c.hash))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deterministic_bytes(seed: u64, len: usize) -> Vec<u8> {
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
    fn ingest_bytes_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let bytes = deterministic_bytes(1, 3 * 1024 * 1024);
        let (stats, manifest) =
            ingest_bytes(&store, &Chunker::default(), &bytes, "/dst/foo").unwrap();
        assert!(stats.chunks_total > 0);
        assert_eq!(stats.chunks_dedup, 0, "fresh store has no dedup");
        assert_eq!(manifest.size, bytes.len() as u64);
        // Manifest persisted.
        let got = store.get_manifest("/dst/foo").unwrap().unwrap();
        assert_eq!(got, manifest);

        // Materialise back.
        let dst = tmp.path().join("out.bin");
        materialise_file(&store, &manifest, &dst).unwrap();
        let reread = std::fs::read(&dst).unwrap();
        assert_eq!(reread, bytes);
    }

    #[test]
    fn second_ingest_of_same_bytes_all_dedup() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let bytes = deterministic_bytes(2, 2 * 1024 * 1024);
        let (first, _) = ingest_bytes(&store, &Chunker::default(), &bytes, "a").unwrap();
        let size_after_first = store.disk_usage_bytes().unwrap();
        let (second, _) = ingest_bytes(&store, &Chunker::default(), &bytes, "b").unwrap();
        let size_after_second = store.disk_usage_bytes().unwrap();
        assert_eq!(second.chunks_new, 0, "second ingest must dedup fully");
        assert_eq!(second.chunks_dedup, first.chunks_total);
        assert_eq!(
            size_after_first, size_after_second,
            "disk usage unchanged when 100% dedup hits"
        );
    }

    #[test]
    fn delta_plan_returns_only_new_chunks() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let v1 = deterministic_bytes(3, 2 * 1024 * 1024);
        let mut v2 = v1.clone();
        // Flip one byte in the middle.
        v2[1024 * 1024] ^= 0xff;
        let (_, m1) = ingest_bytes(&store, &Chunker::default(), &v1, "dst").unwrap();
        let (_, m2) = ingest_bytes(&store, &Chunker::default(), &v2, "dst2").unwrap();
        let plan = delta_plan(&m1, &m2);
        assert!(
            !plan.is_empty(),
            "single-byte change produces at least one new chunk"
        );
        // Key invariant: the plan is strictly smaller than the full
        // set of chunks. FastCDC's small-mutation resilience means a
        // single-byte flip typically touches only 1–3 chunks, but
        // exact counts depend on the cut-point boundary, so we only
        // assert the strict inequality.
        assert!(
            plan.len() < m2.chunks.len(),
            "plan {} vs total {}",
            plan.len(),
            m2.chunks.len()
        );
    }
}
