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
use crate::runner::{RunJob, emit_job_added, run_job};
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

    let mut ids = Vec::with_capacity(sources.len());
    for raw in sources {
        let src = PathBuf::from(raw.trim());
        if src.as_os_str().is_empty() {
            return Err("source path is empty".to_string());
        }
        // Destination for this source: append its basename under the
        // dst root so a drop of multiple items lands in separate
        // subfolders / files rather than overwriting each other.
        let dst = destination_for(&src, &dst_root);

        let (id, ctrl) = state.queue.add(kind, src.clone(), Some(dst.clone()));
        let snapshot = state
            .queue
            .get(id)
            .expect("just-added job must be in queue");
        emit_job_added(&app, JobDto::from_job(&snapshot));

        let run = RunJob {
            app: app.clone(),
            state: state.inner().clone(),
            id,
            kind,
            src,
            dst: Some(dst),
            ctrl,
            verifier: verifier.clone(),
            copy_opts: copy_opts.clone(),
        };
        tokio::spawn(async move {
            run_job(run).await;
        });
        ids.push(id.as_u64());
    }
    Ok(ids)
}

fn destination_for(src: &Path, dst_root: &Path) -> PathBuf {
    let name = src
        .file_name()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("copy"));
    dst_root.join(name)
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
