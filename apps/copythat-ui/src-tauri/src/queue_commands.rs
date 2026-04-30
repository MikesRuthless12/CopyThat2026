//! Phase 45.2 — Tauri IPC for the named-queue / drag-merge / F2-mode
//! UX. Six commands, all backed by the `QueueRegistry` instance held
//! on `AppState`.
//!
//! Wire shape:
//!
//! - `queue_list() -> Vec<QueueSnapshotDto>` — one entry per queue
//!   currently held by the registry, with the badge count the
//!   Phase 45.3 tab strip uses.
//! - `queue_route_job(kind, src, dst) -> RoutedJobDto` — drop a new
//!   job into the right queue (auto-discriminating by physical drive
//!   id, or honouring F2-mode when set). Returns the assigned ids;
//!   actual execution is wired in a later sub-phase as the runner
//!   learns to consume every registry queue.
//! - `queue_merge(src_id, dst_id) -> Result<()>` — collapse two
//!   queues; emits `queue-merged` + `queue-removed` events.
//! - `queue_set_f2_mode(enabled)` — toggle the registry's
//!   `auto_enqueue_next` atomic. Transient — never persisted.
//! - `queue_pin_destination(label, path)` — append a tray
//!   destination target to `Settings::queue::pinned_destinations`
//!   and persist.
//! - `queue_get_pinned() -> Vec<PinnedDestinationDto>` — return the
//!   current pinned-destination list.

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use copythat_core::{JobKind, JobState, QueueId, QueueRegistry, QueueRegistryEvent};
use copythat_settings::{PinnedDestination, Settings};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime};
use tokio::sync::broadcast::error::RecvError;

use crate::state::AppState;

/// Tauri event names. Kept in one place so the JS side has a single
/// source of truth for the strings to `listen()` on.
pub const EVENT_QUEUE_ADDED: &str = "queue-added";
pub const EVENT_QUEUE_REMOVED: &str = "queue-removed";
pub const EVENT_QUEUE_MERGED: &str = "queue-merged";
pub const EVENT_QUEUE_JOB_ROUTED: &str = "queue-job-routed";

/// Wire shape for `queue-added` events.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueAddedEvent {
    pub id: u64,
    pub name: String,
}

/// Wire shape for `queue-removed` / `queue-merged` events.
/// `merged.src` populates `id` for `queue-removed`; the
/// `queue-merged` event carries both ids in `MergedEvent`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueIdEvent {
    pub id: u64,
}

/// Wire shape for `queue-merged` events.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMergedEvent {
    pub src: u64,
    pub dst: u64,
}

/// Wire shape for `queue-job-routed` events.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueJobRoutedEvent {
    pub queue_id: u64,
    pub job_id: u64,
}

/// Wire shape used by the Phase 45.3 tab strip. Mirrors what the
/// frontend needs to render one tab per queue: identity (`id` /
/// `name`), the badge count (Pending + Running jobs), and a flag
/// for the F2-mode pulse animation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSnapshotDto {
    /// Stable id (`QueueId::as_u64()`). Frontend keeps a
    /// `Map<id, tab>` keyed on this; ordering on the wire is
    /// insertion order.
    pub id: u64,
    /// Tab label — typically `"D: queue"` on Windows, `"default"`
    /// for the back-compat queue, `"queue N"` when no probe label
    /// is available.
    pub name: String,
    /// Number of `Pending` + `Running` jobs in this queue. Drives
    /// the badge counter in the tab strip.
    pub badge_count: usize,
    /// `true` when at least one job in this queue is currently
    /// `Running`. Phase 45.5 uses this to render the F2-mode pulse
    /// on whichever tab is the active routing target.
    pub running: bool,
}

/// Wire shape returned from [`queue_route_job`]. Frontend stashes
/// the ids on the JobList row so subsequent pause/resume/cancel
/// IPC calls can reference the right queue.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutedJobDto {
    /// Queue the job ended up in (matches a row from
    /// [`queue_list`]).
    pub queue_id: u64,
    /// Per-job id, unique across the whole registry.
    pub job_id: u64,
}

/// Wire shape for [`PinnedDestination`]. Mirrors the persisted form
/// 1:1 — kept distinct from the settings struct so adding a UI-only
/// field (e.g. a colour swatch in a future phase) doesn't churn
/// `copythat-settings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedDestinationDto {
    pub label: String,
    pub path: String,
}

impl From<PinnedDestination> for PinnedDestinationDto {
    fn from(p: PinnedDestination) -> Self {
        Self {
            label: p.label,
            path: p.path,
        }
    }
}

impl From<PinnedDestinationDto> for PinnedDestination {
    fn from(p: PinnedDestinationDto) -> Self {
        Self {
            label: p.label,
            path: p.path,
        }
    }
}

// ---------------------------------------------------------------------
// Tauri command thin wrappers — bodies live in the AppState helpers
// below so tests can exercise the logic without a real Tauri runtime.
// ---------------------------------------------------------------------

/// `queue_list()` — snapshot the registry. Read-only; never mutates.
#[tauri::command]
pub fn queue_list(state: tauri::State<'_, AppState>) -> Vec<QueueSnapshotDto> {
    queue_list_impl(state.inner())
}

/// `queue_route_job(kind, src, dst)` — enqueue a job through the
/// registry's auto-discriminating router. The wire-string `kind`
/// uses the same vocabulary as the legacy `start_copy` / job-list
/// surface (`"copy"`, `"move"`, `"delete"`, `"secure-delete"`,
/// `"verify"`).
#[tauri::command]
pub fn queue_route_job(
    state: tauri::State<'_, AppState>,
    kind: String,
    src: String,
    dst: Option<String>,
) -> Result<RoutedJobDto, String> {
    queue_route_job_impl(state.inner(), &kind, &src, dst.as_deref())
}

/// `queue_merge(src_id, dst_id)` — collapse two queues. No-op when
/// `src_id == dst_id`. Errors when either id doesn't resolve.
#[tauri::command]
pub fn queue_merge(
    state: tauri::State<'_, AppState>,
    src_id: u64,
    dst_id: u64,
) -> Result<(), String> {
    queue_merge_impl(state.inner(), src_id, dst_id)
}

/// `queue_set_f2_mode(enabled)` — flip the registry's
/// `auto_enqueue_next` flag. The frontend's F2 keybinding
/// invokes this. Transient state — never written to settings.
#[tauri::command]
pub fn queue_set_f2_mode(state: tauri::State<'_, AppState>, enabled: bool) {
    queue_set_f2_mode_impl(state.inner(), enabled);
}

/// `queue_pin_destination(label, path)` — append a row to
/// `Settings::queue::pinned_destinations` and persist. Duplicate
/// `(label, path)` pairs are ignored so a chatty UI can replay
/// the same call without growing the list. Phase 45.6 — calls
/// [`crate::rebuild_tray_menu`] on success so the OS tray menu
/// reflects the new row immediately.
#[tauri::command]
pub fn queue_pin_destination<R: Runtime>(
    state: tauri::State<'_, AppState>,
    app: AppHandle<R>,
    label: String,
    path: String,
) -> Result<Vec<PinnedDestinationDto>, String> {
    let result = queue_pin_destination_impl(state.inner(), &label, &path)?;
    let _ = crate::rebuild_tray_menu(&app);
    Ok(result)
}

/// `queue_get_pinned()` — return the current pinned-destination list.
#[tauri::command]
pub fn queue_get_pinned(state: tauri::State<'_, AppState>) -> Vec<PinnedDestinationDto> {
    queue_get_pinned_impl(state.inner())
}

/// `queue_unpin_destination(label, path)` — remove the matching row
/// from `Settings::queue::pinned_destinations` and persist. Returns
/// the post-removal list. Idempotent — removing a row that isn't
/// pinned is a no-op (returns the unchanged list). Phase 45.6 —
/// calls [`crate::rebuild_tray_menu`] on success.
#[tauri::command]
pub fn queue_unpin_destination<R: Runtime>(
    state: tauri::State<'_, AppState>,
    app: AppHandle<R>,
    label: String,
    path: String,
) -> Result<Vec<PinnedDestinationDto>, String> {
    let result = queue_unpin_destination_impl(state.inner(), &label, &path)?;
    let _ = crate::rebuild_tray_menu(&app);
    Ok(result)
}

// ---------------------------------------------------------------------
// Test-friendly helpers — these are what the smoke test exercises.
// ---------------------------------------------------------------------

/// Implementation of [`queue_list`]. Public for tests.
pub fn queue_list_impl(state: &AppState) -> Vec<QueueSnapshotDto> {
    state
        .queues
        .queues()
        .into_iter()
        .map(|q| {
            let snap = q.snapshot();
            let badge_count = snap
                .iter()
                .filter(|j| matches!(j.state, JobState::Pending | JobState::Running))
                .count();
            let running = snap.iter().any(|j| j.state == JobState::Running);
            QueueSnapshotDto {
                id: q.id().as_u64(),
                name: q.name().to_string(),
                badge_count,
                running,
            }
        })
        .collect()
}

/// Implementation of [`queue_route_job`]. Public for tests.
pub fn queue_route_job_impl(
    state: &AppState,
    kind: &str,
    src: &str,
    dst: Option<&str>,
) -> Result<RoutedJobDto, String> {
    let kind = job_kind_from_wire(kind)?;
    let src = PathBuf::from(src);
    let dst = dst.map(PathBuf::from);
    let (qid, jid, _control) = state.queues.route(kind, src, dst);
    Ok(RoutedJobDto {
        queue_id: qid.as_u64(),
        job_id: jid.as_u64(),
    })
}

/// Implementation of [`queue_merge`]. Public for tests.
pub fn queue_merge_impl(state: &AppState, src_id: u64, dst_id: u64) -> Result<(), String> {
    state
        .queues
        .merge_into(QueueId::from_u64(src_id), QueueId::from_u64(dst_id))
        .map_err(|e| e.to_string())
}

/// Implementation of [`queue_set_f2_mode`]. Public for tests.
pub fn queue_set_f2_mode_impl(state: &AppState, enabled: bool) {
    state
        .queues
        .auto_enqueue_next
        .store(enabled, Ordering::Relaxed);
}

/// Implementation of [`queue_pin_destination`]. Public for tests.
pub fn queue_pin_destination_impl(
    state: &AppState,
    label: &str,
    path: &str,
) -> Result<Vec<PinnedDestinationDto>, String> {
    let label = label.trim().to_string();
    let path = path.trim().to_string();
    if label.is_empty() {
        return Err("err-pinned-destination-label-empty".to_string());
    }
    if path.is_empty() {
        return Err("err-pinned-destination-path-empty".to_string());
    }
    let entry = PinnedDestination { label, path };
    let mut s = state
        .settings
        .write()
        .map_err(|_| "settings lock poisoned".to_string())?;
    if !s.queue.pinned_destinations.iter().any(|p| p == &entry) {
        s.queue.pinned_destinations.push(entry);
    }
    save_settings(state, &s)?;
    Ok(s.queue
        .pinned_destinations
        .iter()
        .cloned()
        .map(PinnedDestinationDto::from)
        .collect())
}

/// Implementation of [`queue_get_pinned`]. Public for tests.
pub fn queue_get_pinned_impl(state: &AppState) -> Vec<PinnedDestinationDto> {
    state
        .settings_snapshot()
        .queue
        .pinned_destinations
        .into_iter()
        .map(PinnedDestinationDto::from)
        .collect()
}

/// Implementation of [`queue_unpin_destination`]. Public for tests.
pub fn queue_unpin_destination_impl(
    state: &AppState,
    label: &str,
    path: &str,
) -> Result<Vec<PinnedDestinationDto>, String> {
    let label = label.trim().to_string();
    let path = path.trim().to_string();
    let target = PinnedDestination { label, path };
    let mut s = state
        .settings
        .write()
        .map_err(|_| "settings lock poisoned".to_string())?;
    s.queue.pinned_destinations.retain(|p| p != &target);
    save_settings(state, &s)?;
    Ok(s.queue
        .pinned_destinations
        .iter()
        .cloned()
        .map(PinnedDestinationDto::from)
        .collect())
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn job_kind_from_wire(s: &str) -> Result<JobKind, String> {
    Ok(match s {
        "copy" => JobKind::Copy,
        "move" => JobKind::Move,
        "delete" => JobKind::Delete,
        "secure-delete" => JobKind::SecureDelete,
        "verify" => JobKind::Verify,
        other => return Err(format!("unknown job kind: {other:?}")),
    })
}

fn save_settings(state: &AppState, s: &Settings) -> Result<(), String> {
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        // Tests construct AppState with an empty path; skip the
        // persistence step so they don't write into the OS config dir.
        return Ok(());
    }
    s.save_to(path).map_err(|e| format!("save settings: {e}"))
}

/// Subscribe to the registry's broadcast channel and forward every
/// [`QueueRegistryEvent`] to the matching Tauri event so the
/// frontend can react without polling. Returns the spawned
/// `JoinHandle`; callers store it on the runtime so the task
/// outlives the setup hook.
///
/// On `Lagged` (subscriber too slow to keep up with the bounded
/// channel) the pump skips silently and resyncs on the next event:
/// the frontend's reconciliation path is `queue_list()`, which
/// every tab-strip refresh already calls.
pub fn spawn_registry_event_pump(
    app: AppHandle,
    registry: QueueRegistry,
) -> tauri::async_runtime::JoinHandle<()> {
    let mut rx = registry.subscribe();
    tauri::async_runtime::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(QueueRegistryEvent::QueueAdded { id, name }) => {
                    let _ = app.emit(
                        EVENT_QUEUE_ADDED,
                        QueueAddedEvent {
                            id: id.as_u64(),
                            name: name.to_string(),
                        },
                    );
                }
                Ok(QueueRegistryEvent::QueueRemoved { id }) => {
                    let _ = app.emit(
                        EVENT_QUEUE_REMOVED,
                        QueueIdEvent { id: id.as_u64() },
                    );
                }
                Ok(QueueRegistryEvent::QueueMerged { src, dst }) => {
                    let _ = app.emit(
                        EVENT_QUEUE_MERGED,
                        QueueMergedEvent {
                            src: src.as_u64(),
                            dst: dst.as_u64(),
                        },
                    );
                }
                Ok(QueueRegistryEvent::JobRouted { queue_id, job_id }) => {
                    let _ = app.emit(
                        EVENT_QUEUE_JOB_ROUTED,
                        QueueJobRoutedEvent {
                            queue_id: queue_id.as_u64(),
                            job_id: job_id.as_u64(),
                        },
                    );
                }
                Err(RecvError::Lagged(_)) => {
                    // Subscriber fell behind. Frontend's tab-strip
                    // reconcile path is `queue_list()`; resyncing
                    // there is cheaper than buffering events.
                    continue;
                }
                Err(RecvError::Closed) => break,
            }
        }
    })
}
