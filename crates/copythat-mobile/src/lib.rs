//! `copythat-mobile` — Phase 37 desktop-side mobile companion.
//!
//! Goal: a phone running the Copy That mobile PWA can pair with the
//! desktop over a PeerJS WebRTC data channel, browse the Phase 9
//! history (read-only), trigger Phase 12 saved profiles plus Phase
//! 36 TOML jobspecs, drive every active job (pause / resume /
//! cancel / resolve collision), and receive APNs / FCM push events
//! when those jobs finish.
//!
//! # Architecture
//!
//! - **Pairing transport.** PeerJS over WebRTC, signaling through
//!   the public PeerJS broker. The desktop registers a stable peer
//!   ID (persisted in [`MobileSettings::desktop_peer_id`]) and the
//!   phone scans a QR carrying `cthat-pair://<peer-id>?sas=<seed>`.
//!   The data channel runs over DTLS so confidentiality + integrity
//!   are handled at the transport layer; this crate only owns the
//!   message vocabulary.
//! - **PWA distribution.** The phone-side app lives in
//!   `apps/copythat-mobile/` (Vite + Svelte 5 + peerjs). Users
//!   "Add to Home Screen" from their browser — no App Store
//!   gatekeeping. The PWA manifest reuses the desktop's `icon.png`
//!   so the home-screen icon matches the desktop tray.
//! - **Push.** APNs ES256 + FCM RS256 JWT signers in [`notify`]
//!   for completion notifications when the data channel is asleep.
//!
//! # Module layout
//!
//! - [`pairing`] — `cthat-pair://` token grammar, four-emoji SAS
//!   derivation, peer-id minting, QR PNG encoder.
//! - [`server`] — over-the-wire `RemoteCommand` / `RemoteResponse`
//!   vocabulary + the [`server::RemoteControl`] async trait the
//!   Tauri shell implements.
//! - [`notify`] — APNs / FCM JWT signers + `HttpDispatcher`.
//! - [`settings`] — runtime `MobileSettings` shape.
//! - [`settings_bridge`] — converts between the persistence shape
//!   in `copythat-settings::MobileSettings` and the runtime shape
//!   here.
//!
//! # What the Phase 37 follow-up shipped
//!
//! - PeerJS pairing-token URL grammar with deterministic SAS
//!   round-trip on both sides of the data channel.
//! - APNs ES256 + FCM RS256 JWT signers + `HttpDispatcher` with
//!   per-target auth header.
//! - `MobileSettings` wired into `copythat-settings::Settings`
//!   under `mobile`, with a settings-bridge round-trip.
//! - `RemoteControl` async-trait + `RemoteCommand` /
//!   `RemoteResponse` vocabulary covering job listing, pause /
//!   resume / cancel, collision resolution, globals, history
//!   browse + rerun, secure delete, and start-copy.
//!
//! # Phase 38 hardening — pairing replay + identity-swap defences
//!
//! The Phase 37 handshake stopped at "looked up the device's
//! pubkey in `MobileSettings::pairings`" and trusted DTLS for the
//! rest. Phase 38 closes three holes the cloud/mobile/sync
//! security review flagged:
//!
//! 1. *Replay protection.* Every paired `Hello` now lands in a
//!    challenge-response handshake — the desktop sends a fresh
//!    32-byte nonce as
//!    [`server::RemoteResponse::Challenge`], the phone signs
//!    `phone_pub || nonce || counter_le` with HMAC-SHA-256 keyed
//!    by the X25519 ECDH shared secret, and the desktop verifies
//!    the MAC in constant time before unlocking any privileged
//!    command. The first counter the phone supplies installs the
//!    high-water mark; subsequent privileged commands advance it
//!    monotonically and any out-of-order arrival is dropped at
//!    the dispatcher.
//! 2. *Identity-swap invalidation.* A mid-session `Hello` that
//!    presents a *different* pubkey wipes the entire
//!    [`server::SessionAuth`] before the new key gets a fresh
//!    challenge — so a rogue browser tab piggy-backed on the
//!    same WebRTC connection can't hijack the active phone's
//!    privileges.
//! 3. *Keep-awake rate limit + audit.* `SetKeepAwake` toggles are
//!    capped at one per
//!    [`server::KEEP_AWAKE_RATE_LIMIT_SECS`] seconds per
//!    session, and every honoured / rejected toggle calls into
//!    [`server::RemoteControl::audit_command`] so the audit-log
//!    sink (when wired) records the activity.

#![forbid(unsafe_code)]

pub mod notify;
pub mod pairing;
pub mod server;
pub mod settings;
pub mod settings_bridge;

pub use notify::{
    ApnsSigner, FcmSigner, HttpDispatcher, NotifyDispatcher, PushPayload, PushReceipt,
    PushSendError, PushSigner, PushTarget,
};
pub use pairing::{
    PAIRING_SAS_SEED_BYTES, PAIRING_SCHEME, PairingError, PairingRecord, PairingToken,
    SAS_EMOJI_SLOTS, SasFingerprint, generate_qr_png, mint_desktop_keypair, mint_peer_id,
    sas_fingerprint, sas_fingerprint_to_emoji,
};
pub use server::{
    CollisionAction, HistoryRow, JobSummary, KEEP_AWAKE_RATE_LIMIT_SECS, RemoteCommand,
    RemoteControl, RemoteResponse, SESSION_MAC_BYTES, SESSION_NONCE_BYTES, SessionAuth,
    compute_challenge_mac, dispatch, dispatch_with_auth,
};
pub use settings::{MobileSettings, validate_peerjs_broker};
