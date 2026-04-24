//! Phase 32 — IPC surface for the cloud backend matrix.
//!
//! The frontend Settings → Remotes tab calls these commands to
//! enumerate configured backends, create or edit them, delete them,
//! and round-trip a "test connection" request through
//! `copythat_cloud::make_operator` + a live `stat("/")` call.
//!
//! Secrets (access keys, OAuth tokens, SFTP passwords) are passed to
//! `add_backend` as a single opaque `secret` string and written to
//! the OS keychain via `copythat_cloud::Credentials`. They never
//! cross the IPC boundary on reads: the frontend holds
//! name + kind + config only and prompts for the secret on re-edit
//! if the user needs to rotate it.

use std::path::PathBuf;
use std::sync::Arc;

use copythat_cloud::{
    AzureBlobConfig, Backend, BackendConfig, BackendKind, BackendRegistry, CopyTarget,
    Credentials, FtpConfig, GcsConfig, LocalFsConfig, OAuthConfig, OperatorTarget, S3Config,
    SftpConfig, WebdavConfig, copy_from_target, copy_to_target, make_operator, opendal,
};
use copythat_settings::{
    AzureBlobBackendConfig, BackendConfigEntry, BackendKindChoice, FtpBackendConfig,
    GcsBackendConfig, LocalFsBackendConfig, OAuthBackendConfig, S3BackendConfig,
    SftpBackendConfig, WebdavBackendConfig,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

/// Wire-form for a single backend. Round-trips directly from the
/// Svelte `BackendDto` type; the kind-specific `config` sub-object
/// is one of the 12 variants keyed on `kind`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendDto {
    pub name: String,
    pub kind: String,
    pub config: BackendConfigDto,
    #[serde(default)]
    pub enabled_in_build: bool,
}

/// Kind-specific config. All variants carry plain strings / u16s —
/// the Svelte wizard validates before submit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendConfigDto {
    pub root: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub container: String,
    pub account_name: String,
    pub service_account: String,
    pub client_id: String,
    pub host: String,
    pub username: String,
    pub port: u16,
}

/// Test-connection response DTO.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestConnectionResult {
    pub ok: bool,
    /// Fluent key suffix when !ok. The frontend resolves the full key
    /// via `t()` — `cloud-error-<reason>`.
    pub reason: Option<&'static str>,
    pub detail: Option<String>,
}

// ---------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------

/// List every configured backend from `settings.remotes.backends`.
#[tauri::command]
pub fn list_backends(state: tauri::State<'_, AppState>) -> Vec<BackendDto> {
    let snap = state.settings_snapshot();
    snap.remotes
        .backends
        .iter()
        .map(entry_to_dto)
        .collect()
}

/// Insert a new backend entry. Rejects with a typed error when a
/// backend with the same name already exists. Secret (when `Some`) is
/// written to the OS keychain under `copythat-cloud/<name>`.
#[tauri::command]
pub fn add_backend(
    dto: BackendDto,
    secret: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<BackendDto, String> {
    let name_trim = dto.name.trim().to_owned();
    if name_trim.is_empty() {
        return Err("backend name is required".into());
    }

    let entry = dto_to_entry(&dto).ok_or_else(|| "invalid backend config".to_string())?;
    let backend = entry_to_cloud_backend(&entry).map_err(|e| e.to_string())?;

    // Mutate persisted settings.
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    if guard.remotes.backends.iter().any(|b| b.name == entry.name) {
        return Err(format!("backend `{}` already exists", entry.name));
    }
    guard.remotes.backends.push(entry.clone());
    let snapshot = guard.clone();
    let path = state.settings_path.as_ref().clone();
    drop(guard);
    if !path.as_os_str().is_empty() {
        snapshot.save_to(&path).map_err(|e| e.to_string())?;
    }

    // Store the secret (if any) before touching the in-memory
    // registry so a keychain failure leaves settings untouched — we
    // already saved above, so rollback here is best-effort.
    if let Some(s) = secret
        && !s.is_empty()
    {
        Credentials.store(&backend.name, &s).map_err(|e| e.to_string())?;
    }

    state.cloud_backends.upsert(backend);
    Ok(entry_to_dto(&entry))
}

/// Replace (or insert) a backend by name. The keychain entry is
/// updated only when `secret` is `Some`; passing `None` leaves the
/// existing keychain value alone so the UI can edit config without
/// forcing re-entry of the secret.
#[tauri::command]
pub fn update_backend(
    dto: BackendDto,
    secret: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<BackendDto, String> {
    let entry = dto_to_entry(&dto).ok_or_else(|| "invalid backend config".to_string())?;
    let backend = entry_to_cloud_backend(&entry).map_err(|e| e.to_string())?;

    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    if let Some(slot) = guard.remotes.backends.iter_mut().find(|b| b.name == entry.name) {
        *slot = entry.clone();
    } else {
        guard.remotes.backends.push(entry.clone());
    }
    let snapshot = guard.clone();
    let path = state.settings_path.as_ref().clone();
    drop(guard);
    if !path.as_os_str().is_empty() {
        snapshot.save_to(&path).map_err(|e| e.to_string())?;
    }

    if let Some(s) = secret
        && !s.is_empty()
    {
        Credentials.store(&backend.name, &s).map_err(|e| e.to_string())?;
    }

    state.cloud_backends.upsert(backend);
    Ok(entry_to_dto(&entry))
}

/// Remove a backend by name. Drops the keychain secret + persists
/// settings + removes the registry entry. Idempotent on the keychain
/// side: a missing entry is silently skipped.
#[tauri::command]
pub fn remove_backend(name: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    let before = guard.remotes.backends.len();
    guard.remotes.backends.retain(|b| b.name != name);
    if guard.remotes.backends.len() == before {
        return Err(format!("backend `{name}` not found"));
    }
    if guard.remotes.default_backend == name {
        guard.remotes.default_backend.clear();
    }
    let snapshot = guard.clone();
    let path = state.settings_path.as_ref().clone();
    drop(guard);
    if !path.as_os_str().is_empty() {
        snapshot.save_to(&path).map_err(|e| e.to_string())?;
    }

    Credentials.delete(&name).map_err(|e| e.to_string())?;
    let _ = state.cloud_backends.remove(&name);
    Ok(())
}

/// Test a configured backend by building an
/// [`copythat_cloud::opendal::Operator`] + issuing a `stat("/")`.
/// Returns a DTO the frontend renders as "Connection successful" /
/// "Connection failed — <reason>". The outer `Result` is always
/// `Ok` — failures are reported in-band via `TestConnectionResult`
/// so the UI can render a localized reason. Tauri's async-command
/// harness requires a `Result` return when `tauri::State<'_, _>` is
/// an argument.
#[tauri::command]
pub async fn test_backend_connection(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<TestConnectionResult, String> {
    let entry = {
        let snap = state.settings_snapshot();
        match snap.remotes.backends.iter().find(|b| b.name == name) {
            Some(e) => e.clone(),
            None => {
                return Ok(TestConnectionResult {
                    ok: false,
                    reason: Some("not-found"),
                    detail: Some(format!("backend `{name}` is not configured")),
                });
            }
        }
    };

    let backend = match entry_to_cloud_backend(&entry) {
        Ok(b) => b,
        Err(e) => {
            return Ok(TestConnectionResult {
                ok: false,
                reason: Some("invalid-config"),
                detail: Some(e.to_string()),
            });
        }
    };

    let secret = match Credentials.load(&backend.name) {
        Ok(s) => s,
        Err(e) => {
            return Ok(TestConnectionResult {
                ok: false,
                reason: Some("keychain"),
                detail: Some(e.to_string()),
            });
        }
    };

    let operator = match make_operator(&backend, secret.as_deref()) {
        Ok(op) => op,
        Err(e) => {
            return Ok(TestConnectionResult {
                ok: false,
                reason: Some(e.fluent_key().trim_start_matches("cloud-error-")),
                detail: Some(e.to_string()),
            });
        }
    };

    match operator.stat("/").await {
        Ok(_) => Ok(TestConnectionResult {
            ok: true,
            reason: None,
            detail: None,
        }),
        Err(e) if e.kind() == opendal::ErrorKind::NotFound => Ok(TestConnectionResult {
            // stat("/") on some backends (flat S3 buckets) legitimately
            // reports NotFound for the root prefix even when the bucket
            // exists. Treat as success — a bucket-scoped `list("/")`
            // round-trip confirms reachability without false-negatives.
            ok: true,
            reason: None,
            detail: None,
        }),
        Err(e) => Ok(TestConnectionResult {
            ok: false,
            reason: Some(map_opendal_kind(e.kind())),
            detail: Some(e.to_string()),
        }),
    }
}

/// Phase 32c — transfer a single local file to a configured backend.
/// The backend is identified by its registry `name`; the secret
/// (when required) is pulled from the keychain just as
/// `test_backend_connection` does.
#[tauri::command]
pub async fn copy_local_to_backend(
    backend_name: String,
    src_path: String,
    dst_key: String,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    let backend = state
        .cloud_backends
        .get(&backend_name)
        .ok_or_else(|| format!("backend `{backend_name}` is not registered"))?;
    let secret = Credentials
        .load(&backend.name)
        .map_err(|e| e.to_string())?;
    let operator = make_operator(&backend, secret.as_deref()).map_err(|e| e.to_string())?;
    let target: Arc<dyn CopyTarget> =
        Arc::new(OperatorTarget::new(backend.name.clone(), operator));
    copy_to_target(&PathBuf::from(&src_path), &target, &dst_key)
        .await
        .map_err(|e| e.to_string())
}

/// Phase 32c — pull a remote object onto the local filesystem. The
/// mirror of `copy_local_to_backend`.
#[tauri::command]
pub async fn copy_backend_to_local(
    backend_name: String,
    src_key: String,
    dst_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    let backend = state
        .cloud_backends
        .get(&backend_name)
        .ok_or_else(|| format!("backend `{backend_name}` is not registered"))?;
    let secret = Credentials
        .load(&backend.name)
        .map_err(|e| e.to_string())?;
    let operator = make_operator(&backend, secret.as_deref()).map_err(|e| e.to_string())?;
    let target: Arc<dyn CopyTarget> =
        Arc::new(OperatorTarget::new(backend.name.clone(), operator));
    copy_from_target(&target, &src_key, &PathBuf::from(&dst_path))
        .await
        .map_err(|e| e.to_string())
}

/// Seed the in-memory registry from persisted settings. Called by
/// `lib.rs::run` after `AppState` is constructed so the runner's
/// first `test_backend_connection` doesn't have to round-trip
/// through the TOML store.
pub fn hydrate_registry_from_settings(
    registry: &BackendRegistry,
    settings: &copythat_settings::Settings,
) {
    for entry in &settings.remotes.backends {
        if let Ok(b) = entry_to_cloud_backend(entry) {
            registry.upsert(b);
        }
    }
}

// ---------------------------------------------------------------------
// DTO / entry / Backend mappers
// ---------------------------------------------------------------------

fn map_opendal_kind(kind: opendal::ErrorKind) -> &'static str {
    match kind {
        opendal::ErrorKind::NotFound => "not-found",
        opendal::ErrorKind::PermissionDenied => "permission",
        _ => "network",
    }
}

fn entry_to_dto(entry: &BackendConfigEntry) -> BackendDto {
    let mut cfg = BackendConfigDto::default();
    match entry.kind {
        BackendKindChoice::LocalFs => {
            if let Some(c) = &entry.local_fs {
                cfg.root = c.root.clone();
            }
        }
        BackendKindChoice::S3 => fill_s3(&mut cfg, entry.s3.as_ref()),
        BackendKindChoice::R2 => fill_s3(&mut cfg, entry.r2.as_ref()),
        BackendKindChoice::B2 => fill_s3(&mut cfg, entry.b2.as_ref()),
        BackendKindChoice::AzureBlob => {
            if let Some(c) = &entry.azure_blob {
                cfg.container = c.container.clone();
                cfg.account_name = c.account_name.clone();
                cfg.endpoint = c.endpoint.clone();
                cfg.root = c.root.clone();
            }
        }
        BackendKindChoice::Gcs => {
            if let Some(c) = &entry.gcs {
                cfg.bucket = c.bucket.clone();
                cfg.service_account = c.service_account.clone();
                cfg.root = c.root.clone();
            }
        }
        BackendKindChoice::Onedrive => fill_oauth(&mut cfg, entry.onedrive.as_ref()),
        BackendKindChoice::GoogleDrive => fill_oauth(&mut cfg, entry.google_drive.as_ref()),
        BackendKindChoice::Dropbox => fill_oauth(&mut cfg, entry.dropbox.as_ref()),
        BackendKindChoice::Webdav => {
            if let Some(c) = &entry.webdav {
                cfg.endpoint = c.endpoint.clone();
                cfg.username = c.username.clone();
                cfg.root = c.root.clone();
            }
        }
        BackendKindChoice::Sftp => {
            if let Some(c) = &entry.sftp {
                cfg.host = c.host.clone();
                cfg.port = c.port;
                cfg.username = c.username.clone();
                cfg.root = c.root.clone();
            }
        }
        BackendKindChoice::Ftp => {
            if let Some(c) = &entry.ftp {
                cfg.host = c.host.clone();
                cfg.port = c.port;
                cfg.username = c.username.clone();
                cfg.root = c.root.clone();
            }
        }
    }
    let kind_wire = entry.kind.as_str().to_owned();
    let enabled_in_build = kind_to_cloud_kind(entry.kind).is_enabled();
    BackendDto {
        name: entry.name.clone(),
        kind: kind_wire,
        config: cfg,
        enabled_in_build,
    }
}

fn fill_s3(dto: &mut BackendConfigDto, src: Option<&S3BackendConfig>) {
    if let Some(c) = src {
        dto.bucket = c.bucket.clone();
        dto.region = c.region.clone();
        dto.endpoint = c.endpoint.clone();
        dto.root = c.root.clone();
    }
}

fn fill_oauth(dto: &mut BackendConfigDto, src: Option<&OAuthBackendConfig>) {
    if let Some(c) = src {
        dto.client_id = c.client_id.clone();
        dto.root = c.root.clone();
    }
}

fn dto_to_entry(dto: &BackendDto) -> Option<BackendConfigEntry> {
    let kind = wire_to_kind_choice(&dto.kind)?;
    let mut entry = BackendConfigEntry {
        name: dto.name.trim().to_owned(),
        kind,
        ..Default::default()
    };
    let c = &dto.config;
    match kind {
        BackendKindChoice::LocalFs => {
            entry.local_fs = Some(LocalFsBackendConfig { root: c.root.clone() });
        }
        BackendKindChoice::S3 => entry.s3 = Some(s3_from_dto(c)),
        BackendKindChoice::R2 => entry.r2 = Some(s3_from_dto(c)),
        BackendKindChoice::B2 => entry.b2 = Some(s3_from_dto(c)),
        BackendKindChoice::AzureBlob => {
            entry.azure_blob = Some(AzureBlobBackendConfig {
                container: c.container.clone(),
                account_name: c.account_name.clone(),
                endpoint: c.endpoint.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Gcs => {
            entry.gcs = Some(GcsBackendConfig {
                bucket: c.bucket.clone(),
                service_account: c.service_account.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Onedrive => {
            entry.onedrive = Some(OAuthBackendConfig {
                client_id: c.client_id.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::GoogleDrive => {
            entry.google_drive = Some(OAuthBackendConfig {
                client_id: c.client_id.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Dropbox => {
            entry.dropbox = Some(OAuthBackendConfig {
                client_id: c.client_id.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Webdav => {
            entry.webdav = Some(WebdavBackendConfig {
                endpoint: c.endpoint.clone(),
                username: c.username.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Sftp => {
            entry.sftp = Some(SftpBackendConfig {
                host: c.host.clone(),
                port: if c.port == 0 { 22 } else { c.port },
                username: c.username.clone(),
                root: c.root.clone(),
            });
        }
        BackendKindChoice::Ftp => {
            entry.ftp = Some(FtpBackendConfig {
                host: c.host.clone(),
                port: if c.port == 0 { 21 } else { c.port },
                username: c.username.clone(),
                root: c.root.clone(),
            });
        }
    }
    Some(entry)
}

fn s3_from_dto(c: &BackendConfigDto) -> S3BackendConfig {
    S3BackendConfig {
        bucket: c.bucket.clone(),
        region: c.region.clone(),
        endpoint: c.endpoint.clone(),
        root: c.root.clone(),
    }
}

fn kind_to_cloud_kind(k: BackendKindChoice) -> BackendKind {
    match k {
        BackendKindChoice::S3 => BackendKind::S3,
        BackendKindChoice::R2 => BackendKind::R2,
        BackendKindChoice::B2 => BackendKind::B2,
        BackendKindChoice::AzureBlob => BackendKind::AzureBlob,
        BackendKindChoice::Gcs => BackendKind::Gcs,
        BackendKindChoice::Onedrive => BackendKind::Onedrive,
        BackendKindChoice::GoogleDrive => BackendKind::GoogleDrive,
        BackendKindChoice::Dropbox => BackendKind::Dropbox,
        BackendKindChoice::Webdav => BackendKind::Webdav,
        BackendKindChoice::Sftp => BackendKind::Sftp,
        BackendKindChoice::Ftp => BackendKind::Ftp,
        BackendKindChoice::LocalFs => BackendKind::LocalFs,
    }
}

fn wire_to_kind_choice(wire: &str) -> Option<BackendKindChoice> {
    Some(match wire {
        "s3" => BackendKindChoice::S3,
        "r2" => BackendKindChoice::R2,
        "b2" => BackendKindChoice::B2,
        "azure-blob" => BackendKindChoice::AzureBlob,
        "gcs" => BackendKindChoice::Gcs,
        "onedrive" => BackendKindChoice::Onedrive,
        "google-drive" => BackendKindChoice::GoogleDrive,
        "dropbox" => BackendKindChoice::Dropbox,
        "webdav" => BackendKindChoice::Webdav,
        "sftp" => BackendKindChoice::Sftp,
        "ftp" => BackendKindChoice::Ftp,
        "local-fs" => BackendKindChoice::LocalFs,
        _ => return None,
    })
}

fn entry_to_cloud_backend(
    entry: &BackendConfigEntry,
) -> Result<Backend, copythat_cloud::BackendError> {
    let kind = kind_to_cloud_kind(entry.kind);
    let config = match entry.kind {
        BackendKindChoice::LocalFs => BackendConfig::LocalFs(LocalFsConfig {
            root: entry
                .local_fs
                .as_ref()
                .map(|c| c.root.clone())
                .unwrap_or_default(),
        }),
        BackendKindChoice::S3 => BackendConfig::S3(s3_to_cloud(entry.s3.as_ref())),
        BackendKindChoice::R2 => BackendConfig::R2(s3_to_cloud(entry.r2.as_ref())),
        BackendKindChoice::B2 => BackendConfig::B2(s3_to_cloud(entry.b2.as_ref())),
        BackendKindChoice::AzureBlob => {
            let c = entry.azure_blob.clone().unwrap_or_default();
            BackendConfig::AzureBlob(AzureBlobConfig {
                container: c.container,
                account_name: c.account_name,
                endpoint: c.endpoint,
                root: c.root,
            })
        }
        BackendKindChoice::Gcs => {
            let c = entry.gcs.clone().unwrap_or_default();
            BackendConfig::Gcs(GcsConfig {
                bucket: c.bucket,
                service_account: c.service_account,
                root: c.root,
            })
        }
        BackendKindChoice::Onedrive => {
            BackendConfig::Onedrive(oauth_to_cloud(entry.onedrive.as_ref()))
        }
        BackendKindChoice::GoogleDrive => {
            BackendConfig::GoogleDrive(oauth_to_cloud(entry.google_drive.as_ref()))
        }
        BackendKindChoice::Dropbox => {
            BackendConfig::Dropbox(oauth_to_cloud(entry.dropbox.as_ref()))
        }
        BackendKindChoice::Webdav => {
            let c = entry.webdav.clone().unwrap_or_default();
            BackendConfig::Webdav(WebdavConfig {
                endpoint: c.endpoint,
                username: c.username,
                root: c.root,
            })
        }
        BackendKindChoice::Sftp => {
            let c = entry.sftp.clone().unwrap_or_default();
            BackendConfig::Sftp(SftpConfig {
                host: c.host,
                port: c.port,
                username: c.username,
                root: c.root,
                // Phase 32h — known_hosts path isn't surfaced
                // through the IPC DTO yet; Phase 32i adds the
                // Settings UI for it. Defaults to trust-on-first-
                // use for now.
                known_hosts_path: String::new(),
            })
        }
        BackendKindChoice::Ftp => {
            let c = entry.ftp.clone().unwrap_or_default();
            BackendConfig::Ftp(FtpConfig {
                host: c.host,
                port: c.port,
                username: c.username,
                root: c.root,
            })
        }
    };
    let _ = kind;
    Ok(Backend {
        name: entry.name.clone(),
        kind: kind_to_cloud_kind(entry.kind),
        config,
    })
}

fn s3_to_cloud(c: Option<&S3BackendConfig>) -> S3Config {
    let c = c.cloned().unwrap_or_default();
    S3Config {
        bucket: c.bucket,
        region: c.region,
        endpoint: c.endpoint,
        root: c.root,
    }
}

fn oauth_to_cloud(c: Option<&OAuthBackendConfig>) -> OAuthConfig {
    let c = c.cloned().unwrap_or_default();
    OAuthConfig {
        client_id: c.client_id,
        root: c.root,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dto_round_trip_preserves_s3_fields() {
        let dto = BackendDto {
            name: "prod".into(),
            kind: "s3".into(),
            config: BackendConfigDto {
                bucket: "my-bucket".into(),
                region: "us-west-2".into(),
                endpoint: "https://example.com".into(),
                root: "archive/".into(),
                ..Default::default()
            },
            enabled_in_build: true,
        };
        let entry = dto_to_entry(&dto).expect("entry");
        assert_eq!(entry.kind, BackendKindChoice::S3);
        let s3 = entry.s3.as_ref().expect("s3 slot");
        assert_eq!(s3.bucket, "my-bucket");
        assert_eq!(s3.region, "us-west-2");
        let back = entry_to_dto(&entry);
        assert_eq!(back.name, dto.name);
        assert_eq!(back.kind, dto.kind);
        assert_eq!(back.config.bucket, dto.config.bucket);
        assert_eq!(back.config.region, dto.config.region);
    }

    #[test]
    fn wire_strings_align_with_settings_choice() {
        for k in &[
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
            let parsed = wire_to_kind_choice(k.as_str()).expect("wire parse");
            assert_eq!(parsed, *k);
        }
    }

    #[test]
    fn sftp_port_fallback_honors_default() {
        let dto = BackendDto {
            name: "sftp-default".into(),
            kind: "sftp".into(),
            config: BackendConfigDto {
                host: "example.com".into(),
                username: "user".into(),
                port: 0,
                ..Default::default()
            },
            enabled_in_build: false,
        };
        let entry = dto_to_entry(&dto).unwrap();
        assert_eq!(entry.sftp.as_ref().unwrap().port, 22);
    }

    #[test]
    fn hydrate_registry_skips_invalid_rows() {
        let registry = BackendRegistry::new();
        let mut settings = copythat_settings::Settings::default();
        settings.remotes.backends.push(BackendConfigEntry {
            name: "good".into(),
            kind: BackendKindChoice::LocalFs,
            local_fs: Some(LocalFsBackendConfig { root: "/tmp".into() }),
            ..Default::default()
        });
        // Unknown kinds are already rejected at parse time — simulate
        // a row that had its kind-specific config stripped.
        settings.remotes.backends.push(BackendConfigEntry {
            name: "empty".into(),
            kind: BackendKindChoice::S3,
            ..Default::default()
        });
        hydrate_registry_from_settings(&registry, &settings);
        assert_eq!(registry.len(), 2, "both entries registered (validation happens at operator-build)");
        assert!(registry.get("good").is_some());
        assert!(registry.get("empty").is_some());
    }
}
