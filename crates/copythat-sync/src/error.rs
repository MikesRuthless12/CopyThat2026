//! Typed failure surface.

use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SyncError>;

/// Why a sync call failed.
///
/// The variants deliberately avoid swallowing underlying errors —
/// every `Io` carries its offending path + `io::Error`, every
/// `Database` carries the redb error string, and `Cancelled` is a
/// distinct variant the Tauri runner maps to a `cancelled` event.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SyncError {
    /// The pair root on one of the two sides does not exist or isn't
    /// accessible. The engine's pre-flight check surfaces this before
    /// a single file is touched so a typo or an unmounted share
    /// aborts cleanly.
    #[error("sync root not accessible: {path} ({reason})")]
    RootNotAccessible { path: PathBuf, reason: String },

    /// File-system error while walking / reading / writing. Carries
    /// the path at which the error surfaced so the UI can render a
    /// per-file row rather than a generic "sync failed".
    #[error("sync I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// redb library error — wraps every kind so callers don't have to
    /// match on the four redb error families separately.
    #[error("sync database error: {0}")]
    Database(String),

    /// Serde JSON failed to encode / decode a row. Should not happen
    /// in practice (every type round-trips in unit tests); surfaced
    /// as a typed error so a corrupt on-disk row never panics.
    #[error("sync codec error: {0}")]
    Codec(String),

    /// The copy engine returned an error while propagating a file.
    /// Carries the error's localized key + display string verbatim so
    /// the UI can render its existing error toast.
    #[error("sync propagation failed for {relpath}: {inner}")]
    Propagation {
        relpath: String,
        #[source]
        inner: Box<copythat_core::CopyError>,
    },

    /// Caller issued a cancel via [`crate::SyncControl::cancel`]. The
    /// engine aborts cleanly — already-applied actions stay applied,
    /// the DB remains consistent, and the caller surfaces the outcome
    /// to the UI.
    #[error("sync cancelled by caller")]
    Cancelled,

    /// A relpath the sync layer was about to operate on contained
    /// `..`, control characters, or a Windows drive prefix — any of
    /// which would let an attacker who can write the
    /// `.copythat-sync.db` baseline plant a key that escapes the
    /// pair root. The engine refuses to delete / rename / copy the
    /// offending entry; a future sync round will skip it again.
    #[error("sync refused unsafe relpath: {0}")]
    UnsafeRelpath(String),
}

impl From<redb::Error> for SyncError {
    fn from(e: redb::Error) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::DatabaseError> for SyncError {
    fn from(e: redb::DatabaseError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::TransactionError> for SyncError {
    fn from(e: redb::TransactionError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::TableError> for SyncError {
    fn from(e: redb::TableError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::StorageError> for SyncError {
    fn from(e: redb::StorageError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<redb::CommitError> for SyncError {
    fn from(e: redb::CommitError) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<serde_json::Error> for SyncError {
    fn from(e: serde_json::Error) -> Self {
        Self::Codec(e.to_string())
    }
}
