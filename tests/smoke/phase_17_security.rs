//! Phase 17a smoke test — red-team path-safety guard.
//!
//! The Phase 17 prompt's mandatory smoke check is:
//! > a scripted red-team check — craft a path like
//! > `foo/../../../etc/passwd` as a destination; assert rejection
//! > with a typed PathEscape error.
//!
//! This file exercises the rejection across every layer the
//! attacker can reach:
//!
//! 1. **Pure helper** — `safety::validate_path_no_traversal`
//!    rejects the crafted string and surfaces a typed
//!    [`PathSafetyError::ParentTraversal`].
//! 2. **Engine entry** — `copy_file` returns
//!    `CopyError::path_escape` (i.e. `CopyErrorKind::PathEscape`)
//!    when called with the same destination, *without* creating
//!    any partial file at the resolved path.
//! 3. **Tree entry** — `copy_tree` rejects the same way before any
//!    `TreeStarted` event fires.
//! 4. **Localised key** — the kind's `localized_key()` resolves to
//!    the new `err-path-escape` Fluent key, which `xtask
//!    i18n-lint` proves is present in every locale.
//!
//! The test never touches the actual `/etc/passwd` path; it works
//! entirely inside a `TempDir` and asserts on the rejected error
//! type, not on filesystem state.

use std::path::Path;

use copythat_core::{
    CopyControl, CopyErrorKind, CopyOptions, PathSafetyError, TreeOptions, copy_file, copy_tree,
    safety::validate_path_no_traversal,
};
use tempfile::TempDir;
use tokio::sync::mpsc;

/// The attack string that gives the test its name. Same shape the
/// Phase 17 prompt calls out — relative `..` traversal that would
/// otherwise escape any staging root.
const TRAVERSAL_PAYLOAD: &str = "foo/../../../etc/passwd";

#[test]
fn helper_rejects_traversal_payload() {
    let err = validate_path_no_traversal(Path::new(TRAVERSAL_PAYLOAD)).unwrap_err();
    assert!(
        matches!(err, PathSafetyError::ParentTraversal { .. }),
        "expected ParentTraversal, got {err:?}"
    );
    assert_eq!(err.localized_key(), "err-path-escape");
}

#[test]
fn helper_rejects_absolute_traversal_payload() {
    // Even an absolute prefix can't launder a `..` sequence — the
    // engine refuses any path with a parent-dir component, full stop.
    #[cfg(unix)]
    let payload = "/var/data/../../../../etc/passwd";
    #[cfg(windows)]
    let payload = r"C:\ProgramData\..\..\..\..\Windows\System32\config\SAM";
    let err = validate_path_no_traversal(Path::new(payload)).unwrap_err();
    assert!(
        matches!(err, PathSafetyError::ParentTraversal { .. }),
        "expected ParentTraversal, got {err:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn copy_file_rejects_traversal_destination() {
    let tmp = TempDir::new().expect("tempdir");
    // Real source — ensures rejection is *not* coming from a
    // missing-source code path; the `..` destination is the cause.
    let src = tmp.path().join("payload.bin");
    std::fs::write(&src, b"hello").unwrap();

    let dst = tmp.path().join(TRAVERSAL_PAYLOAD);

    let (tx, mut rx) = mpsc::channel(8);
    let result = copy_file(&src, &dst, CopyOptions::default(), CopyControl::new(), tx).await;

    let err = result.expect_err("traversal destination must be rejected");
    assert_eq!(
        err.kind,
        CopyErrorKind::PathEscape,
        "expected PathEscape, got {:?} ({})",
        err.kind,
        err.message,
    );
    assert_eq!(err.localized_key(), "err-path-escape");

    // No event channel traffic — the engine refused before opening
    // anything, so the receiver should be empty.
    assert!(
        rx.try_recv().is_err(),
        "engine should not emit any events on a path-escape rejection",
    );

    // Defence-in-depth: confirm the engine did NOT create any file
    // at the would-be resolved path on disk. We don't compute the
    // real resolution (that's the OS's job); we just sanity-check
    // that the verbatim `..`-laden literal also wasn't dropped on
    // disk inside the tempdir.
    assert!(!dst.exists(), "engine wrote to a rejected path");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn copy_tree_rejects_traversal_destination() {
    let tmp = TempDir::new().expect("tempdir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(src_dir.join("inner")).unwrap();
    std::fs::write(src_dir.join("inner").join("a.txt"), b"hi").unwrap();

    let dst = tmp.path().join(TRAVERSAL_PAYLOAD);

    let (tx, mut rx) = mpsc::channel(8);
    let result = copy_tree(
        &src_dir,
        &dst,
        TreeOptions::default(),
        CopyControl::new(),
        tx,
    )
    .await;

    let err = result.expect_err("traversal destination must be rejected");
    assert_eq!(
        err.kind,
        CopyErrorKind::PathEscape,
        "expected PathEscape, got {:?} ({})",
        err.kind,
        err.message,
    );
    assert!(
        rx.try_recv().is_err(),
        "tree walker should not emit TreeStarted on a path-escape rejection",
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn copy_file_rejects_traversal_source() {
    // Symmetry check: the engine must reject a crafted *source* path
    // too. Otherwise an attacker who controls only the source side
    // (e.g. a recursive index that silently inserts `..` segments)
    // could still trigger a traversal.
    let tmp = TempDir::new().expect("tempdir");
    let dst = tmp.path().join("dst.bin");
    let src = tmp.path().join(TRAVERSAL_PAYLOAD);

    let (tx, _rx) = mpsc::channel(8);
    let err = copy_file(&src, &dst, CopyOptions::default(), CopyControl::new(), tx)
        .await
        .expect_err("traversal source must be rejected");
    assert_eq!(err.kind, CopyErrorKind::PathEscape);
}

#[test]
fn err_path_escape_key_resolves() {
    // Sanity check: the kind's stable Fluent key matches the new
    // English source-of-truth string. `xtask i18n-lint` runs the
    // 18-locale parity check; this anchors the constant.
    assert_eq!(
        copythat_core::CopyErrorKind::PathEscape.localized_key(),
        "err-path-escape"
    );
}
