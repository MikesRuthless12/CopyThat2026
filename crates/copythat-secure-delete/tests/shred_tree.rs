//! Integration tests for `shred_tree`.

use std::path::{Path, PathBuf};

use copythat_core::CopyControl;
use copythat_secure_delete::{ShredEvent, ShredMethod, shred_tree};
use tempfile::tempdir;
use tokio::sync::mpsc;

fn seed_tree(root: &Path) -> Vec<PathBuf> {
    // Shape:
    //   root/
    //     top.bin
    //     a/
    //       a1.bin
    //       a2.bin
    //       aa/
    //         aa1.bin
    //     b/                  (empty dir)
    //     c/
    //       c1.bin
    let mut files = Vec::new();
    let top = root.join("top.bin");
    std::fs::write(&top, b"top").unwrap();
    files.push(top);

    std::fs::create_dir_all(root.join("a/aa")).unwrap();
    for (name, bytes) in [
        ("a/a1.bin", b"aaa1" as &[u8]),
        ("a/a2.bin", b"aaa2"),
        ("a/aa/aa1.bin", b"deep"),
    ] {
        let p = root.join(name);
        std::fs::write(&p, bytes).unwrap();
        files.push(p);
    }

    std::fs::create_dir_all(root.join("b")).unwrap();
    std::fs::create_dir_all(root.join("c")).unwrap();
    let c1 = root.join("c/c1.bin");
    std::fs::write(&c1, b"cc1!").unwrap();
    files.push(c1);

    files
}

#[tokio::test]
async fn shred_tree_removes_every_file_and_every_directory() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("target");
    std::fs::create_dir(&root).unwrap();
    let files = seed_tree(&root);

    let (tx, _rx) = mpsc::channel::<ShredEvent>(1024);
    let ctrl = CopyControl::new();
    let report = shred_tree(&root, ShredMethod::Zero, ctrl, tx)
        .await
        .expect("tree shred");

    assert_eq!(report.files, files.len() as u64);
    assert!(!root.exists(), "root survived: {root:?}");
    for f in &files {
        assert!(!f.exists(), "file survived: {f:?}");
    }
}

#[tokio::test]
async fn shred_tree_depth_first_contents_before_directories() {
    // Assert that the event stream never reports `TreeCompleted`
    // before every PassCompleted fires — i.e. no directory removal
    // is attempted while files still exist inside it.
    let dir = tempdir().unwrap();
    let root = dir.path().join("dfroot");
    std::fs::create_dir(&root).unwrap();
    let files = seed_tree(&root);

    let (tx, mut rx) = mpsc::channel::<ShredEvent>(1024);
    let ctrl = CopyControl::new();
    let root_clone = root.clone();
    let task =
        tokio::spawn(async move { shred_tree(&root_clone, ShredMethod::Zero, ctrl, tx).await });

    let mut files_completed: u64 = 0;
    let mut tree_completed_seen_at: Option<u64> = None;
    while let Some(evt) = rx.recv().await {
        match evt {
            ShredEvent::Completed { .. } => files_completed += 1,
            ShredEvent::TreeCompleted { .. } => {
                tree_completed_seen_at = Some(files_completed);
            }
            _ => {}
        }
    }
    let _ = task.await.unwrap().expect("tree shred");

    assert_eq!(files_completed, files.len() as u64);
    assert_eq!(
        tree_completed_seen_at,
        Some(files.len() as u64),
        "TreeCompleted fired before all files completed"
    );
}

#[tokio::test]
async fn shred_tree_on_file_delegates_to_shred_file() {
    let dir = tempdir().unwrap();
    let f = dir.path().join("solo.bin");
    std::fs::write(&f, b"solo").unwrap();

    let (tx, _rx) = mpsc::channel::<ShredEvent>(64);
    let report = shred_tree(&f, ShredMethod::Zero, CopyControl::new(), tx)
        .await
        .expect("solo file");
    assert_eq!(report.files, 1);
    assert!(!f.exists());
}

#[tokio::test]
async fn shred_tree_handles_empty_directory() {
    let dir = tempdir().unwrap();
    let empty = dir.path().join("empty");
    std::fs::create_dir(&empty).unwrap();

    let (tx, _rx) = mpsc::channel::<ShredEvent>(64);
    let report = shred_tree(&empty, ShredMethod::Zero, CopyControl::new(), tx)
        .await
        .expect("empty dir");
    assert_eq!(report.files, 0);
    assert!(!empty.exists());
}
