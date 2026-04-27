//! Phase 19a — Tauri IPC surface for the disk-backed scan engine.
//!
//! The Rust runtime owns each active scan's [`ScanControl`] via a
//! shared [`ScanRegistry`], so the frontend can pause / resume /
//! cancel a scan by id after `scan_start` returns. Scan lifecycle
//! events are forwarded onto the Tauri event bus using the
//! `EVENT_SCAN_*` constants in [`crate::ipc`]; the frontend listens
//! via `tauri::Event::listen` on each event name.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use copythat_core::scan::{self, ScanControl, ScanEvent, ScanId, ScanOptions, Scanner};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

use crate::ipc::{
    ActiveScanDto, EVENT_SCAN_BATCH, EVENT_SCAN_CANCELLED, EVENT_SCAN_COMPLETED, EVENT_SCAN_FAILED,
    EVENT_SCAN_PAUSED, EVENT_SCAN_PROGRESS, EVENT_SCAN_RESUMED, EVENT_SCAN_STARTED, ScanBatchDto,
    ScanCompletedDto, ScanFailedDto, ScanProgressDto, ScanStartedDto,
};
use crate::state::AppState;

/// One row in the scan registry. Cheap to clone — `ctrl` is
/// internally Arc'd.
#[derive(Clone)]
struct ScanHandle {
    ctrl: ScanControl,
    db_path: PathBuf,
    root: PathBuf,
}

/// Tracks every scan the app has spawned so IPC commands can look
/// up the control handle by id. Wrapped in an `Arc<RwLock<..>>` so
/// clones are free and every Tauri command can read without
/// contention.
#[derive(Clone, Default)]
pub struct ScanRegistry {
    inner: Arc<RwLock<HashMap<ScanId, ScanHandle>>>,
}

impl ScanRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&self, id: ScanId, handle: ScanHandle) {
        self.inner
            .write()
            .expect("scan registry poisoned")
            .insert(id, handle);
    }

    fn remove(&self, id: ScanId) {
        self.inner
            .write()
            .expect("scan registry poisoned")
            .remove(&id);
    }

    fn get(&self, id: ScanId) -> Option<ScanHandle> {
        self.inner
            .read()
            .expect("scan registry poisoned")
            .get(&id)
            .cloned()
    }
}

/// Start a scan under a fresh id. Returns the hyphenated UUID so
/// the frontend can correlate subsequent progress events. The
/// scanner runs on a background tokio task; events are forwarded to
/// the Tauri event bus via the private `forward_scan_events` helper
/// in this module.
#[tauri::command]
pub async fn scan_start(
    src: String,
    hash_during_scan: Option<bool>,
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let src_path = PathBuf::from(&src);
    let settings_snapshot = state.settings_snapshot();
    let db_dir = settings_snapshot.scan.database_path.clone();
    let hash = hash_during_scan.unwrap_or(settings_snapshot.scan.hash_during_scan);

    let scan_id = ScanId::new();
    let scanner = Scanner::create(
        scan_id,
        &src_path,
        ScanOptions {
            db_dir,
            hash_during_scan: hash,
            ..ScanOptions::default()
        },
    )
    .map_err(|e| format!("scan create failed: {e}"))?;

    let db_path = scanner.db_path().to_path_buf();
    let root = scanner.root().to_path_buf();
    let ctrl = ScanControl::new();

    let registry = state.scans.clone();
    registry.insert(
        scan_id,
        ScanHandle {
            ctrl: ctrl.clone(),
            db_path: db_path.clone(),
            root: root.clone(),
        },
    );

    let (tx, rx) = mpsc::channel::<ScanEvent>(256);
    let app_for_events = app.clone();
    tokio::spawn(forward_scan_events(app_for_events, scan_id, rx));

    let app_for_run = app.clone();
    let db_for_started = db_path.clone();
    let _ = app.emit(
        EVENT_SCAN_STARTED,
        ScanStartedDto {
            scan_id: scan_id.as_str(),
            root: root.to_string_lossy().to_string(),
            db_path: db_for_started.to_string_lossy().to_string(),
        },
    );

    let registry_for_cleanup = registry.clone();
    let started_at = Instant::now();
    tokio::spawn(async move {
        let result = scanner.run(ctrl, tx).await;
        registry_for_cleanup.remove(scan_id);
        match result {
            Ok(report) => {
                let _ = app_for_run.emit(
                    EVENT_SCAN_COMPLETED,
                    ScanCompletedDto {
                        scan_id: scan_id.as_str(),
                        db_path: report.db_path.to_string_lossy().to_string(),
                        root: report.root.to_string_lossy().to_string(),
                        files: report.stats.total_files,
                        bytes: report.stats.total_bytes,
                        hashed_files: report.hashed_files,
                        duration_ms: started_at.elapsed().as_millis() as u64,
                    },
                );
            }
            Err(err) if err.is_cancelled() => {
                let _ = app_for_run.emit(
                    EVENT_SCAN_CANCELLED,
                    ScanFailedDto {
                        scan_id: scan_id.as_str(),
                        message: "cancelled".to_string(),
                    },
                );
            }
            Err(err) => {
                let _ = app_for_run.emit(
                    EVENT_SCAN_FAILED,
                    ScanFailedDto {
                        scan_id: scan_id.as_str(),
                        message: err.message.clone(),
                    },
                );
            }
        }
    });

    Ok(scan_id.as_str())
}

#[tauri::command]
pub fn scan_pause(scan_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let id = parse_id(&scan_id)?;
    match state.scans.get(id) {
        Some(h) => {
            h.ctrl.pause();
            Ok(())
        }
        None => Err(format!("unknown scan id: {scan_id}")),
    }
}

#[tauri::command]
pub fn scan_resume(scan_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let id = parse_id(&scan_id)?;
    match state.scans.get(id) {
        Some(h) => {
            h.ctrl.resume();
            Ok(())
        }
        None => Err(format!("unknown scan id: {scan_id}")),
    }
}

#[tauri::command]
pub fn scan_cancel(scan_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let id = parse_id(&scan_id)?;
    match state.scans.get(id) {
        Some(h) => {
            h.ctrl.cancel();
            Ok(())
        }
        None => Err(format!("unknown scan id: {scan_id}")),
    }
}

/// List scans that are `Running` or `Paused` in `main.db`. Called
/// on app startup so the UI can offer to resume pending scans.
#[tauri::command]
pub fn scan_list_unfinished() -> Result<Vec<ActiveScanDto>, String> {
    let Some(index_path) = scan::index::default_index_path() else {
        return Ok(Vec::new());
    };
    let Ok(conn) = scan::index::open_index(&index_path) else {
        return Ok(Vec::new());
    };
    let rows = scan::index::list_unfinished(&conn).map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|r| ActiveScanDto {
            scan_id: r.scan_id.as_str(),
            db_path: r.db_path.to_string_lossy().to_string(),
            job_id: r.job_id,
            created_at_ms: r.created_at_ms,
            status: r.status.as_str().to_string(),
        })
        .collect())
}

fn parse_id(s: &str) -> Result<ScanId, String> {
    ScanId::parse(s).map_err(|e| format!("malformed scan id `{s}`: {e}"))
}

async fn forward_scan_events(app: AppHandle, scan_id: ScanId, mut rx: mpsc::Receiver<ScanEvent>) {
    while let Some(evt) = rx.recv().await {
        match evt {
            ScanEvent::Started { .. } => {
                // Already emitted at `scan_start` call time so the
                // frontend has the payload before the first tick of
                // the runner's event loop; ignore the echo here.
            }
            ScanEvent::Progress {
                files_discovered,
                bytes_discovered,
                files_written,
                files_hashed,
            } => {
                let _ = app.emit(
                    EVENT_SCAN_PROGRESS,
                    ScanProgressDto {
                        scan_id: scan_id.as_str(),
                        files_discovered,
                        bytes_discovered,
                        files_written,
                        files_hashed,
                    },
                );
            }
            ScanEvent::BatchFlushed {
                batch_size,
                files_committed,
                last_rel_path,
            } => {
                let _ = app.emit(
                    EVENT_SCAN_BATCH,
                    ScanBatchDto {
                        scan_id: scan_id.as_str(),
                        batch_size,
                        files_committed,
                        last_rel_path,
                    },
                );
            }
            ScanEvent::Paused => {
                let _ = app.emit(
                    EVENT_SCAN_PAUSED,
                    ScanFailedDto {
                        scan_id: scan_id.as_str(),
                        message: "paused".to_string(),
                    },
                );
            }
            ScanEvent::Resumed => {
                let _ = app.emit(
                    EVENT_SCAN_RESUMED,
                    ScanFailedDto {
                        scan_id: scan_id.as_str(),
                        message: "resumed".to_string(),
                    },
                );
            }
            // Terminal events are emitted by `scan_start`'s spawned
            // task using the final `ScanReport`, which carries
            // richer totals than the `ScanEvent::Completed` carries
            // mid-stream; ignore the in-stream variants here. The
            // `_` catches future non-exhaustive additions.
            ScanEvent::Completed { .. } | ScanEvent::Cancelled | ScanEvent::Failed { .. } => {}
            _ => {}
        }
    }
    // Channel closed — the scanner task owns the terminal emission.
    let _ = scan_id;
}

// Accessors on the scan handle — exposed so the runner / future
// commands can fetch the per-scan DB path without re-querying the
// filesystem.
impl ScanHandle {
    #[allow(dead_code)]
    pub fn db_path(&self) -> &std::path::Path {
        &self.db_path
    }
    #[allow(dead_code)]
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }
}
