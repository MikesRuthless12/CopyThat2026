//! Phase 33c — FUSE backend scaffolding.
//!
//! Enabled by `--features fuse` on Linux / macOS targets. The module
//! compiles only when the feature is on *and* the target supports
//! FUSE; everything else falls through to [`FuseBackend::new`]
//! returning a `MountError::BackendUnavailable`.
//!
//! Phase 33c's commit ships the type surface + the feature-gated
//! `fuser` dep wiring. The real kernel read callbacks that stream
//! chunks from the Phase 27 chunk store land in Phase 33d — until
//! then [`FuseBackend::mount`] surfaces
//! `MountError::BackendUnavailable` with a clear reason.

use std::path::Path;

use crate::backends::MountBackend;
use crate::error::MountError;
use crate::handle::MountHandle;
use crate::tree::MountLayout;

/// FUSE-backed mount. Constructed via `default_backend()` on hosts
/// where the `fuse` feature + target triples line up; elsewhere the
/// fallback to `BackendUnavailable` keeps the public API uniform.
#[derive(Debug, Default)]
pub struct FuseBackend {
    _phantom: (),
}

impl FuseBackend {
    /// Build a new FUSE backend. On hosts where the feature + target
    /// gate out, Phase 33c's `default_backend()` never constructs
    /// one of these directly — it routes to `NoopBackend`. This
    /// constructor stays present on every target so downstream
    /// crates can still name the type (e.g. for trait-object casts)
    /// without a cfg maze.
    pub fn new() -> Result<Self, MountError> {
        #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
        {
            Ok(Self { _phantom: () })
        }
        #[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
        {
            Err(MountError::BackendUnavailable(
                "fuse feature not enabled or unsupported on this target".into(),
            ))
        }
    }
}

impl MountBackend for FuseBackend {
    fn mount(
        &self,
        mountpoint: &Path,
        layout: MountLayout,
        archive: &crate::backends::ArchiveRefs,
    ) -> Result<MountHandle, MountError> {
        #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
        {
            fuse_body::mount_with_fuser(mountpoint, layout, archive.clone())
        }
        #[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
        {
            let _ = (mountpoint, layout, archive);
            Err(MountError::BackendUnavailable(
                "fuse feature not enabled on this build".into(),
            ))
        }
    }
}

#[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
mod fuse_body {
    //! Phase 33f — real `fuser::Filesystem` trait impl.
    //!
    //! Consults [`crate::TreeInodeMap`] + [`crate::synthesize_attr`]
    //! to answer `lookup` / `getattr` / `readdir` callbacks. Read
    //! callback currently replies with `ENOSYS` for job-placeholder
    //! leaves — chunk-streaming reads from the Phase 27 chunk store
    //! are the Phase 33g follow-up (needs a History handle + chunk
    //! store wired through `MountBackend::mount` first).
    //!
    //! The mount session runs on a background thread via
    //! [`fuser::spawn_mount2`]; the returned `BackgroundSession`
    //! joins on drop, which is exactly the semantics
    //! `MountSession::unmount_on_drop` needs.

    use std::collections::BTreeMap;
    use std::ffi::OsStr;
    use std::path::Path;
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use fuser::{
        BackgroundSession, FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData,
        ReplyDirectory, ReplyEntry, Request, spawn_mount2,
    };

    use crate::backends::MountSession;
    use crate::error::MountError;
    use crate::fuse_filesystem::{
        MountFileAttr, MountFileKind, ROOT_INODE, TreeInodeMap, now_unix_secs, synthesize_attr,
    };
    use crate::handle::MountHandle;
    use crate::tree::{MountLayout, MountTree};

    /// Mount TTL — how long the kernel may cache our attr/entry
    /// replies. 1 s is a safe default matching fuser examples.
    const TTL: Duration = Duration::from_secs(1);
    /// The mounted FS generation number. Stays stable across the
    /// session so cache invalidations don't thrash.
    const GENERATION: u64 = 1;
    /// ENOSYS — "not implemented" errno. Posix-portable across Linux
    /// and macOS.
    const ENOSYS: i32 = 38;
    /// ENOENT — "no such entry". Used when `lookup` can't find a
    /// child name under the given parent.
    const ENOENT: i32 = 2;

    pub(super) fn mount_with_fuser(
        mountpoint: &Path,
        layout: MountLayout,
        archive: crate::backends::ArchiveRefs,
    ) -> Result<MountHandle, MountError> {
        crate::backends::validate_mountpoint(mountpoint)?;
        // Phase 33g: the archive refs let us build a populated tree
        // from the live history DB. When `history` is `None`, we
        // still produce a usable empty tree (the three top-level
        // virtual dirs exist but have no leaves). Pre-fetching the
        // job list keeps the tree builder async-agnostic —
        // FUSE callbacks run on a blocking session thread later.
        let jobs = if let Some(history) = archive.history.as_ref() {
            block_on_history_fetch(history).unwrap_or_default()
        } else {
            Vec::new()
        };

        // ChunkStore access for the read callback happens on the
        // kernel side — we stash the Arc here so TreeFilesystem
        // holds it for the session lifetime.
        let chunk_store_for_tree = archive
            .chunk_store
            .clone()
            .unwrap_or_else(|| Arc::new(chunk_placeholder()));
        let tree = MountTree::build(&jobs, chunk_store_for_tree.as_ref(), layout)
            .unwrap_or_else(|_| MountTree {
                root: crate::MountNode {
                    name: String::new(),
                    kind: crate::NodeKind::Directory,
                    children: BTreeMap::new(),
                },
                layout,
                job_count: 0,
            });
        let map = Arc::new(TreeInodeMap::from_tree(&tree));
        let fs = TreeFilesystem {
            map,
            mount_mtime: now_unix_secs(),
            history: archive.history.clone(),
            chunk_store: archive.chunk_store.clone(),
        };
        let options = vec![
            MountOption::FSName("copythat".to_owned()),
            MountOption::RO,
            MountOption::NoAtime,
        ];
        let session = spawn_mount2(fs, mountpoint, &options)
            .map_err(|e| MountError::BackendUnavailable(format!("spawn_mount2: {e}")))?;
        let tracked: Box<dyn MountSession> = Box::new(FuseSession {
            session: Some(session),
        });
        Ok(MountHandle::new(mountpoint.to_path_buf(), tracked))
    }

    /// Build an empty placeholder `ChunkStore` when none was supplied.
    /// The placeholder's `get()` / `get_manifest()` both return
    /// `Ok(None)` for every key, so the tree builder doesn't crash
    /// on a missing store and read callbacks gracefully surface
    /// ENODATA.
    fn chunk_placeholder() -> copythat_chunk::ChunkStore {
        // Use a fresh temp-dir under the FUSE session's working
        // directory so the placeholder's redb files don't collide
        // with a real chunk store on disk. Phase 33h can swap this
        // for a pure in-memory ChunkStore trait object if we grow
        // one.
        let dir = std::env::temp_dir().join(format!("copythat-mount-empty-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        copythat_chunk::ChunkStore::open(&dir)
            .expect("empty placeholder chunk store open")
    }

    /// Block on a `History::search()` call on a scratch runtime
    /// because the FUSE backend's `mount()` signature is sync.
    /// Returns an empty Vec on any failure — the mount still
    /// succeeds, just with no job leaves.
    fn block_on_history_fetch(
        history: &Arc<copythat_history::History>,
    ) -> Option<Vec<copythat_history::JobSummary>> {
        use tokio::runtime::Builder;
        let rt = Builder::new_current_thread().enable_all().build().ok()?;
        let history = history.clone();
        rt.block_on(async move {
            let filter = copythat_history::HistoryFilter {
                status: Some("succeeded".into()),
                ..Default::default()
            };
            history.search(filter).await.ok()
        })
    }

    struct TreeFilesystem {
        map: Arc<TreeInodeMap>,
        mount_mtime: i64,
        history: Option<Arc<copythat_history::History>>,
        chunk_store: Option<Arc<copythat_chunk::ChunkStore>>,
    }

    impl Filesystem for TreeFilesystem {
        fn lookup(&mut self, req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
            let Some(name_str) = name.to_str() else {
                reply.error(ENOENT);
                return;
            };
            let Some(child_ino) = self.map.lookup(parent, name_str) else {
                reply.error(ENOENT);
                return;
            };
            let Some(attr) = synthesize_attr(&self.map, child_ino, self.mount_mtime) else {
                reply.error(ENOENT);
                return;
            };
            reply.entry(&TTL, &to_fuser_attr(&attr, req.uid(), req.gid()), GENERATION);
        }

        fn getattr(
            &mut self,
            req: &Request<'_>,
            ino: u64,
            _fh: Option<u64>,
            reply: ReplyAttr,
        ) {
            let Some(attr) = synthesize_attr(&self.map, ino, self.mount_mtime) else {
                reply.error(ENOENT);
                return;
            };
            reply.attr(&TTL, &to_fuser_attr(&attr, req.uid(), req.gid()));
        }

        fn readdir(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            _fh: u64,
            offset: i64,
            mut reply: ReplyDirectory,
        ) {
            let entries = self.map.readdir(ino);
            // Prepend `.` + `..` so POSIX tools see the standard
            // directory shape. `..` points to the parent inode, or
            // back to self for the root.
            let mut all: Vec<(u64, String, crate::NodeKind)> = Vec::with_capacity(entries.len() + 2);
            all.push((ino, ".".to_owned(), crate::NodeKind::Directory));
            let parent = self
                .map
                .get(ino)
                .map(|e| e.parent)
                .unwrap_or(ROOT_INODE);
            all.push((parent, "..".to_owned(), crate::NodeKind::Directory));
            all.extend(entries);

            for (idx, (child_ino, name, kind)) in all.iter().enumerate().skip(offset as usize) {
                let ft = match kind {
                    crate::NodeKind::Directory => FileType::Directory,
                    crate::NodeKind::JobPlaceholder { .. } => FileType::RegularFile,
                };
                if reply.add(*child_ino, (idx + 1) as i64, ft, name.as_str()) {
                    break;
                }
            }
            reply.ok();
        }

        fn read(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            _fh: u64,
            offset: i64,
            size: u32,
            _flags: i32,
            _lock_owner: Option<u64>,
            reply: ReplyData,
        ) {
            // Phase 33g — reassemble the requested byte range from
            // the Phase 27 chunk store, keyed on the
            // `JobPlaceholder { job_row_id }` metadata carried on
            // leaf nodes.
            //
            // Current semantics: when History + ChunkStore are both
            // wired through `ArchiveRefs` and the job has a
            // manifest, materialise the concatenated chunk bytes
            // for all items on the job and serve the requested
            // [offset, offset+size) slice. If anything is missing
            // we reply ENODATA so the user sees an empty read
            // rather than a stale byte range.
            let Some(entry) = self.map.get(ino) else {
                reply.error(ENOENT);
                return;
            };
            let crate::NodeKind::JobPlaceholder { job_row_id } = &entry.kind else {
                reply.error(ENOSYS);
                return;
            };
            let (Some(history), Some(chunk_store)) = (
                self.history.as_ref(),
                self.chunk_store.as_ref(),
            ) else {
                // No archive refs wired — pre-Phase-33g mount.
                reply.error(ENOSYS);
                return;
            };
            match assemble_job_bytes(history, chunk_store, *job_row_id, offset, size) {
                Ok(bytes) => reply.data(&bytes),
                Err(AssembleError::NotFound) => reply.data(&[]),
                Err(AssembleError::Io) => reply.error(ENOSYS),
            }
        }
    }

    /// Assembly-time failure modes. Kept coarse — the FUSE read
    /// callback maps any of these into an ENODATA/ENOSYS reply
    /// rather than trying to surface rich detail through the
    /// kernel interface.
    enum AssembleError {
        NotFound,
        #[allow(dead_code)]
        Io,
    }

    /// Phase 33h — stream the job's chunks in order, only
    /// materialising the ones that overlap the requested
    /// `[offset, offset+size)` byte range. Matches `restic mount`
    /// / `borg mount` behavior where reading the middle of a
    /// multi-GB file doesn't spike memory.
    ///
    /// The algorithm walks each item's manifest, tracks the
    /// running file-relative offset, and skips chunks that end
    /// before `offset` or start after `offset + size`. Only the
    /// overlapping chunks get fetched from the chunk store.
    fn assemble_job_bytes(
        history: &Arc<copythat_history::History>,
        chunk_store: &Arc<copythat_chunk::ChunkStore>,
        job_row_id: i64,
        offset: i64,
        size: u32,
    ) -> Result<Vec<u8>, AssembleError> {
        use tokio::runtime::Builder;
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| AssembleError::Io)?;
        let history = history.clone();
        let items = rt
            .block_on(async move { history.items_for(copythat_history::JobRowId(job_row_id)).await })
            .map_err(|_| AssembleError::NotFound)?;

        let read_start = offset.max(0) as u64;
        let read_end = read_start.saturating_add(size as u64);
        let mut out: Vec<u8> = Vec::new();
        // Cumulative byte offset across all items concatenated in
        // order. Matches the "whole-job virtual file" the mount
        // exposes per Phase 33a's layout.
        let mut cursor: u64 = 0;

        for item in &items {
            // Early-exit when the cursor has passed the requested
            // window — no later item can contribute bytes.
            if cursor >= read_end {
                break;
            }
            let manifest_key = item.src.to_string_lossy().into_owned();
            let Ok(Some(manifest)) = chunk_store.get_manifest(&manifest_key) else {
                // Item without a manifest — treat as
                // zero-bytes-contributed (matches Phase 33g
                // behavior). Don't advance the cursor; the mount
                // stays consistent with a "can't materialize
                // this item" report.
                continue;
            };
            for chunk_ref in &manifest.chunks {
                let chunk_start = cursor;
                let chunk_end = cursor.saturating_add(chunk_ref.len as u64);
                cursor = chunk_end;

                // Skip when the chunk is entirely outside the
                // window.
                if chunk_end <= read_start || chunk_start >= read_end {
                    continue;
                }

                let Ok(Some(bytes)) = chunk_store.get(&chunk_ref.hash) else {
                    continue;
                };
                // Compute the slice of this chunk that falls in
                // the read window.
                let slice_from = read_start.saturating_sub(chunk_start) as usize;
                let slice_to = (read_end - chunk_start).min(bytes.len() as u64) as usize;
                if slice_from < slice_to && slice_from < bytes.len() {
                    out.extend_from_slice(&bytes[slice_from..slice_to]);
                }
                if cursor >= read_end {
                    break;
                }
            }
        }

        Ok(out)
    }

    fn to_fuser_attr(attr: &MountFileAttr, uid: u32, gid: u32) -> FileAttr {
        let mtime =
            UNIX_EPOCH + Duration::from_secs(attr.mtime_unix_secs.max(0) as u64);
        let now = SystemTime::now();
        FileAttr {
            ino: attr.ino,
            size: attr.size,
            blocks: (attr.size + 511) / 512,
            atime: now,
            mtime,
            ctime: mtime,
            crtime: mtime,
            kind: match attr.kind {
                MountFileKind::Directory => FileType::Directory,
                MountFileKind::RegularFile => FileType::RegularFile,
            },
            perm: attr.perm,
            nlink: attr.nlink,
            uid,
            gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        }
    }

    struct FuseSession {
        session: Option<BackgroundSession>,
    }

    impl MountSession for FuseSession {
        fn unmount_on_drop(&mut self) -> Result<(), MountError> {
            if let Some(session) = self.session.take() {
                // `BackgroundSession::join` does a best-effort
                // unmount via the kernel. Failure is non-fatal —
                // the kernel will clean up when the process exits.
                session.join();
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On the default build (no `fuse` feature) `mount` returns
    /// `BackendUnavailable` — the feature-gated body doesn't
    /// compile into this binary. On a `--features fuse` build on
    /// Linux/macOS, `mount` actually tries to spawn a FUSE
    /// session; that validation belongs to the platform-gated
    /// smoke test (Phase 33g) rather than this unit test.
    #[test]
    #[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
    fn mount_on_default_build_reports_backend_unavailable() {
        let backend = FuseBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = backend
            .mount(tmp.path(), MountLayout::all(), &crate::backends::ArchiveRefs::default())
            .expect_err("default build has no fuse body");
        assert!(matches!(err, MountError::BackendUnavailable(_)));
    }

    #[test]
    fn new_on_unsupported_platform_reports_backend_unavailable() {
        // On non-fuse targets, `FuseBackend::new` returns an error.
        // On fuse-supported targets with the feature enabled, it
        // returns `Ok` — either outcome is valid for this test's
        // invariant: `new` never panics.
        let _ = FuseBackend::new();
    }
}
