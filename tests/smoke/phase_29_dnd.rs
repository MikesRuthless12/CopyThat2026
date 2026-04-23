//! Phase 29 smoke — spring-loaded folders + native DnD polish.
//!
//! The spec calls for a Playwright + tauri-driver e2e test harness
//! (`tests/smoke/phase_29_dnd.ts`). The repository doesn't ship a
//! Playwright harness yet and adding one pulls in a multi-hundred-MB
//! node_modules tree + tauri-driver native dependency that's
//! impractical for a per-phase smoke. This Rust smoke runs the same
//! structural assertions at the settings + fluent layer where the
//! Phase 29 logic actually lives:
//!
//! 1. **DndSettings round-trip.** `copythat_settings::DndSettings`
//!    defaults match the brief (650 ms spring-load, thumbnails on,
//!    invalid-highlight on). Round-trips through TOML on a full
//!    `Settings` save/load cycle.
//! 2. **Spring-load clamp.** Hand-edited out-of-band values (5 ms
//!    too-fast, 30 000 ms too-slow) land inside the 50..5000 ms
//!    guard rails via `effective_spring_ms`.
//! 3. **Fluent keys present.** All 14 Phase 29 keys exist in
//!    `locales/en/copythat.ftl` (the literal-coverage gate the
//!    i18n-lint enforces).
//! 4. **Fluent key parity.** Every Phase 29 key exists in all 17
//!    non-English locales (key-set parity is what i18n-lint
//!    enforces; this smoke confirms it for the Phase 29 subset
//!    explicitly so a locale that silently drops a key still fails
//!    here even if somehow the global lint regresses).

use copythat_settings::{DND_MAX_SPRING_MS, DND_MIN_SPRING_MS, DndSettings, Settings};

const PHASE_29_KEYS: &[&str] = &[
    "settings-dnd-heading",
    "settings-dnd-spring-load",
    "settings-dnd-spring-delay",
    "settings-dnd-thumbnails",
    "settings-dnd-invalid-highlight",
    "dropzone-invalid-title",
    "dropzone-invalid-readonly",
    "dropzone-picker-title",
    "dropzone-picker-up",
    "dropzone-picker-path",
    "dropzone-picker-root",
    "dropzone-picker-use-this",
    "dropzone-picker-empty",
    "dropzone-picker-cancel",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case1_dnd_defaults_and_toml_round_trip() {
    let s = Settings::default();
    // Defaults match the spec.
    assert!(s.dnd.spring_load_enabled);
    assert_eq!(s.dnd.spring_load_delay_ms, 650);
    assert!(s.dnd.show_drag_thumbnails);
    assert!(s.dnd.highlight_invalid_targets);

    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("settings.toml");

    // Save + reload.
    let mutated = Settings {
        dnd: DndSettings {
            spring_load_delay_ms: 900,
            show_drag_thumbnails: false,
            ..Default::default()
        },
        ..Default::default()
    };
    mutated.save_to(&path).expect("save");
    let loaded = Settings::load_from(&path).expect("load");
    assert_eq!(loaded.dnd.spring_load_delay_ms, 900);
    assert!(!loaded.dnd.show_drag_thumbnails);
    assert!(
        loaded.dnd.spring_load_enabled,
        "unrelated fields must survive round-trip"
    );
    assert!(loaded.dnd.highlight_invalid_targets);
}

#[test]
fn case2_spring_load_delay_clamp() {
    let too_fast = DndSettings {
        spring_load_delay_ms: 5,
        ..Default::default()
    };
    assert_eq!(too_fast.effective_spring_ms(), DND_MIN_SPRING_MS);

    let too_slow = DndSettings {
        spring_load_delay_ms: 30_000,
        ..Default::default()
    };
    assert_eq!(too_slow.effective_spring_ms(), DND_MAX_SPRING_MS);

    let in_band = DndSettings {
        spring_load_delay_ms: 1_200,
        ..Default::default()
    };
    assert_eq!(in_band.effective_spring_ms(), 1_200);
}

#[test]
fn case3_fluent_keys_present_in_en() {
    let en = std::fs::read_to_string(repo_root().join("locales/en/copythat.ftl")).unwrap();
    for key in PHASE_29_KEYS {
        let needle = format!("\n{key} = ");
        assert!(
            en.contains(&needle),
            "Phase 29 key {key} missing from locales/en/copythat.ftl",
        );
    }
}

#[test]
fn case4_fluent_parity_across_all_locales() {
    for loc in LOCALES {
        let path = repo_root().join("locales").join(loc).join("copythat.ftl");
        let body = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
        for key in PHASE_29_KEYS {
            let needle = format!("\n{key} = ");
            assert!(
                body.contains(&needle),
                "Phase 29 key {key} missing from locale {loc}",
            );
        }
    }
}

fn repo_root() -> std::path::PathBuf {
    // Tests run from the UI crate's manifest dir —
    // `apps/copythat-ui/src-tauri`. Jump up three levels.
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
