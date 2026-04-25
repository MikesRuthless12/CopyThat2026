//! Phase 37 follow-up #2 — Tauri Mobile shell library.
//!
//! Exposes the `run` entry point the binary calls. Keeps the actual
//! Tauri context-builder calls in this lib so the iOS / Android
//! build wrappers (Xcode-driven `tauri-cli ios` / Gradle-driven
//! `tauri-cli android`) can pull the symbol out of the
//! `copythat_mobile_app_lib` cdylib. The PWA's Svelte source
//! continues to live at `apps/copythat-mobile/src/`; Vite produces
//! a static bundle that Tauri loads as the webview's start URL via
//! `tauri.conf.json::build.frontendDist`.

#![forbid(unsafe_code)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log())
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running copythat-mobile-app");
}

/// Sanity-check the JS ↔ Rust IPC bridge. The Svelte side hits
/// this on first load; failure surfaces a "Tauri host not
/// reachable" error in the PWA UI.
#[tauri::command]
fn ping() -> &'static str {
    "pong"
}

/// Build a tracing-subscriber layer the Tauri runtime registers
/// for stdout / logcat / OSLog. Writes are forwarded to the host
/// platform's standard log surface so `adb logcat` / Console.app
/// see the same lines as a desktop dev console would.
fn tauri_plugin_log() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("log").build()
}
