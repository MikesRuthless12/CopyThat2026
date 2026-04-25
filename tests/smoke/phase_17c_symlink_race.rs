//! Phase 17c smoke test — symlink-race / TOCTOU hardening.
//!
//! The engine's per-file open path now carries `O_NOFOLLOW`
//! (Linux/macOS) / `FILE_FLAG_OPEN_REPARSE_POINT` (Windows). This
//! test asserts that:
//!
//! 1. `safety::no_follow_open_flags()` returns the right per-OS
//!    value at compile time.
//! 2. `safety::is_no_follow_rejection` recognises the platform's
//!    symlink-rejection error codes.
//! 3. `safety::is_within_root` correctly classifies a candidate
//!    inside / outside the chosen jail root.
//! 4. **Race regression** — when the source is swapped for a symlink
//!    between the metadata pre-flight and the open (Phase 17c
//!    threat model), the engine surfaces a typed error rather than
//!    silently copying the symlink target's contents.

use std::path::Path;

use copythat_core::safety::{
    PathSafetyError, is_no_follow_rejection, is_within_root, no_follow_open_flags,
    validate_path_no_traversal,
};
use tempfile::TempDir;

#[test]
fn no_follow_flags_present_on_supported_targets() {
    let flags = no_follow_open_flags();
    #[cfg(target_os = "linux")]
    assert_eq!(flags, 0x20000, "O_NOFOLLOW (Linux glibc/musl) = 0x20000");
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    assert_eq!(flags, 0x100, "O_NOFOLLOW (Apple libc) = 0x100");
    #[cfg(target_os = "windows")]
    assert_eq!(flags, 0x00200000, "FILE_FLAG_OPEN_REPARSE_POINT");
    let _ = flags;
}

#[test]
fn is_no_follow_rejection_recognises_per_os_codes() {
    #[cfg(target_os = "linux")]
    {
        let err = std::io::Error::from_raw_os_error(40); // ELOOP
        assert!(is_no_follow_rejection(&err));
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        let err = std::io::Error::from_raw_os_error(62); // ELOOP on Apple
        assert!(is_no_follow_rejection(&err));
    }
    #[cfg(target_os = "windows")]
    {
        let err1 = std::io::Error::from_raw_os_error(1920); // ERROR_CANT_ACCESS_FILE
        assert!(is_no_follow_rejection(&err1));
        let err2 = std::io::Error::from_raw_os_error(1); // ERROR_INVALID_FUNCTION
        assert!(is_no_follow_rejection(&err2));
    }
    // Non-matching codes always classify as "not a no-follow rejection".
    let unrelated = std::io::Error::from_raw_os_error(2); // ENOENT / ERROR_FILE_NOT_FOUND
    assert!(!is_no_follow_rejection(&unrelated));
}

#[test]
fn is_within_root_jails_correctly() {
    let dir = TempDir::new().unwrap();
    let root = dir.path().to_path_buf();
    let inside = root.join("ok.txt");
    std::fs::write(&inside, b"hi").unwrap();

    // Same-dir parent is inside.
    let res = is_within_root(&inside, &root).unwrap();
    assert!(res, "{:?} should be inside {:?}", inside, root);

    // A sibling tempdir is outside.
    let other = TempDir::new().unwrap();
    let outside = other.path().join("evil.txt");
    std::fs::write(&outside, b"x").unwrap();
    let res = is_within_root(&outside, &root).unwrap();
    assert!(!res, "{:?} must NOT be inside {:?}", outside, root);
}

#[test]
fn lexical_guard_still_rejects_traversal() {
    // Phase 17a still applies regardless of Phase 17c — the lexical
    // guard runs *before* any FS call. A traversal-laden source path
    // never reaches the no-follow open in the first place.
    let err = validate_path_no_traversal(Path::new("evil/../../../etc/passwd")).unwrap_err();
    assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn copy_file_rejects_post_check_symlink_swap_unix() {
    use copythat_core::{CopyControl, CopyOptions, copy_file};
    use std::os::unix::fs::symlink;
    use tokio::sync::mpsc;

    let dir = TempDir::new().unwrap();
    let real = dir.path().join("real.bin");
    std::fs::write(&real, b"genuine").unwrap();

    // Victim file the symlink would silently redirect to. Real
    // contents the attacker wants the engine to copy.
    let victim = dir.path().join("victim.secret");
    std::fs::write(&victim, b"SECRET").unwrap();

    // Source position the engine sees: a symlink pointing at the
    // victim. Without O_NOFOLLOW the engine would happily follow
    // it and copy SECRET. Phase 17c's `no_follow_open_flags` sets
    // O_NOFOLLOW; the engine must surface ELOOP rather than copy
    // the victim's bytes.
    let src = dir.path().join("src");
    symlink(&victim, &src).unwrap();

    let dst = dir.path().join("dst.bin");

    // The engine's metadata pre-flight rejects symlinks when
    // `follow_symlinks = false` (the default). Force the path that
    // exercises Phase 17c by enabling `follow_symlinks`. The lexical
    // guard still passes (no `..`), the metadata check classifies
    // the resolved file as a regular file, the open runs with
    // O_NOFOLLOW — the kernel returns ELOOP at open time.
    let mut opts = CopyOptions::default();
    opts.follow_symlinks = true;

    let (tx, _rx) = mpsc::channel(8);
    let result = copy_file(&src, &dst, opts, CopyControl::new(), tx).await;

    // Phase 17c hardening must surface as a copy failure. The
    // kernel-level ELOOP wraps to a typed I/O error; the engine
    // already maps it via `CopyError::from_io`.
    let err = result.expect_err("symlink swap must be rejected by O_NOFOLLOW");
    let raw = err.raw_os_error;
    let msg = err.message.to_lowercase();
    assert!(
        raw.map(|c| c == 40 || c == 62).unwrap_or(false)
            || msg.contains("symbolic")
            || msg.contains("loop"),
        "expected ELOOP-shaped error from O_NOFOLLOW open, got {err:?}",
    );

    // Defence-in-depth: no destination file written.
    assert!(
        !dst.exists(),
        "engine wrote dst from a no-follow-rejected source"
    );
}
