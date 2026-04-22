//! Typed errors for the copy engine.

use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::safety::PathSafetyError;

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
    /// Phase 17a — the IPC / CLI caller gave us a path containing
    /// traversal (`..`) components or other unsafe bytes. The engine
    /// refused before opening anything, so no partial destination
    /// exists. See `copythat_core::safety`.
    PathEscape,
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

    /// Stable i18n key that every UI renders through its locale table.
    ///
    /// The key is constant per variant so Phase 11's full i18n audit
    /// doesn't need to touch the engine again — translations evolve
    /// by editing the locale `.ftl` files. The `err-` prefix scopes
    /// these inside the existing key namespace.
    pub const fn localized_key(self) -> &'static str {
        match self {
            Self::NotFound => "err-not-found",
            Self::PermissionDenied => "err-permission-denied",
            Self::DiskFull => "err-disk-full",
            Self::Interrupted => "err-interrupted",
            Self::VerifyFailed => "err-verify-failed",
            Self::PathEscape => "err-path-escape",
            Self::IoOther => "err-io-other",
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

    /// Build a `CopyError::PathEscape` from a safety-layer rejection.
    /// Carries both src + dst so the UI row still has full context —
    /// even though by construction one of them is the path that
    /// failed the lexical traversal check.
    pub fn path_escape(src: &Path, dst: &Path, reason: PathSafetyError) -> Self {
        Self {
            kind: CopyErrorKind::PathEscape,
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: None,
            message: reason.to_string(),
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

    /// Convenience delegator; spares call sites a two-hop through
    /// `self.kind.localized_key()`.
    pub const fn localized_key(&self) -> &'static str {
        self.kind.localized_key()
    }
}
