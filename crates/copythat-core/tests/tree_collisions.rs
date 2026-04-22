//! Every CollisionPolicy branch.

mod common;

use std::time::Duration;

use copythat_core::{
    CollisionPolicy, CollisionResolution, CopyControl, CopyEvent, TreeOptions, copy_tree,
};
use filetime::{FileTime, set_file_mtime};
use tempfile::tempdir;
use tokio::sync::mpsc;

async fn drain_until_done(mut rx: mpsc::Receiver<CopyEvent>) -> Vec<CopyEvent> {
    let mut out = Vec::new();
    while let Some(e) = rx.recv().await {
        out.push(e);
    }
    out
}

async fn run_tree(
    src: &std::path::Path,
    dst: &std::path::Path,
    opts: TreeOptions,
) -> (
    Result<copythat_core::TreeReport, copythat_core::CopyError>,
    Vec<CopyEvent>,
) {
    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let ctrl = CopyControl::new();
    let src = src.to_path_buf();
    let dst = dst.to_path_buf();
    let task = tokio::spawn(async move { copy_tree(&src, &dst, opts, ctrl, tx).await });
    let events = drain_until_done(rx).await;
    (task.await.unwrap(), events)
}

fn prep(src: &std::path::Path, dst: &std::path::Path) {
    std::fs::create_dir_all(src).unwrap();
    std::fs::create_dir_all(dst).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn skip_leaves_existing_destination_untouched() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("f.txt"), b"SRC").unwrap();
    std::fs::write(dst.join("f.txt"), b"DST").unwrap();

    let opts = TreeOptions {
        collision: CollisionPolicy::Skip,
        ..Default::default()
    };
    let (report, _) = run_tree(&src, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.files, 0);
    assert_eq!(report.skipped, 1);
    assert_eq!(std::fs::read(dst.join("f.txt")).unwrap(), b"DST");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn overwrite_replaces_destination() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("f.txt"), b"SRC").unwrap();
    std::fs::write(dst.join("f.txt"), b"DST").unwrap();

    let opts = TreeOptions {
        collision: CollisionPolicy::Overwrite,
        ..Default::default()
    };
    let (report, _) = run_tree(&src, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.files, 1);
    assert_eq!(std::fs::read(dst.join("f.txt")).unwrap(), b"SRC");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn overwrite_if_newer_only_replaces_when_source_is_newer() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("fresh.txt"), b"FRESH").unwrap();
    std::fs::write(src.join("stale.txt"), b"STALE").unwrap();
    std::fs::write(dst.join("fresh.txt"), b"OLD").unwrap();
    std::fs::write(dst.join("stale.txt"), b"NEW").unwrap();

    // source-fresh is newer than dst
    set_file_mtime(
        src.join("fresh.txt"),
        FileTime::from_unix_time(2_000_000_000, 0),
    )
    .unwrap();
    set_file_mtime(
        dst.join("fresh.txt"),
        FileTime::from_unix_time(1_500_000_000, 0),
    )
    .unwrap();
    // source-stale is older than dst
    set_file_mtime(
        src.join("stale.txt"),
        FileTime::from_unix_time(1_500_000_000, 0),
    )
    .unwrap();
    set_file_mtime(
        dst.join("stale.txt"),
        FileTime::from_unix_time(2_000_000_000, 0),
    )
    .unwrap();

    let opts = TreeOptions {
        collision: CollisionPolicy::OverwriteIfNewer,
        ..Default::default()
    };
    let (report, _) = run_tree(&src, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.files, 1, "only the newer file should be copied");
    assert_eq!(report.skipped, 1);
    assert_eq!(std::fs::read(dst.join("fresh.txt")).unwrap(), b"FRESH");
    assert_eq!(std::fs::read(dst.join("stale.txt")).unwrap(), b"NEW");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keep_both_creates_parenthesised_suffix() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("doc.txt"), b"NEW").unwrap();
    std::fs::write(dst.join("doc.txt"), b"OLD").unwrap();

    let opts = TreeOptions {
        collision: CollisionPolicy::KeepBoth,
        ..Default::default()
    };
    let (report, _) = run_tree(&src, &dst, opts).await;
    report.unwrap();
    assert_eq!(std::fs::read(dst.join("doc.txt")).unwrap(), b"OLD");
    // `keep_both_path` names the duplicate `doc_2.txt` — the original
    // is the implicit "1", so the first generated suffix is `_2`.
    assert_eq!(std::fs::read(dst.join("doc_2.txt")).unwrap(), b"NEW");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rename_uses_given_filename() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("a.txt"), b"XYZ").unwrap();
    std::fs::write(dst.join("a.txt"), b"keep").unwrap();

    let opts = TreeOptions {
        collision: CollisionPolicy::Rename("a-copy.txt".into()),
        ..Default::default()
    };
    let (report, _) = run_tree(&src, &dst, opts).await;
    report.unwrap();
    assert_eq!(std::fs::read(dst.join("a.txt")).unwrap(), b"keep");
    assert_eq!(std::fs::read(dst.join("a-copy.txt")).unwrap(), b"XYZ");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn prompt_resolution_overwrite_applies_fresh_content() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("f.txt"), b"SRC").unwrap();
    std::fs::write(dst.join("f.txt"), b"DST").unwrap();

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
    let ctrl = CopyControl::new();
    let opts = TreeOptions {
        collision: CollisionPolicy::Prompt,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_tree(&src_task, &dst_task, opts, ctrl, tx).await });

    let mut saw_collision = false;
    while let Some(evt) = rx.recv().await {
        if let CopyEvent::Collision(c) = evt {
            saw_collision = true;
            assert!(c.src.ends_with("f.txt"));
            c.resolve(CollisionResolution::Overwrite);
        }
    }
    let report = tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .expect("copy_tree hung after resolving Collision")
        .unwrap()
        .unwrap();
    assert!(saw_collision);
    assert_eq!(report.files, 1);
    assert_eq!(std::fs::read(dst.join("f.txt")).unwrap(), b"SRC");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn prompt_without_receiver_treats_as_skip() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    prep(&src, &dst);
    std::fs::write(src.join("f.txt"), b"SRC").unwrap();
    std::fs::write(dst.join("f.txt"), b"DST").unwrap();

    // Drop the receiver immediately; the engine must not hang.
    let (tx, rx) = mpsc::channel::<CopyEvent>(8);
    drop(rx);
    let ctrl = CopyControl::new();
    let opts = TreeOptions {
        collision: CollisionPolicy::Prompt,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_tree(&src_task, &dst_task, opts, ctrl, tx).await });
    let report = tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .expect("copy_tree hung without collision receiver")
        .unwrap()
        .unwrap();
    assert_eq!(report.skipped, 1);
    assert_eq!(std::fs::read(dst.join("f.txt")).unwrap(), b"DST");
}
