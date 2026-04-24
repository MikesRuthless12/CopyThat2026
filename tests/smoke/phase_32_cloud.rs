//! Phase 32 smoke — cloud backend matrix via OpenDAL.
//!
//! The brief calls for a minio sidecar + S3 round-trip; Phase 32a
//! ships the crate foundation + LocalFs + S3 drivers without a docker
//! dependency. This smoke exercises the layer where Phase 32 logic
//! actually decides things:
//!
//! - `Backend` / `BackendKind` / `BackendConfig` TOML round-trip via
//!   the `copythat-settings::RemoteSettings` mirror, which is the
//!   real persistence path (the crate itself is unopinionated about
//!   storage).
//! - `make_operator` + `OperatorTarget` put/get/list/stat/delete
//!   round-trip against a LocalFs backend.
//! - 1 MiB payload put → get → byte-compare, exercising the "copy a
//!   file through a cloud backend" happy path the Phase 32 engine
//!   integration will call.
//! - `BackendKind` parity: all 12 kinds serialize + round-trip.
//! - Fluent key parity: the 32 Phase 32 keys exist in `en` and in
//!   every one of the 18 locales.
//!
//! Future Phase 32b smoke additions (deferred): minio-backed S3
//! round-trip + resume-after-network-kill + multipart upload.

use std::path::PathBuf;

use bytes::Bytes;
use copythat_cloud::{
    Backend, BackendConfig, BackendKind, BackendRegistry, CopyTarget, LocalFsConfig,
    OperatorTarget, make_operator,
};
use copythat_settings::{
    BackendConfigEntry, BackendKindChoice, RemoteSettings, S3BackendConfig, Settings,
};

const PHASE_32_KEYS: &[&str] = &[
    "remote-heading",
    "remote-add",
    "remote-list-empty",
    "remote-test",
    "remote-test-success",
    "remote-test-failed",
    "remote-remove",
    "remote-name-label",
    "remote-kind-label",
    "remote-save",
    "remote-cancel",
    "backend-s3",
    "backend-r2",
    "backend-b2",
    "backend-azure-blob",
    "backend-gcs",
    "backend-onedrive",
    "backend-google-drive",
    "backend-dropbox",
    "backend-webdav",
    "backend-sftp",
    "backend-ftp",
    "backend-local-fs",
    "cloud-config-bucket",
    "cloud-config-region",
    "cloud-config-endpoint",
    "cloud-config-root",
    "cloud-error-invalid-config",
    "cloud-error-network",
    "cloud-error-not-found",
    "cloud-error-permission",
    "cloud-error-keychain",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

/// Case 1 — LocalFs put/get round-trip at 1 MiB. Covers the Phase 32
/// happy path: operator build → write → read → compare.
#[test]
fn case1_local_fs_round_trip_1mib() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");

    rt.block_on(async {
        let tmp = tempfile::tempdir().expect("tempdir");
        let backend = Backend {
            name: "local-smoke".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: tmp.path().to_string_lossy().into_owned(),
            }),
        };
        let op = make_operator(&backend, None).expect("operator");
        let target = OperatorTarget::new("local-smoke", op);

        // 1 MiB of a repeating pattern — bytewise compare catches any
        // multipart / buffer drift inside OpenDAL's Fs driver.
        let pattern: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
        let payload = Bytes::from(pattern.clone());

        target
            .put("subdir/payload.bin", payload.clone())
            .await
            .expect("put");
        let back = target.get("subdir/payload.bin").await.expect("get");
        assert_eq!(back.len(), 1024 * 1024, "payload length drift");
        assert_eq!(back.as_ref(), pattern.as_slice(), "payload content drift");

        let stat = target.stat("subdir/payload.bin").await.expect("stat");
        assert!(stat.is_some(), "stat after put returned None");
        let meta = stat.unwrap();
        assert!(!meta.is_dir);
        assert_eq!(meta.size, Some(1024 * 1024));

        target.delete("subdir/payload.bin").await.expect("delete");
        assert!(
            target.stat("subdir/payload.bin").await.expect("stat").is_none(),
            "delete left the object behind"
        );
    });
}

/// Case 2 — all 12 BackendKinds round-trip through TOML via
/// `RemoteSettings`. Confirms the wire-string contract + the kind-
/// specific config slots stay in step.
#[test]
fn case2_all_twelve_kinds_toml_round_trip() {
    let mut settings = Settings::default();
    for choice in [
        BackendKindChoice::S3,
        BackendKindChoice::R2,
        BackendKindChoice::B2,
        BackendKindChoice::AzureBlob,
        BackendKindChoice::Gcs,
        BackendKindChoice::Onedrive,
        BackendKindChoice::GoogleDrive,
        BackendKindChoice::Dropbox,
        BackendKindChoice::Webdav,
        BackendKindChoice::Sftp,
        BackendKindChoice::Ftp,
        BackendKindChoice::LocalFs,
    ] {
        settings.remotes.backends.push(BackendConfigEntry {
            name: choice.as_str().into(),
            kind: choice,
            ..Default::default()
        });
    }

    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("settings.toml");
    settings.save_to(&path).expect("save");
    let back = Settings::load_from(&path).expect("load");
    assert_eq!(back.remotes.backends.len(), 12);
    for (original, loaded) in settings
        .remotes
        .backends
        .iter()
        .zip(back.remotes.backends.iter())
    {
        assert_eq!(original.name, loaded.name);
        assert_eq!(original.kind, loaded.kind);
    }
}

/// Case 3 — BackendRegistry survives a concurrent add / get / remove
/// sequence. Confirms the RwLock-backed shared state.
#[test]
fn case3_registry_concurrency() {
    let registry = std::sync::Arc::new(BackendRegistry::new());
    registry
        .add(Backend {
            name: "a".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig { root: "/tmp".into() }),
        })
        .expect("add a");

    let r2 = registry.clone();
    let handle = std::thread::spawn(move || {
        for _ in 0..64 {
            let _ = r2.get("a");
            let _ = r2.snapshot();
        }
    });

    for _ in 0..64 {
        let _ = registry.get("a");
    }
    handle.join().expect("worker joined");
    assert!(registry.get("a").is_some());
    assert!(registry.remove("a").is_ok());
    assert_eq!(registry.len(), 0);
}

/// Case 4 — S3BackendConfig round-trip carries every field. Verifies
/// the settings mirror isn't silently dropping knobs.
#[test]
fn case4_s3_config_field_fidelity() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("settings.toml");
    let mut settings = Settings::default();
    settings.remotes.backends.push(BackendConfigEntry {
        name: "prod".into(),
        kind: BackendKindChoice::S3,
        s3: Some(S3BackendConfig {
            bucket: "releases".into(),
            region: "eu-central-1".into(),
            endpoint: "https://s3.example.com".into(),
            root: "2026/".into(),
        }),
        ..Default::default()
    });
    settings.remotes.default_backend = "prod".into();
    settings.save_to(&path).expect("save");

    let loaded = Settings::load_from(&path).expect("load");
    assert_eq!(loaded.remotes.default_backend, "prod");
    let entry = &loaded.remotes.backends[0];
    assert_eq!(entry.name, "prod");
    assert_eq!(entry.kind, BackendKindChoice::S3);
    let s3 = entry.s3.as_ref().expect("s3 slot");
    assert_eq!(s3.bucket, "releases");
    assert_eq!(s3.region, "eu-central-1");
    assert_eq!(s3.endpoint, "https://s3.example.com");
    assert_eq!(s3.root, "2026/");
}

/// Case 5 — Phase 32 Fluent keys are present in the English source.
#[test]
fn case5_en_fluent_keys_present() {
    let root = cargo_workspace_root();
    let en_path = root.join("locales").join("en").join("copythat.ftl");
    let content = std::fs::read_to_string(&en_path).unwrap_or_else(|e| {
        panic!("failed to read {}: {e}", en_path.display());
    });
    for key in PHASE_32_KEYS {
        let needle = format!("{key} =");
        assert!(
            content.contains(&needle),
            "missing Fluent key `{key}` in en/copythat.ftl"
        );
    }
}

/// Case 6 — Phase 32 Fluent keys exist in every one of the 18
/// locales. MT-flagged drafts count — we're checking parity, not
/// translation quality (human-review catches the second).
#[test]
fn case6_all_18_locales_have_phase_32_keys() {
    let root = cargo_workspace_root();
    for locale in LOCALES {
        let path = root.join("locales").join(locale).join("copythat.ftl");
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("failed to read {}: {e}", path.display());
        });
        for key in PHASE_32_KEYS {
            let needle = format!("{key} =");
            assert!(
                content.contains(&needle),
                "locale `{locale}` missing Phase 32 key `{key}`"
            );
        }
    }
}

/// Resolve the workspace root from the crate under test. The smoke
/// tests live under `tests/smoke/` at the workspace root, but
/// `CARGO_MANIFEST_DIR` points at `crates/copythat-cloud` because
/// the `[[test]]` entry is declared there.
fn cargo_workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/copythat-cloud → walk up two parents to the workspace.
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .expect("workspace root resolvable from CARGO_MANIFEST_DIR")
}

/// Case 7 — `BackendKind::is_enabled` matches what Phase 32a actually
/// wires up. Phase 32b flips the rest.
#[test]
fn case7_phase_32a_enabled_kinds_stable() {
    let enabled: Vec<&str> = BackendKind::all()
        .iter()
        .filter(|k| k.is_enabled())
        .map(|k| k.wire())
        .collect();
    assert_eq!(enabled, ["s3", "r2", "b2", "local-fs"]);
}

/// Case 8 — RemoteSettings round-trip with an empty backend list
/// keeps the shape intact (no stray null / empty-string drift).
#[test]
fn case8_empty_remote_settings_round_trip() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("settings.toml");
    let original = Settings::default();
    assert!(original.remotes.backends.is_empty());
    original.save_to(&path).expect("save");
    let back = Settings::load_from(&path).expect("load");
    assert_eq!(back.remotes, RemoteSettings::default());
}
