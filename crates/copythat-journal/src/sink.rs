//! `JournalSink` implementation that the engine attaches to
//! `CopyOptions::journal`.
//!
//! **Phase 39: in-memory batching.** Earlier revisions called
//! `Journal::checkpoint` on every engine emit, which fsync'd a redb
//! transaction every ~50 ms. On a 16-second 10 GiB UI copy that's
//! 320 fsyncs — measured as ~10 seconds of GUI-visible overhead vs
//! the headless engine. We now keep the latest checkpoint for each
//! in-flight `file_idx` in a `Mutex<HashMap>` and write through at
//! most once per `FLUSH_INTERVAL` (default 1 second), or whenever
//! `finish_file` / `finish_job_*` lands (those still flush
//! immediately so terminal state is durable). The user can lose at
//! most 1 second of in-progress checkpoint detail on a crash; the
//! resume planner already tolerates "partial dst, no checkpoint"
//! by restarting that file from offset 0.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use copythat_core::{JournalSink, ResumePlan as CoreResumePlan};

use crate::journal::Journal;
use crate::types::{JobRowId, JobStatus, ResumePlan as JournalResumePlan};

/// How long between forced flushes of the pending-checkpoint map.
/// 1 second is the resume-granularity ceiling we're willing to
/// trade for ~95 % fewer fsyncs on hot copies.
const FLUSH_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Debug, Clone)]
struct PendingCheckpoint {
    dst: PathBuf,
    bytes_done: u64,
    expected_total: u64,
    hash_so_far: [u8; 32],
}

#[derive(Debug)]
struct BatchState {
    pending: HashMap<u64, PendingCheckpoint>,
    /// File indices that have already had a checkpoint hit disk.
    /// First-checkpoint-per-file always flushes through so the
    /// resume planner sees an initial record on a crash, even when
    /// the FLUSH_INTERVAL hasn't elapsed.
    seen_on_disk: HashSet<u64>,
    last_flush: Instant,
}

impl Default for BatchState {
    fn default() -> Self {
        Self {
            pending: HashMap::new(),
            seen_on_disk: HashSet::new(),
            last_flush: Instant::now(),
        }
    }
}

/// Engine-ready bridge. Holds the `JobRowId` so the engine can
/// checkpoint without having to know about journal IDs at all.
#[derive(Debug)]
pub struct CopyThatJournalSink {
    journal: Journal,
    job: JobRowId,
    state: Mutex<BatchState>,
}

impl Clone for CopyThatJournalSink {
    fn clone(&self) -> Self {
        // Each clone gets a fresh batch state — the journal handle
        // itself dedups concurrent transactions, so the batches
        // don't need to be shared. Rare: cloning during a job is
        // not a normal path; the runner builds one sink per job.
        Self {
            journal: self.journal.clone(),
            job: self.job,
            state: Mutex::new(BatchState::default()),
        }
    }
}

impl CopyThatJournalSink {
    pub fn new(journal: Journal, job: JobRowId) -> Self {
        Self {
            journal,
            job,
            state: Mutex::new(BatchState::default()),
        }
    }

    /// Drain everything pending for `file_idx` and write through to
    /// the redb-backed journal in a single transaction. Used by
    /// `finish_file` (terminal state for that file) and the
    /// time-based flush path. Failures are best-effort — the engine
    /// owns the actual copy outcome.
    fn flush_pending_for(&self, file_idx: u64) {
        let pending = {
            let mut s = self.state.lock().expect("journal sink mutex");
            s.pending.remove(&file_idx)
        };
        if let Some(p) = pending {
            let _ = self.journal.checkpoint(
                self.job,
                file_idx,
                &p.dst,
                p.bytes_done,
                p.expected_total,
                p.hash_so_far,
            );
        }
    }

    /// Drain the entire pending map. Called by every
    /// `finish_job_*` so a terminal job has every in-flight file
    /// represented before the job-row flips to terminal.
    fn flush_all(&self) {
        let drained: Vec<(u64, PendingCheckpoint)> = {
            let mut s = self.state.lock().expect("journal sink mutex");
            s.pending.drain().collect()
        };
        for (file_idx, p) in drained {
            let _ = self.journal.checkpoint(
                self.job,
                file_idx,
                &p.dst,
                p.bytes_done,
                p.expected_total,
                p.hash_so_far,
            );
        }
    }
}

impl JournalSink for CopyThatJournalSink {
    fn checkpoint(
        &self,
        file_idx: u64,
        dst: &Path,
        bytes_done: u64,
        expected_total: u64,
        hash_so_far: [u8; 32],
    ) {
        // Stash the latest values in memory; flush immediately when
        // either:
        // - This file_idx has never hit disk (resume planner needs
        //   to see at least the "started" record on a crash), or
        // - At least FLUSH_INTERVAL has elapsed since the last
        //   write-through.
        // Otherwise just keep the in-memory pending entry up to
        // date and let the next time-based flush absorb it.
        let to_flush: Option<Vec<(u64, PendingCheckpoint)>> = {
            let mut s = self.state.lock().expect("journal sink mutex");
            let pending_entry = PendingCheckpoint {
                dst: dst.to_path_buf(),
                bytes_done,
                expected_total,
                hash_so_far,
            };
            s.pending.insert(file_idx, pending_entry);
            let force_first = !s.seen_on_disk.contains(&file_idx);
            if force_first || s.last_flush.elapsed() >= FLUSH_INTERVAL {
                let drained: Vec<(u64, PendingCheckpoint)> = s.pending.drain().collect();
                for (idx, _) in &drained {
                    s.seen_on_disk.insert(*idx);
                }
                s.last_flush = Instant::now();
                Some(drained)
            } else {
                None
            }
        };
        if let Some(drained) = to_flush {
            for (idx, p) in drained {
                let _ = self.journal.checkpoint(
                    self.job,
                    idx,
                    &p.dst,
                    p.bytes_done,
                    p.expected_total,
                    p.hash_so_far,
                );
            }
        }
    }

    fn finish_file(&self, file_idx: u64, final_hash: [u8; 32]) {
        // Terminal per-file state: drain any pending in-flight
        // checkpoint for this file_idx first so resume sees the
        // last-known offset, then write the finish marker.
        self.flush_pending_for(file_idx);
        let _ = self.journal.finish_file(self.job, file_idx, final_hash);
    }

    fn resume_plan(&self, file_idx: u64) -> CoreResumePlan {
        // resume_plan reads from the on-disk state; flush any
        // pending checkpoint for this file_idx first so a "resume
        // mid-flight" call sees the latest in-memory progress
        // (rare, but the planner shouldn't see stale data).
        self.flush_pending_for(file_idx);
        match self.journal.resume_plan(self.job, file_idx) {
            Ok(JournalResumePlan::Resume {
                offset,
                src_hash_at_offset,
            }) => CoreResumePlan::Resume {
                offset,
                src_hash_at_offset,
            },
            Ok(JournalResumePlan::Restart) => CoreResumePlan::Restart,
            Ok(JournalResumePlan::AlreadyComplete { final_hash }) => {
                CoreResumePlan::AlreadyComplete { final_hash }
            }
            // Read errors are surfaced as Restart — safer to start
            // over than to chase a corrupted on-disk state. The
            // engine will create a fresh dst and the user will see
            // the bytes copy.
            Err(_) => CoreResumePlan::Restart,
        }
    }

    fn finish_job_succeeded(&self) {
        self.flush_all();
        let _ = self.journal.finish_job(self.job, JobStatus::Succeeded);
    }

    fn finish_job_failed(&self) {
        self.flush_all();
        let _ = self.journal.finish_job(self.job, JobStatus::Failed);
    }

    fn finish_job_cancelled(&self) {
        self.flush_all();
        let _ = self.journal.finish_job(self.job, JobStatus::Cancelled);
    }
}
