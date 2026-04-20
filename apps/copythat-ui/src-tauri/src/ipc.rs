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
/// Phase 7a — shell-extension hosts (Windows IExplorerCommand, macOS
/// Finder Sync, Linux Nautilus / Dolphin / Thunar) deliver paths to
/// the app via the single-instance plugin's argv hand-off. The
/// frontend listens for this event and routes the payload into its
/// existing drop-staging dialog with the verb pre-selected.
pub const EVENT_SHELL_ENQUEUE: &str = "shell-enqueue";

/// Phase 8 — the engine emitted `CopyEvent::ErrorPrompt` and the
/// runner parked the oneshot in the `ErrorRegistry`. The frontend's
/// `ErrorModal` subscribes to this and replies via `resolve_error`.
pub const EVENT_ERROR_RAISED: &str = "error-raised";
/// Phase 8 — mirror of [`EVENT_ERROR_RAISED`] for
/// `CopyEvent::Collision`. The frontend's `CollisionModal` replies
/// via `resolve_collision`.
pub const EVENT_COLLISION_RAISED: &str = "collision-raised";
/// Phase 8 — terminal notification for an `error-raised` event.
/// Emitted after `ErrorRegistry::resolve` fires the oneshot; lets
/// the frontend close the modal and pop a toast.
pub const EVENT_ERROR_RESOLVED: &str = "error-resolved";
/// Phase 8 — mirror of [`EVENT_ERROR_RESOLVED`] for collisions.
pub const EVENT_COLLISION_RESOLVED: &str = "collision-resolved";

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

/// Paths handed to the app by a shell-extension host via
/// `copythat --enqueue <verb> <paths…>`. Shares shape with
/// [`DropReceivedDto`] so the frontend can reuse its drop-staging
/// flow; adds `verb` so "Move with CopyThat" skips the radio.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellEnqueueDto {
    /// `"copy"` or `"move"`.
    pub verb: &'static str,
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

/// Phase 8 — payload for `error-raised`. The engine emitted an
/// `ErrorPrompt` and the runner parked the oneshot in the
/// `ErrorRegistry`; the frontend replies via `resolve_error(id, …)`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorPromptDto {
    pub id: u64,
    pub job_id: u64,
    pub src: String,
    pub dst: String,
    /// Lowercase-kebab kind name (`"permission-denied"`, …).
    pub kind: &'static str,
    /// Fluent key the frontend renders with `t()`.
    pub localized_key: &'static str,
    pub message: String,
    pub raw_os_error: Option<i32>,
    pub created_at_ms: u64,
}

/// Phase 8 — payload for `collision-raised`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollisionPromptDto {
    pub id: u64,
    pub job_id: u64,
    pub src: String,
    pub dst: String,
    /// Byte size of the source file. Sent ahead of time so the modal
    /// can render the preview without a round-trip.
    pub src_size: Option<u64>,
    /// mtime in ms since epoch — rendered as a localised date.
    pub src_modified_ms: Option<u64>,
    pub dst_size: Option<u64>,
    pub dst_modified_ms: Option<u64>,
}

/// Phase 8 — notification that an `error-raised` prompt was
/// resolved. Mirrors `ErrorPromptDto.id` + the chosen action so the
/// frontend can close the modal + show the right toast.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResolvedDto {
    pub id: u64,
    pub job_id: u64,
    /// `"retry"` / `"skip"` / `"abort"`.
    pub action: &'static str,
}

/// Phase 8 — mirror of [`ErrorResolvedDto`] for collisions.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollisionResolvedDto {
    pub id: u64,
    pub job_id: u64,
    pub resolution: &'static str,
}

/// Phase 8 — a single entry in the error log, as exposed to the
/// Svelte drawer + CSV / TXT exporters.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedErrorDto {
    pub id: u64,
    pub job_id: u64,
    pub timestamp_ms: u64,
    pub src: String,
    pub dst: String,
    pub kind: &'static str,
    pub localized_key: &'static str,
    pub message: String,
    pub raw_os_error: Option<i32>,
    /// `"retry"` / `"skip"` / `"abort"` / `"auto-skip"` / `null`.
    pub resolution: Option<&'static str>,
}

/// Phase 9 — one row as seen by the History drawer. Mirrors
/// [`copythat_history::JobSummary`] with camelCase field names and
/// string-serialised paths.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryJobDto {
    pub row_id: i64,
    pub kind: String,
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub src_root: String,
    pub dst_root: String,
    pub total_bytes: u64,
    pub files_ok: u64,
    pub files_failed: u64,
    pub verify_algo: Option<String>,
    pub options_json: Option<String>,
}

/// Phase 9 — one row from the History detail view. Mirrors
/// [`copythat_history::ItemRow`] for the IPC side.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItemDto {
    pub job_row_id: i64,
    pub src: String,
    pub dst: String,
    pub size: u64,
    pub status: String,
    pub hash_hex: Option<String>,
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
    pub timestamp_ms: i64,
}

/// Phase 9 — filter accepted by the `history_search` command.
/// Every field optional; the Tauri layer forwards them into
/// [`copythat_history::HistoryFilter`] verbatim.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryFilterDto {
    pub started_since_ms: Option<i64>,
    pub started_until_ms: Option<i64>,
    pub kind: Option<String>,
    pub status: Option<String>,
    pub text: Option<String>,
    pub limit: Option<u32>,
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
    /// Phase 8 — per-file error policy. Wire-encoded as a JSON
    /// object so the `RetryN` variant's two knobs round-trip.
    /// Accepted shapes:
    /// - `{"kind":"ask"}` (default)
    /// - `{"kind":"skip"}`
    /// - `{"kind":"abort"}`
    /// - `{"kind":"retryN","maxAttempts":3,"backoffMs":250}`
    pub on_error: Option<ErrorPolicyDto>,
    /// Phase 8 — collision policy (shadows the engine default,
    /// which is `Skip`). Wire-encoded as a short string:
    /// `"skip"` / `"overwrite"` / `"overwrite-if-newer"` /
    /// `"keep-both"` / `"prompt"`.
    ///
    /// `rename(name)` is not exposed here — that's a per-item
    /// decision made via `CollisionModal`, not a whole-job policy.
    pub collision: Option<String>,
}

/// Phase 8 — wire form of `copythat_core::ErrorPolicy`. Tagged by
/// `kind` so the two knobs on `RetryN` survive the JSON hop.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ErrorPolicyDto {
    Ask,
    Skip,
    Abort,
    #[serde(rename = "retryN")]
    RetryN {
        max_attempts: u8,
        backoff_ms: u64,
    },
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
