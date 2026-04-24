//! Phase 33c — WinFsp backend scaffolding.
//!
//! Enabled by `--features winfsp` on Windows targets. The feature
//! pulls in `winfsp-sys`, which needs both the WinFsp driver
//! installed on the build machine *and* libclang for `bindgen`.
//! When the feature is off (or on a non-Windows build), this module
//! compiles but [`WinFspBackend::mount`] surfaces
//! `MountError::BackendUnavailable` with a clear reason.
//!
//! Phase 33c ships the type surface + feature plumbing. The real
//! WinFsp file-system callback implementation lands in Phase 33d
//! alongside the `fuser` kernel wiring.

use std::path::Path;

use crate::backends::MountBackend;
use crate::error::MountError;
use crate::handle::MountHandle;
use crate::tree::MountLayout;

/// WinFsp-backed mount. Pair of [`crate::FuseBackend`] on Windows.
#[derive(Debug, Default)]
pub struct WinFspBackend {
    _phantom: (),
}

impl WinFspBackend {
    pub fn new() -> Result<Self, MountError> {
        #[cfg(all(feature = "winfsp", target_os = "windows"))]
        {
            Ok(Self { _phantom: () })
        }
        #[cfg(not(all(feature = "winfsp", target_os = "windows")))]
        {
            Err(MountError::BackendUnavailable(
                "winfsp feature not enabled or not on Windows".into(),
            ))
        }
    }
}

impl MountBackend for WinFspBackend {
    fn mount(
        &self,
        mountpoint: &Path,
        layout: MountLayout,
        archive: &crate::backends::ArchiveRefs,
    ) -> Result<MountHandle, MountError> {
        #[cfg(all(feature = "winfsp", target_os = "windows"))]
        {
            winfsp_body::mount_with_winfsp(mountpoint, layout, archive.clone())
        }
        #[cfg(not(all(feature = "winfsp", target_os = "windows")))]
        {
            let _ = (mountpoint, layout, archive);
            Err(MountError::BackendUnavailable(
                "winfsp feature not enabled on this build".into(),
            ))
        }
    }
}

#[cfg(all(feature = "winfsp", target_os = "windows"))]
mod winfsp_body {
    //! Phase 33i — WinFsp FileSystemInterface minimum overrides.
    //!
    //! Registers a `winfsp_wrs::FileSystem` via the lower-level
    //! `FspFileSystemCreate` + `FspFileSystemSetMountPoint` +
    //! `FspFileSystemStartDispatcher` chain, implementing the
    //! minimum override set (`get_volume_info`,
    //! `get_security_by_name`, `open`, `close`, `read`,
    //! `read_directory`) consulting `TreeInodeMap` +
    //! `ArchiveRefs` — analogous to the Phase 33g fuse body.
    //!
    //! **Validation**: the impl is compile-validated on Windows
    //! with `--features winfsp` (requires WinFsp SDK install +
    //! LLVM libclang — see `docs/ROADMAP.md` USER ACTION notes).
    //! Actual mount operation requires admin privileges + a free
    //! drive letter + the WinFsp driver running. The
    //! platform-gated smoke test
    //! `tests/smoke/phase_33h_mount.rs` case 2 is what exercises
    //! the live path once those are set up.
    //!
    //! **Deferred to Phase 34-ish**: write paths (create /
    //! overwrite / rename / delete). Copy That's mount surface is
    //! read-only by design: users browse snapshots, not modify
    //! them. Write-through to the chunk store + history would
    //! need a full re-architecture that's a separate milestone.

    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;
    use std::sync::Arc;

    use widestring::{U16CStr, U16CString, U16String};
    use winfsp_wrs::{
        CreateOptions, DirInfo, FileAccessRights, FileAttributes, FileInfo, FileSystem,
        FileSystemInterface, NTSTATUS, PSecurityDescriptor, Params, SecurityDescriptor, VolumeInfo,
    };

    use crate::backends::ArchiveRefs;
    use crate::error::MountError;
    use crate::fuse_filesystem::{ROOT_INODE, TreeInodeMap, now_unix_secs, synthesize_attr};
    use crate::handle::MountHandle;
    use crate::tree::{MountLayout, MountTree};

    /// File-or-directory context. WinFsp asks us to identify each
    /// open handle with a context the next callback receives back;
    /// we use the inode number cast to `usize` (winfsp_wrs's
    /// `FileContextKind::MODE::Node` form).
    type FileCtx = usize;

    /// Minimum Windows error: STATUS_OBJECT_NAME_NOT_FOUND.
    /// winfsp_wrs's `NTSTATUS` is the raw numeric code, not a
    /// macro-friendly enum; we define the handful we actually
    /// return here.
    const STATUS_OBJECT_NAME_NOT_FOUND: NTSTATUS = 0xC0000034u32 as i32;
    const STATUS_NOT_IMPLEMENTED: NTSTATUS = 0xC0000002u32 as i32;
    const STATUS_END_OF_FILE: NTSTATUS = 0xC0000011u32 as i32;

    pub(super) struct CopyThatWinFspFs {
        map: Arc<TreeInodeMap>,
        history: Option<Arc<copythat_history::History>>,
        chunk_store: Option<Arc<copythat_chunk::ChunkStore>>,
        mount_mtime: i64,
        // Default SD — "everyone allow read" — used for every
        // entry. Computed once at build time so every callback
        // clones cheaply.
        default_sd: SecurityDescriptor,
    }

    impl CopyThatWinFspFs {
        fn new(
            map: Arc<TreeInodeMap>,
            history: Option<Arc<copythat_history::History>>,
            chunk_store: Option<Arc<copythat_chunk::ChunkStore>>,
        ) -> Result<Self, MountError> {
            // SDDL: protected DACL granting generic-read to
            // everyone (`WD`). Sufficient for a read-only snapshot
            // view.
            let sddl = U16CString::from_str("D:P(A;;GR;;;WD)")
                .map_err(|e| MountError::BackendUnavailable(format!("sddl: {e}")))?;
            let default_sd = SecurityDescriptor::from_wstr(&sddl).map_err(|e| {
                MountError::BackendUnavailable(format!("parse default sd: {e}"))
            })?;
            Ok(Self {
                map,
                history,
                chunk_store,
                mount_mtime: now_unix_secs(),
                default_sd,
            })
        }

        /// Translate a WinFsp-style leading-backslash path
        /// (e.g. `\by-date\2026-04-21`) into an inode by walking
        /// the tree. Root is `\`. Returns `usize` matching the
        /// winfsp `FileContextKind::Node` mode.
        fn lookup_by_path(&self, wide: &U16CStr) -> Option<usize> {
            let s = wide.to_string_lossy();
            let normalized = s.replace('\\', "/");
            let trimmed = normalized.trim_start_matches('/');
            if trimmed.is_empty() {
                return Some(ROOT_INODE as usize);
            }
            let mut cursor: u64 = ROOT_INODE;
            for segment in trimmed.split('/') {
                if segment.is_empty() {
                    continue;
                }
                cursor = self.map.lookup(cursor, segment)?;
            }
            Some(cursor as usize)
        }

        /// Convert our `MountFileAttr` into a winfsp_wrs `FileInfo`.
        fn file_info_for(&self, inode: usize) -> Option<FileInfo> {
            let inode = inode as u64;
            let attr = synthesize_attr(&self.map, inode, self.mount_mtime)?;
            let mut info = FileInfo::default();
            // FILETIME is 100ns ticks since 1601-01-01. Unix
            // epoch is 11644473600 seconds after 1601; multiply
            // the unix seconds + shift.
            const EPOCH_OFFSET: u64 = 11_644_473_600;
            let filetime = (attr.mtime_unix_secs.max(0) as u64 + EPOCH_OFFSET) * 10_000_000;
            info.set_file_size(attr.size);
            info.set_allocation_size(((attr.size + 511) / 512) * 512);
            info.set_hard_links(attr.nlink);
            info.set_index_number(attr.ino);
            info.set_time(filetime);
            let attrs = if attr.kind_is_dir() {
                FileAttributes::DIRECTORY
            } else {
                FileAttributes::READONLY
            };
            info.set_file_attributes(attrs);
            Some(info)
        }
    }

    impl FileSystemInterface for CopyThatWinFspFs {
        type FileContext = FileCtx;

        // Enable only the callbacks we actually override — the
        // const-generic dispatch pattern from winfsp_wrs skips
        // un-overridden methods at runtime.
        const GET_VOLUME_INFO_DEFINED: bool = true;
        const GET_SECURITY_BY_NAME_DEFINED: bool = true;
        const OPEN_DEFINED: bool = true;
        const CLOSE_DEFINED: bool = true;
        const READ_DEFINED: bool = true;
        const READ_DIRECTORY_DEFINED: bool = true;
        const GET_FILE_INFO_DEFINED: bool = true;

        fn get_volume_info(&self) -> Result<VolumeInfo, NTSTATUS> {
            let label = U16String::from_str("CopyThat");
            VolumeInfo::new(u64::MAX, u64::MAX, &label)
                .map_err(|_| STATUS_NOT_IMPLEMENTED)
        }

        fn get_security_by_name(
            &self,
            file_name: &U16CStr,
            _find_reparse_point: impl Fn() -> Option<FileAttributes>,
        ) -> Result<(FileAttributes, PSecurityDescriptor, bool), NTSTATUS> {
            let ino = self
                .lookup_by_path(file_name)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
            let attr = synthesize_attr(&self.map, ino as u64, self.mount_mtime)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
            let attrs = if attr.kind_is_dir() {
                FileAttributes::DIRECTORY
            } else {
                FileAttributes::READONLY
            };
            let sd: PSecurityDescriptor = (&self.default_sd).into();
            Ok((attrs, sd, false))
        }

        fn open(
            &self,
            file_name: &U16CStr,
            _create_options: CreateOptions,
            _granted_access: FileAccessRights,
        ) -> Result<(Self::FileContext, FileInfo), NTSTATUS> {
            let ino = self
                .lookup_by_path(file_name)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
            let info = self
                .file_info_for(ino)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
            Ok((ino, info))
        }

        fn close(&self, _file_context: Self::FileContext) {
            // No resources to release — FileContext is a u64
            // inode, allocated on the stack.
        }

        fn get_file_info(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
            self.file_info_for(file_context)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)
        }

        fn read(
            &self,
            file_context: Self::FileContext,
            buffer: &mut [u8],
            offset: u64,
        ) -> Result<usize, NTSTATUS> {
            let entry = self
                .map
                .get(file_context as u64)
                .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
            let crate::NodeKind::JobPlaceholder { job_row_id } = &entry.kind else {
                return Err(STATUS_NOT_IMPLEMENTED);
            };
            let (Some(history), Some(chunk_store)) = (
                self.history.as_ref(),
                self.chunk_store.as_ref(),
            ) else {
                // Read without archive refs (test / stubbed mount)
                // reports EOF.
                return Err(STATUS_END_OF_FILE);
            };
            // Reuse the same chunk-streaming helper the FUSE body
            // uses. The WinFsp side mirrors the read semantics.
            let bytes = winfsp_read_bytes(history, chunk_store, *job_row_id, offset, buffer.len())
                .ok_or(STATUS_END_OF_FILE)?;
            let n = bytes.len().min(buffer.len());
            buffer[..n].copy_from_slice(&bytes[..n]);
            if n == 0 {
                return Err(STATUS_END_OF_FILE);
            }
            Ok(n)
        }

        fn read_directory(
            &self,
            file_context: Self::FileContext,
            marker: Option<&U16CStr>,
            mut add_dir_info: impl FnMut(DirInfo) -> bool,
        ) -> Result<(), NTSTATUS> {
            let entries = self.map.readdir(file_context as u64);
            // WinFsp passes `marker` = the last-seen name in the
            // previous batch; skip until we're past it.
            let marker_str = marker.map(|m| m.to_string_lossy()).unwrap_or_default();
            let mut past_marker = marker_str.is_empty();
            for (child_ino, name, _kind) in entries {
                if !past_marker {
                    if name == marker_str {
                        past_marker = true;
                    }
                    continue;
                }
                let Some(info) = self.file_info_for(child_ino as usize) else {
                    continue;
                };
                let wide =
                    U16CString::from_str(&name).map_err(|_| STATUS_OBJECT_NAME_NOT_FOUND)?;
                let dir_info = DirInfo::new(info, &wide);
                if !add_dir_info(dir_info) {
                    // Buffer full — WinFsp will call again with
                    // `marker = name`.
                    break;
                }
            }
            Ok(())
        }
    }

    /// Phase 33i — chunk-streaming read for the WinFsp side. Walks
    /// the job's manifest chunks in order, materialising only the
    /// subset overlapping `[offset, offset+max_len)`. Mirrors the
    /// Phase 33g FUSE `assemble_job_bytes` helper.
    fn winfsp_read_bytes(
        history: &Arc<copythat_history::History>,
        chunk_store: &Arc<copythat_chunk::ChunkStore>,
        job_row_id: i64,
        offset: u64,
        max_len: usize,
    ) -> Option<Vec<u8>> {
        use ::tokio::runtime::Builder;
        let rt = Builder::new_current_thread().enable_all().build().ok()?;
        let history = history.clone();
        let items = rt
            .block_on(
                async move { history.items_for(copythat_history::JobRowId(job_row_id)).await },
            )
            .ok()?;
        let read_end = offset.saturating_add(max_len as u64);
        let mut out: Vec<u8> = Vec::new();
        let mut cursor: u64 = 0;
        for item in &items {
            if cursor >= read_end {
                break;
            }
            let manifest_key = item.src.to_string_lossy().into_owned();
            let Ok(Some(manifest)) = chunk_store.get_manifest(&manifest_key) else {
                continue;
            };
            for chunk_ref in &manifest.chunks {
                let chunk_start = cursor;
                let chunk_end = cursor.saturating_add(chunk_ref.len as u64);
                cursor = chunk_end;
                if chunk_end <= offset || chunk_start >= read_end {
                    continue;
                }
                let Ok(Some(bytes)) = chunk_store.get(&chunk_ref.hash) else {
                    continue;
                };
                let slice_from = offset.saturating_sub(chunk_start) as usize;
                let slice_to = (read_end - chunk_start).min(bytes.len() as u64) as usize;
                if slice_from < slice_to && slice_from < bytes.len() {
                    out.extend_from_slice(&bytes[slice_from..slice_to]);
                }
                if cursor >= read_end {
                    break;
                }
            }
        }
        Some(out)
    }

    pub(super) fn mount_with_winfsp(
        mountpoint: &Path,
        layout: MountLayout,
        archive: ArchiveRefs,
    ) -> Result<MountHandle, MountError> {
        crate::backends::validate_mountpoint(mountpoint)?;
        // Pre-fetch history jobs via a block_on scratch runtime
        // (mirrors the fuse body's approach).
        let jobs = if let Some(history) = archive.history.as_ref() {
            use ::tokio::runtime::Builder;
            Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()
                .and_then(|rt| {
                    rt.block_on(async {
                        let filter = copythat_history::HistoryFilter {
                            status: Some("succeeded".into()),
                            ..Default::default()
                        };
                        history.search(filter).await.ok()
                    })
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let chunk_placeholder = archive
            .chunk_store
            .clone()
            .unwrap_or_else(|| Arc::new(chunk_placeholder_fs()));
        let tree = MountTree::build(&jobs, chunk_placeholder.as_ref(), layout).unwrap_or_else(
            |_| MountTree {
                root: crate::MountNode {
                    name: String::new(),
                    kind: crate::NodeKind::Directory,
                    children: Default::default(),
                },
                layout,
                job_count: 0,
            },
        );
        let map = Arc::new(TreeInodeMap::from_tree(&tree));
        let fs = CopyThatWinFspFs::new(map, archive.history, archive.chunk_store)?;

        // Build VolumeParams. Defaults are fine for a read-only
        // mount; winfsp_wrs fills in sector sizes, label length,
        // etc.
        let mut params = Params::default();
        params.volume_params.set_case_sensitive_search(false);
        params.volume_params.set_reparse_point(false);
        params.volume_params.set_unicode_on_disk(true);
        params.volume_params.set_read_only_volume(true);
        params
            .volume_params
            .set_volume_serial_number(0xC0FF_EE00u32);
        params
            .volume_params
            .set_file_info_timeout(1000 /* ms */);

        // Compose the mountpoint as a wide string. Typical form:
        // a drive letter like `Z:` or an NTFS directory.
        let mp_utf16: Vec<u16> = OsStr::new(mountpoint)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let mp_wide = U16CString::from_vec_truncate(mp_utf16);

        // Start the dispatcher. Ownership of `fs` moves into the
        // trampoline; the returned `FileSystem` is what we hold
        // for the session lifetime.
        let live = FileSystem::start(params, Some(&mp_wide), fs)
            .map_err(|nts| MountError::BackendUnavailable(format!("FspFileSystemStart: {nts}")))?;

        let session: Box<dyn crate::backends::MountSession> = Box::new(WinFspSession {
            fs: Some(live),
        });
        Ok(MountHandle::new(mountpoint.to_path_buf(), session))
    }

    struct WinFspSession {
        fs: Option<FileSystem>,
    }

    impl crate::backends::MountSession for WinFspSession {
        fn unmount_on_drop(&mut self) -> Result<(), MountError> {
            if let Some(fs) = self.fs.take() {
                fs.stop();
            }
            Ok(())
        }
    }

    /// An empty ChunkStore placeholder keeps the tree builder
    /// + winfsp initializer off the hot-path `Option<Arc<_>>`
    /// hunts at every callback.
    fn chunk_placeholder_fs() -> copythat_chunk::ChunkStore {
        let dir = std::env::temp_dir().join(format!(
            "copythat-winfsp-empty-{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        copythat_chunk::ChunkStore::open(&dir).expect("empty winfsp chunk store open")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On the default build (no `winfsp` feature) `mount` returns
    /// `BackendUnavailable`. On a `--features winfsp` build on
    /// Windows, `mount` actually tries to register a WinFsp
    /// file-system which requires a mountable drive letter +
    /// admin; that validation belongs to the platform-gated
    /// Phase 33h smoke rather than this unit test.
    #[test]
    #[cfg(not(all(feature = "winfsp", target_os = "windows")))]
    fn mount_on_default_build_reports_backend_unavailable() {
        let backend = WinFspBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = backend
            .mount(
                tmp.path(),
                MountLayout::all(),
                &crate::backends::ArchiveRefs::default(),
            )
            .expect_err("default build has no winfsp body");
        assert!(matches!(err, MountError::BackendUnavailable(_)));
    }
}
