//! Typed errors for the history store.

use std::path::PathBuf;

use thiserror::Error;

/// Every fallible entry point in the crate returns a
/// `Result<_, HistoryError>`. Keep the variants coarse — the
/// downstream Tauri layer translates to a `String` for IPC, and
/// finer classification isn't load-bearing for the UI.
#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("history I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("could not resolve user-data directory for CopyThat 2026")]
    NoDataDir,

    #[error("history database path is not valid UTF-8: {0}")]
    NonUtf8Path(PathBuf),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// The hand-rolled migrator saw an unexpected `user_version`
    /// (future schema, typically — a newer app wrote to this DB and
    /// the user downgraded). Carried here so the UI can surface a
    /// clear "please upgrade the app" message.
    #[error("unsupported history schema version {0}; expected {1}")]
    UnsupportedSchema(u32, u32),

    /// The `spawn_blocking` task panicked. Should never happen under
    /// normal operation; surface as a generic runtime error rather
    /// than swallowing it into a generic I/O failure.
    #[error("history worker thread panicked")]
    WorkerPanicked,
}
