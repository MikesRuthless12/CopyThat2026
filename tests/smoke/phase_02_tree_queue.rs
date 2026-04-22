//! Phase 2 smoke test.
//!
//! Generates 500 files across 20 directories with sizes spanning
//! 0 B .. ~50 MB (weighted so the total payload is manageable on CI
//! free runners), copies the tree end-to-end through `copy_tree`,
//! asserts byte-for-byte equality and matching mtime on every file,
//! then `move_tree`s the copy back to a fresh destination and asserts
//! the source is gone.

use std::path::{Path, PathBuf};
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent, MoveOptions, TreeOptions, copy_tree, move_tree};
use filetime::{FileTime, set_file_mtime};
use rand::{RngCore, SeedableRng};
use tempfile::tempdir;
use tokio::sync::mpsc;

const FILES: usize = 500;
const DIRS: usize = 20;

fn build_source_tree(root: &Path) -> Vec<(PathBuf, usize)> {
    let mut files = Vec::with_capacity(FILES);
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xC2_5EED);

    for d in 0..DIRS {
        let dir = root.join(format!("dir-{d:02}"));
        std::fs::create_dir_all(&dir).unwrap();
    }

    // Size distribution (bytes). Most files small, a few medium, one
    // large (~5 MB) so the total caps well under 50 MB on disk.
    let size_for_index = |i: usize| -> usize {
        match i % 25 {
            0 => 0,                         // empty
            1..=5 => 1 + (i * 13) % 4096,   // tiny (< 4 KiB)
            6..=18 => 16 * 1024 + i * 37,   // small (~16 KiB)
            19..=23 => 256 * 1024 + i * 91, // medium (~256 KiB)
            _ => 5 * 1024 * 1024,           // one "big" per group (5 MiB)
        }
    };

    for i in 0..FILES {
        let d = i % DIRS;
        let dir = root.join(format!("dir-{d:02}"));
        let path = dir.join(format!("file-{i:04}.bin"));
        let size = size_for_index(i);
        let mut buf = vec![0u8; size];
        if size > 0 {
            rng.fill_bytes(&mut buf);
        }
        std::fs::write(&path, &buf).unwrap();
        // Stamp a deterministic, recognisable mtime per file so
        // we can assert preservation after copy_tree.
        let stamp = FileTime::from_unix_time(1_700_000_000 + (i as i64) * 3, 0);
        set_file_mtime(&path, stamp).unwrap();
        files.push((path, size));
    }

    files
}

fn verify_pair(src: &Path, dst: &Path) {
    let src_bytes = std::fs::read(src).unwrap();
    let dst_bytes = std::fs::read(dst).unwrap();
    assert_eq!(
        src_bytes.len(),
        dst_bytes.len(),
        "size mismatch for {}",
        dst.display()
    );
    assert_eq!(src_bytes, dst_bytes, "byte mismatch for {}", dst.display());

    let src_mtime = FileTime::from_last_modification_time(&std::fs::metadata(src).unwrap());
    let dst_mtime = FileTime::from_last_modification_time(&std::fs::metadata(dst).unwrap());
    assert_eq!(
        src_mtime,
        dst_mtime,
        "mtime mismatch on {}: src={src_mtime:?} dst={dst_mtime:?}",
        dst.display()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn phase_02_smoke_tree_roundtrip_and_move() {
    let root = tempdir().unwrap();
    let src = root.path().join("src");
    let dst = root.path().join("dst");
    let moved = root.path().join("moved");

    let files = build_source_tree(&src);
    assert_eq!(files.len(), FILES);

    // ---------- copy_tree ----------
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(4096);
    let ctrl = CopyControl::new();
    let src_task = src.clone();
    let dst_task = dst.clone();
    let start = Instant::now();
    let task = tokio::spawn(async move {
        copy_tree(&src_task, &dst_task, TreeOptions::default(), ctrl, tx).await
    });
    let mut tree_started = false;
    let mut tree_completed = false;
    while let Some(evt) = rx.recv().await {
        match evt {
            CopyEvent::TreeStarted { .. } => {
                // The streaming walker (Phase 13) fires `TreeStarted`
                // with zero totals — the real denominator grows via
                // `TreeEnumerating` events as the walker discovers
                // files. Final count lands in `TreeCompleted` below.
                tree_started = true;
            }
            CopyEvent::TreeCompleted { files, .. } => {
                tree_completed = true;
                assert_eq!(files as usize, FILES);
            }
            CopyEvent::Failed { err } => panic!("copy_tree failed: {err}"),
            _ => {}
        }
    }
    let report = task.await.unwrap().unwrap();
    let copy_elapsed = start.elapsed();
    assert!(tree_started);
    assert!(tree_completed);
    assert_eq!(report.files as usize, FILES);

    // Byte + mtime verification.
    for (p, _size) in &files {
        let rel = p.strip_prefix(&src).unwrap();
        verify_pair(p, &dst.join(rel));
    }

    eprintln!(
        "PHASE02 SMOKE: copy_tree {} files ({:.2} MiB) in {:.3} s ({:.2} MiB/s)",
        report.files,
        report.bytes as f64 / (1024.0 * 1024.0),
        copy_elapsed.as_secs_f64(),
        report.bytes as f64 / (1024.0 * 1024.0) / copy_elapsed.as_secs_f64().max(1e-9),
    );

    // ---------- move_tree ----------
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(4096);
    let ctrl = CopyControl::new();
    let dst_task = dst.clone();
    let moved_task = moved.clone();
    let start = Instant::now();
    let task = tokio::spawn(async move {
        move_tree(&dst_task, &moved_task, MoveOptions::default(), ctrl, tx).await
    });
    while rx.recv().await.is_some() {}
    let move_report = task.await.unwrap().unwrap();
    let move_elapsed = start.elapsed();

    assert!(!dst.exists(), "source of move should be gone");
    // After move, every file should still match the original source.
    for (p, _) in &files {
        let rel = p.strip_prefix(&src).unwrap();
        verify_pair(p, &moved.join(rel));
    }

    eprintln!(
        "PHASE02 SMOKE: move_tree ({} -> {}) in {:.3} s",
        move_report.root_src.display(),
        move_report.root_dst.display(),
        move_elapsed.as_secs_f64(),
    );
}
