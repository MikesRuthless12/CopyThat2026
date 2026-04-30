//! Copy That v0.19.84 — Tauri 2.x application shell.
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

pub mod audit_commands;
#[cfg(windows)]
pub mod broker_auth;
pub mod cli;
pub mod clipboard;
pub mod clipboard_watcher;
pub mod cloud_commands;
pub mod collisions;
pub mod commands;
pub mod crypt_commands;
pub mod dropstack;
pub mod errors;
pub mod global_paste;
pub mod i18n;
pub mod icon;
#[cfg(windows)]
pub mod instance_broker;
pub mod ipc;
pub mod ipc_safety;
pub mod live_mirror;
pub mod mobile_commands;
pub mod mount_commands;
pub mod offload_commands;
pub mod power;
pub mod preview_commands;
pub mod progress_channel;
pub mod queue_commands;
pub mod recovery_commands;
pub mod reveal;
pub mod runner;
pub mod sanitize_commands;
pub mod scan_commands;
pub mod shell;
pub mod state;
pub mod sync_commands;
pub mod thumbnail;
pub mod updater;
pub mod version_commands;

use std::sync::Mutex;

use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, DragDropEvent, Emitter, Manager, Runtime, WindowEvent};

use crate::cli::CliAction;
use crate::queue_commands::PinnedDestinationDto;
use crate::state::AppState;

/// Stable id of the main tray icon. Used to look the icon up via
/// [`AppHandle::tray_by_id`] when [`rebuild_tray_menu`] swaps in a
/// freshly-built menu after a pinned destination was added or
/// removed (Phase 45.6).
pub const TRAY_ICON_ID: &str = "copythat-main-tray";

/// Tauri event name fired when the user clicks a pinned destination
/// in the tray menu. Payload is a [`PinnedDestinationDto`]; the
/// frontend listens via `EVENTS.trayTargetClicked` and stashes the
/// destination as the active drop target.
pub const EVENT_TRAY_TARGET_CLICKED: &str = "tray-target-clicked";

/// Menu-id prefix for dynamically-generated pinned-destination
/// items. The suffix after the colon is a stable 64-bit hash of the
/// row's `(label, path)` content (see [`tray_target_menu_id`]). The
/// menu-event handler decodes the hash and looks the row up in the
/// current pinned list — content-addressed so concurrent pin
/// reorders/removals between menu-build and click can never
/// misroute the event to the wrong destination.
const TRAY_TARGET_ID_PREFIX: &str = "tray-target:";

/// Hash a pinned destination into the suffix used in its tray menu
/// id. Stable for the lifetime of one process (DefaultHasher seeds
/// per-process); we never persist these ids, so cross-run stability
/// isn't required. 64 bits is more than enough headroom for the
/// `MAX_PINNED_DESTINATIONS = 50` cap enforced at the IPC layer.
fn tray_target_menu_id(p: &PinnedDestinationDto) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    p.label.hash(&mut h);
    p.path.hash(&mut h);
    format!("{TRAY_TARGET_ID_PREFIX}{:016x}", h.finish())
}

/// Build the main tray menu from the current pinned-destinations
/// snapshot. Static items (`Show`, `Drop Stack`, `Quit`) anchor the
/// menu; pinned destinations are inserted between Drop Stack and
/// Quit, separated by `PredefinedMenuItem::separator`. An empty
/// pinned list collapses to the original 3-item menu.
fn build_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
    pinned: &[PinnedDestinationDto],
) -> tauri::Result<Menu<R>> {
    let show = MenuItem::with_id(app, "tray-show", "Show", true, None::<&str>)?;
    let dropstack =
        MenuItem::with_id(app, "tray-dropstack", "Drop Stack", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "tray-quit", "Quit", true, None::<&str>)?;

    if pinned.is_empty() {
        return Menu::with_items(app, &[&show, &dropstack, &quit]);
    }

    // Build the dynamic pinned-destination items first so the
    // borrow check is satisfied when assembling the final
    // `&[&dyn IsMenuItem]` slice.
    let mut pinned_items: Vec<MenuItem<R>> = Vec::with_capacity(pinned.len());
    for p in pinned.iter() {
        let id = tray_target_menu_id(p);
        // Display "Label — Path" so the user can tell two same-
        // labelled rows apart at a glance.
        let display = format!("{} — {}", p.label, p.path);
        pinned_items.push(MenuItem::with_id(app, id, display, true, None::<&str>)?);
    }
    let sep_top = PredefinedMenuItem::separator(app)?;
    let sep_bot = PredefinedMenuItem::separator(app)?;

    let mut items: Vec<&dyn tauri::menu::IsMenuItem<R>> = Vec::new();
    items.push(&show);
    items.push(&dropstack);
    items.push(&sep_top);
    for it in &pinned_items {
        items.push(it);
    }
    items.push(&sep_bot);
    items.push(&quit);
    Menu::with_items(app, &items)
}

/// Rebuild the live tray menu in place. Cheap — invoked after every
/// `queue_pin_destination` / `queue_unpin_destination` so the menu
/// reflects the persisted list without the user having to relaunch.
pub fn rebuild_tray_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let pinned = match app.try_state::<AppState>() {
        Some(s) => crate::queue_commands::queue_get_pinned_impl(s.inner()),
        None => Vec::new(),
    };
    let menu = build_tray_menu(app, &pinned)?;
    if let Some(tray) = app.tray_by_id(TRAY_ICON_ID) {
        tray.set_menu(Some(menu))?;
    }
    Ok(())
}

/// Resolve the menu id back to the pinned destination it was
/// generated from. Looks the row up by content hash, not by index,
/// so a concurrent pin reorder/removal between menu-build and click
/// returns `None` (the user dropped onto a stale menu — silently
/// no-op) instead of misrouting to whichever row now occupies the
/// old index slot.
fn pinned_target_for_menu_id<R: Runtime>(
    app: &AppHandle<R>,
    menu_id: &str,
) -> Option<PinnedDestinationDto> {
    if !menu_id.starts_with(TRAY_TARGET_ID_PREFIX) {
        return None;
    }
    let state = app.try_state::<AppState>()?;
    let pinned = crate::queue_commands::queue_get_pinned_impl(state.inner());
    pinned
        .into_iter()
        .find(|p| tray_target_menu_id(p) == menu_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Wave-2 observability — install a default tracing subscriber so
    // structured `tracing::warn!` / `info!` / `debug!` calls from the
    // platform + core engines + runner land on stderr instead of being
    // silently dropped. Filter is `COPYTHAT_LOG`-controlled (defaults
    // to `info`); a try-init keeps a future audit-side reconfiguration
    // (Phase 34's `AuditLayer`) from panicking on a double-set.
    init_tracing_subscriber();
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

    // Phase 40 — second-instance fast bail. If our process-wide
    // mutex is already owned, a sibling `copythat-ui` is already
    // running; forward our argv through the named-pipe broker
    // instead of doing the full Tauri boot. Saves ~5-7 seconds
    // per `--enqueue` invocation on Windows. On any failure
    // (no server, pipe busy, write error) we fall through to the
    // normal first-instance boot — the existing
    // tauri-plugin-single-instance still kicks in inside
    // builder.run() as a safety net.
    #[cfg(windows)]
    if matches!(&action, CliAction::Enqueue(_)) && instance_broker::is_second_instance() {
        let argv: Vec<String> = std::env::args().collect();
        match instance_broker::try_forward_argv(&argv) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("[broker] forward failed, falling through to full boot: {e}");
                // fall through; normal boot path will set up the
                // single-instance plugin + builder.run() will
                // detect + forward via the older mechanism.
            }
        }
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
        // Phase 17 follow-up — the Tauri updater plugin is
        // disabled until release infra ships. Previously the
        // `[plugins.updater]` block in `tauri.conf.json` carried a
        // placeholder ed25519 pubkey + an endpoint pointed at an
        // unowned domain, while `bundle.createUpdaterArtifacts` was
        // already false. Loading the plugin in that state was a
        // foot-gun: the moment a future contributor flipped
        // `createUpdaterArtifacts: true` without replacing the
        // pubkey, the unsigned-update window would open. Re-enable
        // when the keypair lands and the manifest endpoint is
        // owned. (`crates/copythat-ui/src/updater.rs` still
        // contains the manifest-prefetch + 24h-throttle helper for
        // when the plugin returns.)
        // .plugin(tauri_plugin_updater::Builder::new().build())
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
            // Phase 42 / Gap #14 — Tauri 2.0 Channel<T> opt-in for
            // hot-path progress. Frontend stays on `listen` until a
            // future phase migrates it.
            commands::register_progress_channel,
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
            commands::quick_hash_for_collision,
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
            // Phase 34 — audit log export + WORM mode.
            audit_commands::audit_status,
            audit_commands::audit_test_write,
            audit_commands::audit_verify,
            audit_commands::audit_verify_file,
            // Phase 35 — encryption + compression status surface.
            crypt_commands::crypt_status,
            // Phase 37 follow-up — mobile pairing + push +
            // PeerJS data-channel command dispatcher.
            mobile_commands::mobile_pair_start,
            mobile_commands::mobile_pair_status,
            mobile_commands::mobile_pair_commit,
            mobile_commands::mobile_pair_stop,
            mobile_commands::mobile_revoke,
            mobile_commands::mobile_send_test_push,
            mobile_commands::mobile_handle_remote_command,
            mobile_commands::mobile_onboarding_qr,
            mobile_commands::mobile_onboarding_dismiss,
            // Phase 39 — browser-accessible recovery UI.
            recovery_commands::recovery_status,
            recovery_commands::recovery_apply,
            recovery_commands::recovery_rotate_token,
            // Phase 40 Part B — SMB compression probe + cloud-VM
            // offload-template wizard.
            offload_commands::smb_compression_state,
            offload_commands::render_offload_template,
            // Phase 41 — pre-execution tree-diff preview.
            preview_commands::compute_tree_diff,
            // Phase 42 Part B — per-file rolling versions panel.
            version_commands::list_versions,
            version_commands::select_versions_to_prune,
            version_commands::prune_versions,
            // Phase 44.2 — SSD-aware whole-drive sanitize IPC.
            sanitize_commands::sanitize_list_devices,
            sanitize_commands::sanitize_capabilities_cmd,
            sanitize_commands::sanitize_run,
            sanitize_commands::sanitize_free_space_trim,
            // Phase 45.2 — named-queue / drag-merge / F2-mode IPC.
            queue_commands::queue_list,
            queue_commands::queue_route_job,
            queue_commands::queue_merge,
            queue_commands::queue_set_f2_mode,
            queue_commands::queue_pin_destination,
            queue_commands::queue_get_pinned,
            queue_commands::queue_unpin_destination,
        ])
        .setup(move |app| {
            // Phase 44.2b — install the platform-native CoW probe so
            // `copythat-secure-delete::shred_file` refuses honestly
            // on Btrfs / ZFS / APFS / bcachefs / ReFS. First-set wins
            // per OnceLock; calling once at startup is correct.
            copythat_secure_delete::set_cow_probe(copythat_platform::is_cow_filesystem);

            // Phase 40 — start the named-pipe broker that future
            // `--enqueue` invocations talk to instead of booting a
            // second Tauri instance. See `instance_broker.rs`.
            // Windows-only — macOS/Linux still rely on
            // tauri-plugin-single-instance's argv-forwarding path.
            #[cfg(windows)]
            instance_broker::start_pipe_server(app.handle().clone());

            // Phase 16 / 28 / 45.6 — tray icon + dynamic menu. Visible
            // regardless of the "minimize to tray" setting; the
            // setting only changes what the window's close button
            // does. Phase 28 added the Drop Stack entry between Show
            // and Quit; Phase 45.6 inserts pinned destinations
            // between them via `build_tray_menu` and rebuilds the
            // menu on every pin/unpin via [`rebuild_tray_menu`].
            let pinned_initial = match app.try_state::<AppState>() {
                Some(s) => crate::queue_commands::queue_get_pinned_impl(s.inner()),
                None => Vec::new(),
            };
            let menu = build_tray_menu(&app.handle().clone(), &pinned_initial)?;
            let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
                .tooltip("Copy That v0.19.84")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event: MenuEvent| {
                    let id = event.id.as_ref();
                    match id {
                        "tray-show" => show_main_window(app),
                        "tray-dropstack" => {
                            // Fire-and-forget — the Tauri runtime
                            // owns the async context for the command.
                            let handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = dropstack::dropstack_toggle_window(handle).await;
                            });
                        }
                        "tray-quit" => {
                            app.exit(0);
                        }
                        _ if id.starts_with(TRAY_TARGET_ID_PREFIX) => {
                            // Phase 45.6 — clicking a pinned
                            // destination forwards the row to the
                            // frontend. The Svelte side stashes it
                            // as the active drop target and
                            // bypasses DropStagingDialog on the next
                            // file drop.
                            if let Some(target) = pinned_target_for_menu_id(app, id) {
                                let _ = app.emit(EVENT_TRAY_TARGET_CLICKED, target);
                            }
                        }
                        _ => {}
                    }
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
                tauri::async_runtime::spawn(async move {
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
                // `spawn_poller` calls bare `tokio::spawn` inside; we
                // need to enter Tauri's tokio context first. See the
                // shell.rs:89 comment about the setup-hook path.
                //
                // `clippy::async_yields_async`: yielding the
                // `JoinHandle` is the entire point — we keep it in
                // `_poller` so the runtime keeps the task alive.
                // Awaiting here would block until the poller exits,
                // which never happens during normal app run.
                #[allow(clippy::async_yields_async)]
                let _poller = tauri::async_runtime::block_on(async {
                    app_state
                        .power_bus
                        .spawn_poller(probes, copythat_power::bus::DEFAULT_POLL_PERIOD)
                });
                let _subscriber = power::spawn_power_subscriber(app_state, app.handle().clone());
            }

            // Phase 45.2 — forward QueueRegistryEvent → Tauri events
            // (`queue-added` / `queue-removed` / `queue-merged` /
            // `queue-job-routed`). The handle returned by
            // `spawn_registry_event_pump` is dropped here on purpose —
            // the spawned task keeps itself alive while AppState is
            // around, and the broadcast channel closes naturally on
            // app shutdown so the loop exits cleanly.
            if let Some(state) = app.handle().try_state::<AppState>() {
                let _pump = queue_commands::spawn_registry_event_pump(
                    app.handle().clone(),
                    state.queues.clone(),
                );
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

            // Phase 34 — open the audit sink if the user has audit
            // logging turned on, then record the app-launch
            // `LoginEvent`. Failures log to stderr and the launch
            // continues with the sink idle (matches the Phase 33
            // mount-on-launch convention).
            {
                let handle = app.handle().clone();
                let state: tauri::State<'_, state::AppState> = handle.state();
                let snap = state.settings_snapshot();
                match audit_commands::build_sink(&snap.audit) {
                    Ok(sink) => {
                        state.audit.set(sink);
                        audit_commands::record_login(&state.audit);
                    }
                    Err(e) => {
                        eprintln!("[audit] startup sink open failed: {e}");
                    }
                }
            }

            // Phase 39 — start the recovery web UI if the user has
            // the toggle on. Best-effort; any failure logs and the
            // launch continues with the server idle (matches every
            // other "auto-start on launch" Phase 33+ convention).
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    let state: tauri::State<'_, state::AppState> = handle.state();
                    if !state.settings_snapshot().recovery.enabled {
                        return;
                    }
                    if let Err(e) = recovery_commands::recovery_apply(state).await {
                        eprintln!("[recovery] startup serve failed: {e}");
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Copy That v0.19.84");
}

/// Wave-2 observability — install the process-wide tracing
/// subscriber once. Honours `COPYTHAT_LOG` (e.g. `COPYTHAT_LOG=debug`,
/// `COPYTHAT_LOG=copythat_core=trace,info`) and falls back to `info`
/// when unset. Uses `try_init` so re-entry (tests that call `run()`
/// multiple times, future audit-side overrides) doesn't panic.
fn init_tracing_subscriber() {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_env("COPYTHAT_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(true)
        .try_init();
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
