//! `JournalSink` implementation that the engine attaches to
//! `CopyOptions::journal`.
//!
//! Stateless wrapper around a `Journal` clone — the runner builds
//! one per job at enqueue time so the engine has the right
//! `JobRowId` baked in.

use std::path::Path;

use copythat_core::{JournalSink, ResumePlan as CoreResumePlan};

use crate::journal::Journal;
use crate::types::{JobRowId, JobStatus, ResumePlan as JournalResumePlan};

/// Engine-ready bridge. Holds the `JobRowId` so the engine can
/// checkpoint without having to know about journal IDs at all.
#[derive(Debug, Clone)]
pub struct CopyThatJournalSink {
    journal: Journal,
    job: JobRowId,
}

impl CopyThatJournalSink {
    pub fn new(journal: Journal, job: JobRowId) -> Self {
        Self { journal, job }
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
        // Best-effort — checkpoint failures should never abort a copy.
        // The engine emits to its own logging in the failure path; we
        // just swallow on success-or-fail to avoid coupling.
        let _ = self.journal.checkpoint(
            self.job,
            file_idx,
            dst,
            bytes_done,
            expected_total,
            hash_so_far,
        );
    }

    fn finish_file(&self, file_idx: u64, final_hash: [u8; 32]) {
        let _ = self.journal.finish_file(self.job, file_idx, final_hash);
    }

    fn resume_plan(&self, file_idx: u64) -> CoreResumePlan {
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
        let _ = self.journal.finish_job(self.job, JobStatus::Succeeded);
    }

    fn finish_job_failed(&self) {
        let _ = self.journal.finish_job(self.job, JobStatus::Failed);
    }

    fn finish_job_cancelled(&self) {
        let _ = self.journal.finish_job(self.job, JobStatus::Cancelled);
    }
}
