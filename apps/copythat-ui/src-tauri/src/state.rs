//! Shared application state managed by the Tauri runtime.
//!
//! One `AppState` instance lives inside `tauri::Manager::manage`, cloned
//! cheaply into every command handler via `State<'_, AppState>`. All
//! substate is `Arc`-wrapped so clones are free; the state itself is
//! `Clone + Send + Sync`.

use std::sync::Arc;

use copythat_core::Queue;

use crate::collisions::CollisionRegistry;
use crate::errors::ErrorRegistry;

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
    /// Phase 8 — pending error prompts awaiting user resolution,
    /// plus the in-memory error log the footer drawer reads from.
    pub errors: ErrorRegistry,
    /// Phase 8 — pending collision prompts. Engine emits
    /// `CopyEvent::Collision` → runner parks the oneshot here →
    /// frontend's `CollisionModal` replies via `resolve_collision`.
    pub collisions: CollisionRegistry,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
            globals: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            errors: ErrorRegistry::new(),
            collisions: CollisionRegistry::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
