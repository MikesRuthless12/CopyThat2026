//! Typed error surface. Same shape as `copythat-history::Error`
//! (thiserror-based, `Result<T, SettingsError>` alias) so Tauri
//! handlers can `map_err(|e| e.to_string())` uniformly.

use std::io;
use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SettingsError>;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("could not resolve an OS config directory")]
    NoConfigDir,

    #[error("read {}: {source}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("write {}: {source}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("parse {}: {message}", path.display())]
    Parse { path: PathBuf, message: String },

    #[error("serialize: {message}")]
    Serialize { message: String },

    #[error("profile name must be non-empty and must not contain path separators or `.`")]
    InvalidProfileName,

    #[error("profile `{name}` not found")]
    ProfileNotFound { name: String },

    #[error("profile `{name}` already exists")]
    ProfileExists { name: String },

    #[error("io: {kind:?}")]
    Io { kind: io::ErrorKind },
}
