//! Phase 39 — Tauri IPC commands for the Settings → Advanced →
//! "Recovery web UI" panel.
//!
//! The actual server lives in `copythat-recovery`; the Tauri shell's
//! job here is lifecycle (start / stop on toggle, rotate token,
//! report bound URL + token) and persistence (round-trip the
//! `RecoverySettings` struct through `settings.toml` so a restart
//! picks up the same configuration).

use std::net::SocketAddr;
use std::sync::Arc;

use copythat_recovery::{JoinHandle, generate_token, serve};
use copythat_settings::Settings;
use secrecy::SecretString;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::state::AppState;

/// Shared registry that holds the live recovery-server `JoinHandle`
/// while the user has Settings → Advanced → "Recovery web UI"
/// enabled. `None` when the server is idle.
#[derive(Clone, Default)]
pub struct RecoveryRegistry {
    inner: Arc<Mutex<Option<JoinHandle>>>,
}

impl RecoveryRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Status snapshot the Svelte panel polls. Mirrors the persisted
/// `RecoverySettings` plus a `serverActive` bool + the URL the user
/// can paste into a browser.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryStatusDto {
    pub server_active: bool,
    pub enabled: bool,
    pub bind_address: String,
    pub bound_port: u16,
    pub token: String,
    pub allow_non_loopback: bool,
    /// Pre-formatted URL with the token baked in as `?t=`. `None`
    /// when the server isn't running.
    pub url: Option<String>,
}

/// `recovery_status` — read-only snapshot for the Settings panel.
///
/// Phase 40 review-fix: `server_active` reflects `JoinHandle::is_live`
/// rather than just "we have a handle." A long-running shell can have
/// a `JoinHandle` whose underlying axum task ended with an error
/// (port reclaimed, listener died, runtime panic) — in that case the
/// Settings panel needs to surface "stopped" rather than the stale
/// URL. The `is_live` atomic is set false by the spawn task when it
/// exits, so the read here is always current within one poll cycle.
#[tauri::command]
pub async fn recovery_status(
    state: tauri::State<'_, AppState>,
) -> Result<RecoveryStatusDto, String> {
    let recovery = state.settings_snapshot().recovery.clone();
    let lock = state.recovery.inner.lock().await;
    let (server_active, url, bound_port) = match lock.as_ref() {
        Some(handle) if handle.is_live() => {
            let addr = handle.local_addr();
            (
                true,
                Some(format!("http://{}/?t={}", addr, recovery.token)),
                addr.port(),
            )
        }
        _ => (false, None, recovery.port),
    };
    Ok(RecoveryStatusDto {
        server_active,
        enabled: recovery.enabled,
        bind_address: recovery.bind_address,
        bound_port,
        token: recovery.token,
        allow_non_loopback: recovery.allow_non_loopback,
        url,
    })
}

/// `recovery_apply` — read the persisted `RecoverySettings` and
/// start the server if `enabled = true` + stop it otherwise. Called
/// after `update_settings` and at app boot.
#[tauri::command]
pub async fn recovery_apply(
    state: tauri::State<'_, AppState>,
) -> Result<RecoveryStatusDto, String> {
    apply_recovery_settings(&state).await
}

/// `recovery_rotate_token` — generate a fresh token, persist it,
/// restart the server if it was running. Returns the new status
/// snapshot.
#[tauri::command]
pub async fn recovery_rotate_token(
    state: tauri::State<'_, AppState>,
) -> Result<RecoveryStatusDto, String> {
    let new_token = generate_token();
    {
        let mut s = state
            .settings
            .write()
            .map_err(|_| "settings lock poisoned".to_string())?;
        s.recovery.token = new_token;
        save_settings(&state, &s)?;
    }
    apply_recovery_settings(&state).await
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

async fn apply_recovery_settings(state: &AppState) -> Result<RecoveryStatusDto, String> {
    // Mint a token on first enable so the user has something to copy
    // out of the Settings panel. Done before the start path so the
    // persisted settings carry the new token.
    {
        let mut s = state
            .settings
            .write()
            .map_err(|_| "settings lock poisoned".to_string())?;
        if s.recovery.enabled && s.recovery.token.is_empty() {
            s.recovery.token = generate_token();
            save_settings(state, &s)?;
        }
    }

    let recovery = state.settings_snapshot().recovery.clone();
    let mut lock = state.recovery.inner.lock().await;

    // Always tear down what's running so a config change (port,
    // bind_address, allow_non_loopback, fresh token) takes effect.
    if let Some(handle) = lock.take() {
        handle.shutdown().await;
    }

    if !recovery.enabled {
        return Ok(RecoveryStatusDto {
            server_active: false,
            enabled: false,
            bind_address: recovery.bind_address,
            bound_port: recovery.port,
            token: recovery.token,
            allow_non_loopback: recovery.allow_non_loopback,
            url: None,
        });
    }

    let history = state
        .history
        .as_ref()
        .ok_or_else(|| "history database not open".to_string())?
        .clone();

    let chunk_path = copythat_chunk::default_chunk_store_path()
        .map_err(|e| format!("resolve chunk store path: {e}"))?;
    let chunk = copythat_chunk::ChunkStore::open(&chunk_path)
        .map_err(|e| format!("open chunk store: {e}"))?;

    // Force loopback when the user hasn't acknowledged the warning.
    let bind_address = if recovery.allow_non_loopback {
        recovery.bind_address.clone()
    } else {
        "127.0.0.1".to_string()
    };
    let addr_str = format!("{}:{}", bind_address, recovery.port);
    let addr: SocketAddr = addr_str
        .parse()
        .map_err(|e| format!("parse `{addr_str}`: {e}"))?;

    let token: SecretString = recovery.token.clone().into();
    let handle = serve(addr, Arc::new(history), Arc::new(chunk), token)
        .map_err(|e| format!("recovery serve: {e}"))?;

    let bound_port = handle.local_addr().port();
    // Phase 40 review-fix — only persist the OS-assigned port back to
    // settings when the user asked for `port = 0` (auto-pick). If the
    // user explicitly set a fixed port, leave the persisted value
    // alone so a transient port-in-use bind error on the next boot
    // surfaces against the user's chosen port instead of being
    // silently rewritten to whatever ephemeral port the OS happened
    // to hand out the first time. The `recovery.port` snapshot here
    // is the *requested* port (pre-bind); `bound_port` is what the
    // OS actually gave us.
    if recovery.port == 0 {
        let mut s = state
            .settings
            .write()
            .map_err(|_| "settings lock poisoned".to_string())?;
        s.recovery.port = bound_port;
        save_settings(state, &s)?;
    }

    let url = format!("http://{}/?t={}", handle.local_addr(), recovery.token);
    *lock = Some(handle);

    Ok(RecoveryStatusDto {
        server_active: true,
        enabled: true,
        bind_address,
        bound_port,
        token: recovery.token,
        allow_non_loopback: recovery.allow_non_loopback,
        url: Some(url),
    })
}

fn save_settings(state: &AppState, s: &Settings) -> Result<(), String> {
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        return Ok(()); // tests use an empty path; skip persistence
    }
    s.save_to(path).map_err(|e| format!("save settings: {e}"))
}
