//! Settings types persisted into the `copythat-settings` TOML root.
//!
//! `copythat-settings::MobileSettings` re-exports this struct so the
//! Tauri shell can round-trip it through the same `Settings` blob
//! every other phase already uses. Keeping the shape here means the
//! mobile crate owns its own schema; the settings crate carries the
//! pointer.

use serde::{Deserialize, Serialize};

use crate::pairing::PairingRecord;

/// Top-level mobile settings. Off by default — a fresh install
/// ships with no pair-server listening until the user opens
/// Settings → Mobile and explicitly toggles pairing on.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MobileSettings {
    /// Master toggle. While `true`, the runner can spin up the
    /// pair-server on demand from the Settings panel.
    pub pair_enabled: bool,
    /// Bind port. `0` (the default) lets the OS pick a free
    /// ephemeral port at server start.
    pub bind_port: u16,
    /// Persisted records of every device that has completed
    /// pairing. The user can revoke entries individually from the
    /// Settings panel.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_toml() {
        let s = MobileSettings {
            pair_enabled: true,
            bind_port: 0,
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
