//! Phase 26 — live-mirror runner + registry.
//!
//! One [`LiveMirrorHandle`] per active sync pair in live mode.
//! Each instance owns:
//!
//! - A [`copythat_watch::Watcher`] rooted at the pair's left side,
//!   whose debounced events act as "something changed" pulses.
//! - A tokio task that loops: (1) run a full `copythat_sync::sync`
//!   on the pair, (2) park on `watcher.next_async()` until an event
//!   arrives, (3) coalesce any further events that arrive in the
//!   next short window, (4) go back to step 1.
//! - A stop flag (`AtomicBool`) the frontend flips via
//!   `stop_live_mirror` to terminate the loop cleanly between
//!   iterations.
//!
//! Events after the first sync but before the second are NOT
//! dropped — the watcher's internal debounce queue buffers them.
//! The runner just re-enters `sync()` when any pending pulse is
//! available.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use copythat_sync::{SyncControl, SyncEvent, SyncMode, SyncOptions, SyncPair, sync};
use copythat_watch::{Watcher, WatcherOptions};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

use crate::ipc::{
    EVENT_LIVE_MIRROR_EVENT, EVENT_LIVE_MIRROR_STARTED, EVENT_LIVE_MIRROR_STOPPED, LiveMirrorDto,
    LiveMirrorEventDto,
};
use crate::state::AppState;

/// Coalescing window: after the watcher emits one debounced event,
/// wait this long for additional events before kicking off a re-sync.
/// Keeps a burst of N files from triggering N syncs.
const COALESCE_WINDOW: Duration = Duration::from_millis(300);

/// One running live-mirror.
#[derive(Clone)]
pub struct LiveMirrorHandle {
    pub stop: Arc<AtomicBool>,
}

/// Registry of active live mirrors, keyed by pair id.
#[derive(Clone, Default)]
pub struct LiveMirrorRegistry {
    inner: Arc<RwLock<HashMap<String, LiveMirrorHandle>>>,
}

impl LiveMirrorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&self, pair_id: String, handle: LiveMirrorHandle) {
        self.inner
            .write()
            .expect("live mirror registry poisoned")
            .insert(pair_id, handle);
    }

    fn remove(&self, pair_id: &str) -> Option<LiveMirrorHandle> {
        self.inner
            .write()
            .expect("live mirror registry poisoned")
            .remove(pair_id)
    }

    fn contains(&self, pair_id: &str) -> bool {
        self.inner
            .read()
            .expect("live mirror registry poisoned")
            .contains_key(pair_id)
    }

    pub fn active_ids(&self) -> Vec<String> {
        self.inner
            .read()
            .expect("live mirror registry poisoned")
            .keys()
            .cloned()
            .collect()
    }
}

/// Launch a live-mirror loop for `pair_id`. Returns an error if the
/// pair is unknown, already in live mode, or if the watcher fails
/// to open. The returned future completes once the loop exits
/// (stop flag set or watcher channel closed).
#[tauri::command]
pub async fn start_live_mirror(
    pair_id: String,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    if state.live_mirrors.contains(&pair_id) {
        return Err(format!("pair {pair_id} is already in live mode"));
    }

    // Take a cheap snapshot of the pair config so the runner task
    // doesn't hold a lock across its lifetime.
    let (cfg, host_label_override) = {
        let snap = state.settings_snapshot();
        let cfg = snap
            .sync
            .pairs
            .iter()
            .find(|p| p.id == pair_id)
            .cloned()
            .ok_or_else(|| format!("unknown pair id: {pair_id}"))?;
        (cfg, snap.sync.host_label_override.clone())
    };

    let stop = Arc::new(AtomicBool::new(false));
    state.live_mirrors.insert(
        pair_id.clone(),
        LiveMirrorHandle {
            stop: Arc::clone(&stop),
        },
    );

    let _ = app.emit(
        EVENT_LIVE_MIRROR_STARTED,
        LiveMirrorDto {
            pair_id: pair_id.clone(),
            left: cfg.left.clone(),
        },
    );

    let registry = state.live_mirrors.clone();
    let pair_id_for_task = pair_id.clone();
    let app_for_task = app.clone();
    tokio::spawn(async move {
        run_loop(
            pair_id_for_task.clone(),
            cfg,
            host_label_override,
            stop,
            app_for_task.clone(),
        )
        .await;
        registry.remove(&pair_id_for_task);
        let _ = app_for_task.emit(
            EVENT_LIVE_MIRROR_STOPPED,
            LiveMirrorDto {
                pair_id: pair_id_for_task,
                left: String::new(),
            },
        );
    });

    Ok(pair_id)
}

/// Signal a live-mirror loop to exit. Idempotent — stopping an
/// already-stopped or never-started pair is a no-op-ish Err.
#[tauri::command]
pub fn stop_live_mirror(pair_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    match state.live_mirrors.remove(&pair_id) {
        Some(h) => {
            h.stop.store(true, Ordering::Release);
            Ok(())
        }
        None => Err(format!("pair {pair_id} is not in live mode")),
    }
}

/// Snapshot of pair ids currently in live mode.
#[tauri::command]
pub fn list_live_mirrors(state: tauri::State<'_, AppState>) -> Vec<String> {
    state.live_mirrors.active_ids()
}

async fn run_loop(
    pair_id: String,
    cfg: copythat_settings::SyncPairConfig,
    host_label_override: String,
    stop: Arc<AtomicBool>,
    app: AppHandle,
) {
    let left = PathBuf::from(&cfg.left);
    let right = PathBuf::from(&cfg.right);

    // Open the watcher before the first sync so events that happen
    // during that sync are buffered rather than missed.
    let watcher = match Watcher::with_options(&left, WatcherOptions::default()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[live-mirror] watcher open failed for pair {pair_id}: {e}");
            return;
        }
    };
    let watcher = Arc::new(Mutex::new(watcher));

    let mode = match cfg.mode {
        copythat_settings::SyncModeChoice::TwoWay => SyncMode::TwoWay,
        copythat_settings::SyncModeChoice::MirrorLeftToRight => SyncMode::MirrorLeftToRight,
        copythat_settings::SyncModeChoice::MirrorRightToLeft => SyncMode::MirrorRightToLeft,
        copythat_settings::SyncModeChoice::ContributeLeftToRight => SyncMode::ContributeLeftToRight,
    };

    // First sync: run once unconditionally so the pair starts from
    // a reconciled baseline. Subsequent syncs only fire when the
    // watcher pulses.
    run_one_sync(&pair_id, &cfg, &host_label_override, &left, &right, mode).await;

    loop {
        if stop.load(Ordering::Acquire) {
            break;
        }

        // Wait for a pulse from the watcher.
        let watcher_for_wait = Arc::clone(&watcher);
        let maybe_event = tokio::task::spawn_blocking(move || {
            let mut w = watcher_for_wait.lock().unwrap();
            w.next()
        })
        .await
        .ok()
        .flatten();

        if stop.load(Ordering::Acquire) {
            break;
        }

        if let Some(evt) = maybe_event {
            let _ = app.emit(
                EVENT_LIVE_MIRROR_EVENT,
                LiveMirrorEventDto {
                    pair_id: pair_id.clone(),
                    subject: evt.subject().to_string_lossy().into_owned(),
                },
            );
        } else {
            // Watcher closed — nothing more to do.
            break;
        }

        // Coalesce: drain any additional events that arrive within
        // `COALESCE_WINDOW` before kicking off the sync. Prevents a
        // burst of 100 files from triggering 100 back-to-back syncs.
        let watcher_for_drain = Arc::clone(&watcher);
        let _ = tokio::time::timeout(COALESCE_WINDOW, async {
            tokio::task::spawn_blocking(move || {
                let mut w = watcher_for_drain.lock().unwrap();
                while w.next().is_some() {
                    // Drain — every extra event coalesces into the
                    // upcoming sync round.
                }
            })
            .await
        })
        .await;

        if stop.load(Ordering::Acquire) {
            break;
        }

        run_one_sync(&pair_id, &cfg, &host_label_override, &left, &right, mode).await;
    }
}

async fn run_one_sync(
    pair_id: &str,
    cfg: &copythat_settings::SyncPairConfig,
    host_label_override: &str,
    left: &PathBuf,
    right: &PathBuf,
    mode: SyncMode,
) {
    let mut pair_builder = SyncPair::new(cfg.label.clone(), left, right);
    if !cfg.db_path_override.is_empty() {
        pair_builder = pair_builder.with_db_path(cfg.db_path_override.clone());
    }
    if !host_label_override.is_empty() {
        pair_builder = pair_builder.with_host_label(host_label_override);
    }
    let pair = pair_builder;
    let ctrl = SyncControl::new();
    let (tx, mut rx) = mpsc::channel::<SyncEvent>(256);

    // Drain sync events without forwarding — live mirror uses its
    // own event stream (`live-mirror-event`). The detailed sync-*
    // events would overwhelm the UI if fired on every watcher pulse.
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let opts = SyncOptions::default();
    if let Err(e) = sync(&pair, mode, opts, ctrl, tx).await {
        eprintln!("[live-mirror] sync round failed for pair {pair_id}: {e}");
    }
    let _ = drain.await;
}
