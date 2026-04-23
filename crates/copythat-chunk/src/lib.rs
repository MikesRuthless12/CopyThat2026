//! `copythat-chunk` — content-defined chunk store.
//!
//! Phase 27 foundation. Splits files with FastCDC (min=512 KiB,
//! avg=1 MiB, max=4 MiB — matches restic's tuning), keys every
//! resulting chunk by its BLAKE3 digest, and stores bytes in
//! rolling pack files with a redb index. Persists per-file
//! manifests so the engine can ask "what's new since last time I
//! copied this?" and only re-transfer the changed chunks.
//!
//! # Why content-defined chunking?
//!
//! Fixed-size chunking breaks on insertion: adding a single byte
//! at the start of a 1 GiB file invalidates every chunk after it.
//! FastCDC uses a rolling hash with cut-point resilience — any
//! single-byte edit invalidates at most the chunks covering the
//! mutation, leaving the surrounding chunks reusable.
//!
//! # Uses
//!
//! - **Delta-resume (Phase 27):** when a copy is retried, the
//!   engine compares the source's new manifest against the
//!   destination's old manifest; only the `delta_plan` chunks
//!   need re-writing.
//! - **Same-job dedup:** ten files sharing a template prefix store
//!   the prefix's chunks once. The spec's smoke case 3 verifies
//!   this: 10 files with 50% shared content take < 6× the size
//!   of a single file.
//! - **Moonshot Phases 49–51:** the store becomes the unified
//!   repository for sync + backup + versioning + encrypted
//!   collaboration.
//!
//! # Minimal example
//!
//! ```no_run
//! use copythat_chunk::{ChunkStore, Chunker, ingest_file};
//!
//! let store = ChunkStore::open(std::path::Path::new("/tmp/chunks"))?;
//! let chunker = Chunker::default();
//! let (stats, manifest) = ingest_file(
//!     &store,
//!     &chunker,
//!     std::path::Path::new("/data/big.bin"),
//!     "/dst/big.bin",
//! )?;
//! println!(
//!     "{} chunks, {} new, {} dedup",
//!     stats.chunks_total, stats.chunks_new, stats.chunks_dedup,
//! );
//! # Ok::<(), copythat_chunk::ChunkStoreError>(())
//! ```

#![forbid(unsafe_code)]

pub mod chunker;
pub mod error;
pub mod manifest;
pub mod sink;
pub mod store;
pub mod types;

pub use chunker::{Chunker, DEFAULT_AVG, DEFAULT_MAX, DEFAULT_MIN};
pub use error::{ChunkStoreError, Result};
pub use manifest::{IngestStats, delta_plan, ingest_bytes, ingest_file, materialise_file};
pub use sink::CopyThatChunkSink;
pub use store::{ChunkLocator, ChunkStore, PACK_ROLLOVER_BYTES, default_chunk_store_path};
pub use types::{Blake3Hash, Chunk, ChunkRef, Manifest, hex_of};
