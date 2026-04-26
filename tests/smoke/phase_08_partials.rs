//! Phase 8 partials smoke test.
//!
//! Closes the three deferred items from Phase 8's original brief:
//!
//! 1. **Elevated retry.** The `retry_elevated` IPC was a stub that
//!    always surfaced `retry-elevated-unavailable`. Phase 17d's
//!    `copythat-helper` now backs the path; the IPC routes through
//!    `handle_request(Request::ElevatedRetry, …)` and surfaces a
//!    typed result.
//! 2. **SHA-256 quick-hash button.** New `quick_hash_for_collision`
//!    IPC computes a SHA-256 digest for either side of a
//!    collision so the user can decide overwrite-vs-skip with
//!    content evidence.
//! 3. **Drawer-mode toggle.** Already shipped end-to-end through
//!    `GeneralSettings::error_display_mode`. This smoke confirms
//!    the wire shape is stable + the locale strings are present.

use std::path::{Path, PathBuf};

use copythat_helper::capability::Capability;
use copythat_helper::handle_request;
use copythat_helper::rpc::{Request, Response};
use tempfile::TempDir;

#[test]
fn elevated_retry_via_helper_round_trips_a_real_file() {
    // The IPC layer in `commands.rs::retry_elevated` calls into the
    // helper via the same `handle_request` we exercise here. The
    // production path then either runs in-process (today) or
    // routes through a UAC/sudo/polkit spawn (Phase 17d body fill);
    // the wire shape is identical.
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("payload.bin");
    let dst = dir.path().join("dst.bin");
    std::fs::write(&src, b"phase-8-partials").unwrap();

    let granted = vec![Capability::ElevatedRetry];
    let resp = handle_request(
        &Request::ElevatedRetry {
            src: src.clone(),
            dst: dst.clone(),
        },
        &granted,
    );
    match resp {
        Response::ElevatedRetryOk { bytes } => assert_eq!(bytes, 16),
        other => panic!("expected ElevatedRetryOk, got {other:?}"),
    }
    assert_eq!(std::fs::read(&dst).unwrap(), b"phase-8-partials");
}

#[test]
fn elevated_retry_without_capability_returns_unavailable_key() {
    // The IPC's no-capability surface maps onto the same Fluent key
    // the prior stub used (`retry-elevated-unavailable`), so the UI
    // doesn't have to special-case the new vs old return shape.
    let resp = handle_request(
        &Request::ElevatedRetry {
            src: PathBuf::from("/a"),
            dst: PathBuf::from("/b"),
        },
        &[],
    );
    assert!(matches!(resp, Response::CapabilityDenied { .. }));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn quick_hash_returns_stable_sha256() {
    // The quick_hash_for_collision IPC routes through
    // `copythat_hash::hash_file_async` with `HashAlgorithm::Sha256`.
    // We exercise the same surface here so a regression in the
    // hash algorithm or the report-shape contract trips this smoke
    // before it lands in the modal.
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("hash-me.txt");
    std::fs::write(&p, b"abc").unwrap();
    let (tx, _rx) = tokio::sync::mpsc::channel(8);
    let report = copythat_hash::hash_file_async(
        &p,
        copythat_hash::HashAlgorithm::Sha256,
        copythat_core::CopyControl::new(),
        tx,
    )
    .await
    .unwrap();
    // SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
    assert_eq!(
        report.hex(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

fn repo_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &Path = &here;
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = match cur.parent() {
            Some(p) => p,
            None => break,
        };
    }
    panic!("could not locate repo root");
}

#[test]
fn drawer_toggle_locale_keys_present_in_every_locale() {
    // The Phase 8 brief promised a Settings toggle with two
    // possible values (`error-modal-mode-modal` /
    // `error-modal-mode-drawer`). Confirm both keys exist in
    // every locale so the dropdown renders cleanly worldwide.
    let root = repo_root();
    let locales = std::fs::read_dir(root.join("locales")).unwrap();
    let mut count = 0;
    for entry in locales.flatten() {
        let ftl = entry.path().join("copythat.ftl");
        if !ftl.exists() {
            continue;
        }
        let body = std::fs::read_to_string(&ftl).unwrap();
        for required in [
            "settings-error-display-mode",
            "settings-error-display-modal",
            "settings-error-display-drawer",
        ] {
            assert!(
                body.contains(required),
                "locale {} missing {required}",
                entry.path().display(),
            );
        }
        count += 1;
    }
    assert_eq!(count, 18);
}

#[test]
fn collision_modal_hash_check_locale_key_is_present() {
    // Phase 8's deferred SHA-256 quick-hash button references the
    // pre-existing locale key `collision-modal-hash-check`. Ensure
    // it's still in the source-of-truth locale.
    let body = std::fs::read_to_string(repo_root().join("locales/en/copythat.ftl")).unwrap();
    assert!(body.contains("collision-modal-hash-check"));
}
