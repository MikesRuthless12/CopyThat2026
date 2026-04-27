//! Bridge from [`ChunkStore`] to [`copythat_core::ChunkStoreSink`].
//!
//! The core engine holds a `dyn ChunkStoreSink` on
//! `CopyOptions::chunk_store` so it can consult the store without a
//! direct dep on redb / fastcdc / blake3. [`CopyThatChunkSink`] is
//! the adapter — a thin wrapper that forwards `get_manifest` /
//! `put_manifest` / `has_chunk` to the underlying store and swallows
//! any errors as `tracing::warn` + "act as if the store is absent".
//!
//! Swallowing errors is the right call: a failed redb txn must never
//! cancel an in-flight copy. The worst-case outcome of a store
//! outage is that dedup + delta-resume silently degrade to a
//! full-copy pass, which is correct behaviour.

use std::sync::Arc;

use copythat_core::ChunkStoreSink;

use crate::store::ChunkStore;
use crate::types::{Blake3Hash, Manifest};

/// Arc-wrapped chunk store that implements the engine's trait.
#[derive(Debug, Clone)]
pub struct CopyThatChunkSink {
    inner: Arc<ChunkStore>,
}

impl CopyThatChunkSink {
    /// Wrap an open [`ChunkStore`].
    #[must_use]
    pub fn new(store: Arc<ChunkStore>) -> Self {
        Self { inner: store }
    }

    /// Access the underlying store (e.g. for ingest_file).
    #[must_use]
    pub fn store(&self) -> Arc<ChunkStore> {
        Arc::clone(&self.inner)
    }
}

impl ChunkStoreSink for CopyThatChunkSink {
    fn get_manifest(&self, key: &str) -> Option<Vec<u8>> {
        match self.inner.get_manifest(key) {
            Ok(Some(m)) => match serde_json::to_vec(&m) {
                Ok(v) => Some(v),
                Err(e) => {
                    tracing::warn!(?e, "chunk store manifest reserialise failed");
                    None
                }
            },
            Ok(None) => None,
            Err(e) => {
                tracing::warn!(?e, key, "chunk store get_manifest failed");
                None
            }
        }
    }

    fn put_manifest(&self, key: &str, serialised: &[u8]) {
        let Ok(manifest) = serde_json::from_slice::<Manifest>(serialised) else {
            tracing::warn!(key, "chunk store put_manifest: malformed payload");
            return;
        };
        if let Err(e) = self.inner.put_manifest(key, &manifest) {
            tracing::warn!(?e, key, "chunk store put_manifest failed");
        }
    }

    fn has_chunk(&self, hash: &Blake3Hash) -> bool {
        self.inner.has(hash).unwrap_or_else(|e| {
            tracing::warn!(?e, "chunk store has_chunk probe failed");
            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sink_round_trips_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        let store = Arc::new(ChunkStore::open(tmp.path()).unwrap());
        let sink = CopyThatChunkSink::new(store);
        let m = Manifest {
            file_hash: [1u8; 32],
            size: 100,
            chunks: vec![],
        };
        let payload = serde_json::to_vec(&m).unwrap();
        sink.put_manifest("x", &payload);
        let got = sink.get_manifest("x").unwrap();
        let back: Manifest = serde_json::from_slice(&got).unwrap();
        assert_eq!(back, m);
    }

    #[test]
    fn sink_has_chunk_reports_indexed_hashes() {
        let tmp = tempfile::tempdir().unwrap();
        let store = Arc::new(ChunkStore::open(tmp.path()).unwrap());
        let bytes = vec![55u8; 1024];
        let hash = *blake3::hash(&bytes).as_bytes();
        store.put(hash, &bytes).unwrap();
        let sink = CopyThatChunkSink::new(store);
        assert!(sink.has_chunk(&hash));
        assert!(!sink.has_chunk(&[0u8; 32]));
    }
}
