//! Phase 25 — Tauri IPC surface for two-way sync with vector-clock
//! conflict detection.
//!
//! The Rust runtime owns each active sync run's [`SyncControl`] via
//! a shared [`SyncRegistry`], keyed by the persisted pair ID. That
//! lets the frontend pause / cancel a running sync by pair ID after
//! `start_sync` returns without holding a reference. Sync lifecycle
//! events are forwarded onto the Tauri event bus using the
//! `EVENT_SYNC_*` constants in [`crate::ipc`].

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use copythat_settings::SyncPairConfig;
use copythat_sync::{SyncControl, SyncEvent, SyncMode, SyncOptions, SyncPair, SyncReport, sync};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::ipc::{
    EVENT_SYNC_ACTION, EVENT_SYNC_CANCELLED, EVENT_SYNC_COMPLETED, EVENT_SYNC_CONFLICT,
    EVENT_SYNC_FAILED, EVENT_SYNC_STARTED, EVENT_SYNC_WALK_COMPLETED, EVENT_SYNC_WALK_STARTED,
    SyncActionDto, SyncCompletedDto, SyncConflictDto, SyncFailedDto, SyncPairDto, SyncStartedDto,
    SyncWalkDto,
};
use crate::state::AppState;

/// One row in the sync registry. `ctrl` is cheap-cloneable; the
/// other fields are informational so an in-progress stop by pair ID
/// can log "was syncing <label>".
#[derive(Clone)]
struct SyncHandle {
    ctrl: SyncControl,
    #[allow(dead_code)]
    label: String,
}

#[derive(Clone, Default)]
pub struct SyncRegistry {
    inner: Arc<RwLock<HashMap<String, SyncHandle>>>,
}

impl SyncRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&self, pair_id: String, handle: SyncHandle) {
        self.inner
            .write()
            .expect("sync registry poisoned")
            .insert(pair_id, handle);
    }

    fn remove(&self, pair_id: &str) {
        self.inner
            .write()
            .expect("sync registry poisoned")
            .remove(pair_id);
    }

    fn get(&self, pair_id: &str) -> Option<SyncHandle> {
        self.inner
            .read()
            .expect("sync registry poisoned")
            .get(pair_id)
            .cloned()
    }

    /// List of active (running or paused) pair IDs.
    pub fn active_ids(&self) -> Vec<String> {
        self.inner
            .read()
            .expect("sync registry poisoned")
            .keys()
            .cloned()
            .collect()
    }
}

// ---------------------------------------------------------------------
// IPC: CRUD + run surface
// ---------------------------------------------------------------------

/// List every configured pair from `Settings::sync`. Returns DTOs
/// suitable for direct render in the Svelte layer.
#[tauri::command]
pub fn list_sync_pairs(state: tauri::State<'_, AppState>) -> Vec<SyncPairDto> {
    let snap = state.settings_snapshot();
    let active = state.syncs.active_ids();
    snap.sync
        .pairs
        .iter()
        .map(|c| pair_config_to_dto(c, active.contains(&c.id)))
        .collect()
}

/// Append a new pair and persist settings.
#[tauri::command]
pub fn add_sync_pair(
    label: String,
    left: String,
    right: String,
    mode: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<SyncPairDto, String> {
    if left.trim().is_empty() || right.trim().is_empty() {
        return Err("left and right paths are required".to_string());
    }
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().hyphenated().to_string();
    let chosen_mode = mode
        .as_deref()
        .map(copythat_settings::SyncModeChoice::from_wire)
        .unwrap_or(guard.sync.default_mode);
    let cfg = SyncPairConfig {
        id: id.clone(),
        label,
        left,
        right,
        mode: chosen_mode,
        db_path_override: String::new(),
        last_run_at: String::new(),
        last_run_summary: String::new(),
    };
    guard.sync.pairs.push(cfg.clone());
    let path = state.settings_path.as_ref().clone();
    let snapshot = guard.clone();
    drop(guard);
    if !path.as_os_str().is_empty() {
        snapshot.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(pair_config_to_dto(&cfg, false))
}

/// Remove a pair by id. The `.copythat-sync.db` on disk is preserved
/// so an accidental remove can be recovered by re-adding the pair
/// with the same left root. Explicit file cleanup is a follow-up UI
/// affordance.
#[tauri::command]
pub fn remove_sync_pair(pair_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // If the pair is running, cancel it first so the registry doesn't
    // reference a pair that no longer exists.
    if let Some(h) = state.syncs.get(&pair_id) {
        h.ctrl.cancel();
        state.syncs.remove(&pair_id);
    }
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let before = guard.sync.pairs.len();
    guard.sync.pairs.retain(|p| p.id != pair_id);
    if guard.sync.pairs.len() == before {
        return Err(format!("unknown pair id: {pair_id}"));
    }
    let path = state.settings_path.as_ref().clone();
    let snapshot = guard.clone();
    drop(guard);
    if !path.as_os_str().is_empty() {
        snapshot.save_to(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Kick off a sync run for the pair with id `pair_id`. Returns
/// immediately; progress events fire asynchronously. Returns an
/// error if the pair is unknown or already running.
#[tauri::command]
pub async fn start_sync(
    pair_id: String,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let (cfg, host_label_override) = {
        let snap = state.settings_snapshot();
        let cfg = snap
            .sync
            .pairs
            .iter()
            .find(|p| p.id == pair_id)
            .cloned()
            .ok_or_else(|| format!("unknown pair id: {pair_id}"))?;
        let host = snap.sync.host_label_override.clone();
        (cfg, host)
    };

    if state.syncs.get(&pair_id).is_some() {
        return Err(format!("pair {pair_id} is already running"));
    }

    let ctrl = SyncControl::new();
    state.syncs.insert(
        pair_id.clone(),
        SyncHandle {
            ctrl: ctrl.clone(),
            label: cfg.label.clone(),
        },
    );

    let left = PathBuf::from(&cfg.left);
    let right = PathBuf::from(&cfg.right);
    let mut pair_builder = SyncPair::new(cfg.label.clone(), &left, &right);
    if !cfg.db_path_override.is_empty() {
        pair_builder = pair_builder.with_db_path(cfg.db_path_override.clone());
    }
    if !host_label_override.is_empty() {
        pair_builder = pair_builder.with_host_label(host_label_override);
    }
    let pair = pair_builder;
    let mode = match cfg.mode {
        copythat_settings::SyncModeChoice::TwoWay => SyncMode::TwoWay,
        copythat_settings::SyncModeChoice::MirrorLeftToRight => SyncMode::MirrorLeftToRight,
        copythat_settings::SyncModeChoice::MirrorRightToLeft => SyncMode::MirrorRightToLeft,
        copythat_settings::SyncModeChoice::ContributeLeftToRight => SyncMode::ContributeLeftToRight,
    };

    let (tx, rx) = mpsc::channel::<SyncEvent>(256);
    let app_for_events = app.clone();
    let pair_id_for_events = pair_id.clone();
    tokio::spawn(forward_sync_events(app_for_events, pair_id_for_events, rx));

    let _ = app.emit(
        EVENT_SYNC_STARTED,
        SyncStartedDto {
            pair_id: pair_id.clone(),
            label: cfg.label.clone(),
            left: cfg.left.clone(),
            right: cfg.right.clone(),
            mode: cfg.mode.as_str().to_string(),
        },
    );

    let registry = state.syncs.clone();
    let started_at = Instant::now();
    let app_for_run = app.clone();
    let pair_id_for_run = pair_id.clone();
    let settings_handle = state.settings.clone();
    let settings_path = state.settings_path.clone();

    tokio::spawn(async move {
        let opts = SyncOptions::default();
        let result = sync(&pair, mode, opts, ctrl, tx).await;
        registry.remove(&pair_id_for_run);
        match result {
            Ok(report) => {
                update_last_run(&settings_handle, &settings_path, &pair_id_for_run, &report);
                let _ = app_for_run.emit(
                    EVENT_SYNC_COMPLETED,
                    SyncCompletedDto {
                        pair_id: pair_id_for_run.clone(),
                        applied_left: report.applied_left as u64,
                        applied_right: report.applied_right as u64,
                        deleted_left: report.deleted_left as u64,
                        deleted_right: report.deleted_right as u64,
                        conflicts: report.conflicts.len() as u64,
                        cancelled: report.cancelled,
                        duration_ms: started_at.elapsed().as_millis() as u64,
                    },
                );
            }
            Err(err) => {
                if matches!(err, copythat_sync::SyncError::Cancelled) {
                    let _ = app_for_run.emit(
                        EVENT_SYNC_CANCELLED,
                        SyncFailedDto {
                            pair_id: pair_id_for_run.clone(),
                            message: "cancelled".to_string(),
                        },
                    );
                } else {
                    let _ = app_for_run.emit(
                        EVENT_SYNC_FAILED,
                        SyncFailedDto {
                            pair_id: pair_id_for_run.clone(),
                            message: err.to_string(),
                        },
                    );
                }
            }
        }
    });

    Ok(pair_id)
}

/// Pause a running sync by pair id. Idempotent — pausing an already-
/// paused sync is a no-op.
#[tauri::command]
pub fn pause_sync(pair_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    match state.syncs.get(&pair_id) {
        Some(h) => {
            h.ctrl.pause();
            Ok(())
        }
        None => Err(format!("unknown or idle pair id: {pair_id}")),
    }
}

/// Cancel a running sync by pair id.
#[tauri::command]
pub fn cancel_sync(pair_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    match state.syncs.get(&pair_id) {
        Some(h) => {
            h.ctrl.cancel();
            Ok(())
        }
        None => Err(format!("unknown or idle pair id: {pair_id}")),
    }
}

// ---------------------------------------------------------------------
// Event forwarding
// ---------------------------------------------------------------------

async fn forward_sync_events(app: AppHandle, pair_id: String, mut rx: mpsc::Receiver<SyncEvent>) {
    while let Some(evt) = rx.recv().await {
        match evt {
            SyncEvent::WalkStarted { side } => {
                let _ = app.emit(
                    EVENT_SYNC_WALK_STARTED,
                    SyncWalkDto {
                        pair_id: pair_id.clone(),
                        side: direction_wire(side),
                        files_total: None,
                    },
                );
            }
            SyncEvent::WalkCompleted { side, files_total } => {
                let _ = app.emit(
                    EVENT_SYNC_WALK_COMPLETED,
                    SyncWalkDto {
                        pair_id: pair_id.clone(),
                        side: direction_wire(side),
                        files_total: Some(files_total),
                    },
                );
            }
            SyncEvent::ActionCompleted { action } => {
                let _ = app.emit(
                    EVENT_SYNC_ACTION,
                    SyncActionDto {
                        pair_id: pair_id.clone(),
                        relpath: action.relpath().to_string(),
                        kind: action_kind_wire(&action),
                    },
                );
            }
            SyncEvent::Conflict(c) => {
                let _ = app.emit(
                    EVENT_SYNC_CONFLICT,
                    SyncConflictDto {
                        pair_id: pair_id.clone(),
                        relpath: c.relpath,
                        kind: conflict_kind_wire(c.kind).to_string(),
                        winner_side: direction_wire(c.winner).to_string(),
                        loser_side: direction_wire(c.loser).to_string(),
                        loser_preservation_path: c
                            .loser_preservation_path
                            .to_string_lossy()
                            .into_owned(),
                    },
                );
            }
            _ => {}
        }
    }
}

fn direction_wire(d: copythat_sync::Direction) -> &'static str {
    match d {
        copythat_sync::Direction::LeftToRight => "left-to-right",
        copythat_sync::Direction::RightToLeft => "right-to-left",
    }
}

fn conflict_kind_wire(k: copythat_sync::ConflictKind) -> &'static str {
    match k {
        copythat_sync::ConflictKind::ConcurrentWrite => "concurrent-write",
        copythat_sync::ConflictKind::DeleteEdit => "delete-edit",
        copythat_sync::ConflictKind::AddAdd => "add-add",
        copythat_sync::ConflictKind::CorruptEqual => "corrupt-equal",
    }
}

fn action_kind_wire(a: &copythat_sync::SyncAction) -> String {
    match a {
        copythat_sync::SyncAction::Noop { .. } => "noop".to_string(),
        copythat_sync::SyncAction::Copy { direction, .. } => {
            format!("copy-{}", direction_wire(*direction))
        }
        copythat_sync::SyncAction::Delete { direction, .. } => {
            format!("delete-{}", direction_wire(*direction))
        }
        copythat_sync::SyncAction::KeepConflict { .. } => "keep-conflict".to_string(),
    }
}

fn pair_config_to_dto(c: &SyncPairConfig, running: bool) -> SyncPairDto {
    SyncPairDto {
        id: c.id.clone(),
        label: c.label.clone(),
        left: c.left.clone(),
        right: c.right.clone(),
        mode: c.mode.as_str().to_string(),
        last_run_at: c.last_run_at.clone(),
        last_run_summary: c.last_run_summary.clone(),
        running,
    }
}

fn update_last_run(
    settings: &Arc<std::sync::RwLock<copythat_settings::Settings>>,
    settings_path: &Arc<PathBuf>,
    pair_id: &str,
    report: &SyncReport,
) {
    let Ok(mut guard) = settings.write() else {
        return;
    };
    let now = chrono::Utc::now().to_rfc3339();
    let summary = format!(
        "+{} / −{} / !{}",
        report.applied_left + report.applied_right,
        report.deleted_left + report.deleted_right,
        report.conflicts.len()
    );
    for p in guard.sync.pairs.iter_mut() {
        if p.id == pair_id {
            p.last_run_at = now.clone();
            p.last_run_summary = summary.clone();
            break;
        }
    }
    let snapshot = guard.clone();
    drop(guard);
    if !settings_path.as_os_str().is_empty() {
        let _ = snapshot.save_to(settings_path);
    }
}
