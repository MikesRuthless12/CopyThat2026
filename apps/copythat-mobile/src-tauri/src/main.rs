// Phase 37 follow-up #2 — Tauri Mobile binary entry point.
//
// Wraps the Svelte 5 PWA at `apps/copythat-mobile/` in a Tauri 2
// shell so users on iOS / Android can install a native binary
// instead of going through the PWA's "Add to Home Screen" flow.
// The PeerJS pairing + control protocol is identical — the native
// build just gives the user a tappable home-screen icon that opens
// the Svelte UI inside Tauri's webview rather than the system
// browser.
//
// Building this requires the matching toolchain on the host:
//
// - **iOS**: macOS host with Xcode 14+ installed, plus
//   `cargo install tauri-cli` and `rustup target add
//   aarch64-apple-ios x86_64-apple-ios`. Build with
//   `cargo tauri ios build` from the workspace root.
// - **Android**: Android SDK + NDK + JDK 17+ on PATH, plus
//   `cargo install tauri-cli` and `rustup target add
//   aarch64-linux-android armv7-linux-androideabi
//   x86_64-linux-android i686-linux-android`. Build with
//   `cargo tauri android build`.
//
// From a Windows-only desktop the binary won't bundle (Xcode +
// Android SDK are unavailable) but `cargo build` against the
// workspace still type-checks the Rust source so the scaffold
// stays in sync.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    copythat_mobile_app_lib::run();
}
