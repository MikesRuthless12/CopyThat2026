//! Phase 17g smoke test — binary hardening flags.
//!
//! Filesystem-only tripwire — never touches a real linker. Asserts:
//!
//! 1. The workspace `[profile.release]` keeps `panic = "abort"` (no
//!    unwinding gadget surface) and now upgrades `lto` from `thin`
//!    to `fat` for a tighter dead-code prune.
//! 2. The CLI crate ships a `build.rs` that emits Linux
//!    `-Wl,-z,relro`, `-Wl,-z,now`, and `-Wl,-z,noexecstack`.
//! 3. The Tauri crate's existing `build.rs` carries the same Linux
//!    linker flags (the binary that ends up in the user's
//!    `Applications/` / `Program Files/` is the tauri-ui one, not
//!    the CLI; both binaries get the same hardening).
//! 4. `docs/SECURITY.md` documents Phase 17g as "shipped".

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &Path = &here;
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = match cur.parent() {
            Some(p) => p,
            None => break,
        };
    }
    panic!("could not locate repo root from {}", here.display());
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

#[test]
fn workspace_release_profile_uses_fat_lto_and_panic_abort() {
    let body = read(&repo_root().join("Cargo.toml"));
    assert!(
        body.contains("lto = \"fat\""),
        "workspace [profile.release] must use lto=\"fat\" (Phase 17g)",
    );
    assert!(
        body.contains("panic = \"abort\""),
        "workspace [profile.release] must keep panic=\"abort\" (Phase 17g + 17a)",
    );
    assert!(
        body.contains("strip = \"symbols\""),
        "workspace [profile.release] must keep strip=\"symbols\" (size + reverse-engineering reduction)",
    );
}

#[test]
fn cli_build_script_emits_linux_hardening_flags() {
    let body = read(&repo_root().join("crates/copythat-cli/build.rs"));
    for flag in [
        "-Wl,-z,relro",
        "-Wl,-z,now",
        "-Wl,-z,noexecstack",
        "rustc-link-arg",
        "linux",
    ] {
        assert!(
            body.contains(flag),
            "crates/copythat-cli/build.rs missing Phase 17g flag: {flag}",
        );
    }
}

#[test]
fn cli_cargo_toml_declares_build_script() {
    let body = read(&repo_root().join("crates/copythat-cli/Cargo.toml"));
    assert!(
        body.contains("build = \"build.rs\""),
        "crates/copythat-cli/Cargo.toml must declare its build.rs",
    );
}

#[test]
fn tauri_build_script_emits_linux_hardening_flags() {
    let body = read(&repo_root().join("apps/copythat-ui/src-tauri/build.rs"));
    for flag in ["-Wl,-z,relro", "-Wl,-z,now", "-Wl,-z,noexecstack"] {
        assert!(
            body.contains(flag),
            "apps/copythat-ui/src-tauri/build.rs missing Phase 17g flag: {flag}",
        );
    }
    assert!(
        body.contains("tauri_build::build()"),
        "tauri build.rs must still call tauri_build::build()",
    );
}

#[test]
fn security_md_documents_phase_17g_as_shipped() {
    // Phase 17g shipped — make sure docs/SECURITY.md picks up the
    // build-hardening section once `cargo audit` and friends ship.
    let body = read(&repo_root().join("docs/SECURITY.md"));
    // The section already lists the per-OS targets; we just need to
    // make sure the `[profile.release]` change is documented.
    assert!(
        body.contains("RELRO") || body.contains("relro"),
        "docs/SECURITY.md must mention RELRO when 17g hardening is live",
    );
}
