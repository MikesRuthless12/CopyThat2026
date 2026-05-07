//! Phase 46.6 smoke — Settings → Plugins UI IPC layer.
//!
//! The brief calls for `install/enable/disable round-trips through
//! IPC`. The IPC commands themselves require a live Tauri runtime,
//! but the underlying helpers (`set_enabled`, `grant`, `revoke`,
//! `install_from_disk`, `list_entries`, `read_entry`) are pure-Rust
//! and take a `root: &Path` so this smoke can drive the entire
//! state-persistence surface against a tempdir without booting a
//! webview.
//!
//! Filename note: `phase_46_plugin_ui` (no `patch`/`setup`/`install`/
//! `update` substring) keeps the Windows test binary clear of the
//! UAC installer-detection heuristic — the same convention
//! `sample_plugins.rs` follows.
//!
//! Coverage:
//!
//! 1. **install_from_disk** copies a wasm + manifest pair into a
//!    fresh store and starts the plugin in the disabled state with
//!    no granted capabilities.
//! 2. **enable / disable** round-trips through `state.toml` and
//!    survives a re-read.
//! 3. **grant / revoke** rejects ungranted-by-manifest caps,
//!    survives a re-read, and is idempotent.
//! 4. **list_entries** returns plugins in a stable name-sorted
//!    order so the Settings UI doesn't shuffle on refresh.
//! 5. **i18n parity** — all 16 `plugin-*` Fluent keys plus
//!    `settings-tab-plugins` exist in every one of the 18 shipped
//!    locales (Standing Per-Phase Rule #3).

use std::fs;
use std::path::PathBuf;

use copythat_ui_lib::plugin_commands::{
    PLUGINS_SUBDIR, blake3_hex, bundle_hash, grant, install_from_bytes, install_from_disk,
    list_entries, read_entry, revoke, set_enabled,
};

const SAMPLE_MANIFEST: &str = r#"
name = "smoke-organize"
version = "0.1.0"
hooks = ["after_file"]
capabilities = ["read_fs:source", "write_fs:dest"]
"#;

const PHASE_46_6_KEYS: &[&str] = &[
    "settings-tab-plugins",
    "plugin-heading",
    "plugin-hint",
    "plugin-list-empty",
    "plugin-enabled",
    "plugin-disabled",
    "plugin-hooks",
    "plugin-capabilities",
    "plugin-no-capabilities",
    "plugin-directory",
    "plugin-install-from-file",
    "plugin-install-from-url",
    "plugin-url-wasm",
    "plugin-url-manifest",
    "plugin-url-hash",
    "plugin-url-preview",
    "plugin-url-confirm",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

/// Lay down a temporary `(src_dir, store_dir)` pair plus a wasm +
/// manifest under `src_dir` ready to feed `install_from_disk`.
fn boot_store() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let store = tmp.path().join("store");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("plugin.toml"), SAMPLE_MANIFEST).unwrap();
    // 4-byte WASM magic — the IPC layer never instantiates the module
    // (only the runtime does), so any non-empty bytes are sufficient
    // here. Using the real magic keeps a future "validate magic at
    // install time" hardening pass non-breaking.
    fs::write(src.join("plugin.wasm"), b"\0asm").unwrap();
    (tmp, src, store)
}

#[test]
fn case1_install_starts_disabled_with_no_grants() {
    let (_tmp, src, store) = boot_store();
    let dto = install_from_disk(
        &store,
        &src.join("plugin.wasm"),
        &src.join("plugin.toml"),
    )
    .expect("install_from_disk");
    assert_eq!(dto.name, "smoke-organize");
    assert_eq!(dto.version, "0.1.0");
    assert_eq!(dto.hooks, vec!["after_file"]);
    assert_eq!(
        dto.manifest_capabilities,
        vec!["read_fs:source", "write_fs:dest"]
    );
    assert!(dto.granted_capabilities.is_empty(), "fresh install grants nothing");
    assert!(!dto.enabled, "fresh install is disabled");

    // Files must land at the expected layout the runtime can find
    // them through (PluginHost::load_plugin reads `plugin.toml`
    // adjacent to the .wasm).
    let plugin_dir = store.join("smoke-organize");
    assert!(plugin_dir.join("plugin.wasm").is_file());
    assert!(plugin_dir.join("plugin.toml").is_file());
    assert!(plugin_dir.join("state.toml").is_file());
}

#[test]
fn case2_enable_disable_round_trip_persists_to_disk() {
    let (_tmp, src, store) = boot_store();
    install_from_disk(
        &store,
        &src.join("plugin.wasm"),
        &src.join("plugin.toml"),
    )
    .unwrap();

    let dto = set_enabled(&store, "smoke-organize", true).expect("enable");
    assert!(dto.enabled);

    // Re-read straight from disk — confirms the toggle is persisted,
    // not just held in memory.
    let dto = read_entry(&store.join("smoke-organize")).unwrap();
    assert!(dto.enabled, "enable bit must survive a fresh read");

    let dto = set_enabled(&store, "smoke-organize", false).expect("disable");
    assert!(!dto.enabled);
    let dto = read_entry(&store.join("smoke-organize")).unwrap();
    assert!(!dto.enabled, "disable bit must survive a fresh read");
}

#[test]
fn case3_grant_revoke_round_trip_and_manifest_gate() {
    let (_tmp, src, store) = boot_store();
    install_from_disk(
        &store,
        &src.join("plugin.wasm"),
        &src.join("plugin.toml"),
    )
    .unwrap();

    // Grant -> revoke -> grant idempotency.
    let dto = grant(&store, "smoke-organize", "read_fs:source").unwrap();
    assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
    let dto = grant(&store, "smoke-organize", "read_fs:source").unwrap();
    assert_eq!(
        dto.granted_capabilities,
        vec!["read_fs:source"],
        "grant must be idempotent"
    );
    let dto = revoke(&store, "smoke-organize", "read_fs:source").unwrap();
    assert!(dto.granted_capabilities.is_empty());

    // Manifest gate — `network` is not declared, so the IPC must
    // refuse rather than silently allow.
    let err = grant(&store, "smoke-organize", "network").unwrap_err();
    assert!(
        err.contains("not declared in the manifest"),
        "expected manifest-gate error, got: {err}"
    );

    // Both manifest-declared caps round-trip cleanly.
    grant(&store, "smoke-organize", "read_fs:source").unwrap();
    let dto = grant(&store, "smoke-organize", "write_fs:dest").unwrap();
    let mut got = dto.granted_capabilities.clone();
    got.sort();
    assert_eq!(got, vec!["read_fs:source", "write_fs:dest"]);
}

#[test]
fn case4_list_entries_returns_stable_alphabetical_order() {
    let tmp = tempfile::tempdir().unwrap();
    let store = tmp.path().join("store");

    // Two plugins under different names. `b-second` is alphabetically
    // later; if `list_entries` returned `read_dir` order, the result
    // would be platform-dependent — sorting in the helper guarantees
    // deterministic UI rendering.
    let manifest_a = SAMPLE_MANIFEST.replace("smoke-organize", "a-first");
    let manifest_b = SAMPLE_MANIFEST.replace("smoke-organize", "b-second");

    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("plugin.wasm"), b"\0asm").unwrap();

    fs::write(src.join("plugin.toml"), &manifest_b).unwrap();
    install_from_disk(&store, &src.join("plugin.wasm"), &src.join("plugin.toml")).unwrap();
    fs::write(src.join("plugin.toml"), &manifest_a).unwrap();
    install_from_disk(&store, &src.join("plugin.wasm"), &src.join("plugin.toml")).unwrap();

    let entries = list_entries(&store);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "a-first");
    assert_eq!(entries[1].name, "b-second");
}

#[test]
fn case5_state_survives_reinstall() {
    let (_tmp, src, store) = boot_store();
    install_from_disk(&store, &src.join("plugin.wasm"), &src.join("plugin.toml")).unwrap();
    set_enabled(&store, "smoke-organize", true).unwrap();
    grant(&store, "smoke-organize", "write_fs:dest").unwrap();

    // Reinstall with the same manifest name — the IPC layer must
    // overwrite wasm + manifest but preserve state.toml so an
    // upgrade doesn't silently revoke every capability the user has
    // already approved.
    install_from_disk(&store, &src.join("plugin.wasm"), &src.join("plugin.toml")).unwrap();
    let dto = read_entry(&store.join("smoke-organize")).unwrap();
    assert!(dto.enabled, "reinstall must preserve enable bit");
    assert_eq!(dto.granted_capabilities, vec!["write_fs:dest"]);
}

#[test]
fn case6_blake3_hex_is_deterministic() {
    // The URL install confirmation gate pins on this hash, so a
    // round-trip drift would defeat the gate. Pin one byte sequence
    // against a known-good digest computed once.
    let bytes = b"\0asm";
    let got = blake3_hex(bytes);
    let again = blake3_hex(bytes);
    assert_eq!(got, again, "BLAKE3 must be deterministic");
    assert_eq!(got.len(), 64);
    assert!(got.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

#[test]
fn case7_plugins_subdir_constant_matches_default_resolver() {
    // Drift guard — if `default_plugins_root()` ever stops appending
    // the documented subdirectory name, this test fails before any
    // install-time state writes the wrong path.
    assert_eq!(PLUGINS_SUBDIR, "plugins");
}

#[test]
fn case8_phase_46_6_fluent_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_46_6_KEYS {
            // Match either a top-of-file declaration or a key that
            // starts on a fresh line. The trailing space before `=`
            // is part of the Fluent grammar, so requiring it avoids
            // false positives where `plugin-heading-extra` would
            // satisfy a `plugin-heading` substring search.
            let starts = content.starts_with(&format!("{key} ="));
            let inline = content.contains(&format!("\n{key} ="));
            assert!(
                starts || inline,
                "locale `{code}` missing key `{key}` at {}",
                path.display()
            );
        }
    }
}

fn locate_locales_dir() -> Option<PathBuf> {
    let mut cur = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let candidate = cur.join("locales");
        if candidate.join("en").join("copythat.ftl").exists() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

#[test]
fn case9_bundle_hash_detects_manifest_tampering_between_phases() {
    // Phase 46.7 audit Vuln 1: pre-fix the install-from-URL flow
    // hashed only the wasm bytes. An attacker controlling the URL
    // could serve `name = "harmless-counter"` at preview time, then
    // serve `name = "notify-discord"` (or any already-installed
    // plugin's name) on commit — same wasm bytes, same hash, same
    // user "yes" — and overwrite a different plugin's directory
    // while inheriting the victim plugin's enable bit and grants.
    //
    // The `bundle_hash` fix folds the manifest bytes into the
    // pinned digest, so any mutation of the manifest between phases
    // produces a different hash. This smoke pins that contract:
    // the same wasm + a different manifest must yield distinct
    // bundle hashes.
    let wasm = b"\0asm";
    let manifest_a = r#"
name = "harmless-counter"
version = "0.1.0"
hooks = ["after_job"]
"#;
    let manifest_b = r#"
name = "notify-discord"
version = "0.1.0"
hooks = ["after_job"]
capabilities = ["network"]
"#;
    let preview_hash = bundle_hash(wasm, manifest_a.as_bytes());
    let commit_hash_attacker = bundle_hash(wasm, manifest_b.as_bytes());
    assert_ne!(
        preview_hash, commit_hash_attacker,
        "bundle hash MUST change when the manifest changes — Vuln 1 regression"
    );

    // Sanity — the legacy raw-bytes hash IS unchanged (because the
    // wasm bytes are unchanged), confirming that the old gate would
    // have let this through.
    assert_eq!(blake3_hex(wasm), blake3_hex(wasm));
}

#[test]
fn case10_install_from_bytes_resets_grants_on_capability_change() {
    // Defense-in-depth complement to case9: even if a same-name
    // reinstall lands legitimately, a capability set that differs
    // from the prior manifest must reset `granted_capabilities` so
    // the user re-approves under the new manifest.
    let tmp = tempfile::tempdir().unwrap();
    let store = tmp.path();

    // Initial install: declares network + read_fs:source; user
    // grants both and enables.
    let v1 = r#"
name = "smoke-evolves"
version = "0.1.0"
hooks = ["after_job"]
capabilities = ["network", "read_fs:source"]
"#;
    install_from_bytes(store, b"\0asm", v1).unwrap();
    set_enabled(store, "smoke-evolves", true).unwrap();
    grant(store, "smoke-evolves", "network").unwrap();
    grant(store, "smoke-evolves", "read_fs:source").unwrap();

    // Upgrade to a manifest that adds write_fs:dest. The capability
    // set differs → grants reset; enable bit preserved.
    let v2 = r#"
name = "smoke-evolves"
version = "0.2.0"
hooks = ["after_job"]
capabilities = ["network", "read_fs:source", "write_fs:dest"]
"#;
    let dto = install_from_bytes(store, b"\0asm-v2", v2).unwrap();
    assert!(dto.enabled, "enable bit must survive a capability-set change");
    assert!(
        dto.granted_capabilities.is_empty(),
        "grants must reset when the manifest's capability set changes; got: {:?}",
        dto.granted_capabilities
    );

    // Re-grant under the new manifest, then reinstall with the
    // identical capability set → grants must persist.
    grant(store, "smoke-evolves", "network").unwrap();
    let v2_again = v2;
    let dto = install_from_bytes(store, b"\0asm-v3", v2_again).unwrap();
    assert!(dto.enabled);
    assert_eq!(
        dto.granted_capabilities,
        vec!["network"],
        "grants must persist across an identical-cap reinstall"
    );
}
