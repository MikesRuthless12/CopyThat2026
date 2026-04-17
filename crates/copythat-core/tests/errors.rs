//! Error-path tests: missing source, read-only destination, disk-full.

mod common;

use common::run_copy;
use copythat_core::{CopyError, CopyErrorKind, CopyOptions};
use tempfile::tempdir;

#[tokio::test]
async fn missing_source_is_not_found() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("nope.bin");
    let dst = dir.path().join("out.bin");
    let (result, _) = run_copy(&src, &dst, CopyOptions::default()).await;
    let err: CopyError = result.expect_err("missing source must error");
    assert_eq!(err.kind, CopyErrorKind::NotFound);
}

#[cfg(unix)]
#[tokio::test]
async fn readonly_destination_directory_yields_permission_denied() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempdir().unwrap();
    let src = dir.path().join("in.bin");
    let dst_dir = dir.path().join("ro");
    std::fs::create_dir(&dst_dir).unwrap();
    std::fs::write(&src, b"hello").unwrap();
    let mut p = std::fs::metadata(&dst_dir).unwrap().permissions();
    // Strip write permission so OpenOptions::create fails.
    p.set_mode(0o555);
    std::fs::set_permissions(&dst_dir, p).unwrap();

    let dst = dst_dir.join("out.bin");
    let (result, _) = run_copy(&src, &dst, CopyOptions::default()).await;
    let err = result.expect_err("must error");
    assert_eq!(err.kind, CopyErrorKind::PermissionDenied);

    // Restore so tempdir cleanup succeeds.
    let mut back = std::fs::metadata(&dst_dir).unwrap().permissions();
    back.set_mode(0o755);
    std::fs::set_permissions(&dst_dir, back).unwrap();
}

#[tokio::test]
async fn fail_if_exists_rejects_overwrite() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("a.bin");
    let dst = dir.path().join("b.bin");
    std::fs::write(&src, b"abc").unwrap();
    std::fs::write(&dst, b"xyz").unwrap();

    let opts = CopyOptions {
        fail_if_exists: true,
        ..Default::default()
    };
    let (result, _) = run_copy(&src, &dst, opts).await;
    let err = result.expect_err("must error");
    // Whichever kind the OS serves up, it must not be NotFound.
    assert_ne!(err.kind, CopyErrorKind::NotFound);
    // The existing destination must remain untouched (we refused to
    // overwrite — not "overwrote then errored").
    assert_eq!(std::fs::read(&dst).unwrap(), b"xyz");
}

// Linux-only: simulate ENOSPC by mounting a tmpfs with a tiny size
// cap, write more bytes than fit, and assert the engine reports
// DiskFull. Gated because it requires `mount` privileges.
#[cfg(all(target_os = "linux", feature = "slow-tests"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn linux_disk_full_classifies_correctly() {
    use std::process::Command;

    // Only meaningful if we can mount tmpfs — skip cleanly otherwise
    // so it doesn't fail on unprivileged CI.
    let check = Command::new("sh")
        .arg("-c")
        .arg("mount | grep -q tmpfs || true")
        .status();
    if check.is_err() {
        eprintln!("skipping: mount unavailable");
        return;
    }

    let root = tempdir().unwrap();
    let tmpfs_dir = root.path().join("tmpfs");
    std::fs::create_dir(&tmpfs_dir).unwrap();
    let mount = Command::new("sudo")
        .args([
            "-n",
            "mount",
            "-t",
            "tmpfs",
            "-o",
            "size=1m",
            "tmpfs",
            tmpfs_dir.to_str().unwrap(),
        ])
        .status();
    let Ok(st) = mount else {
        eprintln!("skipping: sudo mount failed");
        return;
    };
    if !st.success() {
        eprintln!("skipping: tmpfs mount declined");
        return;
    }

    let src = root.path().join("big.bin");
    std::fs::write(&src, vec![0u8; 4 * 1024 * 1024]).unwrap();
    let dst = tmpfs_dir.join("big.bin");
    let (result, _) = run_copy(&src, &dst, CopyOptions::default()).await;

    let _ = Command::new("sudo")
        .args(["-n", "umount", tmpfs_dir.to_str().unwrap()])
        .status();

    let err = result.expect_err("disk full must error");
    assert_eq!(err.kind, CopyErrorKind::DiskFull);
}
