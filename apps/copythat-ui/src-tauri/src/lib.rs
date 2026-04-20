//! Copy That 2026 — Tauri 2.x application shell.
//!
//! The Rust side wires the Phase 1–4 engines to the Svelte frontend:
//!
//! - `cli` — argv parsing for shell-integration entry points
//!   (`copythat --enqueue <verb> <paths…>`). Stable across Phase 7
//!   (Windows COM DLL, macOS Finder Sync, Linux `.desktop` / KDE
//!   ServiceMenu / Thunar UCA all funnel through this surface).
//! - `commands` — the `#[tauri::command]` surface the UI calls into.
//! - `runner` — spawns one tokio task per queued job, bridges engine
//!   [`copythat_core::CopyEvent`] onto the Tauri event bus, and keeps
//!   the queue's `bytes_done` / `files_done` / `state` fields in sync
//!   so a fresh `list_jobs` after a reconnect re-renders cleanly.
//! - `shell` — shared enqueue helper plus a dispatcher the
//!   single-instance plugin and the initial-launch setup hook both
//!   call to route a parsed [`cli::CliAction`] into the running app.
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

pub mod cli;
pub mod collisions;
pub mod commands;
pub mod errors;
pub mod i18n;
pub mod icon;
pub mod ipc;
pub mod reveal;
pub mod runner;
pub mod shell;
pub mod state;

use std::sync::Mutex;

use tauri::{DragDropEvent, Emitter, Manager, WindowEvent};

use crate::cli::CliAction;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let argv: Vec<std::ffi::OsString> = std::env::args_os().collect();
    let action = cli::parse_args(argv).unwrap_or_else(|err| {
        // Flag errors print the reason + the usage block and launch
        // normally, so a mis-typed shell extension argv never strands
        // the user with a silent no-op. The binary's primary purpose
        // is still the GUI.
        eprintln!("copythat: {err}");
        eprintln!("{}", cli::HELP);
        CliAction::Run
    });

    match &action {
        CliAction::PrintHelp => {
            println!("{}", cli::HELP);
            return;
        }
        CliAction::PrintVersion => {
            println!("copythat {}", cli::VERSION);
            return;
        }
        _ => {}
    }

    // The setup hook consumes this once; the Mutex<Option<_>> lets
    // the closure be `Fn` (Tauri's setup bound) while still allowing
    // a one-shot move-out on first call.
    let initial_action = Mutex::new(Some(action));

    let mut builder = tauri::Builder::default();

    // Single-instance plugin: routes a second launch's argv back to
    // the first live instance, which re-parses and dispatches.
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(
            |app: &tauri::AppHandle, argv: Vec<String>, _cwd: String| {
                let os_argv: Vec<std::ffi::OsString> =
                    argv.into_iter().map(std::ffi::OsString::from).collect();
                match cli::parse_args(os_argv) {
                    Ok(a) => shell::dispatch_cli_action(app, a),
                    Err(err) => {
                        eprintln!("copythat: second invocation rejected: {err}");
                    }
                }
            },
        ));
    }

    // Phase 9 — open (or create) the SQLite history at the OS
    // user-data directory. Failure is non-fatal: the app still
    // launches with `history: None`, the runner skips recording,
    // and the history drawer shows a typed "unavailable" message.
    let app_state = match open_history_blocking() {
        Some(history) => state::AppState::with_history(history),
        None => state::AppState::new(),
    };

    builder
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
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
            // Phase 8 — error / collision / log surface.
            commands::resolve_error,
            commands::resolve_collision,
            commands::error_log,
            commands::clear_error_log,
            commands::error_log_export,
            commands::retry_elevated,
            // Phase 9 — SQLite history surface.
            commands::history_search,
            commands::history_items,
            commands::history_purge,
            commands::history_export_csv,
            commands::history_rerun,
        ])
        .setup(move |app| {
            if let Some(action) = initial_action.lock().ok().and_then(|mut g| g.take()) {
                shell::dispatch_cli_action(&app.handle().clone(), action);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Copy That 2026");
}

/// Phase 9 — synchronous helper for opening the SQLite history from
/// the non-async `run()` entry point. Spins a private 1-thread
/// Tokio runtime just to call `History::open_default()` (which is
/// async) and block on its completion. Returns `None` on any error
/// so the UI can still boot.
fn open_history_blocking() -> Option<copythat_history::History> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()?;
    match rt.block_on(copythat_history::History::open_default()) {
        Ok(h) => Some(h),
        Err(e) => {
            eprintln!("copythat: history open failed: {e}");
            None
        }
    }
}
