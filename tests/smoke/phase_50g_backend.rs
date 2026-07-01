//! Phase 50g smoke — the pluggable `BlobBackend` + `ChunkStore` routing pack
//! I/O through it. The full chunk suite already exercises the store over the
//! default `LocalFsBackend`; this pins the public backend contract (both
//! shipped backends) + that the store's disk accounting reads via the backend.

use copythat_chunk::{BlobBackend, ChunkStore, Chunker, LocalFsBackend, MemBackend, ingest_bytes};

#[test]
fn shipped_backends_honour_the_contract() {
    let tmp = tempfile::tempdir().unwrap();
    let backends: Vec<Box<dyn BlobBackend>> = vec![
        Box::new(LocalFsBackend::new(tmp.path().to_path_buf())),
        Box::new(MemBackend::default()),
    ];
    for be in backends {
        assert!(!be.exists("pack-1.pack").unwrap());
        assert_eq!(be.size("pack-1.pack").unwrap(), None);
        be.put("pack-1.pack", b"abcdefgh").unwrap();
        assert!(be.exists("pack-1.pack").unwrap());
        assert_eq!(be.size("pack-1.pack").unwrap(), Some(8));
        assert_eq!(
            be.get("pack-1.pack").unwrap().as_deref(),
            Some(&b"abcdefgh"[..])
        );
        assert_eq!(
            be.get_range("pack-1.pack", 2, 3).unwrap().as_deref(),
            Some(&b"cde"[..])
        );
        be.put("pack-2.pack", b"z").unwrap();
        let mut names = be.list("pack-").unwrap();
        names.sort();
        assert_eq!(
            names,
            vec!["pack-1.pack".to_string(), "pack-2.pack".to_string()]
        );
        assert_eq!(be.delete("pack-1.pack").unwrap(), 8);
        assert_eq!(be.delete("pack-1.pack").unwrap(), 0); // idempotent
    }
}

#[test]
fn chunk_store_disk_accounting_reads_via_backend() {
    let tmp = tempfile::tempdir().unwrap();
    let store = ChunkStore::open(tmp.path()).unwrap();
    let chunker = Chunker::default();
    let data = vec![9u8; 200_000];
    let (_stats, manifest) = ingest_bytes(&store, &chunker, &data, "/f").unwrap();

    // disk_usage_bytes now sums pack sizes through the backend.
    assert!(store.disk_usage_bytes().unwrap() >= data.len() as u64);
    // ... and get reads the chunk back through the backend.
    let h = manifest.chunks[0].hash;
    assert!(store.get(&h).unwrap().is_some());
}
