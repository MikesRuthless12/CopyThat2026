//! Phase 48 — Tauri IPC commands for the Settings → Server panel.
//!
//! The actual server lives in `copythat-server`; the Tauri shell's job
//! here is lifecycle (start / stop on the panel's Start / Stop control)
//! and reporting (running? bound address? `/metrics` URL). The persisted
//! `ServerSettings` round-trips through `settings.toml` with the rest of
//! the settings (the panel edits it via the standard `update_settings`
//! flow); these commands read that saved config when the user clicks
//! Start. Mirrors the Phase 39 `recovery_commands` registry shape.

use std::path::PathBuf;
use std::sync::Arc;

use copythat_server::{AuthMode, Protocol, ServerConfig, ServerHandle, serve};
use serde::Serialize;
use tokio::sync::Mutex;

use crate::state::AppState;

/// Shared registry holding the live `ServerHandle` while the user has a
/// server running from Settings → Server. `None` when stopped. A
/// `tokio::sync::Mutex` (not `std::sync`) because the start / stop
/// commands hold it across the `serve().await` / `shutdown().await`
/// suspension points.
#[derive(Clone, Default)]
pub struct ServerRegistry {
    pub inner: Arc<Mutex<Option<ServerHandle>>>,
}

impl ServerRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Status snapshot the Settings panel renders. `metrics_url` is set only
/// while running with an HTTP-family protocol (WebDAV / HTTP) enabled —
/// the `/metrics` endpoint shares that listener; SFTP / S3 have no scrape
/// surface.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusDto {
    pub running: bool,
    /// The socket the listener actually bound to (reflects the
    /// OS-assigned port when the config used `:0`). `None` when stopped.
    pub bound_addr: Option<String>,
    /// `http://<addr>/metrics` when running with WebDAV / HTTP on.
    pub metrics_url: Option<String>,
}

impl ServerStatusDto {
    fn stopped() -> Self {
        Self {
            running: false,
            bound_addr: None,
            metrics_url: None,
        }
    }
}

/// Build a live [`ServerConfig`] from the persisted [`ServerSettings`].
fn build_config(s: &copythat_settings::ServerSettings) -> ServerConfig {
    let mut protocols = Vec::new();
    if s.webdav {
        protocols.push(Protocol::WebDav);
    }
    if s.http {
        protocols.push(Protocol::Http);
    }
    if s.s3 {
        protocols.push(Protocol::S3);
    }
    if s.sftp {
        protocols.push(Protocol::Sftp);
    }

    let auth = match s.auth.mode.as_str() {
        "bearer" => AuthMode::Bearer {
            token: s.auth.token.clone(),
        },
        "basic" => AuthMode::Basic {
            user: s.auth.user.clone(),
            password: s.auth.password.clone(),
        },
        _ => AuthMode::None,
    };

    // Empty root → the process working directory, matching
    // `ServerConfig::default`'s `"."`.
    let root = if s.root.trim().is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(&s.root)
    };

    ServerConfig {
        bind_addr: s.bind_addr.clone(),
        protocols,
        auth,
        root,
        readonly: s.readonly,
    }
}

/// Snapshot a live handle into the wire DTO.
fn status_from_handle(handle: &ServerHandle) -> ServerStatusDto {
    let addr = handle.local_addr();
    let http_family = handle.config().protocols.iter().any(|p| p.is_http_family());
    ServerStatusDto {
        running: true,
        bound_addr: Some(addr.to_string()),
        metrics_url: http_family.then(|| format!("http://{addr}/metrics")),
    }
}

/// `server_status` — read-only snapshot for the Settings panel.
#[tauri::command]
pub async fn server_status(state: tauri::State<'_, AppState>) -> Result<ServerStatusDto, String> {
    let lock = state.server.inner.lock().await;
    Ok(match lock.as_ref() {
        Some(handle) => status_from_handle(handle),
        None => ServerStatusDto::stopped(),
    })
}

/// `server_start` — build a [`ServerConfig`] from the saved
/// [`ServerSettings`], `serve()` it, and stash the handle. Any prior
/// handle is torn down first so a config change takes effect. Returns
/// the bound address (via the status DTO) or the server error's stable
/// Fluent key (`err-server-*`) so the frontend toast can localize it.
#[tauri::command]
pub async fn server_start(state: tauri::State<'_, AppState>) -> Result<ServerStatusDto, String> {
    let settings = state.settings_snapshot().server.clone();
    let config = build_config(&settings);

    let mut lock = state.server.inner.lock().await;
    // Tear down anything already running so a restart picks up the new
    // config (bind address, protocols, auth, root, read-only).
    if let Some(handle) = lock.take() {
        handle.shutdown().await;
    }

    let handle = serve(config)
        .await
        .map_err(|e| e.localized_key().to_string())?;
    let status = status_from_handle(&handle);
    *lock = Some(handle);
    Ok(status)
}

/// `server_stop` — take the live handle and drain it. Idempotent: a
/// stop with nothing running just reports stopped.
#[tauri::command]
pub async fn server_stop(state: tauri::State<'_, AppState>) -> Result<ServerStatusDto, String> {
    let mut lock = state.server.inner.lock().await;
    if let Some(handle) = lock.take() {
        handle.shutdown().await;
    }
    Ok(ServerStatusDto::stopped())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_maps_protocols_and_auth() {
        let s = copythat_settings::ServerSettings {
            webdav: true,
            http: false,
            s3: false,
            sftp: false,
            bind_addr: "127.0.0.1:0".to_string(),
            root: String::new(),
            readonly: true,
            auth: copythat_settings::ServerAuthSettings {
                mode: "bearer".to_string(),
                token: "tok".to_string(),
                ..Default::default()
            },
            otel_endpoint: String::new(),
            webhooks: Vec::new(),
        };
        let cfg = build_config(&s);
        assert_eq!(cfg.protocols, vec![Protocol::WebDav]);
        assert!(cfg.readonly);
        assert_eq!(cfg.root, PathBuf::from("."));
        assert!(matches!(cfg.auth, AuthMode::Bearer { token } if token == "tok"));
    }

    #[test]
    fn build_config_unknown_auth_falls_back_to_none() {
        let s = copythat_settings::ServerSettings {
            auth: copythat_settings::ServerAuthSettings {
                mode: "garbage".to_string(),
                ..Default::default()
            },
            root: "/srv".to_string(),
            ..Default::default()
        };
        let cfg = build_config(&s);
        assert!(matches!(cfg.auth, AuthMode::None));
        assert!(cfg.protocols.is_empty());
        assert_eq!(cfg.root, PathBuf::from("/srv"));
    }
}
