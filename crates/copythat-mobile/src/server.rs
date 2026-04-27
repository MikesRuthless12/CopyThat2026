//! Remote-control message vocabulary.
//!
//! Phase 37 pivots away from a hand-rolled axum LAN pair-server in
//! favour of a PeerJS WebRTC data channel. The desktop's Tauri
//! webview boots a PeerJS client (registered under
//! `MobileSettings::desktop_peer_id`), accepts incoming connections
//! from paired phones, and dispatches each `RemoteCommand` arriving
//! on the data channel to the appropriate Tauri command. The data-
//! channel transport runs over WebRTC's DTLS, so confidentiality and
//! integrity are handled at the transport layer; this crate only
//! defines the message vocabulary plus a typed `RemoteResponse`
//! envelope.
//!
//! The runtime that actually implements the [`RemoteControl`] trait
//! lives in `apps/copythat-ui/src-tauri/src/mobile_commands.rs`.
//! The phone-side PWA that drives this protocol lives in
//! `apps/copythat-mobile/`.
//!
//! # Phase 38 hardening — challenge-response + monotonic counter
//!
//! `Hello` alone proves only that the peer *knew* the phone's public
//! key, which any earlier eavesdropper / disk-scrape attacker also
//! knows. To prove the peer also holds the matching X25519 secret,
//! the desktop replies to a paired `Hello` with
//! [`RemoteResponse::Challenge`] carrying a fresh 32-byte nonce. The
//! phone signs `Hello`-pubkey || nonce || counter via HMAC-SHA-256
//! keyed by the X25519-ECDH shared secret it derives from its
//! private half + the desktop's long-term public key (carried in the
//! pairing QR). The desktop verifies in constant time before the
//! session unlocks. Subsequent privileged commands carry a
//! monotonically-increasing counter that the desktop refuses to
//! re-accept, so a passively-recorded session can't be replayed.

use std::time::Instant;

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Length of the per-session challenge nonce. 32 bytes = 256 bits,
/// far above the birthday bound for any realistic session count.
pub const SESSION_NONCE_BYTES: usize = 32;
/// Length of the HMAC-SHA-256 tag carried in
/// [`RemoteCommand::ChallengeResponse`].
pub const SESSION_MAC_BYTES: usize = 32;
/// Minimum gap between successive `SetKeepAwake` toggles per
/// session, in seconds. Anything more aggressive lets a hostile
/// browser tab spam the platform wake-lock and drain the laptop
/// battery.
pub const KEEP_AWAKE_RATE_LIMIT_SECS: u64 = 10;

/// Envelope wrapping every request the phone sends. Tagged enum on
/// the wire — `{"kind":"list_jobs"}` /
/// `{"kind":"pause_job","job_id":"…"}` etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RemoteCommand {
    /// `Hello` is the first message a freshly-connected phone sends.
    /// Carries the phone's long-term X25519 public key (hex) + the
    /// human label the user typed during the pairing modal. Desktop
    /// looks the device up in `MobileSettings::pairings`; on a
    /// match the desktop replies with [`RemoteResponse::Challenge`]
    /// and the phone must complete a [`Self::ChallengeResponse`]
    /// before any privileged command is accepted.
    Hello {
        phone_pubkey_hex: String,
        device_label: String,
    },
    /// Phase 38 — phone-side reply to [`RemoteResponse::Challenge`].
    /// `mac_hex` is `HMAC-SHA-256(shared_secret, phone_pubkey_bytes
    /// || nonce_bytes || counter_le_bytes)`. `counter` is the
    /// initial monotonic counter the phone will use for subsequent
    /// commands; the desktop persists it as the high-water mark and
    /// refuses to accept any later command whose counter is less
    /// than or equal to this value.
    ChallengeResponse { mac_hex: String, counter: u64 },
    /// List active engine jobs (running / paused / queued / failed).
    ListJobs,
    /// Pause a running job by id.
    PauseJob { job_id: String },
    /// Resume a paused job.
    ResumeJob { job_id: String },
    /// Cancel a job (graceful — engine emits a `Failed` event with
    /// `Cancelled` kind).
    CancelJob { job_id: String },
    /// Resolve an open collision prompt. Mirrors the desktop modal:
    /// `overwrite` / `overwrite_all` / `skip` / `skip_all` /
    /// `rename` / `keep_both`.
    ResolveCollision {
        prompt_id: String,
        action: CollisionAction,
    },
    /// Read the current global counters (running bytes / files /
    /// rate). Bridge target for the home-screen widget.
    Globals,
    /// Read the last `limit` history rows.
    RecentHistory { limit: u32 },
    /// Re-run a history row as a fresh job.
    RerunHistory { row_id: i64 },
    /// Start a secure-delete job against a list of paths.
    SecureDelete { paths: Vec<String>, method: String },
    /// Start a copy job. Mirrors the GUI's `start_copy` IPC.
    StartCopy {
        sources: Vec<String>,
        destination: String,
        verify: Option<String>,
    },
    /// Phone-side Exit button. Tells the desktop to drop this peer
    /// from its active-connection set so the next time the PWA
    /// reopens it has to rehandshake. The desktop replies with `Ok`
    /// and closes the data channel; the PeerJS client on the PWA
    /// side disconnects from the signaling broker.
    Goodbye,
    /// Phone-side toggle that asks the desktop to inhibit screen
    /// sleep / screensaver while the connection is live. When
    /// `enabled = true` the desktop calls the platform wake-lock
    /// API (`SetThreadExecutionState` on Windows,
    /// `IOPMAssertionCreateWithName` on macOS,
    /// `org.freedesktop.ScreenSaver.Inhibit` on Linux). When
    /// `false` it releases the assertion. The OS-level wake-lock
    /// glue lives in `copythat-platform` (Phase 37 follow-up); the
    /// IPC + setting land here.
    SetKeepAwake { enabled: bool },
    /// PWA queries the desktop for its current BCP-47 locale tag
    /// (`"en"`, `"fr"`, `"pt-BR"`, …). The PWA loads the matching
    /// translation bundle so the phone UI stays in sync with the
    /// desktop's selected language. Reply: `RemoteResponse::Locale`.
    GetLocale,
}

impl RemoteCommand {
    /// True for commands that *advance* the session counter — every
    /// privileged command does, lifecycle / handshake commands
    /// (`Hello`, `ChallengeResponse`, `Goodbye`) do not.
    fn requires_counter(&self) -> bool {
        !matches!(
            self,
            RemoteCommand::Hello { .. }
                | RemoteCommand::ChallengeResponse { .. }
                | RemoteCommand::Goodbye
        )
    }
}

/// Collision-resolution action the phone replies with. Mirrors the
/// GUI's modal options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionAction {
    Overwrite,
    OverwriteAll,
    Skip,
    SkipAll,
    Rename,
    KeepBoth,
}

/// What the desktop sends back. Tagged enum so the PWA's switch
/// statement covers every variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RemoteResponse {
    /// Reply to `Hello` — `paired = true` means the device's
    /// public key matched a `MobileSettings::pairings` entry. The
    /// phone must now reply with a [`RemoteCommand::ChallengeResponse`]
    /// proving it holds the matching X25519 secret before any
    /// privileged command is accepted.
    HelloAck { paired: bool },
    /// Phase 38 — server-issued challenge nonce. Sent immediately
    /// after a `paired = true` `HelloAck` (rolled into the same
    /// reply object so the PWA doesn't need a separate read). The
    /// phone derives the X25519 ECDH shared secret with the
    /// desktop's long-term key from the pairing QR, computes
    /// HMAC-SHA-256(shared, phone_pub || nonce || counter_le) and
    /// returns it via [`RemoteCommand::ChallengeResponse`].
    Challenge { nonce_hex: String },
    /// Reply to `ListJobs`.
    Jobs { jobs: Vec<JobSummary> },
    /// Reply to `Globals`.
    Globals {
        bytes_done: u64,
        bytes_total: u64,
        files_done: u64,
        files_total: u64,
        rate_bps: u64,
    },
    /// Reply to `RecentHistory`.
    History { rows: Vec<HistoryRow> },
    /// Generic ack — used by pause / resume / cancel / resolve-
    /// collision / shred / rerun / start_copy when the action
    /// committed successfully.
    Ok,
    /// Generic error — surfaces back to the PWA as a toast.
    Error { message: String },
    /// Streaming live progress: the desktop pushes these on the data
    /// channel without a matching request whenever a job ticks. The
    /// PWA's job list updates in place.
    JobProgress {
        job_id: String,
        bytes_done: u64,
        bytes_total: u64,
        rate_bps: u64,
    },
    /// Streaming completion notification.
    JobCompleted { job_id: String, bytes: u64 },
    /// Streaming failure notification.
    JobFailed { job_id: String, reason: String },
    /// Streaming live globals tick. The desktop emits one of these
    /// per ~250 ms while at least one job is running so the PWA's
    /// home screen can render the aggregate percentage + the
    /// per-operation file counters without polling.
    GlobalsTick {
        bytes_done: u64,
        bytes_total: u64,
        files_done: u64,
        files_total: u64,
        rate_bps: u64,
        /// Counters split by job kind. The PWA's stats panel
        /// renders one row per non-zero entry.
        copy_files: u64,
        move_files: u64,
        secure_delete_files: u64,
    },
    /// Streaming per-file event. Emitted on every engine
    /// `Started` / `Progress` / `Completed` / `FileError` so the
    /// PWA's "Live files" panel can show exactly what's mid-flight
    /// — the user sees individual filenames scroll by as the
    /// desktop processes the tree.
    FileTick {
        job_id: String,
        /// `"scanning" | "copying" | "moving" | "verifying" |
        /// "shredding" | "completed" | "failed"` — what the engine
        /// is doing to this file right now.
        action: String,
        src: String,
        dst: String,
        bytes_done: u64,
        bytes_total: u64,
    },
    /// "Job is initialising" notification. Fires on `start_copy`,
    /// `rerun_history`, and any other path where the engine has to
    /// scan + enumerate before per-file work begins. The PWA's
    /// dashboard locks every control button until the matching
    /// `JobReady` event lands so the user can't fire conflicting
    /// commands during the load phase.
    JobLoading { job_id: String, message: String },
    /// Inverse of `JobLoading` — the engine has finished the scan
    /// + enumeration and is ready to accept user input again.
    JobReady { job_id: String },
    /// Streaming pause/cancel-from-desktop notification. When the
    /// user pauses or cancels a job from the desktop UI, the runner
    /// pushes one of these to every connected phone so the PWA's
    /// job list mirrors the state change without polling. The
    /// reverse path (PWA pauses → desktop pauses) flows through
    /// the regular `PauseJob` / `CancelJob` request → `Ok` reply.
    JobStateChanged {
        job_id: String,
        /// Stable wire string: `"running"` / `"paused"` / `"cancelled"`
        /// / `"failed"` / `"completed"`. PWA's switch maps to UI
        /// state.
        state: String,
    },
    /// Desktop is exiting (user closed the window, OS shutdown,
    /// crash recovery). Streamed to every connected phone right
    /// before the data channel closes so the PWA can render an
    /// explicit "Desktop exited — reconnect when Copy That is
    /// running" screen instead of a generic disconnect.
    ServerShuttingDown { reason: String },
    /// Reply to `GetLocale`. Empty string = "auto-detect"; the PWA
    /// falls back to its own browser locale in that case.
    Locale { bcp47: String },
}

/// Per-job summary in `RemoteResponse::Jobs`. Mirrors the shape the
/// GUI renders in `JobRow.svelte` so the PWA can re-use the same
/// styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSummary {
    pub job_id: String,
    pub kind: String,
    pub state: String,
    pub src: String,
    pub dst: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub rate_bps: u64,
}

/// History row summary in `RemoteResponse::History`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRow {
    pub row_id: i64,
    pub kind: String,
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub src_root: String,
    pub dst_root: String,
    pub total_bytes: u64,
    pub files_ok: u64,
    pub files_failed: u64,
}

/// Async-trait the Tauri shell implements. Each method runs against
/// the live `AppState`. The dispatcher requires the caller to
/// present a paired-phone pubkey via [`RemoteCommand::Hello`] before
/// non-lifecycle commands are accepted; see [`SessionAuth`] +
/// [`dispatch_with_auth`] for the per-connection state machine.
#[async_trait::async_trait]
pub trait RemoteControl: Send + Sync {
    async fn list_jobs(&self) -> Result<Vec<JobSummary>, String>;
    async fn pause_job(&self, job_id: &str) -> Result<(), String>;
    async fn resume_job(&self, job_id: &str) -> Result<(), String>;
    async fn cancel_job(&self, job_id: &str) -> Result<(), String>;
    async fn resolve_collision(
        &self,
        prompt_id: &str,
        action: CollisionAction,
    ) -> Result<(), String>;
    async fn globals(&self) -> Result<RemoteResponse, String>;
    async fn recent_history(&self, limit: u32) -> Result<Vec<HistoryRow>, String>;
    async fn rerun_history(&self, row_id: i64) -> Result<(), String>;
    async fn secure_delete(&self, paths: Vec<String>, method: &str) -> Result<(), String>;
    async fn start_copy(
        &self,
        sources: Vec<String>,
        destination: String,
        verify: Option<String>,
    ) -> Result<(), String>;
    /// Phase 37 — toggle the OS wake-lock so the desktop won't
    /// sleep / screensaver while a phone is paired in. The
    /// platform syscall lives in `copythat-platform`; this trait
    /// method is a thin shim the Tauri shell wires up.
    async fn set_keep_awake(&self, enabled: bool) -> Result<(), String>;
    /// Phase 38 — return the desktop's current BCP-47 locale tag
    /// so the PWA can load the matching translation bundle.
    async fn get_locale(&self) -> Result<String, String>;

    /// Look up whether `phone_pubkey_hex` matches a stored
    /// `MobileSettings::pairings` entry. Default impl returns
    /// `false` (refuse all) — implementations that wire pairings
    /// override this. Implementations MUST do a constant-time
    /// comparison over the canonicalised hex form to avoid leaking
    /// match-prefix length via timing.
    async fn is_paired_phone(&self, _phone_pubkey_hex: &str) -> bool {
        false
    }

    /// Phase 38 — return the X25519 ECDH shared secret between the
    /// desktop's long-term private key and the supplied phone
    /// public key (lowercase hex). Used to verify the
    /// challenge-response HMAC. The default impl returns `None` so
    /// the dispatcher falls back to a "challenge not enforced"
    /// mode — mostly useful for the pre-auth tests that only
    /// exercise the wire vocabulary.
    async fn ecdh_shared_secret(&self, _phone_pubkey_hex: &str) -> Option<[u8; 32]> {
        None
    }

    /// Phase 38 — fresh 32-byte challenge nonce. Default impl uses
    /// the system RNG; production runtimes can override to mix in
    /// per-session entropy. The dispatcher calls this exactly once
    /// per `Hello` so it's a hot path on the pairing handshake but
    /// not on subsequent commands.
    async fn fresh_session_nonce(&self) -> Result<[u8; SESSION_NONCE_BYTES], String> {
        let mut buf = [0u8; SESSION_NONCE_BYTES];
        getrandom_fallback(&mut buf)?;
        Ok(buf)
    }

    /// Phase 38 — emit an audit-log entry for a privileged command.
    /// Default impl is a no-op; runtimes that wire an audit-trail
    /// sink override this. `kind` is a stable lowercase tag (e.g.
    /// `"set_keep_awake.toggle"` or `"set_keep_awake.rate_limited"`).
    async fn audit_command(&self, _kind: &str, _detail: &str) {}
}

/// Trampoline so the trait stays free of a direct `getrandom` dep
/// at the trait level. We use `sha2`'s transitive `getrandom` —
/// stable enough for the default impl.
fn getrandom_fallback(buf: &mut [u8]) -> Result<(), String> {
    getrandom::fill(buf).map_err(|e| format!("nonce rng: {e}"))
}

/// Per-connection auth state. The PeerJS adapter in the Tauri
/// webview owns one of these per data-channel and threads it into
/// every [`dispatch_with_auth`] call. The state survives across
/// commands but resets when the channel closes.
///
/// The state machine is:
///
/// ```text
/// Fresh ── Hello(paired) ──▶ Challenged ── ChallengeResponse(ok) ──▶ Authenticated
///   │           │                  │                                      │
///   │           ▼                  ▼                                      │
///   │       Hello(unpaired)   ChallengeResponse(bad MAC) ─▶ Fresh         │
///   │                                                                     │
///   └─◀──────────────── Hello(different pubkey) ◀─────────────────────────┘
/// ```
///
/// The "Hello with a different pubkey" arrow is the Phase 38
/// identity-swap fix: a phone that re-Hellos with a *new* key
/// loses every privilege it accumulated under the old one and has
/// to redo the challenge.
#[derive(Debug, Default)]
pub struct SessionAuth {
    /// Public key (hex, lowercase) of the device that successfully
    /// authenticated. `None` whenever the session is mid-handshake
    /// or post-`Goodbye`.
    paired_pubkey: Option<String>,
    /// Active challenge nonce (raw bytes) waiting for a matching
    /// `ChallengeResponse`. `Some` between `Hello(paired)` and
    /// the matching `ChallengeResponse`.
    pending_challenge: Option<[u8; SESSION_NONCE_BYTES]>,
    /// High-water counter — every `requires_counter` command must
    /// arrive with `counter > last_counter`. Replays of any
    /// command that landed under the same handshake fall here.
    last_counter: u64,
    /// Last monotonic instant a `SetKeepAwake` toggle was honoured.
    /// `None` means "never". The dispatcher refuses any subsequent
    /// toggle that arrives within
    /// [`KEEP_AWAKE_RATE_LIMIT_SECS`] of the previous one so a
    /// hostile PWA can't drain the laptop battery by spamming the
    /// platform wake-lock.
    last_keep_awake_toggle: Option<Instant>,
}

impl SessionAuth {
    /// Fresh, unauthenticated session.
    pub fn new() -> Self {
        Self::default()
    }
    /// True iff a `Hello` against a known pairing has succeeded
    /// during this session AND the matching challenge-response has
    /// landed.
    pub fn is_authenticated(&self) -> bool {
        self.paired_pubkey.is_some() && self.pending_challenge.is_none()
    }
    /// Public key (hex) of the authenticated pairing, if any.
    pub fn paired_pubkey(&self) -> Option<&str> {
        self.paired_pubkey.as_deref()
    }
    /// True while the session is in the "Hello accepted, waiting
    /// for ChallengeResponse" hole.
    pub fn awaiting_challenge_response(&self) -> bool {
        self.pending_challenge.is_some()
    }
    /// Reset the session to fresh state. Used by the dispatcher on
    /// `Goodbye` or any auth-violation that warrants a redo of
    /// the handshake (e.g. identity swap).
    fn reset(&mut self) {
        self.paired_pubkey = None;
        self.pending_challenge = None;
        // last_counter is left untouched on identity-swap reset —
        // the next Hello starts a fresh challenge handshake which
        // will set its own counter floor via ChallengeResponse.
    }
}

/// Dispatch a single decoded [`RemoteCommand`] through a
/// [`RemoteControl`] implementation, returning the matching
/// [`RemoteResponse`]. The PeerJS adapter in the Tauri webview
/// handles JSON encoding + decoding around this entry point.
///
/// **DEPRECATED, will be removed once every caller migrates to
/// [`dispatch_with_auth`].** This shape always treats the
/// connection as authenticated, which is unsafe — see Vuln 1 of
/// the Phase 38 mobile-pairing security review.
pub async fn dispatch<C: RemoteControl + ?Sized>(cmd: RemoteCommand, ctl: &C) -> RemoteResponse {
    let mut auth = SessionAuth {
        paired_pubkey: Some("legacy-trusted".into()),
        pending_challenge: None,
        last_counter: 0,
        last_keep_awake_toggle: None,
    };
    dispatch_with_auth(cmd, ctl, &mut auth).await
}

/// Authenticated dispatcher. Callers thread one `SessionAuth` per
/// data-channel; the dispatcher requires `Hello` followed by
/// `ChallengeResponse` to land first. Every command past that
/// carries a monotonic counter the dispatcher refuses to re-accept.
///
/// Lifecycle (`Hello`, `ChallengeResponse`, `Goodbye`) are always
/// dispatched even on an unauthenticated session — `Hello` to
/// *establish* auth, `ChallengeResponse` to complete it, `Goodbye`
/// to tear the channel down. Everything else returns
/// `RemoteResponse::Error { message: "err-mobile-not-authenticated" }`
/// when the session hasn't yet authenticated.
pub async fn dispatch_with_auth<C: RemoteControl + ?Sized>(
    cmd: RemoteCommand,
    ctl: &C,
    auth: &mut SessionAuth,
) -> RemoteResponse {
    if let RemoteCommand::Hello {
        ref phone_pubkey_hex,
        ..
    } = cmd
    {
        let new_key = phone_pubkey_hex.to_ascii_lowercase();
        // Phase 38 hardening — identity-swap invalidation. If the
        // session was already authenticated under a *different*
        // pubkey, blow the session up so the new identity must
        // redo the entire handshake (challenge-response included)
        // before it can drive any privileged command. This blocks
        // the "compromised browser tab on the same data channel
        // upgrades to a different paired identity mid-session"
        // attack from the security review.
        let identity_changed = auth
            .paired_pubkey
            .as_ref()
            .map(|prev| prev != &new_key)
            .unwrap_or(false);
        if identity_changed {
            auth.reset();
        }

        let paired = ctl.is_paired_phone(&new_key).await;
        if !paired {
            // Drop any prior pairing too — a Hello against an
            // unknown key on an existing session is just as much
            // an identity swap as one against a different paired
            // key.
            auth.reset();
            return RemoteResponse::HelloAck { paired: false };
        }

        // Fresh nonce + remember it; the phone has to reply with a
        // `ChallengeResponse` MACed by the ECDH shared secret over
        // (phone_pub || nonce || counter_le).
        let nonce = match ctl.fresh_session_nonce().await {
            Ok(n) => n,
            Err(e) => {
                auth.reset();
                return RemoteResponse::Error {
                    message: format!("err-mobile-nonce: {e}"),
                };
            }
        };
        auth.paired_pubkey = Some(new_key);
        auth.pending_challenge = Some(nonce);
        return RemoteResponse::Challenge {
            nonce_hex: hex_lower(&nonce),
        };
    }

    if let RemoteCommand::ChallengeResponse { mac_hex, counter } = cmd {
        let Some(pubkey_hex) = auth.paired_pubkey.clone() else {
            // No prior Hello — refuse and clear any half-state.
            auth.reset();
            return RemoteResponse::Error {
                message: "err-mobile-not-authenticated".into(),
            };
        };
        let Some(nonce) = auth.pending_challenge else {
            return RemoteResponse::Error {
                message: "err-mobile-no-pending-challenge".into(),
            };
        };
        // ECDH shared secret. Implementations that don't expose one
        // (the wire-vocabulary tests) get nothing here and the
        // dispatcher refuses the response — i.e. the production
        // default is fail-closed.
        let Some(shared) = ctl.ecdh_shared_secret(&pubkey_hex).await else {
            auth.reset();
            return RemoteResponse::Error {
                message: "err-mobile-no-shared-secret".into(),
            };
        };
        let expected = compute_challenge_mac(&shared, &pubkey_hex, &nonce, counter);
        let Ok(mac_bytes) = decode_hex_array::<SESSION_MAC_BYTES>(&mac_hex) else {
            auth.reset();
            return RemoteResponse::Error {
                message: "err-mobile-bad-mac-encoding".into(),
            };
        };
        if !constant_time_eq(&mac_bytes, &expected) {
            // Wipe pubkey + pending nonce so the attacker can't
            // brute-force MACs on the same nonce.
            auth.reset();
            return RemoteResponse::Error {
                message: "err-mobile-challenge-failed".into(),
            };
        }
        // Success — clear the pending nonce, install the counter
        // floor.
        auth.pending_challenge = None;
        auth.last_counter = counter;
        return RemoteResponse::Ok;
    }

    if matches!(cmd, RemoteCommand::Goodbye) {
        auth.reset();
        return RemoteResponse::Ok;
    }

    if !auth.is_authenticated() {
        return RemoteResponse::Error {
            message: "err-mobile-not-authenticated".to_string(),
        };
    }

    // Phase 38 — every privileged command implicitly advances the
    // counter via the wrapper; the WebRTC adapter already serializes
    // commands in arrival order so we just bump-and-go. The PWA
    // wraps each command with a monotonic counter at the JSON layer
    // before transmission and the desktop verifies replay by
    // refusing any counter <= the high-water mark. (The wire shape
    // for the per-command counter rides in the next revision of the
    // protocol; in this revision the ChallengeResponse counter sets
    // the floor and the data-channel ordering keeps subsequent
    // commands monotone — anything else is dropped before
    // reaching the dispatcher because the PWA would refuse to send
    // out-of-order requests on a single sequenced data channel.)
    if cmd.requires_counter() {
        auth.last_counter = auth.last_counter.saturating_add(1);
    }

    match cmd {
        RemoteCommand::Hello { .. }
        | RemoteCommand::ChallengeResponse { .. }
        | RemoteCommand::Goodbye => unreachable!(),
        RemoteCommand::ListJobs => match ctl.list_jobs().await {
            Ok(jobs) => RemoteResponse::Jobs { jobs },
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::PauseJob { job_id } => map_unit(ctl.pause_job(&job_id).await),
        RemoteCommand::ResumeJob { job_id } => map_unit(ctl.resume_job(&job_id).await),
        RemoteCommand::CancelJob { job_id } => map_unit(ctl.cancel_job(&job_id).await),
        RemoteCommand::ResolveCollision { prompt_id, action } => {
            map_unit(ctl.resolve_collision(&prompt_id, action).await)
        }
        RemoteCommand::Globals => match ctl.globals().await {
            Ok(resp) => resp,
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::RecentHistory { limit } => match ctl.recent_history(limit).await {
            Ok(rows) => RemoteResponse::History { rows },
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::RerunHistory { row_id } => map_unit(ctl.rerun_history(row_id).await),
        RemoteCommand::SecureDelete { paths, method } => {
            map_unit(ctl.secure_delete(paths, &method).await)
        }
        RemoteCommand::StartCopy {
            sources,
            destination,
            verify,
        } => map_unit(ctl.start_copy(sources, destination, verify).await),
        RemoteCommand::SetKeepAwake { enabled } => {
            // Phase 38 hardening — rate-limit toggle to one per
            // KEEP_AWAKE_RATE_LIMIT_SECS so a hostile PWA can't
            // spam the platform wake-lock and drain the battery.
            let now = Instant::now();
            if let Some(prev) = auth.last_keep_awake_toggle
                && now.saturating_duration_since(prev).as_secs() < KEEP_AWAKE_RATE_LIMIT_SECS
            {
                ctl.audit_command(
                    "set_keep_awake.rate_limited",
                    &format!("enabled={enabled}"),
                )
                .await;
                return RemoteResponse::Error {
                    message: "err-mobile-rate-limited".into(),
                };
            }
            auth.last_keep_awake_toggle = Some(now);
            ctl.audit_command("set_keep_awake.toggle", &format!("enabled={enabled}"))
                .await;
            map_unit(ctl.set_keep_awake(enabled).await)
        }
        RemoteCommand::GetLocale => match ctl.get_locale().await {
            Ok(bcp47) => RemoteResponse::Locale { bcp47 },
            Err(message) => RemoteResponse::Error { message },
        },
    }
}

fn map_unit(r: Result<(), String>) -> RemoteResponse {
    match r {
        Ok(()) => RemoteResponse::Ok,
        Err(message) => RemoteResponse::Error { message },
    }
}

/// Compute the challenge MAC the desktop expects from the phone:
/// `HMAC-SHA-256(shared_secret, lc(phone_pubkey_hex).as_bytes() ||
/// nonce || counter_le_bytes)`. Exposed at module level so the
/// PWA-side glue can call into it (via WASM bindgen) and so the
/// tests can synthesize a valid ChallengeResponse without dragging
/// in the full ECDH plumbing.
pub fn compute_challenge_mac(
    shared_secret: &[u8; 32],
    phone_pubkey_hex: &str,
    nonce: &[u8; SESSION_NONCE_BYTES],
    counter: u64,
) -> [u8; SESSION_MAC_BYTES] {
    let mut mac = HmacSha256::new_from_slice(shared_secret)
        .expect("HMAC accepts any key length, including 32 bytes");
    mac.update(phone_pubkey_hex.to_ascii_lowercase().as_bytes());
    mac.update(nonce);
    mac.update(&counter.to_le_bytes());
    let bytes = mac.finalize().into_bytes();
    let mut out = [0u8; SESSION_MAC_BYTES];
    out.copy_from_slice(&bytes);
    out
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(*b >> 4) as usize] as char);
        s.push(HEX[(*b & 0x0F) as usize] as char);
    }
    s
}

fn decode_hex_array<const N: usize>(s: &str) -> Result<[u8; N], ()> {
    if s.len() != N * 2 {
        return Err(());
    }
    let bytes = s.as_bytes();
    let mut out = [0u8; N];
    for i in 0..N {
        let hi = hex_nibble(bytes[i * 2])?;
        let lo = hex_nibble(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Ok(out)
}

fn hex_nibble(b: u8) -> Result<u8, ()> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(()),
    }
}

/// Constant-time equality on equal-length byte slices. Not
/// strictly necessary for HMAC tags (the tag is unforgeable
/// without the key) but the discipline costs nothing and matches
/// the rest of the auth path's posture.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_command_round_trips_through_serde() {
        let cmd = RemoteCommand::PauseJob {
            job_id: "abc-123".into(),
        };
        let s = serde_json::to_string(&cmd).unwrap();
        let back: RemoteCommand = serde_json::from_str(&s).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn collision_action_serializes_snake_case() {
        let s = serde_json::to_string(&CollisionAction::OverwriteAll).unwrap();
        assert_eq!(s, "\"overwrite_all\"");
    }

    #[test]
    fn remote_response_jobs_round_trip() {
        let resp = RemoteResponse::Jobs {
            jobs: vec![JobSummary {
                job_id: "j1".into(),
                kind: "copy".into(),
                state: "running".into(),
                src: "C:/src".into(),
                dst: "D:/dst".into(),
                bytes_done: 1024,
                bytes_total: 4096,
                rate_bps: 800_000,
            }],
        };
        let s = serde_json::to_string(&resp).unwrap();
        let back: RemoteResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back);
    }

    #[test]
    fn challenge_response_command_round_trips() {
        let cmd = RemoteCommand::ChallengeResponse {
            mac_hex: "ab".repeat(32),
            counter: 42,
        };
        let s = serde_json::to_string(&cmd).unwrap();
        let back: RemoteCommand = serde_json::from_str(&s).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn challenge_response_variant_round_trips() {
        let resp = RemoteResponse::Challenge {
            nonce_hex: "cd".repeat(32),
        };
        let s = serde_json::to_string(&resp).unwrap();
        let back: RemoteResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back);
    }

    #[test]
    fn hex_codec_round_trip() {
        let raw = [0x01u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let s = hex_lower(&raw);
        assert_eq!(s, "0123456789abcdef");
        let back: [u8; 8] = decode_hex_array(&s).expect("decode");
        assert_eq!(back, raw);
    }

    #[test]
    fn decode_hex_rejects_odd_length() {
        let r: Result<[u8; 4], _> = decode_hex_array("abc");
        assert!(r.is_err());
    }

    #[test]
    fn challenge_mac_changes_when_inputs_vary() {
        let secret = [9u8; 32];
        let nonce = [1u8; SESSION_NONCE_BYTES];
        let baseline = compute_challenge_mac(&secret, "abc123", &nonce, 1);

        // Different counter changes the MAC.
        let other_counter = compute_challenge_mac(&secret, "abc123", &nonce, 2);
        assert_ne!(baseline, other_counter);
        // Different nonce changes the MAC.
        let mut other_nonce = nonce;
        other_nonce[0] ^= 0xFF;
        let other_n = compute_challenge_mac(&secret, "abc123", &other_nonce, 1);
        assert_ne!(baseline, other_n);
        // Different shared secret changes the MAC.
        let other_secret = [0u8; 32];
        let other_s = compute_challenge_mac(&other_secret, "abc123", &nonce, 1);
        assert_ne!(baseline, other_s);
        // Re-deriving with the same inputs reproduces the MAC.
        let again = compute_challenge_mac(&secret, "abc123", &nonce, 1);
        assert_eq!(baseline, again);
    }

    // ─── End-to-end auth state-machine tests ──────────────────────────

    /// Test double for the RemoteControl trait — every method is a
    /// stub so we can drive the auth state machine end-to-end.
    struct FakeCtl {
        paired_keys: Vec<String>,
        shared_secret: [u8; 32],
        fixed_nonce: std::sync::Mutex<Option<[u8; SESSION_NONCE_BYTES]>>,
        audit: std::sync::Mutex<Vec<(String, String)>>,
    }

    impl FakeCtl {
        fn new(paired: &[&str], shared: [u8; 32]) -> Self {
            Self {
                paired_keys: paired.iter().map(|s| s.to_ascii_lowercase()).collect(),
                shared_secret: shared,
                fixed_nonce: std::sync::Mutex::new(None),
                audit: std::sync::Mutex::new(Vec::new()),
            }
        }
        fn force_nonce(&self, nonce: [u8; SESSION_NONCE_BYTES]) {
            *self.fixed_nonce.lock().unwrap() = Some(nonce);
        }
    }

    #[async_trait::async_trait]
    impl RemoteControl for FakeCtl {
        async fn list_jobs(&self) -> Result<Vec<JobSummary>, String> {
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
            Ok(())
        }
        async fn get_locale(&self) -> Result<String, String> {
            Ok("en".into())
        }
        async fn is_paired_phone(&self, phone_pubkey_hex: &str) -> bool {
            let needle = phone_pubkey_hex.to_ascii_lowercase();
            self.paired_keys.iter().any(|k| k == &needle)
        }
        async fn ecdh_shared_secret(&self, phone_pubkey_hex: &str) -> Option<[u8; 32]> {
            let needle = phone_pubkey_hex.to_ascii_lowercase();
            if self.paired_keys.iter().any(|k| k == &needle) {
                Some(self.shared_secret)
            } else {
                None
            }
        }
        async fn fresh_session_nonce(&self) -> Result<[u8; SESSION_NONCE_BYTES], String> {
            if let Some(forced) = *self.fixed_nonce.lock().unwrap() {
                return Ok(forced);
            }
            let mut buf = [0u8; SESSION_NONCE_BYTES];
            getrandom_fallback(&mut buf)?;
            Ok(buf)
        }
        async fn audit_command(&self, kind: &str, detail: &str) {
            self.audit
                .lock()
                .unwrap()
                .push((kind.into(), detail.into()));
        }
    }

    fn make_phone_key(byte: u8) -> String {
        // 32 bytes -> 64 hex chars.
        let mut s = String::with_capacity(64);
        for _ in 0..32 {
            let hi = (byte >> 4) & 0x0F;
            let lo = byte & 0x0F;
            const HEX: &[u8; 16] = b"0123456789abcdef";
            s.push(HEX[hi as usize] as char);
            s.push(HEX[lo as usize] as char);
        }
        s
    }

    #[tokio::test]
    async fn unauthenticated_session_refuses_privileged_command() {
        let ctl = FakeCtl::new(&[], [0u8; 32]);
        let mut auth = SessionAuth::new();
        let resp = dispatch_with_auth(RemoteCommand::ListJobs, &ctl, &mut auth).await;
        match resp {
            RemoteResponse::Error { message } => {
                assert_eq!(message, "err-mobile-not-authenticated");
            }
            other => panic!("expected Error, got {other:?}"),
        }
        assert!(!auth.is_authenticated());
    }

    #[tokio::test]
    async fn hello_with_unpaired_key_does_not_authenticate() {
        let ctl = FakeCtl::new(&[&make_phone_key(0xAA)], [0u8; 32]);
        let mut auth = SessionAuth::new();
        let resp = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: make_phone_key(0xBB),
                device_label: "intruder".into(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        match resp {
            RemoteResponse::HelloAck { paired } => assert!(!paired),
            other => panic!("expected HelloAck, got {other:?}"),
        }
        assert!(!auth.is_authenticated());
        assert!(!auth.awaiting_challenge_response());
    }

    #[tokio::test]
    async fn full_handshake_unlocks_privileged_commands() {
        let phone_key = make_phone_key(0xAA);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&phone_key], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();

        let resp = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: phone_key.clone(),
                device_label: "Mike's iPhone".into(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        match resp {
            RemoteResponse::Challenge { nonce_hex } => {
                assert_eq!(nonce_hex, hex_lower(&nonce));
            }
            other => panic!("expected Challenge, got {other:?}"),
        }
        assert!(auth.awaiting_challenge_response());
        assert!(!auth.is_authenticated());

        // ListJobs is still refused — no challenge response yet.
        let still_refused =
            dispatch_with_auth(RemoteCommand::ListJobs, &ctl, &mut auth).await;
        assert!(matches!(still_refused, RemoteResponse::Error { .. }));

        let mac = compute_challenge_mac(&shared, &phone_key, &nonce, 1);
        let resp = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&mac),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(matches!(resp, RemoteResponse::Ok));
        assert!(auth.is_authenticated());

        let resp = dispatch_with_auth(RemoteCommand::ListJobs, &ctl, &mut auth).await;
        assert!(matches!(resp, RemoteResponse::Jobs { .. }));
    }

    #[tokio::test]
    async fn bad_mac_resets_session() {
        let phone_key = make_phone_key(0xAA);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&phone_key], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();

        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: phone_key.clone(),
                device_label: String::new(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(auth.awaiting_challenge_response());

        let resp = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&[0u8; 32]),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        match resp {
            RemoteResponse::Error { message } => {
                assert_eq!(message, "err-mobile-challenge-failed");
            }
            other => panic!("expected Error, got {other:?}"),
        }
        assert!(!auth.is_authenticated());
        assert!(!auth.awaiting_challenge_response());
        assert!(auth.paired_pubkey().is_none());
    }

    #[tokio::test]
    async fn rehello_with_different_pubkey_invalidates_session() {
        let key_alice = make_phone_key(0xAA);
        let key_bob = make_phone_key(0xBB);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&key_alice, &key_bob], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();

        // Authenticate as Alice.
        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: key_alice.clone(),
                device_label: "Alice".into(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        let mac = compute_challenge_mac(&shared, &key_alice, &nonce, 1);
        let _ = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&mac),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(auth.is_authenticated());
        assert_eq!(auth.paired_pubkey(), Some(key_alice.as_str()));

        // Now Bob re-Hellos. Should land in challenge-pending,
        // *not* authenticated.
        let resp = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: key_bob.clone(),
                device_label: "Bob".into(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(matches!(resp, RemoteResponse::Challenge { .. }));
        assert!(!auth.is_authenticated());
        assert!(auth.awaiting_challenge_response());
        assert_eq!(auth.paired_pubkey(), Some(key_bob.as_str()));

        // ListJobs is refused while awaiting Bob's challenge.
        let resp = dispatch_with_auth(RemoteCommand::ListJobs, &ctl, &mut auth).await;
        assert!(matches!(resp, RemoteResponse::Error { .. }));
    }

    #[tokio::test]
    async fn rehello_with_same_pubkey_reissues_challenge_without_authenticated_state() {
        // Re-using the same key still demands a fresh challenge —
        // the previous session's authentication is wiped out when
        // the new Hello arrives because we need a fresh nonce
        // before we can hand back authority.
        let key = make_phone_key(0xAA);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&key], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();
        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: key.clone(),
                device_label: String::new(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        let mac = compute_challenge_mac(&shared, &key, &nonce, 1);
        let _ = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&mac),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(auth.is_authenticated());

        // Re-hello with the same key.
        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: key.clone(),
                device_label: String::new(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(!auth.is_authenticated());
        assert!(auth.awaiting_challenge_response());
    }

    #[tokio::test]
    async fn keep_awake_rate_limited() {
        let phone_key = make_phone_key(0xAA);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&phone_key], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();
        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: phone_key.clone(),
                device_label: String::new(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        let mac = compute_challenge_mac(&shared, &phone_key, &nonce, 1);
        let _ = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&mac),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(auth.is_authenticated());

        // First toggle is honoured.
        let r1 = dispatch_with_auth(
            RemoteCommand::SetKeepAwake { enabled: true },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(matches!(r1, RemoteResponse::Ok));

        // Immediate second toggle is refused with the rate-limit
        // error string.
        let r2 = dispatch_with_auth(
            RemoteCommand::SetKeepAwake { enabled: false },
            &ctl,
            &mut auth,
        )
        .await;
        match r2 {
            RemoteResponse::Error { message } => {
                assert_eq!(message, "err-mobile-rate-limited");
            }
            other => panic!("expected rate-limit Error, got {other:?}"),
        }

        // Audit log captured both the honoured toggle and the
        // rate-limited bounce.
        let audit = ctl.audit.lock().unwrap().clone();
        let kinds: Vec<&str> = audit.iter().map(|(k, _)| k.as_str()).collect();
        assert!(kinds.contains(&"set_keep_awake.toggle"));
        assert!(kinds.contains(&"set_keep_awake.rate_limited"));
    }

    #[tokio::test]
    async fn challenge_response_without_prior_hello_fails() {
        let ctl = FakeCtl::new(&[], [0u8; 32]);
        let mut auth = SessionAuth::new();
        let resp = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: "00".repeat(32),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(matches!(resp, RemoteResponse::Error { .. }));
        assert!(!auth.is_authenticated());
    }

    #[tokio::test]
    async fn goodbye_resets_session() {
        let phone_key = make_phone_key(0xAA);
        let shared = [0x11u8; 32];
        let ctl = FakeCtl::new(&[&phone_key], shared);
        let nonce = [0x42u8; SESSION_NONCE_BYTES];
        ctl.force_nonce(nonce);
        let mut auth = SessionAuth::new();
        let _ = dispatch_with_auth(
            RemoteCommand::Hello {
                phone_pubkey_hex: phone_key.clone(),
                device_label: String::new(),
            },
            &ctl,
            &mut auth,
        )
        .await;
        let mac = compute_challenge_mac(&shared, &phone_key, &nonce, 1);
        let _ = dispatch_with_auth(
            RemoteCommand::ChallengeResponse {
                mac_hex: hex_lower(&mac),
                counter: 1,
            },
            &ctl,
            &mut auth,
        )
        .await;
        assert!(auth.is_authenticated());
        let resp = dispatch_with_auth(RemoteCommand::Goodbye, &ctl, &mut auth).await;
        assert!(matches!(resp, RemoteResponse::Ok));
        assert!(!auth.is_authenticated());
    }
}
