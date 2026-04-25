//! Local-network pair server.
//!
//! The runner spins this up while the user has Settings → Mobile
//! open. The server exposes two routes:
//!
//! * `POST /pair/begin` — phone announces itself with its X25519
//!   public key + an optional `device_label`. The desktop replies
//!   with its own ephemeral X25519 public key. Both sides derive
//!   the shared secret and SAS fingerprint; the desktop renders the
//!   4-emoji SAS and waits for the human-side "match" tap.
//! * `POST /pair/complete` — phone sends the user-confirmed pairing
//!   token back. The desktop persists the [`PairingRecord`] and
//!   shuts the server down.
//!
//! The TLS cert is ephemeral and pinned by fingerprint at scan-time
//! — no PKI, no domain validation. Real cert generation lands in a
//! Phase 37 follow-up that pulls in `rcgen`; today the server runs
//! plain HTTP on the loopback address (`127.0.0.1`) so the smoke
//! test can exercise the protocol end-to-end without TLS plumbing.
//! Production binds to the LAN interface with TLS once the cert
//! generation lands.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;

use crate::pairing::{
    PAIRING_TOKEN_BYTES, PairingRecord, PairingToken, SasFingerprint, shared_secret_from_token,
};

/// Top-level error surface for the pair server.
#[derive(Debug, thiserror::Error)]
pub enum PairServerError {
    #[error("bind 127.0.0.1:{port}: {source}")]
    Bind {
        port: u16,
        #[source]
        source: std::io::Error,
    },
    #[error("axum serve: {0}")]
    Serve(String),
    #[error("server already stopped")]
    AlreadyStopped,
}

/// Shared server state accessible from inside the route handlers.
#[derive(Clone)]
struct ServerState {
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    /// Desktop's ephemeral X25519 secret. Re-minted per server run.
    desktop_secret: x25519_dalek::StaticSecret,
    /// The `PairingToken` the runner advertised on the QR.
    expected_token: PairingToken,
    /// Set after `/pair/begin` completes. Drives the SAS panel.
    pending: Option<PendingPair>,
    /// Set after `/pair/complete` succeeds. The runner reads this
    /// out before stopping the server.
    committed: Option<PairingRecord>,
}

#[derive(Debug, Clone)]
struct PendingPair {
    phone_public: x25519_dalek::PublicKey,
    label: String,
    sas: SasFingerprint,
}

/// Public handle the runner holds while the server is alive.
pub struct PairServerHandle {
    addr: SocketAddr,
    state: ServerState,
    join: Option<JoinHandle<Result<(), PairServerError>>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl PairServerHandle {
    /// LAN address the server actually bound to (with the picked port).
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Read the latest SAS fingerprint announced by a `/pair/begin`
    /// caller. `None` means no phone has announced yet.
    pub async fn pending_sas(&self) -> Option<SasFingerprint> {
        self.state
            .inner
            .lock()
            .await
            .pending
            .as_ref()
            .map(|p| p.sas)
    }

    /// Read out a successful pairing record once the phone has
    /// finished `/pair/complete`. The server caller is expected to
    /// poll this and call [`Self::shutdown`] when it's `Some`.
    pub async fn committed(&self) -> Option<PairingRecord> {
        self.state.inner.lock().await.committed.clone()
    }

    /// Cooperative shutdown. Idempotent.
    pub async fn shutdown(&mut self) -> Result<(), PairServerError> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.join.take() {
            handle
                .await
                .map_err(|e| PairServerError::Serve(format!("join: {e}")))??;
        }
        Ok(())
    }
}

/// Construction-time builder for the pair server.
#[derive(Debug, Default, Clone, Copy)]
pub struct PairServer {
    /// Port to bind on. `0` lets the OS pick a free ephemeral port,
    /// which is what the runner uses by default.
    pub port: u16,
}

impl PairServer {
    /// Spawn the server. Returns once the listener is bound + the
    /// route handlers are wired. The caller wraps the returned
    /// [`PairingToken`] in a QR via [`crate::generate_qr_png`].
    pub async fn start(self) -> Result<(PairServerHandle, PairingToken), PairServerError> {
        let host = "127.0.0.1";
        let listener = TcpListener::bind((host, self.port))
            .await
            .map_err(|source| PairServerError::Bind {
                port: self.port,
                source,
            })?;
        let addr = listener
            .local_addr()
            .map_err(|source| PairServerError::Bind {
                port: self.port,
                source,
            })?;

        // Mint the desktop's ephemeral keypair + the pairing token
        // the runner advertises on the QR.
        let mut desktop_secret_bytes = [0u8; 32];
        getrandom::fill(&mut desktop_secret_bytes)
            .map_err(|e| PairServerError::Serve(format!("getrandom: {e}")))?;
        let desktop_secret = x25519_dalek::StaticSecret::from(desktop_secret_bytes);
        let desktop_public = x25519_dalek::PublicKey::from(&desktop_secret);
        let token = PairingToken::new(host, addr.port(), &desktop_public)
            .map_err(|e| PairServerError::Serve(format!("token mint: {e}")))?;

        let state = ServerState {
            inner: Arc::new(Mutex::new(ServerInner {
                desktop_secret,
                expected_token: token.clone(),
                pending: None,
                committed: None,
            })),
        };

        let app = Router::new()
            .route("/pair/begin", post(handle_pair_begin))
            .route("/pair/complete", post(handle_pair_complete))
            .with_state(state.clone());

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let join = tokio::spawn(async move {
            let server = axum::serve(listener, app).with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            });
            server
                .await
                .map_err(|e| PairServerError::Serve(e.to_string()))
        });

        Ok((
            PairServerHandle {
                addr,
                state,
                join: Some(join),
                shutdown_tx: Some(shutdown_tx),
            },
            token,
        ))
    }
}

// ---------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct BeginRequest {
    /// Hex-encoded 32-byte X25519 public key the phone owns.
    phone_pubkey: String,
    /// Phone-side device label. The desktop persists this if the
    /// pairing commits.
    device_label: String,
    /// 64-character base32 token from the QR. Both sides MUST agree
    /// before the server admits a pairing attempt.
    token: String,
}

#[derive(Debug, Clone, Serialize)]
struct BeginResponse {
    /// Hex-encoded 32-byte X25519 public key the desktop minted at
    /// server start. The phone derives the SAS fingerprint from
    /// `dh(phone_secret, this)` and shows the user the same 4
    /// emojis the desktop is showing.
    desktop_pubkey: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CompleteRequest {
    token: String,
    /// Optional APNs / FCM token the phone is offering up.
    push_target: Option<crate::notify::PushTarget>,
}

#[derive(Debug, Clone, Serialize)]
struct CompleteResponse {
    paired: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ErrorResponse {
    error: String,
}

// ---------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------

async fn handle_pair_begin(
    State(state): State<ServerState>,
    Json(req): Json<BeginRequest>,
) -> impl IntoResponse {
    let mut inner = state.inner.lock().await;
    if !token_matches(&inner.expected_token, &req.token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "pairing token mismatch".into(),
            }),
        )
            .into_response();
    }

    let phone_pub = match decode_pubkey(&req.phone_pubkey) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("phone_pubkey: {e}"),
                }),
            )
                .into_response();
        }
    };

    let (_shared, sas) = shared_secret_from_token(&inner.desktop_secret, &phone_pub);
    inner.pending = Some(PendingPair {
        phone_public: phone_pub,
        label: req.device_label,
        sas,
    });

    let desktop_pub = x25519_dalek::PublicKey::from(&inner.desktop_secret);
    let resp = BeginResponse {
        desktop_pubkey: hex_encode(desktop_pub.as_bytes()),
    };
    (StatusCode::OK, Json(resp)).into_response()
}

async fn handle_pair_complete(
    State(state): State<ServerState>,
    Json(req): Json<CompleteRequest>,
) -> impl IntoResponse {
    let mut inner = state.inner.lock().await;
    if !token_matches(&inner.expected_token, &req.token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "pairing token mismatch".into(),
            }),
        )
            .into_response();
    }

    let pending = match inner.pending.as_ref() {
        Some(p) => p.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "no pending pair-begin to complete".into(),
                }),
            )
                .into_response();
        }
    };

    let now_secs = chrono_now_secs();
    let record = PairingRecord {
        label: pending.label,
        phone_public_key: *pending.phone_public.as_bytes(),
        paired_at: now_secs,
        push_target: req.push_target,
    };
    inner.committed = Some(record);
    let resp = CompleteResponse { paired: true };
    (StatusCode::OK, Json(resp)).into_response()
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn token_matches(expected: &PairingToken, supplied_b32: &str) -> bool {
    let Some(decoded) = base32::decode(base32::Alphabet::Crockford, supplied_b32) else {
        return false;
    };
    decoded.len() == PAIRING_TOKEN_BYTES && constant_time_eq(&decoded, &expected.token_bytes)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn decode_pubkey(hex: &str) -> Result<x25519_dalek::PublicKey, &'static str> {
    if hex.len() != 64 {
        return Err("expected 64 hex characters");
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        let pair = &hex[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(pair, 16).map_err(|_| "non-hex char")?;
    }
    Ok(x25519_dalek::PublicKey::from(bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}

fn chrono_now_secs() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
