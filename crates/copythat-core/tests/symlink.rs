//! Symlink follow vs. no-follow.
//!
//! On Windows, creating a symlink requires either admin rights or
//! Developer Mode. These tests skip (PASS) if symlink creation fails
//! so CI doesn't need elevation.

mod common;

use std::path::Path;

use common::{run_copy, write_random};
use copythat_core::CopyOptions;
use tempfile::tempdir;

fn try_symlink_file(target: &Path, link: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(target, link)
    }
}

#[tokio::test]
async fn follow_symlinks_copies_the_target_contents() {
    let dir = tempdir().unwrap();
    let real = dir.path().join("real.bin");
    let link = dir.path().join("link.bin");
    let dst = dir.path().join("out.bin");
    let expected = write_random(&real, 32 * 1024, 21);
    if try_symlink_file(&real, &link).is_err() {
        eprintln!("skipping: no symlink privilege on this host");
        return;
    }

    let opts = CopyOptions {
        follow_symlinks: true,
        ..Default::default()
    };
    let (report, _) = run_copy(&link, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.bytes as usize, expected.len());
    // Destination must be a regular file, not a symlink.
    let md = std::fs::symlink_metadata(&dst).unwrap();
    assert!(!md.file_type().is_symlink());
    assert_eq!(std::fs::read(&dst).unwrap(), expected);
}

#[tokio::test]
async fn no_follow_clones_the_symlink_itself() {
    let dir = tempdir().unwrap();
    let real = dir.path().join("real.bin");
    let link = dir.path().join("link.bin");
    let dst = dir.path().join("out.bin");
    write_random(&real, 512, 22);
    if try_symlink_file(&real, &link).is_err() {
        eprintln!("skipping: no symlink privilege on this host");
        return;
    }

    let opts = CopyOptions {
        follow_symlinks: false,
        ..Default::default()
    };
    let (report, _) = run_copy(&link, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.bytes, 0, "symlink clone reports 0 bytes");

    let md = std::fs::symlink_metadata(&dst).unwrap();
    assert!(md.file_type().is_symlink(), "dst must itself be a symlink");
    let target = std::fs::read_link(&dst).unwrap();
    assert_eq!(target, real);
}
