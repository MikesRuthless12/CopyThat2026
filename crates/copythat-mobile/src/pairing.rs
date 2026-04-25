//! Pairing-token format, QR encoding, SAS-emoji fingerprint, and
//! X25519 key exchange.
//!
//! Wire format:
//!
//! ```text
//! cthat-pair://<host>:<port>?token=<base32-256-bit>&fingerprint=<base32-256-bit>
//! ```
//!
//! * `host` — IPv4 / IPv6 / hostname the desktop binds the pair-server to.
//! * `port` — random ephemeral port the pair-server picked.
//! * `token` — 256-bit one-shot pairing secret, base32-encoded
//!   (Crockford alphabet, no padding) so the QR stays short and
//!   unambiguous.
//! * `fingerprint` — SHA-256 of the desktop's ephemeral X25519
//!   public key, base32-encoded. The phone uses this to authenticate
//!   the desktop's first request (TOFU on the phone side, replayed
//!   to the desktop SAS panel for human-side confirmation).

use std::fmt;

use base32::Alphabet;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};

/// URL scheme prefix for the pairing QR. Stable wire string —
/// renaming the variant requires a major-version bump on the mobile
/// app side.
pub const PAIRING_SCHEME: &str = "cthat-pair://";

/// The four emojis shown in the SAS panel on both desktop and phone.
/// Picked to be visually distinct + culturally neutral; the index
/// space is `0..256` (one byte per slot, mod 32 of the table below).
const SAS_EMOJI_TABLE: [&str; 32] = [
    "🐶", "🐱", "🦊", "🐼", "🐨", "🐸", "🦁", "🐯", "🐮", "🐷", "🐵", "🦄", "🦋", "🐢", "🐧", "🦉",
    "🌱", "🌳", "🌲", "🍀", "🌸", "🌻", "🌙", "⭐", "🌈", "🍎", "🍋", "🍇", "🍉", "🍓", "🍒", "🍌",
];

/// Length, in bytes, of the random pairing token. 256 bits keeps a
/// brute-force attempt against the 60-second pairing window
/// astronomical.
pub const PAIRING_TOKEN_BYTES: usize = 32;

/// Fingerprint length matches the SHA-256 digest length.
pub const PAIRING_FINGERPRINT_BYTES: usize = 32;

/// SAS slot count — 4 emojis ≈ 20 bits of human-verifiable entropy
/// per emoji table position (5 bits × 4 = 20 bits).
pub const SAS_EMOJI_SLOTS: usize = 4;

/// Top-level error surface for the pairing module.
#[derive(Debug, thiserror::Error)]
pub enum PairingError {
    #[error("pairing token must be a `cthat-pair://` URL, got `{0}`")]
    BadScheme(String),
    #[error("pairing token URL is missing the `host:port` component")]
    MissingHostPort,
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
    pub host: String,
    pub port: u16,
    /// 256-bit one-shot pairing secret. Used by the phone to
    /// authenticate the very first request to the desktop's
    /// `/pair/begin` endpoint.
    pub token_bytes: [u8; PAIRING_TOKEN_BYTES],
    /// SHA-256 of the desktop's ephemeral X25519 public key.
    pub fingerprint_bytes: [u8; PAIRING_FINGERPRINT_BYTES],
}

impl PairingToken {
    /// Mint a fresh pairing token on the desktop side. `host` /
    /// `port` are passed in by the caller (the axum server resolves
    /// them after binding).
    pub fn new(
        host: impl Into<String>,
        port: u16,
        ephemeral_pubkey: &PublicKey,
    ) -> Result<Self, PairingError> {
        let mut token_bytes = [0u8; PAIRING_TOKEN_BYTES];
        getrandom::fill(&mut token_bytes).map_err(|e| PairingError::Random(e.to_string()))?;
        let fingerprint_bytes = sha256_of(ephemeral_pubkey.as_bytes());
        Ok(Self {
            host: host.into(),
            port,
            token_bytes,
            fingerprint_bytes,
        })
    }

    /// Encode the token as a `cthat-pair://` URL the phone scans
    /// off the QR. The base32 alphabet is Crockford (no padding) so
    /// the QR contains only `[0-9A-Z]` — the densest QR mode that
    /// stays readable in low-light.
    pub fn to_url(&self) -> String {
        format!(
            "{scheme}{host}:{port}?token={token}&fingerprint={fp}",
            scheme = PAIRING_SCHEME,
            host = self.host,
            port = self.port,
            token = base32::encode(Alphabet::Crockford, &self.token_bytes),
            fp = base32::encode(Alphabet::Crockford, &self.fingerprint_bytes),
        )
    }

    /// Inverse of [`Self::to_url`]. Surfaces a typed error per
    /// failure mode so the phone can render a precise user-facing
    /// message ("token field is malformed" vs "URL host missing").
    pub fn parse(url: &str) -> Result<Self, PairingError> {
        let rest = url
            .strip_prefix(PAIRING_SCHEME)
            .ok_or_else(|| PairingError::BadScheme(url.to_string()))?;

        let (host_port, query) = rest.split_once('?').ok_or(PairingError::MissingHostPort)?;
        let (host, port) = host_port
            .rsplit_once(':')
            .ok_or(PairingError::MissingHostPort)?;
        let port = port
            .parse::<u16>()
            .map_err(|_| PairingError::MissingHostPort)?;

        let mut token_b32: Option<&str> = None;
        let mut fingerprint_b32: Option<&str> = None;
        for kv in query.split('&') {
            let Some((k, v)) = kv.split_once('=') else {
                continue;
            };
            match k {
                "token" => token_b32 = Some(v),
                "fingerprint" => fingerprint_b32 = Some(v),
                _ => {}
            }
        }
        let token_b32 = token_b32.ok_or(PairingError::MissingQueryParam("token"))?;
        let fingerprint_b32 =
            fingerprint_b32.ok_or(PairingError::MissingQueryParam("fingerprint"))?;

        let token_vec = base32::decode(Alphabet::Crockford, token_b32)
            .ok_or(PairingError::BadBase32 { field: "token" })?;
        let fp_vec = base32::decode(Alphabet::Crockford, fingerprint_b32).ok_or(
            PairingError::BadBase32 {
                field: "fingerprint",
            },
        )?;

        let token_bytes: [u8; PAIRING_TOKEN_BYTES] =
            token_vec
                .try_into()
                .map_err(|v: Vec<u8>| PairingError::BadFieldLength {
                    field: "token",
                    actual: v.len(),
                    expected: PAIRING_TOKEN_BYTES,
                })?;
        let fingerprint_bytes: [u8; PAIRING_FINGERPRINT_BYTES] =
            fp_vec
                .try_into()
                .map_err(|v: Vec<u8>| PairingError::BadFieldLength {
                    field: "fingerprint",
                    actual: v.len(),
                    expected: PAIRING_FINGERPRINT_BYTES,
                })?;

        Ok(Self {
            host: host.to_string(),
            port,
            token_bytes,
            fingerprint_bytes,
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

/// Derive the four-emoji SAS fingerprint from a 32-byte X25519
/// shared secret. Both sides feed the SAME shared secret in and get
/// the SAME four emojis out — the user reads them off both screens
/// and taps "match" to commit the pairing.
pub fn sas_fingerprint_from_shared_secret(shared: &[u8; 32]) -> SasFingerprint {
    let digest = sha256_of(shared);
    SasFingerprint([digest[0], digest[1], digest[2], digest[3]])
}

/// Helper used by the smoke test + the server: from a desktop
/// long-term `StaticSecret` and the phone's public key, return the
/// shared secret + the SAS fingerprint derived from it.
pub fn shared_secret_from_token(
    desktop_secret: &StaticSecret,
    phone_public: &PublicKey,
) -> ([u8; 32], SasFingerprint) {
    let shared = desktop_secret.diffie_hellman(phone_public);
    let bytes = *shared.as_bytes();
    let sas = sas_fingerprint_from_shared_secret(&bytes);
    (bytes, sas)
}

/// Encode a pairing URL as a QR PNG. Returns the raw PNG bytes the
/// Tauri panel can hand to the Svelte side as an
/// `<img src="data:image/png;base64,…">` (the base64 step happens in
/// the IPC bridge, not here).
///
/// Implemented in pure Rust via `qrcodegen` so the desktop ships
/// without `libqrencode` or `imagemagick`. The image is rendered as
/// a 1-bit bitmap → PNG-encoded by hand: 4 bytes per pixel for the
/// raw IDAT (RGBA white / black), framed in the minimal PNG
/// chunks (`IHDR`, `IDAT`, `IEND`). Saves a `png` crate dep for a
/// tiny payload.
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

/// Persistent record stored in `MobileSettings` after a successful
/// pairing. Includes the device's stable X25519 public key + the
/// human-typed device label.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PairingRecord {
    /// User-supplied device label. Defaults to a kebab-case form of
    /// the device's hostname when the phone announces it during the
    /// pairing handshake.
    pub label: String,
    /// 32-byte X25519 long-term public key the phone announced
    /// during the handshake. The desktop never sees the matching
    /// secret.
    pub phone_public_key: [u8; 32],
    /// Unix-epoch seconds when the pairing was committed.
    pub paired_at: i64,
    /// Optional APNs / FCM token the phone hands to the desktop so
    /// the runner can dispatch push notifications. Stored as the
    /// raw provider string; signing the JWT is the runner's job.
    pub push_target: Option<crate::notify::PushTarget>,
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn sha256_of(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// Minimal RGBA → PNG encoder. Public domain implementation in the
/// style of `lodepng-tiny`; chosen over the `png` crate to keep the
/// dep tree small for a feature this narrow.
fn encode_png_rgba(rgba: &[u8], dim: u32) -> Vec<u8> {
    use std::io::Write;

    fn crc32(data: &[u8]) -> u32 {
        // Pure-Rust IEEE 802.3 CRC32. Cheaper than pulling another
        // crate in for a one-off PNG payload.
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

    // Filter every scanline with the `0` (None) filter byte, then
    // wrap the lot in a single uncompressed zlib block. Output is
    // larger than `flate2` would produce, but the QR PNG is small
    // (a few kilobytes) and we save a dep.
    let stride = dim as usize * 4;
    let mut filtered = Vec::with_capacity(rgba.len() + dim as usize);
    for row in rgba.chunks(stride) {
        filtered.push(0);
        filtered.extend_from_slice(row);
    }
    // zlib stream: 0x78 0x01 (default compression, no preset dict)
    // followed by stored deflate blocks.
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
    ihdr.push(8); // bit depth
    ihdr.push(6); // color type RGBA
    ihdr.push(0); // compression
    ihdr.push(0); // filter
    ihdr.push(0); // interlace
    write_chunk(&mut out, b"IHDR", &ihdr);

    write_chunk(&mut out, b"IDAT", &zlib);
    write_chunk(&mut out, b"IEND", &[]);

    let _ = out.flush();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_compat::Rng;

    mod rand_compat {
        // Helper trait so the test stays sync without bringing
        // `rand` into the dep tree.
        pub trait Rng {
            fn fill(&mut self, b: &mut [u8]);
        }

        pub struct OsRng;
        impl Rng for OsRng {
            fn fill(&mut self, b: &mut [u8]) {
                getrandom::fill(b).expect("getrandom");
            }
        }
    }

    fn make_pubkey() -> PublicKey {
        let mut secret_bytes = [0u8; 32];
        rand_compat::OsRng.fill(&mut secret_bytes);
        let secret = StaticSecret::from(secret_bytes);
        PublicKey::from(&secret)
    }

    #[test]
    fn token_round_trips_through_url() {
        let pk = make_pubkey();
        let token = PairingToken::new("192.168.1.50", 39512, &pk).expect("mint");
        let url = token.to_url();
        let parsed = PairingToken::parse(&url).expect("parse");
        assert_eq!(parsed, token);
    }

    #[test]
    fn parse_rejects_wrong_scheme() {
        let err = PairingToken::parse("https://nope.example/").unwrap_err();
        assert!(matches!(err, PairingError::BadScheme(_)), "{err:?}");
    }

    #[test]
    fn parse_rejects_missing_token() {
        let err = PairingToken::parse(
            "cthat-pair://h:1?fingerprint=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        )
        .unwrap_err();
        assert!(
            matches!(err, PairingError::MissingQueryParam("token")),
            "{err:?}"
        );
    }

    #[test]
    fn sas_fingerprint_is_deterministic() {
        let shared = [0xBE; 32];
        let a = sas_fingerprint_from_shared_secret(&shared);
        let b = sas_fingerprint_from_shared_secret(&shared);
        assert_eq!(a, b);
        assert_eq!(a.0.len(), SAS_EMOJI_SLOTS);
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
    fn shared_secret_matches_on_both_sides() {
        let mut alice_bytes = [0u8; 32];
        let mut bob_bytes = [0u8; 32];
        rand_compat::OsRng.fill(&mut alice_bytes);
        rand_compat::OsRng.fill(&mut bob_bytes);
        let alice_sec = StaticSecret::from(alice_bytes);
        let bob_sec = StaticSecret::from(bob_bytes);
        let alice_pub = PublicKey::from(&alice_sec);
        let bob_pub = PublicKey::from(&bob_sec);
        let (a_secret, a_sas) = shared_secret_from_token(&alice_sec, &bob_pub);
        let (b_secret, b_sas) = shared_secret_from_token(&bob_sec, &alice_pub);
        assert_eq!(a_secret, b_secret);
        assert_eq!(a_sas, b_sas);
    }

    #[test]
    fn qr_png_is_a_valid_png_signature() {
        let pk = make_pubkey();
        let token = PairingToken::new("desktop.local", 55555, &pk).expect("mint");
        let png = generate_qr_png(&token.to_url(), 4).expect("qr");
        assert!(png.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
        // IDAT chunk must exist somewhere in the byte stream.
        assert!(png.windows(4).any(|w| w == b"IDAT"));
    }
}
