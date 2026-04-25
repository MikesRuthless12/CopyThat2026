//! Phase 37 smoke — desktop-side mobile companion foundation.
//!
//! Five cases covering the protocol primitives + the in-process
//! HTTP pairing handshake:
//!
//! 1. Pairing-token URL round-trips through `to_url` + `parse`
//!    without losing a byte.
//! 2. SAS fingerprint is deterministic + identical on both sides
//!    of an X25519 ECDH exchange (the human reads the same 4
//!    emojis on phone and desktop).
//! 3. QR encoder produces a valid PNG signature + an `IDAT` chunk.
//! 4. End-to-end pairing handshake against a live `PairServer`:
//!    the smoke test plays the phone, sends `/pair/begin` then
//!    `/pair/complete`, asserts the desktop committed a
//!    `PairingRecord` matching the announced label + push target.
//! 5. All 15 Phase 37 Fluent keys present in every one of the 18
//!    locales.

use std::fs;
use std::path::PathBuf;

use copythat_mobile::pairing::{
    PAIRING_FINGERPRINT_BYTES, PAIRING_TOKEN_BYTES, PairingToken, SasFingerprint,
    sas_fingerprint_from_shared_secret, shared_secret_from_token,
};
use copythat_mobile::{PairServer, PushTarget, generate_qr_png};
use serde_json::json;
use x25519_dalek::{PublicKey, StaticSecret};

const PHASE_37_KEYS: &[&str] = &[
    "settings-mobile-heading",
    "settings-mobile-hint",
    "settings-mobile-pair-toggle",
    "settings-mobile-pair-active",
    "settings-mobile-pair-button",
    "settings-mobile-revoke-button",
    "settings-mobile-no-pairings",
    "pair-sas-prompt",
    "pair-sas-confirm",
    "pair-sas-reject",
    "pair-toast-success",
    "pair-toast-failed",
    "push-toast-sent",
    "push-toast-failed",
    "settings-mobile-pair-port",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case01_token_url_round_trips() {
    let mut secret_bytes = [0u8; 32];
    getrandom::fill(&mut secret_bytes).unwrap();
    let secret = StaticSecret::from(secret_bytes);
    let pubkey = PublicKey::from(&secret);

    let token = PairingToken::new("desktop.lan", 51234, &pubkey).expect("mint");
    let url = token.to_url();
    assert!(url.starts_with("cthat-pair://"));
    let parsed = PairingToken::parse(&url).expect("parse");
    assert_eq!(parsed.host, token.host);
    assert_eq!(parsed.port, token.port);
    assert_eq!(parsed.token_bytes.len(), PAIRING_TOKEN_BYTES);
    assert_eq!(parsed.fingerprint_bytes.len(), PAIRING_FINGERPRINT_BYTES);
    assert_eq!(parsed, token);
}

#[test]
fn case02_sas_matches_on_both_sides_of_dh() {
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    getrandom::fill(&mut a).unwrap();
    getrandom::fill(&mut b).unwrap();
    let alice = StaticSecret::from(a);
    let bob = StaticSecret::from(b);
    let alice_pub = PublicKey::from(&alice);
    let bob_pub = PublicKey::from(&bob);

    let (alice_secret, alice_sas) = shared_secret_from_token(&alice, &bob_pub);
    let (bob_secret, bob_sas) = shared_secret_from_token(&bob, &alice_pub);

    assert_eq!(alice_secret, bob_secret, "shared secret diverged");
    assert_eq!(alice_sas, bob_sas, "SAS fingerprint diverged");

    let direct = sas_fingerprint_from_shared_secret(&alice_secret);
    assert_eq!(direct, alice_sas);

    let glyphs = alice_sas.as_emoji_string();
    assert_eq!(
        glyphs.split_whitespace().count(),
        4,
        "expected 4 SAS emojis, got `{glyphs}`"
    );
}

#[test]
fn case03_qr_encoder_emits_valid_png() {
    let secret = StaticSecret::from([0xAB; 32]);
    let pubkey = PublicKey::from(&secret);
    let token = PairingToken::new("phone.local", 9000, &pubkey).expect("mint");
    let png = generate_qr_png(&token.to_url(), 4).expect("qr");
    assert!(
        png.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
        "missing PNG signature"
    );
    assert!(png.windows(4).any(|w| w == b"IDAT"), "missing IDAT chunk");
    assert!(png.windows(4).any(|w| w == b"IEND"), "missing IEND chunk");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn case04_full_pairing_handshake_round_trips() {
    let (mut server, token) = PairServer::default().start().await.expect("start");
    let addr = server.addr();

    // Phone side: mint our own X25519 keypair.
    let mut phone_secret_bytes = [0u8; 32];
    getrandom::fill(&mut phone_secret_bytes).unwrap();
    let phone_secret = StaticSecret::from(phone_secret_bytes);
    let phone_public = PublicKey::from(&phone_secret);

    let token_b32 = base32::encode(base32::Alphabet::Crockford, &token.token_bytes);

    let client = reqwest::Client::new();
    let begin = client
        .post(format!("http://{addr}/pair/begin"))
        .json(&json!({
            "phone_pubkey": hex_encode(phone_public.as_bytes()),
            "device_label": "Mike's iPhone",
            "token": token_b32,
        }))
        .send()
        .await
        .expect("/pair/begin send");
    assert_eq!(begin.status(), 200);
    let begin_body: serde_json::Value = begin.json().await.expect("/pair/begin json");
    let desktop_pub_hex = begin_body["desktop_pubkey"]
        .as_str()
        .expect("desktop_pubkey");
    let desktop_pub_bytes = hex_decode(desktop_pub_hex).expect("hex decode");
    let desktop_pub: [u8; 32] = desktop_pub_bytes.try_into().expect("len 32");
    let desktop_pub = PublicKey::from(desktop_pub);

    // Server should now expose the SAS fingerprint to the desktop UI.
    let server_sas = server
        .pending_sas()
        .await
        .expect("server has a pending pair");
    let phone_shared = phone_secret.diffie_hellman(&desktop_pub);
    let phone_sas = sas_fingerprint_from_shared_secret(phone_shared.as_bytes());
    assert_eq!(server_sas, phone_sas, "SAS divergence end-to-end");

    // Phone side: the human matches the SAS, then completes the
    // handshake with an FCM token + "phone says paired".
    let push_target = PushTarget::Fcm {
        token: "fake-fcm-token".into(),
    };
    let complete = client
        .post(format!("http://{addr}/pair/complete"))
        .json(&json!({
            "token": token_b32,
            "push_target": push_target,
        }))
        .send()
        .await
        .expect("/pair/complete send");
    assert_eq!(complete.status(), 200);
    let complete_body: serde_json::Value = complete.json().await.expect("/pair/complete json");
    assert_eq!(complete_body["paired"], serde_json::Value::Bool(true));

    let record = server.committed().await.expect("commit landed");
    assert_eq!(record.label, "Mike's iPhone");
    assert_eq!(record.phone_public_key, *phone_public.as_bytes());
    assert!(record.push_target.is_some());

    server.shutdown().await.expect("shutdown");

    let _ = SasFingerprint([0; 4]); // ensure type re-exports compile
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn case05_unauthorized_token_is_rejected() {
    let (mut server, _token) = PairServer::default().start().await.expect("start");
    let addr = server.addr();

    let mut phone_secret_bytes = [0u8; 32];
    getrandom::fill(&mut phone_secret_bytes).unwrap();
    let phone_secret = StaticSecret::from(phone_secret_bytes);
    let phone_public = PublicKey::from(&phone_secret);

    let client = reqwest::Client::new();
    let begin = client
        .post(format!("http://{addr}/pair/begin"))
        .json(&json!({
            "phone_pubkey": hex_encode(phone_public.as_bytes()),
            "device_label": "Attacker",
            "token": base32::encode(base32::Alphabet::Crockford, &[0u8; 32]),
        }))
        .send()
        .await
        .expect("send");
    assert_eq!(begin.status(), 401, "wrong token must be rejected");

    server.shutdown().await.expect("shutdown");
}

#[test]
fn case06_phase_37_fluent_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_37_KEYS {
            let starts = content.starts_with(&format!("{key} ="));
            let inline = content.contains(&format!("\n{key} ="));
            assert!(
                starts || inline,
                "locale `{code}` missing key `{key}` at {}",
                path.display()
            );
        }
    }
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in 0..(s.len() / 2) {
        let pair = &s[i * 2..i * 2 + 2];
        out.push(u8::from_str_radix(pair, 16).ok()?);
    }
    Some(out)
}

fn locate_locales_dir() -> Option<PathBuf> {
    let mut cur = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let candidate = cur.join("locales");
        if candidate.join("en").join("copythat.ftl").exists() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}
