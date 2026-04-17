//! move_file / move_tree — same-volume rename fast path, and
//! cross-device fallback shaped by the strict_rename flag.

mod common;

use copythat_core::{CopyControl, CopyEvent, MoveOptions, move_file, move_tree};
use tempfile::tempdir;
use tokio::sync::mpsc;

use common::write_random;

async fn run_move_file(
    src: &std::path::Path,
    dst: &std::path::Path,
    opts: MoveOptions,
) -> Result<copythat_core::CopyReport, copythat_core::CopyError> {
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(256);
    let ctrl = CopyControl::new();
    let src = src.to_path_buf();
    let dst = dst.to_path_buf();
    let task = tokio::spawn(async move { move_file(&src, &dst, opts, ctrl, tx).await });
    while rx.recv().await.is_some() {}
    task.await.unwrap()
}

#[tokio::test]
async fn move_file_same_volume_is_atomic_rename() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("a.bin");
    let dst = dir.path().join("b.bin");
    std::fs::write(&src, b"payload").unwrap();

    run_move_file(&src, &dst, MoveOptions::default())
        .await
        .unwrap();
    assert!(!src.exists(), "source should be gone after move");
    assert_eq!(std::fs::read(&dst).unwrap(), b"payload");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn move_file_into_nested_missing_parent_errors() {
    // Without create_dir_all the destination's parent is missing —
    // rename fails, and since it's NotFound (not EXDEV) the strict
    // path applies.
    let dir = tempdir().unwrap();
    let src = dir.path().join("a.bin");
    std::fs::write(&src, b"x").unwrap();
    let dst = dir.path().join("nope/nested/b.bin");

    let res = run_move_file(&src, &dst, MoveOptions::default()).await;
    assert!(res.is_err());
    assert!(src.exists(), "source must remain on rename failure");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn move_tree_same_volume_is_atomic_rename() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    std::fs::create_dir_all(src.join("a/b")).unwrap();
    write_random(&src.join("a/b/f.bin"), 4096, 10);
    std::fs::write(src.join("top.txt"), b"top").unwrap();

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(128);
    let ctrl = CopyControl::new();
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move {
        move_tree(&src_task, &dst_task, MoveOptions::default(), ctrl, tx).await
    });
    while rx.recv().await.is_some() {}
    task.await.unwrap().unwrap();

    assert!(!src.exists(), "source tree should be gone");
    assert!(dst.join("a/b/f.bin").is_file());
    assert_eq!(std::fs::read(dst.join("top.txt")).unwrap(), b"top");
}

#[tokio::test]
async fn strict_rename_surfaces_rename_errors_as_is() {
    // With strict_rename = true, a broken parent becomes a hard
    // error without triggering the copy fallback.
    let dir = tempdir().unwrap();
    let src = dir.path().join("a.bin");
    std::fs::write(&src, b"x").unwrap();
    let dst = dir.path().join("does/not/exist/a.bin");

    let opts = MoveOptions {
        strict_rename: true,
        ..Default::default()
    };
    let res = run_move_file(&src, &dst, opts).await;
    assert!(res.is_err());
    assert!(src.exists());
}
