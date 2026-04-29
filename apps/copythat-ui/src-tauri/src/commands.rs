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
use crate::ipc_safety::{err_string, validate_ipc_path, validate_ipc_paths};
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
    // Phase 17f — moved off `eprintln!` so production builds don't
    // surface user paths on stderr; opt in via `RUST_LOG=copythat=debug`.
    tracing::debug!(
        target: "copythat::ipc",
        source_count = sources.len(),
        "start_copy begin",
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
    // Phase 17e — route the destination through the IPC gate so
    // U+FFFD-laden strings (lossy WTF-16 → UTF-8 coercion at the
    // Tauri serde boundary) are rejected before any history row.
    // The earlier shape did `PathBuf::from(destination.trim())` then
    // `validate_path_no_traversal`, which catches `..` + NUL but
    // skips the encoding check; an attacker who could land a
    // U+FFFD payload would slip past `start_copy` even though
    // `destination_free_bytes` (and every other path-typed command)
    // rejects it.
    let dst_root = match crate::ipc_safety::validate_ipc_path(&destination) {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(
                target: "copythat::ipc",
                kind = ?kind,
                err_key = e.localized_key(),
                "destination rejected at IPC gate",
            );
            return Err(e.localized_key().to_string());
        }
    };
    // Pre-flight that the destination exists *and* is a directory.
    // Without this synchronous check the engine would later surface
    // a low-level NotFound / NotADirectory deep inside the runner,
    // by which point the caller has already received a job-id and
    // started rendering progress UI for a copy that's about to
    // fail-fast on the very first file. Reject at the IPC boundary
    // so the frontend can toast a typed error before the queue
    // accepts the job.
    if !dst_root.is_dir() {
        tracing::debug!(
            target: "copythat::ipc",
            kind = ?kind,
            "destination not a directory",
        );
        return Err("err-destination-not-directory".to_string());
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

    // Phase 17e — same gate the destination went through. Filter
    // out fully-empty entries (a stray empty drag-drop payload
    // shouldn't take down the whole batch), then validate the
    // remainder through the IPC gate which adds the U+FFFD encoding
    // check on top of `..`/NUL. If every entry filtered out, surface
    // `err-source-empty`.
    let raw_srcs: Vec<&str> = sources
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if raw_srcs.is_empty() {
        return Err("err-source-empty".to_string());
    }
    let srcs: Vec<PathBuf> = match crate::ipc_safety::validate_ipc_paths(&raw_srcs) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!(
                target: "copythat::ipc",
                kind = ?kind,
                err_key = e.localized_key(),
                "source rejected at IPC gate",
            );
            return Err(e.localized_key().to_string());
        }
    };
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
        preserve_sparseness: settings.transfer.preserve_sparseness,
        // Phase 42 — bridge the new transfer knobs into the engine.
        // Defaults match `CopyOptions::default()`; users can change
        // them via Settings.
        paranoid_verify: settings.transfer.paranoid_verify,
        sharing_violation_retries: settings.transfer.sharing_violation_retries,
        sharing_violation_base_delay_ms: settings.transfer.sharing_violation_base_delay_ms,
        preserve_security_metadata: settings.transfer.preserve_security_metadata,
        meta_policy: copythat_core::MetaPolicy {
            preserve_motw: settings.transfer.preserve_motw,
            preserve_xattrs: settings.transfer.preserve_posix_acls,
            preserve_posix_acls: settings.transfer.preserve_posix_acls,
            preserve_selinux: settings.transfer.preserve_selinux_contexts,
            preserve_resource_forks: settings.transfer.preserve_resource_forks,
            appledouble_fallback: settings.transfer.appledouble_fallback,
        },
        strategy: match settings.transfer.reflink {
            copythat_settings::ReflinkPreference::Prefer => copythat_core::CopyStrategy::Auto,
            copythat_settings::ReflinkPreference::Avoid => copythat_core::CopyStrategy::NoReflink,
            copythat_settings::ReflinkPreference::Disabled => {
                copythat_core::CopyStrategy::AlwaysAsync
            }
        },
        on_locked: match settings.transfer.on_locked {
            copythat_settings::LockedFilePolicyChoice::Ask => copythat_core::LockedFilePolicy::Ask,
            copythat_settings::LockedFilePolicyChoice::Retry => {
                copythat_core::LockedFilePolicy::Retry
            }
            copythat_settings::LockedFilePolicyChoice::Skip => {
                copythat_core::LockedFilePolicy::Skip
            }
            copythat_settings::LockedFilePolicyChoice::Snapshot => {
                copythat_core::LockedFilePolicy::Snapshot
            }
        },
        ..Default::default()
    };

    // Phase 39 — attach the platform fast-copy hook unconditionally.
    // Without this, the engine's reflink + `CopyFileExW` paths are
    // dark and every UI copy falls through to the pure-Rust async
    // loop. Phase 39 bench on Win 11 NVMe: with hook = 2429 MiB/s,
    // without = ~600 MiB/s on a 10 GiB same-volume copy. Stateless
    // unit struct; Arc is effectively free.
    opts.fast_copy_hook = Some(std::sync::Arc::new(copythat_platform::PlatformFastCopyHook));

    // Phase 23 — attach the platform extent-introspection hook whenever
    // sparseness preservation is enabled. The hook is stateless (a
    // unit struct) so the Arc is effectively free; the engine skips
    // the sparse pathway at runtime if `detect_extents` reports a
    // fully-dense source.
    if opts.preserve_sparseness {
        opts.sparse_ops = Some(std::sync::Arc::new(copythat_platform::PlatformSparseOps));
    }

    // Phase 24 — attach the platform security-metadata hook whenever
    // metadata preservation is enabled. Stateless unit struct, same
    // Arc-is-free pattern. The engine skips the apply pass at
    // runtime if `meta_ops.capture` returns an empty snapshot
    // (vanilla files with no out-of-band streams).
    if opts.preserve_security_metadata {
        opts.meta_ops = Some(std::sync::Arc::new(copythat_platform::PlatformMetaOps));
    }

    // Phase 19b — attach the snapshot bridge whenever the user opted
    // into `Snapshot` (and also eagerly when `Ask` is in effect, so
    // the runner has a hook ready the moment the user resolves the
    // prompt without a second IPC round-trip). The hook is stateless,
    // so the Arc costs nothing when `on_locked == Retry`.
    if matches!(
        opts.on_locked,
        copythat_core::LockedFilePolicy::Snapshot | copythat_core::LockedFilePolicy::Ask
    ) {
        opts.snapshot_hook = Some(std::sync::Arc::new(
            copythat_snapshot::CopyThatSnapshotHook::new(),
        ));
    }

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

/// Phase 42 / Gap #14 — register a frontend-supplied
/// [`tauri::ipc::Channel`] for a specific job's hot-path progress
/// stream. After this returns, [`crate::runner::emit_progress`]
/// dual-emits: the legacy `app.emit(EVENT_JOB_PROGRESS, …)` keeps
/// firing for back-compat, and `state.progress_channels.try_send` also
/// pushes the same DTO into the channel.
///
/// Frontend usage:
///
/// ```ts
/// import { Channel, invoke } from '@tauri-apps/api/core';
///
/// const ids = await invoke<number[]>('start_copy', { sources, destination });
/// const channel = new Channel<JobProgressDto>();
/// channel.onmessage = (dto) => { /* update UI */ };
/// await invoke('register_progress_channel', { jobId: ids[0], channel });
/// ```
///
/// The `jobId` does not need to correspond to a live job —
/// registering for an id that completes before any progress fires is
/// harmless. The registry has no auto-eviction; future work can add a
/// teardown call to the runner's terminal-state hooks if leaks become
/// observable.
#[tauri::command]
pub fn register_progress_channel(
    job_id: u64,
    channel: tauri::ipc::Channel<crate::ipc::JobProgressDto>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Look up the live `JobId` by its `u64` form. The shadowed
    // binding mirrors the existing `pause_job` / `resume_job`
    // shape — `self::job_id` errors when the queue has no record of
    // the id, which is the right shape here too (registering for a
    // phantom job is almost certainly a bug worth surfacing).
    let id = self::job_id(job_id, &state)?;
    state.progress_channels.register(id, channel);
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
///
/// Phase 17e — `path` is gated through [`validate_ipc_path`] before
/// classification. A traversal-laden or empty input renders as the
/// generic icon rather than touching disk; the gate keeps the
/// classifier from leaking enum variants based on a forged input.
#[tauri::command]
pub fn file_icon(path: String) -> FileIconDto {
    match validate_ipc_path(&path) {
        Ok(p) => crate::icon::classify(&p),
        // Stub icon for rejected input so the renderer doesn't crash
        // on a Result; the input path is bad enough that no
        // classification is meaningful.
        Err(_) => FileIconDto {
            kind: "unknown",
            extension: None,
        },
    }
}

/// Reveal a path in the platform's file manager. No-op + Err if
/// the path does not exist.
///
/// Phase 17e — gates `path` through [`validate_ipc_path`] before
/// the platform-specific reveal handler, so a `..`-laden input
/// can't escape the user's intended folder.
#[tauri::command]
pub fn reveal_in_folder(path: String) -> Result<(), String> {
    let p = validate_ipc_path(&path).map_err(err_string)?;
    crate::reveal::reveal(&p)
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
    // Phase 34 — audit the user-driven resolution. No-op when audit
    // is disabled.
    let audit_job_id = format!("job-{}", resolved.job_id);
    crate::audit_commands::record_collision_resolved(
        &state.audit,
        &audit_job_id,
        &resolved.src,
        &resolved.dst,
        crate::collisions::resolution_name(&resolved.resolution),
    );
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
    // Phase 17e — gate the destination path before opening it.
    let dst = validate_ipc_path(&path).map_err(err_string)?;
    let entries = state.errors.log();
    let body = match format.as_str() {
        "csv" => crate::errors::export_csv(&entries),
        "txt" => crate::errors::export_txt(&entries),
        other => return Err(format!("unknown export format: {other}")),
    };
    let bytes = body.len() as u64;
    std::fs::write(&dst, body).map_err(|e| format!("export write failed: {e}"))?;
    Ok(bytes)
}

/// Phase 8 follow-up + Phase 17d wiring — "Retry with elevated
/// permissions" button.
///
/// The error registry holds a pending prompt at `id`; we look up
/// its `(src, dst)` and route the request through the
/// `copythat-helper` request handler. Today the helper runs in-
/// process (the unprivileged main app dispatches `handle_request`
/// directly); the actual UAC / sudo / polkit spawn ceremony
/// reuses Phase 19b's `Start-Process -Verb RunAs` pattern and
/// lands when the per-OS spawn helpers are wired in
/// `crates/copythat-helper/src/spawn.rs`.
///
/// This wiring is a meaningful step forward from the prior stub:
/// the helper's path-safety bar, capability gate, and error-key
/// mapping all run; a click that previously surfaced
/// `retry-elevated-unavailable` now surfaces the actual class of
/// failure (`err-permission-denied` if the OS still refuses,
/// `err-path-escape` if the registry held a tainted path, etc.).
#[tauri::command]
pub async fn retry_elevated(id: u64, state: State<'_, AppState>) -> Result<u64, String> {
    use copythat_helper::capability::Capability;
    use copythat_helper::handle_request;
    use copythat_helper::rpc::{Request, Response};

    let (src, dst) = state
        .errors
        .pending_paths(id)
        .ok_or_else(|| "err-helper-unknown-prompt".to_string())?;

    // Run the helper dispatch on the blocking pool. `handle_request`
    // performs `std::fs::open` + a synchronous byte loop in
    // `copy_no_follow`; in a future phase it will block on a UAC /
    // sudo / polkit consent prompt for seconds at a time. Doing
    // that on the Tauri runtime thread would freeze every other
    // IPC command pinned to the same worker.
    let resp = tokio::task::spawn_blocking(move || {
        let granted = vec![Capability::ElevatedRetry];
        handle_request(&Request::ElevatedRetry { src, dst }, &granted)
    })
    .await
    .map_err(|e| format!("err-helper-join:{e}"))?;
    match resp {
        Response::ElevatedRetryOk { bytes } => Ok(bytes),
        Response::ElevatedRetryFailed { localized_key, .. } => Err(localized_key),
        Response::PathRejected { localized_key, .. } => Err(localized_key),
        Response::CapabilityDenied { .. } => Err("retry-elevated-unavailable".to_string()),
        other => Err(format!("err-helper-unexpected:{other:?}")),
    }
}

/// Phase 8 follow-up — "Quick hash (SHA-256)" button on the
/// collision modal. Hashes the named path and returns the lowercase
/// hex digest. The Svelte modal calls this on demand for each side
/// of the collision and shows the two digests next to each other so
/// the user can decide overwrite-vs-skip with content evidence
/// rather than just metadata.
///
/// SHA-256 (not BLAKE3) for two reasons: (1) the existing
/// `collision-modal-hash-check` Fluent key promises SHA-256 in
/// every locale; (2) SHA-256 sidecars are the dominant on-disk
/// format users encounter, so showing the same digest the
/// `*.sha256` sidecar carries is what the user expects to compare
/// against.
#[tauri::command]
pub async fn quick_hash_for_collision(path: String) -> Result<String, String> {
    let p = validate_ipc_path(&path).map_err(err_string)?;
    let (events_tx, mut events_rx) = tokio::sync::mpsc::channel(8);
    // Drain the progress channel concurrently — if we kept the rx
    // alive without reading it, the bounded sender would block
    // indefinitely once the 9th progress event tried to enqueue,
    // and `hash_file_async` would never return on files large
    // enough to emit > 8 events.
    let drain = tokio::spawn(async move { while events_rx.recv().await.is_some() {} });
    let report = copythat_hash::hash_file_async(
        &p,
        copythat_hash::HashAlgorithm::Sha256,
        copythat_core::CopyControl::new(),
        events_tx,
    )
    .await
    .map_err(|e| e.to_string())?;
    // Sender drops when `hash_file_async` returns, ending the
    // drain task; await it so we don't leak the join handle.
    let _ = drain.await;
    Ok(report.hex())
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
    // Phase 17e — gate the destination path before opening it.
    let dst = validate_ipc_path(&path).map_err(err_string)?;
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
    std::fs::write(&dst, body).map_err(|e| format!("history export write failed: {e}"))?;
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
    // Phase 39 — rerun must attach the platform fast-copy hook too,
    // otherwise the engine falls through to the async-fallback Rust
    // loop and runs at ~600 MiB/s instead of CopyFileExW's 2400+.
    let copy_opts = CopyOptions {
        fast_copy_hook: Some(std::sync::Arc::new(copythat_platform::PlatformFastCopyHook)),
        ..CopyOptions::default()
    };
    Ok(crate::shell::enqueue_jobs(
        &app,
        state.inner(),
        kind,
        sources,
        &dst_root,
        copy_opts,
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
    // Phase 34 — fingerprint both the prior and next settings for a
    // SettingsChanged audit record. The write-out happens whether
    // or not audit is enabled; the record call is a no-op when the
    // sink is idle.
    let prev_snapshot = state.settings_snapshot();
    let before_hash = crate::audit_commands::settings_fingerprint(&prev_snapshot);
    let after_hash = crate::audit_commands::settings_fingerprint(&next);
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
    // Phase 21 — re-apply the network settings to the live Shape and
    // emit `shape-rate-changed` so the header badge re-renders. The
    // schedule poller in `lib.rs` covers the minute-tick path; this
    // covers the "user dragged a slider in Settings" path.
    crate::state::apply_network_settings_to_shape(&state.shape, &next.network);
    let _ = app.emit(
        crate::ipc::EVENT_SHAPE_RATE_CHANGED,
        build_shape_rate_dto(state.inner()),
    );

    // Phase 34 — hot-swap the audit sink when the audit block
    // changed. Rebuilds with the new config (format / path / WORM)
    // even if only a subset changed — the brief's "seamless toggle"
    // expectation.
    //
    // Drain pending writes before swapping. `AuditSink::record` takes
    // a private mutex, calls `file.flush()` after each write, and
    // releases the mutex before returning — every observed record is
    // already on the kernel by the time the snapshot returns. But a
    // concurrent `record_*` from the runner may have just cloned an
    // `Arc<AuditSink>` via `snapshot()` and is about to call into it
    // *right now*; if we drop the registry slot's reference to the
    // old sink while that call is in flight, the file handle stays
    // alive (the Arc the runner holds keeps it pinned) — but a
    // *new* event after the swap could race with the close on the
    // old handle and land in the new sink with a chained hash that
    // points at content from the previous file, breaking the chain
    // for replay. `AuditRegistry::flush` re-acquires the sink's
    // writer mutex (so it blocks until any in-flight `record()`
    // clears the post-snapshot critical section) and then calls
    // `file.flush()`. That is the correct synchronization primitive
    // here — no wall-clock guess required.
    if prev_snapshot.audit != next.audit {
        state.audit.flush();
        match crate::audit_commands::build_sink(&next.audit) {
            Ok(new_sink) => state.audit.set(new_sink),
            Err(e) => eprintln!("[audit] rebuild sink failed: {e}"),
        }
    }
    if prev_snapshot != next {
        // Record the delta regardless of which group changed. The
        // field name lists the top-level group name(s) affected so a
        // compliance reviewer sees "network.mode flipped" at a glance
        // without round-tripping the full TOML blob.
        let field = diff_setting_groups(&prev_snapshot, &next);
        crate::audit_commands::record_settings_changed(
            &state.audit,
            &field,
            &before_hash,
            &after_hash,
        );
    }

    Ok((&next).into())
}

/// Name the top-level settings groups whose sub-fields differ
/// between `before` and `after`. Output is a comma-separated string
/// (`general`, `transfer`, …, `audit`) used as
/// `SettingsChanged.field`.
fn diff_setting_groups(
    before: &copythat_settings::Settings,
    after: &copythat_settings::Settings,
) -> String {
    let mut groups: Vec<&'static str> = Vec::new();
    if before.general != after.general {
        groups.push("general");
    }
    if before.transfer != after.transfer {
        groups.push("transfer");
    }
    if before.shell != after.shell {
        groups.push("shell");
    }
    if before.secure_delete != after.secure_delete {
        groups.push("secure_delete");
    }
    if before.advanced != after.advanced {
        groups.push("advanced");
    }
    if before.filters != after.filters {
        groups.push("filters");
    }
    if before.updater != after.updater {
        groups.push("updater");
    }
    if before.scan != after.scan {
        groups.push("scan");
    }
    if before.network != after.network {
        groups.push("network");
    }
    if before.conflict_profiles != after.conflict_profiles {
        groups.push("conflict_profiles");
    }
    if before.sync != after.sync {
        groups.push("sync");
    }
    if before.chunk_store != after.chunk_store {
        groups.push("chunk_store");
    }
    if before.drop_stack != after.drop_stack {
        groups.push("drop_stack");
    }
    if before.dnd != after.dnd {
        groups.push("dnd");
    }
    if before.path_translation != after.path_translation {
        groups.push("path_translation");
    }
    if before.power != after.power {
        groups.push("power");
    }
    if before.remotes != after.remotes {
        groups.push("remotes");
    }
    if before.mount != after.mount {
        groups.push("mount");
    }
    if before.audit != after.audit {
        groups.push("audit");
    }
    if groups.is_empty() {
        "unchanged".to_string()
    } else {
        groups.join(",")
    }
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

/// Phase 15 — hit the configured updater endpoint for the current
/// channel, parse the manifest, and report back whether the advertised
/// version is strictly newer than the running binary.
///
/// When `force` is `false`, the 24 h throttle gates the network hit —
/// the reply in that case carries `skippedByThrottle = true` so the UI
/// can render "last checked: X h ago" without lying about having just
/// hit the server. Independent of `force`, a successful fetch bumps
/// the persisted `last_check_unix_secs` timestamp so the next launch
/// check honours the throttle.
///
/// The `endpoint_override` argument lets tests point at a local HTTP
/// fixture without mutating `settings.toml`. Production calls pass
/// `None` and the handler composes the endpoint from the first entry
/// of `tauri.conf.json` → `plugins.updater.endpoints` after placeholder
/// substitution.
#[tauri::command]
pub async fn updater_check_now(
    force: bool,
    endpoint_override: Option<String>,
    state: State<'_, AppState>,
) -> Result<crate::updater::UpdateCheckDto, String> {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    // Snapshot throttle state + channel without holding the lock
    // across the network call.
    let (due, channel, current_ver) = {
        let snap = state.settings_snapshot();
        let due = force || snap.updater.due_for_check(now_secs);
        (
            due,
            snap.updater.channel.as_str().to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
        )
    };

    if !due {
        // Throttle the auto-check path; the UI can still call with
        // `force: true` from the "check now" button.
        return Ok(crate::updater::UpdateCheckDto {
            available_version: String::new(),
            notes: String::new(),
            pub_date: String::new(),
            is_newer: false,
            checked_at_unix_secs: {
                // Echo the stored last-check timestamp so the UI can
                // render "last checked: …".
                state.settings_snapshot().updater.last_check_unix_secs
            },
            skipped_by_throttle: true,
        });
    }

    let endpoint = match endpoint_override {
        Some(u) => u,
        None => {
            // Compose from the first configured endpoint. Placeholder
            // substitution matches the format Tauri's plugin expects.
            let tmpl = DEFAULT_UPDATER_ENDPOINT_TEMPLATE.to_string();
            crate::updater::format_endpoint(
                &tmpl,
                &channel,
                crate::updater::current_target_platform()
                    .split('-')
                    .next()
                    .unwrap_or("windows"),
                crate::updater::current_target_platform()
                    .split('-')
                    .nth(1)
                    .unwrap_or("x86_64"),
                &current_ver,
            )
        }
    };

    // Network hit off the Tauri-runtime thread — the helper is
    // blocking but bounded by the 10 s timeout.
    let manifest_res = tokio::task::spawn_blocking(move || {
        crate::updater::fetch_manifest_http(&endpoint, Duration::from_secs(10))
    })
    .await
    .map_err(|e| format!("updater-task-join: {e}"))?;

    let manifest = manifest_res.map_err(|e| e.to_string())?;

    let is_newer = crate::updater::is_strictly_newer(&manifest.version, &current_ver);

    // Persist the successful check. Only writes if a settings_path
    // was resolved (production launch); tests pass an empty path and
    // the write step is a no-op.
    {
        let path = state.settings_path.as_ref();
        let mut live = state
            .settings
            .write()
            .map_err(|_| "settings-lock-poisoned".to_string())?;
        live.updater.last_check_unix_secs = now_secs;
        if !path.as_os_str().is_empty() {
            let _ = live.save_to(path);
        }
    }

    Ok(crate::updater::UpdateCheckDto {
        available_version: manifest.version,
        notes: manifest.notes,
        pub_date: manifest.pub_date,
        is_newer,
        checked_at_unix_secs: now_secs,
        skipped_by_throttle: false,
    })
}

/// Phase 15 — persist that the user actively dismissed the named
/// version ("skip this release"). The updater UI suppresses banners
/// for exactly that version until a newer one is announced. Passing
/// an empty string clears the dismissal.
#[tauri::command]
pub fn updater_dismiss_version(version: String, state: State<'_, AppState>) -> Result<(), String> {
    let path = state.settings_path.as_ref().clone();
    let mut live = state
        .settings
        .write()
        .map_err(|_| "settings-lock-poisoned".to_string())?;
    live.updater.dismissed_version = version;
    if !path.as_os_str().is_empty() {
        live.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Default manifest endpoint template. Encoded as a constant here
/// (rather than hard-coded in `tauri.conf.json`) so tests that stand
/// up a local HTTP server can override via `endpoint_override` without
/// mutating shipped config. Placeholder grammar:
/// - `{{channel}}` — `"stable"` / `"beta"` from `UpdaterSettings`.
/// - `{{target}}` / `{{arch}}` — OS / CPU from `current_target_platform`.
/// - `{{current_version}}` — `CARGO_PKG_VERSION`.
const DEFAULT_UPDATER_ENDPOINT_TEMPLATE: &str =
    "https://releases.copythat.app/{{channel}}/{{target}}-{{arch}}.json";

/// Phase 14 — free bytes available on the volume backing `path`.
/// Powers the preflight check in the UI. Returns `0` when the probe
/// can't resolve the volume (unmounted, permission denied, UNC host
/// offline) so the UI falls through to "unknown — proceed".
#[tauri::command]
pub async fn destination_free_bytes(path: String) -> Result<u64, String> {
    // Phase 17e — gate the IPC arg before any FS probe.
    let p = validate_ipc_path(&path).map_err(err_string)?;
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
    // Phase 17e — gate every IPC arg before the walk. Empty list →
    // typed `EmptyList` Fluent key.
    let validated = validate_ipc_paths(&paths).map_err(err_string)?;
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
        for root in validated {
            if started.elapsed().as_millis() >= HARD_LIMIT_MS {
                return Err("too-large".to_string());
            }
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
    // Phase 17f — moved off `eprintln!`. Path bodies stay out of
    // stderr; the count is enough for the diagnostic.
    tracing::debug!(
        target: "copythat::ipc",
        path_count = paths.len(),
        "enumerate_tree_files start",
    );
    // Phase 17e — gate every IPC arg before any traversal.
    let validated = validate_ipc_paths(&paths).map_err(err_string)?;
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
        for root in validated {
            if overflow {
                break;
            }
            walk_files_streaming(&root, &mut files, HARD_LIMIT, &mut overflow, &mut ctx);
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
        // Phase 17f — counts only; user paths stay out of stderr.
        tracing::debug!(
            target: "copythat::ipc",
            count = files.len(),
            overflow,
            "enumerate_tree_files done",
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
    // Phase 17f — count-only diagnostic; path bodies stay off stderr.
    tracing::debug!(
        target: "copythat::ipc",
        path_count = paths.len(),
        "path_metadata start",
    );
    // Phase 17e — gate the IPC arg list before any FS probe.
    let validated = validate_ipc_paths(&paths).map_err(err_string)?;
    // The DropStagingDialog fires this command as part of the
    // file-order UI, so it has to return within ~2 s even when a
    // user dropped `D:\` with millions of files. A small entry cap
    // + a per-source wall-clock budget keeps the modal responsive;
    // any directory too big to measure lands as `size: u64::MAX`
    // and the UI renders "too large to count".
    const HARD_LIMIT_ENTRIES: u64 = 20_000;
    const HARD_LIMIT_MS: u128 = 2_000;
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::with_capacity(validated.len());
        for root in validated {
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
        tracing::debug!(target: "copythat::ipc", "path_metadata done");
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
    // Phase 17e — gate every IPC arg before walking.
    let validated = validate_ipc_paths(&paths).map_err(err_string)?;
    const HARD_LIMIT_ENTRIES: u64 = 200_000;
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::with_capacity(validated.len());
        for root in validated {
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
    // Phase 17e — gate the destination path before opening it.
    let dst = validate_ipc_path(&dest).map_err(err_string)?;
    state
        .profiles
        .export(&name, &dst)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_profile(
    name: String,
    src: String,
    state: State<'_, AppState>,
) -> Result<crate::ipc::ProfileInfoDto, String> {
    // Phase 17e — gate the source path before reading it.
    let src_path = validate_ipc_path(&src).map_err(err_string)?;
    let info = state
        .profiles
        .import(&name, &src_path)
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

// ---------------------------------------------------------------------
// Phase 20 — resume journal
// ---------------------------------------------------------------------

/// Snapshot of jobs the journal flagged as unfinished at app start.
/// Returns an empty list when journaling is disabled or the boot
/// sweep found nothing — both are perfectly valid "no work to
/// resume" states.
#[tauri::command]
pub fn pending_resumes(state: State<'_, AppState>) -> Vec<crate::ipc::PendingResumeDto> {
    let guard = state.startup_unfinished.lock().expect("lock poisoned");
    guard
        .iter()
        .map(|u| {
            let last = u
                .files
                .iter()
                .map(|f| f.last_checkpoint_at_ms)
                .max()
                .unwrap_or(0);
            crate::ipc::PendingResumeDto {
                row_id: u.row_id.as_u64(),
                kind: u.record.kind.clone(),
                src_root: u.record.src_root.to_string_lossy().into_owned(),
                dst_root: u
                    .record
                    .dst_root
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned()),
                status: u.record.status.as_str().to_string(),
                started_at_ms: u.record.started_at_ms,
                bytes_done: u.record.bytes_done,
                bytes_total: u.record.bytes_total,
                files_done: u.record.files_done,
                files_total: u.record.files_total,
                last_checkpoint_at_ms: last,
            }
        })
        .collect()
}

/// Discard a pending resume — the user picked "Don't resume" on the
/// modal. Removes the journal row + its file checkpoints so the next
/// launch's `pending_resumes()` is empty for this id.
///
/// `row_id` is the `JobRowId` returned by `pending_resumes()`.
#[tauri::command]
pub fn discard_resume(row_id: u64, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(j) = state.journal.as_ref() {
        j.delete_job(copythat_journal::JobRowId(row_id))
            .map_err(|e| e.to_string())?;
    }
    state
        .startup_unfinished
        .lock()
        .expect("lock poisoned")
        .retain(|u| u.row_id.as_u64() != row_id);
    Ok(())
}

// ---------------------------------------------------------------------
// Phase 21 — bandwidth shape introspection
// ---------------------------------------------------------------------

/// Snapshot the active bandwidth-shaping rate for the header badge.
///
/// Returns the live `Shape::current_rate` plus a short `source`
/// label so the UI can render "🔻 30 MB/s · scheduled" vs.
/// "🔻 paused · battery". The source is derived from the persisted
/// NetworkSettings, not the OS probes (Phase 21 stubs them); when the
/// per-OS bridges land in Phase 31 the runner-side schedule poller
/// can stamp the live source onto the EVENT_SHAPE_RATE_CHANGED
/// payload directly.
#[tauri::command]
pub fn current_shape_rate(state: State<'_, AppState>) -> crate::ipc::ShapeRateDto {
    build_shape_rate_dto(state.inner())
}

/// Helper used by both the IPC command and the post-`update_settings`
/// emit path. Takes `&AppState` so it doesn't need the `State<'_>`
/// wrapper from Tauri's command surface.
pub(crate) fn build_shape_rate_dto(state: &AppState) -> crate::ipc::ShapeRateDto {
    use copythat_settings::{AutoThrottleRule, BandwidthMode};
    let snap = state.settings_snapshot();
    let live_rate = state.shape.current_rate().map(|r| r.bytes_per_second());
    let source: &'static str = if matches!(
        snap.network.auto_on_cellular,
        AutoThrottleRule::Cap { .. } | AutoThrottleRule::Pause
    ) {
        "auto-cellular"
    } else if matches!(
        snap.network.auto_on_metered,
        AutoThrottleRule::Cap { .. } | AutoThrottleRule::Pause
    ) {
        "auto-metered"
    } else if matches!(
        snap.network.auto_on_battery,
        AutoThrottleRule::Cap { .. } | AutoThrottleRule::Pause
    ) {
        "auto-battery"
    } else {
        match snap.network.mode {
            BandwidthMode::Off => "off",
            BandwidthMode::Fixed => "settings",
            BandwidthMode::Schedule => "schedule",
        }
    };
    crate::ipc::ShapeRateDto {
        bytes_per_second: live_rate,
        source,
    }
}

/// Validate a schedule string from the Settings UI without saving
/// it. Returns the parsed rule count on success, an error message
/// on failure. Lets the textarea show inline feedback as the user
/// types.
#[tauri::command]
pub fn validate_schedule_spec(spec: String) -> Result<usize, String> {
    copythat_shape::Schedule::parse(&spec)
        .map(|s| s.rules().len())
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------
// Phase 22 — aggregate conflict dialog v2
// ---------------------------------------------------------------------

/// Render a thumbnail for `path` — a 240×240-ish PNG data URL for
/// images, a file-kind icon descriptor for everything else. Result
/// is cached under `<cache-dir>/thumb-cache/` keyed on
/// `(path, mtime, size, max_dim)` so repeat-opens of the same
/// conflict don't re-decode.
#[tauri::command]
pub async fn thumbnail_for(
    path: String,
    max_dim: Option<u32>,
) -> Result<crate::ipc::ThumbnailDto, String> {
    // Phase 17e — gate the IPC arg before reading the file.
    let p = validate_ipc_path(&path).map_err(err_string)?;
    let dim = max_dim.unwrap_or(crate::thumbnail::DEFAULT_MAX_DIM);
    tokio::task::spawn_blocking(move || crate::thumbnail::thumbnail_for(&p, dim))
        .await
        .map_err(|e| e.to_string())
}

/// Append one rule to a running job's live conflict-resolution
/// set. Returns the current rule count after the append. The UI
/// calls this from the aggregate dialog's bulk-action bar (`Apply
/// to all of this extension`, `Apply to glob…`, `Apply to all
/// remaining`); the runner consults the list before raising any
/// further collision prompts for this job.
#[tauri::command]
pub fn add_conflict_rule(
    job_id: u64,
    pattern: String,
    resolution: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    if pattern.trim().is_empty() {
        return Err("err-conflict-rule-empty-pattern".to_string());
    }
    // Compile-check the glob up-front so a malformed pattern can't
    // wedge the running job. The match helper on `ConflictProfile`
    // also refuses malformed patterns silently, but we'd rather
    // surface the error at insertion time.
    globset::Glob::new(&pattern).map_err(|e| format!("err-conflict-rule-invalid-glob: {e}"))?;
    let rule = copythat_settings::ConflictRule {
        pattern,
        resolution: copythat_settings::ConflictRuleResolution::from_wire(&resolution),
    };
    state.collisions.append_rule(job_id, rule);
    Ok(state
        .collisions
        .rules_for(job_id)
        .map(|p| p.rules.len())
        .unwrap_or(0))
}

/// Snapshot of a running job's live rule list. The profile-save
/// dialog renders this so the user sees what's about to be
/// persisted before clicking Save. Returns an empty
/// `ConflictProfile` when the job has no rules (not an error).
#[tauri::command]
pub fn current_conflict_rules(
    job_id: u64,
    state: State<'_, AppState>,
) -> copythat_settings::ConflictProfile {
    state.collisions.rules_for(job_id).unwrap_or_default()
}

/// List saved conflict-profile names (alphabetical). Reads from the
/// live settings — same source of truth the runner consults at job
/// start.
#[tauri::command]
pub fn list_conflict_profiles(state: State<'_, AppState>) -> Vec<String> {
    let live = state.settings_snapshot();
    live.conflict_profiles.profiles.keys().cloned().collect()
}

/// Save `profile` under `name`. Overwrites an existing entry — the
/// UI's save dialog surfaces a "replace existing?" confirmation
/// up-front, so this handler is the force-path. Returns the
/// updated list of profile names.
#[tauri::command]
pub fn save_conflict_profile(
    name: String,
    profile: copythat_settings::ConflictProfile,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("err-conflict-profile-empty-name".to_string());
    }
    let path = state.settings_path.as_ref().clone();
    let mut live = state
        .settings
        .write()
        .map_err(|_| "settings-lock-poisoned".to_string())?;
    live.conflict_profiles
        .profiles
        .insert(trimmed.to_string(), profile);
    if !path.as_os_str().is_empty() {
        live.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(live.conflict_profiles.profiles.keys().cloned().collect())
}

/// Delete a saved conflict profile. Clears the `active` pointer if
/// it happened to name the profile being deleted.
#[tauri::command]
pub fn delete_conflict_profile(
    name: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let path = state.settings_path.as_ref().clone();
    let mut live = state
        .settings
        .write()
        .map_err(|_| "settings-lock-poisoned".to_string())?;
    live.conflict_profiles.profiles.remove(&name);
    if live.conflict_profiles.active.as_deref() == Some(&name) {
        live.conflict_profiles.active = None;
    }
    if !path.as_os_str().is_empty() {
        live.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(live.conflict_profiles.profiles.keys().cloned().collect())
}

/// Set the currently-active profile. Pass an empty string to clear
/// the active selection (revert to "always prompt").
#[tauri::command]
pub fn set_active_conflict_profile(
    name: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let path = state.settings_path.as_ref().clone();
    let mut live = state
        .settings
        .write()
        .map_err(|_| "settings-lock-poisoned".to_string())?;
    let active = if name.trim().is_empty() {
        None
    } else if live.conflict_profiles.profiles.contains_key(&name) {
        Some(name.clone())
    } else {
        return Err("err-conflict-profile-not-found".to_string());
    };
    live.conflict_profiles.active = active.clone();
    if !path.as_os_str().is_empty() {
        live.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(active)
}

/// "Discard all" header button. Clears every unfinished journal row
/// + the in-memory list. Best-effort: a per-row delete failure is
/// surfaced via the returned `Vec<String>` of error messages so the
/// UI can show "discarded N, kept M (errors: …)".
#[tauri::command]
pub fn discard_all_resumes(state: State<'_, AppState>) -> Vec<String> {
    let mut errors = Vec::new();
    let ids: Vec<u64> = state
        .startup_unfinished
        .lock()
        .expect("lock poisoned")
        .iter()
        .map(|u| u.row_id.as_u64())
        .collect();
    if let Some(j) = state.journal.as_ref() {
        for id in &ids {
            if let Err(e) = j.delete_job(copythat_journal::JobRowId(*id)) {
                errors.push(format!("row {id}: {e}"));
            }
        }
    }
    state
        .startup_unfinished
        .lock()
        .expect("lock poisoned")
        .clear();
    errors
}

// ---------------------------------------------------------------------
// Phase 29 — destination picker support: list immediate children of a
// directory so the Svelte picker can render one level at a time and
// spring-load its way deeper. Returns only directory entries; files
// are not relevant for a *destination* picker. Hidden entries are
// included — the UI decides whether to filter them.
// ---------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirChildDto {
    pub name: String,
    pub path: String,
    /// Readable + writable = valid destination. Surfaces as the
    /// `invalid` flag on the DropTarget so the UI can paint the error
    /// border + tooltip without a second round-trip.
    pub writable: bool,
}

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<DirChildDto>, String> {
    // Phase 17e — gate the IPC arg before traversing the dir.
    let root = validate_ipc_path(&path).map_err(err_string)?;
    tokio::task::spawn_blocking(move || {
        if !root.is_dir() {
            return Err(format!("not a directory: {}", root.display()));
        }
        let mut out: Vec<DirChildDto> = Vec::new();
        let rd = std::fs::read_dir(&root).map_err(|e| e.to_string())?;
        for entry in rd.flatten() {
            let ft = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };
            if !ft.is_dir() {
                continue;
            }
            let child_path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            // "Writable" probe: on Unix check the mode; on Windows we
            // can't cheaply know without attempting a create, so fall
            // back to `true` and let the actual copy surface the
            // permission error. Read-only dir detection on Windows
            // uses the FILE_ATTRIBUTE_READONLY bit, which is advisory
            // anyway.
            let writable = is_dir_writable(&child_path);
            out.push(DirChildDto {
                name,
                path: child_path.to_string_lossy().into_owned(),
                writable,
            });
        }
        out.sort_by_key(|e| e.name.to_lowercase());
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn is_dir_writable(p: &std::path::Path) -> bool {
    // Cheap advisory probe. Real permission is only knowable by a
    // create attempt; we defer that to the actual enqueue so the
    // picker stays snappy.
    match std::fs::metadata(p) {
        Ok(md) => !md.permissions().readonly(),
        Err(_) => false,
    }
}

/// Phase 29 — root list for the destination picker. On Windows the
/// "root" is the list of drive letters; on Unix it's just `/`. The
/// picker calls this once on open and then walks children via
/// `list_directory`.
#[tauri::command]
pub async fn list_roots() -> Result<Vec<DirChildDto>, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(windows)]
        {
            let mut out = Vec::new();
            for letter in b'A'..=b'Z' {
                let p = format!("{}:\\", letter as char);
                let path = std::path::PathBuf::from(&p);
                if path.exists() {
                    out.push(DirChildDto {
                        name: p.clone(),
                        path: p,
                        writable: is_dir_writable(&path),
                    });
                }
            }
            Ok(out)
        }
        #[cfg(not(windows))]
        {
            Ok(vec![DirChildDto {
                name: "/".to_string(),
                path: "/".to_string(),
                writable: is_dir_writable(std::path::Path::new("/")),
            }])
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

// ---------------------------------------------------------------------
// Phase 29 — drag-out stub. Given a list of paths already staged
// elsewhere (e.g. Drop Stack entries), emit an IPC event the frontend
// can bridge into the OS drag-source. Cross-platform native OLE /
// NSPasteboardItem wiring is tracked as Phase 29b and requires a
// third-party crate add; the stub lets the Svelte layer set up the
// drag-image + HTML5 fallback today so in-app drop targets work.
// ---------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DragOutStagedDto {
    pub paths: Vec<String>,
    pub count: usize,
}

#[tauri::command]
pub async fn drag_out_stage(paths: Vec<String>) -> Result<DragOutStagedDto, String> {
    // Phase 17e — every drag-out path runs through the lexical
    // gate. A traversal-laden member rejects the whole batch
    // (consistent with `start_copy`'s "fail the whole list on the
    // first bad path" policy).
    let validated = validate_ipc_paths(&paths).map_err(err_string)?;
    // Then drop entries that no longer exist on disk.
    let kept: Vec<String> = validated
        .into_iter()
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    Ok(DragOutStagedDto {
        count: kept.len(),
        paths: kept,
    })
}
