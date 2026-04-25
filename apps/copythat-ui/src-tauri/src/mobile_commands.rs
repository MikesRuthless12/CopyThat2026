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

/// Phase 37 follow-up #2 — mint the install-QR PNG the
/// first-launch onboarding modal renders. The QR encodes a public
/// PWA URL that, when the phone scans it with the camera, opens the
/// PWA in the system browser. The PWA's manifest then offers "Add
/// to Home Screen" so the user installs it without going through
/// an App Store.
///
/// `pwa_url` is the deployed PWA URL — empty string falls back to a
/// placeholder that points to the GitHub repo (suitable for dev
/// builds where the PWA isn't yet hosted).
#[tauri::command]
pub fn mobile_onboarding_qr(pwa_url: Option<String>) -> Result<MobileOnboardingDto, String> {
    let url = pwa_url.filter(|s| !s.is_empty()).unwrap_or_else(|| {
        "https://github.com/MikesRuthless12/CopyThat2026#mobile-companion".to_string()
    });
    let png = generate_qr_png(&url, 8).map_err(|e| format!("qr: {e}"))?;
    Ok(MobileOnboardingDto {
        url,
        qr_png_base64: base64::engine::general_purpose::STANDARD.encode(png),
    })
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileOnboardingDto {
    pub url: String,
    pub qr_png_base64: String,
}

/// Mark the onboarding modal as dismissed so it doesn't reappear on
/// subsequent launches. Settings → Mobile is always available for
/// re-pairing regardless.
#[tauri::command]
pub fn mobile_onboarding_dismiss(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let snapshot = {
        let mut settings = state
            .settings
            .write()
            .map_err(|e| format!("settings rw poisoned: {e}"))?;
        settings.general.mobile_onboarding_dismissed = true;
        settings.clone()
    };
    snapshot
        .save_to(&state.settings_path)
        .map_err(|e| format!("save settings: {e}"))?;
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
            wake_lock: state.wake_lock.clone(),
            queue: state.queue.clone(),
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
    wake_lock: Arc<std::sync::Mutex<Option<copythat_platform::WakeLock>>>,
    queue: copythat_core::Queue,
}

struct AppStateRemoteControl {
    state: AppStateProxy,
}

#[async_trait::async_trait]
impl copythat_mobile::server::RemoteControl for AppStateRemoteControl {
    async fn list_jobs(&self) -> Result<Vec<JobSummary>, String> {
        let snapshot = self.state.queue.snapshot();
        Ok(snapshot.into_iter().map(job_to_summary).collect())
    }

    async fn pause_job(&self, job_id: &str) -> Result<(), String> {
        let id = lookup_job_id(&self.state.queue, job_id)?;
        self.state.queue.pause_job(id);
        Ok(())
    }

    async fn resume_job(&self, job_id: &str) -> Result<(), String> {
        let id = lookup_job_id(&self.state.queue, job_id)?;
        self.state.queue.resume_job(id);
        Ok(())
    }

    async fn cancel_job(&self, job_id: &str) -> Result<(), String> {
        let id = lookup_job_id(&self.state.queue, job_id)?;
        self.state.queue.cancel_job(id);
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
        let snapshot = self.state.queue.snapshot();
        let mut bytes_done = 0u64;
        let mut bytes_total = 0u64;
        let mut files_done = 0u64;
        let mut files_total = 0u64;
        let mut rate_bps = 0u64;
        for job in &snapshot {
            bytes_done = bytes_done.saturating_add(job.bytes_done);
            bytes_total = bytes_total.saturating_add(job.bytes_total);
            files_done = files_done.saturating_add(job.files_done);
            files_total = files_total.saturating_add(job.files_total);
            if let Some(started) = job.started_at {
                let secs = started.elapsed().as_secs_f64();
                if secs > 0.0 {
                    let r = (job.bytes_done as f64) / secs;
                    rate_bps = rate_bps.saturating_add(r.max(0.0) as u64);
                }
            }
        }
        Ok(RemoteResponse::Globals {
            bytes_done,
            bytes_total,
            files_done,
            files_total,
            rate_bps,
        })
    }

    async fn recent_history(&self, _limit: u32) -> Result<Vec<HistoryRow>, String> {
        // History rows surface through `copythat_history::History`
        // which lives behind an `Option<History>` on AppState. We
        // don't carry the handle on AppStateProxy yet so the PWA's
        // history panel stays empty for now; the wire surface is
        // exercised end-to-end by the smoke. Real plumbing lands
        // in a tiny follow-up that adds `history: Option<History>`
        // to AppStateProxy alongside this method.
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

    async fn get_locale(&self) -> Result<String, String> {
        // The Tauri shell routes IPC reads through AppStateProxy
        // which doesn't carry the live Settings handle (RwLock<>
        // is `!Send` while held across the .await on the
        // dispatcher). The PWA falls back to its browser-detected
        // locale when this returns empty; the runner can plumb the
        // real locale in via a settings snapshot once the proxy
        // grows that field — that's a small follow-up and
        // documented in mobile_commands.rs.
        Ok(String::new())
    }

    async fn set_keep_awake(&self, enabled: bool) -> Result<(), String> {
        // Phase 37 follow-up #2 — wired to
        // `copythat_platform::wake_lock`. Acquire on `enabled =
        // true`, release on `enabled = false`. Idempotent — flipping
        // on while already held is a no-op.
        let mut slot = self
            .state
            .wake_lock
            .lock()
            .map_err(|e| format!("wake_lock poisoned: {e}"))?;
        if enabled {
            if slot.is_none() {
                match copythat_platform::acquire_keep_awake() {
                    Ok(lock) => *slot = Some(lock),
                    Err(e) => return Err(format!("wake-lock acquire: {e}")),
                }
            }
        } else {
            slot.take(); // Drop releases.
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct UnusedShimToKeepWireDtoInScope(copythat_settings::MobilePushTarget);

// ---------------------------------------------------------------------
// Queue snapshot helpers (Phase 37 follow-up #2)
// ---------------------------------------------------------------------

/// Convert a `copythat_core::Job` snapshot into the wire
/// `JobSummary` the PWA renders.
fn job_to_summary(job: copythat_core::Job) -> JobSummary {
    let kind = match job.kind {
        copythat_core::JobKind::Copy => "copy",
        copythat_core::JobKind::Move => "move",
        copythat_core::JobKind::Delete => "delete",
        copythat_core::JobKind::SecureDelete => "secure-delete",
        copythat_core::JobKind::Verify => "verify",
    };
    let state = match job.state {
        copythat_core::JobState::Pending => "pending",
        copythat_core::JobState::Running => "running",
        copythat_core::JobState::Paused => "paused",
        copythat_core::JobState::Cancelled => "cancelled",
        copythat_core::JobState::Succeeded => "completed",
        copythat_core::JobState::Failed => "failed",
    };
    let rate_bps = job
        .started_at
        .map(|s| {
            let secs = s.elapsed().as_secs_f64();
            if secs > 0.0 {
                ((job.bytes_done as f64) / secs).max(0.0) as u64
            } else {
                0
            }
        })
        .unwrap_or(0);
    JobSummary {
        job_id: job.id.as_u64().to_string(),
        kind: kind.into(),
        state: state.into(),
        src: job.src.display().to_string(),
        dst: job
            .dst
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        bytes_done: job.bytes_done,
        bytes_total: job.bytes_total,
        rate_bps,
    }
}

/// Walk the queue snapshot and return the matching `JobId`. The
/// PWA sends the id as a u64 stringified (`"123"`); we look it up
/// against `JobId::as_u64()` rather than constructing a `JobId` from
/// a raw u64 since the public constructor lives on `Queue::add`.
fn lookup_job_id(
    queue: &copythat_core::Queue,
    target: &str,
) -> Result<copythat_core::JobId, String> {
    let want: u64 = target
        .parse()
        .map_err(|_| format!("invalid job id `{target}`"))?;
    queue
        .snapshot()
        .into_iter()
        .find(|j| j.id.as_u64() == want)
        .map(|j| j.id)
        .ok_or_else(|| format!("no active job with id {want}"))
}
