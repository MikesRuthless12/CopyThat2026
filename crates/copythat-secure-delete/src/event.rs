//! Events emitted by the shredder, plus the final report.
//!
//! Shape mirrors `copythat_core::CopyEvent` so a UI can route copy,
//! verify, and shred jobs through the same progress widget without
//! special casing.

use std::path::PathBuf;
use std::time::Duration;

use crate::error::ShredError;
use crate::method::ShredMethod;
use crate::sanitize::SsdSanitizeMode;

/// A single event emitted on the `events` channel during a shred.
///
/// Dropped sends are tolerated: if the receiver disappears the engine
/// keeps working and stops reporting. Progress is advisory — if your
/// caller's UI misses a tick the pass still finishes.
///
/// `#[non_exhaustive]` so future phases can add variants without
/// breaking downstream match arms.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ShredEvent {
    /// Emitted once per operation, before the first pass starts.
    Started {
        path: PathBuf,
        method: ShredMethod,
        total_passes: usize,
        /// File size in bytes at the moment the shred started. Each pass
        /// writes this many bytes (except for any zero-length files).
        file_size: u64,
    },
    /// Advisory-only notice that the target lives on an SSD. Fluent
    /// key is stable so the caller can localize the message.
    SsdAdvisory {
        path: PathBuf,
        /// Fluent message key. Current value: `shred-ssd-advisory`.
        localized_key: &'static str,
    },
    /// A pass is starting.
    PassStarted {
        /// 1-based index of the pass (1..=total_passes).
        pass_index: usize,
        total_passes: usize,
        /// Stable short name of the pattern kind:
        /// `zero` / `one` / `random` / `tiled` / `fixed-0xNN`.
        pattern: String,
    },
    /// Progress within a pass. Throttled to at most one per 16 KiB
    /// and per 50 ms, same rhythm as the copy engine.
    PassProgress {
        pass_index: usize,
        total_passes: usize,
        bytes: u64,
        total: u64,
        rate_bps: u64,
    },
    /// A pass finished. If the pass had `verify`, this event fires
    /// after the re-read has succeeded.
    PassCompleted {
        pass_index: usize,
        total_passes: usize,
        bytes: u64,
        duration: Duration,
        verified: bool,
    },
    /// Advisory: the per-pass `sync_all` (durable flush to medium)
    /// failed for `pass_index`. Emitted *before* the engine returns
    /// the corresponding hard failure, so a UI can distinguish "drive
    /// firmware refused fsync" from a generic write error. The shred
    /// operation will fail; this event exists so the user sees that
    /// the pass bytes were never confirmed durable.
    PassFlushFailed {
        pass_index: usize,
        total_passes: usize,
        /// String description of the underlying I/O error. Stable
        /// across releases as a human-readable message; do not parse.
        error: String,
    },
    /// Advisory: the SSD probe could not determine whether the target
    /// lives on solid-state or spinning media. Distinct from the
    /// `Some(false)` "definitely HDD" answer (no event) and from
    /// `Some(true)` (`SsdAdvisory`). UI should render
    /// "could not determine media type; SSD advisory not applicable".
    SsdAdvisoryUnknown {
        path: PathBuf,
        /// Fluent message key. Current value: `shred-ssd-advisory-unknown`.
        localized_key: &'static str,
    },
    /// The caller pressed pause. Mirrors `CopyEvent::Paused`.
    Paused,
    /// The caller resumed. Mirrors `CopyEvent::Resumed`.
    Resumed,
    /// The file has been unlinked after the final pass.
    Completed {
        path: PathBuf,
        method: ShredMethod,
        passes: usize,
        bytes_per_pass: u64,
        duration: Duration,
    },
    /// Tree mode: the shredder began walking a directory.
    TreeStarted {
        root: PathBuf,
        total_files: u64,
        total_bytes: u64,
    },
    /// Tree mode: aggregate progress across the tree. `files_done`
    /// counts files whose shred completed; `bytes_done` counts the
    /// total shredded bytes across all passes of all finished files.
    TreeProgress {
        files_done: u64,
        files_total: u64,
        bytes_done: u64,
        bytes_total: u64,
    },
    /// Tree mode: the whole tree finished (directories removed too).
    TreeCompleted {
        root: PathBuf,
        files: u64,
        bytes: u64,
        duration: Duration,
    },
    Failed {
        err: ShredError,
    },
    /// Phase 44 — whole-drive sanitize started. Fires before the
    /// privileged helper is invoked. Pair with `SanitizeCompleted`
    /// or `Failed`.
    SanitizeStarted {
        device: PathBuf,
        mode: SsdSanitizeMode,
    },
    /// Phase 44 — whole-drive sanitize finished successfully.
    /// `mode` reflects the actual mode that ran, which may differ
    /// from the requested mode when the helper falls back.
    SanitizeCompleted {
        device: PathBuf,
        mode: SsdSanitizeMode,
        duration: Duration,
    },
    /// Phase 44.1 — mid-sanitize progress percent (0-100) for
    /// long-running modes (NVMe Sanitize Block can take tens of
    /// minutes). Helpers that support polling (Linux:
    /// `nvme sanitize-log`, NVM Express §5.24.1 SPROG; Windows:
    /// IOCTL_STORAGE_REINITIALIZE_MEDIA progress reads) emit these
    /// at ~1 Hz. Helpers without polling support never emit; the
    /// UI sees Started → Completed only.
    SanitizeProgress {
        device: PathBuf,
        mode: SsdSanitizeMode,
        /// 0–100. The helper clamps to this range; a `100` value
        /// is advisory — the authoritative completion signal is
        /// `SanitizeCompleted`.
        percent: u8,
    },
}

/// Final success record returned by `shred_file` and `shred_tree`.
#[derive(Debug, Clone)]
pub struct ShredReport {
    /// The path that was shredded. In tree mode this is the root.
    pub path: PathBuf,
    pub method: ShredMethod,
    /// Number of passes executed per file.
    pub passes: usize,
    /// For `shred_file`: file size in bytes (one pass worth).
    /// For `shred_tree`: total bytes across all files × all passes.
    pub bytes_per_pass: u64,
    /// For `shred_tree` only; always 1 for `shred_file`.
    pub files: u64,
    pub duration: Duration,
}
