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
//! # What's still open
//!
//! - Phone-side authentication beyond DTLS — currently the
//!   `Hello` handshake only checks the device's long-term X25519
//!   public key against `MobileSettings::pairings`. A signed
//!   nonce challenge lands in a follow-up if the threat model
//!   tightens (rogue browser tab on the same WebRTC connection).

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
    CollisionAction, HistoryRow, JobSummary, RemoteCommand, RemoteControl, RemoteResponse,
    SessionAuth, dispatch, dispatch_with_auth,
};
pub use settings::{MobileSettings, validate_peerjs_broker};
