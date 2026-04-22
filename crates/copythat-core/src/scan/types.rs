//! Phase 19a — shared scan data types.
//!
//! A scan produces [`ScanItem`] rows persisted to an on-disk SQLite
//! database. Nothing in this module touches the filesystem or the DB
//! directly; the types are the wire format between the enumerator
//! task, the DB-writer, the cursor reader, and the engine.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use uuid::Uuid;

/// Unique identifier for a scan. Generated with [`ScanId::new`] at
/// scan creation time and used as both the primary key in the
/// `main.db` registry (see [`crate::scan::index`]) and the filename
/// of the per-scan DB (`scan-<uuid>.db`).
///
/// The UUID v4 gives us collision-free ids without needing a
/// monotonic counter coordinated across app instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScanId(pub Uuid);

impl ScanId {
    /// Fresh UUID v4.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse from the hyphenated UUID form emitted by [`Self::as_str`].
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }

    /// Hyphenated UUID form (e.g. `"c1a2e3b4-..."`). Matches the
    /// default `Uuid::to_string` output.
    pub fn as_str(&self) -> String {
        self.0.hyphenated().to_string()
    }
}

impl Default for ScanId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ScanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}

/// Public counterpart of the crate-private `tree::EntryKind`. Scan
/// items surface every entry kind the walker sees; the copy engine
/// only actions `File` / `Symlink` (directories are re-created by the
/// tree dispatcher from their relative paths).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
    Other,
}

impl EntryKind {
    /// DB column encoding. See `scan_items.kind` in
    /// [`crate::scan::schema`].
    pub fn to_i64(self) -> i64 {
        match self {
            EntryKind::File => 0,
            EntryKind::Dir => 1,
            EntryKind::Symlink => 2,
            EntryKind::Other => 3,
        }
    }

    pub fn from_i64(v: i64) -> Self {
        match v {
            0 => EntryKind::File,
            1 => EntryKind::Dir,
            2 => EntryKind::Symlink,
            _ => EntryKind::Other,
        }
    }
}

/// Per-entry attribute bits. Bitmask persisted to the `scan_items.attrs`
/// INTEGER column so a reader can filter by "non-hidden, non-system
/// files only" without re-stat'ing the source.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AttrFlags(pub u32);

impl AttrFlags {
    pub const HIDDEN: u32 = 1 << 0;
    pub const SYSTEM: u32 = 1 << 1;
    pub const READONLY: u32 = 1 << 2;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn set(&mut self, bit: u32, on: bool) {
        if on {
            self.0 |= bit;
        } else {
            self.0 &= !bit;
        }
    }

    pub fn has(&self, bit: u32) -> bool {
        self.0 & bit != 0
    }

    pub fn to_i64(self) -> i64 {
        self.0 as i64
    }

    pub fn from_i64(v: i64) -> Self {
        Self(v as u32)
    }
}

/// One row of a scan. Serialized to `scan_items`; the cursor reader
/// (see [`crate::scan::cursor`]) hydrates rows back into this shape.
#[derive(Debug, Clone)]
pub struct ScanItem {
    /// Path relative to the scan root. Always present (the root
    /// itself is not emitted as a row).
    pub rel_path: String,
    /// File size in bytes. Dirs and symlinks record 0.
    pub size: u64,
    /// Last-modification time captured at scan time.
    pub mtime: SystemTime,
    pub kind: EntryKind,
    pub attrs: AttrFlags,
    /// BLAKE3-256 of the file contents when the scan was run with
    /// [`ScanOptions::hash_during_scan`]. `None` otherwise.
    pub content_hash: Option<[u8; 32]>,
}

/// Running counters surfaced via [`crate::scan::Scanner::stats`] and
/// the `scan_meta` totals row.
#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    pub total_files: u64,
    pub total_bytes: u64,
    /// Count per entry kind. Dir / Symlink / Other keys only appear
    /// when those kinds were actually observed.
    pub by_kind: HashMap<EntryKind, u64>,
}

/// Lifecycle state persisted as a `TEXT` value in `scan_meta.status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    Running,
    Paused,
    Complete,
    Cancelled,
    Failed,
}

impl ScanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ScanStatus::Running => "Running",
            ScanStatus::Paused => "Paused",
            ScanStatus::Complete => "Complete",
            ScanStatus::Cancelled => "Cancelled",
            ScanStatus::Failed => "Failed",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "Running" => ScanStatus::Running,
            "Paused" => ScanStatus::Paused,
            "Complete" => ScanStatus::Complete,
            "Cancelled" => ScanStatus::Cancelled,
            "Failed" => ScanStatus::Failed,
            _ => return None,
        })
    }
}

/// Behaviour knobs for a single scan. Cheap to clone.
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Destination directory for the scan DB file. When `None` the
    /// Scanner resolves the OS user-config dir
    /// (`ProjectDirs::from("com", "CopyThat", "CopyThat2026").config_dir()/scans/`).
    pub db_dir: Option<PathBuf>,
    /// Follow symlinks during enumeration. Mirrors
    /// `TreeOptions::follow_symlinks_in_tree` semantics.
    pub follow_symlinks: bool,
    /// Optional glob / size / date / attribute filters. Applied
    /// inline during enumeration; rejected files are omitted from
    /// `scan_items` entirely.
    pub filters: Option<crate::filter::FilterSet>,
    /// When true, spawn [`Self::hash_workers`] BLAKE3 workers that
    /// hash file contents in parallel with the walker; the final
    /// row has `content_hash` populated. Off by default — enabling
    /// roughly doubles scan time on HDD storage.
    pub hash_during_scan: bool,
    /// Number of parallel BLAKE3 hashers when
    /// [`Self::hash_during_scan`] is true. Clamped to `[1, 64]`.
    /// Defaults to `num_cpus()/2`, minimum 1.
    pub hash_workers: usize,
    /// Emit [`crate::scan::ScanEvent::Progress`] every N files.
    /// `0` disables progress pumping (the Completed event still
    /// fires). Default 500 — matches the tree walker's own
    /// `TreeEnumerating` cadence.
    pub progress_every: u32,
}

impl Default for ScanOptions {
    fn default() -> Self {
        let workers = (num_logical_cpus() / 2).max(1);
        Self {
            db_dir: None,
            follow_symlinks: false,
            filters: None,
            hash_during_scan: false,
            hash_workers: workers,
            progress_every: 500,
        }
    }
}

fn num_logical_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Final success record for a completed scan.
#[derive(Debug, Clone)]
pub struct ScanReport {
    pub scan_id: ScanId,
    pub db_path: PathBuf,
    pub root: PathBuf,
    pub stats: ScanStats,
    pub duration: Duration,
    pub hashed_files: u64,
}
