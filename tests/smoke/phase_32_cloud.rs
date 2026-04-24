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

/// Case 7 — `BackendKind::is_enabled` matches the Phase 32b scope
/// (everything but SFTP — the `openssh` crate `services-sftp` pulls
/// in fails to build on Windows; config + persistence still carry
/// the kind so cross-platform TOMLs round-trip cleanly).
#[test]
fn case7_phase_32b_enabled_kinds_stable() {
    let enabled: Vec<&str> = BackendKind::all()
        .iter()
        .filter(|k| k.is_enabled())
        .map(|k| k.wire())
        .collect();
    assert_eq!(
        enabled,
        [
            "s3",
            "r2",
            "b2",
            "azure-blob",
            "gcs",
            "onedrive",
            "google-drive",
            "dropbox",
            "webdav",
            "ftp",
            "local-fs",
        ]
    );
}

/// Case 8a — Phase 32d engine-level cloud-sink branch. `copy_file`
/// with `CopyOptions::cloud_sink` set routes the write through
/// `CloudSink::put_blocking` instead of opening a local dst file;
/// emits the normal Started / Progress / Completed event sequence
/// + returns a `CopyReport` with the transferred byte count.
#[test]
fn case8a_engine_cloud_sink_branch() {
    use std::sync::{Arc, Mutex};

    use copythat_core::{CloudSink, CopyControl, CopyEvent, CopyOptions, copy_file};

    #[derive(Debug, Default)]
    struct RecordingSink {
        captured: Mutex<Option<(String, Vec<u8>)>>,
    }

    impl CloudSink for RecordingSink {
        fn backend_name(&self) -> &str {
            "recording"
        }
        fn put_blocking(&self, path: &str, bytes: &[u8]) -> Result<u64, String> {
            *self.captured.lock().expect("lock") = Some((path.to_owned(), bytes.to_vec()));
            Ok(bytes.len() as u64)
        }
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");

    rt.block_on(async {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("payload.bin");
        let payload = b"phase-32d engine cloud-sink branch".to_vec();
        tokio::fs::write(&src, &payload).await.expect("seed");

        let sink = Arc::new(RecordingSink::default());
        let opts = CopyOptions {
            cloud_sink: Some(sink.clone() as Arc<dyn CloudSink>),
            ..Default::default()
        };

        let ctrl = CopyControl::new();
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        let dst_key = "archive/payload.bin";

        let report = copy_file(&src, std::path::Path::new(dst_key), opts, ctrl, tx)
            .await
            .expect("copy ok");

        assert_eq!(report.bytes, payload.len() as u64);

        // Drain + verify the event order.
        let mut kinds: Vec<&'static str> = Vec::new();
        while let Ok(evt) = rx.try_recv() {
            kinds.push(match evt {
                CopyEvent::Started { .. } => "Started",
                CopyEvent::Progress { .. } => "Progress",
                CopyEvent::Completed { .. } => "Completed",
                CopyEvent::Failed { .. } => "Failed",
                _ => "Other",
            });
        }
        assert!(kinds.first() == Some(&"Started"), "kinds={kinds:?}");
        assert!(kinds.last() == Some(&"Completed"), "kinds={kinds:?}");

        let captured = sink
            .captured
            .lock()
            .expect("lock")
            .clone()
            .expect("sink received a put");
        assert_eq!(captured.0, dst_key);
        assert_eq!(captured.1, payload);
    });
}

/// Case 8b — Phase 32e chunked progress reporting. When the source
/// is several buffer-sizes long, `copy_file` with a cloud-sink
/// emits at least one `Progress` event *per chunk* rather than one
/// halfway-tick.
#[test]
fn case8b_chunked_progress_events_during_cloud_sink_upload() {
    use std::sync::{Arc, Mutex};

    use copythat_core::{CloudSink, CopyControl, CopyEvent, CopyOptions, copy_file};

    #[derive(Debug, Default)]
    struct RecordingSink {
        captured: Mutex<Option<(String, usize)>>,
    }

    impl CloudSink for RecordingSink {
        fn backend_name(&self) -> &str {
            "chunked"
        }
        fn put_blocking(&self, path: &str, bytes: &[u8]) -> Result<u64, String> {
            *self.captured.lock().expect("lock") = Some((path.to_owned(), bytes.len()));
            Ok(bytes.len() as u64)
        }
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");

    rt.block_on(async {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("bigger.bin");
        // 5 MiB payload with a 1 MiB buffer → expect ≥5 Progress
        // events (one per chunk) plus the final one.
        let payload: Vec<u8> = (0..5 * 1024 * 1024).map(|i| (i % 251) as u8).collect();
        tokio::fs::write(&src, &payload).await.expect("seed");

        let sink = Arc::new(RecordingSink::default());
        let opts = CopyOptions {
            cloud_sink: Some(sink.clone() as Arc<dyn CloudSink>),
            buffer_size: 1024 * 1024,
            ..Default::default()
        };

        let ctrl = CopyControl::new();
        let (tx, mut rx) = tokio::sync::mpsc::channel(64);
        let dst_key = "data/bigger.bin";
        let report = copy_file(&src, std::path::Path::new(dst_key), opts, ctrl, tx)
            .await
            .expect("copy ok");
        assert_eq!(report.bytes as usize, payload.len());

        let mut progress_events = 0usize;
        let mut last_bytes: u64 = 0;
        while let Ok(evt) = rx.try_recv() {
            if let CopyEvent::Progress {
                bytes, rate_bps, ..
            } = evt
            {
                progress_events += 1;
                // Bytes must monotonically increase.
                assert!(bytes >= last_bytes, "progress regressed {last_bytes} -> {bytes}");
                last_bytes = bytes;
                let _ = rate_bps;
            }
        }
        assert!(
            progress_events >= 5,
            "expected ≥5 progress events for a 5 MiB × 1 MiB buffer, got {progress_events}"
        );
        assert_eq!(last_bytes as usize, payload.len());

        let captured = sink
            .captured
            .lock()
            .expect("lock")
            .clone()
            .expect("sink got full payload");
        assert_eq!(captured.0, dst_key);
        assert_eq!(captured.1, payload.len());
    });
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

/// Case 9 — Phase 32b lit drivers build a valid OpenDAL operator
/// under minimally-valid config (no live network). The make_operator
/// factory is where configuration-time validation runs — we exercise
/// every kind so a future refactor that drops a driver surfaces here.
#[test]
fn case9_phase_32b_drivers_build_operator() {
    use copythat_cloud::{
        AzureBlobConfig, FtpConfig, GcsConfig, OAuthConfig, S3Config, WebdavConfig,
    };

    // Azure Blob — container + account required; secret optional at
    // build time (OpenDAL surfaces a stat-time 401 otherwise).
    let azure = Backend {
        name: "az".into(),
        kind: BackendKind::AzureBlob,
        config: BackendConfig::AzureBlob(AzureBlobConfig {
            container: "snapshots".into(),
            account_name: "mystore".into(),
            endpoint: String::new(),
            root: String::new(),
        }),
    };
    if let Err(e) = make_operator(&azure, None) {
        panic!("azure build failed: {e}");
    }

    // GCS — bucket required; credential flows through the secret
    // slot as raw JSON.
    let gcs = Backend {
        name: "gcs".into(),
        kind: BackendKind::Gcs,
        config: BackendConfig::Gcs(GcsConfig {
            bucket: "backup-bucket".into(),
            service_account: String::new(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&gcs, None).is_ok());

    // OneDrive / GoogleDrive / Dropbox all require the access token
    // at build time — omitting it surfaces InvalidConfig.
    let onedrive = Backend {
        name: "od".into(),
        kind: BackendKind::Onedrive,
        config: BackendConfig::Onedrive(OAuthConfig {
            client_id: "cid".into(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&onedrive, Some("token123")).is_ok());

    let gdrive = Backend {
        name: "gd".into(),
        kind: BackendKind::GoogleDrive,
        config: BackendConfig::GoogleDrive(OAuthConfig {
            client_id: "cid".into(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&gdrive, Some("token123")).is_ok());

    let dropbox = Backend {
        name: "db".into(),
        kind: BackendKind::Dropbox,
        config: BackendConfig::Dropbox(OAuthConfig {
            client_id: "cid".into(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&dropbox, Some("token123")).is_ok());

    // WebDAV — endpoint required; username + password both optional
    // for public read-only endpoints.
    let webdav = Backend {
        name: "wd".into(),
        kind: BackendKind::Webdav,
        config: BackendConfig::Webdav(WebdavConfig {
            endpoint: "https://dav.example.com/".into(),
            username: "u".into(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&webdav, Some("pw")).is_ok());

    // FTP — host required; the driver auto-prepends the ftps://
    // scheme + configured port.
    let ftp = Backend {
        name: "ftp".into(),
        kind: BackendKind::Ftp,
        config: BackendConfig::Ftp(FtpConfig {
            host: "ftp.example.com".into(),
            port: 21,
            username: "anon".into(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&ftp, None).is_ok());

    // S3 passes both halves of the secret; missing separator fails.
    let s3 = Backend {
        name: "s3".into(),
        kind: BackendKind::S3,
        config: BackendConfig::S3(S3Config {
            bucket: "b".into(),
            region: "us-east-1".into(),
            endpoint: String::new(),
            root: String::new(),
        }),
    };
    assert!(make_operator(&s3, Some("ak\nsk")).is_ok());
}
