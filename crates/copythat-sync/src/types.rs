//! Public types: pair / options / side state / action / events /
//! report.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// A configured sync relationship between two directories on disk.
///
/// `left` / `right` are local paths. `db_path` defaults to
/// `<left>/.copythat-sync.db` but the spec also allows an
/// out-of-tree path for test fixtures — keep it explicit on the
/// struct so callers can override cleanly.
#[derive(Debug, Clone)]
pub struct SyncPair {
    pub left: PathBuf,
    pub right: PathBuf,
    pub db_path: PathBuf,
    /// Stable human label — "Documents ↔ NAS", "Photos ↔ Backup",
    /// etc. Used as the row header in the UI and as the history
    /// entry's source description. Not required by the algorithm.
    pub label: String,
    /// Optional override for the hostname portion of conflict-file
    /// suffixes. Defaults to `hostname::get()` at engine startup, but
    /// the smoke test wants a deterministic fixture, and cloud /
    /// container deployments may want to substitute a friendly
    /// identifier.
    pub host_label: Option<String>,
}

impl SyncPair {
    /// Build a pair with the default `.copythat-sync.db` path under
    /// the left root.
    pub fn new<L: AsRef<Path>, R: AsRef<Path>>(
        label: impl Into<String>,
        left: L,
        right: R,
    ) -> Self {
        let left = left.as_ref().to_path_buf();
        let right = right.as_ref().to_path_buf();
        let db_path = left.join(".copythat-sync.db");
        Self {
            left,
            right,
            db_path,
            label: label.into(),
            host_label: None,
        }
    }

    /// Override the persisted DB location (for fixtures / shared
    /// network-storage pairs).
    pub fn with_db_path(mut self, db_path: impl Into<PathBuf>) -> Self {
        self.db_path = db_path.into();
        self
    }

    /// Override the hostname used in conflict-file suffixes.
    pub fn with_host_label(mut self, label: impl Into<String>) -> Self {
        self.host_label = Some(label.into());
        self
    }
}

/// Direction of propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

impl Direction {
    pub fn flip(self) -> Self {
        match self {
            Self::LeftToRight => Self::RightToLeft,
            Self::RightToLeft => Self::LeftToRight,
        }
    }
}

/// Sync direction policy for the pair.
///
/// `TwoWay` is the main mode. `MirrorLeftToRight` deletes extras on
/// right; `MirrorRightToLeft` is the mirror. `ContributeLeftToRight`
/// is a one-way "upload" that never deletes anything on the receiver
/// — it's there for backup-style use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SyncMode {
    #[default]
    TwoWay,
    MirrorLeftToRight,
    MirrorRightToLeft,
    ContributeLeftToRight,
}

/// Behaviour knobs for a single `sync()` run.
#[derive(Debug, Clone)]
pub struct SyncOptions {
    /// If `true`, compute the plan but don't touch either side.
    /// Useful for the UI's "Preview changes" button. Default off.
    pub dry_run: bool,
    /// Bound the event channel the caller passes in — a runaway
    /// caller that stopped draining the receiver should not panic
    /// the engine. The engine uses this hint to pre-size its internal
    /// buffers; it does not resize the caller's channel.
    pub event_buffer_hint: usize,
    /// Forwarded to [`copythat_core::CopyOptions`] as the per-file
    /// buffer size.
    pub copy_buffer_size: usize,
    /// Forwarded to [`copythat_core::CopyOptions::fsync_on_close`].
    /// Default on — we want sync runs to be durable end-to-end.
    pub fsync_on_close: bool,
    /// Plug-point for the Tauri runner: when present, every copy the
    /// engine issues gets this hook attached so fast paths / verify
    /// / sparseness / security metadata all carry through.
    pub copy_hook: Option<Arc<dyn CopyHookFactory>>,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            event_buffer_hint: 256,
            copy_buffer_size: copythat_core::DEFAULT_BUFFER_SIZE,
            fsync_on_close: true,
            copy_hook: None,
        }
    }
}

/// Factory for per-file `CopyOptions`. The runner attaches platform
/// hooks (fast-copy, sparse, meta) via this indirection so the sync
/// engine doesn't need to know about them directly. Defaults to a
/// no-op factory that returns `CopyOptions::default()`.
pub trait CopyHookFactory: Send + Sync + std::fmt::Debug {
    /// Called once per file the engine is about to copy. Returns the
    /// fully-configured options struct.
    fn build(&self) -> copythat_core::CopyOptions;
}

/// Per-file metadata captured during the walk pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMeta {
    pub mtime_ms: i64,
    pub size: u64,
    pub blake3: [u8; 32],
}

/// What one side looks like for a given relpath at the start of a
/// sync round. `meta` is `None` when the file is absent from that
/// side.
#[derive(Debug, Clone)]
pub struct SideState {
    pub relpath: String,
    pub meta: Option<FileMeta>,
}

/// The engine's per-file decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncAction {
    /// Both sides agree with the baseline — nothing to do.
    Noop { relpath: String },
    /// Propagate content in one direction.
    Copy {
        relpath: String,
        direction: Direction,
    },
    /// Propagate an absence in one direction — file is gone on the
    /// source side and unchanged on the destination since last sync.
    Delete {
        relpath: String,
        direction: Direction,
    },
    /// Conflict: preserve the losing side's content locally with a
    /// `sync-conflict-*` suffix and propagate the winner's content
    /// to the loser's canonical path. The `winner` field is the side
    /// that sourced the propagated content; the `loser` side is
    /// where the conflict file lives after the run.
    KeepConflict {
        relpath: String,
        winner: Direction,
        loser: Direction,
        kind: ConflictKind,
    },
}

impl SyncAction {
    pub fn relpath(&self) -> &str {
        match self {
            Self::Noop { relpath }
            | Self::Copy { relpath, .. }
            | Self::Delete { relpath, .. }
            | Self::KeepConflict { relpath, .. } => relpath,
        }
    }
}

/// What kind of conflict the matrix detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictKind {
    /// Both sides modified the same relpath from a common ancestor.
    ConcurrentWrite,
    /// One side deleted, the other modified.
    DeleteEdit,
    /// Both sides added a new file at the same relpath with
    /// different content.
    AddAdd,
    /// Vector clocks compare equal but content differs — one side is
    /// corrupt or was edited behind sync's back.
    CorruptEqual,
}

/// A conflict entry included in the [`SyncReport`].
#[derive(Debug, Clone)]
pub struct Conflict {
    pub relpath: String,
    pub kind: ConflictKind,
    pub winner: Direction,
    pub loser: Direction,
    /// Absolute path of the conflict-preservation file written on
    /// the losing side. The UI links to this so the user can open
    /// it and inspect before resolving.
    pub loser_preservation_path: PathBuf,
}

/// Outcome of one `sync()` call.
#[derive(Debug, Clone, Default)]
pub struct SyncReport {
    pub applied_left: usize,
    pub applied_right: usize,
    pub deleted_left: usize,
    pub deleted_right: usize,
    pub conflicts: Vec<Conflict>,
    /// Walltime of the sync call in milliseconds. Populated by the
    /// engine at return.
    pub elapsed_ms: u64,
    /// Set to `true` when the caller cancelled the run mid-stream
    /// (see [`crate::SyncControl::cancel`]).
    pub cancelled: bool,
}

/// Progress + decision events emitted as `sync()` runs.
///
/// The caller threads these through an `mpsc::Sender<SyncEvent>` they
/// own — same pattern as `copythat_core::CopyEvent`.
#[derive(Debug, Clone)]
pub enum SyncEvent {
    /// Walk pass is starting for one side. `side` identifies which.
    WalkStarted { side: Direction },
    /// Progress during the walk — `scanned` is the running count of
    /// entries (files + directories) visited.
    WalkProgress { side: Direction, scanned: u64 },
    /// Walk pass finished. `files_total` is the number of regular
    /// files recorded.
    WalkCompleted { side: Direction, files_total: u64 },
    /// The engine decided an action for `relpath`.
    ActionPlanned { action: SyncAction },
    /// The engine started executing an action.
    ActionStarted { action: SyncAction },
    /// The engine finished executing an action without error.
    ActionCompleted { action: SyncAction },
    /// A conflict was detected + preserved. One of these fires per
    /// conflict, alongside the corresponding `ActionCompleted` for
    /// the `KeepConflict` action.
    Conflict(Conflict),
    /// The sync run finished. Carries the full [`SyncReport`].
    Finished { report: SyncReport },
}
