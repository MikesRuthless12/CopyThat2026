//! Phase 7b smoke test — cross-platform layer.
//!
//! Verifies the host-independent bits of the Windows COM shell
//! extension: argv composition for the detached `copythat` spawn,
//! registry-key layout for per-user and system-wide install, and
//! the opt-in copy-verb interceptor. Runs on every CI runner
//! (Linux, macOS, Windows) because these helpers are pure Rust.
//!
//! The live COM / `regsvr32` round-trip lives in
//! `tests/smoke/phase_07b_shellext.ps1` — that script is the
//! Windows-only "plug in the cable and check the light" test and
//! is invoked from the host's PowerShell directly, not via cargo.
//!
//! Together the two scripts cover Phase 7b's smoke-test obligation:
//! the COM DLL's registration surface is shaped correctly (here),
//! and the compiled DLL exposes the four COM entry points
//! `regsvr32` + the COM runtime need (there).

use copythat_shellext::consts::{
    CLSID_COPY_STR, CLSID_MOVE_STR, DISPLAY_COPY, DISPLAY_MOVE, HOST_BIN, VERB_COPY, VERB_MOVE,
};
use copythat_shellext::registry::{
    InstallScope, SHELL_TARGETS, all_registration_keys, class_registration_keys,
    copy_interceptor_keys, verb_registration_keys,
};
use copythat_shellext::spawn::{Verb, build_argv};
use std::ffi::OsString;

#[test]
fn argv_layout_matches_phase_7a_parser_contract() {
    // Shell extensions must emit exactly the shape the Phase 7a
    // CLI parser accepts: `copythat --enqueue <verb> -- <paths...>`.
    let paths = vec![
        OsString::from(r"C:\pictures\a.jpg"),
        OsString::from(r"C:\pictures\b.jpg"),
    ];
    let argv = build_argv(Verb::Copy, &paths);
    assert_eq!(argv[0], OsString::from(HOST_BIN));
    assert_eq!(argv[1], OsString::from("--enqueue"));
    assert_eq!(argv[2], OsString::from("copy"));
    assert_eq!(argv[3], OsString::from("--"));
    assert_eq!(argv[4], paths[0]);
    assert_eq!(argv[5], paths[1]);
}

#[test]
fn argv_move_verb_swaps_cleanly() {
    let paths = vec![OsString::from(r"C:\a")];
    let argv = build_argv(Verb::Move, &paths);
    assert_eq!(argv[2], OsString::from("move"));
}

#[test]
fn argv_preserves_weird_names_after_double_dash() {
    let paths = vec![
        OsString::from(r"--help.txt"),
        OsString::from(r"C:\--odd"),
        OsString::from(r"C:\legit\a.bin"),
    ];
    let argv = build_argv(Verb::Copy, &paths);
    let dash_idx = argv
        .iter()
        .position(|x| x == &OsString::from("--"))
        .unwrap();
    assert_eq!(dash_idx, 3);
    // Every provided path lives strictly after the `--`.
    for p in &paths {
        let idx = argv.iter().position(|x| x == p).unwrap();
        assert!(idx > dash_idx, "path {p:?} must appear after --");
    }
}

#[test]
fn per_user_registration_layout_has_expected_keys() {
    let dll = r"C:\Program Files\CopyThat2026\copythat_shellext.dll";
    let keys = all_registration_keys(InstallScope::PerUser, dll);

    // 2 classes × 3 tuples (default + InprocServer32 default + ThreadingModel)
    // + 2 verbs × 2 targets × 3 tuples (default + MUIVerb + DelegateExecute)
    // = 6 + 12 = 18
    assert_eq!(keys.len(), 18);

    // Every path must sit under HKCU (per-user scope).
    for (path, _, _) in &keys {
        assert!(
            path.starts_with("HKCU\\"),
            "per-user keys must live under HKCU: {path}"
        );
    }

    // The DLL file path must appear verbatim in the InprocServer32
    // defaults for both classes. If this assertion ever fires,
    // Explorer will load the wrong binary or fail to find us.
    let inproc_hits = keys
        .iter()
        .filter(|(path, name, value)| {
            path.ends_with(r"\InprocServer32") && name.is_empty() && value == dll
        })
        .count();
    assert_eq!(inproc_hits, 2, "expected one InprocServer32 per CLSID");

    // Both display strings must show up.
    assert!(keys.iter().any(|(_, _, v)| v == DISPLAY_COPY));
    assert!(keys.iter().any(|(_, _, v)| v == DISPLAY_MOVE));
}

#[test]
fn system_wide_registration_routes_through_hklm() {
    let keys = all_registration_keys(InstallScope::LocalMachine, r"C:\does-not-matter.dll");
    for (path, _, _) in &keys {
        assert!(
            path.starts_with("HKLM\\"),
            "system-wide keys must live under HKLM: {path}"
        );
    }
}

#[test]
fn verb_registration_hits_every_shell_target() {
    let keys = verb_registration_keys(
        InstallScope::PerUser,
        VERB_COPY,
        CLSID_COPY_STR,
        DISPLAY_COPY,
    );
    // Every SHELL_TARGETS entry must appear in at least one key path.
    for target in SHELL_TARGETS {
        let needle = format!(r"\{target}\shell\{VERB_COPY}");
        assert!(
            keys.iter().any(|(path, _, _)| path.contains(&needle)),
            "missing shell target: {target}"
        );
    }
    // Every verb key must declare a DelegateExecute pointing at the CLSID.
    let delegate_count = keys
        .iter()
        .filter(|(_, name, value)| name == "DelegateExecute" && value == CLSID_COPY_STR)
        .count();
    assert_eq!(delegate_count, SHELL_TARGETS.len());
}

#[test]
fn copy_interceptor_is_opt_in_hkcu_only() {
    let keys = copy_interceptor_keys(CLSID_COPY_STR);
    assert_eq!(keys.len(), 1, "interceptor touches exactly one key");
    assert!(
        keys[0].0.starts_with(r"HKCU\Software\Classes\*\shell\copy"),
        "interceptor must be per-user: {}",
        keys[0].0
    );
    assert_eq!(keys[0].1, "DelegateExecute");
    assert_eq!(keys[0].2, CLSID_COPY_STR);
}

#[test]
fn class_registration_separates_threading_model_from_default() {
    let keys = class_registration_keys(
        InstallScope::PerUser,
        CLSID_MOVE_STR,
        "Copy That 2026 — Move command",
        r"C:\x.dll",
    );
    // Three tuples: class-key default + InprocServer32 default + InprocServer32 ThreadingModel.
    assert_eq!(keys.len(), 3);

    // The class-key default carries a friendly name; the ThreadingModel
    // tuple must carry exactly "Apartment" (shell extensions load on
    // Explorer's STA).
    let threading_count = keys
        .iter()
        .filter(|(_, name, value)| name == "ThreadingModel" && value == "Apartment")
        .count();
    assert_eq!(threading_count, 1);
}

#[test]
fn clsid_strings_are_curly_brace_formatted() {
    // Windows expects CLSIDs in `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`
    // format under the registry. A regression that drops the braces
    // (or swaps them for parens) breaks `regsvr32` silently.
    for s in [CLSID_COPY_STR, CLSID_MOVE_STR] {
        assert!(s.starts_with('{'));
        assert!(s.ends_with('}'));
        assert_eq!(s.len(), 38); // "{........-....-....-....-............}"
    }
}

#[test]
fn verbs_do_not_collide_with_explorer_builtins() {
    // Our canonical verb names have the `CopyThat.` prefix so they
    // cannot shadow Explorer's own `copy`, `cut`, `paste`, `rename`
    // verbs by accident. (The opt-in interceptor uses the literal
    // `copy` key, but that's declared explicitly in
    // `copy_interceptor_keys` — and gated behind a user toggle.)
    for v in [VERB_COPY, VERB_MOVE] {
        assert!(v.starts_with("CopyThat."), "verb {v} lacks project prefix");
    }
}
