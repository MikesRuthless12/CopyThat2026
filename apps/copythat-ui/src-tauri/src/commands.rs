//! Tauri commands — the thin glue between the Svelte frontend and
//! the `copythat-core` engine.
//!
//! Each command is kept as small as it can be: validate input,
//! translate to an engine call, spawn a runner task (for long-running
//! work), return an id or a DTO. Long-running work never blocks the
//! frontend: the `start_*` commands return the list of job ids as
//! soon as the queue knows about them, and progress flows back via
//! the Tauri event bus.

use std::path::{Path, PathBuf};
use std::str::FromStr;

use copythat_core::{CopyOptions, JobKind};
use tauri::{AppHandle, State};

use crate::ipc::{CopyOptionsDto, FileIconDto, JobDto};
use crate::shell::enqueue_jobs;
use crate::state::AppState;

/// Start one or more copy jobs. Each source path becomes its own
/// job; the destination is the same folder for all of them (the
/// frontend picks it via the dialog plugin).
///
/// Returns the list of newly-allocated job ids in the same order as
/// `sources`. The UI can cross-reference these with subsequent
/// `job-added` / `job-progress` events.
#[tauri::command]
pub async fn start_copy(
    sources: Vec<String>,
    destination: String,
    options: Option<CopyOptionsDto>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<u64>, String> {
    enqueue(
        JobKind::Copy,
        sources,
        destination,
        options.unwrap_or_default(),
        app,
        state,
    )
    .await
}

/// Start one or more move jobs. Same shape as `start_copy`.
#[tauri::command]
pub async fn start_move(
    sources: Vec<String>,
    destination: String,
    options: Option<CopyOptionsDto>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<u64>, String> {
    enqueue(
        JobKind::Move,
        sources,
        destination,
        options.unwrap_or_default(),
        app,
        state,
    )
    .await
}

async fn enqueue(
    kind: JobKind,
    sources: Vec<String>,
    destination: String,
    options: CopyOptionsDto,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<u64>, String> {
    if sources.is_empty() {
        return Err("at least one source path is required".to_string());
    }
    let dst_root = PathBuf::from(destination.trim());
    if dst_root.as_os_str().is_empty() {
        return Err("destination path is empty".to_string());
    }

    let copy_opts = apply_options(&options)?;
    let verifier = resolve_verifier(&options)?;
    let collision_policy = parse_collision_policy(options.collision.as_deref())?;
    let error_policy = parse_error_policy(options.on_error.as_ref());

    let srcs: Vec<PathBuf> = sources
        .into_iter()
        .map(|raw| PathBuf::from(raw.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .collect();
    if srcs.is_empty() {
        return Err("source path is empty".to_string());
    }
    Ok(enqueue_jobs(
        &app,
        state.inner(),
        kind,
        srcs,
        &dst_root,
        copy_opts,
        verifier,
        collision_policy,
        error_policy,
    ))
}

fn parse_collision_policy(raw: Option<&str>) -> Result<copythat_core::CollisionPolicy, String> {
    use copythat_core::CollisionPolicy;
    let Some(s) = raw else {
        return Ok(CollisionPolicy::default());
    };
    match s {
        "" => Ok(CollisionPolicy::default()),
        "skip" => Ok(CollisionPolicy::Skip),
        "overwrite" => Ok(CollisionPolicy::Overwrite),
        "overwrite-if-newer" => Ok(CollisionPolicy::OverwriteIfNewer),
        "keep-both" => Ok(CollisionPolicy::KeepBoth),
        "prompt" => Ok(CollisionPolicy::Prompt),
        other => Err(format!("unknown collision policy: {other}")),
    }
}

fn parse_error_policy(raw: Option<&crate::ipc::ErrorPolicyDto>) -> copythat_core::ErrorPolicy {
    use crate::ipc::ErrorPolicyDto;
    use copythat_core::ErrorPolicy;
    match raw {
        None | Some(ErrorPolicyDto::Ask) => ErrorPolicy::Ask,
        Some(ErrorPolicyDto::Skip) => ErrorPolicy::Skip,
        Some(ErrorPolicyDto::Abort) => ErrorPolicy::Abort,
        Some(ErrorPolicyDto::RetryN {
            max_attempts,
            backoff_ms,
        }) => ErrorPolicy::RetryN {
            max_attempts: *max_attempts,
            backoff_ms: *backoff_ms,
        },
    }
}

fn apply_options(dto: &CopyOptionsDto) -> Result<CopyOptions, String> {
    let mut opts = CopyOptions::default();
    if let Some(v) = dto.preserve_times {
        opts.preserve_times = v;
    }
    if let Some(v) = dto.preserve_permissions {
        opts.preserve_permissions = v;
    }
    if let Some(v) = dto.fsync_on_close {
        opts.fsync_on_close = v;
    }
    if let Some(v) = dto.follow_symlinks {
        opts.follow_symlinks = v;
    }
    Ok(opts)
}

fn resolve_verifier(dto: &CopyOptionsDto) -> Result<Option<copythat_core::Verifier>, String> {
    let Some(name) = dto.verify.as_deref() else {
        return Ok(None);
    };
    let name = name.trim();
    if name.is_empty() {
        return Ok(None);
    }
    let algo = copythat_hash::HashAlgorithm::from_str(name)
        .map_err(|e| format!("unknown verify algorithm: {e}"))?;
    Ok(Some(algo.verifier()))
}

#[tauri::command]
pub fn pause_job(id: u64, state: State<'_, AppState>) -> Result<(), String> {
    let job_id = job_id(id, &state)?;
    state.queue.pause_job(job_id);
    Ok(())
}

#[tauri::command]
pub fn resume_job(id: u64, state: State<'_, AppState>) -> Result<(), String> {
    let job_id = job_id(id, &state)?;
    state.queue.resume_job(job_id);
    Ok(())
}

#[tauri::command]
pub fn cancel_job(id: u64, state: State<'_, AppState>) -> Result<(), String> {
    let job_id = job_id(id, &state)?;
    state.queue.cancel_job(job_id);
    Ok(())
}

#[tauri::command]
pub fn remove_job(id: u64, state: State<'_, AppState>) -> Result<(), String> {
    let job_id = job_id(id, &state)?;
    state.queue.remove(job_id);
    Ok(())
}

#[tauri::command]
pub fn pause_all(state: State<'_, AppState>) -> Result<(), String> {
    for job in state.queue.snapshot() {
        state.queue.pause_job(job.id);
    }
    Ok(())
}

#[tauri::command]
pub fn resume_all(state: State<'_, AppState>) -> Result<(), String> {
    for job in state.queue.snapshot() {
        state.queue.resume_job(job.id);
    }
    Ok(())
}

#[tauri::command]
pub fn cancel_all(state: State<'_, AppState>) -> Result<(), String> {
    for job in state.queue.snapshot() {
        state.queue.cancel_job(job.id);
    }
    Ok(())
}

#[tauri::command]
pub fn list_jobs(state: State<'_, AppState>) -> Vec<JobDto> {
    state
        .queue
        .snapshot()
        .iter()
        .map(JobDto::from_job)
        .collect()
}

#[tauri::command]
pub fn globals(state: State<'_, AppState>) -> crate::ipc::GlobalsDto {
    crate::runner::build_globals(&state.queue)
}

/// Classify a path for the frontend to pick a Lucide icon. Ships
/// without a native file-icon bridge — Phase 7 extends this with
/// SHGetFileInfo / NSWorkspace / GIO lookups.
#[tauri::command]
pub fn file_icon(path: String) -> FileIconDto {
    crate::icon::classify(Path::new(&path))
}

/// Reveal a path in the platform's file manager. No-op + Err if
/// the path does not exist.
#[tauri::command]
pub fn reveal_in_folder(path: String) -> Result<(), String> {
    crate::reveal::reveal(Path::new(&path))
}

/// Return all translations for one locale. Falls back to `en` if
/// the requested locale is unknown.
#[tauri::command]
pub fn translations(locale: String) -> std::collections::HashMap<String, String> {
    crate::i18n::translations(&locale)
}

#[tauri::command]
pub fn available_locales() -> Vec<String> {
    crate::i18n::available_locales()
}

#[tauri::command]
pub fn system_locale() -> String {
    crate::i18n::system_locale()
}

fn job_id(id: u64, state: &State<'_, AppState>) -> Result<copythat_core::JobId, String> {
    state
        .queue
        .snapshot()
        .into_iter()
        .find(|j| j.id.as_u64() == id)
        .map(|j| j.id)
        .ok_or_else(|| format!("unknown job id: {id}"))
}

// ---------------------------------------------------------------------
// Phase 8 — error / collision / error-log commands
// ---------------------------------------------------------------------

/// Resolve an `error-raised` prompt. The Svelte modal calls this
/// with the user's choice; the registry fires the oneshot the
/// engine is awaiting on and logs the decision.
#[tauri::command]
pub fn resolve_error(
    id: u64,
    action: String,
    apply_to_all: Option<bool>,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let act = parse_error_action(&action)?;
    let resolved = state
        .errors
        .resolve(id, act, apply_to_all.unwrap_or(false))?;
    use tauri::Emitter;
    let _ = app.emit(
        crate::ipc::EVENT_ERROR_RESOLVED,
        crate::ipc::ErrorResolvedDto {
            id: resolved.id,
            job_id: resolved.job_id,
            action: crate::errors::action_name(resolved.action),
        },
    );
    Ok(())
}

/// Resolve a `collision-raised` prompt. `rename_to` is required
/// when `resolution == "rename"`; ignored otherwise.
#[tauri::command]
pub fn resolve_collision(
    id: u64,
    resolution: String,
    rename_to: Option<String>,
    apply_to_all: Option<bool>,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let res = parse_collision_resolution(&resolution, rename_to)?;
    let resolved = state
        .collisions
        .resolve(id, res, apply_to_all.unwrap_or(false))?;
    use tauri::Emitter;
    let _ = app.emit(
        crate::ipc::EVENT_COLLISION_RESOLVED,
        crate::ipc::CollisionResolvedDto {
            id: resolved.id,
            job_id: resolved.job_id,
            resolution: crate::collisions::resolution_name(&resolved.resolution),
        },
    );
    Ok(())
}

/// Snapshot of the in-memory error log, oldest-first. The footer
/// "errors" link opens a drawer that calls this.
#[tauri::command]
pub fn error_log(state: State<'_, AppState>) -> Vec<crate::ipc::LoggedErrorDto> {
    state
        .errors
        .log()
        .into_iter()
        .map(|e| crate::ipc::LoggedErrorDto {
            id: e.id,
            job_id: e.job_id,
            timestamp_ms: e.timestamp_ms,
            src: path_to_string(&e.src),
            dst: path_to_string(&e.dst),
            kind: e.kind,
            localized_key: e.localized_key,
            message: e.message,
            raw_os_error: e.raw_os_error,
            resolution: e.resolution,
        })
        .collect()
}

/// Wipe the in-memory log. Phase 9's SQLite history is untouched.
#[tauri::command]
pub fn clear_error_log(state: State<'_, AppState>) -> Result<(), String> {
    state.errors.clear_log();
    Ok(())
}

/// Export the log to a file. Two formats today; a Phase 12 user-
/// preference switch can pick the default. Writes atomically via
/// `write_then_rename`.
#[tauri::command]
pub fn error_log_export(
    format: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let entries = state.errors.log();
    let body = match format.as_str() {
        "csv" => crate::errors::export_csv(&entries),
        "txt" => crate::errors::export_txt(&entries),
        other => return Err(format!("unknown export format: {other}")),
    };
    let bytes = body.len() as u64;
    std::fs::write(&path, body).map_err(|e| format!("export write failed: {e}"))?;
    Ok(bytes)
}

/// Stub for the Phase 8 "Retry with elevated permissions" button.
/// Full UAC / `AuthorizationServices` / polkit wiring lands with
/// Phase 17 privilege separation. Until then this surfaces a
/// localised "not available yet" message — lets the UI carry the
/// button without us needing to hide it conditionally.
#[tauri::command]
pub fn retry_elevated(_id: u64) -> Result<(), String> {
    Err("retry-elevated-unavailable".to_string())
}

fn parse_error_action(action: &str) -> Result<copythat_core::ErrorAction, String> {
    match action {
        "retry" => Ok(copythat_core::ErrorAction::Retry),
        "skip" => Ok(copythat_core::ErrorAction::Skip),
        "abort" => Ok(copythat_core::ErrorAction::Abort),
        other => Err(format!("unknown error action: {other}")),
    }
}

fn parse_collision_resolution(
    resolution: &str,
    rename_to: Option<String>,
) -> Result<copythat_core::CollisionResolution, String> {
    match resolution {
        "skip" => Ok(copythat_core::CollisionResolution::Skip),
        "overwrite" => Ok(copythat_core::CollisionResolution::Overwrite),
        "rename" => {
            let name =
                rename_to.ok_or_else(|| "rename resolution requires rename_to".to_string())?;
            if name.is_empty() {
                return Err("rename_to must not be empty".to_string());
            }
            // Reject any directory component — keeps the user inside
            // the same parent folder and matches the engine's
            // `CollisionResolution::Rename` contract.
            if name.contains('/') || name.contains('\\') {
                return Err("rename_to must not contain a directory separator".to_string());
            }
            Ok(copythat_core::CollisionResolution::Rename(name))
        }
        "abort" => Ok(copythat_core::CollisionResolution::Abort),
        other => Err(format!("unknown collision resolution: {other}")),
    }
}

fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

// ---------------------------------------------------------------------
// Phase 9 — SQLite history commands
// ---------------------------------------------------------------------

/// Search the history. Frontend can refine with any subset of
/// filter fields; the handler forwards into
/// [`copythat_history::History::search`].
#[tauri::command]
pub async fn history_search(
    filter: Option<crate::ipc::HistoryFilterDto>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::ipc::HistoryJobDto>, String> {
    let history = require_history(&state)?;
    let filter = filter.unwrap_or_default();
    let f = copythat_history::HistoryFilter {
        started_since_ms: filter.started_since_ms,
        started_until_ms: filter.started_until_ms,
        kind: filter.kind,
        status: filter.status,
        text: filter.text,
        limit: filter.limit,
    };
    let rows = history.search(f).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(history_job_to_dto).collect())
}

/// Fetch the per-item rows attached to one history job. The
/// History drawer calls this when the user clicks a job.
#[tauri::command]
pub async fn history_items(
    row_id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::ipc::HistoryItemDto>, String> {
    let history = require_history(&state)?;
    let id = copythat_history::JobRowId(row_id);
    let rows = history.items_for(id).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(history_item_to_dto).collect())
}

/// Delete every job older than `days` days. Cascades to the
/// attached `items` rows via the FK rule.
#[tauri::command]
pub async fn history_purge(days: u32, state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let history = require_history(&state)?;
    history
        .purge_older_than(days)
        .await
        .map_err(|e| e.to_string())
}

/// Export the entire history (or a filtered subset) as CSV. Writes
/// to `path` on disk; returns the byte count written.
#[tauri::command]
pub async fn history_export_csv(
    path: String,
    filter: Option<crate::ipc::HistoryFilterDto>,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    let history = require_history(&state)?;
    let filter = filter.unwrap_or_default();
    let f = copythat_history::HistoryFilter {
        started_since_ms: filter.started_since_ms,
        started_until_ms: filter.started_until_ms,
        kind: filter.kind,
        status: filter.status,
        text: filter.text,
        limit: filter.limit,
    };
    let rows = history.search(f).await.map_err(|e| e.to_string())?;
    let body = copythat_history::export_csv(&rows);
    let bytes = body.len() as u64;
    std::fs::write(&path, body).map_err(|e| format!("history export write failed: {e}"))?;
    Ok(bytes)
}

/// Re-run the enqueue described by a history row. Walks the job's
/// `items` list and enqueues each `src` back into the runner with
/// the original `dst`'s parent as the destination root.
///
/// Intentionally minimal for Phase 9 — parses just what it needs
/// from the historical row rather than trying to restore every
/// detail of the original options. Phase 14 "resume interrupted"
/// will reuse this plumbing.
#[tauri::command]
pub async fn history_rerun(
    row_id: i64,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<u64>, String> {
    let history = require_history(&state)?;
    let id = copythat_history::JobRowId(row_id);
    let summary = history
        .get(id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "history row not found".to_string())?;

    let kind = match summary.kind.as_str() {
        "copy" => JobKind::Copy,
        "move" => JobKind::Move,
        other => return Err(format!("history rerun: unsupported job kind `{other}`")),
    };

    let sources = vec![summary.src_root];
    let dst_root = summary.dst_root;
    if dst_root.as_os_str().is_empty() {
        return Err("history rerun: destination path is empty".to_string());
    }
    // Re-run uses the engine defaults; the original row's opaque
    // `options_json` is ignored until Phase 12 can round-trip it.
    Ok(crate::shell::enqueue_jobs(
        &app,
        state.inner(),
        kind,
        sources,
        &dst_root,
        CopyOptions::default(),
        None,
        copythat_core::CollisionPolicy::default(),
        copythat_core::ErrorPolicy::Ask,
    ))
}

fn require_history(
    state: &tauri::State<'_, AppState>,
) -> Result<copythat_history::History, String> {
    state
        .inner()
        .history
        .clone()
        .ok_or_else(|| "history-unavailable".to_string())
}

fn history_job_to_dto(s: copythat_history::JobSummary) -> crate::ipc::HistoryJobDto {
    crate::ipc::HistoryJobDto {
        row_id: s.row_id,
        kind: s.kind,
        status: s.status,
        started_at_ms: s.started_at_ms,
        finished_at_ms: s.finished_at_ms,
        src_root: s.src_root.to_string_lossy().into_owned(),
        dst_root: s.dst_root.to_string_lossy().into_owned(),
        total_bytes: s.total_bytes,
        files_ok: s.files_ok,
        files_failed: s.files_failed,
        verify_algo: s.verify_algo,
        options_json: s.options_json,
    }
}

/// Phase 10 — lifetime aggregates for the Totals drawer.
#[tauri::command]
pub async fn history_totals(
    since_ms: Option<i64>,
    state: tauri::State<'_, AppState>,
) -> Result<crate::ipc::TotalsDto, String> {
    let history = require_history(&state)?;
    let t = history.totals(since_ms).await.map_err(|e| e.to_string())?;
    Ok(totals_to_dto(t))
}

/// Phase 10 — per-day buckets for the sparkline. `since_ms` should
/// be "30 days ago UTC-midnight" or similar; the handler forwards
/// verbatim.
#[tauri::command]
pub async fn history_daily(
    since_ms: i64,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::ipc::DayTotalDto>, String> {
    let history = require_history(&state)?;
    let rows = history
        .daily_totals(since_ms)
        .await
        .map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|d| crate::ipc::DayTotalDto {
            date_ms: d.date_ms,
            bytes: d.bytes,
            files: d.files,
            jobs: d.jobs,
        })
        .collect())
}

/// Phase 10 — reset every stored job/item. Cascades to items.
/// Returns the count of `jobs` rows that were deleted.
#[tauri::command]
pub async fn history_clear_all(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let history = require_history(&state)?;
    history.clear_all().await.map_err(|e| e.to_string())
}

fn totals_to_dto(t: copythat_history::Totals) -> crate::ipc::TotalsDto {
    crate::ipc::TotalsDto {
        bytes: t.bytes,
        files: t.files,
        jobs: t.jobs,
        errors: t.errors,
        duration_ms: t.duration_ms,
        by_kind: t
            .by_kind
            .into_iter()
            .map(|(kind, v)| crate::ipc::KindBreakdownDto {
                kind,
                bytes: v.bytes,
                files: v.files,
                jobs: v.jobs,
            })
            .collect(),
    }
}

fn history_item_to_dto(r: copythat_history::ItemRow) -> crate::ipc::HistoryItemDto {
    crate::ipc::HistoryItemDto {
        job_row_id: r.job_row_id,
        src: r.src.to_string_lossy().into_owned(),
        dst: r.dst.to_string_lossy().into_owned(),
        size: r.size,
        status: r.status,
        hash_hex: r.hash_hex,
        error_code: r.error_code,
        error_msg: r.error_msg,
        timestamp_ms: r.timestamp_ms,
    }
}
