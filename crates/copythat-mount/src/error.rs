//! Phase 33 — typed errors for mount operations.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MountError {
    /// The requested mountpoint is unsafe (path traversal, contains
    /// a NUL byte, or is empty). Caught by
    /// `copythat_core::safety::validate_path_no_traversal`.
    #[error("mountpoint rejected: {0}")]
    UnsafeMountpoint(String),

    /// The mountpoint already exists and is not an empty directory.
    /// Most FUSE backends require an empty dir to mount onto; WinFsp
    /// can mount over a drive letter. We reject non-empty dirs in
    /// both cases so behaviour is portable.
    #[error("mountpoint `{0}` is not an empty directory")]
    MountpointNotEmpty(PathBuf),

    /// The mount backend refused — either because the platform
    /// doesn't support it at build time (no `fuse` / `winfsp`
    /// feature), or because the kernel / driver isn't available at
    /// runtime (missing fusermount binary, WinFsp not installed).
    #[error("mount backend unavailable: {0}")]
    BackendUnavailable(String),

    /// A passed-through history lookup or chunk-store read failed.
    /// Detail kept as a string so we don't drag every downstream
    /// error type into the crate's public API.
    #[error("archive read failed: {0}")]
    ArchiveRead(String),

    /// Generic IO — mkdir / readdir at the mountpoint, etc.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl MountError {
    /// Stable Fluent-key suffix for end-user surface.
    /// `mount-error-<suffix>` is the full key the frontend
    /// resolves.
    pub fn fluent_suffix(&self) -> &'static str {
        match self {
            Self::UnsafeMountpoint(_) => "unsafe-mountpoint",
            Self::MountpointNotEmpty(_) => "mountpoint-not-empty",
            Self::BackendUnavailable(_) => "backend-unavailable",
            Self::ArchiveRead(_) => "archive-read",
            Self::Io(_) => "io",
        }
    }
}
