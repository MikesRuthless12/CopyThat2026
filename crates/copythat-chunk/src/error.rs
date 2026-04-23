//! Error type surfaced by the Phase 27 chunk store.
//!
//! The crate uses one `thiserror` enum so callers (the engine copy
//! loop, the Tauri IPC shim, the smoke test) can pattern-match on the
//! variant and react accordingly — a corrupt pack is a hard "discard
//! and fall back to full copy", a missing chunk entry is a
//! "re-upload", an I/O error on put is usually a disk-full that
//! surfaces through the normal copy error path.

use std::path::PathBuf;

/// Convenience `Result` alias for the crate.
pub type Result<T> = std::result::Result<T, ChunkStoreError>;

/// Every failure mode the chunk store can surface.
#[derive(Debug, thiserror::Error)]
pub enum ChunkStoreError {
    /// I/O against a pack file or the redb index.
    #[error("I/O error at {path}: {source}")]
    Io {
        /// Offending path (pack file, manifest key, or the store root).
        path: PathBuf,
        /// Underlying `std::io::Error`.
        #[source]
        source: std::io::Error,
    },

    /// The redb backing store refused a transaction. Usually means
    /// either the index file is corrupt or another process is holding
    /// an exclusive lock.
    #[error("redb error: {0}")]
    Redb(String),

    /// A manifest's JSON deserialisation failed. Indicates a
    /// forward/backward incompatible on-disk format change — the
    /// engine falls back to a full copy and the store should be
    /// rebuilt.
    #[error("manifest serialisation error: {0}")]
    Manifest(#[from] serde_json::Error),

    /// A requested chunk wasn't in the store but the caller expected
    /// it to be (e.g. a manifest references a hash that's been garbage
    /// collected).
    #[error("missing chunk: {hash}")]
    MissingChunk {
        /// Hex-encoded BLAKE3 of the missing chunk.
        hash: String,
    },

    /// A chunk's bytes read back from the pack didn't BLAKE3-hash to
    /// the expected value. On-disk corruption or bit rot.
    #[error("corrupt chunk at {hash}: pack verify failed")]
    CorruptChunk {
        /// Hex-encoded expected BLAKE3.
        hash: String,
    },

    /// Couldn't locate the OS user-data dir. Same failure shape as
    /// Phase 9 / Phase 20.
    #[error("could not locate user data directory for default chunk store path")]
    NoDataDir,
}

// redb's public error types are split across `DatabaseError`,
// `TransactionError`, `TableError`, `StorageError`, `CommitError`.
// Collapse them into our `Redb` variant with a human-readable string
// so downstream matching stays simple — corrupted redb is always the
// same recovery path (nuke + rebuild), the exact subtype doesn't
// change behaviour.
impl From<redb::DatabaseError> for ChunkStoreError {
    fn from(e: redb::DatabaseError) -> Self {
        Self::Redb(e.to_string())
    }
}
impl From<redb::TransactionError> for ChunkStoreError {
    fn from(e: redb::TransactionError) -> Self {
        Self::Redb(e.to_string())
    }
}
impl From<redb::TableError> for ChunkStoreError {
    fn from(e: redb::TableError) -> Self {
        Self::Redb(e.to_string())
    }
}
impl From<redb::StorageError> for ChunkStoreError {
    fn from(e: redb::StorageError) -> Self {
        Self::Redb(e.to_string())
    }
}
impl From<redb::CommitError> for ChunkStoreError {
    fn from(e: redb::CommitError) -> Self {
        Self::Redb(e.to_string())
    }
}
