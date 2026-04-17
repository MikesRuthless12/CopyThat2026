//! Events emitted during a copy, plus the final report.

use std::path::PathBuf;
use std::time::Duration;

use crate::error::CopyError;

/// A single event emitted on the `events` channel during a copy.
///
/// Dropped sends are tolerated: if the receiver disappears (e.g. the
/// caller unplugged the UI), the engine keeps copying and just stops
/// reporting. Progress is advisory, not load-bearing.
#[derive(Debug, Clone)]
pub enum CopyEvent {
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
}

/// Final success record returned by `copy_file`.
#[derive(Debug, Clone)]
pub struct CopyReport {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
}
