//! Typed failure surface.

use std::path::PathBuf;

use thiserror::Error;

use crate::SnapshotKind;

/// Why a snapshot request failed.
///
/// Variants correspond 1-to-1 with the retry decisions the engine
/// makes: `Unsupported` falls through to the next `LockedFilePolicy`,
/// `NeedsElevation` / `UacDenied` bubbles back to the UI for a toast,
/// `BackendFailure` is the generic "tried and it didn't work" case.
/// Each variant carries a stable Fluent key via [`Self::localized_key`]
/// so the UI renders a translated message without branching on the
/// variant itself.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SnapshotError {
    /// The filesystem at the requested path has no snapshot primitive
    /// this crate knows how to drive. Engine falls through to the
    /// caller's next policy (Retry / Skip / Ask).
    #[error("filesystem at {path} has no supported snapshot backend")]
    Unsupported { path: PathBuf },

    /// VSS-only: the current process is not running as Administrator,
    /// and the caller asked for an in-process (non-helper) snapshot.
    /// Engine retries via the elevated helper binary automatically;
    /// this variant is only surfaced to the UI when the helper itself
    /// can't be spawned (below).
    #[error("VSS requires Administrator privilege")]
    NeedsElevation,

    /// The user dismissed the UAC prompt when the helper was spawned.
    /// Engine treats this as a hard failure for the current file —
    /// there's no second chance within the same policy decision.
    #[error("user cancelled the UAC prompt for the VSS helper")]
    UacDenied,

    /// Required backend tool is not installed or not on `$PATH`.
    /// Examples: `zfs` binary missing on a FreeBSD machine where the
    /// ZFS kernel module is unloaded; `tmutil` missing on a macOS
    /// build that stripped the system tools.
    #[error("snapshot backend tool missing: {tool}")]
    BackendMissing { tool: &'static str },

    /// The backend tool ran but returned a non-zero exit / a RPC error.
    /// `message` carries the backend's stderr / error code for the UI
    /// to surface (and the error log to capture) verbatim.
    #[error("snapshot backend {kind:?} failed: {message}")]
    BackendFailure { kind: SnapshotKind, message: String },

    /// I/O or process-spawn failure before the backend even ran.
    #[error("I/O error during snapshot setup: {0}")]
    Io(#[from] std::io::Error),

    /// JSON-RPC protocol error talking to the VSS helper.
    #[error("VSS helper protocol error: {0}")]
    Protocol(String),
}

impl SnapshotError {
    /// Fluent key the UI renders. Kept stable across phases so the
    /// locale files keep working without engine edits.
    pub const fn localized_key(&self) -> &'static str {
        match self {
            Self::Unsupported { .. } => "snapshot-create-failed",
            Self::NeedsElevation => "snapshot-vss-needs-elevation",
            Self::UacDenied => "snapshot-vss-needs-elevation",
            Self::BackendMissing { .. } => "snapshot-create-failed",
            Self::BackendFailure { .. } => "snapshot-create-failed",
            Self::Io(_) => "snapshot-create-failed",
            Self::Protocol(_) => "snapshot-create-failed",
        }
    }
}
