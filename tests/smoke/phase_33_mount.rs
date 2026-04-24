//! Phase 33 smoke — mount-as-filesystem foundation.
//!
//! Phase 33a ships the pure-Rust tree builder + handle + NoopBackend
//! without pulling in FUSE / WinFsp at all. This smoke proves:
//!
//! - `MountTree::build` composes the three-view layout from a
//!   `Vec<JobSummary>` + a chunk store reference, matching the brief's
//!   mount layout (`by-date/` / `by-source/` / `by-job-id/`).
//! - `MountHandle::Drop` invokes the session's `unmount_on_drop`
//!   exactly once.
//! - Mountpoint validation rejects unsafe paths + non-empty dirs.
//! - `MountLayout` toggles suppress branches deterministically.
//! - All 8 Phase 33 Fluent keys ship in every one of the 18 locales.
//!
//! Real FUSE / WinFsp mounts land in Phase 33b behind the `fuse` +
//! `winfsp` features; the smoke would gain a `#[cfg(feature = "...")]`-
//! gated case at that point.

use std::path::PathBuf;

use copythat_chunk::ChunkStore;
use copythat_history::JobSummary;
use copythat_mount::{
    ArchiveRefs, MountBackend, MountError, MountFileKind, MountHandle, MountLayout, MountTree,
    NodeKind, NoopBackend, ROOT_INODE, TreeInodeMap, default_backend_name, synthesize_attr,
};

const PHASE_33_KEYS: &[&str] = &[
    "mount-heading",
    "mount-action-mount",
    "mount-action-unmount",
    "mount-status-mounted",
    "mount-error-unsafe-mountpoint",
    "mount-error-mountpoint-not-empty",
    "mount-error-backend-unavailable",
    "mount-error-archive-read",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

fn sample_job(row_id: i64, src: &str, dst: &str, started_at_ms: i64) -> JobSummary {
    JobSummary {
        row_id,
        kind: "copy".into(),
        status: "succeeded".into(),
        started_at_ms,
        finished_at_ms: Some(started_at_ms + 10_000),
        src_root: PathBuf::from(src),
        dst_root: PathBuf::from(dst),
        total_bytes: 1024,
        files_ok: 1,
        files_failed: 0,
        verify_algo: None,
        options_json: None,
    }
}

/// Case 1 — `MountTree::build` emits the canonical three-view layout
/// from a hand-seeded job set.
#[test]
fn case1_build_emits_three_views() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_store = ChunkStore::open(tmp.path()).expect("chunk store");
    let jobs = vec![
        sample_job(1, r"C:\src\project", r"C:\dst\archive", 1_776_781_935_000),
        sample_job(2, r"D:\photos", r"C:\backup\photos", 1_776_781_936_000),
    ];

    let tree = MountTree::build(&jobs, &chunk_store, MountLayout::all()).expect("build");
    assert_eq!(tree.job_count, 2);

    // by-date: 2026-04-21/14-32-15/archive-copy for job 1.
    let by_date = tree
        .lookup("by-date/2026-04-21/14-32-15/archive-copy")
        .expect("by-date leaf for job 1");
    assert!(matches!(
        by_date.kind,
        NodeKind::JobPlaceholder { job_row_id: 1 }
    ));

    // by-source: Windows drive escaping.
    assert!(tree.lookup("by-source/C--src-project").is_some());
    assert!(tree.lookup("by-source/D--photos").is_some());

    // by-job-id: numeric, direct lookup.
    let by_id_1 = tree.lookup("by-job-id/1/archive-copy").expect("id 1");
    assert!(matches!(
        by_id_1.kind,
        NodeKind::JobPlaceholder { job_row_id: 1 }
    ));
}

/// Case 2 — empty history builds an empty tree without panicking.
#[test]
fn case2_empty_history_empty_tree() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_store = ChunkStore::open(tmp.path()).expect("chunk store");
    let tree = MountTree::build(&[], &chunk_store, MountLayout::all()).expect("build");
    assert_eq!(tree.job_count, 0);
    assert_eq!(tree.root.children.len(), 0);
}

/// Case 3 — NoopBackend Drop fires unmount_on_drop exactly once.
#[test]
fn case3_noop_backend_drop_semantics() {
    let backend = NoopBackend::default();
    let tmp = tempfile::tempdir().expect("tempdir");
    let counter = backend.unmount_counter();
    assert_eq!(*counter.lock().expect("lock"), 0);

    let handle: MountHandle = backend
        .mount(tmp.path(), MountLayout::all(), &ArchiveRefs::default())
        .expect("mount");
    assert!(handle.is_live());
    assert_eq!(handle.mountpoint(), tmp.path());
    drop(handle);

    assert_eq!(*counter.lock().expect("lock"), 1);
}

/// Case 4 — Explicit unmount skips the Drop-time pass.
#[test]
fn case4_explicit_unmount_then_drop_noop() {
    let backend = NoopBackend::default();
    let tmp = tempfile::tempdir().expect("tempdir");
    let handle = backend
        .mount(tmp.path(), MountLayout::all(), &ArchiveRefs::default())
        .expect("mount");
    handle.unmount().expect("explicit unmount");

    // The Drop path has nothing to run after explicit unmount —
    // only one count total.
    assert_eq!(*backend.unmount_counter().lock().expect("lock"), 1);
}

/// Case 5 — Unsafe mountpoints + non-empty dirs are rejected.
#[test]
fn case5_mountpoint_validation() {
    let backend = NoopBackend::default();

    // ../escape rejected.
    let err = backend
        .mount(
            std::path::Path::new("../escape"),
            MountLayout::all(),
            &ArchiveRefs::default(),
        )
        .expect_err("traversal rejected");
    assert!(matches!(err, MountError::UnsafeMountpoint(_)));

    // Non-empty dir rejected.
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::write(tmp.path().join("stray"), b"hi").expect("write stray");
    let err = backend
        .mount(tmp.path(), MountLayout::all(), &ArchiveRefs::default())
        .expect_err("nonempty rejected");
    assert!(matches!(err, MountError::MountpointNotEmpty(_)));
}

/// Case 6 — MountLayout toggles suppress the non-selected branches.
#[test]
fn case6_partial_layout_suppresses_branches() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_store = ChunkStore::open(tmp.path()).expect("chunk store");
    let jobs = vec![sample_job(1, "/a", "/dst/x", 1_776_781_935_000)];

    let only_by_id = MountLayout {
        by_date: false,
        by_source: false,
        by_job_id: true,
    };
    let tree = MountTree::build(&jobs, &chunk_store, only_by_id).expect("build");
    assert!(tree.lookup("by-date").is_none());
    assert!(tree.lookup("by-source").is_none());
    assert!(tree.lookup("by-job-id/1").is_some());
}

/// Case 7 — Fluent key presence in `en`.
#[test]
fn case7_en_fluent_keys_present() {
    let root = workspace_root();
    let path = root.join("locales").join("en").join("copythat.ftl");
    let content = std::fs::read_to_string(&path).expect("read en locale");
    for key in PHASE_33_KEYS {
        let needle = format!("{key} =");
        assert!(
            content.contains(&needle),
            "missing Phase 33 Fluent key `{key}` in en/copythat.ftl"
        );
    }
}

/// Case 8 — Fluent key parity across all 18 locales.
#[test]
fn case8_all_18_locales_have_phase_33_keys() {
    let root = workspace_root();
    for locale in LOCALES {
        let path = root.join("locales").join(locale).join("copythat.ftl");
        let content = std::fs::read_to_string(&path).expect("read locale");
        for key in PHASE_33_KEYS {
            let needle = format!("{key} =");
            assert!(
                content.contains(&needle),
                "locale `{locale}` missing Phase 33 key `{key}`"
            );
        }
    }
}

fn workspace_root() -> PathBuf {
    // `CARGO_MANIFEST_DIR` is `crates/copythat-mount`; workspace is
    // two parents up.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .expect("workspace root")
}

/// Case 9 — Phase 33c feature-gate contract: on a default build
/// (no `fuse` / `winfsp` features), the runtime selector reports
/// `"noop"`. Platform-enabled builds surface `"fuse"` / `"winfsp"`;
/// either way, the selector never panics.
#[test]
fn case9_default_backend_name_is_stable() {
    let name = default_backend_name();
    assert!(matches!(name, "fuse" | "winfsp" | "noop"));
}

/// Case 11 — Phase 33e `synthesize_attr` projects an inode entry
/// into the platform-neutral `MountFileAttr` a real FUSE/WinFsp
/// `getattr` callback will translate to its native shape.
#[test]
fn case11_synthesize_attr_projection() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_store = ChunkStore::open(tmp.path()).expect("chunk store");
    let jobs = vec![sample_job(1, "/src", "/dst/archive", 1_776_781_935_000)];
    let tree = MountTree::build(&jobs, &chunk_store, MountLayout::all()).expect("build");
    let map = TreeInodeMap::from_tree(&tree);

    // Root: directory, 0o555, nlink = 2 + 3 subdirs.
    let root_attr = synthesize_attr(&map, ROOT_INODE, 1_776_781_935).expect("root");
    assert_eq!(root_attr.kind, MountFileKind::Directory);
    assert_eq!(root_attr.perm, 0o555);
    assert_eq!(root_attr.nlink, 5);

    // Leaf (the single job placeholder under by-job-id/1): file,
    // 0o444, size 0 in 33e.
    let by_job_id = map.lookup(ROOT_INODE, "by-job-id").expect("by-job-id");
    let job_one = map.lookup(by_job_id, "1").expect("1");
    let leaf = map.lookup(job_one, "archive-copy").expect("archive-copy");
    let attr = synthesize_attr(&map, leaf, 1_776_781_935).expect("leaf");
    assert_eq!(attr.kind, MountFileKind::RegularFile);
    assert_eq!(attr.perm, 0o444);
    assert_eq!(attr.size, 0);
    assert_eq!(attr.nlink, 1);

    // Missing inode surfaces None — callback will return ENOENT.
    assert!(synthesize_attr(&map, 999_999, 1_776_781_935).is_none());
}

/// Case 10 — Phase 33d inode map: a `TreeInodeMap` built from a
/// `MountTree` exposes root-inode 1, parent-child links, and a
/// stable-sorted readdir.
#[test]
fn case10_tree_inode_map_exposes_readdir_and_lookup() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_store = ChunkStore::open(tmp.path()).expect("chunk store");
    let jobs = vec![sample_job(1, "/src", "/dst/archive", 1_776_781_935_000)];
    let tree = MountTree::build(&jobs, &chunk_store, MountLayout::all()).expect("build");
    let map = TreeInodeMap::from_tree(&tree);

    let root = map.get(ROOT_INODE).expect("root present");
    assert_eq!(root.parent, ROOT_INODE);

    // readdir at root is sorted: by-date / by-job-id / by-source.
    let top: Vec<String> = map
        .readdir(ROOT_INODE)
        .into_iter()
        .map(|(_, n, _)| n)
        .collect();
    assert_eq!(top, ["by-date", "by-job-id", "by-source"]);

    // Drill into by-job-id/1/archive-copy.
    let by_job_id = map.lookup(ROOT_INODE, "by-job-id").expect("by-job-id");
    let job_one = map.lookup(by_job_id, "1").expect("job 1");
    let leaf = map.lookup(job_one, "archive-copy").expect("leaf");
    let leaf_entry = map.get(leaf).expect("leaf entry");
    assert!(matches!(
        leaf_entry.kind,
        NodeKind::JobPlaceholder { job_row_id: 1 }
    ));
}
