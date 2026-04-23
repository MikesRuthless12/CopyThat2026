//! `Journal` — open the redb file, allocate row ids, write
//! checkpoints, surface unfinished jobs at boot, decide a resume
//! plan when the engine sees a partial destination.
//!
//! Every public method that mutates state opens its own
//! `WriteTransaction` and commits before returning. redb gives us
//! ACID + fsync per commit; the engine's 50 ms checkpoint cadence
//! becomes one fsync per 50 ms in the worst case, which the journal
//! deliberately accepts as the cost of "no work lost on power-cut".

use std::path::{Path, PathBuf};
use std::sync::Arc;

use directories::ProjectDirs;
use redb::{Database, ReadableTable};

use crate::error::{JournalError, Result};
use crate::schema::{FILES, JOBS, SEQ, SEQ_KEY_NEXT_ROW_ID};
use crate::types::{
    CheckpointId, FileCheckpoint, FileStatus, JobRecord, JobRowId, JobStatus, ResumePlan,
    UnfinishedJob, now_ms,
};

/// Default path: `<data-dir>/copythat-journal.redb`. Sits next to
/// `history.db` so a backup script that already grabs the data dir
/// gets resume info for free.
pub fn default_journal_path() -> Result<PathBuf> {
    let dirs =
        ProjectDirs::from("com", "CopyThat", "CopyThat2026").ok_or(JournalError::NoDataDir)?;
    let dir = dirs.data_dir().to_path_buf();
    Ok(dir.join("copythat-journal.redb"))
}

/// Resume journal handle.
///
/// Cheap to clone — the inner redb `Database` is wrapped in an
/// `Arc` so multiple runner tasks share one open file. redb is
/// internally synchronised, so concurrent reads + a single writer
/// is the supported access pattern.
#[derive(Debug, Clone)]
pub struct Journal {
    inner: Arc<Database>,
}

impl Journal {
    /// Open (or create) the journal at `path`. Creates the parent
    /// directory if missing — same opt-in convenience as
    /// `History::open_default`.
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|e| JournalError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let db = Database::create(path)?;
        // Eagerly create the three tables so a fresh journal isn't
        // missing them on first read. `create_table` inside a write
        // txn is a no-op on existing tables.
        let txn = db.begin_write()?;
        let _ = txn.open_table(JOBS)?;
        let _ = txn.open_table(FILES)?;
        let _ = txn.open_table(SEQ)?;
        txn.commit()?;
        Ok(Self {
            inner: Arc::new(db),
        })
    }

    /// Open the journal at [`default_journal_path`].
    pub fn open_default() -> Result<Self> {
        Self::open(&default_journal_path()?)
    }

    /// Allocate a fresh `JobRowId` and persist `record` against it.
    /// `record.status` is forced to `Running` regardless of what the
    /// caller passes — a freshly-begun job is always running by
    /// construction.
    pub fn begin_job(&self, mut record: JobRecord) -> Result<JobRowId> {
        record.status = JobStatus::Running;
        let id = self.allocate_row_id()?;
        let json = serde_json::to_string(&record)?;
        let txn = self.inner.begin_write()?;
        {
            let mut t = txn.open_table(JOBS)?;
            t.insert(id, json.as_str())?;
        }
        txn.commit()?;
        Ok(JobRowId(id))
    }

    /// Persist a per-file checkpoint. Returns the monotonic
    /// [`CheckpointId`] — callers can correlate UI progress events
    /// with on-disk durability ("checkpoint #423 landed at 50%").
    ///
    /// `dst_path` is required so resume can probe the right partial
    /// destination even when the original walk hasn't been replayed
    /// yet at boot.
    pub fn checkpoint(
        &self,
        job: JobRowId,
        file_idx: u64,
        dst_path: &Path,
        bytes_done: u64,
        expected_total: u64,
        hash_so_far: [u8; 32],
    ) -> Result<CheckpointId> {
        let key = (job.0, file_idx);
        let txn = self.inner.begin_write()?;
        let next_seq;
        {
            let mut t = txn.open_table(FILES)?;
            // Read previous checkpoint (if any) so we increment the
            // per-file `last_checkpoint` counter monotonically.
            let prev_seq = match t.get(key)? {
                Some(v) => {
                    let s = v.value();
                    let prev: FileCheckpoint = serde_json::from_str(s)?;
                    prev.last_checkpoint
                }
                None => 0,
            };
            next_seq = prev_seq + 1;
            let cp = FileCheckpoint {
                file_idx,
                bytes_done,
                expected_total,
                hash_so_far,
                final_hash: None,
                status: FileStatus::InFlight,
                last_checkpoint: next_seq,
                last_checkpoint_at_ms: now_ms(),
                dst_path: dst_path.to_path_buf(),
            };
            let json = serde_json::to_string(&cp)?;
            t.insert(key, json.as_str())?;
        }
        txn.commit()?;
        Ok(CheckpointId(next_seq))
    }

    /// Mark a file as finished and capture its `final_hash`. The
    /// per-job `bytes_done` / `files_done` counters update
    /// atomically in the same transaction.
    pub fn finish_file(&self, job: JobRowId, file_idx: u64, final_hash: [u8; 32]) -> Result<()> {
        let key = (job.0, file_idx);
        let txn = self.inner.begin_write()?;
        {
            let mut files_t = txn.open_table(FILES)?;
            let prev = files_t.get(key)?.ok_or(JournalError::NotFound(job.0))?;
            let mut cp: FileCheckpoint = serde_json::from_str(prev.value())?;
            drop(prev);
            let bytes_added = cp.expected_total.saturating_sub(cp.bytes_done);
            cp.bytes_done = cp.expected_total.max(cp.bytes_done);
            cp.final_hash = Some(final_hash);
            cp.status = FileStatus::Finished;
            cp.last_checkpoint += 1;
            cp.last_checkpoint_at_ms = now_ms();
            let json = serde_json::to_string(&cp)?;
            files_t.insert(key, json.as_str())?;

            // Update job-level totals so `unfinished()` carries the
            // running progress without a per-file scan.
            let mut jobs_t = txn.open_table(JOBS)?;
            let prev_job = jobs_t.get(job.0)?.ok_or(JournalError::NotFound(job.0))?;
            let mut rec: JobRecord = serde_json::from_str(prev_job.value())?;
            drop(prev_job);
            rec.files_done = rec.files_done.saturating_add(1);
            rec.bytes_done = rec.bytes_done.saturating_add(bytes_added);
            let json = serde_json::to_string(&rec)?;
            jobs_t.insert(job.0, json.as_str())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Mark a job as terminal — success / failure / cancellation.
    /// Terminal jobs no longer surface in [`Self::unfinished`].
    /// File checkpoints stay in the table so the UI can still render
    /// a "what got copied" detail view from history.
    pub fn finish_job(&self, job: JobRowId, status: JobStatus) -> Result<()> {
        let txn = self.inner.begin_write()?;
        {
            let mut t = txn.open_table(JOBS)?;
            let prev = t.get(job.0)?.ok_or(JournalError::NotFound(job.0))?;
            let mut rec: JobRecord = serde_json::from_str(prev.value())?;
            drop(prev);
            rec.status = status;
            let json = serde_json::to_string(&rec)?;
            t.insert(job.0, json.as_str())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Return every job whose status is still
    /// [`JobStatus::Running`] or [`JobStatus::Paused`], with the
    /// latest per-file checkpoints attached.
    ///
    /// Callers populate the boot-time `ResumePromptModal` from this
    /// list. The order is by ascending `JobRowId` — i.e. the order
    /// jobs were begun.
    pub fn unfinished(&self) -> Result<Vec<UnfinishedJob>> {
        let txn = self.inner.begin_read()?;
        let jobs_t = txn.open_table(JOBS)?;
        let files_t = txn.open_table(FILES)?;

        let mut out: Vec<UnfinishedJob> = Vec::new();
        for entry in jobs_t.iter()? {
            let (k, v) = entry?;
            let row_id = JobRowId(k.value());
            let rec: JobRecord = serde_json::from_str(v.value())?;
            if !rec.status.is_unfinished() {
                continue;
            }
            // Pull every checkpoint with the matching job prefix.
            // redb sorts the composite key by tuple semantics, so a
            // half-open range scan from (job, 0) → (job+1, 0) gives
            // us exactly that job's files.
            let files: Vec<FileCheckpoint> = files_t
                .range((row_id.0, 0)..(row_id.0 + 1, 0))?
                .filter_map(|r| {
                    r.ok()
                        .and_then(|(_, v)| serde_json::from_str(v.value()).ok())
                })
                .collect();
            out.push(UnfinishedJob {
                row_id,
                record: rec,
                files,
            });
        }
        Ok(out)
    }

    /// Decide what the engine should do with the existing partial
    /// destination at `dst_path`. The engine calls this after it
    /// sees `dst.exists() && dst.metadata().len() < expected_total`.
    ///
    /// Returns:
    /// - `ResumePlan::AlreadyComplete` when the latest checkpoint's
    ///   `final_hash` is `Some` and the dst is already at the expected
    ///   size. (The caller still hashes the dst to confirm before
    ///   skipping the copy entirely.)
    /// - `ResumePlan::Resume { offset, src_hash_at_offset }` when
    ///   we have a checkpoint with `bytes_done > 0` and the dst's
    ///   length is at least `bytes_done`. The engine re-hashes the
    ///   dst's first `offset` bytes and either continues or aborts.
    /// - `ResumePlan::Restart` otherwise.
    pub fn resume_plan(&self, job: JobRowId, file_idx: u64) -> Result<ResumePlan> {
        let txn = self.inner.begin_read()?;
        let files_t = txn.open_table(FILES)?;
        let key = (job.0, file_idx);
        let Some(v) = files_t.get(key)? else {
            return Ok(ResumePlan::Restart);
        };
        let cp: FileCheckpoint = serde_json::from_str(v.value())?;
        if let Some(final_hash) = cp.final_hash {
            return Ok(ResumePlan::AlreadyComplete { final_hash });
        }
        if cp.bytes_done == 0 {
            return Ok(ResumePlan::Restart);
        }
        Ok(ResumePlan::Resume {
            offset: cp.bytes_done,
            src_hash_at_offset: cp.hash_so_far,
        })
    }

    /// Look up a job by id. Used by the runner to render the
    /// resume modal row and by the smoke test for asserts.
    pub fn job(&self, job: JobRowId) -> Result<JobRecord> {
        let txn = self.inner.begin_read()?;
        let t = txn.open_table(JOBS)?;
        let v = t.get(job.0)?.ok_or(JournalError::NotFound(job.0))?;
        Ok(serde_json::from_str(v.value())?)
    }

    /// All file checkpoints for a job, sorted by `file_idx`.
    pub fn files(&self, job: JobRowId) -> Result<Vec<FileCheckpoint>> {
        let txn = self.inner.begin_read()?;
        let files_t = txn.open_table(FILES)?;
        let mut out: Vec<FileCheckpoint> = files_t
            .range((job.0, 0)..(job.0 + 1, 0))?
            .filter_map(|r| {
                r.ok()
                    .and_then(|(_, v)| serde_json::from_str(v.value()).ok())
            })
            .collect();
        out.sort_by_key(|f| f.file_idx);
        Ok(out)
    }

    /// Delete a job + its file checkpoints. The "Discard" button on
    /// the resume modal calls this; the smoke test exercises it
    /// after a successful resume.
    pub fn delete_job(&self, job: JobRowId) -> Result<()> {
        let txn = self.inner.begin_write()?;
        {
            let mut jobs_t = txn.open_table(JOBS)?;
            jobs_t.remove(job.0)?;

            let mut files_t = txn.open_table(FILES)?;
            // Collect keys first; redb's iterator borrows the table
            // across remove calls.
            let keys: Vec<(u64, u64)> = files_t
                .range((job.0, 0)..(job.0 + 1, 0))?
                .filter_map(|r| r.ok().map(|(k, _)| k.value()))
                .collect();
            for k in keys {
                files_t.remove(k)?;
            }
        }
        txn.commit()?;
        Ok(())
    }

    fn allocate_row_id(&self) -> Result<u64> {
        let txn = self.inner.begin_write()?;
        let id;
        {
            let mut t = txn.open_table(SEQ)?;
            let next = t.get(SEQ_KEY_NEXT_ROW_ID)?.map(|v| v.value()).unwrap_or(1);
            id = next;
            t.insert(SEQ_KEY_NEXT_ROW_ID, next + 1)?;
        }
        txn.commit()?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fresh_journal() -> (Journal, tempfile::TempDir) {
        let d = tempdir().unwrap();
        let path = d.path().join("j.redb");
        let j = Journal::open(&path).unwrap();
        (j, d)
    }

    fn dummy_record() -> JobRecord {
        JobRecord::new("copy", "/src", Some(PathBuf::from("/dst")))
    }

    #[test]
    fn open_then_reopen_persists_jobs() {
        let d = tempdir().unwrap();
        let path = d.path().join("j.redb");
        {
            let j = Journal::open(&path).unwrap();
            let id = j.begin_job(dummy_record()).unwrap();
            assert_eq!(id.as_u64(), 1);
        }
        let j2 = Journal::open(&path).unwrap();
        let unfinished = j2.unfinished().unwrap();
        assert_eq!(unfinished.len(), 1);
        assert_eq!(unfinished[0].row_id.as_u64(), 1);
    }

    #[test]
    fn allocate_row_id_is_monotonic_across_open_calls() {
        let d = tempdir().unwrap();
        let path = d.path().join("j.redb");
        let id_a = {
            let j = Journal::open(&path).unwrap();
            j.begin_job(dummy_record()).unwrap()
        };
        let id_b = {
            let j = Journal::open(&path).unwrap();
            j.begin_job(dummy_record()).unwrap()
        };
        assert_eq!(id_a.as_u64(), 1);
        assert_eq!(id_b.as_u64(), 2);
    }

    #[test]
    fn checkpoint_then_resume_plan_returns_resume() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        let dst = std::path::Path::new("/dst/f0");
        j.checkpoint(id, 0, dst, 1024, 4096, [9u8; 32]).unwrap();
        let plan = j.resume_plan(id, 0).unwrap();
        match plan {
            ResumePlan::Resume {
                offset,
                src_hash_at_offset,
            } => {
                assert_eq!(offset, 1024);
                assert_eq!(src_hash_at_offset, [9u8; 32]);
            }
            other => panic!("expected Resume, got {other:?}"),
        }
    }

    #[test]
    fn finish_file_then_resume_plan_returns_already_complete() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        let dst = std::path::Path::new("/dst/f0");
        j.checkpoint(id, 0, dst, 4096, 4096, [9u8; 32]).unwrap();
        j.finish_file(id, 0, [42u8; 32]).unwrap();
        let plan = j.resume_plan(id, 0).unwrap();
        match plan {
            ResumePlan::AlreadyComplete { final_hash } => assert_eq!(final_hash, [42u8; 32]),
            other => panic!("expected AlreadyComplete, got {other:?}"),
        }
    }

    #[test]
    fn missing_checkpoint_returns_restart() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        assert_eq!(j.resume_plan(id, 99).unwrap(), ResumePlan::Restart);
    }

    #[test]
    fn finish_job_removes_from_unfinished() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        assert_eq!(j.unfinished().unwrap().len(), 1);
        j.finish_job(id, JobStatus::Succeeded).unwrap();
        assert_eq!(j.unfinished().unwrap().len(), 0);
    }

    #[test]
    fn delete_job_removes_files_too() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        j.checkpoint(id, 0, Path::new("/dst/0"), 1, 1, [0u8; 32])
            .unwrap();
        j.checkpoint(id, 1, Path::new("/dst/1"), 1, 1, [0u8; 32])
            .unwrap();
        assert_eq!(j.files(id).unwrap().len(), 2);
        j.delete_job(id).unwrap();
        assert_eq!(j.files(id).unwrap().len(), 0);
        assert_eq!(j.unfinished().unwrap().len(), 0);
    }

    #[test]
    fn finish_file_increments_job_totals() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        j.checkpoint(id, 0, Path::new("/dst/0"), 0, 1024, [0u8; 32])
            .unwrap();
        j.finish_file(id, 0, [1u8; 32]).unwrap();
        let rec = j.job(id).unwrap();
        assert_eq!(rec.files_done, 1);
        assert_eq!(rec.bytes_done, 1024);
    }

    #[test]
    fn checkpoint_increments_per_file_seq() {
        let (j, _d) = fresh_journal();
        let id = j.begin_job(dummy_record()).unwrap();
        let cp1 = j
            .checkpoint(id, 0, Path::new("/dst/0"), 100, 1000, [0u8; 32])
            .unwrap();
        let cp2 = j
            .checkpoint(id, 0, Path::new("/dst/0"), 200, 1000, [0u8; 32])
            .unwrap();
        assert_eq!(cp1.0 + 1, cp2.0);
    }
}
