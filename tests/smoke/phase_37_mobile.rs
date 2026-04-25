//! Phase 37 smoke — desktop-side mobile companion (PeerJS edition).
//!
//! Six cases covering the protocol primitives + the wire vocabulary:
//!
//! 1. Pairing-token URL round-trips through `to_url` + `parse`
//!    without losing a byte.
//! 2. SAS fingerprint is deterministic + sensitive to either
//!    pubkey changing (so an attacker who substitutes one of the
//!    keys produces different emojis on at least one screen).
//! 3. QR encoder produces a valid PNG signature + an `IDAT` chunk
//!    given a `cthat-pair://<peer-id>?sas=…` URL.
//! 4. `mint_peer_id` yields distinct ids on consecutive calls.
//! 5. `RemoteCommand` + `RemoteResponse` round-trip through serde
//!    for every variant the data channel currently carries.
//! 6. All Phase 37 Fluent keys present in every one of the 18
//!    locales.

use std::fs;
use std::path::PathBuf;

use copythat_mobile::pairing::{
    PAIRING_SAS_SEED_BYTES, PairingToken, SasFingerprint, sas_fingerprint, sas_fingerprint_to_emoji,
};
use copythat_mobile::server::{
    CollisionAction, HistoryRow, JobSummary, RemoteCommand, RemoteResponse,
};
use copythat_mobile::{generate_qr_png, mint_peer_id};

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
    let token = PairingToken::new("DESKTOP-PEER-XYZ", [0xAB; 32]).expect("mint");
    let url = token.to_url();
    let parsed = PairingToken::parse(&url).expect("parse");
    assert_eq!(parsed.peer_id, token.peer_id);
    assert_eq!(parsed.sas_seed.len(), PAIRING_SAS_SEED_BYTES);
    assert_eq!(parsed, token);
    assert!(url.starts_with("cthat-pair://"));
    assert!(url.contains("?sas="));
}

#[test]
fn case02_sas_changes_when_either_pubkey_changes() {
    let seed = [0xAA; 32];
    let desktop_pub = [1u8; 32];
    let phone_pub = [2u8; 32];

    let baseline = sas_fingerprint(&seed, &desktop_pub, &phone_pub);
    // Re-deriving with the same inputs must produce the same SAS —
    // this is the property the user relies on when they read both
    // screens.
    let again = sas_fingerprint(&seed, &desktop_pub, &phone_pub);
    assert_eq!(baseline, again);

    // Swapping either pubkey must change the SAS so an attacker
    // who substitutes one of them stands out visually.
    let mut tampered_phone = phone_pub;
    tampered_phone[0] ^= 0xFF;
    let with_tampered_phone = sas_fingerprint(&seed, &desktop_pub, &tampered_phone);
    assert_ne!(baseline, with_tampered_phone);

    let mut tampered_desktop = desktop_pub;
    tampered_desktop[0] ^= 0xFF;
    let with_tampered_desktop = sas_fingerprint(&seed, &tampered_desktop, &phone_pub);
    assert_ne!(baseline, with_tampered_desktop);

    let glyphs = sas_fingerprint_to_emoji(&baseline);
    assert_eq!(glyphs.len(), 4);
}

#[test]
fn case03_qr_encoder_emits_valid_png() {
    let token = PairingToken::new("phone-connect-target", [0xCD; 32]).expect("mint");
    let png = generate_qr_png(&token.to_url(), 4).expect("qr");
    assert!(
        png.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
        "missing PNG signature"
    );
    assert!(png.windows(4).any(|w| w == b"IDAT"), "missing IDAT chunk");
    assert!(png.windows(4).any(|w| w == b"IEND"), "missing IEND chunk");
}

#[test]
fn case04_mint_peer_id_yields_distinct_ids() {
    let a = mint_peer_id().expect("a");
    let b = mint_peer_id().expect("b");
    assert_ne!(a, b);
    assert!(!a.is_empty());
    // Crockford-base32 of 20 bytes is 32 chars, no padding.
    assert_eq!(a.len(), 32);
}

#[test]
fn case05_remote_command_response_round_trip() {
    // Spot-check one variant per command shape so a serde rename
    // breaks the test, plus the streaming GlobalsTick that the PWA
    // home screen subscribes to for live stats.
    let cases: Vec<RemoteCommand> = vec![
        RemoteCommand::Hello {
            phone_pubkey_hex: "ab".repeat(32),
            device_label: "Mike's iPhone".into(),
        },
        RemoteCommand::ListJobs,
        RemoteCommand::PauseJob {
            job_id: "j1".into(),
        },
        RemoteCommand::ResumeJob {
            job_id: "j1".into(),
        },
        RemoteCommand::CancelJob {
            job_id: "j1".into(),
        },
        RemoteCommand::ResolveCollision {
            prompt_id: "p1".into(),
            action: CollisionAction::OverwriteAll,
        },
        RemoteCommand::Globals,
        RemoteCommand::RecentHistory { limit: 25 },
        RemoteCommand::RerunHistory { row_id: 42 },
        RemoteCommand::SecureDelete {
            paths: vec!["C:/secret.bin".into()],
            method: "dod3".into(),
        },
        RemoteCommand::StartCopy {
            sources: vec!["C:/src".into()],
            destination: "D:/dst".into(),
            verify: Some("blake3".into()),
        },
        RemoteCommand::Goodbye,
        RemoteCommand::SetKeepAwake { enabled: true },
        RemoteCommand::GetLocale,
    ];
    for cmd in cases {
        let s = serde_json::to_string(&cmd).unwrap();
        let back: RemoteCommand = serde_json::from_str(&s).unwrap();
        assert_eq!(cmd, back, "command round-trip failed: {s}");
    }

    let resp_cases: Vec<RemoteResponse> = vec![
        RemoteResponse::HelloAck { paired: true },
        RemoteResponse::Jobs {
            jobs: vec![JobSummary {
                job_id: "j1".into(),
                kind: "copy".into(),
                state: "running".into(),
                src: "C:/src".into(),
                dst: "D:/dst".into(),
                bytes_done: 1024,
                bytes_total: 4096,
                rate_bps: 800_000,
            }],
        },
        RemoteResponse::History {
            rows: vec![HistoryRow {
                row_id: 7,
                kind: "copy".into(),
                status: "succeeded".into(),
                started_at_ms: 1_700_000_000_000,
                finished_at_ms: Some(1_700_000_010_000),
                src_root: "C:/src".into(),
                dst_root: "D:/dst".into(),
                total_bytes: 1_048_576,
                files_ok: 12,
                files_failed: 0,
            }],
        },
        RemoteResponse::Ok,
        RemoteResponse::Error {
            message: "boom".into(),
        },
        RemoteResponse::JobProgress {
            job_id: "j1".into(),
            bytes_done: 100,
            bytes_total: 200,
            rate_bps: 5_000_000,
        },
        RemoteResponse::JobCompleted {
            job_id: "j1".into(),
            bytes: 4_096,
        },
        RemoteResponse::JobFailed {
            job_id: "j1".into(),
            reason: "permission denied".into(),
        },
        RemoteResponse::GlobalsTick {
            bytes_done: 1_000_000,
            bytes_total: 5_000_000,
            files_done: 5,
            files_total: 25,
            rate_bps: 12_000_000,
            copy_files: 3,
            move_files: 1,
            secure_delete_files: 1,
        },
        RemoteResponse::FileTick {
            job_id: "j1".into(),
            action: "copying".into(),
            src: "C:/src/big.iso".into(),
            dst: "D:/dst/big.iso".into(),
            bytes_done: 8_388_608,
            bytes_total: 16_777_216,
        },
        RemoteResponse::JobLoading {
            job_id: "j1".into(),
            message: "Scanning source tree…".into(),
        },
        RemoteResponse::JobReady {
            job_id: "j1".into(),
        },
        RemoteResponse::JobStateChanged {
            job_id: "j1".into(),
            state: "paused".into(),
        },
        RemoteResponse::ServerShuttingDown {
            reason: "user closed the desktop window".into(),
        },
        RemoteResponse::Locale { bcp47: "fr".into() },
    ];
    for resp in resp_cases {
        let s = serde_json::to_string(&resp).unwrap();
        let back: RemoteResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back, "response round-trip failed: {s}");
    }

    let _ = SasFingerprint([0; 4]);
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
