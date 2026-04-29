//! Typed errors for the shred pipeline.

use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Classification of a shred-pipeline failure.
///
/// Mirrors the shape of `copythat_core::CopyErrorKind` so UI code can
/// route both through the same retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShredErrorKind {
    NotFound,
    PermissionDenied,
    /// Caller cancelled the operation between passes or inside a pass
    /// buffer boundary.
    Interrupted,
    /// A pass's `verify` step read bytes that didn't match the pattern
    /// the engine wrote. Typically indicates unreliable hardware; the
    /// file is left on disk so the caller can investigate.
    VerifyFailed,
    /// The caller asked for [`crate::ShredMethod::Nist80088Purge`] but
    /// the hardware secure-erase path isn't available on this platform
    /// / device combination. Recommend Clear + full-disk-encryption key
    /// rotation instead.
    PurgeNotSupported,
    /// Phase 44 — the per-file shred is meaningless on this
    /// filesystem (CoW: Btrfs / ZFS / APFS, or thin-provisioned
    /// LVM). Block-level overwrites won't reach the original
    /// content. Caller should route the user to whole-drive
    /// sanitize ([`crate::SsdSanitizeMode`]) plus FDE key rotation.
    ShredMeaningless,
    /// The target is a directory but the caller passed it to
    /// `shred_file`, or a symlink which the shredder refuses to follow.
    BadTarget,
    /// Any other I/O failure (disk full, etc.).
    IoOther,
}

/// The shredder's public error. Always carries the target path.
#[derive(Debug, Error, Clone)]
pub struct ShredError {
    pub kind: ShredErrorKind,
    pub path: PathBuf,
    pub raw_os_error: Option<i32>,
    pub message: String,
}

impl std::fmt::Display for ShredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "shred {}: {} ({:?}",
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

impl ShredError {
    pub(crate) fn from_io(path: &Path, err: io::Error) -> Self {
        let raw = err.raw_os_error();
        let kind = match err.kind() {
            io::ErrorKind::NotFound => ShredErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ShredErrorKind::PermissionDenied,
            io::ErrorKind::Interrupted => ShredErrorKind::Interrupted,
            _ => ShredErrorKind::IoOther,
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
            kind: ShredErrorKind::Interrupted,
            path: path.to_path_buf(),
            raw_os_error: None,
            message: "shred cancelled by caller".to_string(),
        }
    }

    pub(crate) fn verify_failed(path: &Path, pass_index: usize) -> Self {
        Self {
            kind: ShredErrorKind::VerifyFailed,
            path: path.to_path_buf(),
            raw_os_error: None,
            message: format!(
                "pass {pass_index} verify failed — bytes on disk diverge from pattern"
            ),
        }
    }

    pub(crate) fn purge_not_supported(path: &Path) -> Self {
        Self {
            kind: ShredErrorKind::PurgeNotSupported,
            path: path.to_path_buf(),
            raw_os_error: None,
            message:
                "NIST 800-88 Purge requires a hardware secure-erase path (ATA SECURE ERASE or \
                      NVMe Format w/ Secure Erase) that is not available here; use Clear plus \
                      full-disk-encryption key rotation instead"
                    .to_string(),
        }
    }

    pub(crate) fn bad_target(path: &Path, reason: &'static str) -> Self {
        Self {
            kind: ShredErrorKind::BadTarget,
            path: path.to_path_buf(),
            raw_os_error: None,
            message: reason.to_string(),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind == ShredErrorKind::Interrupted && self.raw_os_error.is_none()
    }

    pub fn is_verify_failed(&self) -> bool {
        self.kind == ShredErrorKind::VerifyFailed
    }

    pub fn is_purge_not_supported(&self) -> bool {
        self.kind == ShredErrorKind::PurgeNotSupported
    }
}
