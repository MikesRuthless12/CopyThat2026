//! Cross-boundary types: what callers hand us on write, what we
//! hand back on read.
//!
//! Kept intentionally simple and mostly stringly-typed — `kind` /
//! `status` travel as short lowercase-kebab strings so callers can
//! map them to their own enums without a shared dependency. The
//! Tauri layer maps `JobKind` → `"copy"` / `"move"` / etc. already;
//! this crate treats those as opaque wire names.

use std::path::PathBuf;

/// Newtype on the rowid so callers can't accidentally swap it with
/// the queue's in-memory `JobId`. Both are `u64` under the hood but
/// they're different spaces — `JobId` lives for the process
/// lifetime, `JobRowId` is the persisted primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JobRowId(pub i64);

impl JobRowId {
    pub fn as_i64(self) -> i64 {
        self.0
    }
}

/// One row in the `jobs` table. Carries the full lifecycle payload;
/// `record_start` inserts it with `status = "running"` and
/// `finished_at_ms = None`, `record_finish` updates the same row.
#[derive(Debug, Clone)]
pub struct JobSummary {
    /// Populated after `record_start` returns. Zero on insert-path
    /// input.
    pub row_id: i64,
    /// `"copy"`, `"move"`, `"delete"`, `"secure-delete"`, `"verify"`.
    pub kind: String,
    /// `"running"`, `"succeeded"`, `"failed"`, `"cancelled"`.
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub src_root: PathBuf,
    pub dst_root: PathBuf,
    pub total_bytes: u64,
    pub files_ok: u64,
    pub files_failed: u64,
    /// `None` when verification was disabled; else e.g. `"sha256"`.
    pub verify_algo: Option<String>,
    /// Pre-serialised `CopyOptions` blob. Opaque JSON text; Phase 14
    /// will parse this to replay "scheduled jobs".
    pub options_json: Option<String>,
}

/// One row in the `items` table — covers both a successful per-file
/// `Completed` event and a `FileError` that the policy absorbed.
/// Status strings: `"ok"` / `"failed"` / `"skipped"` / `"cancelled"`.
#[derive(Debug, Clone)]
pub struct ItemRow {
    pub job_row_id: i64,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub size: u64,
    pub status: String,
    /// Lowercase hex of the verify hash if one ran. `None` otherwise.
    pub hash_hex: Option<String>,
    /// Kebab-case engine error-kind (`"permission-denied"`, …).
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
    /// When the item terminated. Usually close to the job's
    /// `finished_at_ms`; separate so we can render a per-item
    /// timeline in Phase 10.
    pub timestamp_ms: i64,
}

/// Search filter for `History::search`. `None` fields mean
/// "no filter on this dimension".
#[derive(Debug, Clone, Default)]
pub struct HistoryFilter {
    /// Inclusive lower bound on `started_at_ms`.
    pub started_since_ms: Option<i64>,
    /// Inclusive upper bound on `started_at_ms`.
    pub started_until_ms: Option<i64>,
    /// Kind filter (short wire name; wildcard when `None`).
    pub kind: Option<String>,
    /// Status filter.
    pub status: Option<String>,
    /// Case-insensitive substring match on `src_root` / `dst_root`.
    pub text: Option<String>,
    /// Max rows to return. `None` means "all" but the handler
    /// internally caps at [`DEFAULT_SEARCH_LIMIT`] to keep the IPC
    /// payload bounded.
    pub limit: Option<u32>,
}

/// Cap applied when a caller passes `HistoryFilter.limit = None`.
/// Phase 9 doesn't paginate; 5000 rows × the JSON DTO is ~1 MiB
/// which is comfortable for a single IPC round trip.
pub const DEFAULT_SEARCH_LIMIT: u32 = 5_000;
