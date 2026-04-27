//! In-memory job queue.
//!
//! The queue is a tracked collection with pub/sub — **it does not
//! itself run jobs**. Callers (e.g. the Tauri bridge or a future
//! `Runner` service) are responsible for pulling Pending jobs,
//! invoking `copy_file` / `copy_tree` / `move_*` / verify / shred,
//! and reporting progress back through
//! `Queue::{start, set_progress, mark_completed, mark_failed}`.
//!
//! Each job has its own `CopyControl`. The queue's
//! `pause_job` / `resume_job` / `cancel_job` drive that control so the
//! UI doesn't need to track them separately; the running worker only
//! has to hold the `CopyControl` clone returned by `add`.
//!
//! Persistence is out of scope for Phase 2 and lands in Phase 10
//! (SQLite history). Queue state lives in memory for the process
//! lifetime.

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::broadcast;

use crate::control::CopyControl;
use crate::error::CopyError;

/// Opaque identifier for a queued job.
///
/// IDs are monotonically increasing within a process and never
/// reused, even after a job is removed. Serialising them is safe —
/// they're just `u64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JobId(u64);

impl JobId {
    /// Return the inner `u64` value (e.g. for IPC serialisation).
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Classification of queued work. Matches the engines we'll wire up
/// in later phases (Verify = Phase 3, SecureDelete = Phase 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobKind {
    /// File / tree copy.
    Copy,
    /// File / tree move (copy + remove source).
    Move,
    /// Plain delete (single recycle-bin trip).
    Delete,
    /// Multi-pass shred via `copythat-secure-delete`.
    SecureDelete,
    /// Re-hash an existing destination against a recorded sidecar.
    Verify,
}

/// Lifecycle state of a single job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    /// Queued, not yet started.
    Pending,
    /// Engine task is currently running this job.
    Running,
    /// User requested pause; engine is parked at a buffer boundary.
    Paused,
    /// User cancelled; terminal.
    Cancelled,
    /// Completed without error; terminal.
    Succeeded,
    /// Completed with error; terminal.
    Failed,
}

/// Snapshot of a job. Mutable fields (`state`, `bytes_done`,
/// `bytes_total`, `started_at`, `finished_at`) are refreshed every
/// time `Queue::snapshot` or `Queue::get` is called.
#[derive(Debug, Clone)]
pub struct Job {
    /// Process-unique identifier for this job.
    pub id: JobId,
    /// What kind of work the engine plans to do.
    pub kind: JobKind,
    /// Source root the engine reads from.
    pub src: PathBuf,
    /// Destination root the engine writes to. `None` for delete jobs.
    pub dst: Option<PathBuf>,
    /// Lifecycle state at the time the snapshot was taken.
    pub state: JobState,
    /// Bytes copied so far.
    pub bytes_done: u64,
    /// Total bytes the engine plans to copy (or 0 before walk completes).
    pub bytes_total: u64,
    /// Files completed (success + skip + error).
    pub files_done: u64,
    /// Files in the planned set.
    pub files_total: u64,
    /// Wall-clock instant the job entered `Running`.
    pub started_at: Option<Instant>,
    /// Wall-clock instant the job entered a terminal state.
    pub finished_at: Option<Instant>,
    /// Most recent typed error, when the job is in `Failed`.
    pub last_error: Option<CopyError>,
}

/// Events fired on the queue's broadcast channel.
///
/// Clone semantics: `JobFailed` carries a `CopyError` — cheap to
/// clone (it's already `Clone`). Reordered carries just the id and
/// the new index.
#[derive(Debug, Clone)]
pub enum QueueEvent {
    /// A new job has been added to the queue.
    JobAdded(JobId),
    /// A pending job entered the `Running` state.
    JobStarted(JobId),
    /// Streaming progress for a running job.
    JobProgress {
        /// Identifies which job this event belongs to.
        id: JobId,
        /// Bytes copied so far.
        bytes_done: u64,
        /// Total bytes the engine plans to copy.
        bytes_total: u64,
        /// Files completed.
        files_done: u64,
        /// Files in the planned set.
        files_total: u64,
    },
    /// A running job entered `Paused`.
    JobPaused(JobId),
    /// A paused job re-entered `Running`.
    JobResumed(JobId),
    /// A job entered the terminal `Cancelled` state.
    JobCancelled(JobId),
    /// A job entered the terminal `Succeeded` state.
    JobCompleted(JobId),
    /// A job entered the terminal `Failed` state.
    JobFailed {
        /// Identifies which job failed.
        id: JobId,
        /// Typed error detail.
        err: CopyError,
    },
    /// A job was moved to a new index in the queue.
    JobReordered {
        /// Identifies which job moved.
        id: JobId,
        /// 0-based index after the reorder.
        new_index: usize,
    },
    /// A job was removed from the queue.
    JobRemoved(JobId),
}

const DEFAULT_BROADCAST_CAPACITY: usize = 1024;

struct Entry {
    job: Job,
    control: CopyControl,
}

struct Inner {
    entries: Vec<Entry>,
}

/// The queue itself. Cheap to `clone` — internals are `Arc`.
pub struct Queue {
    inner: std::sync::Arc<Mutex<Inner>>,
    tx: broadcast::Sender<QueueEvent>,
    next_id: std::sync::Arc<AtomicU64>,
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            tx: self.tx.clone(),
            next_id: self.next_id.clone(),
        }
    }
}

impl std::fmt::Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        f.debug_struct("Queue")
            .field("jobs", &guard.entries.len())
            .finish()
    }
}

impl Queue {
    /// Build an empty queue with the default broadcast channel capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BROADCAST_CAPACITY)
    }

    /// Construct a queue with a custom broadcast channel capacity.
    /// Subscribers that fall behind by more than `capacity` events
    /// start receiving `Lagged` errors and must reconcile via
    /// `snapshot`. 1024 is a generous default for GUI use.
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self {
            inner: std::sync::Arc::new(Mutex::new(Inner {
                entries: Vec::new(),
            })),
            tx,
            next_id: std::sync::Arc::new(AtomicU64::new(1)),
        }
    }

    /// Enqueue a new job. Returns its id and a `CopyControl` clone the
    /// runner should pass to the engine call. Queue-driven pause /
    /// resume / cancel flow through the same control.
    pub fn add(&self, kind: JobKind, src: PathBuf, dst: Option<PathBuf>) -> (JobId, CopyControl) {
        let id = JobId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let control = CopyControl::new();
        let job = Job {
            id,
            kind,
            src,
            dst,
            state: JobState::Pending,
            bytes_done: 0,
            bytes_total: 0,
            files_done: 0,
            files_total: 0,
            started_at: None,
            finished_at: None,
            last_error: None,
        };
        {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            guard.entries.push(Entry {
                job,
                control: control.clone(),
            });
        }
        let _ = self.tx.send(QueueEvent::JobAdded(id));
        (id, control)
    }

    /// Subscribe to queue events. Capacity is fixed at construction
    /// time; callers must consume quickly or catch up via `snapshot`.
    pub fn subscribe(&self) -> broadcast::Receiver<QueueEvent> {
        self.tx.subscribe()
    }

    /// Snapshot every queued job. Cloned data — modifications by the
    /// caller don't reach back into the queue.
    pub fn snapshot(&self) -> Vec<Job> {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        guard.entries.iter().map(|e| e.job.clone()).collect()
    }

    /// Look up a single queued job by id.
    pub fn get(&self, id: JobId) -> Option<Job> {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        guard
            .entries
            .iter()
            .find(|e| e.job.id == id)
            .map(|e| e.job.clone())
    }

    /// Return a clone of the [`CopyControl`] handle the engine task
    /// is steering through.
    pub fn control(&self, id: JobId) -> Option<CopyControl> {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        guard
            .entries
            .iter()
            .find(|e| e.job.id == id)
            .map(|e| e.control.clone())
    }

    /// Pause the job's running engine (no-op if not running).
    pub fn pause_job(&self, id: JobId) {
        let control = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if matches!(entry.job.state, JobState::Running | JobState::Pending) {
                entry.job.state = JobState::Paused;
                Some(entry.control.clone())
            } else {
                None
            }
        };
        if let Some(c) = control {
            c.pause();
            let _ = self.tx.send(QueueEvent::JobPaused(id));
        }
    }

    /// Resume a paused job. No-op for jobs in any other state.
    pub fn resume_job(&self, id: JobId) {
        let control = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if entry.job.state == JobState::Paused {
                entry.job.state = if entry.job.started_at.is_some() {
                    JobState::Running
                } else {
                    JobState::Pending
                };
                Some(entry.control.clone())
            } else {
                None
            }
        };
        if let Some(c) = control {
            c.resume();
            let _ = self.tx.send(QueueEvent::JobResumed(id));
        }
    }

    /// Cancel the job. Terminal — `cancel` wins over `resume`.
    pub fn cancel_job(&self, id: JobId) {
        let control = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if !matches!(
                entry.job.state,
                JobState::Cancelled | JobState::Succeeded | JobState::Failed
            ) {
                entry.job.state = JobState::Cancelled;
                entry.job.finished_at = Some(Instant::now());
                Some(entry.control.clone())
            } else {
                None
            }
        };
        if let Some(c) = control {
            c.cancel();
            let _ = self.tx.send(QueueEvent::JobCancelled(id));
        }
    }

    /// Move the job to index `new_index` (saturates at `0` and
    /// `len - 1`). No-op if the id is unknown.
    pub fn reorder(&self, id: JobId, new_index: usize) {
        let clamped = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(current) = guard.entries.iter().position(|e| e.job.id == id) else {
                return;
            };
            let clamped = new_index.min(guard.entries.len().saturating_sub(1));
            if current == clamped {
                return;
            }
            let entry = guard.entries.remove(current);
            guard.entries.insert(clamped, entry);
            clamped
        };
        let _ = self.tx.send(QueueEvent::JobReordered {
            id,
            new_index: clamped,
        });
    }

    /// Remove the job from the queue. No-op if unknown. Cancels first
    /// if still active so the running engine stops before the record
    /// disappears.
    pub fn remove(&self, id: JobId) {
        self.cancel_job(id);
        let removed = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            if let Some(pos) = guard.entries.iter().position(|e| e.job.id == id) {
                guard.entries.remove(pos);
                true
            } else {
                false
            }
        };
        if removed {
            let _ = self.tx.send(QueueEvent::JobRemoved(id));
        }
    }

    // ---------- runner-driven state transitions ----------

    /// Mark the job Running and stamp `started_at`. Idempotent.
    pub fn start(&self, id: JobId) {
        let changed = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if matches!(entry.job.state, JobState::Pending | JobState::Paused) {
                entry.job.state = JobState::Running;
                if entry.job.started_at.is_none() {
                    entry.job.started_at = Some(Instant::now());
                }
                true
            } else {
                false
            }
        };
        if changed {
            let _ = self.tx.send(QueueEvent::JobStarted(id));
        }
    }

    /// Update live progress. Emits `QueueEvent::JobProgress` every
    /// call — callers should throttle upstream if they're hot.
    pub fn set_progress(
        &self,
        id: JobId,
        bytes_done: u64,
        bytes_total: u64,
        files_done: u64,
        files_total: u64,
    ) {
        {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            entry.job.bytes_done = bytes_done;
            entry.job.bytes_total = bytes_total;
            entry.job.files_done = files_done;
            entry.job.files_total = files_total;
        }
        let _ = self.tx.send(QueueEvent::JobProgress {
            id,
            bytes_done,
            bytes_total,
            files_done,
            files_total,
        });
    }

    /// Move the job to terminal `Succeeded`. Idempotent.
    pub fn mark_completed(&self, id: JobId) {
        let changed = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if entry.job.state != JobState::Succeeded {
                entry.job.state = JobState::Succeeded;
                entry.job.finished_at = Some(Instant::now());
                if entry.job.bytes_total > 0 {
                    entry.job.bytes_done = entry.job.bytes_total;
                }
                true
            } else {
                false
            }
        };
        if changed {
            let _ = self.tx.send(QueueEvent::JobCompleted(id));
        }
    }

    /// Move the job to terminal `Failed`. Idempotent — only the first
    /// call records the error; subsequent calls are no-ops.
    pub fn mark_failed(&self, id: JobId, err: CopyError) {
        let emit = {
            let mut guard = self.inner.lock().expect("queue mutex poisoned");
            let Some(entry) = guard.entries.iter_mut().find(|e| e.job.id == id) else {
                return;
            };
            if entry.job.state != JobState::Failed {
                entry.job.state = JobState::Failed;
                entry.job.finished_at = Some(Instant::now());
                entry.job.last_error = Some(err.clone());
                true
            } else {
                false
            }
        };
        if emit {
            let _ = self.tx.send(QueueEvent::JobFailed { id, err });
        }
    }

    /// Elapsed wall-clock for a completed/running job, `None` if it
    /// hasn't started. Handy for UI callers.
    pub fn elapsed(&self, id: JobId) -> Option<Duration> {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        let entry = guard.entries.iter().find(|e| e.job.id == id)?;
        let start = entry.job.started_at?;
        Some(entry.job.finished_at.unwrap_or_else(Instant::now) - start)
    }

    /// Number of jobs currently in the queue (any state).
    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .expect("queue mutex poisoned")
            .entries
            .len()
    }

    /// `true` when the queue holds no jobs.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
