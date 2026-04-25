//! `copythat-mobile` — Phase 37 desktop-side mobile companion.
//!
//! Goal: a phone (iOS or Android) running the future Tauri Mobile
//! target can pair with the desktop over the local network, browse
//! the Phase 9 history (read-only), trigger Phase 12 saved profiles
//! plus Phase 36 TOML jobspecs, and receive APNs / FCM push events
//! when those jobs finish.
//!
//! This crate is the **desktop side** of that contract:
//!
//! - [`pairing`] — `cthat-pair://` token format, SAS-emoji
//!   fingerprint, X25519 key exchange, [`PairingRecord`] storage
//!   types.
//! - [`server`] — minimal axum 0.8 router that exposes the pair-
//!   begin / pair-complete handshake on a local-network port. The
//!   Tauri runner spins this up while Settings → Mobile shows the
//!   QR code; the server stops the moment the user closes the
//!   Settings panel or a successful pairing lands.
//! - [`notify`] — APNs / FCM push dispatch primitives. Real
//!   provider-token signing (APNs ed25519 JWT, FCM Google service
//!   account) is intentionally deferred to a Phase 37 follow-up;
//!   the call surface is wired here so the runner can hand off a
//!   `PushTarget` without depending on the provider crates today.
//! - [`settings`] — `MobileSettings` sub-struct that persists into
//!   the existing `copythat-settings::Settings` TOML root via the
//!   wire bridge in `copythat-settings::mobile`.
//!
//! # What this crate does NOT contain
//!
//! - The iOS / Android Tauri Mobile binary. Building the iOS target
//!   needs Xcode on a macOS host; the Android target needs the
//!   Android SDK + an emulator. Both are documented as a Phase 37
//!   follow-up in `CopyThat2026-Build-Prompts-Guide2.md`.
//! - The Settings → Mobile Svelte panel. Lives in
//!   `apps/copythat-ui/src/lib/components/settings/MobilePanel.svelte`
//!   in the Phase 37 follow-up; the Tauri IPC commands the panel
//!   calls are stubbed in `apps/copythat-ui/src-tauri/src/mobile.rs`
//!   (also follow-up).
//! - Live progress events streamed back to the phone. The same
//!   `CopyEvent` mpsc channel the GUI consumes is the source; a
//!   bridge that forwards selected events to the paired mobile lands
//!   in the follow-up alongside the actual mobile UI.
//!
//! # Threat model
//!
//! - Pairing happens on a single LAN. The QR code carries the host
//!   IP, the random port, a 256-bit pairing token (base32), and the
//!   server's TLS fingerprint. The phone must scan the QR while
//!   physically near the desktop — no remote attacker can guess the
//!   token + reach the loopback-bound or LAN-bound port in the
//!   ~60-second pairing window.
//! - Both sides verify the SAS fingerprint (4 emojis) before the
//!   X25519 long-term keypair is finalized. Any MITM on the LAN
//!   would have to forge an X25519 public key whose SHA-256
//!   fingerprint produces the same 4 emojis the desktop is showing
//!   — 32 bits of fingerprint entropy makes online MITM obvious.
//! - After pairing, every request from the phone is signed against
//!   the long-term shared secret; replay is blocked by a monotonic
//!   nonce on each side.
//! - Mobile-as-source / sync-to-photos / e2e-encrypted file transfer
//!   are intentionally OUT OF SCOPE for Phase 37 — those need a
//!   file-provider extension on iOS and scoped storage on Android,
//!   which add an order of magnitude of platform-specific surface.

#![forbid(unsafe_code)]

pub mod notify;
pub mod pairing;
pub mod server;
pub mod settings;

pub use notify::{NotifyDispatcher, PushPayload, PushReceipt, PushSendError, PushTarget};
pub use pairing::{
    PAIRING_SCHEME, PairingError, PairingRecord, PairingToken, SasFingerprint, generate_qr_png,
    sas_fingerprint_from_shared_secret, sas_fingerprint_to_emoji, shared_secret_from_token,
};
pub use server::{PairServer, PairServerError, PairServerHandle};
pub use settings::MobileSettings;
