//! Phase 35 smoke — destination encryption + on-the-fly compression.
//!
//! Three brief-mandated cases plus i18n parity:
//!
//! 1. Compressible payload (16 MiB of `'A'`) copied through
//!    Always(level=3) — assert the destination is < 4 MiB and
//!    `CopyThatCryptHook::transform` reports a compression ratio
//!    < 0.25.
//! 2. Incompressible payload (16 MiB of pseudo-random bytes) named
//!    `noise.jpg` copied through Smart policy with the default
//!    deny list — assert compression is OFF for the file
//!    (output_bytes == input_bytes).
//! 3. 1 MiB plaintext copied through passphrase encryption — assert
//!    the destination is age-format and decryption with the same
//!    passphrase round-trips byte-for-byte.
//! 4. All Phase 35 Fluent keys present in every one of the 18
//!    locales.

use std::fs;
use std::io::Read;
use std::path::Path;

use copythat_core::TransformSink;
use copythat_crypt::{
    CompressionLevel, CompressionPolicy, CopyThatCryptHook, EncryptionPolicy, Identity,
    decrypted_reader,
};
use secrecy::SecretString;
use tempfile::tempdir;

const PHASE_35_KEYS: &[&str] = &[
    "settings-crypt-heading",
    "settings-crypt-hint",
    "settings-crypt-encryption-mode",
    "settings-crypt-encryption-off",
    "settings-crypt-encryption-passphrase",
    "settings-crypt-encryption-recipients",
    "settings-crypt-encryption-hint",
    "settings-crypt-recipients-file",
    "settings-crypt-recipients-file-placeholder",
    "settings-crypt-compression-mode",
    "settings-crypt-compression-off",
    "settings-crypt-compression-always",
    "settings-crypt-compression-smart",
    "settings-crypt-compression-hint",
    "settings-crypt-compression-level",
    "settings-crypt-compression-level-hint",
    "compress-footer-savings",
    "compress-savings-toast",
    "crypt-toast-recipients-loaded",
    "crypt-toast-recipients-error",
    "crypt-toast-passphrase-required",
    "crypt-toast-passphrase-set",
    "crypt-footer-encrypted-badge",
    "crypt-footer-compressed-badge",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[tokio::test]
async fn case01_always_compression_shrinks_a_repeating_payload() {
    let dir = tempdir().expect("tempdir");
    let src = dir.path().join("repeat.txt");
    let dst = dir.path().join("repeat.txt.zst");

    let payload: Vec<u8> = std::iter::repeat_n(b'A', 16 * 1024 * 1024).collect();
    fs::write(&src, &payload).expect("write src");

    let hook = CopyThatCryptHook::new(
        CompressionPolicy::Always {
            level: CompressionLevel(3),
        },
        None,
    );
    let outcome = hook
        .transform(src.clone(), dst.clone())
        .await
        .expect("transform");

    assert_eq!(outcome.input_bytes, payload.len() as u64);
    let dst_size = fs::metadata(&dst).unwrap().len();
    assert!(
        dst_size < 4 * 1024 * 1024,
        "expected dst < 4 MiB, got {dst_size}"
    );
    let ratio = outcome.compression_ratio.expect("ratio");
    assert!(
        ratio < 0.25,
        "expected ratio < 0.25 for highly-compressible payload, got {ratio}"
    );
}

#[tokio::test]
async fn case02_smart_policy_denies_jpg_extension() {
    let src_dir = tempdir().expect("src tempdir");
    let dst_dir = tempdir().expect("dst tempdir");
    let src = src_dir.path().join("noise.jpg");
    let dst = dst_dir.path().join("noise.jpg");

    // Pseudo-random bytes — zstd shouldn't get any meaningful
    // ratio out of this even if it ran, so the test is on the
    // policy's *deny* decision, not the compressor's behaviour.
    let mut payload = vec![0u8; 16 * 1024 * 1024];
    for (i, b) in payload.iter_mut().enumerate() {
        *b = ((i * 37) ^ (i >> 4)) as u8;
    }
    fs::write(&src, &payload).expect("write src");

    let hook = CopyThatCryptHook::new(CompressionPolicy::smart(), None);
    let plan = hook.will_transform("jpg");
    assert!(
        plan.is_noop(),
        "smart policy must deny jpg, plan was {plan:?}"
    );

    // Run through transform anyway (the hook is wired noop-safe);
    // assert no compression happened.
    let outcome = hook
        .transform(src.clone(), dst.clone())
        .await
        .expect("transform");
    assert_eq!(outcome.input_bytes, payload.len() as u64);
    assert_eq!(
        outcome.output_bytes, outcome.input_bytes,
        "denied jpg must pass bytes through unchanged"
    );
    assert!(outcome.compression_ratio.is_none());
}

#[tokio::test]
async fn case03_passphrase_encryption_round_trips() {
    let dir = tempdir().expect("tempdir");
    let src = dir.path().join("plain.txt");
    let dst = dir.path().join("plain.txt.age");

    let payload: Vec<u8> = std::iter::repeat_n(b'P', 1024 * 1024).collect();
    fs::write(&src, &payload).expect("write src");

    let pw = SecretString::from("phase35-correct-horse-battery".to_string());
    let hook = CopyThatCryptHook::new(
        CompressionPolicy::Off,
        Some(EncryptionPolicy::passphrase(pw.clone())),
    );
    let outcome = hook
        .transform(src.clone(), dst.clone())
        .await
        .expect("transform");
    assert_eq!(outcome.input_bytes, payload.len() as u64);
    assert!(outcome.encrypted);

    let encrypted = fs::read(&dst).expect("read dst");
    let identity = Identity::new().with_passphrase(pw);
    let mut reader =
        decrypted_reader(std::io::Cursor::new(encrypted), &identity).expect("decryptor");
    let mut round_trip = Vec::new();
    reader.read_to_end(&mut round_trip).expect("read");
    assert_eq!(round_trip, payload, "round-trip byte mismatch");
}

#[test]
fn case04_all_phase_35_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_35_KEYS {
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

fn locate_locales_dir() -> Option<std::path::PathBuf> {
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

fn _touch(_p: &Path) {}
