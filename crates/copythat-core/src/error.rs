//! Typed errors for the copy engine.

use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Classification of an engine error. Distilled to the kinds the UI and
/// retry logic actually branch on; richer platform detail stays in the
/// wrapped `io::Error` on the `source` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyErrorKind {
    NotFound,
    PermissionDenied,
    DiskFull,
    Interrupted,
    /// Post-copy verification detected a hash mismatch between source
    /// and destination. The partial destination is removed unless
    /// `CopyOptions::keep_partial` is set.
    VerifyFailed,
    IoOther,
}

impl CopyErrorKind {
    fn from_io(kind: io::ErrorKind, raw_os: Option<i32>) -> Self {
        use io::ErrorKind::*;
        match kind {
            NotFound => Self::NotFound,
            PermissionDenied => Self::PermissionDenied,
            Interrupted => Self::Interrupted,
            _ => {
                if is_disk_full(kind, raw_os) {
                    Self::DiskFull
                } else {
                    Self::IoOther
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn is_disk_full(_kind: io::ErrorKind, raw_os: Option<i32>) -> bool {
    // ERROR_DISK_FULL = 112, ERROR_HANDLE_DISK_FULL = 39.
    matches!(raw_os, Some(112) | Some(39))
}

#[cfg(not(target_os = "windows"))]
fn is_disk_full(kind: io::ErrorKind, raw_os: Option<i32>) -> bool {
    // On recent Rust the dedicated StorageFull kind exists; older errno
    // paths still surface as ENOSPC = 28 on Linux/macOS/BSD.
    kind.to_string().eq_ignore_ascii_case("storage full") || matches!(raw_os, Some(28))
}

/// The engine's public error. Always carries the source and destination
/// path that the engine was operating on, so callers don't have to thread
/// them through separately.
#[derive(Debug, Error, Clone)]
pub struct CopyError {
    pub kind: CopyErrorKind,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub raw_os_error: Option<i32>,
    pub message: String,
}

impl std::fmt::Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "copy {} -> {}: {} ({:?}",
            self.src.display(),
            self.dst.display(),
            self.message,
            self.kind
        )?;
        if let Some(errno) = self.raw_os_error {
            write!(f, ", os={errno}")?;
        }
        write!(f, ")")
    }
}

impl CopyError {
    /// Build a `CopyError` from an `io::Error` plus the source / destination
    /// paths the engine was operating on.
    ///
    /// Used inside the engine and by sibling crates (e.g. `copythat-platform`)
    /// that surface OS-native fast-path failures through the same error type.
    pub fn from_io(src: &Path, dst: &Path, err: io::Error) -> Self {
        let raw = err.raw_os_error();
        Self {
            kind: CopyErrorKind::from_io(err.kind(), raw),
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: raw,
            message: err.to_string(),
        }
    }

    pub(crate) fn cancelled(src: &Path, dst: &Path) -> Self {
        Self {
            kind: CopyErrorKind::Interrupted,
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: None,
            message: "copy cancelled by caller".to_string(),
        }
    }

    pub(crate) fn verify_failed(
        src: &Path,
        dst: &Path,
        algorithm: &str,
        src_hex: &str,
        dst_hex: &str,
    ) -> Self {
        Self {
            kind: CopyErrorKind::VerifyFailed,
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: None,
            message: format!("verify mismatch ({algorithm}): src={src_hex} dst={dst_hex}"),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind == CopyErrorKind::Interrupted && self.raw_os_error.is_none()
    }

    pub fn is_verify_failed(&self) -> bool {
        self.kind == CopyErrorKind::VerifyFailed
    }
}
