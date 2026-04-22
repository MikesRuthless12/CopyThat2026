//! Shared application state managed by the Tauri runtime.
//!
//! One `AppState` instance lives inside `tauri::Manager::manage`, cloned
//! cheaply into every command handler via `State<'_, AppState>`. All
//! substate is `Arc`-wrapped so clones are free; the state itself is
//! `Clone + Send + Sync`.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use copythat_core::Queue;
use copythat_history::History;
use copythat_settings::{ProfileStore, Settings};

use crate::clipboard_watcher::WatcherHandle;
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
    /// Phase 9 — SQLite-backed copy history. `None` when the disk
    /// open failed at startup (read-only user profile, locked DB,
    /// permission denied). The runner checks `is_some()` before
    /// recording; the Tauri commands surface a typed error.
    pub history: Option<History>,
    /// Phase 12 — live user preferences. Read-write behind a lock
    /// so hot-reload (IPC `update_settings` call) is visible to the
    /// next `enqueue_jobs` without restart. The `settings_path`
    /// companion field is where the state is persisted on every
    /// update (atomic write via `Settings::save_to`).
    pub settings: Arc<RwLock<Settings>>,
    /// Where the live `settings` are persisted. Defaults to
    /// `Settings::default_path()` — the OS config dir — but tests
    /// override with a tempdir path.
    pub settings_path: Arc<PathBuf>,
    /// Phase 12 — named profile store. Lazily creates its
    /// directory on first save; construction has no IO.
    pub profiles: ProfileStore,
    /// Post-Phase-12 — the clipboard-watcher task handle. `Some`
    /// while the opt-in setting is on; swapped to `None` when the
    /// user toggles off. Drop stops the task within one poll
    /// interval. Wrapped in `Mutex` because `update_settings` needs
    /// a `&mut` view to start / stop without cloning `AppState`.
    pub clipboard_watcher: Arc<Mutex<Option<WatcherHandle>>>,
}

impl AppState {
    /// Construct with history disabled and default-path settings.
    /// Used by tests that don't exercise the filesystem settings
    /// store; production callers use [`AppState::new_with`].
    pub fn new() -> Self {
        Self::new_with(
            None,
            Settings::default(),
            PathBuf::new(),
            ProfileStore::new(PathBuf::new()),
        )
    }

    /// Construct with a ready `History` handle, pre-loaded settings,
    /// and a profile store. All four paths (`history`, `settings`,
    ///   `settings_path`, `profiles`) are resolved in `lib.rs` at
    ///   startup so a failing config-dir resolution bubbles up there
    ///   rather than at every `State<'_, AppState>` access.
    pub fn new_with(
        history: Option<History>,
        settings: Settings,
        settings_path: PathBuf,
        profiles: ProfileStore,
    ) -> Self {
        Self {
            queue: Queue::new(),
            globals: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            errors: ErrorRegistry::new(),
            collisions: CollisionRegistry::new(),
            history,
            settings: Arc::new(RwLock::new(settings)),
            settings_path: Arc::new(settings_path),
            profiles,
            clipboard_watcher: Arc::new(Mutex::new(None)),
        }
    }

    /// Convenience wrapper preserved for callers that predate
    /// Phase 12 (tests that only care about the queue +
    /// history). Uses default settings + an empty profile store.
    pub fn with_history(history: History) -> Self {
        Self::new_with(
            Some(history),
            Settings::default(),
            PathBuf::new(),
            ProfileStore::new(PathBuf::new()),
        )
    }

    /// Snapshot the current settings. Short lock window; callers
    /// should drop before any long-running work.
    pub fn settings_snapshot(&self) -> Settings {
        self.settings
            .read()
            .expect("settings lock poisoned")
            .clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
