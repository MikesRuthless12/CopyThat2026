//! Typed errors for the hashing pipeline.

use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Classification of a hash-pipeline failure. Mirrors the shape of
/// `copythat_core::CopyErrorKind` so UI code can route both into the
/// same retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashErrorKind {
    NotFound,
    PermissionDenied,
    Interrupted,
    /// Underlying I/O failure that didn't fit one of the specific kinds.
    IoOther,
}

/// Error returned by `hash_file_async`. Always carries the path the
/// engine was operating on.
#[derive(Debug, Error, Clone)]
pub struct HashError {
    pub kind: HashErrorKind,
    pub path: PathBuf,
    pub raw_os_error: Option<i32>,
    pub message: String,
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hash {}: {} ({:?}",
            self.path.display(),
            self.message,
            self.kind
        )?;
        if let Some(errno) = self.raw_os_error {
            write!(f, ", os={errno}")?;
        }
        write!(f, ")")
    }
}

impl HashError {
    pub(crate) fn from_io(path: &Path, err: io::Error) -> Self {
        let raw = err.raw_os_error();
        let kind = match err.kind() {
            io::ErrorKind::NotFound => HashErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => HashErrorKind::PermissionDenied,
            io::ErrorKind::Interrupted => HashErrorKind::Interrupted,
            _ => HashErrorKind::IoOther,
        };
        Self {
            kind,
            path: path.to_path_buf(),
            raw_os_error: raw,
            message: err.to_string(),
        }
    }

    pub(crate) fn cancelled(path: &Path) -> Self {
        Self {
            kind: HashErrorKind::Interrupted,
            path: path.to_path_buf(),
            raw_os_error: None,
            message: "hash cancelled by caller".to_string(),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind == HashErrorKind::Interrupted && self.raw_os_error.is_none()
    }
}
