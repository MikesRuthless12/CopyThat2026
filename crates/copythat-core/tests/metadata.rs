//! Metadata preservation: mtime, atime, permissions.

mod common;

use std::time::{Duration, SystemTime};

use common::{run_copy, write_random};
use copythat_core::CopyOptions;
use filetime::{FileTime, set_file_mtime};
use tempfile::tempdir;

#[tokio::test]
async fn mtime_is_preserved() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("t.bin");
    let dst = dir.path().join("t.out");
    write_random(&src, 64 * 1024, 99);

    // Stamp the source with an mtime we can recognise.
    let stamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    set_file_mtime(&src, FileTime::from_system_time(stamp)).unwrap();

    let (report, _) = run_copy(&src, &dst, CopyOptions::default()).await;
    let report = report.unwrap();
    assert_eq!(report.bytes, 64 * 1024);

    let src_md = std::fs::metadata(&src).unwrap();
    let dst_md = std::fs::metadata(&dst).unwrap();
    let src_mtime = FileTime::from_last_modification_time(&src_md);
    let dst_mtime = FileTime::from_last_modification_time(&dst_md);
    assert_eq!(
        src_mtime, dst_mtime,
        "mtime on dst ({dst_mtime:?}) does not match src ({src_mtime:?})"
    );
}

#[tokio::test]
async fn preserve_times_false_leaves_fresh_mtime() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("t.bin");
    let dst = dir.path().join("t.out");
    write_random(&src, 4 * 1024, 11);

    let old = FileTime::from_unix_time(1_600_000_000, 0);
    set_file_mtime(&src, old).unwrap();

    let opts = CopyOptions {
        preserve_times: false,
        ..Default::default()
    };
    let (report, _) = run_copy(&src, &dst, opts).await;
    report.unwrap();

    let dst_mtime = FileTime::from_last_modification_time(&std::fs::metadata(&dst).unwrap());
    assert_ne!(
        dst_mtime, old,
        "with preserve_times = false the destination should carry a fresh mtime"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn unix_permissions_are_preserved() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempdir().unwrap();
    let src = dir.path().join("p.bin");
    let dst = dir.path().join("p.out");
    std::fs::write(&src, b"x").unwrap();
    std::fs::set_permissions(&src, std::fs::Permissions::from_mode(0o640)).unwrap();

    let (report, _) = run_copy(&src, &dst, CopyOptions::default()).await;
    report.unwrap();

    let mode = std::fs::metadata(&dst).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o640);
}

// Clippy (rightly) warns that `set_readonly(false)` on Unix makes a
// file world-writable. This test is Windows-only — on NTFS the
// operation simply clears the FILE_ATTRIBUTE_READONLY bit and is the
// only portable way through std to do it. Scope the allow narrowly.
#[cfg(windows)]
#[allow(clippy::permissions_set_readonly_false)]
#[tokio::test]
async fn windows_readonly_bit_is_preserved() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("r.bin");
    let dst = dir.path().join("r.out");
    std::fs::write(&src, b"x").unwrap();
    let mut perms = std::fs::metadata(&src).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&src, perms).unwrap();

    let (report, _) = run_copy(&src, &dst, CopyOptions::default()).await;
    report.unwrap();

    assert!(
        std::fs::metadata(&dst).unwrap().permissions().readonly(),
        "destination should have inherited readonly"
    );
    // Release the readonly so tempdir cleanup can unlink the file.
    let mut unlock = std::fs::metadata(&dst).unwrap().permissions();
    unlock.set_readonly(false);
    std::fs::set_permissions(&dst, unlock).unwrap();
    let mut unlock_src = std::fs::metadata(&src).unwrap().permissions();
    unlock_src.set_readonly(false);
    std::fs::set_permissions(&src, unlock_src).unwrap();
}
