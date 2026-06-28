//! Final record returned by [`crate::fast_copy`].

use std::time::Duration;

/// Identifies which acceleration path actually moved the bytes.
///
/// Tests assert against this to confirm the dispatcher reached the
/// expected syscall on each OS (e.g. `CopyFileExW` on Windows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChosenStrategy {
    /// `reflink-copy`: instant block-clone (Linux FICLONE / macOS
    /// `clonefile` / Windows `FSCTL_DUPLICATE_EXTENTS_TO_FILE`).
    Reflink,
    /// Windows `CopyFileExW`.
    CopyFileExW,
    /// macOS `copyfile(3)` with `COPYFILE_ALL`.
    Copyfile,
    /// Linux `copy_file_range(2)`.
    CopyFileRange,
    /// Linux `sendfile(2)` (used as fallback when `copy_file_range`
    /// returns `EXDEV` / `EINVAL` and the source is <2 GiB).
    Sendfile,
    /// Windows Phase 13c parallel multi-chunk copy — N concurrent
    /// offset streams (expert/manual override, or RAID / SMB / iSCSI /
    /// file-backed-virtual auto-gated).
    ParallelChunks,
    /// The Phase 1 [`copythat_core::copy_file`] async loop.
    AsyncFallback,
}

impl ChosenStrategy {
    /// Short label suitable for logs and the smoke-test stdout banner.
    pub fn label(self) -> &'static str {
        match self {
            ChosenStrategy::Reflink => "reflink",
            ChosenStrategy::CopyFileExW => "CopyFileExW",
            ChosenStrategy::Copyfile => "copyfile",
            ChosenStrategy::CopyFileRange => "copy_file_range",
            ChosenStrategy::Sendfile => "sendfile",
            ChosenStrategy::ParallelChunks => "parallel-chunks",
            ChosenStrategy::AsyncFallback => "async-fallback",
        }
    }
}

/// Final success record returned by [`crate::fast_copy`].
#[derive(Debug, Clone)]
pub struct FastCopyOutcome {
    /// Which acceleration path actually moved the bytes.
    pub strategy: ChosenStrategy,
    /// Total bytes copied (== source size on success).
    pub bytes: u64,
    /// Wall-clock duration end-to-end.
    pub duration: Duration,
    /// Average throughput across the copy, bytes per second.
    pub rate_bps: u64,
}
