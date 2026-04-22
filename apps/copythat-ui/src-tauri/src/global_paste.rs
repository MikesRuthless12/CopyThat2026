//! System-wide "paste files via Copy That" hotkey.
//!
//! Registers the user-configured combo (default `CmdOrCtrl+Shift+V`)
//! via `tauri-plugin-global-shortcut`. When the combo fires the
//! handler reads file URLs off the OS clipboard ([`crate::clipboard`])
//! and routes them through the same enqueue-or-stage path the drag-
//! drop and shell-extension flows already use:
//!
//! - Files on clipboard → emit `shell-enqueue` event so the frontend
//!   opens DropStagingDialog with the paths pre-seeded. User picks
//!   destination, then the existing `start_copy` / `start_move`
//!   pipeline takes over.
//! - Nothing / non-file content → emit a `clipboard-files-detected`
//!   event with `count: 0` so the UI can surface a subtle "no files
//!   on clipboard" toast. Keeps the keypress from feeling ignored.
//!
//! Re-binding at runtime (user changes the combo in Settings) goes
//! through [`rebind_paste_shortcut`]. Unbinding (toggle off) goes
//! through [`unregister_paste_shortcut`]. Both are idempotent — a
//! no-op on a combo we don't hold is not an error.

use std::path::PathBuf;

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::clipboard;
use crate::ipc::{
    ClipboardFilesDetectedDto, EVENT_CLIPBOARD_FILES_DETECTED, EVENT_SHELL_ENQUEUE,
    ShellEnqueueDto,
};
use crate::state::AppState;

/// Register the configured combo. Call once at startup and every time
/// the user updates the combo in Settings.
///
/// Returns `Err` when the plugin refuses the combo (malformed or
/// already-held by another app). Callers surface the error as a
/// toast — the app still runs, the user just can't use the hotkey
/// until they pick a different combo.
pub fn register_paste_shortcut(app: &AppHandle, combo: &str) -> Result<(), String> {
    let shortcuts = app.global_shortcut();
    // Match the Tauri 2 handler signature: register by string and
    // route the press in the app-wide `on_shortcut` closure installed
    // by the plugin builder below. Here we only register; the global
    // handler fans out by shortcut-id.
    shortcuts
        .register(combo)
        .map_err(|e| format!("register global shortcut `{combo}`: {e}"))
}

/// Drop the combo we previously registered. No-op on combos we don't
/// hold (the plugin returns Err, we swallow — makes the "toggle off"
/// UX forgiving).
pub fn unregister_paste_shortcut(app: &AppHandle, combo: &str) {
    let shortcuts = app.global_shortcut();
    let _ = shortcuts.unregister(combo);
}

/// Re-bind: drop the old combo, register the new one. The `enabled`
/// flag wins over any string the caller hands in — `false` is an
/// unregister, regardless of `new_combo`.
pub fn rebind_paste_shortcut(
    app: &AppHandle,
    old_combo: &str,
    new_combo: &str,
    enabled: bool,
) -> Result<(), String> {
    // Always drop the prior binding first; registering over a live
    // handle silently keeps the old handler on some platforms.
    unregister_paste_shortcut(app, old_combo);
    if !enabled {
        return Ok(());
    }
    register_paste_shortcut(app, new_combo)
}

/// Fire the handler for one paste-hotkey press. Split out so the
/// global `on_shortcut` closure can call it and the tests can drive
/// the post-clipboard path directly.
pub fn handle_paste_press(app: &AppHandle) {
    let paths = match clipboard::read_file_paths() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[paste-hotkey] clipboard read failed: {e}");
            Vec::new()
        }
    };
    let combo = app
        .state::<AppState>()
        .settings_snapshot()
        .general
        .paste_shortcut
        .clone();
    if paths.is_empty() {
        // Tell the UI so it can toast; don't pop the staging dialog
        // for an empty clipboard or a text paste.
        let dto = ClipboardFilesDetectedDto {
            paths: Vec::new(),
            count: 0,
            shortcut: combo,
        };
        let _ = app.emit(EVENT_CLIPBOARD_FILES_DETECTED, dto);
        return;
    }
    emit_shell_enqueue(app, &paths);
}

/// Convert the path list into a `ShellEnqueueDto` and fire the same
/// event `dispatch_cli_action` uses for interactive enqueues — so the
/// frontend's `DropStagingDialog` picks it up unchanged.
///
/// Public so the clipboard watcher's "paste now" affordance (a future
/// UX) can reuse the plumbing.
pub fn emit_shell_enqueue(app: &AppHandle, paths: &[PathBuf]) {
    let dto = ShellEnqueueDto {
        // The staging dialog defaults to "Copy"; the user can flip to
        // Move inside the dialog. Hotkey paste doesn't carry the
        // verb — it just surfaces "these files".
        verb: "copy",
        paths: paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
    };
    let _ = app.emit(EVENT_SHELL_ENQUEUE, dto);
    bring_to_front(app);
}

fn bring_to_front(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

/// Is `combo` the paste-hotkey state currently persisted in the app?
/// Used by the on-shortcut handler to filter out unrelated combos the
/// plugin may surface (e.g. a future Phase-45 F2-queue shortcut).
pub fn is_paste_combo(app: &AppHandle, combo: &str) -> bool {
    let want = app
        .state::<AppState>()
        .settings_snapshot()
        .general
        .paste_shortcut
        .clone();
    combo.eq_ignore_ascii_case(&want)
}

/// Tauri's plugin closure hands back a `Shortcut` struct, not the raw
/// combo string. Normalise both ends to the same lowercased form so
/// our compare matches regardless of user capitalisation.
pub fn shortcut_matches(app: &AppHandle, combo_pressed: &str, state: ShortcutState) -> bool {
    if state != ShortcutState::Pressed {
        return false;
    }
    is_paste_combo(app, combo_pressed)
}
