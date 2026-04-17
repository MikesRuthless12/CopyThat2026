//! IPC types exchanged between the Tauri Rust layer and the Svelte
//! frontend.
//!
//! Every value that crosses the boundary is `Serialize` (Rust → JS) or
//! `Deserialize` (JS → Rust). Field names use `camelCase` to match
//! idiomatic TypeScript — the `#[serde(rename_all = "camelCase")]`
//! attribute handles the translation. Event *names* stay `kebab-case`
//! (Tauri's convention for channels) and are declared in `EVENT_*`
//! constants below so there's exactly one source of truth for each
//! string.
//!
//! Kept free of engine types — `copythat_core::JobKind` etc. are
//! translated into stable lowercase strings before leaving this
//! module. That insulates the frontend from internal enum reshuffles.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use copythat_core::{Job, JobKind, JobState};

pub const EVENT_JOB_ADDED: &str = "job-added";
pub const EVENT_JOB_STARTED: &str = "job-started";
pub const EVENT_JOB_PROGRESS: &str = "job-progress";
pub const EVENT_JOB_PAUSED: &str = "job-paused";
pub const EVENT_JOB_RESUMED: &str = "job-resumed";
pub const EVENT_JOB_CANCELLED: &str = "job-cancelled";
pub const EVENT_JOB_COMPLETED: &str = "job-completed";
pub const EVENT_JOB_FAILED: &str = "job-failed";
pub const EVENT_JOB_REMOVED: &str = "job-removed";
pub const EVENT_GLOBALS_TICK: &str = "globals-tick";
pub const EVENT_DROP_RECEIVED: &str = "drop-received";

/// UI-facing snapshot of a single queue job.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDto {
    pub id: u64,
    pub kind: &'static str,
    pub state: &'static str,
    pub src: String,
    pub dst: Option<String>,
    pub name: String,
    pub subpath: Option<String>,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    pub rate_bps: u64,
    pub eta_seconds: Option<u64>,
    pub started_at_ms: Option<u64>,
    pub finished_at_ms: Option<u64>,
    pub last_error: Option<String>,
}

impl JobDto {
    /// Build a DTO from a queue snapshot. `rate_bps` is left at 0 —
    /// the runner carries live rate in the `job-progress` event and
    /// the frontend tracks it there.
    pub fn from_job(job: &Job) -> Self {
        let (name, subpath) = split_display(&job.src);
        Self {
            id: job.id.as_u64(),
            kind: job_kind_name(job.kind),
            state: job_state_name(job.state),
            src: path_to_string(&job.src),
            dst: job.dst.as_deref().map(path_to_string),
            name,
            subpath,
            bytes_done: job.bytes_done,
            bytes_total: job.bytes_total,
            files_done: job.files_done,
            files_total: job.files_total,
            rate_bps: 0,
            eta_seconds: None,
            started_at_ms: job.started_at.map(|_| now_ms()),
            finished_at_ms: job.finished_at.map(|_| now_ms()),
            last_error: job.last_error.as_ref().map(|e| e.message.clone()),
        }
    }
}

/// Payload for `job-progress`. Named fields (not positional) because
/// a frontend reading `.bytesTotal` is much easier to debug than
/// `.fields[1]`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobProgressDto {
    pub id: u64,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    pub rate_bps: u64,
    pub eta_seconds: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobIdDto {
    pub id: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobFailedDto {
    pub id: u64,
    pub message: String,
}

/// Global-level summary emitted on every progress tick. The header
/// strip and footer in the Svelte UI bind directly to this.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsDto {
    pub state: &'static str,
    pub active_jobs: u64,
    pub queued_jobs: u64,
    pub paused_jobs: u64,
    pub failed_jobs: u64,
    pub succeeded_jobs: u64,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub rate_bps: u64,
    pub eta_seconds: Option<u64>,
    pub errors: u64,
}

/// Paths dropped onto the app window. The frontend picks a
/// destination (via the dialog plugin) and then calls
/// [`crate::commands::start_copy`].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropReceivedDto {
    pub paths: Vec<String>,
}

/// File-icon classification returned by the `file_icon` command.
/// Lightweight by design: the frontend renders a matching Lucide
/// glyph locally. Phase 7 will extend this with real native
/// file-type icons (SHGetFileInfo / NSWorkspace / GIO).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIconDto {
    pub kind: &'static str,
    pub extension: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyOptionsDto {
    /// Post-copy verification algorithm name (`sha256`, `blake3`, ...).
    /// Parsed via `HashAlgorithm::from_str`; unknown values surface as
    /// a typed error from the invoking command.
    pub verify: Option<String>,
    pub preserve_times: Option<bool>,
    pub preserve_permissions: Option<bool>,
    pub fsync_on_close: Option<bool>,
    pub follow_symlinks: Option<bool>,
}

fn job_kind_name(kind: JobKind) -> &'static str {
    match kind {
        JobKind::Copy => "copy",
        JobKind::Move => "move",
        JobKind::Delete => "delete",
        JobKind::SecureDelete => "secure-delete",
        JobKind::Verify => "verify",
    }
}

pub fn job_state_name(state: JobState) -> &'static str {
    match state {
        JobState::Pending => "pending",
        JobState::Running => "running",
        JobState::Paused => "paused",
        JobState::Cancelled => "cancelled",
        JobState::Succeeded => "succeeded",
        JobState::Failed => "failed",
    }
}

fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// Split a path into (filename, parent-display). On a bare
/// filename, parent is `None`.
fn split_display(p: &Path) -> (String, Option<String>) {
    let name = p
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| p.to_string_lossy().to_string());
    let subpath = p
        .parent()
        .filter(|pp| !pp.as_os_str().is_empty())
        .map(|pp| pp.to_string_lossy().to_string());
    (name, subpath)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
