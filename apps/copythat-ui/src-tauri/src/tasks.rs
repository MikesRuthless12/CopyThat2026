//! Phase 49j — tasks & progress center.
//!
//! A [`TaskRegistry`] (mirroring the `ScanRegistry` idiom) is the single
//! place long, mutating, cancellable operations — gc / compaction (49i),
//! backups, migration — report progress, completion, failure, and
//! cancellation. The record store is **AppHandle-free** (the emit methods
//! take the handle as a parameter), so it unit-tests without a Tauri
//! runtime; it also retains a bounded ring of recently-finished tasks so
//! the center can show "just completed".

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::ipc::{
    EVENT_TASK_CANCELLED, EVENT_TASK_COMPLETED, EVENT_TASK_FAILED, EVENT_TASK_PROGRESS,
    EVENT_TASK_STARTED,
};

/// Monotonic task id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskId(pub u64);

/// How many finished tasks the center remembers.
const RECENT_CAP: usize = 50;

/// Wire shape for one task. Mirrors a row in the Tasks center.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: u64,
    /// `"gc"` | `"compact"` | `"migrate"` | …
    pub kind: String,
    pub label: String,
    /// `"running"` | `"completed"` | `"failed"` | `"cancelled"`.
    pub state: String,
    /// `0.0..=1.0` (NaN-guarded to 0.0).
    pub progress: f32,
    pub detail: String,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub error: Option<String>,
}

struct TaskRecord {
    dto: TaskDto,
    cancel: Arc<AtomicBool>,
}

#[derive(Default)]
struct Inner {
    next_id: u64,
    active: HashMap<u64, TaskRecord>,
    recent: VecDeque<TaskDto>,
}

/// Shared task registry. Cheap to clone (an `Arc` inside) so every
/// `State<AppState>` clone shares it.
#[derive(Clone, Default)]
pub struct TaskRegistry {
    inner: Arc<Mutex<Inner>>,
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

fn sane_progress(p: f32) -> f32 {
    if p.is_nan() { 0.0 } else { p.clamp(0.0, 1.0) }
}

impl TaskRegistry {
    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Create a new `Running` task and return its id.
    pub fn create(&self, kind: &str, label: &str) -> TaskId {
        let mut g = self.lock();
        g.next_id += 1;
        let id = g.next_id;
        g.active.insert(
            id,
            TaskRecord {
                dto: TaskDto {
                    id,
                    kind: kind.into(),
                    label: label.into(),
                    state: "running".into(),
                    progress: 0.0,
                    detail: String::new(),
                    started_at_ms: now_ms(),
                    finished_at_ms: None,
                    error: None,
                },
                cancel: Arc::new(AtomicBool::new(false)),
            },
        );
        TaskId(id)
    }

    /// The cooperative-cancel flag for `id` (the worker polls it). A fresh
    /// unset flag is returned for an unknown id.
    pub fn cancel_flag(&self, id: TaskId) -> Arc<AtomicBool> {
        self.lock()
            .active
            .get(&id.0)
            .map(|r| r.cancel.clone())
            .unwrap_or_else(|| Arc::new(AtomicBool::new(false)))
    }

    /// Request cancellation; the worker observes the flag and finishes with
    /// [`Self::mark_cancelled`]. Returns `false` if the task isn't active.
    pub fn request_cancel(&self, id: TaskId) -> bool {
        let g = self.lock();
        match g.active.get(&id.0) {
            Some(r) => {
                r.cancel.store(true, Ordering::Relaxed);
                true
            }
            None => false,
        }
    }

    // ----- record-only mutations (AppHandle-free; unit-testable) -----

    /// Record progress without emitting. Returns the updated DTO.
    pub fn mark_progress(&self, id: TaskId, progress: f32, detail: &str) -> Option<TaskDto> {
        let mut g = self.lock();
        let r = g.active.get_mut(&id.0)?;
        r.dto.progress = sane_progress(progress);
        r.dto.detail = detail.into();
        Some(r.dto.clone())
    }

    fn finish(
        &self,
        id: TaskId,
        state: &str,
        detail: &str,
        error: Option<String>,
    ) -> Option<TaskDto> {
        let mut g = self.lock();
        let mut rec = g.active.remove(&id.0)?;
        rec.dto.state = state.into();
        if !detail.is_empty() {
            rec.dto.detail = detail.into();
        }
        rec.dto.finished_at_ms = Some(now_ms());
        rec.dto.error = error;
        if state != "cancelled" {
            rec.dto.progress = 1.0;
        }
        let dto = rec.dto.clone();
        g.recent.push_front(dto.clone());
        while g.recent.len() > RECENT_CAP {
            g.recent.pop_back();
        }
        Some(dto)
    }

    /// Move a task to `completed` + push it to the recent ring.
    pub fn mark_complete(&self, id: TaskId, detail: &str) -> Option<TaskDto> {
        self.finish(id, "completed", detail, None)
    }

    /// Move a task to `failed` with an error message.
    pub fn mark_failed(&self, id: TaskId, err: &str) -> Option<TaskDto> {
        self.finish(id, "failed", "", Some(err.into()))
    }

    /// Move a task to `cancelled`.
    pub fn mark_cancelled(&self, id: TaskId, detail: &str) -> Option<TaskDto> {
        self.finish(id, "cancelled", detail, None)
    }

    /// Active (newest id first) then recently-finished tasks.
    pub fn list(&self) -> Vec<TaskDto> {
        let g = self.lock();
        let mut active: Vec<TaskDto> = g.active.values().map(|r| r.dto.clone()).collect();
        active.sort_by_key(|d| std::cmp::Reverse(d.id));
        active.into_iter().chain(g.recent.iter().cloned()).collect()
    }

    /// One task (active or recent) by id.
    pub fn get(&self, id: TaskId) -> Option<TaskDto> {
        let g = self.lock();
        g.active
            .get(&id.0)
            .map(|r| r.dto.clone())
            .or_else(|| g.recent.iter().find(|d| d.id == id.0).cloned())
    }

    // ----- emit wrappers (take the AppHandle) -----

    /// Emit `task-started` for `id`.
    pub fn started(&self, app: &AppHandle, id: TaskId) {
        if let Some(dto) = self.get(id) {
            let _ = app.emit(EVENT_TASK_STARTED, dto);
        }
    }
    /// Record progress + emit `task-progress`.
    pub fn update(&self, app: &AppHandle, id: TaskId, progress: f32, detail: &str) {
        if let Some(dto) = self.mark_progress(id, progress, detail) {
            let _ = app.emit(EVENT_TASK_PROGRESS, dto);
        }
    }
    /// Complete + emit `task-completed`.
    pub fn complete(&self, app: &AppHandle, id: TaskId, detail: &str) {
        if let Some(dto) = self.mark_complete(id, detail) {
            let _ = app.emit(EVENT_TASK_COMPLETED, dto);
        }
    }
    /// Fail + emit `task-failed`.
    pub fn fail(&self, app: &AppHandle, id: TaskId, err: &str) {
        if let Some(dto) = self.mark_failed(id, err) {
            let _ = app.emit(EVENT_TASK_FAILED, dto);
        }
    }
    /// Cancel + emit `task-cancelled`.
    pub fn cancelled(&self, app: &AppHandle, id: TaskId, detail: &str) {
        if let Some(dto) = self.mark_cancelled(id, detail) {
            let _ = app.emit(EVENT_TASK_CANCELLED, dto);
        }
    }
}

// ----- Tauri commands -----

/// All tasks (active + recent), newest first.
#[tauri::command]
pub fn tasks_list(state: tauri::State<'_, crate::state::AppState>) -> Vec<TaskDto> {
    state.tasks.list()
}

/// One task by id, if known.
#[tauri::command]
pub fn task_get(state: tauri::State<'_, crate::state::AppState>, id: u64) -> Option<TaskDto> {
    state.tasks.get(TaskId(id))
}

/// Request cooperative cancellation of a running task. Returns false if the
/// task isn't active.
#[tauri::command]
pub fn task_cancel(state: tauri::State<'_, crate::state::AppState>, id: u64) -> bool {
    state.tasks.request_cancel(TaskId(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_create_progress_complete() {
        let reg = TaskRegistry::default();
        let id = reg.create("gc", "Cleanup");
        assert_eq!(reg.get(id).unwrap().state, "running");
        reg.mark_progress(id, 0.5, "halfway");
        assert_eq!(reg.get(id).unwrap().progress, 0.5);
        reg.mark_complete(id, "done");
        let dto = reg.get(id).unwrap();
        assert_eq!(dto.state, "completed");
        assert_eq!(dto.progress, 1.0);
        assert!(dto.finished_at_ms.is_some());
        assert!(reg.list().iter().any(|d| d.id == id.0));
    }

    #[test]
    fn fail_and_cancel_paths() {
        let reg = TaskRegistry::default();
        let a = reg.create("migrate", "Import");
        reg.mark_failed(a, "disk full");
        let dto = reg.get(a).unwrap();
        assert_eq!(dto.state, "failed");
        assert_eq!(dto.error.as_deref(), Some("disk full"));

        let b = reg.create("compact", "Compaction");
        let flag = reg.cancel_flag(b);
        assert!(!flag.load(Ordering::Relaxed));
        assert!(reg.request_cancel(b));
        assert!(flag.load(Ordering::Relaxed));
        reg.mark_cancelled(b, "");
        assert_eq!(reg.get(b).unwrap().state, "cancelled");

        // request_cancel on an unknown id is a no-op false.
        assert!(!reg.request_cancel(TaskId(9999)));
    }

    #[test]
    fn nan_progress_is_zeroed_and_recent_is_bounded() {
        let reg = TaskRegistry::default();
        let id = reg.create("gc", "x");
        reg.mark_progress(id, f32::NAN, "");
        assert_eq!(reg.get(id).unwrap().progress, 0.0);

        // Recent ring stays bounded at RECENT_CAP.
        for _ in 0..(RECENT_CAP + 10) {
            let t = reg.create("gc", "y");
            reg.mark_complete(t, "");
        }
        let finished = reg.list().iter().filter(|d| d.state == "completed").count();
        assert!(finished <= RECENT_CAP, "recent ring should be bounded");
    }
}
