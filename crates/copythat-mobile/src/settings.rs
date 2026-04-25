//! Runtime mobile-companion settings shape.
//!
//! Mirrored on disk by `copythat-settings::MobileSettings` (stringly
//! typed so the settings crate stays free of axum / rustls /
//! reqwest). The Tauri runner converts via
//! [`crate::settings_bridge`].

use serde::{Deserialize, Serialize};

use crate::pairing::PairingRecord;

/// Top-level mobile settings. Off by default — a fresh install
/// ships with `pair_enabled = false` and the runner skips
/// registering the desktop peer-id with PeerJS until the user
/// flips the toggle on in Settings → Mobile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MobileSettings {
    /// Master toggle for new-device enrolment. When `true`, the
    /// Settings → Mobile panel shows the pairing QR + accepts new
    /// pairing handshakes. Off by default so a fresh install
    /// ships with no possibility of unintended pairings.
    pub pair_enabled: bool,
    /// "Always connect to mobile app" — when `true`, the runner
    /// registers the persisted `desktop_peer_id` with the PeerJS
    /// broker every time Copy That launches, so already-paired
    /// phones can connect anytime the desktop is running.
    ///
    /// **Auto-connect requires at least one paired device.** If
    /// `auto_connect = true` but `pairings` is empty, the runner
    /// surfaces the first-launch onboarding flow (install QR +
    /// "Pair a phone first" prompt) instead of registering with
    /// the broker — there's no point announcing a peer-id that
    /// nothing on the LAN is going to dial. Flipping the toggle
    /// on with no pairings doesn't auto-register; the desktop
    /// shows the callout, the user installs the PWA + completes
    /// the pairing handshake, and from then on every launch
    /// auto-connects.
    pub auto_connect: bool,
    /// PeerJS broker URL. Empty string means the public default
    /// (`0.peerjs.com`); production deployments override with a
    /// self-hosted broker.
    pub peerjs_broker: String,
    /// Stable PeerJS peer-id the desktop registers under.
    /// Persisted across launches so already-paired phones can
    /// reconnect without re-pairing.
    pub desktop_peer_id: String,
    /// Persisted records of every device that has completed
    /// pairing.
    pub pairings: Vec<PairingRecord>,
}

impl MobileSettings {
    /// Look up a previously-paired device by its public key.
    pub fn find_by_pubkey(&self, key: &[u8; 32]) -> Option<&PairingRecord> {
        self.pairings.iter().find(|p| &p.phone_public_key == key)
    }

    /// Drop a pairing record. No-op when the key isn't present.
    pub fn revoke(&mut self, key: &[u8; 32]) {
        self.pairings.retain(|p| &p.phone_public_key != key);
    }
}

/// Validate a PeerJS broker URL/host before the PWA's PeerJS
/// adapter dials it. Empty = use the upstream default. Otherwise
/// require a bare hostname or `https://`-only URL — refuse
/// `http://`, custom schemes, embedded paths/queries, and any
/// shape that could let a settings-import payload route the
/// signaling channel through an attacker-controlled server.
///
/// Returns `Ok(())` when the broker is acceptable, `Err(reason)`
/// otherwise.
pub fn validate_peerjs_broker(raw: &str) -> Result<(), String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    // Reject obvious URL-injection shapes: control chars, embedded
    // whitespace, path/query/fragment characters, user-info `@`,
    // backslash, embedded NUL.
    if trimmed
        .chars()
        .any(|c| c.is_control() || c.is_whitespace() || matches!(c, '@' | '\\' | '?' | '#' | '<' | '>' | '"' | '\''))
    {
        return Err("peerjs_broker contains forbidden characters".into());
    }
    // Two acceptable shapes: a bare hostname (no scheme) or an
    // explicit `https://` URL whose authority matches a hostname.
    if let Some(rest) = trimmed.strip_prefix("https://") {
        let host = rest.split('/').next().unwrap_or("");
        if host.is_empty() || host.contains(':') && !host.split(':').nth(1).map(|p| p.chars().all(|c| c.is_ascii_digit())).unwrap_or(false) {
            return Err("peerjs_broker https URL has malformed authority".into());
        }
        Ok(())
    } else if trimmed.contains("://") {
        Err("peerjs_broker scheme must be https:// (got something else)".into())
    } else {
        // Bare hostname: must look like a hostname (alnum, hyphen,
        // dot, optional `:port`). Reject `..` to avoid relative
        // paths slipping through.
        if trimmed.contains("..") {
            return Err("peerjs_broker hostname cannot contain `..`".into());
        }
        for c in trimmed.chars() {
            if !(c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':')) {
                return Err(format!(
                    "peerjs_broker contains illegal character {c:?}"
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_toml() {
        let s = MobileSettings {
            pair_enabled: true,
            auto_connect: true,
            peerjs_broker: "0.peerjs.com".into(),
            desktop_peer_id: "DESKTOP-PEER-12345".into(),
            pairings: vec![PairingRecord {
                label: "Mike's iPhone".into(),
                phone_public_key: [7u8; 32],
                paired_at: 1_700_000_000,
                push_target: None,
            }],
        };
        let toml = toml::to_string(&s).expect("ser");
        let back: MobileSettings = toml::from_str(&toml).expect("de");
        assert_eq!(s, back);
    }

    #[test]
    fn revoke_drops_matching_key() {
        let mut s = MobileSettings {
            pairings: vec![
                PairingRecord {
                    label: "Alice".into(),
                    phone_public_key: [1u8; 32],
                    paired_at: 1,
                    push_target: None,
                },
                PairingRecord {
                    label: "Bob".into(),
                    phone_public_key: [2u8; 32],
                    paired_at: 2,
                    push_target: None,
                },
            ],
            ..MobileSettings::default()
        };
        s.revoke(&[1u8; 32]);
        assert_eq!(s.pairings.len(), 1);
        assert_eq!(s.pairings[0].label, "Bob");
    }
}
