//! Background clipboard watcher.
//!
//! When the user opts in via `general.clipboard_watcher_enabled`, a
//! single tokio task polls [`crate::clipboard::sequence_token`] every
//! 500 ms. When the token changes AND the new clipboard contains
//! files, we emit [`crate::ipc::EVENT_CLIPBOARD_FILES_DETECTED`] with
//! the path list and the current paste-shortcut combo. The frontend
//! turns that into a subtle toast:
//!
//! > "3 files on clipboard — ⌘⇧V to paste via Copy That"
//!
//! No polling happens when the setting is off. A second toggle to
//! the on state spawns a fresh task; the previous task has already
//! observed its cancellation flag and exited.
//!
//! Why polling vs. native callbacks:
//! - **Windows** has `AddClipboardFormatListener`, but it requires an
//!   HWND and we don't own a raw window (Tauri hides its webview
//!   HWND). `GetClipboardSequenceNumber` is windowless and sufficient.
//! - **macOS** has no push-style clipboard notification — polling
//!   `NSPasteboard.changeCount` is the documented pattern.
//! - **Linux Wayland** sandboxes clipboard access to the focused
//!   window; polling only sees our own reads when Copy That has
//!   focus. That's the expected Wayland behaviour.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};

use crate::clipboard;
use crate::ipc::{ClipboardFilesDetectedDto, EVENT_CLIPBOARD_FILES_DETECTED};
use crate::state::AppState;

/// Handle to a running watcher. Drop-safely stops the background
/// task — the task re-checks `stop.load(Acquire)` on every tick, so
/// cancellation lands within one poll interval.
pub struct WatcherHandle {
    stop: Arc<AtomicBool>,
}

impl WatcherHandle {
    /// Stop the poller. Idempotent — a second `stop()` or a `Drop` is
    /// a no-op. The task itself exits on the next tick.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Poll the clipboard in a background task. `app` is cloned into the
/// task so the caller can drop its handle once the watcher is
/// running. Returns a [`WatcherHandle`] the caller stashes in
/// `AppState` — dropping the handle stops the poller.
///
/// `initial_token` seeds the "have we already notified for this
/// content?" guard so the very first tick doesn't fire a toast for
/// whatever was on the clipboard before the user opted in. The
/// watcher ignores the first token change and only fires from the
/// second onward.
pub fn spawn(app: AppHandle) -> WatcherHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    let app_clone = app.clone();

    tauri::async_runtime::spawn(async move {
        const POLL_INTERVAL: Duration = Duration::from_millis(500);
        // Seed with the current token so the first *change* is what
        // fires the toast — paste-after-launch shouldn't surface a
        // notification for the user's pre-launch copy.
        let mut last_token = clipboard::sequence_token();
        // Rate-limit the "same path list" case: if the user copies
        // the same three files repeatedly (common workflow: re-copy
        // to make sure it "took"), only toast once per ~10s.
        let mut last_paths: Vec<String> = Vec::new();
        let mut last_toast_ms: u128 = 0;
        const DEDUPE_MS: u128 = 10_000;

        while !stop_clone.load(Ordering::Acquire) {
            tokio::time::sleep(POLL_INTERVAL).await;
            if stop_clone.load(Ordering::Acquire) {
                break;
            }
            let token = clipboard::sequence_token();
            if token == last_token {
                continue;
            }
            last_token = token;

            // Only fire when the new clipboard carries files. Text
            // and image pastes shouldn't nag.
            let paths = match clipboard::read_file_paths() {
                Ok(p) if !p.is_empty() => p,
                _ => continue,
            };
            let paths_str: Vec<String> =
                paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();

            // Dedupe by path list within the window.
            let now_ms = now_ms();
            if paths_str == last_paths && now_ms.saturating_sub(last_toast_ms) < DEDUPE_MS {
                continue;
            }
            last_paths = paths_str.clone();
            last_toast_ms = now_ms;

            let combo = app_clone
                .state::<AppState>()
                .settings_snapshot()
                .general
                .paste_shortcut
                .clone();
            let dto = ClipboardFilesDetectedDto {
                count: paths_str.len(),
                paths: paths_str,
                shortcut: combo,
            };
            let _ = app_clone.emit(EVENT_CLIPBOARD_FILES_DETECTED, dto);
        }
    });

    WatcherHandle { stop }
}

fn now_ms() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
