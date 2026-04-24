//! Public DTOs: JobRecord, FileCheckpoint, ResumePlan, etc.
//!
//! Kept in their own module so `lib.rs`'s public surface stays terse
//! and so the redb codec implementations (`schema.rs`) can derive
//! Serialize/Deserialize without circular use chains.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Stable identifier for a journal job row. Allocated monotonically
/// by [`crate::Journal::begin_job`]; survives process restarts via
/// the `seq` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobRowId(pub u64);

impl JobRowId {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for JobRowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "job#{}", self.0)
    }
}

/// Monotonic checkpoint sequence number per file. Returned by
/// [`crate::Journal::checkpoint`] so callers can correlate UI
/// updates with on-disk state. Wraps every 2^64 calls, which is
/// 50 ms × 2^64 = ~29 billion years — not a concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckpointId(pub u64);

/// What the journal knows about a job at the high level.
///
/// Mirrors `copythat_history::JobSummary` shape but lives separately
/// so this crate does not depend on the history crate. The Tauri
/// runner translates between the two at the boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct JobRecord {
    pub kind: String,
    pub src_root: PathBuf,
    /// `None` for shred/delete jobs — the journal carries them so
    /// the resume modal can show "DELETE in progress on <path>" at
    /// boot, even though there's no resume strategy for delete.
    pub dst_root: Option<PathBuf>,
    pub status: JobStatus,
    /// Wall-clock millis since epoch when the job was first
    /// `begin_job`-ed. Stable across resumes.
    pub started_at_ms: i64,
    /// Files in the planned set (denominator). `0` until the engine
    /// learns the total — checkpoints update this in place.
    pub files_total: u64,
    pub bytes_total: u64,
    /// Files that finished (numerator). Updated by `finish_file`.
    pub files_done: u64,
    pub bytes_done: u64,
    /// Phase 32f — when the destination is a cloud backend rather
    /// than a local path, carries the backend's registry name so
    /// the boot-time resume sweep can distinguish local-dst from
    /// remote-dst jobs. For local copies this stays `None` (the
    /// pre-32f default, preserved via `#[serde(default)]` on the
    /// whole struct). Resume of a remote-dst job currently falls
    /// through to "abandon + restart from scratch" because the
    /// backend-specific partial-upload reconciliation (list the
    /// partial MPU, resume from the last uploaded part) is the
    /// Phase 32g follow-up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_backend_name: Option<String>,
}

impl JobRecord {
    /// Build a new `JobRecord` ready for `Journal::begin_job`. Uses
    /// the system clock for `started_at_ms`; tests that need a
    /// deterministic timestamp build the struct by hand instead.
    pub fn new(
        kind: impl Into<String>,
        src_root: impl Into<PathBuf>,
        dst_root: Option<PathBuf>,
    ) -> Self {
        Self {
            kind: kind.into(),
            src_root: src_root.into(),
            dst_root,
            status: JobStatus::Running,
            started_at_ms: now_ms(),
            files_total: 0,
            bytes_total: 0,
            files_done: 0,
            bytes_done: 0,
            remote_backend_name: None,
        }
    }

    /// Phase 32f — constructor for a cloud-destined job. `dst_key`
    /// is the remote-backend key path; the caller stores it as
    /// `dst_root` (reused field) and the `remote_backend_name`
    /// tag flags the job as remote-dst at resume time.
    pub fn new_remote(
        kind: impl Into<String>,
        src_root: impl Into<PathBuf>,
        backend_name: impl Into<String>,
        dst_key: impl Into<String>,
    ) -> Self {
        let key = dst_key.into();
        Self {
            kind: kind.into(),
            src_root: src_root.into(),
            dst_root: Some(PathBuf::from(&key)),
            status: JobStatus::Running,
            started_at_ms: now_ms(),
            files_total: 0,
            bytes_total: 0,
            files_done: 0,
            bytes_done: 0,
            remote_backend_name: Some(backend_name.into()),
        }
    }

    /// True when the destination for this job is a cloud backend
    /// rather than a local path.
    pub fn is_remote(&self) -> bool {
        self.remote_backend_name.is_some()
    }
}

impl Default for JobRecord {
    fn default() -> Self {
        Self {
            kind: String::new(),
            src_root: PathBuf::new(),
            dst_root: None,
            status: JobStatus::Running,
            started_at_ms: 0,
            files_total: 0,
            bytes_total: 0,
            files_done: 0,
            bytes_done: 0,
            remote_backend_name: None,
        }
    }
}

/// Coarse job state. Mirrors the `JobState` set used by the live
/// queue but stays in this crate so the journal's wire schema is
/// owned by it alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobStatus {
    /// Job is in flight. The default for a freshly-`begin_job`'d row.
    Running,
    /// User pressed pause. The journal still ticks for in-flight
    /// files; resume from this state is identical to resume from
    /// `Running`.
    Paused,
    /// Reached the planned end of the file list. Terminal.
    Succeeded,
    /// Aborted (user cancelled, error policy hit, etc.). Terminal —
    /// `unfinished()` does not return it.
    Cancelled,
    /// Engine returned an error. Terminal — same exclusion as
    /// `Cancelled`. The caller may still inspect the per-file rows
    /// to render a "what got copied before the failure" view.
    Failed,
}

impl JobStatus {
    /// True when this status counts as "still pending" for the
    /// boot-time resume sweep.
    pub fn is_unfinished(self) -> bool {
        matches!(self, JobStatus::Running | JobStatus::Paused)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Succeeded => "succeeded",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }
}

/// Per-file checkpoint snapshot. One row per `(JobRowId, file_idx)`
/// in the `files` table; updated in place on every checkpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileCheckpoint {
    pub file_idx: u64,
    /// Bytes confirmed written to the destination as of this
    /// checkpoint. Always `<= expected_total` once the engine learns
    /// the size.
    pub bytes_done: u64,
    /// Total bytes the file is expected to hit at completion. `0`
    /// until the first checkpoint sees the source metadata.
    pub expected_total: u64,
    /// Running BLAKE3 of the source bytes already consumed (the same
    /// hasher state the engine feeds with every read). 32 bytes.
    /// All-zero if the engine isn't hashing yet (pre-first-byte).
    pub hash_so_far: [u8; 32],
    /// Final BLAKE3 once the file finishes. `None` until
    /// [`crate::Journal::finish_file`] is called.
    pub final_hash: Option<[u8; 32]>,
    pub status: FileStatus,
    /// Monotonic counter — mirrors the [`CheckpointId`] returned by
    /// the most recent `Journal::checkpoint` call for this file.
    pub last_checkpoint: u64,
    /// Wall-clock millis when the last checkpoint landed; the UI
    /// renders "X minutes ago" in the resume modal.
    pub last_checkpoint_at_ms: i64,
    /// Absolute path to the destination this file was being written
    /// to. Stored so resume can probe the existing partial dst even
    /// when the original tree walk hasn't been replayed.
    pub dst_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FileStatus {
    /// Engine is actively writing. `hash_so_far` is partial.
    InFlight,
    /// Engine wrote every byte and the verify pass (if any) succeeded.
    /// `final_hash` is `Some(...)`.
    Finished,
    /// Engine surfaced an error on this file. The tree may have
    /// continued; we keep the checkpoint so resume can pick up the
    /// next file without re-walking.
    Failed,
}

/// One row of `Journal::unfinished()` — a job the engine started
/// but didn't terminate before the process exited. The runner uses
/// this to populate the boot-time `ResumePromptModal`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfinishedJob {
    pub row_id: JobRowId,
    pub record: JobRecord,
    /// The latest per-file checkpoints, ordered by `file_idx`.
    /// Empty when the engine died before the first checkpoint
    /// landed (the common case for a sub-50-ms-lifetime job).
    pub files: Vec<FileCheckpoint>,
}

/// Engine's resume strategy for a single file. Returned by
/// [`crate::Journal::resume_plan`] after the engine sees that the
/// destination already exists.
#[derive(Debug, Clone, PartialEq)]
pub enum ResumePlan {
    /// Re-hash the destination's first `offset` bytes via BLAKE3
    /// and compare against `src_hash_at_offset`. On match, seek
    /// both files to `offset` and continue. On mismatch, the
    /// engine emits `CopyEvent::ResumeAborted` and falls back to a
    /// full restart.
    Resume {
        offset: u64,
        src_hash_at_offset: [u8; 32],
    },
    /// Nothing reusable — start over from byte 0.
    Restart,
    /// The destination is already the right size and the
    /// checkpoint's `final_hash` matches a hash of the existing dst.
    /// Skip the copy entirely.
    AlreadyComplete { final_hash: [u8; 32] },
}

pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_status_unfinished_excludes_terminal_states() {
        assert!(JobStatus::Running.is_unfinished());
        assert!(JobStatus::Paused.is_unfinished());
        assert!(!JobStatus::Succeeded.is_unfinished());
        assert!(!JobStatus::Cancelled.is_unfinished());
        assert!(!JobStatus::Failed.is_unfinished());
    }

    #[test]
    fn job_status_wire_strings_are_kebab() {
        assert_eq!(JobStatus::Running.as_str(), "running");
        assert_eq!(JobStatus::Paused.as_str(), "paused");
        assert_eq!(JobStatus::Succeeded.as_str(), "succeeded");
        assert_eq!(JobStatus::Cancelled.as_str(), "cancelled");
        assert_eq!(JobStatus::Failed.as_str(), "failed");
    }

    #[test]
    fn job_record_new_starts_running_with_zero_progress() {
        let r = JobRecord::new("copy", PathBuf::from("/src"), Some(PathBuf::from("/dst")));
        assert_eq!(r.status, JobStatus::Running);
        assert_eq!(r.kind, "copy");
        assert_eq!(r.bytes_done, 0);
        assert_eq!(r.files_done, 0);
        assert!(r.started_at_ms > 0);
    }

    #[test]
    fn job_record_round_trips_via_serde() {
        let before = JobRecord {
            kind: "move".into(),
            src_root: PathBuf::from("/a"),
            dst_root: Some(PathBuf::from("/b")),
            status: JobStatus::Paused,
            started_at_ms: 1_700_000_000_000,
            files_total: 5,
            bytes_total: 1024,
            files_done: 2,
            bytes_done: 512,
            remote_backend_name: None,
        };
        let s = serde_json::to_string(&before).unwrap();
        let after: JobRecord = serde_json::from_str(&s).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn new_remote_tags_backend_name_and_is_remote() {
        let job = JobRecord::new_remote("copy", "/src", "prod-s3", "data/out.bin");
        assert!(job.is_remote());
        assert_eq!(
            job.remote_backend_name.as_deref(),
            Some("prod-s3")
        );
        assert_eq!(
            job.dst_root.as_deref().map(|p| p.to_string_lossy().into_owned()),
            Some("data/out.bin".to_owned())
        );
    }

    #[test]
    fn new_local_has_no_remote_tag() {
        let job = JobRecord::new("copy", "/src", Some(PathBuf::from("/dst")));
        assert!(!job.is_remote());
        assert!(job.remote_backend_name.is_none());
    }

    #[test]
    fn round_trip_preserves_remote_backend_name() {
        let before = JobRecord::new_remote("copy", "/src", "s3-prod", "bucket/obj.bin");
        let s = serde_json::to_string(&before).unwrap();
        let after: JobRecord = serde_json::from_str(&s).unwrap();
        assert_eq!(before, after);
        assert_eq!(after.remote_backend_name.as_deref(), Some("s3-prod"));
    }

    #[test]
    fn legacy_record_without_remote_field_still_deserializes() {
        // Pre-Phase-32f records didn't carry `remote_backend_name`.
        // `#[serde(default)]` on the struct lets them round-trip.
        let legacy = r#"{
            "kind": "copy",
            "src_root": "/a",
            "dst_root": "/b",
            "status": "running",
            "started_at_ms": 1700000000000,
            "files_total": 0,
            "bytes_total": 0,
            "files_done": 0,
            "bytes_done": 0
        }"#;
        let parsed: JobRecord = serde_json::from_str(legacy).expect("legacy parse");
        assert!(parsed.remote_backend_name.is_none());
        assert!(!parsed.is_remote());
    }
}
