//! Phase 42 — per-message authentication for the Phase-40 broker
//! pipe `\\.\pipe\copythat-ui-enqueue-v1`.
//!
//! Without this layer, any process running as the same user could
//! write `{argv: [...]}` JSON to the pipe and bypass the UI's
//! consent gates, queueing arbitrary copy/move jobs. The DACL on
//! the pipe is user-scoped, but "same user" includes any other
//! process that user is running — browser, mail client, npm
//! script — so a single compromised tool could weaponise the
//! broker.
//!
//! The mitigation is HMAC-SHA256 over the JSON payload using a
//! 32-byte secret generated at first-instance startup. The secret
//! is held in process memory **and** written to a session-local
//! file under `%LOCALAPPDATA%\CopyThat\session.token`. A second
//! invocation reads the token, computes the HMAC, includes it in
//! the wire payload, and the server verifies before parsing argv.
//!
//! Why a file at all? The second instance is a **separate
//! process** — it can't share the first instance's heap. The token
//! file is the trust anchor every same-user copythat-ui binary
//! agrees on.
//!
//! Why is "any same-user process can read the file" still
//! acceptable? Because that's the same threat model as the pipe
//! itself: a same-user attacker can already read the pipe's DACL.
//! The point of the HMAC is to prevent a process that is **not**
//! a copythat-ui binary from forging messages: a hostile process
//! has to (a) know the file location and (b) read it before
//! sending. Both raise the bar from "open the well-known pipe
//! name and dump JSON" to "deliberately steal a per-session
//! secret", which crosses the line from drive-by exploitation to
//! credential theft and surfaces in EDR logs.
//!
//! See `RESEARCH/RESEARCH_PHASE_42.md` for the threat-model
//! rationale.

#![cfg(windows)]

use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Filename under `%LOCALAPPDATA%\CopyThat\` that holds the raw 32
/// bytes of the per-session HMAC secret. Re-created at every
/// first-instance startup so a stale token from a crashed prior
/// session can't be replayed against this session's pipe.
const TOKEN_FILE: &str = "session.token";

/// In-process cache of the 32-byte secret. Set once at first-
/// instance startup (via [`init_first_instance_secret`]) on the
/// server side, or once at second-instance launch (via
/// [`load_secret_for_client`]) on the client side. Stored as raw
/// bytes so the value never serialises into stderr / panic
/// messages.
static SECRET: OnceLock<[u8; 32]> = OnceLock::new();

/// Resolve `%LOCALAPPDATA%\CopyThat\session.token`. We deliberately
/// use `BaseDirs::data_local_dir()` rather than `ProjectDirs` so
/// the file lives directly under `LocalAppData\CopyThat\` and is
/// trivially diff'able with the Phase-40 pipe + mutex naming.
fn token_path() -> Result<PathBuf, String> {
    let base = directories::BaseDirs::new().ok_or_else(|| "BaseDirs unavailable".to_string())?;
    let dir = base.data_local_dir().join("CopyThat");
    Ok(dir.join(TOKEN_FILE))
}

/// First-instance entry point. Generates a fresh 32-byte secret,
/// caches it in [`SECRET`], and writes it to `session.token`. The
/// parent directory `%LOCALAPPDATA%\CopyThat\` is already
/// user-scoped by Windows convention — no extra ACL plumbing is
/// required for the same-user threat model. The file is created
/// with `create(true).truncate(true)` so a stale token from a
/// crashed prior session is overwritten.
///
/// Returns the 32-byte secret on success so the caller can pass
/// it to [`hmac_hex`] without a second `OnceLock` read.
pub fn init_first_instance_secret() -> Result<[u8; 32], String> {
    if let Some(s) = SECRET.get() {
        return Ok(*s);
    }
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).map_err(|e| format!("getrandom failed: {e}"))?;

    let path = token_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create_dir_all session-token parent: {e}"))?;
    }
    // truncate(true) so a leftover token from a crashed prior
    // session is replaced with this session's secret.
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)
        .map_err(|e| format!("open {}: {e}", path.display()))?;
    f.write_all(&bytes)
        .map_err(|e| format!("write session token: {e}"))?;
    f.flush().map_err(|e| format!("flush session token: {e}"))?;
    drop(f);

    let _ = SECRET.set(bytes);
    Ok(bytes)
}

/// Second-instance entry point. Reads the on-disk token written
/// by the first instance. Caches in [`SECRET`] for the rest of
/// the second-instance process lifetime (which is short — we exit
/// right after forwarding argv).
///
/// Returns `Err` if the file is missing, short, or unreadable.
/// The caller treats that as "no first instance" and falls
/// through to the normal boot path.
pub fn load_secret_for_client() -> Result<[u8; 32], String> {
    if let Some(s) = SECRET.get() {
        return Ok(*s);
    }
    let path = token_path()?;
    let bytes = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if bytes.len() != 32 {
        return Err(format!(
            "session token wrong length: got {} bytes, want 32",
            bytes.len()
        ));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    let _ = SECRET.set(out);
    Ok(out)
}

/// Compute the lowercase-hex HMAC-SHA256 of `payload` under
/// `secret`. Used by the client to attach an `hmac` field and by
/// the server to recompute the expected value.
pub fn hmac_hex(secret: &[u8; 32], payload: &[u8]) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret).expect("hmac key length is fixed at 32");
    mac.update(payload);
    let bytes = mac.finalize().into_bytes();
    hex::encode(bytes)
}

/// Constant-time verify that `tag_hex` is the lowercase-hex HMAC
/// of `payload` under `secret`. We let `Mac::verify_slice` do the
/// timing-safe comparison after decoding the hex.
pub fn verify_hmac(secret: &[u8; 32], payload: &[u8], tag_hex: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;
    let Ok(tag) = hex::decode(tag_hex) else {
        return false;
    };
    let mut mac = match HmacSha256::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(payload);
    mac.verify_slice(&tag).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip a payload through `hmac_hex` + `verify_hmac` —
    /// the only invariant downstream code relies on.
    #[test]
    fn hmac_round_trip_verifies() {
        let secret = [7u8; 32];
        let payload = br#"{"argv":["copythat-ui.exe","--enqueue"]}"#;
        let tag = hmac_hex(&secret, payload);
        assert!(verify_hmac(&secret, payload, &tag));
    }

    /// A flipped bit in the payload must invalidate the HMAC.
    #[test]
    fn hmac_rejects_tampered_payload() {
        let secret = [7u8; 32];
        let payload = br#"{"argv":["copythat-ui.exe","--enqueue"]}"#;
        let tag = hmac_hex(&secret, payload);
        let mut tampered = payload.to_vec();
        tampered[0] ^= 0x01;
        assert!(!verify_hmac(&secret, &tampered, &tag));
    }

    /// A different secret must fail to verify a tag computed with
    /// the original secret.
    #[test]
    fn hmac_rejects_wrong_secret() {
        let secret_a = [7u8; 32];
        let secret_b = [9u8; 32];
        let payload = br#"{"argv":[]}"#;
        let tag = hmac_hex(&secret_a, payload);
        assert!(!verify_hmac(&secret_b, payload, &tag));
    }

    /// Malformed hex must reject cleanly without panicking.
    #[test]
    fn hmac_rejects_garbage_hex() {
        let secret = [0u8; 32];
        let payload = b"";
        assert!(!verify_hmac(&secret, payload, "not-hex"));
        assert!(!verify_hmac(&secret, payload, ""));
    }
}
