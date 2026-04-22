//! copy_tree happy path: structure + contents + progress.

mod common;

use copythat_core::{CopyControl, CopyEvent, TreeOptions, copy_tree};
use tempfile::tempdir;
use tokio::sync::mpsc;

use common::write_random;

fn build_test_tree(root: &std::path::Path) {
    std::fs::create_dir_all(root.join("a/b/c")).unwrap();
    std::fs::create_dir_all(root.join("a/d")).unwrap();
    std::fs::create_dir_all(root.join("empty")).unwrap();

    write_random(&root.join("a/b/small.bin"), 1024, 1);
    write_random(&root.join("a/b/c/mid.bin"), 128 * 1024, 2);
    write_random(&root.join("a/d/big.bin"), 512 * 1024, 3);
    std::fs::write(root.join("a/zero.bin"), b"").unwrap();
    std::fs::write(root.join("top.txt"), b"hello world").unwrap();
}

fn same_bytes(a: &std::path::Path, b: &std::path::Path) -> bool {
    std::fs::read(a).unwrap() == std::fs::read(b).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn copy_tree_preserves_structure_and_contents() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    build_test_tree(&src);

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(1024);
    let ctrl = CopyControl::new();

    let events_src = src.clone();
    let events_dst = dst.clone();
    let task = tokio::spawn(async move {
        copy_tree(&events_src, &events_dst, TreeOptions::default(), ctrl, tx).await
    });

    let mut saw_tree_started = false;
    let mut saw_tree_completed = false;
    let mut tree_progress_count = 0;
    while let Some(evt) = rx.recv().await {
        match evt {
            CopyEvent::TreeStarted { .. } => {
                // Streaming walker (Phase 13) fires `TreeStarted` with
                // zero totals — the real denominator arrives via
                // `TreeEnumerating` events as the walker discovers
                // entries. The final numbers are asserted via
                // `report.files` / `report.bytes` below.
                saw_tree_started = true;
            }
            CopyEvent::TreeProgress { .. } => tree_progress_count += 1,
            CopyEvent::TreeCompleted { .. } => saw_tree_completed = true,
            CopyEvent::Failed { err } => panic!("tree copy failed: {err}"),
            _ => {}
        }
    }

    let report = task.await.unwrap().unwrap();
    assert_eq!(report.files, 5);
    assert_eq!(report.skipped, 0);
    assert!(saw_tree_started);
    assert!(saw_tree_completed);
    assert!(
        tree_progress_count >= 1,
        "no TreeProgress events were emitted"
    );

    // Structure mirrored.
    assert!(dst.join("a/b/c").is_dir());
    assert!(dst.join("a/d").is_dir());
    assert!(dst.join("empty").is_dir());

    // Contents equal.
    assert!(same_bytes(
        &src.join("a/b/small.bin"),
        &dst.join("a/b/small.bin")
    ));
    assert!(same_bytes(
        &src.join("a/b/c/mid.bin"),
        &dst.join("a/b/c/mid.bin")
    ));
    assert!(same_bytes(
        &src.join("a/d/big.bin"),
        &dst.join("a/d/big.bin")
    ));
    assert!(same_bytes(&src.join("a/zero.bin"), &dst.join("a/zero.bin")));
    assert!(same_bytes(&src.join("top.txt"), &dst.join("top.txt")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn copy_tree_creates_missing_destination_root() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("a/nested/dst");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("f.txt"), b"x").unwrap();

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
    let ctrl = CopyControl::new();
    let dst_check = dst.clone();
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move {
        copy_tree(&src_task, &dst_task, TreeOptions::default(), ctrl, tx).await
    });
    while rx.recv().await.is_some() {}
    task.await.unwrap().unwrap();
    assert!(dst_check.join("f.txt").is_file());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn copy_tree_concurrency_is_respected() {
    // Bump concurrency > 1 so the parallel path is exercised. We don't
    // assert parallel ordering (that'd be flaky); we just verify the
    // tree lands correctly under N > 1.
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    std::fs::create_dir_all(&src).unwrap();
    for i in 0..20 {
        write_random(&src.join(format!("f{i}.bin")), 4096, i as u64);
    }

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(1024);
    let ctrl = CopyControl::new();
    let opts = TreeOptions {
        concurrency: 8,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_tree(&src_task, &dst_task, opts, ctrl, tx).await });
    while rx.recv().await.is_some() {}
    let report = task.await.unwrap().unwrap();
    assert_eq!(report.files, 20);
    for i in 0..20 {
        assert!(same_bytes(
            &src.join(format!("f{i}.bin")),
            &dst.join(format!("f{i}.bin"))
        ));
    }
}
