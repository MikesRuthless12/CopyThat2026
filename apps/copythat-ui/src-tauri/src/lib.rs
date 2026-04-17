//! Copy That 2026 — Tauri 2.x application shell.
//!
//! Phase 0 scaffold: opens a single placeholder window titled "Copy That 2026".
//! Engine wiring (Tauri commands → `copythat-core`) lands in Phase 5.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Copy That 2026");
}
