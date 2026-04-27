// Phase 16 + Phase 42 — `tauri.conf.json` security note.
//
// `app.security.csp = null` in `tauri.conf.json` is intentional and
// acceptable for Tauri-2 desktop apps because:
//
// - The asset protocol is sandboxed by Tauri itself; loading
//   resources from outside the bundle requires explicit allowlist
//   entries elsewhere in the config.
// - The frontend ships a fixed Svelte bundle compiled at build time;
//   no third-party script ever loads dynamically.
// - There is no in-app web browser surface. Outbound URLs open in
//   the user's default browser via `tauri-plugin-shell`, not in the
//   Tauri webview.
//
// Revisit this decision if any of the above change — specifically
// if we add embedded third-party scripts, an in-app browser tab, or
// a marketplace surface that fetches HTML from network endpoints.
// See `docs/RESEARCH_PHASE_42.md` for the full Phase 42 audit.
// (JSON has no comment syntax, so this rationale lives here next to
// the file it documents rather than inside `tauri.conf.json` itself,
// where Tauri's strict schema rejects unknown sibling fields.)

fn main() {
    // Phase 17g — Linux RELRO + BIND_NOW + noexecstack for the
    // `copythat-ui` Tauri binary. Mirrors the build.rs in
    // `crates/copythat-cli/build.rs`. Other targets fall through
    // — Windows /guard:cf is automatic on MSVC, macOS arm64
    // PAC+BTI is automatic on the Apple linker.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-Wl,-z,relro");
        println!("cargo:rustc-link-arg=-Wl,-z,now");
        println!("cargo:rustc-link-arg=-Wl,-z,noexecstack");
    }
    tauri_build::build()
}
