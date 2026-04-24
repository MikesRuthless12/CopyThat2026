//! Copy That v1.25.0 — Tauri 2.x application shell.
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
pub mod cloud_commands;
pub mod clipboard;
pub mod clipboard_watcher;
pub mod collisions;
pub mod commands;
pub mod dropstack;
pub mod errors;
pub mod global_paste;
pub mod i18n;
pub mod icon;
pub mod ipc;
pub mod live_mirror;
pub mod mount_commands;
pub mod power;
pub mod reveal;
pub mod runner;
pub mod scan_commands;
pub mod shell;
pub mod state;
pub mod sync_commands;
pub mod thumbnail;
pub mod updater;

use std::sync::Mutex;

use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{DragDropEvent, Emitter, Manager, WindowEvent};

use crate::cli::CliAction;
use crate::state::AppState;

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
    let history = open_history_blocking();

    // Phase 12 — load persisted preferences. Failure is non-fatal
    // too: a missing / unreadable `settings.toml` falls through to
    // `Settings::default()`, and a bogus `settings-profiles/`
    // directory just surfaces an empty profile list. Errors get
    // logged to stderr so the first-run log captures them.
    let (settings, settings_path) = load_settings_blocking();
    let profiles = match copythat_settings::ProfileStore::default_store() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("copythat: profiles store init failed: {e}");
            copythat_settings::ProfileStore::new(std::path::PathBuf::new())
        }
    };

    let app_state = state::AppState::new_with(history, settings, settings_path, profiles);

    // Phase 32 — hydrate the cloud-backend registry from the
    // `Settings::remotes.backends` mirror so the first
    // `test_backend_connection` hits memory instead of re-parsing
    // TOML. Best-effort — invalid rows are skipped with a stderr
    // log; a later upsert from the Add-backend wizard brings the
    // registry back in sync with disk.
    {
        let snap = app_state.settings_snapshot();
        cloud_commands::hydrate_registry_from_settings(&app_state.cloud_backends, &snap);
    }

    // Phase 20 — open the resume journal alongside history. Failure
    // is non-fatal: the runner skips checkpointing and the resume
    // modal stays empty, but the app still launches.
    let app_state = match copythat_journal::Journal::open_default() {
        Ok(j) => {
            let unfinished = j.unfinished().unwrap_or_else(|e| {
                eprintln!("copythat: journal scan at startup failed: {e}");
                Vec::new()
            });
            app_state.with_journal(j, unfinished)
        }
        Err(e) => {
            eprintln!("copythat: journal open failed: {e}");
            app_state
        }
    };

    // Post-Phase-12 — system-wide paste hotkey. The plugin registers
    // no combos at build time; `global_paste::register_paste_shortcut`
    // does that from the setup hook based on live settings. Handler
    // dispatches to `handle_paste_press`, which reads the clipboard
    // and funnels files through the existing shell-enqueue event.
    let paste_handler =
        |app: &tauri::AppHandle,
         shortcut: &tauri_plugin_global_shortcut::Shortcut,
         event: tauri_plugin_global_shortcut::ShortcutEvent| {
            // `shortcut.into_string()` renders the same canonical form we
            // persist in settings, modulo case. Compare case-insensitive
            // so "cmdorctrl+shift+v" and "CmdOrCtrl+Shift+V" both match.
            let pressed = shortcut.into_string();
            if !crate::global_paste::shortcut_matches(app, &pressed, event.state()) {
                return;
            }
            crate::global_paste::handle_paste_press(app);
        };

    builder
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(paste_handler)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .on_window_event(|window, event| {
            match event {
                WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
                    eprintln!(
                        "[drop-received] paths={}",
                        paths
                            .iter()
                            .map(|p| p.to_string_lossy().into_owned())
                            .collect::<Vec<_>>()
                            .join(" | ")
                    );
                    let dto = ipc::DropReceivedDto {
                        paths: paths
                            .iter()
                            .map(|p| p.to_string_lossy().into_owned())
                            .collect(),
                    };
                    let _ = window.app_handle().emit(ipc::EVENT_DROP_RECEIVED, dto);
                }
                // Phase 16 — "Minimize to tray on close". When the user
                // toggles the General-tab checkbox ON, pressing the
                // window's X button hides the window into the tray
                // instead of tearing down the process. The tray icon
                // registered below keeps the runtime alive and lets
                // the user restore / quit from its menu.
                WindowEvent::CloseRequested { api, .. } => {
                    let handle = window.app_handle();
                    if let Some(state) = handle.try_state::<AppState>() {
                        let minimize = state.settings_snapshot().general.minimize_to_tray;
                        if minimize {
                            api.prevent_close();
                            let _ = window.hide();
                        }
                    }
                }
                _ => {}
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
            // Phase 10 — cumulative totals + daily buckets + reset.
            commands::history_totals,
            commands::history_daily,
            commands::history_clear_all,
            // Phase 12 — settings + profiles + debug hook.
            commands::get_settings,
            commands::update_settings,
            commands::reset_settings,
            commands::effective_buffer_size,
            commands::list_profiles,
            commands::save_profile,
            commands::load_profile,
            commands::delete_profile,
            commands::export_profile,
            commands::import_profile,
            commands::post_completion_action,
            // Phase 14 — preflight free-space + path-size probes.
            commands::destination_free_bytes,
            commands::path_total_bytes,
            commands::path_sizes_individual,
            commands::path_metadata,
            commands::enumerate_tree_files,
            // Phase 15 — auto-update manifest check + dismiss.
            commands::updater_check_now,
            commands::updater_dismiss_version,
            // Phase 19a — disk-backed scan lifecycle.
            scan_commands::scan_start,
            scan_commands::scan_pause,
            scan_commands::scan_resume,
            scan_commands::scan_cancel,
            scan_commands::scan_list_unfinished,
            // Phase 20 — durable resume journal surface.
            commands::pending_resumes,
            commands::discard_resume,
            commands::discard_all_resumes,
            // Phase 21 — bandwidth shape introspection + schedule lint.
            commands::current_shape_rate,
            commands::validate_schedule_spec,
            // Phase 22 — aggregate conflict dialog (thumbnails +
            // per-pattern rules + conflict profiles).
            commands::thumbnail_for,
            commands::add_conflict_rule,
            commands::current_conflict_rules,
            commands::list_conflict_profiles,
            commands::save_conflict_profile,
            commands::delete_conflict_profile,
            commands::set_active_conflict_profile,
            // Phase 25 — two-way sync pair management + lifecycle.
            sync_commands::list_sync_pairs,
            sync_commands::add_sync_pair,
            sync_commands::remove_sync_pair,
            sync_commands::start_sync,
            sync_commands::pause_sync,
            sync_commands::cancel_sync,
            // Phase 26 — live-mirror loop lifecycle.
            live_mirror::start_live_mirror,
            live_mirror::stop_live_mirror,
            live_mirror::list_live_mirrors,
            // Phase 28 — tray-resident Drop Stack.
            dropstack::dropstack_add,
            dropstack::dropstack_remove,
            dropstack::dropstack_clear,
            dropstack::dropstack_list,
            dropstack::dropstack_toggle_window,
            dropstack::dropstack_copy_all_to,
            dropstack::dropstack_move_all_to,
            // Phase 29 — destination picker + drag-out staging.
            commands::list_directory,
            commands::list_roots,
            commands::drag_out_stage,
            // Phase 31 — power-aware copying test-inject IPC.
            power::inject_power_event,
            // Phase 32 — cloud backend matrix CRUD + test-connection.
            cloud_commands::list_backends,
            cloud_commands::add_backend,
            cloud_commands::update_backend,
            cloud_commands::remove_backend,
            cloud_commands::test_backend_connection,
            // Phase 32c — local <-> backend transfer.
            cloud_commands::copy_local_to_backend,
            cloud_commands::copy_backend_to_local,
            // Phase 33 — mount-as-filesystem CRUD.
            mount_commands::list_mounts,
            mount_commands::mount_snapshot,
            mount_commands::unmount_snapshot,
            mount_commands::mount_backend_name,
        ])
        .setup(move |app| {
            // Phase 16 / 28 — tray icon + menu. Visible regardless
            // of the "minimize to tray" setting; the setting only
            // changes what the window's close button does. Phase 28
            // adds the Drop Stack entry between Show and Quit so the
            // user can pop the stack window from the tray without
            // first restoring the main window.
            let show = MenuItem::with_id(app, "tray-show", "Show", true, None::<&str>)?;
            let dropstack =
                MenuItem::with_id(app, "tray-dropstack", "Drop Stack", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "tray-quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &dropstack, &quit])?;
            let _tray = TrayIconBuilder::with_id("copythat-main-tray")
                .tooltip("Copy That v1.25.0")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event: MenuEvent| match event.id.as_ref() {
                    "tray-show" => show_main_window(app),
                    "tray-dropstack" => {
                        // Fire-and-forget — the Tauri runtime owns
                        // the async context for the command.
                        let handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = dropstack::dropstack_toggle_window(handle).await;
                        });
                    }
                    "tray-quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // Left-click on the tray icon restores the main
                    // window. Right-click opens the menu (Tauri's
                    // default behaviour when `show_menu_on_left_click`
                    // is false).
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            // Register the configured paste hotkey if enabled. Live
            // re-binding (user flips the setting) goes through the
            // `update_paste_shortcut` IPC command; this path is the
            // first-run / cold-start case only.
            let handle = app.handle().clone();
            if let Some(state) = handle.try_state::<AppState>() {
                let snap = state.settings_snapshot();
                if snap.general.paste_shortcut_enabled {
                    if let Err(e) =
                        global_paste::register_paste_shortcut(&handle, &snap.general.paste_shortcut)
                    {
                        eprintln!("[paste-hotkey] initial register failed: {e}");
                    }
                }
                if snap.general.clipboard_watcher_enabled {
                    let watcher = clipboard_watcher::spawn(handle.clone());
                    if let Ok(mut slot) = state.clipboard_watcher.lock() {
                        *slot = Some(watcher);
                    }
                }

                // Phase 21 — apply persisted NetworkSettings on cold
                // start, then spawn the minute-tick schedule poller.
                // The poller re-evaluates `effective_shape_rate`
                // every 60 seconds; if the value changed, it emits
                // `shape-rate-changed` so the header badge updates
                // without a UI poll.
                state::apply_network_settings_to_shape(&state.shape, &snap.network);
                let poll_handle = handle.clone();
                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
                    // Skip the immediate first tick — startup already
                    // applied the rate above.
                    interval.tick().await;
                    let mut last_rate: Option<u64> = poll_handle
                        .try_state::<AppState>()
                        .and_then(|s| s.shape.current_rate().map(|r| r.bytes_per_second()));
                    loop {
                        interval.tick().await;
                        let Some(state) = poll_handle.try_state::<AppState>() else {
                            break;
                        };
                        let snap = state.settings_snapshot();
                        state::apply_network_settings_to_shape(&state.shape, &snap.network);
                        let new_rate = state.shape.current_rate().map(|r| r.bytes_per_second());
                        if new_rate != last_rate {
                            let _ = poll_handle.emit(
                                ipc::EVENT_SHAPE_RATE_CHANGED,
                                commands::build_shape_rate_dto(state.inner()),
                            );
                            last_rate = new_rate;
                        }
                    }
                });
            }

            // Phase 28 — load persisted Drop Stack entries. Paths
            // that no longer resolve are dropped from the stack and
            // announced to the frontend via one event per missing
            // path so the UI can render a one-time toast.
            if let Some(state) = app.handle().try_state::<AppState>() {
                let registry = state.dropstack.clone();
                match registry.load() {
                    Ok(missing) => {
                        for p in &missing {
                            dropstack::emit_path_missing(app.handle(), p);
                        }
                    }
                    Err(e) => {
                        eprintln!("copythat: dropstack load failed: {e}");
                    }
                }
            }

            // Phase 31 — power-aware copying. Spawn the probe poller
            // (real battery + x86 thermal + stubs for the per-OS FFI
            // probes that land in Phase 31b), then the subscriber
            // task that maps PowerEvents into pause_all / resume_all
            // / shape cap via the user's PowerPoliciesSettings.
            if let Some(state) = app.handle().try_state::<AppState>() {
                let app_state: AppState = state.inner().clone();
                let probes = copythat_power::ProbeSet::production();
                let _poller = app_state
                    .power_bus
                    .spawn_poller(probes, copythat_power::bus::DEFAULT_POLL_PERIOD);
                let _subscriber = power::spawn_power_subscriber(app_state, app.handle().clone());
            }

            if let Some(action) = initial_action.lock().ok().and_then(|mut g| g.take()) {
                shell::dispatch_cli_action(&app.handle().clone(), action);
            }

            // Phase 33 — auto-mount the latest snapshot if the user
            // has enabled `settings.mount.mount_on_launch`. Best-
            // effort: any failure logs to stderr and launch proceeds.
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    let state: tauri::State<'_, state::AppState> = handle.state();
                    mount_commands::mount_latest_on_launch(&state).await;
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Copy That v1.25.0");
}

/// Phase 16 — restore the main window from the tray. `show` +
/// `unminimize` + `set_focus` is the idiomatic bring-to-front
/// combination on every Tauri 2.x target; missing pieces no-op.
fn show_main_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
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

/// Phase 12 — load `settings.toml` from the OS config dir. Returns
/// `(Settings, path)` — the companion path is handed to `AppState`
/// so every subsequent `update_settings` writes back to the same
/// file without re-resolving.
fn load_settings_blocking() -> (copythat_settings::Settings, std::path::PathBuf) {
    match copythat_settings::Settings::default_path() {
        Ok(path) => {
            let settings = copythat_settings::Settings::load_from(&path).unwrap_or_else(|e| {
                eprintln!("copythat: settings load failed ({e}); falling back to defaults");
                copythat_settings::Settings::default()
            });
            (settings, path)
        }
        Err(e) => {
            eprintln!("copythat: settings path resolution failed: {e}");
            (
                copythat_settings::Settings::default(),
                std::path::PathBuf::new(),
            )
        }
    }
}
