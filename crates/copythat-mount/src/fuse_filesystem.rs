//! Phase 33d — inode map layer for the real FUSE / WinFsp backends.
//!
//! FUSE identifies entries by integer inode rather than path.
//! `TreeInodeMap::from_tree` walks a [`crate::MountTree`] and assigns
//! a unique `u64` inode to every node, plus records each node's
//! parent + name + [`NodeKind`] so the platform backends can answer
//! `lookup` / `getattr` / `readdir` / `read` callbacks in O(1)
//! hashmap ops.
//!
//! Everything in this module is pure-Rust and platform-agnostic —
//! no `fuser`, no `winfsp-sys`. The body-fill lands in the
//! platform backends (Phase 33e for the validated kernel wiring);
//! this module is the contract those impls will read from.
//!
//! ```text
//! inode 1 ── ROOT ("")
//!   inode 2 ── by-date/          (Directory)
//!     inode 3 ── 2026-04-21/     (Directory)
//!       inode 4 ── 14-32-15/     (Directory)
//!         inode 5 ── archive-copy (JobPlaceholder)
//!   inode 6 ── by-source/        (Directory)
//!     ...
//! ```

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tree::{MountNode, MountTree, NodeKind};

/// Root inode — matches the FUSE / POSIX convention. Every child
/// inode counts up from 2.
pub const ROOT_INODE: u64 = 1;

/// Per-inode record the FUSE / WinFsp callbacks consult. Stays
/// plain-data so the whole map can be cheaply cloned across the
/// platform-specific backend + a read-side request handler.
#[derive(Debug, Clone, PartialEq)]
pub struct InodeEntry {
    /// Parent inode. [`ROOT_INODE`] for the root itself so the
    /// invariant "root is its own parent" matches POSIX semantics.
    pub parent: u64,
    /// Name within the parent directory. Empty string for the root.
    pub name: String,
    /// What the node represents. Drives the `FileType` that gets
    /// returned to FUSE (Directory vs RegularFile).
    pub kind: NodeKind,
    /// Children keyed by display name → their inode. Empty on leaf
    /// nodes.
    pub children: HashMap<String, u64>,
}

/// Assigns and stores inodes for every node in a [`MountTree`].
#[derive(Debug, Clone, Default)]
pub struct TreeInodeMap {
    entries: HashMap<u64, InodeEntry>,
    next_inode: u64,
}

impl TreeInodeMap {
    /// Build from a reference to the live tree. The map is a
    /// snapshot — a later [`MountTree`] mutation invalidates it and
    /// the caller must rebuild.
    pub fn from_tree(tree: &MountTree) -> Self {
        let mut this = TreeInodeMap {
            entries: HashMap::new(),
            next_inode: ROOT_INODE + 1,
        };
        this.entries.insert(
            ROOT_INODE,
            InodeEntry {
                parent: ROOT_INODE,
                name: String::new(),
                kind: NodeKind::Directory,
                children: HashMap::new(),
            },
        );
        this.insert_children(ROOT_INODE, &tree.root);
        this
    }

    fn insert_children(&mut self, parent_ino: u64, parent_node: &MountNode) {
        // `BTreeMap` iteration is already sorted → deterministic
        // inode assignment.
        for (name, child) in &parent_node.children {
            let ino = self.next_inode;
            self.next_inode += 1;
            // Record the child entry before recursing so nested
            // children can reference it.
            self.entries.insert(
                ino,
                InodeEntry {
                    parent: parent_ino,
                    name: name.clone(),
                    kind: child.kind.clone(),
                    children: HashMap::new(),
                },
            );
            // Link the child into its parent's children table.
            if let Some(parent_entry) = self.entries.get_mut(&parent_ino) {
                parent_entry.children.insert(name.clone(), ino);
            }
            self.insert_children(ino, child);
        }
    }

    /// Look up an inode record. `None` for an invalid / stale inode.
    pub fn get(&self, inode: u64) -> Option<&InodeEntry> {
        self.entries.get(&inode)
    }

    /// Find a child inode by `(parent, name)` — how FUSE resolves a
    /// `lookup` call. `None` when the parent doesn't have a child
    /// with that name.
    pub fn lookup(&self, parent_inode: u64, name: &str) -> Option<u64> {
        self.entries
            .get(&parent_inode)
            .and_then(|parent| parent.children.get(name).copied())
    }

    /// List direct children of a directory inode. Returns sorted
    /// `(inode, name, kind)` triples so platform readdir callbacks
    /// can stream them in a stable order matching `ls -la`.
    pub fn readdir(&self, parent_inode: u64) -> Vec<(u64, String, NodeKind)> {
        let Some(parent) = self.entries.get(&parent_inode) else {
            return Vec::new();
        };
        let mut entries: Vec<(u64, String, NodeKind)> = parent
            .children
            .iter()
            .filter_map(|(name, ino)| {
                self.entries
                    .get(ino)
                    .map(|e| (*ino, name.clone(), e.kind.clone()))
            })
            .collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        entries
    }

    /// Total inode count including the root.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ---------------------------------------------------------------------
// Phase 33e — `MountFileAttr` synthesizer
// ---------------------------------------------------------------------
//
// A real `fuser::Filesystem::getattr` callback returns a `FileAttr`
// with ~12 fields (ino, size, blocks, atime/mtime/ctime, kind,
// perm, nlink, uid, gid, rdev, blksize). WinFsp's equivalent table
// fills a `FspFileInfo` struct with similar semantics. This
// `MountFileAttr` is the platform-neutral projection those two
// callback families share — a pure-data struct that backends map
// into their native shape at send-time.

/// POSIX-shaped file kind. Kept minimal (dir vs regular) because
/// the mount exposes only virtual directories + job-placeholder
/// leaves. Symlinks / block-devices / char-devices never appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountFileKind {
    Directory,
    RegularFile,
}

/// Platform-neutral stat record a FUSE/WinFsp `getattr` callback
/// can translate 1:1 into its native `FileAttr` / `FspFileInfo`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountFileAttr {
    pub ino: u64,
    /// Apparent size in bytes. Directories report 0. Job-placeholder
    /// leaves report 0 until Phase 33f wires chunk-store streaming,
    /// at which point `size` reflects the aggregate
    /// `History::items_for(job_row_id)` byte-sum.
    pub size: u64,
    pub kind: MountFileKind,
    /// POSIX permission bits (e.g. `0o555` for read-only dirs +
    /// `0o444` for read-only files — the mount is read-only at
    /// every layer).
    pub perm: u16,
    /// Hard-link count. `2 + <subdir count>` for directories
    /// (following the `.` + `..` + subdirs convention); `1` for
    /// files.
    pub nlink: u32,
    /// Last-modified wall-clock seconds since the Unix epoch.
    /// Phase 33e seeds this to a deterministic per-mount sentinel
    /// (the mount's creation moment); Phase 33f will back this with
    /// the job's `finished_at_ms` from the history row.
    pub mtime_unix_secs: i64,
}

impl MountFileAttr {
    pub fn kind_is_dir(&self) -> bool {
        matches!(self.kind, MountFileKind::Directory)
    }
}

/// Synthesize a [`MountFileAttr`] for the given inode. `mount_mtime_unix_secs`
/// is the fallback timestamp the callback stamps onto every entry —
/// typically `SystemTime::now()` at mount-creation time so
/// successive `getattr` calls return stable values during the
/// session.
///
/// Returns `None` when the inode isn't registered in this map. Real
/// FUSE/WinFsp callbacks map that to `ENOENT` (FUSE) /
/// `STATUS_OBJECT_NAME_NOT_FOUND` (WinFsp).
pub fn synthesize_attr(
    map: &TreeInodeMap,
    inode: u64,
    mount_mtime_unix_secs: i64,
) -> Option<MountFileAttr> {
    let entry = map.get(inode)?;
    Some(match &entry.kind {
        NodeKind::Directory => {
            let child_count = entry.children.len() as u32;
            MountFileAttr {
                ino: inode,
                size: 0,
                kind: MountFileKind::Directory,
                perm: 0o555,
                // `.` + `..` + one per subdir — standard POSIX
                // convention.
                nlink: 2 + child_count,
                mtime_unix_secs: mount_mtime_unix_secs,
            }
        }
        NodeKind::JobPlaceholder { .. } => MountFileAttr {
            ino: inode,
            // 33e: stub out at zero bytes. 33f's chunk-streaming
            // read path populates this from `History::items_for`.
            size: 0,
            kind: MountFileKind::RegularFile,
            perm: 0o444,
            nlink: 1,
            mtime_unix_secs: mount_mtime_unix_secs,
        },
    })
}

/// Convenience — `SystemTime::now()` as Unix seconds. Used as the
/// default `mount_mtime` in [`synthesize_attr`] when the caller
/// doesn't have a more specific timestamp.
pub fn now_unix_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{MountLayout, build_from_rows};
    use std::path::PathBuf;

    fn seeded_tree() -> MountTree {
        build_from_rows(
            &[
                (
                    1,
                    PathBuf::from("/a"),
                    PathBuf::from("/dst/x"),
                    1_776_781_935_000_i64,
                    "copy".into(),
                ),
                (
                    2,
                    PathBuf::from("/b"),
                    PathBuf::from("/dst/y"),
                    1_776_781_936_000_i64,
                    "copy".into(),
                ),
            ],
            MountLayout::all(),
        )
    }

    #[test]
    fn root_inode_is_1_and_self_parent() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let root = map.get(ROOT_INODE).expect("root present");
        assert_eq!(root.parent, ROOT_INODE);
        assert_eq!(root.name, "");
        assert_eq!(root.kind, NodeKind::Directory);
        assert_eq!(root.children.len(), 3, "by-date + by-source + by-job-id");
    }

    #[test]
    fn inodes_are_unique_and_dense() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        // Descendant count is 16 under MountLayout::all() for this
        // two-job seed; plus the root = 17 entries total.
        assert_eq!(map.len(), 17);
        let mut seen: Vec<u64> = map.entries.keys().copied().collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), map.len(), "inodes are unique");
    }

    #[test]
    fn lookup_resolves_by_name() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let by_date = map.lookup(ROOT_INODE, "by-date").expect("by-date");
        let by_source = map.lookup(ROOT_INODE, "by-source").expect("by-source");
        let by_job_id = map.lookup(ROOT_INODE, "by-job-id").expect("by-job-id");

        // All three top-level dirs are distinct inodes.
        assert_ne!(by_date, by_source);
        assert_ne!(by_source, by_job_id);
        assert_ne!(by_date, by_job_id);

        // Descend: by-job-id/1/x-copy.
        let one = map.lookup(by_job_id, "1").expect("job-id/1");
        let leaf = map.lookup(one, "x-copy").expect("x-copy leaf");
        let leaf_entry = map.get(leaf).expect("leaf entry");
        assert!(matches!(
            leaf_entry.kind,
            NodeKind::JobPlaceholder { job_row_id: 1 }
        ));
    }

    #[test]
    fn lookup_returns_none_for_missing_name() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        assert!(map.lookup(ROOT_INODE, "does-not-exist").is_none());
    }

    #[test]
    fn readdir_is_sorted() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let names: Vec<String> = map
            .readdir(ROOT_INODE)
            .into_iter()
            .map(|(_, n, _)| n)
            .collect();
        assert_eq!(names, ["by-date", "by-job-id", "by-source"]);
    }

    #[test]
    fn readdir_on_missing_inode_returns_empty() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        assert!(map.readdir(99_999).is_empty());
    }

    #[test]
    fn parent_child_round_trip() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let by_date = map.lookup(ROOT_INODE, "by-date").expect("by-date");
        let entry = map.get(by_date).expect("entry");
        assert_eq!(entry.parent, ROOT_INODE);
        assert_eq!(entry.name, "by-date");
        // Drill down: by-date/2026-04-21/14-32-15/archive-copy.
        let date_dir = map.lookup(by_date, "2026-04-21").expect("date dir");
        let date_entry = map.get(date_dir).expect("date entry");
        assert_eq!(date_entry.parent, by_date);
    }

    #[test]
    fn empty_tree_has_only_root() {
        let tree = build_from_rows(&[], MountLayout::all());
        let map = TreeInodeMap::from_tree(&tree);
        assert_eq!(map.len(), 1);
        assert!(map.readdir(ROOT_INODE).is_empty());
    }

    #[test]
    fn synthesize_attr_returns_none_for_missing_inode() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        assert!(synthesize_attr(&map, 99_999, 1_776_781_935).is_none());
    }

    #[test]
    fn synthesize_attr_marks_root_as_readonly_dir_with_nlink() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let root_attr = synthesize_attr(&map, ROOT_INODE, 1_776_781_935).expect("root");
        assert_eq!(root_attr.ino, ROOT_INODE);
        assert_eq!(root_attr.size, 0);
        assert_eq!(root_attr.kind, MountFileKind::Directory);
        assert_eq!(root_attr.perm, 0o555);
        // Root has 3 subdirs (by-date / by-source / by-job-id), so
        // nlink is 2 + 3 = 5.
        assert_eq!(root_attr.nlink, 5);
        assert_eq!(root_attr.mtime_unix_secs, 1_776_781_935);
        assert!(root_attr.kind_is_dir());
    }

    #[test]
    fn synthesize_attr_marks_job_placeholder_as_readonly_file() {
        let tree = seeded_tree();
        let map = TreeInodeMap::from_tree(&tree);
        let by_job_id = map.lookup(ROOT_INODE, "by-job-id").expect("by-job-id");
        let one = map.lookup(by_job_id, "1").expect("1");
        let leaf = map.lookup(one, "x-copy").expect("x-copy");
        let attr = synthesize_attr(&map, leaf, 1_776_781_935).expect("leaf attr");
        assert_eq!(attr.ino, leaf);
        assert_eq!(attr.kind, MountFileKind::RegularFile);
        assert_eq!(attr.perm, 0o444);
        assert_eq!(attr.nlink, 1);
        // Size is 0 at 33e — 33f populates from the chunk store.
        assert_eq!(attr.size, 0);
        assert!(!attr.kind_is_dir());
    }

    #[test]
    fn now_unix_secs_is_nonzero_and_monotonic() {
        let a = now_unix_secs();
        let b = now_unix_secs();
        assert!(a > 0 && b > 0);
        assert!(b >= a);
    }
}
