//! Phase 42 — hardlink-set tracking for tree copies.
//!
//! When a tree copy walks `src/` and finds two files that share the
//! same NTFS file-index (same `(VolumeSerialNumber, FileIndex)` triple
//! on Windows; same `(st_dev, st_ino)` on Unix), they are members of a
//! single hardlink set and the destination should preserve that
//! relationship — not copy the bytes twice. Robocopy `/COPYALL` and
//! FastCopy `/linkdest` both do this.
//!
//! This module provides the data structures and the
//! [`HardlinkSet::dispatch`] helper that callers (the engine's tree
//! walk) can use to:
//!
//! 1. Probe a source path's hardlink identity (`identify`).
//! 2. Look up whether they've already copied a member of that set
//!    (`get`).
//! 3. Either issue `CreateHardLinkW` / `link(2)` to point the new
//!    destination at an already-copied file, or fall through to a
//!    real byte copy and remember the destination path for the next
//!    member of the set (`record`).
//!
//! Wiring into the existing `copy_tree` is tracked as a Phase 43
//! follow-up — the engine's tree-walk doesn't currently expose a
//! per-file pre-copy hook for hardlink interception, and adding one
//! cleanly is a larger refactor than the rest of Phase 42 warranted.
//! The scaffolding here is shippable and unit-tested.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Identity of a hardlink-set member. Same identity = same inode
/// (Unix) / same NTFS file index (Windows) — i.e., bytes are stored
/// once on disk and any number of names can resolve to them.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LinkIdentity {
    /// Volume serial (Windows) / `st_dev` (Unix). Disambiguates
    /// inodes that happen to collide across volumes.
    pub volume: u64,
    /// File index high+low packed into u128 on Windows
    /// (`nFileIndexHigh:nFileIndexLow`); `st_ino` zero-extended on
    /// Unix.
    pub file_id: u128,
}

/// Per-tree-copy hardlink ledger. Thread-safe so a future parallel
/// tree walk can call into it from multiple workers.
#[derive(Debug, Default)]
pub struct HardlinkSet {
    inner: Mutex<HashMap<LinkIdentity, PathBuf>>,
}

impl HardlinkSet {
    /// Construct an empty ledger. Equivalent to
    /// `HardlinkSet::default()`; provided as the canonical name so
    /// callers don't have to remember which trait we picked for the
    /// no-arg constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Probe `src`'s identity. Returns `None` for paths that don't
    /// expose an identity (rare — happens on some network paths or
    /// when permission-denied on stat). `Some(LinkIdentity)` with a
    /// `link_count` of 1 is a singleton — callers can skip the
    /// hardlink ledger entirely for those.
    pub fn identify(src: &Path) -> Option<LinkIdentity> {
        identity_impl(src)
    }

    /// Returns the destination already used for a previous member of
    /// the same hardlink set, if any.
    pub fn get(&self, ident: &LinkIdentity) -> Option<PathBuf> {
        // Recover from a poisoned mutex rather than silently bypassing
        // the ledger: a single panic in `record` would otherwise
        // force every subsequent member of every hardlink set to
        // byte-copy, exactly the "store the bytes once" invariant we
        // built this for.
        let guard = self.inner.lock().unwrap_or_else(|e| {
            eprintln!("copythat-platform::hardlink_set: recovering from poisoned ledger (get)");
            e.into_inner()
        });
        guard.get(ident).cloned()
    }

    /// Record that `dst` is the canonical destination for the
    /// hardlink set identified by `ident`. Subsequent `get(ident)`
    /// calls will return `dst`. Idempotent — calling twice with the
    /// same `ident` overwrites with the latest `dst`.
    pub fn record(&self, ident: LinkIdentity, dst: PathBuf) {
        let mut g = self.inner.lock().unwrap_or_else(|e| {
            eprintln!("copythat-platform::hardlink_set: recovering from poisoned ledger (record)");
            e.into_inner()
        });
        g.insert(ident, dst);
    }

    /// Helper for tree-walk callers. If `src` is a member of an
    /// already-copied hardlink set, create a hardlink at `dst`
    /// pointing at the prior destination and return `Ok(true)`.
    /// Otherwise return `Ok(false)` (caller should byte-copy).
    /// Returns `Err(io::Error)` if the link syscall failed for a
    /// reason other than "feature not supported".
    ///
    /// This function does NOT record the dst; call [`record`] after
    /// a successful byte copy when this returns `Ok(false)`.
    pub fn dispatch(&self, src: &Path, dst: &Path) -> io::Result<bool> {
        let Some(ident) = Self::identify(src) else {
            return Ok(false);
        };
        let Some(prior_dst) = self.get(&ident) else {
            return Ok(false);
        };
        // Explicit pre-check: if the recorded prior destination has been
        // deleted between record() and dispatch() (e.g. user undid a
        // previous copy), CreateHardLinkW would surface a clear error
        // anyway, but signalling Ok(false) here lets the caller fall
        // straight back to a fresh byte copy without an io::Error
        // round-trip and keeps the intent obvious.
        if !prior_dst.exists() {
            return Ok(false);
        }
        match create_hardlink(&prior_dst, dst) {
            Ok(()) => Ok(true),
            Err(e) if is_unsupported(&e) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

fn is_unsupported(err: &io::Error) -> bool {
    use io::ErrorKind::*;
    // Cross-volume hardlink, FAT32 dest, network share that doesn't
    // support hardlinks → fall through.
    matches!(err.kind(), Unsupported | InvalidInput | CrossesDevices)
        || matches!(err.raw_os_error(), Some(17) | Some(18) | Some(50))
}

#[cfg(target_os = "windows")]
fn identity_impl(src: &Path) -> Option<LinkIdentity> {
    use std::ffi::OsStr;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, CreateFileW, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
        GetFileInformationByHandle, OPEN_EXISTING,
    };

    let mut wide: Vec<u16> = OsStr::new(src).encode_wide().collect();
    wide.push(0);
    // FILE_FLAG_BACKUP_SEMANTICS: required to open directories.
    // FILE_FLAG_OPEN_REPARSE_POINT: get info on the link itself, not
    // its target — matters for symlink/junction sources.
    // SAFETY: wide is NUL-terminated.
    let h = unsafe {
        CreateFileW(
            wide.as_ptr(),
            0, // no access — info-only
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
            ptr::null_mut(),
        )
    };
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return None;
    }
    struct HGuard(*mut core::ffi::c_void);
    impl Drop for HGuard {
        fn drop(&mut self) {
            // SAFETY: handle obtained from CreateFileW.
            unsafe { CloseHandle(self.0) };
        }
    }
    let _g = HGuard(h);
    let mut info: MaybeUninit<BY_HANDLE_FILE_INFORMATION> = MaybeUninit::zeroed();
    // SAFETY: info is properly sized.
    let ok = unsafe { GetFileInformationByHandle(h, info.as_mut_ptr()) };
    if ok == 0 {
        return None;
    }
    // SAFETY: ok != 0 means info is filled in.
    let info = unsafe { info.assume_init() };
    let file_id = ((info.nFileIndexHigh as u128) << 32) | (info.nFileIndexLow as u128);
    Some(LinkIdentity {
        volume: info.dwVolumeSerialNumber as u64,
        file_id,
    })
}

#[cfg(unix)]
fn identity_impl(src: &Path) -> Option<LinkIdentity> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::symlink_metadata(src).ok()?;
    Some(LinkIdentity {
        volume: m.dev(),
        file_id: m.ino() as u128,
    })
}

#[cfg(not(any(target_os = "windows", unix)))]
fn identity_impl(_src: &Path) -> Option<LinkIdentity> {
    None
}

#[cfg(target_os = "windows")]
fn create_hardlink(existing: &Path, new_link: &Path) -> io::Result<()> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    use windows_sys::Win32::Storage::FileSystem::CreateHardLinkW;

    let mut existing_w: Vec<u16> = OsStr::new(existing).encode_wide().collect();
    existing_w.push(0);
    let mut new_w: Vec<u16> = OsStr::new(new_link).encode_wide().collect();
    new_w.push(0);
    // SAFETY: both buffers are NUL-terminated.
    let ok = unsafe { CreateHardLinkW(new_w.as_ptr(), existing_w.as_ptr(), ptr::null_mut()) };
    if ok != 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(unix)]
fn create_hardlink(existing: &Path, new_link: &Path) -> io::Result<()> {
    std::fs::hard_link(existing, new_link)
}

#[cfg(not(any(target_os = "windows", unix)))]
fn create_hardlink(_existing: &Path, _new_link: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "hardlink not supported on this platform",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn identity_returns_some_on_real_file() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("a.txt");
        fs::write(&p, b"hello").unwrap();
        let id = HardlinkSet::identify(&p);
        assert!(id.is_some());
    }

    #[test]
    fn identity_none_for_missing_path() {
        let p = Path::new("zzz-does-not-exist-zzz");
        assert!(HardlinkSet::identify(p).is_none());
    }

    #[test]
    fn record_then_get_round_trip() {
        let set = HardlinkSet::new();
        let id = LinkIdentity {
            volume: 42,
            file_id: 99,
        };
        let dst = PathBuf::from("/dst/a");
        assert!(set.get(&id).is_none());
        set.record(id, dst.clone());
        assert_eq!(set.get(&id), Some(dst));
    }

    #[test]
    fn dispatch_returns_false_for_unknown_source() {
        let set = HardlinkSet::new();
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        fs::write(&src, b"x").unwrap();
        let dst = dir.path().join("dst.txt");
        assert!(!set.dispatch(&src, &dst).unwrap());
    }

    #[test]
    fn dispatch_creates_hardlink_for_known_source() {
        let set = HardlinkSet::new();
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        fs::write(&src, b"hello").unwrap();
        let id = HardlinkSet::identify(&src).unwrap();
        let prior_dst = dir.path().join("prior.txt");
        fs::write(&prior_dst, b"hello").unwrap();
        set.record(id, prior_dst.clone());

        let new_dst = dir.path().join("new.txt");
        let linked = set.dispatch(&src, &new_dst).unwrap();
        assert!(linked, "expected dispatch to create a hardlink");
        assert!(new_dst.exists());
        // The new path should resolve to the same inode as prior_dst.
        let id_a = HardlinkSet::identify(&prior_dst).unwrap();
        let id_b = HardlinkSet::identify(&new_dst).unwrap();
        assert_eq!(id_a, id_b);
    }

    /// Phase 42 wave-2 — when the prior destination has been deleted
    /// between `record()` and `dispatch()` (e.g. user undid a previous
    /// copy, or a TOCTOU race against tree cleanup), `dispatch` must
    /// return `Ok(false)` to let the caller fall back to a fresh
    /// byte copy — NOT `Err(io::Error)` from a failed
    /// `CreateHardLinkW` / `link(2)` syscall. This is the wave-1
    /// hardening (`!prior_dst.exists()` early-return); pinning the
    /// behaviour here.
    #[test]
    fn dispatch_returns_ok_false_when_prior_dst_deleted() {
        let set = HardlinkSet::new();
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        fs::write(&src, b"hello").unwrap();
        let id = HardlinkSet::identify(&src).unwrap();

        // Record a path that doesn't exist (prior copy was undone).
        let prior_dst = dir.path().join("prior_deleted.txt");
        // NOTE: never create the file. record(id, ...) accepts any
        // path; we want the dispatch path to discover the missing
        // file and short-circuit.
        set.record(id, prior_dst.clone());
        assert!(
            !prior_dst.exists(),
            "test precondition: prior_dst must not exist on disk"
        );

        let new_dst = dir.path().join("new.txt");
        let outcome = set.dispatch(&src, &new_dst);
        match outcome {
            Ok(false) => {
                // Correct: fell through to byte-copy path.
                assert!(
                    !new_dst.exists(),
                    "Ok(false) means dispatch did NOT create the link"
                );
            }
            Ok(true) => panic!(
                "dispatch claimed success but prior_dst was missing — \
                 should have fallen through"
            ),
            Err(e) => panic!(
                "dispatch should return Ok(false) on missing prior_dst, \
                 got Err: {e}"
            ),
        }
    }

    /// Phase 42 wave-2 — `is_unsupported` recognises the cross-volume
    /// / FAT32-dest / no-hardlinks errors and folds them into
    /// `Ok(false)` rather than `Err`. Locks in the error-mapping
    /// surface so a future refactor doesn't accidentally narrow
    /// `is_unsupported` and start propagating "wrong filesystem" as
    /// a real error to the caller.
    #[test]
    fn is_unsupported_recognises_known_filesystem_limits() {
        // Kind-based detection.
        assert!(is_unsupported(&io::Error::new(
            io::ErrorKind::Unsupported,
            "no hardlinks"
        )));
        assert!(is_unsupported(&io::Error::new(
            io::ErrorKind::InvalidInput,
            "bad arg"
        )));
        assert!(is_unsupported(&io::Error::new(
            io::ErrorKind::CrossesDevices,
            "cross volume"
        )));

        // Raw-os-error detection (Linux EXDEV=18; Windows
        // ERROR_NOT_SAME_DEVICE=17 / ERROR_NOT_SUPPORTED=50).
        assert!(is_unsupported(&io::Error::from_raw_os_error(17)));
        assert!(is_unsupported(&io::Error::from_raw_os_error(18)));
        assert!(is_unsupported(&io::Error::from_raw_os_error(50)));

        // Real I/O failures must NOT be folded.
        assert!(!is_unsupported(&io::Error::new(
            io::ErrorKind::PermissionDenied,
            "denied"
        )));
        assert!(!is_unsupported(&io::Error::new(
            io::ErrorKind::NotFound,
            "missing"
        )));
    }
}
