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
use copythat_journal::{Journal, UnfinishedJob};
use copythat_settings::{ProfileStore, Settings};
use copythat_shape::Shape;

use crate::clipboard_watcher::WatcherHandle;
use crate::collisions::CollisionRegistry;
use crate::errors::ErrorRegistry;
use crate::progress_channel::ProgressChannelRegistry;
use crate::scan_commands::ScanRegistry;

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
    /// Phase 19a — active scan-control handles keyed by scan id.
    /// Populated on `scan_start`, drained by the scanner task when
    /// it exits (normal / cancelled / failed).
    pub scans: ScanRegistry,
    /// Phase 20 — durable resume journal. `None` when the redb file
    /// open failed at startup (read-only profile, locked DB, etc.);
    /// the runner skips checkpointing in that case but otherwise
    /// works unchanged.
    pub journal: Option<Journal>,
    /// Phase 20 — unfinished jobs detected at app start. The
    /// frontend's `ResumePromptModal` reads this once via
    /// `pending_resumes()`, then this slot stays as the canonical
    /// list until the user resumes/discards each row. Wrapped in a
    /// Mutex<Vec<...>> so resolution can drain without cloning the
    /// whole AppState.
    pub startup_unfinished: Arc<Mutex<Vec<UnfinishedJob>>>,
    /// Phase 21 — shared bandwidth-shaping bucket. Always present
    /// (rate `None` = unlimited); the runner attaches a sink
    /// pointing at this Shape to every queued job's CopyOptions.
    /// Hot-updated via `Shape::set_rate` from the schedule poller
    /// task spawned in `lib.rs::run`.
    pub shape: Arc<Shape>,
    /// Phase 25 — registry of actively-running sync pair IDs keyed
    /// to their `SyncControl`. Pause / cancel IPC commands look up
    /// the handle here; absent entries mean the pair is idle.
    pub syncs: crate::sync_commands::SyncRegistry,
    /// Phase 26 — registry of pairs running in live-mirror mode.
    /// Each entry owns a stop flag the watcher loop checks between
    /// iterations.
    pub live_mirrors: crate::live_mirror::LiveMirrorRegistry,
    /// Phase 28 — tray-resident Drop Stack. Holds the list of
    /// staged paths + persists JSON to
    /// `<config-dir>/dropstack.json` on every mutation.
    pub dropstack: crate::dropstack::DropStackRegistry,
    /// Phase 31 — power-aware copying broadcast bus. The runner's
    /// subscriber task consumes PowerEvents, maps them through
    /// `PowerPoliciesSettings`, and drives pause_all / resume_all /
    /// shape cap. Test-only `inject_power_event` IPC shares this
    /// same bus so the smoke test can fire synthetic events through
    /// the real end-to-end path without the OS probes.
    pub power_bus: copythat_power::PowerBus,
    /// Phase 32 — cloud backend matrix. Owns the in-memory
    /// `BackendRegistry` that mirrors `RemoteSettings::backends` on
    /// disk. The Add-backend wizard + test-connection IPC read and
    /// write through this registry; operators are built on-demand by
    /// `copythat_cloud::make_operator` using the persisted config +
    /// the secret pulled from the OS keychain.
    pub cloud_backends: Arc<copythat_cloud::BackendRegistry>,
    /// Phase 33 — mount-as-filesystem registry. Holds one live
    /// `MountHandle` per currently-mounted `job_row_id`. Phase 33b
    /// uses `copythat_mount::NoopBackend` for every mount; Phase 33c
    /// swaps in `fuser` / `winfsp` behind the mount crate's feature
    /// flags.
    pub mounts: crate::mount_commands::MountRegistry,
    /// Phase 34 — audit sink registry. Holds an `Arc<AuditSink>`
    /// when `Settings::audit.enabled` is true and the open
    /// succeeded; the runner records `JobStarted` / `JobCompleted`
    /// / `FileCopied` / `FileFailed` / `CollisionResolved` /
    /// `SettingsChanged` into the sink via helpers in
    /// `audit_commands`.
    pub audit: crate::audit_commands::AuditRegistry,
    /// Phase 37 — mobile-pairing registry. Holds the in-flight
    /// `PairServerHandle` while Settings → Mobile shows the QR; the
    /// runner spins it up via `mobile_pair_start` and tears it down
    /// via `mobile_pair_stop` (or when a successful pairing
    /// commits).
    pub mobile: crate::mobile_commands::MobileRegistry,
    /// Phase 37 follow-up #2 — OS wake-lock guard. `Some` while
    /// the PWA's "Keep desktop awake" toggle is on; release on
    /// drop / Goodbye / Exit so the screensaver isn't permanently
    /// suppressed.
    pub wake_lock: std::sync::Arc<Mutex<Option<copythat_platform::WakeLock>>>,
    /// Phase 42 / Gap #14 — frontend-supplied per-job progress
    /// channels. Empty by default; populated when the UI invokes
    /// `register_progress_channel(jobId, channel)` after `start_copy`.
    /// The runner dual-emits — the legacy `app.emit(EVENT_JOB_PROGRESS,
    /// …)` keeps firing so existing `listen('job:progress', …)`
    /// surfaces work unchanged. See `progress_channel.rs` for the
    /// migration path forward.
    pub progress_channels: ProgressChannelRegistry,
    /// Phase 39 — browser-accessible recovery UI. Holds the live
    /// `copythat_recovery::JoinHandle` while the user has Settings →
    /// Advanced → "Recovery web UI" enabled; `None` when the toggle
    /// is off. `recovery_commands::recovery_apply` is the only
    /// writer.
    pub recovery: crate::recovery_commands::RecoveryRegistry,
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
    /// and a profile store. Phase 20 callers also wire the resume
    /// journal via [`AppState::with_journal`] right after
    /// construction. Production startup (`lib.rs`) does both.
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
            scans: ScanRegistry::new(),
            journal: None,
            startup_unfinished: Arc::new(Mutex::new(Vec::new())),
            // Default-unlimited Shape; the lib.rs startup hook calls
            // `apply_network_settings_to_shape` to honour the persisted
            // NetworkSettings on the first tick.
            shape: Arc::new(Shape::new(None)),
            syncs: crate::sync_commands::SyncRegistry::new(),
            live_mirrors: crate::live_mirror::LiveMirrorRegistry::new(),
            // Default-empty DropStack pointed at the OS config dir
            // (or an ephemeral test path when the default resolver
            // fails — tests override via `with_dropstack_path`).
            dropstack: crate::dropstack::DropStackRegistry::new(
                crate::dropstack::default_dropstack_path()
                    .unwrap_or_else(|| std::path::PathBuf::from("dropstack.json")),
            ),
            // Idle bus — the runner attaches the real probes in
            // `lib.rs::run` once the Tauri runtime is up. Tests and
            // the smoke leave the bus idle and drive it via
            // `inject_power_event`.
            power_bus: copythat_power::PowerBus::new(),
            // Phase 32 — empty registry; `lib.rs::run` hydrates it
            // from `settings.remotes.backends` after Settings load.
            cloud_backends: Arc::new(copythat_cloud::BackendRegistry::new()),
            // Phase 33 — empty mount registry; populated lazily when
            // the user invokes `mount_snapshot` or by
            // `mount_latest_on_launch` at startup if the setting is on.
            mounts: crate::mount_commands::MountRegistry::new(),
            // Phase 34 — idle audit registry; `lib.rs::run` builds
            // a sink from `settings.audit` when the user has the
            // toggle on, otherwise it stays empty.
            audit: crate::audit_commands::AuditRegistry::new(),
            // Phase 37 — idle mobile registry; the user spins the
            // pair-server up on demand from Settings → Mobile.
            mobile: crate::mobile_commands::MobileRegistry::new(),
            // Phase 37 follow-up #2 — wake-lock idle by default.
            // The PWA's "Keep desktop awake" toggle acquires it on
            // demand.
            wake_lock: Arc::new(Mutex::new(None)),
            // Phase 42 / Gap #14 — empty channel registry; frontend
            // opts in per-job via the `register_progress_channel`
            // command (see `progress_channel.rs`).
            progress_channels: ProgressChannelRegistry::new(),
            // Phase 39 — idle recovery server. `recovery_apply` at
            // boot (and on every `update_settings`) flips this to
            // `Some` when the user has the toggle on.
            recovery: crate::recovery_commands::RecoveryRegistry::new(),
        }
    }

    /// Test hook — override the DropStack persistence path. Avoids
    /// tests racing each other over the OS config dir.
    pub fn with_dropstack_path(mut self, path: PathBuf) -> Self {
        self.dropstack = crate::dropstack::DropStackRegistry::new(path);
        self
    }

    /// Phase 20 — attach an opened `Journal` and the
    /// `Vec<UnfinishedJob>` the boot-time sweep produced. Builder-
    /// shaped so `lib.rs::run` can chain it after `new_with`.
    pub fn with_journal(mut self, journal: Journal, unfinished: Vec<UnfinishedJob>) -> Self {
        self.journal = Some(journal);
        self.startup_unfinished = Arc::new(Mutex::new(unfinished));
        self
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

// ---------------------------------------------------------------------
// Phase 21 — translate NetworkSettings -> the live Shape rate.
// ---------------------------------------------------------------------

/// Compute the effective rate the [`Shape`] should be set to right
/// now, given the user's persisted NetworkSettings + the live
/// network/power state from `copythat-shape::auto`.
///
/// Auto-throttle rules win over the configured mode (Off / Fixed /
/// Schedule) because their entire purpose is "override when the
/// matching condition holds". Within the auto rules, the order is
/// `cellular > metered > battery` — most restrictive first, so a
/// laptop on cellular battery doesn't accidentally honour the
/// looser metered cap.
///
/// Returns `None` for unlimited, `Some(ByteRate(0))` for paused,
/// `Some(ByteRate(n))` for cap.
pub fn effective_shape_rate(
    settings: &copythat_settings::NetworkSettings,
) -> Option<copythat_shape::ByteRate> {
    use copythat_settings::{AutoThrottleRule, BandwidthMode};
    use copythat_shape::{ByteRate, NetworkClass, PowerState};

    // Auto rules — Phase 21 stubs always return Unmetered /
    // PluggedIn so these branches don't fire in practice. Wire is
    // ready for the per-OS bridges that Phase 31 lands.
    let net = copythat_shape::current_network_class();
    let power = copythat_shape::current_power_state();

    let candidate_overrides = [
        (net == NetworkClass::Cellular, settings.auto_on_cellular),
        (net == NetworkClass::Metered, settings.auto_on_metered),
        (power == PowerState::OnBattery, settings.auto_on_battery),
    ];
    for (active, rule) in candidate_overrides {
        if !active {
            continue;
        }
        match rule {
            AutoThrottleRule::Unchanged => {}
            AutoThrottleRule::Pause => return Some(ByteRate::new(0)),
            AutoThrottleRule::Cap { bytes_per_second } => {
                return Some(ByteRate::new(bytes_per_second));
            }
        }
    }

    // Fall through to the configured mode.
    match settings.mode {
        BandwidthMode::Off => None,
        BandwidthMode::Fixed => {
            if settings.fixed_bytes_per_second == 0 {
                None
            } else {
                Some(ByteRate::new(settings.fixed_bytes_per_second))
            }
        }
        BandwidthMode::Schedule => {
            // Empty / unparseable schedules degrade to "no rule
            // applies" which is unlimited. Surfacing a parse error
            // is the Settings UI's job; the runner just runs.
            let parsed = copythat_shape::Schedule::parse(&settings.schedule_spec).ok();
            parsed
                .as_ref()
                .and_then(|s| s.current_limit(chrono::Local::now()))
        }
    }
}

/// Recompute + apply the effective rate to the AppState's shared
/// Shape. Called once at startup, on every settings update, and on
/// the schedule poller's minute tick.
pub fn apply_network_settings_to_shape(
    shape: &Shape,
    settings: &copythat_settings::NetworkSettings,
) {
    shape.set_rate(effective_shape_rate(settings));
}
