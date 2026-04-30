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

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
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

/// Opaque identifier for a named [`Queue`].
///
/// A single process can host multiple queues — typically one per
/// physical destination drive — managed by a [`QueueRegistry`]. The
/// registry assigns ids sequentially starting at `1`; the well-known
/// id `0` is reserved for the default queue created by
/// [`Queue::new`] when a registry isn't in play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueueId(u64);

impl QueueId {
    /// Reserved id for the default queue. Matches the value used by
    /// [`Queue::new`] so existing single-queue callers can omit the
    /// registry entirely and still report a stable id over IPC.
    pub const DEFAULT: QueueId = QueueId(0);

    /// Construct a [`QueueId`] from a raw `u64`. Primarily for IPC
    /// deserialisation; in-process code should use the id returned by
    /// [`QueueRegistry::route`] or [`Queue::id`].
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Return the inner `u64` (e.g. for IPC serialisation).
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for QueueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Q{}", self.0)
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
    id: QueueId,
    name: Arc<str>,
    inner: Arc<Mutex<Inner>>,
    tx: broadcast::Sender<QueueEvent>,
    next_id: Arc<AtomicU64>,
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: Arc::clone(&self.name),
            inner: Arc::clone(&self.inner),
            tx: self.tx.clone(),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

impl std::fmt::Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().expect("queue mutex poisoned");
        f.debug_struct("Queue")
            .field("id", &self.id)
            .field("name", &&*self.name)
            .field("jobs", &guard.entries.len())
            .finish()
    }
}

impl Queue {
    /// Build an empty queue with the default broadcast channel capacity.
    ///
    /// The id and name default to [`QueueId::DEFAULT`] / `"default"`
    /// so single-queue callers can keep using `Queue::new()` without
    /// touching a [`QueueRegistry`].
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BROADCAST_CAPACITY)
    }

    /// Construct a queue with a custom broadcast channel capacity.
    /// Subscribers that fall behind by more than `capacity` events
    /// start receiving `Lagged` errors and must reconcile via
    /// `snapshot`. 1024 is a generous default for GUI use.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_id_name_capacity(QueueId::DEFAULT, "default", capacity)
    }

    /// Construct a named queue with an explicit id and capacity. The
    /// id and name are immutable for the lifetime of the queue.
    pub fn with_id_name_capacity(id: QueueId, name: impl Into<Arc<str>>, capacity: usize) -> Self {
        let next_id = Arc::new(AtomicU64::new(1));
        Self::with_shared_counter(id, name, capacity, next_id)
    }

    /// Construct a queue that shares its job-id counter with sibling
    /// queues. [`QueueRegistry`] uses this so [`JobId`]s remain unique
    /// across every queue it owns — a property [`QueueRegistry::merge_into`]
    /// relies on when transferring jobs between queues.
    pub fn with_shared_counter(
        id: QueueId,
        name: impl Into<Arc<str>>,
        capacity: usize,
        next_id: Arc<AtomicU64>,
    ) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self {
            id,
            name: name.into(),
            inner: Arc::new(Mutex::new(Inner {
                entries: Vec::new(),
            })),
            tx,
            next_id,
        }
    }

    /// Return this queue's stable id.
    pub fn id(&self) -> QueueId {
        self.id
    }

    /// Return this queue's human-readable name (e.g. `"D: queue"`).
    pub fn name(&self) -> &str {
        &self.name
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

    /// Move every entry from `other` into `self`. Emits
    /// [`QueueEvent::JobRemoved`] on `other` and
    /// [`QueueEvent::JobAdded`] on `self` for each transferred job.
    ///
    /// IDs are preserved. The caller is responsible for ensuring
    /// `self` and `other` share a job-id counter
    /// (see [`Queue::with_shared_counter`]); otherwise transferred
    /// ids may collide with future ids minted by `self`.
    ///
    /// No-op when `self` and `other` reference the same underlying
    /// queue (cloned from one another).
    pub fn absorb(&self, other: &Queue) {
        if Arc::ptr_eq(&self.inner, &other.inner) {
            return;
        }
        let drained = {
            let mut other_guard = other.inner.lock().expect("queue mutex poisoned");
            std::mem::take(&mut other_guard.entries)
        };
        let drained_ids: Vec<JobId> = drained.iter().map(|e| e.job.id).collect();
        {
            let mut self_guard = self.inner.lock().expect("queue mutex poisoned");
            self_guard.entries.extend(drained);
        }
        for id in drained_ids {
            let _ = other.tx.send(QueueEvent::JobRemoved(id));
            let _ = self.tx.send(QueueEvent::JobAdded(id));
        }
    }
}

// ============================================================
// QueueRegistry — Phase 45.1
// ============================================================

/// Platform hook for identifying which physical drive a path lives
/// on. Implemented by callers that have access to OS-specific volume
/// queries (`copythat_platform::volume_id` is the canonical
/// implementation); tests can plug in a deterministic stub.
///
/// `copythat-core` deliberately does not depend on `copythat-platform`
/// (it would be a circular dep — the platform crate already depends
/// on this one), so the probe is injected at registry-construction
/// time.
pub trait VolumeProbe: Send + Sync + std::fmt::Debug + 'static {
    /// Return a stable identifier for the physical drive backing
    /// `path`. `None` indicates the probe couldn't classify the path
    /// (non-existent, network mount with no serial, etc.) — the
    /// registry treats those as a single anonymous bucket.
    fn volume_id(&self, path: &Path) -> Option<u64>;

    /// Return a human-readable label for the drive backing `path`
    /// (e.g. `"D:"` on Windows, `"/Volumes/Backup"` on macOS). Used
    /// only for naming newly-spawned queues; `None` falls back to a
    /// generic `"queue N"` name.
    fn drive_label(&self, path: &Path) -> Option<String> {
        let _ = path;
        None
    }
}

/// Events fired on the [`QueueRegistry`] broadcast channel.
#[derive(Debug, Clone)]
pub enum QueueRegistryEvent {
    /// A new queue was spawned (typically via [`QueueRegistry::route`]
    /// when a job targeted a previously-unseen drive).
    QueueAdded {
        /// Identifier assigned to the new queue.
        id: QueueId,
        /// Display name (e.g. `"D: queue"`).
        name: Arc<str>,
    },
    /// A queue was removed (the source side of a merge).
    QueueRemoved {
        /// Identifier of the removed queue.
        id: QueueId,
    },
    /// Two queues were merged via [`QueueRegistry::merge_into`].
    QueueMerged {
        /// Source queue (now removed).
        src: QueueId,
        /// Destination queue (absorbed `src`'s jobs).
        dst: QueueId,
    },
    /// A new job was routed to a queue. Tells the UI which tab to
    /// flash without re-snapshotting every queue.
    JobRouted {
        /// Queue the job ended up in.
        queue_id: QueueId,
        /// Identifier of the new job.
        job_id: JobId,
    },
}

/// Failure reason for [`QueueRegistry::merge_into`].
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum QueueMergeError {
    /// `src` does not name a queue currently held by the registry.
    #[error("unknown source queue: {0}")]
    UnknownSrc(QueueId),
    /// `dst` does not name a queue currently held by the registry.
    #[error("unknown destination queue: {0}")]
    UnknownDst(QueueId),
}

struct RegistryEntry {
    queue: Queue,
    /// Volume id the queue was spawned for; `None` for anonymous
    /// (no-probe) queues. Multiple queues may share `None` only via
    /// explicit insertion — [`QueueRegistry::route`] always picks the
    /// existing anonymous queue when probing yields `None`.
    drive_id: Option<u64>,
}

struct RegistryInner {
    entries: Vec<RegistryEntry>,
}

/// Owns multiple [`Queue`]s, one per physical destination drive.
///
/// [`route`](Self::route) is the single entry point a UI / IPC layer
/// uses to enqueue work: it picks an existing queue when the
/// destination lives on the same drive as one already running, or
/// spawns a new named queue when the destination is on a fresh drive.
///
/// Cheap to clone — internals are `Arc`-shared.
pub struct QueueRegistry {
    inner: Arc<Mutex<RegistryInner>>,
    probe: Option<Arc<dyn VolumeProbe>>,
    next_queue_id: Arc<AtomicU64>,
    next_job_id: Arc<AtomicU64>,
    /// F2-mode flag. When set, [`route`](Self::route) re-uses the
    /// queue that currently owns any [`JobState::Running`] job
    /// instead of spawning a new parallel queue. Public so the
    /// UI / Tauri command layer can flip it directly without going
    /// through a setter.
    pub auto_enqueue_next: Arc<AtomicBool>,
    tx: broadcast::Sender<QueueRegistryEvent>,
    capacity: usize,
}

impl Clone for QueueRegistry {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            probe: self.probe.clone(),
            next_queue_id: Arc::clone(&self.next_queue_id),
            next_job_id: Arc::clone(&self.next_job_id),
            auto_enqueue_next: Arc::clone(&self.auto_enqueue_next),
            tx: self.tx.clone(),
            capacity: self.capacity,
        }
    }
}

impl std::fmt::Debug for QueueRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().expect("registry mutex poisoned");
        f.debug_struct("QueueRegistry")
            .field("queues", &guard.entries.len())
            .field("auto_enqueue_next", &self.auto_enqueue_next.load(Ordering::Relaxed))
            .finish()
    }
}

impl Default for QueueRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueRegistry {
    /// Construct an empty registry with no volume probe — every dst
    /// hashes to a single anonymous queue. Equivalent to
    /// `QueueRegistry::new()` followed by no `with_probe` call.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BROADCAST_CAPACITY)
    }

    /// Construct an empty registry with a custom broadcast channel
    /// capacity for [`QueueRegistryEvent`]s.
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self {
            inner: Arc::new(Mutex::new(RegistryInner {
                entries: Vec::new(),
            })),
            probe: None,
            // Queue ids start at 1; id 0 is reserved for the
            // backwards-compat default Queue (see QueueId::DEFAULT).
            next_queue_id: Arc::new(AtomicU64::new(1)),
            next_job_id: Arc::new(AtomicU64::new(1)),
            auto_enqueue_next: Arc::new(AtomicBool::new(false)),
            tx,
            capacity,
        }
    }

    /// Install a [`VolumeProbe`] used to classify destination paths
    /// onto physical drives. Without a probe, every dst falls into a
    /// single anonymous queue.
    pub fn with_probe(mut self, probe: Arc<dyn VolumeProbe>) -> Self {
        self.probe = Some(probe);
        self
    }

    /// Subscribe to registry-level events.
    pub fn subscribe(&self) -> broadcast::Receiver<QueueRegistryEvent> {
        self.tx.subscribe()
    }

    /// Snapshot of every queue currently held by the registry, in
    /// insertion order.
    pub fn queues(&self) -> Vec<Queue> {
        let guard = self.inner.lock().expect("registry mutex poisoned");
        guard.entries.iter().map(|e| e.queue.clone()).collect()
    }

    /// Number of queues currently held.
    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .expect("registry mutex poisoned")
            .entries
            .len()
    }

    /// `true` when no queues exist yet.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Look up a queue by id.
    pub fn get(&self, id: QueueId) -> Option<Queue> {
        let guard = self.inner.lock().expect("registry mutex poisoned");
        guard
            .entries
            .iter()
            .find(|e| e.queue.id() == id)
            .map(|e| e.queue.clone())
    }

    /// Enqueue a new job. Picks an existing queue when `dst` lives on
    /// a drive already covered by one (probed via [`VolumeProbe`]), or
    /// spawns a new queue named from the drive label otherwise. When
    /// [`auto_enqueue_next`](Self::auto_enqueue_next) is set, routes
    /// into whichever queue currently holds a running job (F2 mode).
    pub fn route(
        &self,
        kind: JobKind,
        src: PathBuf,
        dst: Option<PathBuf>,
    ) -> (QueueId, JobId, CopyControl) {
        // 1. F2 mode short-circuit.
        if self.auto_enqueue_next.load(Ordering::Relaxed) {
            if let Some(queue) = self.find_running_queue() {
                let qid = queue.id();
                let (job_id, control) = queue.add(kind, src, dst);
                let _ = self.tx.send(QueueRegistryEvent::JobRouted {
                    queue_id: qid,
                    job_id,
                });
                return (qid, job_id, control);
            }
        }

        // 2. Probe the destination drive (and label) outside any lock —
        //    user-supplied probes may take their own locks / syscalls.
        let probe_path = dst.as_deref().map(Self::probe_path);
        let dst_drive = probe_path
            .as_deref()
            .and_then(|p| self.probe.as_ref().and_then(|pr| pr.volume_id(p)));
        let dst_label = probe_path
            .as_deref()
            .and_then(|p| self.probe.as_ref().and_then(|pr| pr.drive_label(p)));

        // 3. Find or spawn the target queue under a single critical
        //    section so racing routes don't double-create.
        let (queue, just_created) = {
            let mut inner = self.inner.lock().expect("registry mutex poisoned");
            let pos = inner
                .entries
                .iter()
                .position(|e| e.drive_id == dst_drive);
            if let Some(idx) = pos {
                (inner.entries[idx].queue.clone(), false)
            } else {
                let new_qid_value = self.next_queue_id.fetch_add(1, Ordering::Relaxed);
                let new_qid = QueueId(new_qid_value);
                let name: Arc<str> = match (&dst_label, dst_drive) {
                    (Some(label), _) => Arc::from(format!("{label} queue")),
                    (None, Some(_)) => Arc::from(format!("queue {new_qid_value}")),
                    (None, None) => Arc::from("default"),
                };
                let queue = Queue::with_shared_counter(
                    new_qid,
                    Arc::clone(&name),
                    self.capacity,
                    Arc::clone(&self.next_job_id),
                );
                inner.entries.push(RegistryEntry {
                    queue: queue.clone(),
                    drive_id: dst_drive,
                });
                (queue, true)
            }
        };

        if just_created {
            let _ = self.tx.send(QueueRegistryEvent::QueueAdded {
                id: queue.id(),
                name: Arc::from(queue.name()),
            });
        }

        let qid = queue.id();
        let (job_id, control) = queue.add(kind, src, dst);
        let _ = self.tx.send(QueueRegistryEvent::JobRouted {
            queue_id: qid,
            job_id,
        });
        (qid, job_id, control)
    }

    /// Move every job from `src` queue into `dst` queue and remove
    /// `src`. No-op when `src == dst`.
    pub fn merge_into(&self, src: QueueId, dst: QueueId) -> Result<(), QueueMergeError> {
        if src == dst {
            return Ok(());
        }
        // Hold the registry lock through the whole operation so a
        // concurrent merge can't observe the half-merged state. The
        // ordering registry-mutex → queue-mutex is the same one
        // route() uses, so no deadlock.
        let mut inner = self.inner.lock().expect("registry mutex poisoned");
        let src_queue = inner
            .entries
            .iter()
            .find(|e| e.queue.id() == src)
            .map(|e| e.queue.clone())
            .ok_or(QueueMergeError::UnknownSrc(src))?;
        let dst_queue = inner
            .entries
            .iter()
            .find(|e| e.queue.id() == dst)
            .map(|e| e.queue.clone())
            .ok_or(QueueMergeError::UnknownDst(dst))?;

        dst_queue.absorb(&src_queue);

        if let Some(pos) = inner.entries.iter().position(|e| e.queue.id() == src) {
            inner.entries.remove(pos);
        }
        drop(inner);

        let _ = self.tx.send(QueueRegistryEvent::QueueMerged { src, dst });
        let _ = self.tx.send(QueueRegistryEvent::QueueRemoved { id: src });
        Ok(())
    }

    fn find_running_queue(&self) -> Option<Queue> {
        let inner = self.inner.lock().expect("registry mutex poisoned");
        for entry in &inner.entries {
            // snapshot() takes the queue's own mutex, which we acquire
            // while holding the registry mutex. The route/merge paths
            // use the same registry → queue order; no deadlock.
            if entry
                .queue
                .snapshot()
                .iter()
                .any(|j| j.state == JobState::Running)
            {
                return Some(entry.queue.clone());
            }
        }
        None
    }

    /// `volume_id` accepts existing files but rejects unborn paths on
    /// some platforms — probe the parent when the dst itself doesn't
    /// exist yet (matches the heuristic the dispatcher / dedup paths
    /// already use elsewhere).
    fn probe_path(dst: &Path) -> &Path {
        if dst.exists() {
            dst
        } else {
            dst.parent().unwrap_or(dst)
        }
    }
}
