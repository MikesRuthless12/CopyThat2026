//! PeerJS pairing-token format, QR encoding, and SAS-emoji
//! fingerprint derivation.
//!
//! Wire format (the QR the desktop shows + the `?` query the PWA
//! parses out of `window.location`):
//!
//! ```text
//! cthat-pair://<peer-id>?sas=<base32-256-bit>
//! ```
//!
//! * `peer-id` — opaque UTF-8 string the desktop registered with
//!   the PeerJS signaling server. By convention we use a 32-character
//!   Crockford-base32 encoding of a 160-bit identifier so the
//!   PeerJS lookup is stable across desktop launches once
//!   `MobileSettings::desktop_peer_id` is persisted.
//! * `sas=<...>` — 256-bit random per-pairing seed. Both sides
//!   derive the same four SAS emojis from
//!   `SHA-256(sas_seed || phone_pubkey || desktop_pubkey)[0..4]`,
//!   so the user can read them off both screens and confirm the
//!   pairing matches before the long-term shared secret commits.
//!
//! The data channel runs over DTLS thanks to WebRTC, so the SAS
//! is the *only* line of defense against a MITM that has somehow
//! convinced the phone to hit a different `peer-id`. Four emojis
//! drawn from a 32-glyph table give 20 bits of entropy — enough to
//! make any online MITM stand out the moment the user looks at the
//! two screens.

use std::fmt;

use base32::Alphabet;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// URL scheme prefix for the pairing QR. Stable wire string —
/// renaming requires a major-version bump on both sides.
pub const PAIRING_SCHEME: &str = "cthat-pair://";

/// Length, in bytes, of the random SAS seed.
pub const PAIRING_SAS_SEED_BYTES: usize = 32;

/// SAS slot count — 4 emojis ≈ 20 bits of human-verifiable entropy.
pub const SAS_EMOJI_SLOTS: usize = 4;

/// The four-emoji SAS table.
const SAS_EMOJI_TABLE: [&str; 32] = [
    "🐶", "🐱", "🦊", "🐼", "🐨", "🐸", "🦁", "🐯", "🐮", "🐷", "🐵", "🦄", "🦋", "🐢", "🐧", "🦉",
    "🌱", "🌳", "🌲", "🍀", "🌸", "🌻", "🌙", "⭐", "🌈", "🍎", "🍋", "🍇", "🍉", "🍓", "🍒", "🍌",
];

/// Top-level error surface for the pairing module.
#[derive(Debug, thiserror::Error)]
pub enum PairingError {
    #[error("pairing token must be a `cthat-pair://` URL, got `{0}`")]
    BadScheme(String),
    #[error("pairing token URL is missing the peer-id segment")]
    MissingPeerId,
    #[error("pairing token URL is missing required query parameter `{0}`")]
    MissingQueryParam(&'static str),
    #[error("pairing token field `{field}` is malformed base32")]
    BadBase32 { field: &'static str },
    #[error("pairing token field `{field}` has length {actual}, expected {expected}")]
    BadFieldLength {
        field: &'static str,
        actual: usize,
        expected: usize,
    },
    #[error("getrandom: {0}")]
    Random(String),
    #[error("qrcodegen: {0}")]
    Qr(String),
}

/// One pairing handshake's wire payload. Round-trips through
/// [`PairingToken::to_url`] / [`PairingToken::parse`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairingToken {
    /// Desktop's PeerJS peer-id. UTF-8 string; opaque to this crate
    /// (PeerJS picks the format).
    pub peer_id: String,
    /// 256-bit random per-pairing seed. Mixed into the SAS hash on
    /// both sides so a stale QR can't be re-used after the user
    /// dismissed the pairing window.
    pub sas_seed: [u8; PAIRING_SAS_SEED_BYTES],
    /// Desktop's long-term X25519 public key (32 bytes). Mixed into
    /// the SAS hash and used by the phone's `Hello` signature
    /// chain. Carried in the QR URL alongside the seed so the
    /// phone has every input it needs to compute the SAS without
    /// an extra round-trip through PeerJS — and so the SAS the
    /// user visually verifies actually binds to the desktop's real
    /// keypair instead of a value the WebView could forge.
    pub desktop_pubkey: [u8; 32],
}

impl PairingToken {
    /// Mint a fresh pairing token. The caller supplies the desktop's
    /// PeerJS peer-id and long-term X25519 public key; this function
    /// generates the SAS seed.
    pub fn new(
        peer_id: impl Into<String>,
        desktop_pubkey: [u8; 32],
    ) -> Result<Self, PairingError> {
        let mut sas_seed = [0u8; PAIRING_SAS_SEED_BYTES];
        getrandom::fill(&mut sas_seed).map_err(|e| PairingError::Random(e.to_string()))?;
        Ok(Self {
            peer_id: peer_id.into(),
            sas_seed,
            desktop_pubkey,
        })
    }

    /// Encode the token as a `cthat-pair://` URL the phone scans
    /// off the QR.
    pub fn to_url(&self) -> String {
        format!(
            "{scheme}{peer}?sas={seed}&dpk={dpk}",
            scheme = PAIRING_SCHEME,
            peer = self.peer_id,
            seed = base32::encode(Alphabet::Crockford, &self.sas_seed),
            dpk = base32::encode(Alphabet::Crockford, &self.desktop_pubkey),
        )
    }

    /// Inverse of [`Self::to_url`]. Surfaces a typed error per
    /// failure mode so the phone can render a precise user-facing
    /// message.
    pub fn parse(url: &str) -> Result<Self, PairingError> {
        let rest = url
            .strip_prefix(PAIRING_SCHEME)
            .ok_or_else(|| PairingError::BadScheme(url.to_string()))?;

        let (peer_id, query) = rest
            .split_once('?')
            .ok_or(PairingError::MissingQueryParam("sas"))?;
        if peer_id.is_empty() {
            return Err(PairingError::MissingPeerId);
        }

        let mut sas_b32: Option<&str> = None;
        let mut dpk_b32: Option<&str> = None;
        let mut seen_sas = 0u32;
        let mut seen_dpk = 0u32;
        for kv in query.split('&') {
            let Some((k, v)) = kv.split_once('=') else {
                continue;
            };
            if k == "sas" {
                seen_sas = seen_sas.saturating_add(1);
                sas_b32 = Some(v);
            } else if k == "dpk" {
                seen_dpk = seen_dpk.saturating_add(1);
                dpk_b32 = Some(v);
            }
        }
        // Reject pairing URLs with more than one `sas=` parameter.
        // The previous shape silently kept the *last* value, so a
        // crafted URL like `?sas=<honest>&sas=<attacker>` would
        // pair against the attacker's seed — and since
        // `mobile_pair_commit` historically did not enforce SAS
        // confirmation either, the attacker-supplied SAS could
        // smuggle the user past the visual emoji check.
        if seen_sas > 1 {
            return Err(PairingError::BadFieldLength {
                field: "sas",
                actual: seen_sas as usize,
                expected: 1,
            });
        }
        if seen_dpk > 1 {
            return Err(PairingError::BadFieldLength {
                field: "dpk",
                actual: seen_dpk as usize,
                expected: 1,
            });
        }
        let sas_b32 = sas_b32.ok_or(PairingError::MissingQueryParam("sas"))?;
        let dpk_b32 = dpk_b32.ok_or(PairingError::MissingQueryParam("dpk"))?;

        let sas_vec = base32::decode(Alphabet::Crockford, sas_b32)
            .ok_or(PairingError::BadBase32 { field: "sas" })?;
        let sas_seed: [u8; PAIRING_SAS_SEED_BYTES] =
            sas_vec
                .clone()
                .try_into()
                .map_err(|_| PairingError::BadFieldLength {
                    field: "sas",
                    actual: sas_vec.len(),
                    expected: PAIRING_SAS_SEED_BYTES,
                })?;
        let dpk_vec = base32::decode(Alphabet::Crockford, dpk_b32)
            .ok_or(PairingError::BadBase32 { field: "dpk" })?;
        let desktop_pubkey: [u8; 32] =
            dpk_vec
                .clone()
                .try_into()
                .map_err(|_| PairingError::BadFieldLength {
                    field: "dpk",
                    actual: dpk_vec.len(),
                    expected: 32,
                })?;

        Ok(Self {
            peer_id: peer_id.to_string(),
            sas_seed,
            desktop_pubkey,
        })
    }
}

/// 4-emoji short-authentication string. `[u8; SAS_EMOJI_SLOTS]`
/// where each byte indexes into [`SAS_EMOJI_TABLE`] modulo 32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SasFingerprint(pub [u8; SAS_EMOJI_SLOTS]);

impl SasFingerprint {
    /// Render the four emoji characters joined by spaces.
    pub fn as_emoji_string(&self) -> String {
        sas_fingerprint_to_emoji(self).join(" ")
    }
}

impl fmt::Display for SasFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_emoji_string())
    }
}

/// Render a [`SasFingerprint`] as four `&'static str` emoji slices
/// suitable for a `Vec<String>` IPC payload.
pub fn sas_fingerprint_to_emoji(sas: &SasFingerprint) -> Vec<&'static str> {
    sas.0
        .iter()
        .map(|b| SAS_EMOJI_TABLE[(*b as usize) % SAS_EMOJI_TABLE.len()])
        .collect()
}

/// Derive the four-emoji SAS fingerprint from the pairing seed +
/// the two long-term X25519 public keys. Both sides feed the same
/// inputs and get the same four emojis out — the user reads them
/// off both screens and confirms the pairing matches.
pub fn sas_fingerprint(
    sas_seed: &[u8; PAIRING_SAS_SEED_BYTES],
    desktop_pubkey: &[u8; 32],
    phone_pubkey: &[u8; 32],
) -> SasFingerprint {
    let mut hasher = Sha256::new();
    hasher.update(sas_seed);
    hasher.update(desktop_pubkey);
    hasher.update(phone_pubkey);
    let digest = hasher.finalize();
    SasFingerprint([digest[0], digest[1], digest[2], digest[3]])
}

/// Persistent record stored in `MobileSettings` after a successful
/// pairing. Includes the device's stable X25519 public key + the
/// human-typed device label.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PairingRecord {
    /// User-supplied device label.
    pub label: String,
    /// 32-byte X25519 long-term public key the phone announced
    /// during the handshake. The desktop never sees the matching
    /// secret.
    pub phone_public_key: [u8; 32],
    /// Unix-epoch seconds when the pairing was committed.
    pub paired_at: i64,
    /// Optional APNs / FCM token the phone hands to the desktop so
    /// the runner can dispatch push notifications.
    pub push_target: Option<crate::notify::PushTarget>,
}

/// Encode an arbitrary URL string as a QR PNG. Used by both the
/// pairing-handshake QR and the first-launch onboarding QR (which
/// points at the PWA install URL).
pub fn generate_qr_png(url: &str, scale: u32) -> Result<Vec<u8>, PairingError> {
    use qrcodegen::{QrCode, QrCodeEcc};
    let qr =
        QrCode::encode_text(url, QrCodeEcc::Medium).map_err(|e| PairingError::Qr(e.to_string()))?;
    let scale = scale.max(1);
    let size = qr.size() as u32;
    let pixel = scale;
    let img_dim = size * pixel;
    let mut rgba = Vec::with_capacity((img_dim * img_dim * 4) as usize);
    for y in 0..img_dim {
        for x in 0..img_dim {
            let qx = (x / pixel) as i32;
            let qy = (y / pixel) as i32;
            let on = qr.get_module(qx, qy);
            let v = if on { 0u8 } else { 255u8 };
            rgba.push(v);
            rgba.push(v);
            rgba.push(v);
            rgba.push(255);
        }
    }
    Ok(encode_png_rgba(&rgba, img_dim))
}

/// Pure-Rust RGBA → PNG encoder. Public-domain implementation in
/// the style of `lodepng-tiny`; chosen over the `png` crate to keep
/// the dep tree small for a feature this narrow.
fn encode_png_rgba(rgba: &[u8], dim: u32) -> Vec<u8> {
    use std::io::Write;

    fn crc32(data: &[u8]) -> u32 {
        const fn table() -> [u32; 256] {
            let mut t = [0u32; 256];
            let mut i = 0;
            while i < 256 {
                let mut c = i as u32;
                let mut k = 0;
                while k < 8 {
                    c = if c & 1 != 0 {
                        0xEDB88320 ^ (c >> 1)
                    } else {
                        c >> 1
                    };
                    k += 1;
                }
                t[i] = c;
                i += 1;
            }
            t
        }
        const TABLE: [u32; 256] = table();
        let mut crc = 0xFFFF_FFFFu32;
        for &b in data {
            crc = TABLE[((crc ^ b as u32) & 0xFF) as usize] ^ (crc >> 8);
        }
        crc ^ 0xFFFF_FFFF
    }

    fn adler32(data: &[u8]) -> u32 {
        const MOD: u32 = 65521;
        let mut a = 1u32;
        let mut b = 0u32;
        for &c in data {
            a = (a + c as u32) % MOD;
            b = (b + a) % MOD;
        }
        (b << 16) | a
    }

    fn write_chunk(out: &mut Vec<u8>, kind: &[u8; 4], payload: &[u8]) {
        out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        let crc_input_start = out.len();
        out.extend_from_slice(kind);
        out.extend_from_slice(payload);
        let crc = crc32(&out[crc_input_start..]);
        out.extend_from_slice(&crc.to_be_bytes());
    }

    let stride = dim as usize * 4;
    let mut filtered = Vec::with_capacity(rgba.len() + dim as usize);
    for row in rgba.chunks(stride) {
        filtered.push(0);
        filtered.extend_from_slice(row);
    }
    let mut zlib = Vec::with_capacity(filtered.len() + 64);
    zlib.extend_from_slice(&[0x78, 0x01]);
    let mut idx = 0;
    while idx < filtered.len() {
        let block_len = (filtered.len() - idx).min(0xFFFF);
        let last = idx + block_len == filtered.len();
        zlib.push(if last { 0x01 } else { 0x00 });
        zlib.extend_from_slice(&(block_len as u16).to_le_bytes());
        zlib.extend_from_slice(&(!(block_len as u16)).to_le_bytes());
        zlib.extend_from_slice(&filtered[idx..idx + block_len]);
        idx += block_len;
    }
    zlib.extend_from_slice(&adler32(&filtered).to_be_bytes());

    let mut out = Vec::with_capacity(zlib.len() + 256);
    out.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&dim.to_be_bytes());
    ihdr.extend_from_slice(&dim.to_be_bytes());
    ihdr.push(8);
    ihdr.push(6);
    ihdr.push(0);
    ihdr.push(0);
    ihdr.push(0);
    write_chunk(&mut out, b"IHDR", &ihdr);

    write_chunk(&mut out, b"IDAT", &zlib);
    write_chunk(&mut out, b"IEND", &[]);

    let _ = out.flush();
    out
}

/// Mint a stable Crockford-base32 peer-id derived from a 160-bit
/// random source. The peer-id is persisted in `MobileSettings` so
/// once the desktop is paired with a phone, subsequent launches
/// register with PeerJS under the same id and the phone can
/// reconnect without re-pairing.
pub fn mint_peer_id() -> Result<String, PairingError> {
    let mut bytes = [0u8; 20];
    getrandom::fill(&mut bytes).map_err(|e| PairingError::Random(e.to_string()))?;
    Ok(base32::encode(Alphabet::Crockford, &bytes))
}

/// Phase 38 follow-up — mint a fresh X25519 keypair for the
/// desktop's long-term pairing identity. The previous pairing
/// flow accepted a desktop pubkey from the WebView (untrusted) and
/// the PWA shipped a hardcoded all-zero key, so every "paired"
/// device collided on the same identity, defeating the SAS as a
/// MITM defence. Now the desktop owns its own keypair, persisted
/// in the OS keychain alongside other secrets.
///
/// Returns `(secret_bytes_32, public_bytes_32)`. Callers are
/// expected to persist the secret out of band (keychain) and
/// expose only the public bytes to the PWA / settings panel.
pub fn mint_desktop_keypair() -> Result<([u8; 32], [u8; 32]), PairingError> {
    use x25519_dalek::{PublicKey, StaticSecret};
    let mut secret_bytes = [0u8; 32];
    getrandom::fill(&mut secret_bytes).map_err(|e| PairingError::Random(e.to_string()))?;
    let secret = StaticSecret::from(secret_bytes);
    let public = PublicKey::from(&secret);
    let secret_out: [u8; 32] = secret.to_bytes();
    let public_out: [u8; 32] = public.to_bytes();
    Ok((secret_out, public_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trips_through_url() {
        let token = PairingToken::new("DESKTOP-PEER-ID-12345", [7u8; 32]).expect("mint");
        let url = token.to_url();
        let parsed = PairingToken::parse(&url).expect("parse");
        assert_eq!(parsed, token);
        assert!(url.starts_with(PAIRING_SCHEME));
        assert!(url.contains("?sas="));
    }

    #[test]
    fn parse_rejects_wrong_scheme() {
        let err = PairingToken::parse("https://nope.example/").unwrap_err();
        assert!(matches!(err, PairingError::BadScheme(_)), "{err:?}");
    }

    #[test]
    fn parse_rejects_missing_sas() {
        let err = PairingToken::parse("cthat-pair://desktop?other=val").unwrap_err();
        assert!(
            matches!(err, PairingError::MissingQueryParam("sas")),
            "{err:?}"
        );
    }

    #[test]
    fn sas_fingerprint_is_deterministic_across_both_sides() {
        let seed = [0xBE; 32];
        let desktop_pub = [1u8; 32];
        let phone_pub = [2u8; 32];
        let a = sas_fingerprint(&seed, &desktop_pub, &phone_pub);
        let b = sas_fingerprint(&seed, &desktop_pub, &phone_pub);
        assert_eq!(a, b);
        // SAS depends on BOTH pubkeys — swapping them changes the
        // hash so an attacker who substitutes one of the keys
        // produces different emojis.
        let c = sas_fingerprint(&seed, &phone_pub, &desktop_pub);
        assert_ne!(a, c);
    }

    #[test]
    fn sas_emoji_render_yields_four_glyphs() {
        let sas = SasFingerprint([1, 2, 3, 4]);
        let glyphs = sas_fingerprint_to_emoji(&sas);
        assert_eq!(glyphs.len(), 4);
        for g in glyphs {
            assert!(!g.is_empty());
        }
    }

    #[test]
    fn qr_png_is_a_valid_png_signature() {
        let token = PairingToken::new("test-peer", [9u8; 32]).expect("mint");
        let png = generate_qr_png(&token.to_url(), 4).expect("qr");
        assert!(png.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
        assert!(png.windows(4).any(|w| w == b"IDAT"));
    }

    #[test]
    fn mint_peer_id_yields_distinct_ids_per_call() {
        let a = mint_peer_id().expect("a");
        let b = mint_peer_id().expect("b");
        assert_ne!(a, b);
        assert!(!a.is_empty());
    }
}
