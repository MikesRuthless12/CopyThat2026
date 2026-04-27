//! Phase 19a — events emitted during a scan.
//!
//! Broadcast on an `mpsc::Sender<ScanEvent>` the caller hands to
//! [`crate::scan::Scanner::run`]. Dropped sends are tolerated; if the
//! receiver disappears the scanner keeps working and stops reporting.
//! Mirrors the tolerance rules of [`crate::event::CopyEvent`].

use std::path::PathBuf;
use std::time::Duration;

use crate::error::CopyError;
use crate::scan::types::ScanId;

/// Lifecycle + progress events. Marked `#[non_exhaustive]` so later
/// phases can add variants without breaking downstream `match` arms.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ScanEvent {
    /// Scan has been initialised (DB opened, schema applied, root
    /// path validated). Fires exactly once, before any `Progress`.
    Started {
        /// Identifier the scan was registered under.
        scan_id: ScanId,
        /// Source root being walked.
        root: PathBuf,
        /// Absolute path to the SQLite scan database.
        db_path: PathBuf,
    },
    /// Running totals. Fires every `ScanOptions::progress_every`
    /// files while the walker is pumping entries through the DB
    /// writer. `files_total` / `bytes_total` grow monotonically as
    /// the walker discovers more entries.
    Progress {
        /// Files visited by the walker so far.
        files_discovered: u64,
        /// Bytes summed across `File`-kind entries so far.
        bytes_discovered: u64,
        /// Rows committed to the SQLite DB so far.
        files_written: u64,
        /// Files for which the BLAKE3 worker has finalised a hash.
        files_hashed: u64,
    },
    /// One batch of rows has been committed to the scan DB. The scan
    /// is durable up to `files_committed` entries; a crash at this
    /// point loses only the in-flight mpsc buffer.
    BatchFlushed {
        /// Number of rows in the batch just committed.
        batch_size: u32,
        /// Cumulative count of rows committed since the scan started.
        files_committed: u64,
        /// Relative path of the last row in the batch (sentinel for
        /// crash-resume).
        last_rel_path: String,
    },
    /// Scan is paused. The walker and DB writer are blocked on
    /// `ScanControl::wait_while_paused`; no further `Progress` events
    /// fire until `Resumed`.
    Paused,
    /// Scan has resumed.
    Resumed,
    /// Scan finished normally. Final `files_discovered` /
    /// `bytes_discovered` are present in the attached stats.
    Completed {
        /// Total files visited.
        files: u64,
        /// Total bytes summed across `File`-kind entries.
        bytes: u64,
        /// Files that received a content hash.
        hashed_files: u64,
        /// Wall-clock duration end-to-end.
        duration: Duration,
    },
    /// Scan was cancelled via [`crate::scan::ScanControl::cancel`].
    /// The DB is left behind in `Cancelled` state — the caller can
    /// either resume (by re-running with the same `scan_id`) or
    /// delete.
    Cancelled,
    /// Scan failed. The DB is left behind in `Failed` state for
    /// inspection.
    Failed {
        /// Typed error detail.
        err: CopyError,
    },
}
