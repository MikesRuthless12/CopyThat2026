//! Typed failure surface.

use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, WatchError>;

/// Why a watcher call failed.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WatchError {
    /// The root path doesn't exist or isn't accessible.
    #[error("watch root not accessible: {path} ({reason})")]
    RootNotAccessible { path: PathBuf, reason: String },

    /// Underlying `notify` crate error, wrapped as a string so this
    /// error type doesn't propagate the full `notify::Error` generic
    /// surface.
    #[error("watcher backend error: {0}")]
    Backend(String),

    /// I/O error while rescanning a directory after an overflow.
    #[error("watcher I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl From<notify::Error> for WatchError {
    fn from(e: notify::Error) -> Self {
        Self::Backend(e.to_string())
    }
}
