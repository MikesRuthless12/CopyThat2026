//! Phase 17f smoke test — logging & content-scrubbing audit.
//!
//! Three workstreams converge here:
//!
//! 1. **Tracing field scrub.** The `copythat-audit` `AuditLayer`
//!    drops events whose fields are named `body` / `bytes` /
//!    `chunk` / `password` / `passphrase` / `secret` / `token` /
//!    `api_key` regardless of level. The unit test in `layer.rs`
//!    asserts the per-field drop end-to-end through the sink; this
//!    smoke duplicates the assertion at the workspace level so a
//!    silent rename of the field-allowlist function trips the
//!    smoke before it trips production.
//! 2. **Sidecar relpath enforcement.** `copythat_hash::sidecar`
//!    writers refuse absolute / `..` paths so the on-disk format
//!    can't accidentally publish the user's directory layout.
//! 3. **`eprintln!` audit.** The IPC layer in
//!    `apps/copythat-ui/src-tauri/src/commands.rs` no longer prints
//!    user paths via `eprintln!`. The sweep is best-effort — any
//!    new diagnostic line should use `tracing::debug!` instead.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use copythat_audit::{AuditFormat, AuditLayer, AuditSink, WormMode};
use copythat_hash::HashAlgorithm;
use copythat_hash::sidecar::{
    SidecarEntry, validate_sidecar_relpath, write_single_file_sidecar, write_tree_sidecar,
};
use tempfile::TempDir;
use tracing_subscriber::layer::SubscriberExt;

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
    panic!("could not locate repo root from {}", here.display());
}

#[test]
fn audit_layer_drops_sensitive_fields() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("audit.log");
    let sink = Arc::new(
        AuditSink::open(&path, AuditFormat::JsonLines, WormMode::Off).expect("open audit sink"),
    );
    let layer = AuditLayer::new(sink.clone());
    let subscriber = tracing_subscriber::registry().with(layer);

    tracing::subscriber::with_default(subscriber, || {
        tracing::warn!(
            target: "copythat::audit",
            body = "leak-bytes-1",
            password = "leak-pwd-1",
            secret = "leak-secret-1",
            "tag-only"
        );
    });

    let body = fs::read_to_string(&path).unwrap();
    assert!(body.contains("tag-only"));
    for forbidden in [
        "leak-bytes-1",
        "leak-pwd-1",
        "leak-secret-1",
        "body=",
        "password=",
        "secret=",
    ] {
        assert!(
            !body.contains(forbidden),
            "audit log leaked {forbidden}:\n{body}",
        );
    }
}

#[test]
fn sidecar_relpath_validator_rejects_absolutes() {
    let abs = if cfg!(windows) {
        PathBuf::from(r"C:\windows\system32\calc.exe")
    } else {
        PathBuf::from("/etc/passwd")
    };
    let err = validate_sidecar_relpath(&abs).expect_err("absolute must be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn sidecar_relpath_validator_rejects_parent_dirs() {
    let bad = PathBuf::from("foo/../etc/passwd");
    let err = validate_sidecar_relpath(&bad).expect_err("`..` must be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn sidecar_relpath_validator_accepts_relative() {
    validate_sidecar_relpath(Path::new("ok.txt")).unwrap();
    validate_sidecar_relpath(Path::new("subdir/ok.txt")).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn write_tree_sidecar_rejects_absolute_entry() {
    let dir = TempDir::new().unwrap();
    let sidecar_path = dir.path().join("hashes.sha256");
    let entries = [SidecarEntry::new(
        "0000000000000000000000000000000000000000000000000000000000000000",
        // Absolute — must be rejected before any byte is written.
        if cfg!(windows) {
            PathBuf::from(r"C:\evil.txt")
        } else {
            PathBuf::from("/evil.txt")
        },
    )];
    let err = write_tree_sidecar(&sidecar_path, &entries)
        .await
        .expect_err("absolute path entry must fail the write");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        !sidecar_path.exists(),
        "no sidecar file should land when validation fails",
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn write_single_file_sidecar_writes_relative_only() {
    let dir = TempDir::new().unwrap();
    let dst = dir.path().join("payload.bin");
    fs::write(&dst, b"hi").unwrap();

    let sidecar = write_single_file_sidecar(
        &dst,
        HashAlgorithm::Sha256,
        "0000000000000000000000000000000000000000000000000000000000000000",
    )
    .await
    .unwrap();

    let body = fs::read_to_string(&sidecar).unwrap();
    // The line is "<hex>  <relpath>". The relpath must be the
    // file name only; never an absolute path leaking the user's
    // tempdir / home / drive.
    assert!(body.trim_end().ends_with("payload.bin"), "body={body:?}");
    let abs_marker = if cfg!(windows) { ":\\" } else { "/" };
    assert!(
        !body.contains(abs_marker) || body.matches(abs_marker).count() == 0,
        "sidecar leaked absolute path: {body:?}",
    );
}

#[test]
fn ipc_layer_does_not_eprintln_user_paths_in_release_builds() {
    // Tripwire against re-introducing `eprintln!` lines that print
    // raw user paths in the IPC layer. The Phase 17f bullet calls
    // for migrating those to `tracing::debug!` so production builds
    // don't surface paths on stderr. We don't enforce 0 eprintlns —
    // some are still useful for non-path diagnostics — but no
    // eprintln line in commands.rs may interpolate `path` /
    // `paths` / `dst` / `src` / `destination` / `source`.
    let body =
        fs::read_to_string(repo_root().join("apps/copythat-ui/src-tauri/src/commands.rs")).unwrap();
    let mut offenders: Vec<&str> = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("eprintln!") {
            continue;
        }
        // Look for path-typed interpolations. Skip lines that are
        // commenting on what was removed.
        let lc = trimmed.to_ascii_lowercase();
        let path_kw = [
            "{path}",
            "{paths",
            "{src",
            "{dst",
            "{destination",
            "{source",
            ".display()",
        ];
        if path_kw.iter().any(|kw| lc.contains(kw)) {
            offenders.push(line);
        }
    }
    assert!(
        offenders.is_empty(),
        "Phase 17f tripwire — user-path eprintlns in commands.rs:\n{}",
        offenders.join("\n"),
    );
}
