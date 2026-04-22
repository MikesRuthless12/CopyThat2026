//! Phase 14a smoke test — enumeration-time filters.
//!
//! Builds a deliberately varied tree (mixed extensions, sizes,
//! mtimes, attribute bits, and a pruneable `node_modules` subtree),
//! then exercises each filter class independently by running
//! `copy_tree` and asserting on the destination contents. The tree
//! walker's `skipped_by_filter` counter is internal; we verify the
//! observable effect (files that should survive the filter land at
//! dst, files that shouldn't are absent).

use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use copythat_core::{CopyControl, CopyEvent, FilterSet, TreeOptions, copy_tree};
use filetime::{FileTime, set_file_mtime};
use tempfile::tempdir;
use tokio::sync::mpsc;

fn write_file(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn set_mtime(path: &Path, unix_secs: i64) {
    set_file_mtime(path, FileTime::from_unix_time(unix_secs, 0)).unwrap();
}

fn set_readonly(path: &Path) {
    let mut perm = std::fs::metadata(path).unwrap().permissions();
    perm.set_readonly(true);
    std::fs::set_permissions(path, perm).unwrap();
}

/// Strip the readonly bit after a test so tempdir cleanup can remove
/// the file. Windows-only — on Unix the tempdir just deletes the
/// file regardless of mode, and clippy warns that toggling the
/// readonly bit back to `false` on Unix would world-chmod the file.
#[cfg(windows)]
fn clear_readonly(path: &Path) {
    let mut perm = std::fs::metadata(path).unwrap().permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perm.set_readonly(false);
    std::fs::set_permissions(path, perm).unwrap();
}

#[cfg(not(windows))]
fn clear_readonly(_path: &Path) {}

/// Build the sample source tree used by every filter test.
fn seed_tree(src: &Path) {
    // The 2 MiB payload stays a `Vec` because a `[u8; 2 MiB]` stack
    // array would blow the default thread stack on Windows; the
    // smaller ones use stack arrays so clippy doesn't complain.
    write_file(&src.join("readme.md"), &[b'a'; 100]);
    write_file(&src.join("notes.txt"), &[b'b'; 200]);
    write_file(&src.join("data.csv"), &[b'c'; 1_024]);
    write_file(&src.join("large.bin"), &vec![b'd'; 2 * 1024 * 1024]);
    write_file(&src.join("readonly.log"), &[b'e'; 300]);
    write_file(&src.join("photos/a.jpg"), &[b'f'; 500]);
    write_file(&src.join("photos/b.png"), &[b'g'; 600]);
    write_file(&src.join("node_modules/pkg/index.js"), &[b'h'; 100]);
    write_file(&src.join("src_code/main.rs"), &[b'i'; 400]);

    // Ancient file so date filters have something to exclude.
    set_mtime(&src.join("notes.txt"), 1_577_836_800); // 2020-01-01 UTC
    // Everything else: a clearly modern mtime (2025-06-01 UTC). Set
    // so date filters have a predictable cutoff above and below.
    for p in [
        "readme.md",
        "data.csv",
        "large.bin",
        "readonly.log",
        "photos/a.jpg",
        "photos/b.png",
        "node_modules/pkg/index.js",
        "src_code/main.rs",
    ] {
        set_mtime(&src.join(p), 1_748_736_000); // 2025-06-01 UTC
    }

    // Readonly last — on Windows, setting mtime on an already-
    // readonly file fails with ERROR_ACCESS_DENIED (code 5), so all
    // metadata fiddling must complete before the bit goes on.
    set_readonly(&src.join("readonly.log"));
}

async fn run_copy(src: &Path, dst: &Path, filters: Option<FilterSet>) {
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(256);
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    let opts = TreeOptions {
        filters,
        ..TreeOptions::default()
    };
    copy_tree(src, dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy_tree failed");
    drain.await.unwrap();
}

fn collect_relative_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(root).unwrap().to_path_buf();
        out.push(rel);
    }
    out.sort();
    out
}

fn cleanup(dst: &Path) {
    // Readonly files propagate their bit via `preserve_permissions`;
    // clear before tempdir cleanup so Windows can delete them.
    if dst.exists() {
        for entry in walkdir::WalkDir::new(dst) {
            let Ok(entry) = entry else { continue };
            if entry.file_type().is_file() {
                clear_readonly(entry.path());
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn baseline_no_filter_copies_everything() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    run_copy(&src, &dst, None).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert_eq!(files.len(), 9, "baseline must copy all 9 files: {files:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn include_glob_only_keeps_txt() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        include_globs: vec!["**/*.txt".into()],
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert_eq!(files, vec![PathBuf::from("notes.txt")]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exclude_glob_prunes_node_modules_subtree() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        exclude_globs: vec!["**/node_modules".into()],
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert!(
        !files.iter().any(|p| p.starts_with("node_modules")),
        "node_modules subtree must be pruned: {files:?}"
    );
    assert_eq!(files.len(), 8, "everything else survives: {files:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn size_floor_excludes_small_files() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    // 512-byte floor: keeps data.csv (1024), large.bin (2 MB),
    // photos/a.jpg (500… wait 500 < 512, excluded), photos/b.png (600).
    let filters = FilterSet {
        min_size_bytes: Some(512),
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(names.iter().any(|n| n.ends_with("data.csv")), "{names:?}");
    assert!(names.iter().any(|n| n.ends_with("large.bin")), "{names:?}");
    assert!(names.iter().any(|n| n.ends_with("b.png")), "{names:?}");
    assert!(!names.iter().any(|n| n.ends_with("readme.md")), "{names:?}");
    assert!(!names.iter().any(|n| n.ends_with("notes.txt")), "{names:?}");
    assert!(
        !names.iter().any(|n| n.ends_with("readonly.log")),
        "{names:?}"
    );
    assert!(!names.iter().any(|n| n.ends_with("a.jpg")), "{names:?}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn size_ceiling_excludes_the_big_file() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        max_size_bytes: Some(1_500),
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(!names.iter().any(|n| n.ends_with("large.bin")), "{names:?}");
    assert_eq!(
        files.len(),
        8,
        "8 of 9 survive (all but large.bin): {names:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn min_mtime_excludes_ancient_files() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    // Cutoff 2024-01-01 UTC: the 2020-era `notes.txt` is excluded,
    // the 2025-era rest survive.
    let cutoff = UNIX_EPOCH + Duration::from_secs(1_704_067_200);
    let filters = FilterSet {
        min_mtime: Some(cutoff),
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert!(
        !files.iter().any(|p| p.ends_with("notes.txt")),
        "ancient notes.txt must be excluded: {files:?}"
    );
    assert_eq!(files.len(), 8);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn max_mtime_keeps_only_ancient() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let cutoff = UNIX_EPOCH + Duration::from_secs(1_704_067_200);
    let filters = FilterSet {
        max_mtime: Some(cutoff),
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert_eq!(files, vec![PathBuf::from("notes.txt")]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn skip_readonly_drops_readonly_file() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        skip_readonly: true,
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert!(
        !files.iter().any(|p| p.ends_with("readonly.log")),
        "readonly.log must be excluded: {files:?}"
    );
    assert_eq!(files.len(), 8);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_glob_surfaces_error() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        include_globs: vec!["[broken".into()],
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(32);
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    let opts = TreeOptions {
        filters: Some(filters),
        ..TreeOptions::default()
    };
    let err = copy_tree(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect_err("bad glob must fail");
    drain.await.unwrap();
    assert!(
        err.message.contains("include"),
        "error must name the offending list: {err:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn combined_filters_intersect() {
    // include=**/*.png AND size >= 550 should keep only photos/b.png
    // (photos/a.jpg is excluded by the .png glob; photos/b.png at
    // 600 bytes clears the 550 floor).
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    seed_tree(&src);

    let filters = FilterSet {
        include_globs: vec!["**/*.png".into()],
        min_size_bytes: Some(550),
        ..Default::default()
    };
    run_copy(&src, &dst, Some(filters)).await;

    let files = collect_relative_files(&dst);
    cleanup(&dst);
    assert_eq!(files, vec![PathBuf::from("photos/b.png")]);
}
