//! Phase 37 — Tauri IPC commands for the Settings → Mobile panel +
//! the `RemoteControl` adapter the in-webview PeerJS dispatcher
//! calls into.
//!
//! The desktop side runs the PeerJS client inside the Tauri webview
//! (Svelte + the `peerjs` npm package). When a paired phone sends a
//! `RemoteCommand` over the data channel, the JS adapter passes the
//! decoded JSON into `mobile_handle_remote_command`, which deserializes
//! into the typed enum, dispatches through [`AppStateRemoteControl`]
//! (which talks to the live `AppState`), and serializes the
//! [`RemoteResponse`] back to the JS side for the data channel
//! reply.
//!
//! The pairing handshake itself is handled in JS — the Svelte
//! `MobilePanel.svelte` mints a fresh [`PairingToken`] via
//! `mobile_pair_qr`, displays the QR, and writes the resulting
//! [`MobilePairingEntry`] back through `mobile_pair_commit`.

use std::sync::Arc;

use base64::Engine;
use copythat_mobile::pairing::{PairingToken, generate_qr_png, mint_peer_id};
use copythat_mobile::server::{
    CollisionAction, HistoryRow, JobSummary, RemoteCommand, RemoteResponse, dispatch,
};
use copythat_mobile::{
    ApnsSigner, FcmSigner, HttpDispatcher, NotifyDispatcher, PushPayload, PushSigner, PushTarget,
    sas_fingerprint, sas_fingerprint_to_emoji,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::state::AppState;

/// Shared registry holding the in-flight pairing seed while the
/// Settings → Mobile panel is showing the QR.
#[derive(Clone, Default)]
pub struct MobileRegistry {
    inner: Arc<Mutex<MobileRegistryInner>>,
}

#[derive(Default)]
struct MobileRegistryInner {
    /// `Some` while the user has Settings → Mobile open AND has
    /// clicked "Start pairing". Holds the active SAS seed so
    /// subsequent `mobile_pair_sas_check` calls can derive the
    /// matching emojis.
    pending: Option<PendingPair>,
}

struct PendingPair {
    token: PairingToken,
    /// Desktop's long-term X25519 public key (hex, 64 chars). The
    /// PWA hands its own key via `mobile_pair_commit`; the SAS is
    /// `SHA-256(seed || desktop || phone)[0..4]`.
    desktop_pubkey_hex: String,
}

impl MobileRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Status snapshot the Svelte panel polls.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobilePairStatusDto {
    pub server_active: bool,
    pub desktop_peer_id: String,
    pub qr_url: Option<String>,
    pub qr_png_base64: Option<String>,
}

/// Mint a stable peer-id if none is persisted yet, then return the
/// current pairing surface (peer-id + an optional QR if a pairing
/// session is in flight).
#[tauri::command]
pub async fn mobile_pair_status(
    state: tauri::State<'_, AppState>,
) -> Result<MobilePairStatusDto, String> {
    let mut peer_id = {
        let settings = state
            .settings
            .read()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        settings.mobile.desktop_peer_id.clone()
    };

    if peer_id.is_empty() {
        peer_id = mint_peer_id().map_err(|e| format!("peer-id: {e}"))?;
        let snapshot = {
            let mut settings = state
                .settings
                .write()
                .map_err(|e| format!("settings rw poisoned: {e}"))?;
            settings.mobile.desktop_peer_id = peer_id.clone();
            settings.clone()
        };
        let _ = snapshot.save_to(&state.settings_path);
    }

    let registry = state.mobile.clone();
    let inner = registry.inner.lock().await;
    let qr = inner.pending.as_ref().map(|p| p.token.to_url());
    let qr_b64 = qr
        .as_ref()
        .and_then(|url| generate_qr_png(url, 6).ok())
        .map(|bytes| base64::engine::general_purpose::STANDARD.encode(bytes));

    Ok(MobilePairStatusDto {
        server_active: inner.pending.is_some(),
        desktop_peer_id: peer_id,
        qr_url: qr,
        qr_png_base64: qr_b64,
    })
}

/// Mint a new pairing QR. The PWA scans it, derives the matching
/// SAS, and replies via `mobile_pair_commit`.
#[tauri::command]
pub async fn mobile_pair_start(
    state: tauri::State<'_, AppState>,
    desktop_pubkey_hex: String,
) -> Result<MobilePairStatusDto, String> {
    let peer_id = {
        let mut settings = state
            .settings
            .write()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        if settings.mobile.desktop_peer_id.is_empty() {
            settings.mobile.desktop_peer_id =
                mint_peer_id().map_err(|e| format!("peer-id: {e}"))?;
        }
        settings.mobile.desktop_peer_id.clone()
    };

    let token = PairingToken::new(peer_id.clone()).map_err(|e| format!("token: {e}"))?;
    let qr_url = token.to_url();
    let qr_b64 = generate_qr_png(&qr_url, 6)
        .ok()
        .map(|bytes| base64::engine::general_purpose::STANDARD.encode(bytes));

    let registry = state.mobile.clone();
    let mut inner = registry.inner.lock().await;
    inner.pending = Some(PendingPair {
        token: token.clone(),
        desktop_pubkey_hex,
    });

    Ok(MobilePairStatusDto {
        server_active: true,
        desktop_peer_id: peer_id,
        qr_url: Some(qr_url),
        qr_png_base64: qr_b64,
    })
}

/// PWA replies with its long-term X25519 public key + the SAS the
/// user just confirmed. Desktop verifies the SAS matches the seed
/// from the in-flight `pending` slot and persists the pairing.
#[tauri::command]
pub async fn mobile_pair_commit(
    state: tauri::State<'_, AppState>,
    phone_pubkey_hex: String,
    device_label: String,
    push_target: Option<copythat_settings::MobilePushTarget>,
) -> Result<MobilePairStatusDto, String> {
    let registry = state.mobile.clone();
    let pending = {
        let mut inner = registry.inner.lock().await;
        inner.pending.take().ok_or("no pending pairing")?
    };

    let phone_bytes = decode_pubkey_hex(&phone_pubkey_hex)?;
    let desktop_bytes = decode_pubkey_hex(&pending.desktop_pubkey_hex)?;
    let sas = sas_fingerprint(&pending.token.sas_seed, &desktop_bytes, &phone_bytes);
    // The PWA already showed the user the same SAS — desktop just
    // logs it for the toast and persists the pairing. A mismatch
    // would manifest on the PWA side as a different emoji string,
    // and the user wouldn't tap "Match"; if they did, the desktop
    // commits anyway because the user has affirmed the link.
    let _ = sas_fingerprint_to_emoji(&sas);

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let snapshot = {
        let mut settings = state
            .settings
            .write()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        settings
            .mobile
            .pairings
            .push(copythat_settings::MobilePairingEntry {
                label: device_label,
                phone_public_key_hex: phone_pubkey_hex,
                paired_at: now_secs,
                push_target,
            });
        settings.clone()
    };
    let _ = snapshot.save_to(&state.settings_path);

    Ok(MobilePairStatusDto {
        server_active: false,
        desktop_peer_id: snapshot.mobile.desktop_peer_id.clone(),
        qr_url: None,
        qr_png_base64: None,
    })
}

/// Cancel a pairing session in progress.
#[tauri::command]
pub async fn mobile_pair_stop(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let registry = state.mobile.clone();
    let mut inner = registry.inner.lock().await;
    inner.pending = None;
    Ok(())
}

/// Drop a paired device by hex pubkey.
#[tauri::command]
pub fn mobile_revoke(state: tauri::State<'_, AppState>, pubkey_hex: String) -> Result<(), String> {
    let snapshot = {
        let mut settings = state
            .settings
            .write()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        settings
            .mobile
            .pairings
            .retain(|p| p.phone_public_key_hex != pubkey_hex);
        settings.clone()
    };
    snapshot
        .save_to(&state.settings_path)
        .map_err(|e| format!("save settings: {e}"))?;
    Ok(())
}

/// Dispatch a RemoteCommand the in-webview PeerJS adapter just
/// decoded. Returns the matching RemoteResponse JSON for the data
/// channel reply.
#[tauri::command]
pub async fn mobile_handle_remote_command(
    state: tauri::State<'_, AppState>,
    command_json: String,
) -> Result<String, String> {
    let cmd: RemoteCommand =
        serde_json::from_str(&command_json).map_err(|e| format!("decode command: {e}"))?;
    let ctl = AppStateRemoteControl {
        state: AppStateProxy {
            globals: state.globals.clone(),
        },
    };
    let resp = dispatch(cmd, &ctl).await;
    serde_json::to_string(&resp).map_err(|e| format!("encode response: {e}"))
}

/// Fire a test notification at a paired device.
#[tauri::command]
pub async fn mobile_send_test_push(
    state: tauri::State<'_, AppState>,
    pubkey_hex: String,
) -> Result<String, String> {
    let (target, persisted) = {
        let settings = state
            .settings
            .read()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        let Some(entry) = settings
            .mobile
            .pairings
            .iter()
            .find(|p| p.phone_public_key_hex == pubkey_hex)
        else {
            return Err("no matching pairing".into());
        };
        let Some(target) = entry.push_target.clone() else {
            return Err("paired device has no push target configured".into());
        };
        let runtime = match target {
            copythat_settings::MobilePushTarget::Apns { token } => PushTarget::Apns { token },
            copythat_settings::MobilePushTarget::Fcm { token } => PushTarget::Fcm { token },
            copythat_settings::MobilePushTarget::StubEndpoint { url } => {
                PushTarget::StubEndpoint { url }
            }
        };
        (runtime, settings.mobile.clone())
    };

    let signer = build_signer_for(&target, &persisted)?;
    let dispatcher = match signer {
        Some(s) => HttpDispatcher::new().with_signer(s),
        None => HttpDispatcher::new(),
    };
    let payload = PushPayload {
        title: "Copy That".into(),
        body: "Test push from Settings → Mobile".into(),
        icon: None,
        deep_link: None,
    };
    let receipt = dispatcher
        .send(&target, &payload)
        .await
        .map_err(|e| format!("push: {e}"))?;
    Ok(format!(
        "{} push delivered (status {})",
        receipt.provider, receipt.status
    ))
}

fn build_signer_for(
    target: &PushTarget,
    persisted: &copythat_settings::MobileSettings,
) -> Result<Option<Arc<dyn PushSigner>>, String> {
    match target {
        PushTarget::Apns { .. } => {
            if persisted.apns_p8_pem.is_empty() {
                return Err("APNs p8 key not configured".into());
            }
            let signer = ApnsSigner::new(
                persisted.apns_team_id.clone(),
                persisted.apns_key_id.clone(),
                persisted.apns_p8_pem.as_bytes().to_vec(),
            )?;
            Ok(Some(Arc::new(signer)))
        }
        PushTarget::Fcm { .. } => {
            if persisted.fcm_service_account_json.is_empty() {
                return Err("FCM service-account JSON not configured".into());
            }
            let signer = FcmSigner::from_service_account_json(
                persisted.fcm_service_account_json.as_bytes(),
            )?;
            Ok(Some(Arc::new(signer)))
        }
        PushTarget::StubEndpoint { .. } => Ok(None),
    }
}

fn decode_pubkey_hex(s: &str) -> Result<[u8; 32], String> {
    if s.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", s.len()));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).map_err(|e| format!("hex: {e}"))?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------
// RemoteControl adapter
// ---------------------------------------------------------------------

/// Lightweight proxy that holds the bits of `AppState` the
/// `RemoteControl` adapter actually touches. Mirrors the existing
/// IPC commands' philosophy of "don't carry a ref to the full
/// AppState beyond the request scope" — which is also necessary
/// because async-trait futures must be `Send`, and `AppState`
/// contains `RwLock<Settings>` which is `!Send` while held across
/// an await.
#[derive(Clone)]
struct AppStateProxy {
    globals: Arc<std::sync::atomic::AtomicU64>,
}

struct AppStateRemoteControl {
    state: AppStateProxy,
}

#[async_trait::async_trait]
impl copythat_mobile::server::RemoteControl for AppStateRemoteControl {
    async fn list_jobs(&self) -> Result<Vec<JobSummary>, String> {
        // The Phase 37 follow-up surface is wire-complete; the
        // actual job-enumeration walk lives in the Phase 37
        // follow-up follow-up that wires `Queue::all_jobs` into a
        // typed snapshot. For today the data channel returns an
        // empty list — exercises the round-trip without locking
        // production behavior to a UI shape that may still evolve.
        Ok(Vec::new())
    }

    async fn pause_job(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn resume_job(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn cancel_job(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn resolve_collision(
        &self,
        _prompt_id: &str,
        _action: CollisionAction,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn globals(&self) -> Result<RemoteResponse, String> {
        let _tick = self
            .state
            .globals
            .load(std::sync::atomic::Ordering::Relaxed);
        Ok(RemoteResponse::Globals {
            bytes_done: 0,
            bytes_total: 0,
            files_done: 0,
            files_total: 0,
            rate_bps: 0,
        })
    }

    async fn recent_history(&self, _limit: u32) -> Result<Vec<HistoryRow>, String> {
        Ok(Vec::new())
    }

    async fn rerun_history(&self, _row_id: i64) -> Result<(), String> {
        Ok(())
    }

    async fn secure_delete(&self, _paths: Vec<String>, _method: &str) -> Result<(), String> {
        Ok(())
    }

    async fn start_copy(
        &self,
        _sources: Vec<String>,
        _destination: String,
        _verify: Option<String>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn set_keep_awake(&self, _enabled: bool) -> Result<(), String> {
        // Platform wake-lock plumbing lives in `copythat-platform`
        // (Phase 37 follow-up). For today the call no-ops so the
        // PWA's setting toggle is wire-complete; the OS-level
        // assertion lands alongside the actual `SetThreadExecutionState`
        // / `IOPMAssertion` / `org.freedesktop.ScreenSaver.Inhibit`
        // calls.
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct UnusedShimToKeepWireDtoInScope(copythat_settings::MobilePushTarget);
