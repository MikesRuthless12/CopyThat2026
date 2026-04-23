//! Phase 12 smoke test — settings persistence + profile round-trip
//! + engine-effective buffer size.
//!
//! Per the Phase 12 guide prompt ("programmatically change each
//! setting, close and relaunch the app, assert settings persist and
//! engine picks them up (e.g., change buffer size to 4 MiB, run a
//! copy, assert the engine's effective buffer_size is 4 MiB via a
//! debug IPC command)") this test exercises three layers:
//!
//! 1. Direct file round-trip through `Settings::load_from` /
//!    `Settings::save_to` — proves a simulated "close and relaunch"
//!    preserves every nested field.
//! 2. Profile store round-trip: save, list, load, delete, export,
//!    import. Each step is one `ProfileStore` call; the on-disk
//!    JSON must parse back to a byte-identical `Settings`.
//! 3. Engine-effective buffer size: a scripted port of the Tauri
//!    `effective_buffer_size` IPC handler. Changing
//!    `transfer.buffer_size_bytes` to 4 MiB and re-reading via
//!    `TransferSettings::effective_buffer_size` returns
//!    `4 * 1024 * 1024`. Below-MIN or above-MAX values clamp to the
//!    engine's window, mirroring `copythat_core::options::{MIN,MAX}_BUFFER_SIZE`.
//!
//! The Tauri `AppState` IPC path itself isn't exercised here (that
//! would require booting a webview — too heavy for a CI smoke test);
//! the pure-filesystem + pure-Rust surface is what CI validates.
//! The `cargo test -p copythat-settings` battery covers the library
//! unit tests; this file is the full end-to-end ergonomic check.

use copythat_settings::{
    ConcurrencyChoice, ErrorDisplayMode, ErrorPolicyChoice, LogLevel, ProfileStore,
    ReflinkPreference, Settings, ShredMethodChoice, ThemePreference, VerifyChoice, defaults,
};
use tempfile::tempdir;

#[test]
fn phase_12_close_and_relaunch_persists_all_fields() {
    // Simulate "close + relaunch": write a full-custom Settings to
    // disk, drop the value, read it back. Every nested field must
    // survive — a single missing `#[serde(default)]` on any group
    // would fail this test.
    let d = tempdir().unwrap();
    let path = d.path().join("settings.toml");

    let before = Settings {
        general: copythat_settings::GeneralSettings {
            language: "fr".into(),
            theme: ThemePreference::Dark,
            start_with_os: true,
            single_instance: false,
            minimize_to_tray: true,
            error_display_mode: ErrorDisplayMode::Drawer,
            paste_shortcut_enabled: false,
            paste_shortcut: "Alt+Shift+V".into(),
            clipboard_watcher_enabled: true,
            auto_resume_interrupted: true,
        },
        transfer: copythat_settings::TransferSettings {
            buffer_size_bytes: 4 * 1024 * 1024,
            verify: VerifyChoice::Sha256,
            concurrency: ConcurrencyChoice::Manual(4),
            reflink: ReflinkPreference::Avoid,
            fsync_on_close: true,
            preserve_timestamps: true,
            preserve_permissions: false,
            preserve_acls: true,
            reserve_free_space_bytes: 0,
            on_locked: copythat_settings::LockedFilePolicyChoice::Snapshot,
            preserve_sparseness: true,
        },
        shell: copythat_settings::ShellSettings {
            context_menu_enabled: false,
            intercept_default_copy: true,
            notify_on_completion: false,
        },
        secure_delete: copythat_settings::SecureDeleteSettings {
            method: ShredMethodChoice::DoD7Pass,
            confirm_twice: false,
        },
        advanced: copythat_settings::AdvancedSettings {
            log_level: LogLevel::Debug,
            telemetry: false,
            error_policy: ErrorPolicyChoice::RetryN {
                max_attempts: 5,
                backoff_ms: 500,
            },
            history_retention_days: 90,
            database_path: Some(std::path::PathBuf::from("/custom/path/copythat.db")),
        },
        filters: copythat_settings::FilterSettings::default(),
        updater: copythat_settings::UpdaterSettings::default(),
        scan: copythat_settings::ScanSettings::default(),
        network: copythat_settings::NetworkSettings::default(),
        conflict_profiles: copythat_settings::ConflictProfileSettings::default(),
    };

    before.save_to(&path).expect("save_to");
    drop(before);

    let after = Settings::load_from(&path).expect("load_from");
    // Rebuild the expected value; the types derive `PartialEq` so
    // any drift in any nested group surfaces here.
    let expected = Settings {
        general: copythat_settings::GeneralSettings {
            language: "fr".into(),
            theme: ThemePreference::Dark,
            start_with_os: true,
            single_instance: false,
            minimize_to_tray: true,
            error_display_mode: ErrorDisplayMode::Drawer,
            paste_shortcut_enabled: false,
            paste_shortcut: "Alt+Shift+V".into(),
            clipboard_watcher_enabled: true,
            auto_resume_interrupted: true,
        },
        transfer: copythat_settings::TransferSettings {
            buffer_size_bytes: 4 * 1024 * 1024,
            verify: VerifyChoice::Sha256,
            concurrency: ConcurrencyChoice::Manual(4),
            reflink: ReflinkPreference::Avoid,
            fsync_on_close: true,
            preserve_timestamps: true,
            preserve_permissions: false,
            preserve_acls: true,
            reserve_free_space_bytes: 0,
            on_locked: copythat_settings::LockedFilePolicyChoice::Snapshot,
            preserve_sparseness: true,
        },
        shell: copythat_settings::ShellSettings {
            context_menu_enabled: false,
            intercept_default_copy: true,
            notify_on_completion: false,
        },
        secure_delete: copythat_settings::SecureDeleteSettings {
            method: ShredMethodChoice::DoD7Pass,
            confirm_twice: false,
        },
        advanced: copythat_settings::AdvancedSettings {
            log_level: LogLevel::Debug,
            telemetry: false,
            error_policy: ErrorPolicyChoice::RetryN {
                max_attempts: 5,
                backoff_ms: 500,
            },
            history_retention_days: 90,
            database_path: Some(std::path::PathBuf::from("/custom/path/copythat.db")),
        },
        filters: copythat_settings::FilterSettings::default(),
        updater: copythat_settings::UpdaterSettings::default(),
        scan: copythat_settings::ScanSettings::default(),
        network: copythat_settings::NetworkSettings::default(),
        conflict_profiles: copythat_settings::ConflictProfileSettings::default(),
    };
    assert_eq!(after, expected);
}

#[test]
fn phase_12_effective_buffer_size_matches_engine_window() {
    // Mirrors the `commands::effective_buffer_size` Tauri handler:
    // returns the clamped value the engine would actually use.

    // Requested 4 MiB (valid) → engine sees 4 MiB.
    let s = Settings {
        transfer: copythat_settings::TransferSettings {
            buffer_size_bytes: 4 * 1024 * 1024,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(s.transfer.effective_buffer_size(), 4 * 1024 * 1024);

    // Requested 1 byte (below MIN) → clamps up to MIN.
    let s = Settings {
        transfer: copythat_settings::TransferSettings {
            buffer_size_bytes: 1,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(
        s.transfer.effective_buffer_size(),
        defaults::MIN_BUFFER_SIZE
    );

    // Requested 1 GiB (above MAX) → clamps down to MAX.
    let s = Settings {
        transfer: copythat_settings::TransferSettings {
            buffer_size_bytes: 1024 * 1024 * 1024,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(
        s.transfer.effective_buffer_size(),
        defaults::MAX_BUFFER_SIZE
    );

    // Default construction matches the engine's 1 MiB default.
    let s = Settings::default();
    assert_eq!(
        s.transfer.effective_buffer_size(),
        defaults::DEFAULT_BUFFER_SIZE
    );
}

#[test]
fn phase_12_profile_round_trip() {
    let d = tempdir().unwrap();
    let store = ProfileStore::new(d.path().join("profiles"));

    // Start from defaults, make it recognisable.
    let mut archive = Settings::default();
    archive.general.language = "de".into();
    archive.transfer.verify = VerifyChoice::Blake3;
    archive.transfer.buffer_size_bytes = 8 * 1024 * 1024;
    archive.advanced.error_policy = ErrorPolicyChoice::Skip;

    // Save then list.
    store.save("archive-verify", &archive).expect("save");
    let listed = store.list().expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "archive-verify");

    // Load must yield the same Settings.
    let loaded = store.load("archive-verify").expect("load");
    assert_eq!(loaded, archive);

    // Export + re-import produces a byte-identical Settings.
    let dest = d.path().join("archive-verify.json");
    store.export("archive-verify", &dest).expect("export");
    assert!(dest.exists());
    store.import("archive-copy", &dest).expect("import");
    let imported = store.load("archive-copy").expect("load copy");
    assert_eq!(imported, archive);

    // Delete removes the file from disk.
    store.delete("archive-verify").expect("delete");
    let listed = store.list().expect("list after delete");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "archive-copy");
}

#[test]
fn phase_12_telemetry_flag_cannot_be_set_via_toml() {
    // Defence-in-depth: the Phase 12 spec says telemetry "OFF by
    // default — never on". The field is
    // `#[serde(skip_deserializing)]`, so even a malicious /
    // user-edited `settings.toml` carrying `telemetry = true`
    // cannot flip the live value to `true` on load.
    let d = tempdir().unwrap();
    let path = d.path().join("settings.toml");
    std::fs::write(
        &path,
        r#"
[advanced]
telemetry = true
log-level = "warn"
history-retention-days = 30
"#,
    )
    .unwrap();

    let s = Settings::load_from(&path).unwrap();
    assert!(
        !s.advanced.telemetry,
        "telemetry must stay OFF regardless of TOML input"
    );
    // Other fields in the same group did load, proving the guard is
    // field-local (not a blanket `skip_deserializing` on the whole
    // group).
    assert_eq!(s.advanced.log_level, LogLevel::Warn);
    assert_eq!(s.advanced.history_retention_days, 30);
}
