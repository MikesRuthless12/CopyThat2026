//! Phase 30 smoke — cross-platform path translation.
//!
//! Asserts the behaviour the Phase 30 brief calls out, at the
//! value-type boundary where `translate_path` lives:
//!
//! 1. **NFD → NFC for Windows targets.** A filename stored with
//!    combining marks (`re\u{0301}sume\u{0301}.pdf`) translates to the
//!    composed form (`résumé.pdf`) when the destination OS is Windows.
//! 2. **Reserved-name suffix.** `CON.txt` becomes `CON_.txt` under
//!    [`ReservedNameStrategy::Suffix`].
//! 3. **Windows long-path prefix.** A composed destination that
//!    exceeds 260 UTF-16 units gets the `\\?\` namespace prefix under
//!    [`LongPathStrategy::Win32LongPath`]. On Windows the test also
//!    performs an actual `std::fs::copy` against the translated path
//!    so we catch filesystem-level regressions (e.g. a future change
//!    to `apply_long_path_prefix` that emits a malformed prefix).
//! 4. **CRLF/LF content rewrite.** A buffer with mixed line endings
//!    translates to pure LF under
//!    [`translate_content_line_endings`].
//!
//! Two structural smokes round out the suite: the settings allowlist
//! matches the engine's default extension list (single source of
//! truth), and the 12 Phase 30 Fluent keys exist in all 18 locales.

use std::fs;
use std::path::PathBuf;

use copythat_core::{
    LineEndingMode, LongPathStrategy, NormalizationMode, PathPolicy, ReservedNameStrategy,
    TargetOs, TranslateError, default_text_extensions, translate_content_line_endings,
    translate_path,
};
use copythat_settings::{PathTranslationSettings, default_text_extensions_for_settings};

const PHASE_30_KEYS: &[&str] = &[
    "translate-heading",
    "translate-unicode-label",
    "translate-unicode-auto",
    "translate-unicode-windows",
    "translate-unicode-macos",
    "translate-line-endings-label",
    "translate-line-endings-allowlist",
    "reserved-name-label",
    "reserved-name-suffix",
    "reserved-name-reject",
    "long-path-label",
    "long-path-hint",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case1_nfd_normalized_to_nfc_for_windows() {
    let src: PathBuf = "/tmp/re\u{0301}sume\u{0301}.pdf".into();
    let dst_root = if cfg!(windows) {
        PathBuf::from(r"C:\out")
    } else {
        PathBuf::from("/out")
    };
    let policy = PathPolicy {
        target_os: TargetOs::Windows,
        unicode_normalization: NormalizationMode::Auto,
        ..PathPolicy::default()
    };
    let out = translate_path(&src, &dst_root, &policy).expect("translate");
    let final_name = out.file_name().unwrap().to_string_lossy().into_owned();
    assert_eq!(
        final_name, "résumé.pdf",
        "NFD composed form expected; got {final_name}"
    );

    // The source name and destination name should differ — proves the
    // translator actually moved bytes, not just passed through.
    let src_name = src.file_name().unwrap().to_string_lossy().into_owned();
    assert_ne!(
        src_name.as_bytes(),
        final_name.as_bytes(),
        "expected byte-level difference between NFD source and NFC dst"
    );
}

#[test]
fn case2_reserved_name_gets_underscore_suffix() {
    let src = PathBuf::from("/src/CON.txt");
    let dst_root = if cfg!(windows) {
        PathBuf::from(r"C:\out")
    } else {
        PathBuf::from("/out")
    };
    let policy = PathPolicy {
        target_os: TargetOs::Windows,
        reserved_name_strategy: ReservedNameStrategy::Suffix,
        ..PathPolicy::default()
    };
    let out = translate_path(&src, &dst_root, &policy).expect("translate");
    assert_eq!(
        out.file_name().unwrap().to_string_lossy(),
        "CON_.txt",
        "expected CON.txt → CON_.txt rewrite"
    );

    // Reject variant should surface the typed error.
    let reject_policy = PathPolicy {
        target_os: TargetOs::Windows,
        reserved_name_strategy: ReservedNameStrategy::Reject,
        ..PathPolicy::default()
    };
    let err = translate_path(&src, &dst_root, &reject_policy).unwrap_err();
    assert!(
        matches!(err, TranslateError::ReservedName { .. }),
        "expected ReservedName error, got {err:?}"
    );

    // Linux target: CON.txt is just a filename; no rewrite, no error.
    let linux_policy = PathPolicy {
        target_os: TargetOs::Linux,
        reserved_name_strategy: ReservedNameStrategy::Reject,
        ..PathPolicy::default()
    };
    let out = translate_path(&src, &PathBuf::from("/out"), &linux_policy).expect("translate");
    assert_eq!(
        out.file_name().unwrap().to_string_lossy(),
        "CON.txt",
        "Linux target must not rewrite the reserved Windows name"
    );
}

#[test]
fn case3_long_path_prefix_emitted_and_usable() {
    // Compose a > 260 UTF-16-unit destination path for a Windows
    // target. Use a 250-char stem so just `C:\out\<stem>.bin` blows
    // past MAX_PATH.
    let long_stem = "x".repeat(250);
    let src = PathBuf::from(format!("/src/{long_stem}.bin"));
    let dst_root = if cfg!(windows) {
        PathBuf::from(r"C:\out")
    } else {
        // Use a fake Windows absolute root so translate_path's
        // long-path gate fires on non-Windows hosts too. The copy
        // itself is only attempted on Windows (see below).
        PathBuf::from(r"C:\out")
    };
    let policy = PathPolicy {
        target_os: TargetOs::Windows,
        long_path_strategy: LongPathStrategy::Win32LongPath,
        ..PathPolicy::default()
    };
    let out = translate_path(&src, &dst_root, &policy).expect("translate");
    let s = out.to_string_lossy();
    assert!(
        s.starts_with(r"\\?\"),
        "expected \\\\?\\ prefix on long path, got {s}"
    );
    assert!(
        s.contains(&long_stem),
        "expected the original long stem to survive in the output"
    );

    // Windows-only: actually perform the copy to prove the prefix is
    // shape-correct (e.g. not `\\?\\C:\…` with a stray double-
    // backslash). On non-Windows we stop at the string-shape check
    // above — the behaviour is portable enough to assert there.
    #[cfg(windows)]
    {
        let tmp = tempfile::tempdir().expect("tempdir");
        // Build a destination whose composed length exceeds MAX_PATH
        // (260) so the `\\?\` namespace is actually required for the
        // kernel to accept the open call.
        let real_root = tmp.path().to_path_buf();
        let needed_stem_len = 260usize
            .saturating_sub(real_root.to_string_lossy().chars().count() + "/test.bin".len())
            .max(200);
        let stem = "y".repeat(needed_stem_len + 30);
        let real_src_file = tmp.path().join("source.bin");
        fs::write(&real_src_file, b"phase-30 long-path payload").expect("seed");

        let wanted_dst = real_root.join(format!("{stem}.bin"));
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            long_path_strategy: LongPathStrategy::Win32LongPath,
            ..PathPolicy::default()
        };
        let prefixed = translate_path(&wanted_dst, &real_root, &policy).expect("translate real");
        assert!(prefixed.to_string_lossy().starts_with(r"\\?\"));

        // The copy itself.
        fs::copy(&real_src_file, &prefixed).expect("fs::copy through long-path prefix");
        let round = fs::read(&prefixed).expect("read back");
        assert_eq!(round, b"phase-30 long-path payload");
    }
}

#[test]
fn case4_mixed_line_endings_translate_to_lf() {
    let mixed = b"line1\r\nline2\nline3\r\nline4\nline5";
    let out = translate_content_line_endings(mixed, LineEndingMode::Lf);
    assert_eq!(out, b"line1\nline2\nline3\nline4\nline5");

    // Round-trip back to CRLF uniformly.
    let back = translate_content_line_endings(&out, LineEndingMode::Crlf);
    assert_eq!(
        back, b"line1\r\nline2\r\nline3\r\nline4\r\nline5",
        "CRLF rewrite should expand every LF, including those that were \
         originally CRLF"
    );

    // `AsIs` is byte-identity.
    assert_eq!(
        translate_content_line_endings(mixed, LineEndingMode::AsIs),
        mixed
    );
}

#[test]
fn case5_settings_allowlist_matches_engine_default() {
    // The settings crate can't depend on copythat-core, so it
    // copies the default extension allowlist inline. This smoke is
    // the parity check — if either list drifts, the copy-path
    // behaviour (rewrite vs byte-copy) starts disagreeing with the
    // UI's toggle state.
    let engine = default_text_extensions();
    let settings = default_text_extensions_for_settings();
    assert_eq!(
        engine, settings,
        "PathTranslationSettings allowlist must match copythat_core \
         default_text_extensions() — update both when adding extensions"
    );
}

#[test]
fn case6_path_translation_settings_round_trip() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("settings.toml");

    let s = copythat_settings::Settings {
        path_translation: PathTranslationSettings {
            enabled: true,
            unicode_normalization: copythat_settings::NormalizationModeChoice::Nfc,
            line_endings: copythat_settings::LineEndingModeChoice::Lf,
            reserved_name_strategy: copythat_settings::ReservedNameChoice::Reject,
            long_path_strategy: copythat_settings::LongPathChoice::Truncate,
            line_ending_allowlist: vec!["txt".into(), "md".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    s.save_to(&path).expect("save");
    let loaded = copythat_settings::Settings::load_from(&path).expect("load");
    assert_eq!(
        loaded.path_translation.unicode_normalization,
        copythat_settings::NormalizationModeChoice::Nfc
    );
    assert_eq!(
        loaded.path_translation.line_endings,
        copythat_settings::LineEndingModeChoice::Lf
    );
    assert_eq!(
        loaded.path_translation.reserved_name_strategy,
        copythat_settings::ReservedNameChoice::Reject
    );
    assert_eq!(
        loaded.path_translation.long_path_strategy,
        copythat_settings::LongPathChoice::Truncate
    );
    assert_eq!(
        loaded.path_translation.line_ending_allowlist,
        vec!["txt".to_string(), "md".to_string()]
    );
}

#[test]
fn case7_fluent_keys_present_in_en() {
    let en_path = repo_root().join("locales/en/copythat.ftl");
    let en =
        fs::read_to_string(&en_path).unwrap_or_else(|e| panic!("read {}: {e}", en_path.display()));
    for key in PHASE_30_KEYS {
        let needle = format!("\n{key} = ");
        assert!(
            en.contains(&needle),
            "Phase 30 key `{key}` missing from locales/en/copythat.ftl"
        );
    }
}

#[test]
fn case8_fluent_parity_across_all_locales() {
    for loc in LOCALES {
        let path = repo_root().join("locales").join(loc).join("copythat.ftl");
        let body =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_30_KEYS {
            let needle = format!("\n{key} = ");
            assert!(
                body.contains(&needle),
                "Phase 30 key `{key}` missing from locale {loc}"
            );
        }
    }
}

fn repo_root() -> std::path::PathBuf {
    // Tests registered on `copythat-core` run from
    // `crates/copythat-core`. Walk up until we find `locales/en/…`.
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &std::path::Path = manifest.as_path();
    for _ in 0..5 {
        if cur.join("locales").join("en").join("copythat.ftl").exists() {
            return cur.to_path_buf();
        }
        cur = cur.parent().unwrap_or(cur);
    }
    panic!("could not locate repo root from {manifest:?}");
}
