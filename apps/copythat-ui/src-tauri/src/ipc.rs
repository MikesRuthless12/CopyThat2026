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
/// Phase 22 — a collision was resolved by the runner against the
/// active [`ConflictProfile`] without ever showing a prompt to the
/// user. Emitted alongside the usual engine-side resolution so the
/// aggregate dialog renders "via rule '*.docx → newer'" rows.
pub const EVENT_COLLISION_AUTO_RESOLVED: &str = "collision-auto-resolved";

/// Per-file activity — lets the UI render a TeraCopy-style live list
/// of files inside a tree job. Emitted alongside the aggregate
/// `job-progress` events, rate-limited on the runner side so large
/// trees don't overwhelm the event bus.
pub const EVENT_FILE_ACTIVITY: &str = "file-activity";

/// Post-Phase-12 — fired by both the paste hotkey (`global_paste.rs`)
/// and the clipboard watcher (`clipboard_watcher.rs`) when files
/// appear on the OS clipboard. Frontend decides whether to pop a
/// toast or open the staging dialog based on the `count` field.
pub const EVENT_CLIPBOARD_FILES_DETECTED: &str = "clipboard-files-detected";

// Phase 19b — a snapshot source was created for a locked file.
// The frontend shows a "📷 Reading from <kind> snapshot" badge on
// the active row until the file completes.
pub const EVENT_SNAPSHOT_CREATED: &str = "snapshot-created";

// Phase 23 — the destination filesystem doesn't support sparse files.
// The frontend shows a one-shot toast so the user knows the destination
// will be larger on disk than the source.
pub const EVENT_SPARSENESS_NOT_SUPPORTED: &str = "sparseness-not-supported";

// Phase 24 — foreign metadata couldn't land on the destination
// filesystem natively, so the engine wrote an `._<filename>`
// AppleDouble sidecar carrying it. The frontend shows an info badge
// on the affected job row.
pub const EVENT_META_TRANSLATED_TO_APPLEDOUBLE: &str = "meta-translated-to-appledouble";

// Phase 21 — shape rate changed (settings update OR schedule poll
// minute tick). The header badge subscribes so the "🔻 30 MB/s · scheduled"
// pill re-renders without polling.
pub const EVENT_SHAPE_RATE_CHANGED: &str = "shape-rate-changed";

// Phase 19a — disk-backed scan lifecycle bus.
pub const EVENT_SCAN_STARTED: &str = "scan-started";
pub const EVENT_SCAN_PROGRESS: &str = "scan-progress";
pub const EVENT_SCAN_BATCH: &str = "scan-batch";
pub const EVENT_SCAN_PAUSED: &str = "scan-paused";
pub const EVENT_SCAN_RESUMED: &str = "scan-resumed";
pub const EVENT_SCAN_COMPLETED: &str = "scan-completed";
pub const EVENT_SCAN_CANCELLED: &str = "scan-cancelled";
pub const EVENT_SCAN_FAILED: &str = "scan-failed";

// ---------------------------------------------------------------------
// Phase 25 — two-way sync lifecycle events.
// ---------------------------------------------------------------------
pub const EVENT_SYNC_STARTED: &str = "sync-started";
pub const EVENT_SYNC_WALK_STARTED: &str = "sync-walk-started";
pub const EVENT_SYNC_WALK_COMPLETED: &str = "sync-walk-completed";
pub const EVENT_SYNC_ACTION: &str = "sync-action";
pub const EVENT_SYNC_CONFLICT: &str = "sync-conflict";
pub const EVENT_SYNC_COMPLETED: &str = "sync-completed";
pub const EVENT_SYNC_CANCELLED: &str = "sync-cancelled";
pub const EVENT_SYNC_FAILED: &str = "sync-failed";

// ---------------------------------------------------------------------
// Phase 26 — live-mirror lifecycle events.
// ---------------------------------------------------------------------
pub const EVENT_LIVE_MIRROR_STARTED: &str = "live-mirror-started";
pub const EVENT_LIVE_MIRROR_STOPPED: &str = "live-mirror-stopped";
pub const EVENT_LIVE_MIRROR_EVENT: &str = "live-mirror-event";

/// Single entry in the live activity list.
///
/// `phase`:
/// - `"start"`: engine opened the file / folder; no bytes yet.
/// - `"progress"`: mid-copy tick; `bytesDone`/`bytesTotal` populated.
/// - `"done"`: file finished successfully.
/// - `"error"`: file failed (engine logged it, tree continues).
/// - `"dir"`: a directory was created at the destination.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileActivityDto {
    pub job_id: u64,
    pub seq: u64,
    pub phase: &'static str,
    pub src: String,
    pub dst: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub is_dir: bool,
    pub message: Option<String>,
}

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

/// Emitted by both the paste hotkey and the clipboard watcher when
/// they observe files landing on the OS clipboard. `count = 0` with
/// an empty `paths` is a legitimate "nothing to paste" ping — the UI
/// uses it to show "clipboard has no files" when the hotkey fires on
/// a text-only clipboard.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardFilesDetectedDto {
    pub paths: Vec<String>,
    pub count: usize,
    pub shortcut: String,
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

/// Phase 22 — emitted by the runner when an incoming
/// `CopyEvent::Collision` was silently auto-resolved against the
/// active `ConflictProfile`. The UI mirrors this into the
/// aggregate dialog's left rail as a "✓ via rule '*.docx → newer'"
/// row so the user can see *what was decided* without having seen
/// the prompt. The resolution is always the engine's four-variant
/// wire string (`"skip"` / `"overwrite"` / `"rename"`); the
/// `matched_rule_pattern` carries the glob that fired (or `"*"`
/// for a fallback hit).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollisionAutoResolvedDto {
    pub job_id: u64,
    pub src: String,
    pub dst: String,
    /// Engine-facing resolution string — what actually happened.
    pub resolution: &'static str,
    /// UI-facing rule-level resolution — what the user's intent
    /// was (e.g. `"overwrite-if-newer"` resolves to the engine's
    /// `"overwrite"` or `"skip"` depending on mtimes).
    pub rule_resolution: &'static str,
    pub matched_rule_pattern: String,
}

/// Phase 22 — payload for `thumbnail_for`. `kind` selects the
/// rendering branch:
///
/// - `"image"` — `data_url` is a base64-encoded PNG ≤ `max_dim`
///   on its longest edge. The frontend drops it into an `<img>`
///   without a second fetch.
/// - `"icon"` — `icon_kind` / `extension` name a Lucide glyph the
///   existing `FileKindIcon.svelte` already knows how to render.
///   Emitted for PDFs, videos, audio, archives, and anything the
///   pure-Rust decoders can't handle; acceptable fallback per the
///   Phase 22 brief ("for other formats fall back to the file-
///   kind icon").
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailDto {
    /// `"image"` or `"icon"`.
    pub kind: &'static str,
    /// Base64-encoded PNG data URL, populated when `kind == "image"`.
    pub data_url: Option<String>,
    /// Lucide glyph name, populated when `kind == "icon"`.
    pub icon_kind: Option<&'static str>,
    /// Lowercase extension (no dot) for icon-kind payloads — helps
    /// the UI render `.docx` / `.xlsx` badges.
    pub extension: Option<String>,
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

/// Phase 10 — lifetime aggregates for the Totals drawer. Mirrors
/// [`copythat_history::Totals`] with serialisable types.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalsDto {
    pub bytes: u64,
    pub files: u64,
    pub jobs: u64,
    pub errors: u64,
    pub duration_ms: u64,
    pub by_kind: Vec<KindBreakdownDto>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindBreakdownDto {
    pub kind: String,
    pub bytes: u64,
    pub files: u64,
    pub jobs: u64,
}

/// Phase 10 — one bucket of the 30-day sparkline. `dateMs` is the
/// UTC-midnight timestamp so the frontend can render in local tz.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTotalDto {
    pub date_ms: i64,
    pub bytes: u64,
    pub files: u64,
    pub jobs: u64,
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

// ---------------------------------------------------------------------
// Phase 12 — Settings + Profiles DTOs
// ---------------------------------------------------------------------

/// Frontend-facing snapshot of the live settings. Mirrors
/// `copythat_settings::Settings` with camelCase field naming so
/// idiomatic TypeScript can consume it directly.
///
/// The conversion is one-way at the IPC surface: Rust is the owner,
/// the frontend receives a fresh snapshot on each `get_settings` /
/// `update_settings` reply. Partial updates from the frontend go
/// through `update_settings`, which receives a full replacement blob
/// (simpler than tracking a patch language across the boundary).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub general: GeneralDto,
    pub transfer: TransferDto,
    pub shell: ShellDto,
    pub secure_delete: SecureDeleteDto,
    pub advanced: AdvancedDto,
    /// Phase 14a — enumeration-time filters.
    pub filters: FiltersDto,
    /// Phase 15 — auto-update channel + throttle state.
    pub updater: UpdaterDto,
    /// Phase 19a — scan database configuration.
    pub scan: ScanDto,
    /// Phase 21 — bandwidth shaping (global cap + schedule + auto-throttle).
    pub network: NetworkDto,
    /// Phase 29 — drag-and-drop polish (spring-load, drag thumbnails,
    /// invalid-target highlight).
    #[serde(default)]
    pub dnd: DndDto,
    /// Phase 30 — cross-platform path translation (Unicode NFC/NFD,
    /// line-ending rewrite, Windows reserved-name handling, `\\?\`
    /// long-path prefix).
    #[serde(default)]
    pub path_translation: PathTranslationDto,
}

/// Phase 29 — wire form of `copythat_settings::DndSettings`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DndDto {
    pub spring_load_enabled: bool,
    pub spring_load_delay_ms: u32,
    pub show_drag_thumbnails: bool,
    pub highlight_invalid_targets: bool,
}

impl Default for DndDto {
    fn default() -> Self {
        let defaults = copythat_settings::DndSettings::default();
        Self {
            spring_load_enabled: defaults.spring_load_enabled,
            spring_load_delay_ms: defaults.spring_load_delay_ms,
            show_drag_thumbnails: defaults.show_drag_thumbnails,
            highlight_invalid_targets: defaults.highlight_invalid_targets,
        }
    }
}

/// Phase 30 — wire form of `copythat_settings::PathTranslationSettings`.
/// Enum fields ride as short strings so older frontends silently
/// ignore unknown values (fall back to `Default` via the `*::from_wire`
/// helpers on the settings side).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathTranslationDto {
    pub enabled: bool,
    /// `"auto" | "windows" | "macos" | "linux"`.
    pub target_os: String,
    /// `"auto" | "nfc" | "nfd" | "as-is"`.
    pub unicode_normalization: String,
    /// `"as-is" | "crlf" | "lf"`.
    pub line_endings: String,
    /// `"suffix" | "reject"`.
    pub reserved_name_strategy: String,
    /// `"win32-long-path" | "truncate" | "reject"`.
    pub long_path_strategy: String,
    /// Lowercase extensions (no leading dot).
    pub line_ending_allowlist: Vec<String>,
}

impl Default for PathTranslationDto {
    fn default() -> Self {
        let d = copythat_settings::PathTranslationSettings::default();
        Self {
            enabled: d.enabled,
            target_os: d.target_os.as_str().to_string(),
            unicode_normalization: d.unicode_normalization.as_str().to_string(),
            line_endings: d.line_endings.as_str().to_string(),
            reserved_name_strategy: d.reserved_name_strategy.as_str().to_string(),
            long_path_strategy: d.long_path_strategy.as_str().to_string(),
            line_ending_allowlist: d.line_ending_allowlist,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralDto {
    pub language: String,
    /// `"auto" | "light" | "dark"`.
    pub theme: String,
    pub start_with_os: bool,
    pub single_instance: bool,
    pub minimize_to_tray: bool,
    /// `"modal" | "drawer"` — see `copythat_settings::ErrorDisplayMode`.
    pub error_display_mode: String,
    pub paste_shortcut_enabled: bool,
    /// Tauri `global-shortcut` combo string (e.g. `"CmdOrCtrl+Shift+V"`).
    pub paste_shortcut: String,
    pub clipboard_watcher_enabled: bool,
    /// Phase 20 — silent re-enqueue of unfinished jobs at startup.
    pub auto_resume_interrupted: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDto {
    pub buffer_size_bytes: u64,
    /// `"off" | "crc32" | "md5" | "sha1" | "sha256" | "sha512" |
    /// "xxhash3-64" | "xxhash3-128" | "blake3"`.
    pub verify: String,
    /// `"auto"` or `"manual-N"` where N is 1..=16.
    pub concurrency: String,
    /// `"prefer" | "avoid" | "disabled"`.
    pub reflink: String,
    pub fsync_on_close: bool,
    pub preserve_timestamps: bool,
    pub preserve_permissions: bool,
    pub preserve_acls: bool,
    /// Phase 19b — `"ask" | "retry" | "skip" | "snapshot"`. Stored as
    /// a short string so older frontends silently ignore unknown
    /// values (falls back to `"ask"` via `LockedFilePolicyChoice::from_wire`).
    pub on_locked: String,
    /// Phase 23 — preserve source sparseness on the destination.
    /// `true` is the default; frontends that ship without this key
    /// (older builds) fall through serde-default to `true`.
    #[serde(default = "default_true")]
    pub preserve_sparseness: bool,
    /// Phase 24 — master toggle for security-metadata preservation
    /// (NTFS ADS / xattrs / ACLs / SELinux / resource forks).
    #[serde(default = "default_true")]
    pub preserve_security_metadata: bool,
    /// Phase 24 — preserve Mark-of-the-Web (`Zone.Identifier` ADS).
    /// Security-sensitive: tooltip warns the user.
    #[serde(default = "default_true")]
    pub preserve_motw: bool,
    /// Phase 24 — preserve POSIX ACLs and the broader xattr surface.
    #[serde(default = "default_true")]
    pub preserve_posix_acls: bool,
    /// Phase 24 — preserve `security.selinux` Mandatory Access
    /// Control labels.
    #[serde(default = "default_true")]
    pub preserve_selinux_contexts: bool,
    /// Phase 24 — preserve macOS resource forks + Finder info.
    #[serde(default = "default_true")]
    pub preserve_resource_forks: bool,
    /// Phase 24 — fall back to `._<filename>` AppleDouble sidecar
    /// when the destination FS can't accept the foreign metadata.
    #[serde(default = "default_true")]
    pub appledouble_fallback: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellDto {
    pub context_menu_enabled: bool,
    pub intercept_default_copy: bool,
    pub notify_on_completion: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDeleteDto {
    /// `"zero" | "random" | "dod-3-pass" | "dod-7-pass" | "gutmann" |
    /// "nist-800-88"`.
    pub method: String,
    pub confirm_twice: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedDto {
    /// `"off" | "trace" | "debug" | "info" | "warn" | "error"`.
    pub log_level: String,
    /// Always `false` — see `AdvancedSettings::telemetry`. Exposed on
    /// the wire so the frontend can assert it in a visible "(off)"
    /// label and so a TOML dump shows the explicit "nope".
    pub telemetry: bool,
    pub error_policy: ErrorPolicyDtoV2,
    pub history_retention_days: u32,
    /// Absolute path, or `null` for "use default".
    pub database_path: Option<String>,
}

/// Settings-shaped error-policy variant. Different from the Phase 8
/// [`ErrorPolicyDto`] (which is a per-enqueue override) because this
/// one persists to disk and needs serde tagging that TOML / JSON can
/// round-trip cleanly. Kept separate to avoid churning the Phase 8
/// wire contract.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ErrorPolicyDtoV2 {
    Ask,
    Skip,
    #[serde(rename_all = "camelCase")]
    RetryN {
        max_attempts: u8,
        backoff_ms: u64,
    },
    Abort,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoDto {
    pub name: String,
    pub path: String,
}

/// Phase 15 — wire form of `copythat_settings::UpdaterSettings`.
/// `channel` is a short string (`"stable" | "beta"`); every other
/// field maps straight across.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterDto {
    pub auto_check: bool,
    pub channel: String,
    pub last_check_unix_secs: i64,
    pub dismissed_version: String,
    pub check_interval_secs: u32,
}

/// Phase 14a — TOML-friendly mirror of `copythat_settings::FilterSettings`
/// in camelCase for TypeScript consumption. Dates are signed Unix
/// epoch seconds (so pre-1970 mtimes don't wrap), sizes are byte
/// counts, `None` = "no bound on this end".
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltersDto {
    pub enabled: bool,
    pub include_globs: Vec<String>,
    pub exclude_globs: Vec<String>,
    pub min_size_bytes: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub min_mtime_unix_secs: Option<i64>,
    pub max_mtime_unix_secs: Option<i64>,
    pub skip_hidden: bool,
    pub skip_system: bool,
    pub skip_readonly: bool,
}

/// Phase 21 — wire form of `copythat_settings::NetworkSettings`.
///
/// `mode` is the discriminator string (`"off" | "fixed" | "schedule"`).
/// `fixedBytesPerSecond` only matters when `mode == "fixed"`;
/// `scheduleSpec` only matters when `mode == "schedule"`. Auto rules
/// are always honoured if the OS reports the matching state — the
/// per-OS bridges land in Phase 31, so today they're effectively
/// no-ops.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDto {
    pub mode: String,
    pub fixed_bytes_per_second: u64,
    pub schedule_spec: String,
    pub auto_on_metered: AutoThrottleRuleDto,
    pub auto_on_battery: AutoThrottleRuleDto,
    pub auto_on_cellular: AutoThrottleRuleDto,
}

/// Phase 21 — wire form of `copythat_settings::AutoThrottleRule`.
/// Tagged so the `Cap` variant's value round-trips via JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum AutoThrottleRuleDto {
    /// No override.
    Unchanged,
    /// Block the copy entirely (Shape's rate goes to 0).
    Pause,
    /// Cap to N bytes/s.
    Cap(u64),
}

/// Phase 21 — payload for `shape-rate-changed`. Lets the header
/// badge re-render without polling. `bytesPerSecond = 0` for the
/// "Off / paused" rendering, `null` when the shape transitioned
/// back to unlimited.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeRateDto {
    /// `None` (JSON `null`) = unlimited, `Some(0)` = paused,
    /// `Some(n)` = cap.
    pub bytes_per_second: Option<u64>,
    /// `"settings" | "schedule" | "auto-metered" | "auto-battery"
    /// | "auto-cellular" | "off"` — short label the header badge
    /// renders alongside the rate.
    pub source: &'static str,
}

/// Phase 19a — wire form of `copythat_settings::ScanSettings`.
/// Mirrors the config TOML group exposed by the Advanced → Scan
/// database pane in `SettingsModal`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanDto {
    pub hash_during_scan: bool,
    /// Absolute path, or `null` for "use default `<config-dir>/scans/`".
    pub database_path: Option<String>,
    pub auto_delete_after_days: u32,
    pub max_scans_to_keep: u32,
}

/// Phase 20 — one row of the resume modal. Mirrors
/// `copythat_journal::UnfinishedJob` with camelCase field naming
/// and string-formatted paths so the Svelte frontend can render
/// without Rust → JSON glue at every property access.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingResumeDto {
    pub row_id: u64,
    /// `"copy" | "move" | "delete" | "secure-delete" | "verify"`.
    pub kind: String,
    pub src_root: String,
    /// `null` for delete/shred jobs.
    pub dst_root: Option<String>,
    /// `"running" | "paused"` (terminal statuses are excluded by
    /// `Journal::unfinished`).
    pub status: String,
    pub started_at_ms: i64,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    /// Wall-clock millis of the latest per-file checkpoint, or `0`
    /// when no checkpoint landed (e.g. the engine died sub-50 ms).
    pub last_checkpoint_at_ms: i64,
}

/// Phase 19b — `snapshot-created` payload. Emitted when the engine
/// falls through to a filesystem snapshot because the live source was
/// locked. The frontend renders a "📷 Reading from <kind> snapshot"
/// badge on the active row until the file finishes.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCreatedDto {
    pub job_id: u64,
    /// `"vss" | "zfs" | "btrfs" | "apfs"` — see
    /// `copythat_snapshot::SnapshotKind::as_str`.
    pub kind: &'static str,
    pub original: String,
    pub snap_mount: String,
}

/// Phase 23 — `sparseness-not-supported` payload. Emitted when the
/// destination filesystem can't preserve a sparse source, so the
/// copy densified. `dst_fs` is a short lowercase filesystem name
/// (`"exfat"`, `"fat32"`, `"hfs+"`, `"unknown"`).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SparsenessNotSupportedDto {
    pub job_id: u64,
    pub dst_fs: String,
}

/// Phase 24 — `meta-translated-to-appledouble` payload. Emitted when
/// the destination filesystem couldn't accept the source's foreign
/// metadata streams (NTFS ADS / xattrs / resource fork) and the
/// engine fell through to an `._<filename>` AppleDouble sidecar.
/// `ext` is the source file's extension (`"docx"` / `"jpg"` /
/// `"none"`).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaTranslatedToAppleDoubleDto {
    pub job_id: u64,
    pub ext: String,
}

/// Phase 19a — `scan-started` payload.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanStartedDto {
    pub scan_id: String,
    pub root: String,
    pub db_path: String,
}

/// Phase 19a — `scan-progress` payload. Running counters emitted
/// every `ScanOptions::progress_every` committed files.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgressDto {
    pub scan_id: String,
    pub files_discovered: u64,
    pub bytes_discovered: u64,
    pub files_written: u64,
    pub files_hashed: u64,
}

/// Phase 19a — `scan-batch` payload, fired after each 1000-row
/// transaction flush. The `lastRelPath` doubles as a live "last
/// file I saw" status line.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBatchDto {
    pub scan_id: String,
    pub batch_size: u32,
    pub files_committed: u64,
    pub last_rel_path: String,
}

/// Phase 19a — `scan-completed` payload. Final counters + scan DB
/// path so the frontend can immediately trigger a copy from the
/// same database without a round-trip query.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanCompletedDto {
    pub scan_id: String,
    pub db_path: String,
    pub root: String,
    pub files: u64,
    pub bytes: u64,
    pub hashed_files: u64,
    pub duration_ms: u64,
}

/// Phase 19a — `scan-failed` payload.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFailedDto {
    pub scan_id: String,
    pub message: String,
}

/// Phase 19a — one row of `main.db::active_scans`, surfaced to the
/// frontend so it can offer to resume a pending scan at launch.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveScanDto {
    pub scan_id: String,
    pub db_path: String,
    pub job_id: Option<String>,
    pub created_at_ms: i64,
    /// `"Running" | "Paused" | "Complete" | "Cancelled" | "Failed"`.
    pub status: String,
}

// ---------------------------------------------------------------------
// Phase 25 — two-way sync DTOs.
// ---------------------------------------------------------------------

/// One configured sync pair, suitable for rendering a row in the
/// Sync tab's pair list.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncPairDto {
    pub id: String,
    pub label: String,
    pub left: String,
    pub right: String,
    /// `"two-way" | "mirror-left-to-right" | "mirror-right-to-left" |
    /// "contribute-left-to-right"`.
    pub mode: String,
    /// ISO-8601 UTC timestamp of the last run. Empty when never run.
    pub last_run_at: String,
    /// Short summary line — `"+3 / −1 / !2"`. Empty when never run.
    pub last_run_summary: String,
    /// `true` while a sync is actively running for this pair.
    pub running: bool,
    /// Phase 26 — `true` while the live-mirror watcher loop is
    /// active for this pair. The UI renders a green pulsing dot +
    /// "Watching" label when this flag is set.
    pub live_mirror: bool,
}

/// Fired once per `start_sync` call just before the runner spawns
/// the engine task. Gives the UI enough info to instantiate a
/// running-state row without round-tripping for the pair list.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStartedDto {
    pub pair_id: String,
    pub label: String,
    pub left: String,
    pub right: String,
    pub mode: String,
}

/// Fired at the boundary of each walk pass (one for each side).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncWalkDto {
    pub pair_id: String,
    /// `"left-to-right"` = walking left; `"right-to-left"` = walking right.
    pub side: &'static str,
    /// Populated only on `sync-walk-completed`; `None` on started.
    pub files_total: Option<u64>,
}

/// Fired after every action the engine finishes executing
/// (`Noop` / `Copy*` / `Delete*` / `KeepConflict`).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncActionDto {
    pub pair_id: String,
    pub relpath: String,
    /// `"noop" | "copy-left-to-right" | "copy-right-to-left" |
    /// "delete-left-to-right" | "delete-right-to-left" | "keep-conflict"`.
    pub kind: String,
}

/// Fired once per conflict preserved during the run.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncConflictDto {
    pub pair_id: String,
    pub relpath: String,
    /// `"concurrent-write" | "delete-edit" | "add-add" | "corrupt-equal"`.
    pub kind: String,
    /// `"left-to-right" | "right-to-left"`.
    pub winner_side: String,
    /// `"left-to-right" | "right-to-left"`.
    pub loser_side: String,
    /// Absolute path of the `name.sync-conflict-*-<host>.ext` file
    /// the engine wrote on the losing side. The UI surfaces this so
    /// the user can click through to inspect and resolve.
    pub loser_preservation_path: String,
}

/// Final report after a sync run ends normally.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncCompletedDto {
    pub pair_id: String,
    pub applied_left: u64,
    pub applied_right: u64,
    pub deleted_left: u64,
    pub deleted_right: u64,
    pub conflicts: u64,
    pub cancelled: bool,
    pub duration_ms: u64,
}

/// Payload for both `sync-cancelled` and `sync-failed`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncFailedDto {
    pub pair_id: String,
    pub message: String,
}

// ---------------------------------------------------------------------
// Phase 26 — live-mirror DTOs.
// ---------------------------------------------------------------------

/// Payload for `live-mirror-started` / `live-mirror-stopped`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveMirrorDto {
    pub pair_id: String,
    /// Left-side absolute path the watcher is rooted at. Empty on
    /// the stopped event.
    pub left: String,
}

/// Payload for `live-mirror-event` — one emission per debounced
/// filesystem event picked up by the watcher.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveMirrorEventDto {
    pub pair_id: String,
    /// Canonical subject path of the event. For a `Renamed` the
    /// destination side is reported (the sync engine reconciles
    /// against that).
    pub subject: String,
}

// --- Settings ⇄ DTO conversions ---------------------------------------

impl From<&copythat_settings::Settings> for SettingsDto {
    fn from(s: &copythat_settings::Settings) -> Self {
        Self {
            general: GeneralDto {
                language: s.general.language.clone(),
                theme: s.general.theme.as_str().to_string(),
                start_with_os: s.general.start_with_os,
                single_instance: s.general.single_instance,
                minimize_to_tray: s.general.minimize_to_tray,
                error_display_mode: s.general.error_display_mode.as_str().to_string(),
                paste_shortcut_enabled: s.general.paste_shortcut_enabled,
                paste_shortcut: s.general.paste_shortcut.clone(),
                clipboard_watcher_enabled: s.general.clipboard_watcher_enabled,
                auto_resume_interrupted: s.general.auto_resume_interrupted,
            },
            transfer: TransferDto {
                buffer_size_bytes: s.transfer.buffer_size_bytes as u64,
                verify: verify_wire(s.transfer.verify),
                concurrency: concurrency_wire(s.transfer.concurrency),
                reflink: reflink_wire(s.transfer.reflink),
                fsync_on_close: s.transfer.fsync_on_close,
                preserve_timestamps: s.transfer.preserve_timestamps,
                preserve_permissions: s.transfer.preserve_permissions,
                preserve_acls: s.transfer.preserve_acls,
                on_locked: s.transfer.on_locked.as_str().to_string(),
                preserve_sparseness: s.transfer.preserve_sparseness,
                preserve_security_metadata: s.transfer.preserve_security_metadata,
                preserve_motw: s.transfer.preserve_motw,
                preserve_posix_acls: s.transfer.preserve_posix_acls,
                preserve_selinux_contexts: s.transfer.preserve_selinux_contexts,
                preserve_resource_forks: s.transfer.preserve_resource_forks,
                appledouble_fallback: s.transfer.appledouble_fallback,
            },
            shell: ShellDto {
                context_menu_enabled: s.shell.context_menu_enabled,
                intercept_default_copy: s.shell.intercept_default_copy,
                notify_on_completion: s.shell.notify_on_completion,
            },
            secure_delete: SecureDeleteDto {
                method: s.secure_delete.method.as_str().to_string(),
                confirm_twice: s.secure_delete.confirm_twice,
            },
            advanced: AdvancedDto {
                log_level: s.advanced.log_level.as_str().to_string(),
                telemetry: s.advanced.telemetry,
                error_policy: match s.advanced.error_policy {
                    copythat_settings::ErrorPolicyChoice::Ask => ErrorPolicyDtoV2::Ask,
                    copythat_settings::ErrorPolicyChoice::Skip => ErrorPolicyDtoV2::Skip,
                    copythat_settings::ErrorPolicyChoice::Abort => ErrorPolicyDtoV2::Abort,
                    copythat_settings::ErrorPolicyChoice::RetryN {
                        max_attempts,
                        backoff_ms,
                    } => ErrorPolicyDtoV2::RetryN {
                        max_attempts,
                        backoff_ms,
                    },
                },
                history_retention_days: s.advanced.history_retention_days,
                database_path: s
                    .advanced
                    .database_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string()),
            },
            filters: FiltersDto {
                enabled: s.filters.enabled,
                include_globs: s.filters.include_globs.clone(),
                exclude_globs: s.filters.exclude_globs.clone(),
                min_size_bytes: s.filters.min_size_bytes,
                max_size_bytes: s.filters.max_size_bytes,
                min_mtime_unix_secs: s.filters.min_mtime_unix_secs,
                max_mtime_unix_secs: s.filters.max_mtime_unix_secs,
                skip_hidden: s.filters.skip_hidden,
                skip_system: s.filters.skip_system,
                skip_readonly: s.filters.skip_readonly,
            },
            updater: UpdaterDto {
                auto_check: s.updater.auto_check,
                channel: s.updater.channel.as_str().to_string(),
                last_check_unix_secs: s.updater.last_check_unix_secs,
                dismissed_version: s.updater.dismissed_version.clone(),
                check_interval_secs: s.updater.check_interval_secs,
            },
            scan: ScanDto {
                hash_during_scan: s.scan.hash_during_scan,
                database_path: s
                    .scan
                    .database_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string()),
                auto_delete_after_days: s.scan.auto_delete_after_days,
                max_scans_to_keep: s.scan.max_scans_to_keep,
            },
            network: NetworkDto {
                mode: s.network.mode.as_str().to_string(),
                fixed_bytes_per_second: s.network.fixed_bytes_per_second,
                schedule_spec: s.network.schedule_spec.clone(),
                auto_on_metered: auto_throttle_to_dto(s.network.auto_on_metered),
                auto_on_battery: auto_throttle_to_dto(s.network.auto_on_battery),
                auto_on_cellular: auto_throttle_to_dto(s.network.auto_on_cellular),
            },
            dnd: DndDto {
                spring_load_enabled: s.dnd.spring_load_enabled,
                spring_load_delay_ms: s.dnd.effective_spring_ms(),
                show_drag_thumbnails: s.dnd.show_drag_thumbnails,
                highlight_invalid_targets: s.dnd.highlight_invalid_targets,
            },
            path_translation: PathTranslationDto {
                enabled: s.path_translation.enabled,
                target_os: s.path_translation.target_os.as_str().to_string(),
                unicode_normalization: s
                    .path_translation
                    .unicode_normalization
                    .as_str()
                    .to_string(),
                line_endings: s.path_translation.line_endings.as_str().to_string(),
                reserved_name_strategy: s
                    .path_translation
                    .reserved_name_strategy
                    .as_str()
                    .to_string(),
                long_path_strategy: s.path_translation.long_path_strategy.as_str().to_string(),
                line_ending_allowlist: s.path_translation.line_ending_allowlist.clone(),
            },
        }
    }
}

fn auto_throttle_to_dto(r: copythat_settings::AutoThrottleRule) -> AutoThrottleRuleDto {
    use copythat_settings::AutoThrottleRule;
    match r {
        AutoThrottleRule::Unchanged => AutoThrottleRuleDto::Unchanged,
        AutoThrottleRule::Pause => AutoThrottleRuleDto::Pause,
        AutoThrottleRule::Cap { bytes_per_second } => AutoThrottleRuleDto::Cap(bytes_per_second),
    }
}

fn auto_throttle_from_dto(r: AutoThrottleRuleDto) -> copythat_settings::AutoThrottleRule {
    use copythat_settings::AutoThrottleRule;
    match r {
        AutoThrottleRuleDto::Unchanged => AutoThrottleRule::Unchanged,
        AutoThrottleRuleDto::Pause => AutoThrottleRule::Pause,
        AutoThrottleRuleDto::Cap(bytes_per_second) => AutoThrottleRule::Cap { bytes_per_second },
    }
}

fn parse_bandwidth_mode(s: &str) -> copythat_settings::BandwidthMode {
    use copythat_settings::BandwidthMode;
    match s {
        "fixed" => BandwidthMode::Fixed,
        "schedule" => BandwidthMode::Schedule,
        _ => BandwidthMode::Off,
    }
}

impl SettingsDto {
    /// Apply a DTO received from the frontend onto a fresh
    /// `Settings` instance. Unknown / malformed enum strings fall
    /// back to the `Default` — which keeps old frontends working
    /// against newer backends and vice versa. `telemetry` is
    /// unconditionally forced to `false` regardless of what the
    /// frontend sent, enforcing the "never on" contract at every
    /// boundary crossing.
    pub fn into_settings(self) -> copythat_settings::Settings {
        use copythat_settings::LockedFilePolicyChoice;
        use copythat_settings::*;
        let mut s = Settings::default();
        s.general.language = self.general.language;
        s.general.theme = match self.general.theme.as_str() {
            "light" => ThemePreference::Light,
            "dark" => ThemePreference::Dark,
            _ => ThemePreference::Auto,
        };
        s.general.start_with_os = self.general.start_with_os;
        s.general.single_instance = self.general.single_instance;
        s.general.minimize_to_tray = self.general.minimize_to_tray;
        s.general.error_display_mode = match self.general.error_display_mode.as_str() {
            "drawer" => ErrorDisplayMode::Drawer,
            _ => ErrorDisplayMode::Modal,
        };
        s.general.paste_shortcut_enabled = self.general.paste_shortcut_enabled;
        // Empty or whitespace-only combos fall back to the default to
        // avoid persisting a combo the plugin can't parse.
        let combo = self.general.paste_shortcut.trim();
        s.general.paste_shortcut = if combo.is_empty() {
            defaults::DEFAULT_PASTE_SHORTCUT.to_string()
        } else {
            combo.to_string()
        };
        s.general.clipboard_watcher_enabled = self.general.clipboard_watcher_enabled;
        s.general.auto_resume_interrupted = self.general.auto_resume_interrupted;

        s.transfer.buffer_size_bytes = self.transfer.buffer_size_bytes as usize;
        s.transfer.verify = parse_verify(&self.transfer.verify);
        s.transfer.concurrency = parse_concurrency(&self.transfer.concurrency);
        s.transfer.reflink = parse_reflink(&self.transfer.reflink);
        s.transfer.fsync_on_close = self.transfer.fsync_on_close;
        s.transfer.preserve_timestamps = self.transfer.preserve_timestamps;
        s.transfer.preserve_permissions = self.transfer.preserve_permissions;
        s.transfer.preserve_acls = self.transfer.preserve_acls;
        s.transfer.on_locked = LockedFilePolicyChoice::from_wire(&self.transfer.on_locked);
        s.transfer.preserve_sparseness = self.transfer.preserve_sparseness;
        s.transfer.preserve_security_metadata = self.transfer.preserve_security_metadata;
        s.transfer.preserve_motw = self.transfer.preserve_motw;
        s.transfer.preserve_posix_acls = self.transfer.preserve_posix_acls;
        s.transfer.preserve_selinux_contexts = self.transfer.preserve_selinux_contexts;
        s.transfer.preserve_resource_forks = self.transfer.preserve_resource_forks;
        s.transfer.appledouble_fallback = self.transfer.appledouble_fallback;

        s.shell.context_menu_enabled = self.shell.context_menu_enabled;
        s.shell.intercept_default_copy = self.shell.intercept_default_copy;
        s.shell.notify_on_completion = self.shell.notify_on_completion;

        s.secure_delete.method = parse_shred(&self.secure_delete.method);
        s.secure_delete.confirm_twice = self.secure_delete.confirm_twice;

        s.advanced.log_level = parse_log_level(&self.advanced.log_level);
        // Never honour a `telemetry: true` value, regardless of
        // source. Same invariant enforced in the settings crate at
        // deserialize time; reasserted here for defence-in-depth.
        s.advanced.telemetry = false;
        s.advanced.error_policy = match self.advanced.error_policy {
            ErrorPolicyDtoV2::Ask => ErrorPolicyChoice::Ask,
            ErrorPolicyDtoV2::Skip => ErrorPolicyChoice::Skip,
            ErrorPolicyDtoV2::Abort => ErrorPolicyChoice::Abort,
            ErrorPolicyDtoV2::RetryN {
                max_attempts,
                backoff_ms,
            } => ErrorPolicyChoice::RetryN {
                max_attempts,
                backoff_ms,
            },
        };
        s.advanced.history_retention_days = self.advanced.history_retention_days;
        s.advanced.database_path = self.advanced.database_path.map(std::path::PathBuf::from);

        s.filters = copythat_settings::FilterSettings {
            enabled: self.filters.enabled,
            include_globs: self.filters.include_globs,
            exclude_globs: self.filters.exclude_globs,
            min_size_bytes: self.filters.min_size_bytes,
            max_size_bytes: self.filters.max_size_bytes,
            min_mtime_unix_secs: self.filters.min_mtime_unix_secs,
            max_mtime_unix_secs: self.filters.max_mtime_unix_secs,
            skip_hidden: self.filters.skip_hidden,
            skip_system: self.filters.skip_system,
            skip_readonly: self.filters.skip_readonly,
        };

        s.updater = copythat_settings::UpdaterSettings {
            auto_check: self.updater.auto_check,
            channel: match self.updater.channel.as_str() {
                "beta" => copythat_settings::UpdateChannel::Beta,
                _ => copythat_settings::UpdateChannel::Stable,
            },
            last_check_unix_secs: self.updater.last_check_unix_secs,
            dismissed_version: self.updater.dismissed_version,
            check_interval_secs: self.updater.check_interval_secs,
        };

        s.scan = copythat_settings::ScanSettings {
            hash_during_scan: self.scan.hash_during_scan,
            database_path: self.scan.database_path.map(std::path::PathBuf::from),
            auto_delete_after_days: self.scan.auto_delete_after_days,
            max_scans_to_keep: self.scan.max_scans_to_keep,
        };

        s.network = copythat_settings::NetworkSettings {
            mode: parse_bandwidth_mode(&self.network.mode),
            fixed_bytes_per_second: self.network.fixed_bytes_per_second,
            schedule_spec: self.network.schedule_spec,
            auto_on_metered: auto_throttle_from_dto(self.network.auto_on_metered),
            auto_on_battery: auto_throttle_from_dto(self.network.auto_on_battery),
            auto_on_cellular: auto_throttle_from_dto(self.network.auto_on_cellular),
        };

        s.dnd = copythat_settings::DndSettings {
            spring_load_enabled: self.dnd.spring_load_enabled,
            spring_load_delay_ms: self.dnd.spring_load_delay_ms.clamp(
                copythat_settings::DND_MIN_SPRING_MS,
                copythat_settings::DND_MAX_SPRING_MS,
            ),
            show_drag_thumbnails: self.dnd.show_drag_thumbnails,
            highlight_invalid_targets: self.dnd.highlight_invalid_targets,
        };

        s.path_translation = copythat_settings::PathTranslationSettings {
            enabled: self.path_translation.enabled,
            target_os: copythat_settings::TargetOsChoice::from_wire(
                &self.path_translation.target_os,
            ),
            unicode_normalization: copythat_settings::NormalizationModeChoice::from_wire(
                &self.path_translation.unicode_normalization,
            ),
            line_endings: copythat_settings::LineEndingModeChoice::from_wire(
                &self.path_translation.line_endings,
            ),
            reserved_name_strategy: copythat_settings::ReservedNameChoice::from_wire(
                &self.path_translation.reserved_name_strategy,
            ),
            long_path_strategy: copythat_settings::LongPathChoice::from_wire(
                &self.path_translation.long_path_strategy,
            ),
            line_ending_allowlist: self.path_translation.line_ending_allowlist,
        };

        s
    }
}

impl From<&copythat_settings::ProfileInfo> for ProfileInfoDto {
    fn from(p: &copythat_settings::ProfileInfo) -> Self {
        Self {
            name: p.name.clone(),
            path: p.path.to_string_lossy().to_string(),
        }
    }
}

// --- String helpers ---------------------------------------------------

fn verify_wire(v: copythat_settings::VerifyChoice) -> String {
    use copythat_settings::VerifyChoice;
    match v {
        VerifyChoice::Off => "off",
        VerifyChoice::Crc32 => "crc32",
        VerifyChoice::Md5 => "md5",
        VerifyChoice::Sha1 => "sha1",
        VerifyChoice::Sha256 => "sha256",
        VerifyChoice::Sha512 => "sha512",
        VerifyChoice::XxHash3_64 => "xxhash3-64",
        VerifyChoice::XxHash3_128 => "xxhash3-128",
        VerifyChoice::Blake3 => "blake3",
    }
    .to_string()
}

fn parse_verify(s: &str) -> copythat_settings::VerifyChoice {
    use copythat_settings::VerifyChoice;
    match s {
        "crc32" => VerifyChoice::Crc32,
        "md5" => VerifyChoice::Md5,
        "sha1" => VerifyChoice::Sha1,
        "sha256" => VerifyChoice::Sha256,
        "sha512" => VerifyChoice::Sha512,
        "xxhash3-64" => VerifyChoice::XxHash3_64,
        "xxhash3-128" => VerifyChoice::XxHash3_128,
        "blake3" => VerifyChoice::Blake3,
        _ => VerifyChoice::Off,
    }
}

fn concurrency_wire(c: copythat_settings::ConcurrencyChoice) -> String {
    use copythat_settings::ConcurrencyChoice;
    match c {
        ConcurrencyChoice::Auto => "auto".to_string(),
        ConcurrencyChoice::Manual(n) => format!("manual-{n}"),
    }
}

fn parse_concurrency(s: &str) -> copythat_settings::ConcurrencyChoice {
    use copythat_settings::ConcurrencyChoice;
    if let Some(rest) = s.strip_prefix("manual-")
        && let Ok(n) = rest.parse::<u8>()
    {
        return ConcurrencyChoice::Manual(n);
    }
    ConcurrencyChoice::Auto
}

fn reflink_wire(r: copythat_settings::ReflinkPreference) -> String {
    use copythat_settings::ReflinkPreference;
    match r {
        ReflinkPreference::Prefer => "prefer",
        ReflinkPreference::Avoid => "avoid",
        ReflinkPreference::Disabled => "disabled",
    }
    .to_string()
}

fn parse_reflink(s: &str) -> copythat_settings::ReflinkPreference {
    use copythat_settings::ReflinkPreference;
    match s {
        "avoid" => ReflinkPreference::Avoid,
        "disabled" => ReflinkPreference::Disabled,
        _ => ReflinkPreference::Prefer,
    }
}

fn parse_shred(s: &str) -> copythat_settings::ShredMethodChoice {
    use copythat_settings::ShredMethodChoice;
    match s {
        "zero" => ShredMethodChoice::Zero,
        "random" => ShredMethodChoice::Random,
        "dod-7-pass" => ShredMethodChoice::DoD7Pass,
        "gutmann" => ShredMethodChoice::Gutmann,
        "nist-800-88" => ShredMethodChoice::Nist80088,
        _ => ShredMethodChoice::DoD3Pass,
    }
}

fn parse_log_level(s: &str) -> copythat_settings::LogLevel {
    use copythat_settings::LogLevel;
    match s {
        "off" => LogLevel::Off,
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    }
}
