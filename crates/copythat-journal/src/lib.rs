//! `copythat-journal` — durable resume journal.
//!
//! When `copy_file` checkpoints every 50 ms with the running BLAKE3
//! hash and the relative byte offset, a power-cut at 96% of a 2 TB
//! transfer no longer means starting over. On the next launch
//! `Journal::unfinished()` surfaces every job that was running when
//! the process died; `Journal::resume_plan()` decides per file
//! whether to seek-and-continue (prefix-hash matches), restart from
//! zero (mismatch), or skip-as-already-done (size + final hash both
//! match).
//!
//! # Storage
//!
//! Single redb file at `<data-dir>/copythat-journal.redb`, sitting
//! next to `history.db`. Tables:
//!
//! - `jobs`: row-id-keyed JSON of [`JobRecord`] (kind / src / dst /
//!   status / per-file totals).
//! - `files`: keyed by `(JobRowId, file_idx)` — the latest
//!   checkpoint for each file. Updated in place as the engine
//!   progresses; finished files are tagged `Finished` and the final
//!   BLAKE3 digest is captured.
//! - `seq`: a single-row monotonic counter so `JobRowId` allocations
//!   stay unique across process restarts.
//!
//! All writes are wrapped in a redb `WriteTransaction`, which is
//! ACID — a torn write at the FS level cannot leave the journal in a
//! half-committed state. The journal is fsync'd on every commit
//! (redb's default), so a power-cut at the wrong instant loses at
//! most the work since the last checkpoint, not the journal itself.
//!
//! # Engine wiring
//!
//! [`CopyThatJournalSink`] is an `Arc<dyn JournalSink>` from
//! `copythat-core` that the engine drops into
//! `CopyOptions::journal`. The engine then calls
//! `JournalSink::checkpoint` every `PROGRESS_MIN_INTERVAL` (50 ms),
//! and `Journal` collapses the rapid stream of updates into one
//! durable write per tick.
//!
//! # Tauri integration
//!
//! On app start, `Journal::open_default().unfinished()` populates
//! `AppState::startup_unfinished`. The frontend opens the
//! `ResumePromptModal` with one row per unfinished job. The user
//! resumes (re-enqueues with `JournalSink` already attached) or
//! discards (`Journal::finish_job(Cancelled)` to clear the entry).

#![forbid(unsafe_code)]

mod error;
mod journal;
mod schema;
mod sink;
mod types;

pub use error::{JournalError, Result};
pub use journal::{Journal, default_journal_path};
pub use sink::CopyThatJournalSink;
pub use types::{
    CheckpointId, FileCheckpoint, FileStatus, JobRecord, JobRowId, JobStatus, ResumePlan,
    UnfinishedJob,
};
