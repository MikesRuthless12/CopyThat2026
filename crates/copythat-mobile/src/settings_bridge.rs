//! Bridge between `copythat-settings::MobileSettings` (the
//! persistence shape, stringly-typed) and the runtime
//! [`crate::MobileSettings`] / [`crate::PairingRecord`] /
//! [`crate::PushTarget`] shape.

use copythat_settings::{MobilePairingEntry, MobilePushTarget};

use crate::notify::PushTarget;
use crate::pairing::PairingRecord;
use crate::settings::MobileSettings;

/// Convert a persisted [`copythat_settings::MobileSettings`] into the
/// runtime [`MobileSettings`].
pub fn from_persisted(persisted: &copythat_settings::MobileSettings) -> MobileSettings {
    MobileSettings {
        pair_enabled: persisted.pair_enabled,
        auto_connect: persisted.auto_connect,
        peerjs_broker: persisted.peerjs_broker.clone(),
        desktop_peer_id: persisted.desktop_peer_id.clone(),
        pairings: persisted
            .pairings
            .iter()
            .map(pairing_from_persisted)
            .collect(),
    }
}

/// Inverse of [`from_persisted`]. Provider credential strings are
/// preserved verbatim because the runtime shape doesn't carry them
/// (they live only in the persisted shape today; the Phase 37
/// follow-up will move them to the OS keychain).
pub fn to_persisted(
    runtime: &MobileSettings,
    apns_p8_pem: String,
    apns_team_id: String,
    apns_key_id: String,
    fcm_service_account_json: String,
) -> copythat_settings::MobileSettings {
    copythat_settings::MobileSettings {
        pair_enabled: runtime.pair_enabled,
        auto_connect: runtime.auto_connect,
        peerjs_broker: runtime.peerjs_broker.clone(),
        desktop_peer_id: runtime.desktop_peer_id.clone(),
        pairings: runtime.pairings.iter().map(pairing_to_persisted).collect(),
        apns_p8_pem,
        apns_team_id,
        apns_key_id,
        fcm_service_account_json,
    }
}

fn pairing_from_persisted(entry: &MobilePairingEntry) -> PairingRecord {
    PairingRecord {
        label: entry.label.clone(),
        phone_public_key: hex_decode_32(&entry.phone_public_key_hex).unwrap_or([0u8; 32]),
        paired_at: entry.paired_at,
        push_target: entry.push_target.as_ref().map(push_target_from_persisted),
    }
}

fn pairing_to_persisted(record: &PairingRecord) -> MobilePairingEntry {
    MobilePairingEntry {
        label: record.label.clone(),
        phone_public_key_hex: hex_encode_32(&record.phone_public_key),
        paired_at: record.paired_at,
        push_target: record.push_target.as_ref().map(push_target_to_persisted),
    }
}

fn push_target_from_persisted(t: &MobilePushTarget) -> PushTarget {
    match t {
        MobilePushTarget::Apns { token } => PushTarget::Apns {
            token: token.clone(),
        },
        MobilePushTarget::Fcm { token } => PushTarget::Fcm {
            token: token.clone(),
        },
        MobilePushTarget::StubEndpoint { url } => PushTarget::StubEndpoint { url: url.clone() },
    }
}

fn push_target_to_persisted(t: &PushTarget) -> MobilePushTarget {
    match t {
        PushTarget::Apns { token } => MobilePushTarget::Apns {
            token: token.clone(),
        },
        PushTarget::Fcm { token } => MobilePushTarget::Fcm {
            token: token.clone(),
        },
        PushTarget::StubEndpoint { url } => MobilePushTarget::StubEndpoint { url: url.clone() },
    }
}

fn hex_encode_32(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for x in b {
        use std::fmt::Write;
        let _ = write!(&mut s, "{x:02x}");
    }
    s
}

fn hex_decode_32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_through_persisted_shape() {
        let runtime = MobileSettings {
            pair_enabled: true,
            auto_connect: true,
            peerjs_broker: "0.peerjs.com".into(),
            desktop_peer_id: "DESKTOP-PEER-12345".into(),
            pairings: vec![PairingRecord {
                label: "Mike's iPhone".into(),
                phone_public_key: [7u8; 32],
                paired_at: 1_700_000_000,
                push_target: Some(PushTarget::Fcm {
                    token: "fcm-token-here".into(),
                }),
            }],
        };
        let persisted = to_persisted(
            &runtime,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
        let back = from_persisted(&persisted);
        assert_eq!(back, runtime);
    }
}
