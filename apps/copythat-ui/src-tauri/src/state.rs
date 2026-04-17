//! Shared application state managed by the Tauri runtime.
//!
//! One `AppState` instance lives inside `tauri::Manager::manage`, cloned
//! cheaply into every command handler via `State<'_, AppState>`. All
//! substate is `Arc`-wrapped so clones are free; the state itself is
//! `Clone + Send + Sync`.

use std::sync::Arc;

use copythat_core::Queue;

/// Top-level shared state wired into Tauri.
#[derive(Clone)]
pub struct AppState {
    /// The job queue. Every command mutates jobs through here; the
    /// queue's broadcast channel is the single source of truth for
    /// lifecycle transitions.
    pub queue: Queue,
    /// Incarnation counter bumped on every progress event —
    /// the runner uses this to decide how often to synthesise a
    /// `globals-tick` payload without calling into the frontend
    /// faster than it can repaint.
    pub globals: Arc<std::sync::atomic::AtomicU64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
            globals: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
