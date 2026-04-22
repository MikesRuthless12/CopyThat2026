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
use tauri::{AppHandle, Emitter, State};

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
    eprintln!(
        "[start_copy] begin sources={} dst={}",
        sources.join(" | "),
        destination
    );
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
        return Err("err-source-required".to_string());
    }
    let dst_root = PathBuf::from(destination.trim());
    if dst_root.as_os_str().is_empty() {
        return Err("err-destination-empty".to_string());
    }

    // Phase 12: per-enqueue `CopyOptionsDto` overrides come from the
    // frontend's drop-dialog and still win (highest precedence);
    // anything the UI didn't set falls back to the live Settings
    // snapshot so defaults hot-reload from Settings → General /
    // Transfer without a restart. `effective_buffer_size` clamps to
    // the engine's window.
    let settings = state.settings_snapshot();
    let copy_opts = apply_options(&options, &settings)?;
    let verifier = resolve_verifier(&options, &settings)?;
    let collision_policy = parse_collision_policy(options.collision.as_deref())?;
    let error_policy = parse_error_policy(options.on_error.as_ref(), &settings);
    let tree_concurrency = resolve_concurrency(&settings);
    // Phase 14a: filters only make sense for Copy jobs (Move is
    // rename-with-copy-fallback; filtering the fallback copy while
    // the rename path ignored them would be a surprising split).
    let filters = if matches!(kind, JobKind::Copy) {
        resolve_filters(&settings)
    } else {
        None
    };

    let srcs: Vec<PathBuf> = sources
        .into_iter()
        .map(|raw| PathBuf::from(raw.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .collect();
    if srcs.is_empty() {
        return Err("err-source-empty".to_string());
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
        tree_concurrency,
        filters,
    ))
}

/// Phase 13c — resolve `ConcurrencyChoice` from settings into a
/// concrete worker count. `Auto` asks `copythat_platform` for a
/// storage-class-aware recommendation (SSD → up to 8; spinning → 1
/// to avoid seek thrash). `Manual(n)` clamps to `[1, 16]` via the
/// settings crate's own `resolved()`.
fn resolve_concurrency(settings: &copythat_settings::Settings) -> Option<usize> {
    use copythat_settings::ConcurrencyChoice;
    let auto_hint = platform_auto_concurrency_hint();
    let n = match settings.transfer.concurrency {
        ConcurrencyChoice::Auto => auto_hint,
        ConcurrencyChoice::Manual(_) => settings.transfer.concurrency.resolved(auto_hint),
    };
    Some(n as usize)
}

fn platform_auto_concurrency_hint() -> u8 {
    // `copythat-platform::recommend_concurrency` takes src + dst
    // paths; at enqueue time we don't have a single representative
    // src/dst pair (we have a list). Use a simple 4-worker default
    // that matches `DEFAULT_TREE_CONCURRENCY` — tuned further once
    // Phase 13c measures whether bumping helps many-small-file
    // trees.
    4
}

/// Phase 14a — translate the persisted `FilterSettings` into the
/// engine's `FilterSet`, or `None` when filtering is disabled or
/// every field is at its default. Unix epoch seconds become
/// `SystemTime` via `UNIX_EPOCH ± abs(secs)` so negative (pre-1970)
/// values translate losslessly.
fn resolve_filters(settings: &copythat_settings::Settings) -> Option<copythat_core::FilterSet> {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    let f = &settings.filters;
    if f.is_effectively_empty() {
        return None;
    }
    let to_system_time = |secs: i64| -> SystemTime {
        if secs >= 0 {
            UNIX_EPOCH + Duration::from_secs(secs as u64)
        } else {
            UNIX_EPOCH - Duration::from_secs(secs.unsigned_abs())
        }
    };
    Some(copythat_core::FilterSet {
        include_globs: f.include_globs.clone(),
        exclude_globs: f.exclude_globs.clone(),
        min_size_bytes: f.min_size_bytes,
        max_size_bytes: f.max_size_bytes,
        min_mtime: f.min_mtime_unix_secs.map(to_system_time),
        max_mtime: f.max_mtime_unix_secs.map(to_system_time),
        skip_hidden: f.skip_hidden,
        skip_system: f.skip_system,
        skip_readonly: f.skip_readonly,
    })
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

fn parse_error_policy(
    raw: Option<&crate::ipc::ErrorPolicyDto>,
    settings: &copythat_settings::Settings,
) -> copythat_core::ErrorPolicy {
    use crate::ipc::ErrorPolicyDto;
    use copythat_core::ErrorPolicy;
    match raw {
        Some(ErrorPolicyDto::Ask) => ErrorPolicy::Ask,
        Some(ErrorPolicyDto::Skip) => ErrorPolicy::Skip,
        Some(ErrorPolicyDto::Abort) => ErrorPolicy::Abort,
        Some(ErrorPolicyDto::RetryN {
            max_attempts,
            backoff_ms,
        }) => ErrorPolicy::RetryN {
            max_attempts: *max_attempts,
            backoff_ms: *backoff_ms,
        },
        None => match settings.advanced.error_policy {
            copythat_settings::ErrorPolicyChoice::Ask => ErrorPolicy::Ask,
            copythat_settings::ErrorPolicyChoice::Skip => ErrorPolicy::Skip,
            copythat_settings::ErrorPolicyChoice::Abort => ErrorPolicy::Abort,
            copythat_settings::ErrorPolicyChoice::RetryN {
                max_attempts,
                backoff_ms,
            } => ErrorPolicy::RetryN {
                max_attempts,
                backoff_ms,
            },
        },
    }
}

fn apply_options(
    dto: &CopyOptionsDto,
    settings: &copythat_settings::Settings,
) -> Result<CopyOptions, String> {
    // Start from the engine's `CopyOptions::default()`, then layer on
    // Settings (Phase 12 baseline), then the per-enqueue DTO (UI
    // override). DTO-level `Some(_)` always wins so individual copies
    // can still opt out of Settings on a case-by-case basis.
    let mut opts = CopyOptions {
        buffer_size: settings.transfer.effective_buffer_size(),
        fsync_on_close: settings.transfer.fsync_on_close,
        preserve_times: settings.transfer.preserve_timestamps,
        preserve_permissions: settings.transfer.preserve_permissions,
        strategy: match settings.transfer.reflink {
            copythat_settings::ReflinkPreference::Prefer => copythat_core::CopyStrategy::Auto,
            copythat_settings::ReflinkPreference::Avoid => copythat_core::CopyStrategy::NoReflink,
            copythat_settings::ReflinkPreference::Disabled => {
                copythat_core::CopyStrategy::AlwaysAsync
            }
        },
        ..Default::default()
    };

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

fn resolve_verifier(
    dto: &CopyOptionsDto,
    settings: &copythat_settings::Settings,
) -> Result<Option<copythat_core::Verifier>, String> {
    // Per-enqueue override takes precedence; otherwise read from
    // Settings → Transfer → Verify.
    let name_override = dto
        .verify
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let name = match name_override {
        Some(n) => Some(n.to_string()),
        None => settings
            .transfer
            .verify
            .as_algorithm()
            .map(|s| s.to_string()),
    };
    let Some(name) = name else {
        return Ok(None);
    };
    let algo = copythat_hash::HashAlgorithm::from_str(&name)
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
        return Err("err-destination-empty".to_string());
    }
    // Re-run uses the engine defaults; the original row's opaque
    // `options_json` is ignored until Phase 12 can round-trip it.
    // Concurrency still honours the live settings so a user who's
    // bumped the worker count sees it applied on rerun.
    let settings = state.settings_snapshot();
    let tree_concurrency = resolve_concurrency(&settings);
    // Rerun inherits the live `Settings.filters` — if the user has
    // configured a filter set since the original job ran, the rerun
    // honours it. History rows don't carry the filter snapshot (yet).
    let filters = if matches!(kind, JobKind::Copy) {
        resolve_filters(&settings)
    } else {
        None
    };
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
        tree_concurrency,
        filters,
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

// ---------------------------------------------------------------------
// Phase 12 — Settings + Profile commands
// ---------------------------------------------------------------------

/// Snapshot the live settings. Never fails — missing backing file
/// means the store is holding defaults.
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> crate::ipc::SettingsDto {
    let s = state.settings_snapshot();
    (&s).into()
}

/// Replace the live settings with a frontend-authored blob. Persists
/// to `settings_path` atomically; subsequent enqueues read the new
/// values from the in-memory `RwLock` without a restart. Returns the
/// resulting DTO so the frontend can re-bind controls in one step
/// (avoids a racy round-trip through a follow-up `get_settings`).
#[tauri::command]
pub fn update_settings(
    dto: crate::ipc::SettingsDto,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<crate::ipc::SettingsDto, String> {
    let next = dto.into_settings();
    // Persist first — we'd rather keep the old in-memory value if
    // disk write fails than lie to the user that the save succeeded.
    let path = state.settings_path.as_ref();
    if !path.as_os_str().is_empty() {
        next.save_to(path).map_err(|e| e.to_string())?;
    }
    // Snapshot the prior shortcut state *before* we swap — lets us
    // decide whether to rebind without racing with the live lock.
    let (prev_enabled, prev_combo, prev_watcher_enabled) = {
        let prev = state
            .settings
            .read()
            .map_err(|_| "settings-lock-poisoned".to_string())?;
        (
            prev.general.paste_shortcut_enabled,
            prev.general.paste_shortcut.clone(),
            prev.general.clipboard_watcher_enabled,
        )
    };
    {
        let mut live = state
            .settings
            .write()
            .map_err(|_| "settings-lock-poisoned".to_string())?;
        *live = next.clone();
    }
    // Rebind the paste hotkey only when its state actually changed —
    // re-registering the same combo can fail on some platforms and
    // we'd rather the whole `update_settings` call stay green if the
    // user flipped an unrelated field.
    let next_enabled = next.general.paste_shortcut_enabled;
    let next_combo = next.general.paste_shortcut.clone();
    if prev_enabled != next_enabled || prev_combo != next_combo {
        if let Err(e) =
            crate::global_paste::rebind_paste_shortcut(&app, &prev_combo, &next_combo, next_enabled)
        {
            // Don't fail `update_settings` — the new combo is on disk;
            // the user just can't use it until they fix the conflict.
            // A toast through `toast-shortcut-rebind-failed` would be
            // nice but the DTO doesn't carry toasts back.
            eprintln!("[paste-hotkey] rebind failed: {e}");
        }
    }
    // Start / stop the clipboard watcher to match the toggle. Drop
    // semantics on `WatcherHandle` stop the prior task within one
    // poll interval; storing a fresh handle leaves the old one
    // owned by `slot` briefly, which is fine.
    let next_watcher_enabled = next.general.clipboard_watcher_enabled;
    if prev_watcher_enabled != next_watcher_enabled {
        if let Ok(mut slot) = state.clipboard_watcher.lock() {
            if next_watcher_enabled {
                *slot = Some(crate::clipboard_watcher::spawn(app.clone()));
            } else {
                *slot = None;
            }
        }
    }
    Ok((&next).into())
}

/// Replace the live settings with `Settings::default()` and persist.
#[tauri::command]
pub fn reset_settings(state: State<'_, AppState>) -> Result<crate::ipc::SettingsDto, String> {
    let next = copythat_settings::Settings::default();
    let path = state.settings_path.as_ref();
    if !path.as_os_str().is_empty() {
        next.save_to(path).map_err(|e| e.to_string())?;
    }
    {
        let mut live = state
            .settings
            .write()
            .map_err(|_| "settings-lock-poisoned".to_string())?;
        *live = next.clone();
    }
    Ok((&next).into())
}

/// Debug hook used by the Phase 12 smoke test. Returns the clamped
/// buffer size the engine would actually use given the current
/// `transfer.buffer_size_bytes`. Lets the test round-trip "change
/// to 4 MiB" without spinning up a real copy.
#[tauri::command]
pub fn effective_buffer_size(state: State<'_, AppState>) -> u64 {
    state.settings_snapshot().transfer.effective_buffer_size() as u64
}

/// Phase 14 — free bytes available on the volume backing `path`.
/// Powers the preflight check in the UI. Returns `0` when the probe
/// can't resolve the volume (unmounted, permission denied, UNC host
/// offline) so the UI falls through to "unknown — proceed".
#[tauri::command]
pub async fn destination_free_bytes(path: String) -> Result<u64, String> {
    let p = std::path::PathBuf::from(path);
    let moved = p.clone();
    tokio::task::spawn_blocking(move || copythat_platform::free_space_bytes(&moved).unwrap_or(0))
        .await
        .map_err(|e| e.to_string())
}

/// Phase 14 — recursive total size of `paths`. Files are stat'd
/// directly; directories are walked depth-first. The walk bails with
/// `Err("too-large")` once it's counted past `HARD_LIMIT_ENTRIES`
/// entries so a 10 M-file tree doesn't stall the preflight dialog.
/// The UI falls through to "unknown — proceed with warning" on
/// `too-large`; the engine-side reserve check is still the final
/// safety net.
#[tauri::command]
pub async fn path_total_bytes(paths: Vec<String>) -> Result<u64, String> {
    // Whole-drive walks were spending 30+ seconds in here before
    // the preflight dialog ever appeared, which left the Start
    // button looking frozen. Cap at a small entry count + a hard
    // 2-second wall-clock budget; past either, bail with
    // `too-large` and let the UI fall through to "size unknown,
    // proceed with warning".
    const HARD_LIMIT_ENTRIES: u64 = 20_000;
    const HARD_LIMIT_MS: u128 = 2_000;
    tokio::task::spawn_blocking(move || {
        let started = std::time::Instant::now();
        let mut total: u64 = 0;
        let mut entries: u64 = 0;
        for p in paths {
            if started.elapsed().as_millis() >= HARD_LIMIT_MS {
                return Err("too-large".to_string());
            }
            let root = std::path::PathBuf::from(p);
            total = total.saturating_add(walk_size(&root, &mut entries, HARD_LIMIT_ENTRIES)?);
        }
        Ok(total)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Phase 16 — recursive file enumerator for the pre-seeded activity
/// list. Walks every source path depth-first and returns one
/// `{ path, size }` entry per *file* (directories are skipped; the
/// engine creates them as a side-effect of its per-file copies).
/// Capped at `HARD_LIMIT` entries so a 10 M-file tree doesn't seize
/// up the UI. On overflow, returns whatever had been collected
/// before the cap + an `overflow: true` marker so the frontend can
/// fall back to the old lazy-append mode for truly huge trees.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeFileDto {
    pub path: String,
    pub size: u64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeEnumerationDto {
    pub files: Vec<TreeFileDto>,
    pub overflow: bool,
}

/// Phase 19 — streaming enumerator. Emits `enumeration-progress`
/// Tauri events every ~200 ms with the current file count so the
/// DropStagingDialog can show a live counter next to the Start
/// button ("Counting files… 47,312") instead of a silent spinner.
/// Same 10 000-entry cap as before — beyond that we stop walking
/// and set `overflow: true`.
pub const EVENT_ENUMERATION_PROGRESS: &str = "enumeration-progress";

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumerationProgressDto {
    pub count: u64,
    pub done: bool,
    pub overflow: bool,
}

#[tauri::command]
pub async fn enumerate_tree_files(
    paths: Vec<String>,
    app: AppHandle,
) -> Result<TreeEnumerationDto, String> {
    eprintln!("[enumerate_tree_files] start paths={}", paths.join(" | "));
    // Matches the frontend ACTIVITY_LIMIT. Past this the `overflow`
    // flag tells the UI to skip pre-seed; the engine's own walker
    // keeps copying every file regardless of this cap.
    const HARD_LIMIT: usize = 250_000;
    // Emit progress in count-based batches so the IPC bus + the UI
    // renderer don't get slammed. 500 files is big enough that a
    // typical SSD walk fires ~20 ticks in the 10 k cap range — fast
    // enough to feel live, slow enough to stay invisible on the
    // event queue.
    const PROGRESS_EMIT_EVERY: usize = 500;

    tokio::task::spawn_blocking(move || -> Result<TreeEnumerationDto, String> {
        let mut files: Vec<TreeFileDto> = Vec::with_capacity(2_048);
        let mut overflow = false;
        let mut last_emitted_count: usize = 0;
        let mut ctx = StreamingCtx {
            app: &app,
            last_emitted_count: &mut last_emitted_count,
            emit_every: PROGRESS_EMIT_EVERY,
        };
        for p in paths {
            if overflow {
                break;
            }
            walk_files_streaming(
                &std::path::PathBuf::from(p),
                &mut files,
                HARD_LIMIT,
                &mut overflow,
                &mut ctx,
            );
        }
        // Final tick so the UI's counter reflects the complete total
        // even if the last incremental tick got suppressed by the
        // rate limiter.
        let _ = app.emit(
            EVENT_ENUMERATION_PROGRESS,
            EnumerationProgressDto {
                count: files.len() as u64,
                done: true,
                overflow,
            },
        );
        eprintln!(
            "[enumerate_tree_files] done count={} overflow={}",
            files.len(),
            overflow
        );
        Ok(TreeEnumerationDto { files, overflow })
    })
    .await
    .map_err(|e| e.to_string())?
}

struct StreamingCtx<'a> {
    app: &'a AppHandle,
    last_emitted_count: &'a mut usize,
    emit_every: usize,
}

impl StreamingCtx<'_> {
    fn tick(&mut self, count: usize, overflow: bool) {
        // Count-based batching — only fire when we've added at
        // least `emit_every` files since the last emit. The final
        // emit at the end of the walk closes the gap so the UI's
        // counter lands on the real total.
        if count < *self.last_emitted_count + self.emit_every {
            return;
        }
        let _ = self.app.emit(
            EVENT_ENUMERATION_PROGRESS,
            EnumerationProgressDto {
                count: count as u64,
                done: false,
                overflow,
            },
        );
        *self.last_emitted_count = count;
    }
}

fn walk_files_streaming(
    path: &std::path::Path,
    out: &mut Vec<TreeFileDto>,
    limit: usize,
    overflow: &mut bool,
    ctx: &mut StreamingCtx,
) {
    if *overflow || out.len() >= limit {
        *overflow = true;
        return;
    }
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return,
    };
    if meta.file_type().is_symlink() {
        return;
    }
    if meta.is_file() {
        out.push(TreeFileDto {
            path: path.to_string_lossy().into_owned(),
            size: meta.len(),
        });
        ctx.tick(out.len(), *overflow);
        return;
    }
    if !meta.is_dir() {
        return;
    }
    walk_dir_entries_streaming(path, out, limit, overflow, ctx);
}

fn walk_dir_entries_streaming(
    path: &std::path::Path,
    out: &mut Vec<TreeFileDto>,
    limit: usize,
    overflow: &mut bool,
    ctx: &mut StreamingCtx,
) {
    let rd = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return,
    };
    for entry in rd.flatten() {
        if *overflow || out.len() >= limit {
            *overflow = true;
            return;
        }
        let ft = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_symlink() {
            continue;
        }
        if ft.is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            out.push(TreeFileDto {
                path: entry.path().to_string_lossy().into_owned(),
                size,
            });
            ctx.tick(out.len(), *overflow);
        } else if ft.is_dir() {
            walk_files_streaming(&entry.path(), out, limit, overflow, ctx);
            if *overflow {
                return;
            }
        }
    }
}

/// Phase 15 — lightweight per-path metadata for the source-list
/// ordering UI. Returns one `{ is_dir, size }` per input path, in
/// input order. `size` is the file size for files and the recursive
/// tree size for directories (using the same HARD_LIMIT walk as
/// `path_sizes_individual`). `is_dir` is false for files, true for
/// directories, and false for anything we can't classify (broken
/// symlinks, stale UNC shares).
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathMetaDto {
    pub is_dir: bool,
    pub size: u64,
}

#[tauri::command]
pub async fn path_metadata(paths: Vec<String>) -> Result<Vec<PathMetaDto>, String> {
    eprintln!("[path_metadata] start paths={}", paths.join(" | "));
    // The DropStagingDialog fires this command as part of the
    // file-order UI, so it has to return within ~2 s even when a
    // user dropped `D:\` with millions of files. A small entry cap
    // + a per-source wall-clock budget keeps the modal responsive;
    // any directory too big to measure lands as `size: u64::MAX`
    // and the UI renders "too large to count".
    const HARD_LIMIT_ENTRIES: u64 = 20_000;
    const HARD_LIMIT_MS: u128 = 2_000;
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::with_capacity(paths.len());
        for p in paths {
            let root = std::path::PathBuf::from(&p);
            let md = std::fs::symlink_metadata(&root).ok();
            let is_dir = md.as_ref().is_some_and(|m| m.is_dir());
            let size = if let Some(m) = md.as_ref() {
                if m.is_file() {
                    m.len()
                } else if m.is_dir() {
                    let mut entries: u64 = 0;
                    let started = std::time::Instant::now();
                    match walk_size_timed(
                        &root,
                        &mut entries,
                        HARD_LIMIT_ENTRIES,
                        started,
                        HARD_LIMIT_MS,
                    ) {
                        Ok(n) => n,
                        Err(e) if e == "too-large" || e == "time-budget" => u64::MAX,
                        Err(_) => 0,
                    }
                } else {
                    0
                }
            } else {
                0
            };
            out.push(PathMetaDto { is_dir, size });
        }
        eprintln!("[path_metadata] done");
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Walk variant that also enforces a wall-clock budget. Returns
/// the `"time-budget"` sentinel on deadline so the caller can
/// render "too large to count" rather than hanging the modal.
fn walk_size_timed(
    path: &std::path::Path,
    entries: &mut u64,
    limit: u64,
    started: std::time::Instant,
    budget_ms: u128,
) -> Result<u64, String> {
    if started.elapsed().as_millis() >= budget_ms {
        return Err("time-budget".to_string());
    }
    *entries = entries.saturating_add(1);
    if *entries > limit {
        return Err("too-large".to_string());
    }
    let meta = std::fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if meta.is_file() {
        return Ok(meta.len());
    }
    if meta.file_type().is_symlink() || !meta.is_dir() {
        return Ok(0);
    }
    let mut sum: u64 = 0;
    let rd = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return Ok(0),
    };
    for entry in rd.flatten() {
        sum = sum.saturating_add(walk_size_timed(
            &entry.path(),
            entries,
            limit,
            started,
            budget_ms,
        )?);
    }
    Ok(sum)
}

/// Phase 14e — per-path size probe for the subset picker. Returns
/// one `u64` per input path, in input order. Individual entries
/// that fail to stat (permission denied, stale UNC share) land as
/// `0` rather than aborting the whole batch. Uses the same
/// `HARD_LIMIT_ENTRIES` cap per entry as `path_total_bytes`; any
/// entry that blows it becomes `u64::MAX` so the UI can render
/// "too large to count" for that row.
#[tauri::command]
pub async fn path_sizes_individual(paths: Vec<String>) -> Result<Vec<u64>, String> {
    const HARD_LIMIT_ENTRIES: u64 = 200_000;
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::with_capacity(paths.len());
        for p in paths {
            let root = std::path::PathBuf::from(p);
            let mut entries: u64 = 0;
            let sz = match walk_size(&root, &mut entries, HARD_LIMIT_ENTRIES) {
                Ok(n) => n,
                Err(e) if e == "too-large" => u64::MAX,
                Err(_) => 0,
            };
            out.push(sz);
        }
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn walk_size(path: &std::path::Path, entries: &mut u64, limit: u64) -> Result<u64, String> {
    *entries = entries.saturating_add(1);
    if *entries > limit {
        return Err("too-large".to_string());
    }
    let meta = std::fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if meta.is_file() {
        return Ok(meta.len());
    }
    if meta.file_type().is_symlink() {
        return Ok(0);
    }
    if !meta.is_dir() {
        return Ok(0);
    }
    let mut sum: u64 = 0;
    let rd = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        // Permission-denied on a sub-tree is non-fatal for preflight
        // — the engine will surface it as a per-file error. Count
        // what we can see.
        Err(_) => return Ok(0),
    };
    for entry in rd.flatten() {
        sum = sum.saturating_add(walk_size(&entry.path(), entries, limit)?);
    }
    Ok(sum)
}

#[tauri::command]
pub fn list_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<crate::ipc::ProfileInfoDto>, String> {
    let entries = state.profiles.list().map_err(|e| e.to_string())?;
    Ok(entries.iter().map(|p| p.into()).collect())
}

#[tauri::command]
pub fn save_profile(
    name: String,
    state: State<'_, AppState>,
) -> Result<crate::ipc::ProfileInfoDto, String> {
    let settings = state.settings_snapshot();
    let info = state
        .profiles
        .save_replacing(&name, &settings)
        .map_err(|e| e.to_string())?;
    Ok((&info).into())
}

#[tauri::command]
pub fn load_profile(
    name: String,
    state: State<'_, AppState>,
) -> Result<crate::ipc::SettingsDto, String> {
    let profile = state.profiles.load(&name).map_err(|e| e.to_string())?;
    // Loading a profile also activates it — persist + publish on the
    // live settings. Saves the caller a follow-up `update_settings`.
    let path = state.settings_path.as_ref();
    if !path.as_os_str().is_empty() {
        profile.save_to(path).map_err(|e| e.to_string())?;
    }
    {
        let mut live = state
            .settings
            .write()
            .map_err(|_| "settings-lock-poisoned".to_string())?;
        *live = profile.clone();
    }
    Ok((&profile).into())
}

#[tauri::command]
pub fn delete_profile(name: String, state: State<'_, AppState>) -> Result<(), String> {
    state.profiles.delete(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_profile(
    name: String,
    dest: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .profiles
        .export(&name, std::path::Path::new(&dest))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_profile(
    name: String,
    src: String,
    state: State<'_, AppState>,
) -> Result<crate::ipc::ProfileInfoDto, String> {
    let info = state
        .profiles
        .import(&name, std::path::Path::new(&src))
        .map_err(|e| e.to_string())?;
    Ok((&info).into())
}

/// Post-completion action. The frontend calls this once the globals
/// tick shows every job has finished. `action` is one of
/// `keep-open | exit | shutdown | logoff | sleep`. Anything else is
/// a no-op (keeps the app open) so a typo can't nuke a user's work.
///
/// `shutdown` schedules a 30-second delayed shutdown so the user has
/// a moment to cancel (`shutdown /a` in a terminal) if they change
/// their mind; `logoff` and `sleep` fire immediately.
#[tauri::command]
pub fn post_completion_action(action: String, app: AppHandle) -> Result<(), String> {
    use std::process::Command;
    match action.as_str() {
        "keep-open" => Ok(()),
        "exit" => {
            app.exit(0);
            Ok(())
        }
        #[cfg(target_os = "windows")]
        "shutdown" => Command::new("shutdown")
            .args(["/s", "/t", "30"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(target_os = "windows")]
        "logoff" => Command::new("shutdown")
            .args(["/l"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(target_os = "windows")]
        "sleep" => Command::new("rundll32.exe")
            .args(["powrprof.dll,SetSuspendState", "0,1,0"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(target_os = "macos")]
        "shutdown" => Command::new("osascript")
            .args(["-e", "tell app \"System Events\" to shut down"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(target_os = "macos")]
        "logoff" => Command::new("osascript")
            .args(["-e", "tell app \"System Events\" to log out"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(target_os = "macos")]
        "sleep" => Command::new("osascript")
            .args(["-e", "tell app \"System Events\" to sleep"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(all(unix, not(target_os = "macos")))]
        "shutdown" => Command::new("systemctl")
            .args(["poweroff"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(all(unix, not(target_os = "macos")))]
        "logoff" => Command::new("loginctl")
            .args(["terminate-user", &std::env::var("USER").unwrap_or_default()])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        #[cfg(all(unix, not(target_os = "macos")))]
        "sleep" => Command::new("systemctl")
            .args(["suspend"])
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        _ => Ok(()),
    }
}
