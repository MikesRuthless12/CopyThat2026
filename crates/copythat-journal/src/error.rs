//! Typed failure surface.

use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, JournalError>;

/// Why a journal call failed.
///
/// Variants mirror the cases the Tauri runner branches on:
/// `NoDataDir` triggers a "history-unavailable"-style toast at boot;
/// `Database`/`Io` are surfaced as a typed crash log (the journal is
/// best-effort — the engine still copies, just without resume).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JournalError {
    /// `directories::ProjectDirs` couldn't resolve a user-data
    /// directory (rare — sandboxed cron-ish envs). Falls through to
    /// "journal disabled this run".
    #[error("could not resolve OS user-data directory")]
    NoDataDir,

    /// I/O error opening or fsyncing the redb file.
    #[error("journal I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// redb library error — wraps every kind so callers don't have
    /// to match on the four redb error families separately.
    #[error("journal database error: {0}")]
    Database(String),

    /// Caller asked for a record that isn't in the journal. Includes
    /// the requested ID for triage.
    #[error("journal entry not found: job {0:?}")]
    NotFound(u64),

    /// Serde JSON failed to encode / decode a journal value. Should
    /// not happen in practice (every type round-trips in unit tests);
    /// surfaced as a typed error so a corrupt on-disk row doesn't
    /// panic the runner at boot.
    #[error("journal codec error: {0}")]
    Codec(String),

    /// The journal file was written by a newer build with a higher
    /// schema version than this binary knows about. Refusing to
    /// load is safer than re-interpreting the rows under the old
    /// shape — `dst_path` semantics or `JobStatus` variants could
    /// have drifted.
    #[error("journal schema error: {0}")]
    Schema(String),
}

impl From<redb::Error> for JournalError {
    fn from(e: redb::Error) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::DatabaseError> for JournalError {
    fn from(e: redb::DatabaseError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::TransactionError> for JournalError {
    fn from(e: redb::TransactionError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::TableError> for JournalError {
    fn from(e: redb::TableError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::StorageError> for JournalError {
    fn from(e: redb::StorageError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::CommitError> for JournalError {
    fn from(e: redb::CommitError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<serde_json::Error> for JournalError {
    fn from(e: serde_json::Error) -> Self {
        Self::Codec(e.to_string())
    }
}
