//! Phase 49c — Tauri IPC for backup sources.
//!
//! A *source* is a folder the user designates for backup; "Back up now"
//! enumerates it (honouring exclude globs) and snapshots it into the
//! shared unified [`Repository`](copythat_chunk::Repository) as a
//! [`SnapshotKind::Backup`](copythat_chunk::SnapshotKind::Backup). CRUD
//! mirrors `sync_commands` — persisted under `Settings::backup.sources`,
//! UUID-keyed so a label edit never loses the timeline association.
//! Because an unchanged tree's chunks all dedup, a repeat backup adds
//! ~zero stored bytes, so "Back up now" is cheap to run often.

use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::ipc::{
    EVENT_BACKUP_COMPLETED, EVENT_BACKUP_FAILED, EVENT_BACKUP_PROGRESS, EVENT_BACKUP_STARTED,
};
use crate::state::AppState;

/// Wire shape for [`copythat_settings::SourceConfig`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceConfigDto {
    pub id: String,
    pub label: String,
    pub path: String,
    pub exclude_globs: Vec<String>,
    pub last_run_at: String,
    pub last_snapshot_id: Option<u64>,
    /// 49e — the source's retention policy (`{ kind: "keep-all" | "last-n"
    /// | "older-than-days" | "gfs", … }`). Round-trips as the
    /// [`RetentionSettings`](copythat_settings::RetentionSettings) enum.
    pub retention: copythat_settings::RetentionSettings,
    /// 49f — auto-run schedule spec (`""`/`@hourly`/`@daily`/`@weekly`/
    /// `M H * * *`).
    pub schedule: String,
    /// 49f — whether the schedule auto-runs on the minute tick.
    pub enabled: bool,
}

impl From<&copythat_settings::SourceConfig> for SourceConfigDto {
    fn from(c: &copythat_settings::SourceConfig) -> Self {
        Self {
            id: c.id.clone(),
            label: c.label.clone(),
            path: c.path.clone(),
            exclude_globs: c.exclude_globs.clone(),
            last_run_at: c.last_run_at.clone(),
            last_snapshot_id: c.last_snapshot_id,
            retention: c.retention.clone(),
            schedule: c.schedule.clone(),
            enabled: c.enabled,
        }
    }
}

// --- event payloads ---------------------------------------------------

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupStartedDto {
    id: String,
    label: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupProgressDto {
    id: String,
    files_done: u64,
    files_total: u64,
    bytes_done: u64,
    bytes_total: u64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupCompletedDto {
    id: String,
    snapshot_id: u64,
    files: u64,
    bytes: u64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupFailedDto {
    id: String,
    message: String,
}

// --- CRUD -------------------------------------------------------------

/// List every configured backup source.
#[tauri::command]
pub fn sources_list(state: tauri::State<'_, AppState>) -> Vec<SourceConfigDto> {
    state
        .settings_snapshot()
        .backup
        .sources
        .iter()
        .map(SourceConfigDto::from)
        .collect()
}

/// Add a new backup source and persist settings.
#[tauri::command]
pub fn sources_add(
    label: String,
    path: String,
    exclude_globs: Vec<String>,
    include_globs: Vec<String>,
    skip_hidden: bool,
    state: tauri::State<'_, AppState>,
) -> Result<SourceConfigDto, String> {
    if path.trim().is_empty() {
        return Err("backup source path is required".to_string());
    }
    let cfg = copythat_settings::SourceConfig {
        id: Uuid::new_v4().hyphenated().to_string(),
        label,
        path,
        exclude_globs,
        last_run_at: String::new(),
        last_snapshot_id: None,
        // 49e — new sources keep all history until the user picks a policy.
        retention: copythat_settings::RetentionSettings::default(),
        // 49f — manual until the user picks a schedule + enables it.
        schedule: String::new(),
        enabled: false,
        // 49g — gitignore-style include filter + skip-hidden.
        include_globs,
        skip_hidden,
    };
    {
        let mut guard = state.settings.write().map_err(|e| e.to_string())?;
        guard.backup.sources.push(cfg.clone());
        persist(&state, &guard)?;
    }
    Ok(SourceConfigDto::from(&cfg))
}

/// Edit an existing source's label / path / excludes.
#[tauri::command]
pub fn sources_update(
    id: String,
    label: String,
    path: String,
    exclude_globs: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<SourceConfigDto, String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let dto = {
        let src = guard
            .backup
            .sources
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or_else(|| format!("unknown source id: {id}"))?;
        src.label = label;
        src.path = path;
        src.exclude_globs = exclude_globs;
        SourceConfigDto::from(&*src)
    };
    persist(&state, &guard)?;
    Ok(dto)
}

/// Remove a source by id.
#[tauri::command]
pub fn sources_remove(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let before = guard.backup.sources.len();
    guard.backup.sources.retain(|s| s.id != id);
    if guard.backup.sources.len() == before {
        return Err(format!("unknown source id: {id}"));
    }
    persist(&state, &guard)
}

/// "Back up now" IPC — a thin wrapper over [`run_backup`], the same flow
/// the Phase 49f scheduler tick uses.
#[tauri::command]
pub async fn backup_now(
    app: AppHandle,
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    run_backup(app, &state, id).await
}

/// RAII release of a source's in-flight claim in [`AppState::backups_running`],
/// so [`run_backup`] clears it on EVERY exit path (early error, a `?`
/// short-circuit, or normal completion) — never leaking a claim that would
/// wedge the source as permanently "running".
struct InFlightGuard {
    set: std::sync::Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    id: String,
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        if let Ok(mut set) = self.set.lock() {
            set.remove(&self.id);
        }
    }
}

/// The shared backup flow (GUI "Back up now" + the scheduler tick):
/// enumerate the source tree (honouring exclude globs), snapshot it into
/// the shared Repository as `SnapshotKind::Backup` tagged with the source
/// id, apply retention, persist `last_run_at` + `last_snapshot_id`, and
/// emit the `backup-*` lifecycle events. Returns the new snapshot id.
pub(crate) async fn run_backup(
    app: AppHandle,
    state: &AppState,
    id: String,
) -> Result<u64, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let cfg = state
        .settings_snapshot()
        .backup
        .sources
        .iter()
        .find(|s| s.id == id)
        .cloned()
        .ok_or_else(|| format!("unknown source id: {id}"))?;

    // Claim the per-source in-flight lock so a manual "Back up now", a rapid
    // double-click, and the scheduler tick can't run the SAME source
    // concurrently (which would create duplicate snapshots + double prune/gc).
    // Previously only the tick claimed it, so manual runs were invisible to the
    // guard. Released on every exit path by the RAII guard below.
    {
        let claimed = state
            .backups_running
            .lock()
            .map(|mut set| set.insert(id.clone()))
            .unwrap_or(false);
        if !claimed {
            return Err("backup-already-running".to_string());
        }
    }
    let _in_flight = InFlightGuard {
        set: state.backups_running.clone(),
        id: id.clone(),
    };

    let _ = app.emit(
        EVENT_BACKUP_STARTED,
        BackupStartedDto {
            id: id.clone(),
            label: cfg.label.clone(),
        },
    );

    let app_for_blocking = app.clone();
    let id_for_blocking = id.clone();
    let root = PathBuf::from(&cfg.path);
    let label = cfg.label.clone();
    let excludes = cfg.exclude_globs.clone();
    let include_globs = cfg.include_globs.clone();
    let skip_hidden = cfg.skip_hidden;
    let source_id = id.clone();
    let retention = cfg.retention.clone();

    let outcome = tokio::task::spawn_blocking(move || -> Result<(u64, u64, u64), String> {
        // The snapshot_source walk doesn't pre-count, so progress is
        // indeterminate: signal start (files_total 0 → the UI shows an
        // indeterminate bar), then report the final tally on completion.
        let _ = app_for_blocking.emit(
            EVENT_BACKUP_PROGRESS,
            BackupProgressDto {
                id: id_for_blocking.clone(),
                files_done: 0,
                files_total: 0,
                bytes_done: 0,
                bytes_total: 0,
            },
        );
        // 49g — gitignore-style include/exclude + skip-hidden filters.
        let filters = copythat_chunk::FilterSet {
            exclude_globs: excludes,
            include_globs,
            skip_hidden,
            ..Default::default()
        }
        .compile()
        .map_err(|e| e.to_string())?;
        let now = chrono::Utc::now().timestamp_millis();
        let summary = repo
            .snapshot_source(
                copythat_chunk::SnapshotKind::Backup,
                &label,
                Some(&source_id),
                now,
                &root,
                &filters,
            )
            .map_err(|e| e.to_string())?;
        let _ = app_for_blocking.emit(
            EVENT_BACKUP_PROGRESS,
            BackupProgressDto {
                id: id_for_blocking,
                files_done: summary.files,
                files_total: summary.files,
                bytes_done: summary.bytes,
                bytes_total: summary.bytes,
            },
        );
        // 49e — apply this source's retention policy after a successful
        // capture (best-effort: a prune failure must not fail the backup
        // that just succeeded).
        if let Err(e) = repo.prune_source(&source_id, &retention_policy(&retention), now) {
            tracing::warn!(source = %source_id, error = %e, "backup retention prune failed");
        }
        Ok((summary.id.as_u64(), summary.files, summary.bytes))
    })
    .await
    .map_err(|e| format!("backup task: {e}"))?;

    // Advance the schedule's last-attempt timestamp REGARDLESS of outcome, so a
    // persistently-failing source (unplugged drive, permission error) doesn't
    // stay `is_due` and re-fire — storming failure notifications — on every 60s
    // tick. Only a SUCCESS additionally records last_snapshot_id.
    {
        let mut guard = state.settings.write().map_err(|e| e.to_string())?;
        if let Some(src) = guard.backup.sources.iter_mut().find(|s| s.id == id) {
            src.last_run_at = chrono::Utc::now().to_rfc3339();
            if let Ok((snapshot_id, _, _)) = &outcome {
                src.last_snapshot_id = Some(*snapshot_id);
            }
        }
        persist(state, &guard)?;
    }

    match outcome {
        Ok((snapshot_id, files, bytes)) => {
            let _ = app.emit(
                EVENT_BACKUP_COMPLETED,
                BackupCompletedDto {
                    id,
                    snapshot_id,
                    files,
                    bytes,
                },
            );
            // 49q — fire a success notification (gated by on_success).
            crate::notifications::dispatch(
                state,
                copythat_server::JobNotification {
                    kind: "backup_completed".into(),
                    title: cfg.label.clone(),
                    body: format!("{files} files"),
                    ok: true,
                },
            )
            .await;
            Ok(snapshot_id)
        }
        Err(message) => {
            let _ = app.emit(
                EVENT_BACKUP_FAILED,
                BackupFailedDto {
                    id,
                    message: message.clone(),
                },
            );
            // 49q — fire a failure notification (gated by on_failure).
            crate::notifications::dispatch(
                state,
                copythat_server::JobNotification {
                    kind: "backup_failed".into(),
                    title: cfg.label.clone(),
                    body: message.clone(),
                    ok: false,
                },
            )
            .await;
            Err(message)
        }
    }
}

// --- Phase 49e: retention / prune -------------------------------------

/// Wire shape for [`copythat_chunk::PruneReport`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PruneReportDto {
    pub snapshots_removed: u64,
    pub chunks_swept: u64,
    pub bytes_reclaimed: u64,
}

/// Bridge the settings-layer [`RetentionSettings`](copythat_settings::RetentionSettings)
/// mirror to the `copythat-core` `RetentionPolicy` the pruner consumes.
/// Lives here because `copythat-settings` deliberately has no
/// copythat-core edge; `KeepAll` maps to `RetentionPolicy::None`.
fn retention_policy(s: &copythat_settings::RetentionSettings) -> copythat_chunk::RetentionPolicy {
    use copythat_chunk::{GfsPolicy, RetentionPolicy};
    use copythat_settings::RetentionSettings as Rs;
    match s {
        Rs::KeepAll => RetentionPolicy::None,
        Rs::LastN { n } => RetentionPolicy::LastN(*n),
        Rs::OlderThanDays { days } => RetentionPolicy::OlderThanDays(*days),
        Rs::Gfs {
            hourly,
            daily,
            weekly,
            monthly,
        } => RetentionPolicy::Gfs(GfsPolicy {
            keep_hourly: *hourly,
            keep_daily: *daily,
            keep_weekly: *weekly,
            keep_monthly: *monthly,
        }),
    }
}

/// A source's configured retention policy out of the live settings.
fn source_retention(
    state: &tauri::State<'_, AppState>,
    id: &str,
) -> Result<copythat_settings::RetentionSettings, String> {
    state
        .settings_snapshot()
        .backup
        .sources
        .iter()
        .find(|s| s.id == id)
        .map(|s| s.retention.clone())
        .ok_or_else(|| format!("unknown source id: {id}"))
}

/// Set a source's retention policy and persist settings (49e). The policy
/// round-trips as the [`RetentionSettings`](copythat_settings::RetentionSettings)
/// tagged enum (`{ kind: "last-n", n: 5 }`, …).
#[tauri::command]
pub fn sources_set_retention(
    id: String,
    retention: copythat_settings::RetentionSettings,
    state: tauri::State<'_, AppState>,
) -> Result<SourceConfigDto, String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let dto = {
        let src = guard
            .backup
            .sources
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or_else(|| format!("unknown source id: {id}"))?;
        src.retention = retention;
        SourceConfigDto::from(&*src)
    };
    persist(&state, &guard)?;
    Ok(dto)
}

/// Preview which of a source's snapshots its configured retention policy
/// would prune at `now_ms`, **without** mutating anything (49e). Returns
/// the snapshot ids that would be removed.
#[tauri::command]
pub async fn repository_prune_preview(
    id: String,
    now_ms: i64,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<u64>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let policy = retention_policy(&source_retention(&state, &id)?);
    let ids = tokio::task::spawn_blocking(move || repo.plan_source_prune(&id, &policy, now_ms))
        .await
        .map_err(|e| format!("prune preview task: {e}"))?
        .map_err(|e| format!("prune preview: {e}"))?;
    Ok(ids.into_iter().map(|s| s.as_u64()).collect())
}

/// Apply a source's configured retention policy now: forget the
/// policy-selected snapshots in one transaction, then gc to reclaim their
/// chunks (49e). The freshest snapshot is always kept.
#[tauri::command]
pub async fn repository_prune(
    id: String,
    now_ms: i64,
    state: tauri::State<'_, AppState>,
) -> Result<PruneReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let policy = retention_policy(&source_retention(&state, &id)?);
    let report = tokio::task::spawn_blocking(move || repo.prune_source(&id, &policy, now_ms))
        .await
        .map_err(|e| format!("prune task: {e}"))?
        .map_err(|e| format!("prune: {e}"))?;
    Ok(PruneReportDto {
        snapshots_removed: report.snapshots_removed,
        chunks_swept: report.chunks_swept,
        bytes_reclaimed: report.bytes_reclaimed,
    })
}

// --- Phase 49f: scheduling --------------------------------------------

/// Per-source schedule status for the Library "next run" readout.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupSourceStatusDto {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub schedule: String,
    pub last_run_ms: i64,
    pub next_run_ms: Option<i64>,
    pub due: bool,
}

/// Parse a source's RFC-3339 `last_run_at` to epoch-ms (0 = never).
fn rfc3339_to_ms(s: &str) -> i64 {
    if s.is_empty() {
        return 0;
    }
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.timestamp_millis())
        .unwrap_or(0)
}

/// Set a source's auto-run schedule + enabled flag and persist (49f). The
/// spec is validated through `BackupSchedule::parse`, so an unsupported
/// cron is rejected at the boundary (`err-schedule-unsupported-cron`).
#[tauri::command]
pub fn sources_set_schedule(
    id: String,
    schedule: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<SourceConfigDto, String> {
    copythat_shape::BackupSchedule::parse(&schedule).map_err(|e| e.to_string())?;
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let dto = {
        let src = guard
            .backup
            .sources
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or_else(|| format!("unknown source id: {id}"))?;
        src.schedule = schedule;
        src.enabled = enabled;
        SourceConfigDto::from(&*src)
    };
    persist(&state, &guard)?;
    Ok(dto)
}

/// Per-source schedule status for the UI: last/next run + due flag (49f).
#[tauri::command]
pub fn backup_sources_status(state: tauri::State<'_, AppState>) -> Vec<BackupSourceStatusDto> {
    let now = chrono::Local::now();
    state
        .settings_snapshot()
        .backup
        .sources
        .iter()
        .map(|s| {
            let sched = copythat_shape::BackupSchedule::parse(&s.schedule)
                .unwrap_or(copythat_shape::BackupSchedule::Manual);
            let last_run_ms = rfc3339_to_ms(&s.last_run_at);
            BackupSourceStatusDto {
                id: s.id.clone(),
                label: s.label.clone(),
                enabled: s.enabled,
                schedule: s.schedule.clone(),
                last_run_ms,
                next_run_ms: if s.enabled {
                    sched.next_after(now)
                } else {
                    None
                },
                due: s.enabled && sched.is_due(last_run_ms, now),
            }
        })
        .collect()
}

/// Phase 49f scheduler tick — called once a minute by the Tauri poller.
/// Runs every enabled source whose schedule is due (and isn't already
/// running), each in its own task so a long backup never blocks the tick.
pub(crate) fn backup_tick(app: &AppHandle) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };
    let now = chrono::Local::now();
    for cfg in state.settings_snapshot().backup.sources {
        if !cfg.enabled {
            continue;
        }
        let Ok(sched) = copythat_shape::BackupSchedule::parse(&cfg.schedule) else {
            continue; // invalid spec — the UI validates on save; skip here
        };
        if matches!(sched, copythat_shape::BackupSchedule::Manual) {
            continue;
        }
        if !sched.is_due(rfc3339_to_ms(&cfg.last_run_at), now) {
            continue;
        }
        // run_backup now owns the per-source in-flight lock (so manual runs and
        // ticks dedup against each other) and returns "backup-already-running"
        // if a run is already in flight — nothing to claim or release here.
        let app = app.clone();
        let id = cfg.id.clone();
        tauri::async_runtime::spawn(async move {
            let Some(st) = app.try_state::<AppState>() else {
                return;
            };
            if let Err(e) = run_backup(app.clone(), &st, id.clone()).await {
                // The dedup signal is benign (a run is already in progress); log
                // only genuine failures.
                if e != "backup-already-running" {
                    tracing::warn!(source = %id, error = %e, "scheduled backup failed");
                }
            }
        });
    }
}

// --- helpers ----------------------------------------------------------

fn persist(state: &AppState, settings: &copythat_settings::Settings) -> Result<(), String> {
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        return Ok(()); // tests use an empty path; skip persistence
    }
    settings
        .save_to(path)
        .map_err(|e| format!("save settings: {e}"))
}
