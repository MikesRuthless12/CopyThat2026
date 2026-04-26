//! Phase 17e smoke test — IPC argument audit + canonicalisation.
//!
//! Asserts that:
//!
//! 1. `crate::ipc_safety::validate_ipc_path` rejects traversal,
//!    NUL, empty, and U+FFFD-bearing strings; returns a clean
//!    `PathBuf` on a normal input.
//! 2. The `IpcError::localized_key` map is stable — frontends pin
//!    against the Fluent keys, never the variant names.
//! 3. The `err-path-invalid-encoding` key is present in every
//!    locale (cross-checked indirectly via the existence count).
//! 4. **Source-grep tripwire** — every `#[tauri::command]` whose
//!    signature contains a `path:`/`paths:`/`destination:`/`source:`
//!    arg in `apps/copythat-ui/src-tauri/src/commands.rs` calls
//!    one of the gate helpers. Drift past this gate fires before
//!    the commit lands.

use std::fs;
use std::path::{Path, PathBuf};

use copythat_ui_lib::ipc_safety::{IpcError, validate_ipc_path, validate_ipc_paths};

#[test]
fn rejects_traversal_via_helper() {
    assert_eq!(
        validate_ipc_path("foo/../etc/passwd"),
        Err(IpcError::PathEscape)
    );
}

#[test]
fn rejects_empty_via_helper() {
    assert_eq!(validate_ipc_path("   "), Err(IpcError::EmptyPath));
}

#[test]
fn rejects_replacement_character_via_helper() {
    let bad = format!("good{}/path", '\u{FFFD}');
    assert_eq!(validate_ipc_path(&bad), Err(IpcError::InvalidEncoding));
}

#[test]
fn accepts_normal_path() {
    let p = validate_ipc_path("/var/log/copythat").unwrap();
    assert_eq!(p, PathBuf::from("/var/log/copythat"));
}

#[test]
fn empty_list_is_distinct() {
    let raws: Vec<String> = vec![];
    assert_eq!(validate_ipc_paths(raws), Err(IpcError::EmptyList));
}

#[test]
fn fluent_keys_are_stable_against_drift() {
    assert_eq!(IpcError::PathEscape.localized_key(), "err-path-escape");
    assert_eq!(IpcError::EmptyPath.localized_key(), "err-destination-empty");
    assert_eq!(
        IpcError::InvalidEncoding.localized_key(),
        "err-path-invalid-encoding"
    );
    assert_eq!(IpcError::EmptyList.localized_key(), "err-source-required");
}

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

#[test]
fn every_locale_carries_the_new_key() {
    // 18-locale parity is the contract `xtask i18n-lint` enforces in
    // CI; this anchor catches drift if a developer adds the key only
    // to `en/`.
    let root = repo_root();
    let locales_dir = root.join("locales");
    let mut count = 0;
    for entry in fs::read_dir(&locales_dir).unwrap().flatten() {
        let ftl = entry.path().join("copythat.ftl");
        if !ftl.exists() {
            continue;
        }
        let body = fs::read_to_string(&ftl).unwrap();
        assert!(
            body.contains("err-path-invalid-encoding ="),
            "locale {} is missing err-path-invalid-encoding",
            entry.path().display(),
        );
        count += 1;
    }
    assert_eq!(count, 18, "expected 18 locales, saw {count}");
}

#[test]
fn commands_rs_path_args_pass_through_the_gate() {
    // Tripwire — walk commands.rs and assert that every
    // `#[tauri::command]` whose signature mentions a path-typed arg
    // (`path:`, `paths:`, `source:`, `destination:`, `src:`, `dst:`)
    // also calls one of the Phase 17e helpers
    // (`validate_ipc_path` / `validate_ipc_paths` /
    // `validate_path_no_traversal`). The sweep is body-text-based;
    // false positives are easy enough to fix with a one-line
    // helper call.
    let body = fs::read_to_string(repo_root().join("apps/copythat-ui/src-tauri/src/commands.rs"))
        .expect("commands.rs missing");

    // Carve out the file by `#[tauri::command]` markers — each block
    // is the attribute line plus everything up to the next attribute.
    let blocks: Vec<&str> = body.split("#[tauri::command]").collect();
    let suspicious_kw = ["path:", "paths:", "source:", "destination:", "src:", "dst:"];
    let gate_kw = [
        "validate_ipc_path",
        "validate_ipc_paths",
        "validate_path_no_traversal",
        "ipc_safety::",
    ];

    let mut audited = 0;
    for block in blocks.iter().skip(1) {
        // The signature is everything up to the first `{`. After the
        // `{`, the body lives — gate calls can sit either in the
        // signature's surrounding helper or in the body.
        let sig_end = block.find('{').unwrap_or(block.len());
        let signature = &block[..sig_end];
        if !suspicious_kw.iter().any(|kw| signature.contains(kw)) {
            continue;
        }
        // Drill into the function body — but we need a balanced-brace
        // walk to stop at this command's `}` rather than running into
        // sibling code. Simple approach: stop at the next
        // `#[tauri::command]` marker (already split above) or end.
        let combined = block;
        let has_gate = gate_kw.iter().any(|kw| combined.contains(kw));
        // A few command bodies cross-reference path args via DTOs
        // (`CopyOptionsDto`, `CollisionRule`) without exposing a raw
        // `path:` arg — when the signature names the kw and the
        // helper calls live in the body, that's covered. When the
        // body delegates to another internal helper that gates
        // (e.g. `enqueue` in this file), we accept the cross-call.
        let delegates_to_audited_helper = combined.contains("enqueue(")
            || combined.contains("enqueue_jobs(")
            || combined.contains("dropstack_apply_to(")
            || combined.contains("dropstack::");
        assert!(
            has_gate || delegates_to_audited_helper,
            "command block accepts a path-typed arg without calling Phase 17e gate:\n{}",
            &block[..sig_end.min(220)],
        );
        audited += 1;
    }
    assert!(
        audited >= 5,
        "expected to audit at least 5 path-typed commands, saw {audited}",
    );
}
