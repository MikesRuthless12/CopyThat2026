//! Public event type emitted by the watcher.
//!
//! Deliberately smaller surface than `notify::Event`: a rename is a
//! single `Renamed(from, to)` pair, a create is always a `Created(path)`,
//! etc. Downstream consumers (the Phase 25 sync engine, anything
//! plugging into live mirror) never see `notify`'s extended
//! `EventKind::Modify(ModifyKind::Metadata(Ownership))` variants — those
//! are either folded into `Modified` or dropped as noise, depending on
//! the kind.

use std::path::PathBuf;

/// A filesystem event the watcher has observed, post-filter and
/// post-debounce.
///
/// Every emitted path is absolute. `Renamed` carries both the source
/// and destination so a downstream sync engine can avoid re-hashing
/// both sides to figure out what happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEvent {
    /// A new file appeared at this path.
    Created(PathBuf),
    /// The contents (or metadata meaningful for sync — mtime, size,
    /// permissions) of this file changed. This is the catch-all
    /// variant — debounce collapses repeated writes into one
    /// `Modified`.
    Modified(PathBuf),
    /// The file at this path disappeared.
    Removed(PathBuf),
    /// The file at `0` was renamed to `1`. Both paths are absolute.
    /// For atomic-save-pattern renames (temp → final), the temp side
    /// never surfaces — the watcher emits `Modified(final)` instead.
    Renamed(PathBuf, PathBuf),
}

impl FsEvent {
    /// Canonical "subject" path for the event — the path the sync
    /// engine should reconcile against. For `Renamed`, returns the
    /// destination.
    pub fn subject(&self) -> &std::path::Path {
        match self {
            Self::Created(p) | Self::Modified(p) | Self::Removed(p) => p,
            Self::Renamed(_, to) => to,
        }
    }
}
