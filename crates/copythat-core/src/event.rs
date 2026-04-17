//! Events emitted during a copy / move, plus the final reports.
//!
//! `CopyEvent` covers both single-file (Phase 1) and tree / collision
//! (Phase 2) flows. Per-file `Started` / `Progress` / `Completed`
//! events continue to fire inside a tree copy; tree-level aggregates
//! are layered on top so a UI can paint both an overall progress bar
//! and a "current item" row without doing its own accounting.

use std::path::PathBuf;
use std::time::Duration;

use tokio::sync::oneshot;

use crate::error::CopyError;

/// A single event emitted on the `events` channel during a copy or
/// move. Dropped sends are tolerated: if the receiver disappears the
/// engine keeps working and stops reporting. Progress is advisory,
/// not load-bearing.
///
/// Marked `#[non_exhaustive]` so future phases can add variants
/// without breaking downstream `match` arms.
#[derive(Debug)]
#[non_exhaustive]
pub enum CopyEvent {
    // ---------- single file (Phase 1) ----------
    Started {
        src: PathBuf,
        dst: PathBuf,
        total_bytes: u64,
    },
    Progress {
        bytes: u64,
        total: u64,
        rate_bps: u64,
    },
    Paused,
    Resumed,
    Completed {
        bytes: u64,
        duration: Duration,
        rate_bps: u64,
    },
    Failed {
        err: CopyError,
    },
    // ---------- tree-level aggregates (Phase 2) ----------
    TreeStarted {
        root_src: PathBuf,
        root_dst: PathBuf,
        total_files: u64,
        total_bytes: u64,
    },
    TreeProgress {
        files_done: u64,
        files_total: u64,
        bytes_done: u64,
        bytes_total: u64,
        rate_bps: u64,
    },
    TreeCompleted {
        files: u64,
        bytes: u64,
        duration: Duration,
        rate_bps: u64,
    },
    // ---------- collision (Phase 2) ----------
    Collision(Collision),
}

impl Clone for CopyEvent {
    fn clone(&self) -> Self {
        match self {
            CopyEvent::Started {
                src,
                dst,
                total_bytes,
            } => CopyEvent::Started {
                src: src.clone(),
                dst: dst.clone(),
                total_bytes: *total_bytes,
            },
            CopyEvent::Progress {
                bytes,
                total,
                rate_bps,
            } => CopyEvent::Progress {
                bytes: *bytes,
                total: *total,
                rate_bps: *rate_bps,
            },
            CopyEvent::Paused => CopyEvent::Paused,
            CopyEvent::Resumed => CopyEvent::Resumed,
            CopyEvent::Completed {
                bytes,
                duration,
                rate_bps,
            } => CopyEvent::Completed {
                bytes: *bytes,
                duration: *duration,
                rate_bps: *rate_bps,
            },
            CopyEvent::Failed { err } => CopyEvent::Failed { err: err.clone() },
            CopyEvent::TreeStarted {
                root_src,
                root_dst,
                total_files,
                total_bytes,
            } => CopyEvent::TreeStarted {
                root_src: root_src.clone(),
                root_dst: root_dst.clone(),
                total_files: *total_files,
                total_bytes: *total_bytes,
            },
            CopyEvent::TreeProgress {
                files_done,
                files_total,
                bytes_done,
                bytes_total,
                rate_bps,
            } => CopyEvent::TreeProgress {
                files_done: *files_done,
                files_total: *files_total,
                bytes_done: *bytes_done,
                bytes_total: *bytes_total,
                rate_bps: *rate_bps,
            },
            CopyEvent::TreeCompleted {
                files,
                bytes,
                duration,
                rate_bps,
            } => CopyEvent::TreeCompleted {
                files: *files,
                bytes: *bytes,
                duration: *duration,
                rate_bps: *rate_bps,
            },
            // Collision carries a oneshot sender; it can't be cloned.
            // Broadcast subscribers only see a placeholder.
            CopyEvent::Collision(_) => CopyEvent::Collision(Collision::placeholder_for_clone()),
        }
    }
}

/// Final success record returned by `copy_file` and `move_file`.
#[derive(Debug, Clone)]
pub struct CopyReport {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
}

/// Final success record returned by `copy_tree` and `move_tree`.
#[derive(Debug, Clone)]
pub struct TreeReport {
    pub root_src: PathBuf,
    pub root_dst: PathBuf,
    pub files: u64,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
    /// Files the caller asked us to skip (via collision policy).
    pub skipped: u64,
}

/// Destination-already-exists prompt. Consumers reply on the enclosed
/// oneshot to resolve. If the sender is dropped without replying, the
/// engine treats the collision as a Skip.
#[derive(Debug)]
pub struct Collision {
    pub src: PathBuf,
    pub dst: PathBuf,
    /// Reply channel. `None` only on cloned placeholders — cloned
    /// events can't drive the engine forward, so a subscriber that
    /// saw the clone must also be attached to the original mpsc to
    /// actually resolve the collision.
    pub resolver: Option<oneshot::Sender<CollisionResolution>>,
}

impl Collision {
    pub(crate) fn new(
        src: PathBuf,
        dst: PathBuf,
        resolver: oneshot::Sender<CollisionResolution>,
    ) -> Self {
        Self {
            src,
            dst,
            resolver: Some(resolver),
        }
    }

    fn placeholder_for_clone() -> Self {
        Self {
            src: PathBuf::new(),
            dst: PathBuf::new(),
            resolver: None,
        }
    }

    /// Resolve the collision. Consumes the oneshot. No-op on a cloned
    /// placeholder.
    pub fn resolve(mut self, resolution: CollisionResolution) {
        if let Some(tx) = self.resolver.take() {
            let _ = tx.send(resolution);
        }
    }
}

/// Decision returned by the collision prompter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollisionResolution {
    Skip,
    Overwrite,
    /// Use this final filename instead (no directory component; stays
    /// in the same parent as the original destination).
    Rename(String),
    /// Abort the whole tree operation.
    Abort,
}
