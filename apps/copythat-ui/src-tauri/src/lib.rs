//! Copy That 2026 — Tauri 2.x application shell.
//!
//! The Rust side wires the Phase 1–4 engines to the Svelte frontend:
//!
//! - `commands` — the `#[tauri::command]` surface the UI calls into.
//! - `runner` — spawns one tokio task per queued job, bridges engine
//!   [`copythat_core::CopyEvent`] onto the Tauri event bus, and keeps
//!   the queue's `bytes_done` / `files_done` / `state` fields in sync
//!   so a fresh `list_jobs` after a reconnect re-renders cleanly.
//! - `state::AppState` — shared `Queue` + globals incarnation, cloned
//!   into every command through `State<'_, AppState>`.
//! - `ipc` — serde DTOs that cross the boundary. Field names are
//!   camelCase to match idiomatic TypeScript; event names
//!   (`job-added`, `job-progress`, ...) are kebab-case constants.
//! - `i18n` — Fluent-lite loader: all 18 `.ftl` files are
//!   `include_str!`'d so the packaged binary is self-contained, with
//!   a minimal key-only parser that Phase 11 will replace with real
//!   `fluent-rs`.
//! - `icon` / `reveal` — path→icon classification and a
//!   "show in folder" bridge.
//!
//! Window defaults come from `tauri.conf.json` (720×480, min 560×360,
//! drag-drop enabled). The frontend learns about dropped paths via
//! the `tauri://drag-drop` window event which this crate translates
//! into the `drop-received` IPC event for the Svelte layer.

pub mod commands;
pub mod i18n;
pub mod icon;
pub mod ipc;
pub mod reveal;
pub mod runner;
pub mod state;

use tauri::{DragDropEvent, Emitter, Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(state::AppState::new())
        .on_window_event(|window, event| {
            if let WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) = event {
                let dto = ipc::DropReceivedDto {
                    paths: paths
                        .iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect(),
                };
                let _ = window.app_handle().emit(ipc::EVENT_DROP_RECEIVED, dto);
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_copy,
            commands::start_move,
            commands::pause_job,
            commands::resume_job,
            commands::cancel_job,
            commands::remove_job,
            commands::pause_all,
            commands::resume_all,
            commands::cancel_all,
            commands::list_jobs,
            commands::globals,
            commands::file_icon,
            commands::reveal_in_folder,
            commands::translations,
            commands::available_locales,
            commands::system_locale,
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Copy That 2026");
}
