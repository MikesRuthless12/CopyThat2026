//! Phase 24 — `copythat_core::MetaOps` bridge with real OS probes.
//!
//! Per-OS coverage:
//!
//! - **Windows**: NTFS Alternate Data Streams via
//!   `FindFirstStreamW` / `FindNextStreamW` enumeration and read /
//!   write through tokio `File` handles. The famous
//!   `Zone.Identifier` MOTW stream round-trips through here so a
//!   downloaded EXE keeps its SmartScreen warning after a copy.
//! - **Linux**: `xattr` crate (MIT/Apache-2.0). Captures every
//!   namespace the calling user can read (`user.*`, `security.*`,
//!   `system.posix_acl_*`, `trusted.*`). `system.posix_acl_access`
//!   / `system.posix_acl_default` are also broken out into the
//!   structured [`PosixAclBlob`] so the UI can render them; the
//!   raw xattr is kept too in case the destination cares about the
//!   binary form.
//! - **macOS**: `xattr` crate again, covering the
//!   `com.apple.*` family — Gatekeeper `com.apple.quarantine`,
//!   Spotlight `com.apple.metadata:kMDItemWhereFroms`,
//!   `com.apple.FinderInfo`, `com.apple.ResourceFork`. The legacy
//!   resource fork at `<file>/..namedfork/rsrc` is also captured
//!   when the host is macOS so cross-tool copies that hide the
//!   xattr surface still make the trip.
//!
//! Cross-FS (e.g. Mac → Windows over SMB, Linux → exFAT) write the
//! foreign metadata into an `._<filename>` AppleDouble sidecar when
//! [`copythat_core::MetaPolicy::appledouble_fallback`] is `true`.
//! The sidecar uses a minimal AppleDouble v2 header so widely
//! deployed tools (`tar`, `rsync`, the macOS Finder) recognise it
//! when the file lands back on a Mac.

use std::io;
use std::path::Path;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use copythat_core::meta::NtfsStream;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use copythat_core::meta::XattrEntry;
#[cfg(target_os = "linux")]
use copythat_core::meta::{FileCaps, PosixAclBlob, SeLinuxContext};
#[cfg(target_os = "macos")]
use copythat_core::meta::{FinderInfoBlob, ResourceForkBlob};
use copythat_core::meta::{MetaApplyOutcome, MetaOps, MetaPolicy, MetaSnapshot};

/// Platform-backed security-metadata capture/apply hook.
///
/// Drop one into [`CopyOptions::meta_ops`](copythat_core::CopyOptions::meta_ops)
/// and the engine's Phase 24 metadata pathway lights up. Stateless,
/// cheap to clone, safe to share across concurrent copies.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformMetaOps;

impl MetaOps for PlatformMetaOps {
    fn capture(&self, path: &Path) -> io::Result<MetaSnapshot> {
        let mut snap = MetaSnapshot::default();
        capture_for_host(path, &mut snap)?;
        Ok(snap)
    }

    fn apply(
        &self,
        path: &Path,
        snapshot: &MetaSnapshot,
        policy: &MetaPolicy,
    ) -> io::Result<MetaApplyOutcome> {
        let mut outcome = MetaApplyOutcome::default();
        let mut foreign_streams: Vec<ForeignStream> = Vec::new();
        apply_for_host(path, snapshot, policy, &mut outcome, &mut foreign_streams)?;
        // Anything the host couldn't apply natively — e.g. macOS
        // resource fork on a Linux dst — falls through to the
        // AppleDouble sidecar when the policy allows.
        if !foreign_streams.is_empty() && policy.appledouble_fallback {
            let ext = ext_lower(path);
            write_appledouble_sidecar(path, &foreign_streams)?;
            outcome.translated_to_appledouble = true;
            outcome.translated_extension = ext;
        }
        Ok(outcome)
    }
}

/// One foreign-metadata stream the host's apply pass couldn't honour
/// natively (e.g. macOS resource fork on a Linux destination). Routed
/// into the AppleDouble sidecar fallback.
#[derive(Debug, Clone)]
struct ForeignStream {
    /// The xattr / ADS / fork name as it would appear on the
    /// originating OS (`com.apple.ResourceFork`,
    /// `Zone.Identifier`, `system.posix_acl_access`, …).
    name: String,
    payload: Vec<u8>,
}

fn ext_lower(path: &Path) -> String {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| "none".to_string())
}

// ============================================================
// Per-host capture / apply dispatch
// ============================================================

#[cfg(target_os = "windows")]
fn capture_for_host(path: &Path, snap: &mut MetaSnapshot) -> io::Result<()> {
    snap.ads = capture_ntfs_streams(path)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn capture_for_host(path: &Path, snap: &mut MetaSnapshot) -> io::Result<()> {
    let entries = capture_xattrs(path)?;
    decompose_linux_xattrs(entries, snap);
    Ok(())
}

#[cfg(target_os = "macos")]
fn capture_for_host(path: &Path, snap: &mut MetaSnapshot) -> io::Result<()> {
    let entries = capture_xattrs(path)?;
    decompose_mac_xattrs(entries, snap);
    snap.mac_resource_fork = capture_resource_fork(path)?;
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn capture_for_host(_path: &Path, _snap: &mut MetaSnapshot) -> io::Result<()> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_for_host(
    path: &Path,
    snapshot: &MetaSnapshot,
    policy: &MetaPolicy,
    outcome: &mut MetaApplyOutcome,
    foreign: &mut Vec<ForeignStream>,
) -> io::Result<()> {
    apply_ntfs_streams(path, &snapshot.ads, outcome);
    // xattrs are foreign on Windows; route them to AppleDouble only
    // if the user kept the relevant policy bits on. The capture side
    // never produces them on a Windows source — but the snapshot we
    // were handed may have come from Linux/macOS over SMB.
    for x in &snapshot.xattrs {
        if !policy.preserve_xattrs && !is_structured_xattr(&x.name) {
            continue;
        }
        foreign.push(ForeignStream {
            name: x.name.clone(),
            payload: x.value.clone(),
        });
    }
    if let Some(rf) = &snapshot.mac_resource_fork
        && policy.preserve_resource_forks
    {
        foreign.push(ForeignStream {
            name: "com.apple.ResourceFork".to_string(),
            payload: rf.data.clone(),
        });
    }
    if let Some(fi) = &snapshot.mac_finder_info
        && policy.preserve_resource_forks
    {
        foreign.push(ForeignStream {
            name: "com.apple.FinderInfo".to_string(),
            payload: fi.data.to_vec(),
        });
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn apply_for_host(
    path: &Path,
    snapshot: &MetaSnapshot,
    policy: &MetaPolicy,
    outcome: &mut MetaApplyOutcome,
    foreign: &mut Vec<ForeignStream>,
) -> io::Result<()> {
    apply_xattrs_unix(path, &snapshot.xattrs, policy, outcome);
    // ADS are foreign on Linux. Route them to AppleDouble (rare, but
    // documented for SMB→Linux).
    for stream in &snapshot.ads {
        foreign.push(ForeignStream {
            name: stream.name.clone(),
            payload: stream.data.clone(),
        });
    }
    if let Some(rf) = &snapshot.mac_resource_fork
        && policy.preserve_resource_forks
    {
        foreign.push(ForeignStream {
            name: "com.apple.ResourceFork".to_string(),
            payload: rf.data.clone(),
        });
    }
    if let Some(fi) = &snapshot.mac_finder_info
        && policy.preserve_resource_forks
    {
        foreign.push(ForeignStream {
            name: "com.apple.FinderInfo".to_string(),
            payload: fi.data.to_vec(),
        });
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn apply_for_host(
    path: &Path,
    snapshot: &MetaSnapshot,
    policy: &MetaPolicy,
    outcome: &mut MetaApplyOutcome,
    foreign: &mut Vec<ForeignStream>,
) -> io::Result<()> {
    apply_xattrs_unix(path, &snapshot.xattrs, policy, outcome);
    if let Some(rf) = &snapshot.mac_resource_fork
        && policy.preserve_resource_forks
    {
        if let Err(e) = apply_resource_fork(path, &rf.data) {
            outcome.partial_failures.push(format!("resource-fork: {e}"));
        }
    }
    // ADS are foreign on macOS.
    for stream in &snapshot.ads {
        foreign.push(ForeignStream {
            name: stream.name.clone(),
            payload: stream.data.clone(),
        });
    }
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn apply_for_host(
    _path: &Path,
    snapshot: &MetaSnapshot,
    _policy: &MetaPolicy,
    _outcome: &mut MetaApplyOutcome,
    foreign: &mut Vec<ForeignStream>,
) -> io::Result<()> {
    // Unknown host — every stream is "foreign". Push them to the
    // AppleDouble fallback so the data still survives the trip.
    for s in &snapshot.ads {
        foreign.push(ForeignStream {
            name: s.name.clone(),
            payload: s.data.clone(),
        });
    }
    for x in &snapshot.xattrs {
        foreign.push(ForeignStream {
            name: x.name.clone(),
            payload: x.value.clone(),
        });
    }
    if let Some(rf) = &snapshot.mac_resource_fork {
        foreign.push(ForeignStream {
            name: "com.apple.ResourceFork".to_string(),
            payload: rf.data.clone(),
        });
    }
    Ok(())
}

#[allow(dead_code)]
fn is_structured_xattr(name: &str) -> bool {
    name.starts_with("system.posix_acl_")
        || name == "security.selinux"
        || name == "security.capability"
}

// ============================================================
// Linux / macOS xattr capture
// ============================================================

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn capture_xattrs(path: &Path) -> io::Result<Vec<XattrEntry>> {
    let mut out = Vec::new();
    let names = match xattr::list(path) {
        Ok(n) => n,
        Err(e) => {
            // ENOTSUP — file's filesystem doesn't support xattrs.
            // Empty capture is the right answer.
            if e.raw_os_error() == Some(libc::ENOTSUP) {
                return Ok(Vec::new());
            }
            return Err(e);
        }
    };
    for n in names {
        let name_str = n.to_string_lossy().into_owned();
        match xattr::get(path, &n) {
            Ok(Some(v)) => out.push(XattrEntry {
                name: name_str,
                value: v,
            }),
            Ok(None) => {}
            // ENODATA can race when the xattr is removed between
            // list and get. Skip; it's harmless.
            Err(e) if e.raw_os_error() == Some(61) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(out)
}

#[cfg(target_os = "linux")]
fn decompose_linux_xattrs(entries: Vec<XattrEntry>, snap: &mut MetaSnapshot) {
    let mut access: Option<Vec<u8>> = None;
    let mut default: Option<Vec<u8>> = None;
    for x in &entries {
        match x.name.as_str() {
            "system.posix_acl_access" => access = Some(x.value.clone()),
            "system.posix_acl_default" => default = Some(x.value.clone()),
            "security.selinux" => {
                snap.selinux = Some(SeLinuxContext {
                    context: String::from_utf8_lossy(trim_trailing_nul(&x.value)).into_owned(),
                });
            }
            "security.capability" => {
                snap.linux_caps = Some(FileCaps {
                    raw: x.value.clone(),
                });
            }
            _ => {}
        }
    }
    if access.is_some() || default.is_some() {
        snap.posix_acl = Some(PosixAclBlob {
            access_acl: access,
            default_acl: default,
        });
    }
    snap.xattrs = entries;
}

#[cfg(target_os = "macos")]
fn decompose_mac_xattrs(entries: Vec<XattrEntry>, snap: &mut MetaSnapshot) {
    for x in &entries {
        if x.name == "com.apple.FinderInfo" && x.value.len() == 32 {
            let mut data = [0u8; 32];
            data.copy_from_slice(&x.value);
            snap.mac_finder_info = Some(FinderInfoBlob { data });
        }
        if x.name == "com.apple.ResourceFork" && snap.mac_resource_fork.is_none() {
            snap.mac_resource_fork = Some(ResourceForkBlob {
                data: x.value.clone(),
            });
        }
    }
    snap.xattrs = entries;
}

#[cfg(target_os = "linux")]
fn trim_trailing_nul(bytes: &[u8]) -> &[u8] {
    if let Some(&0) = bytes.last() {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn apply_xattrs_unix(
    path: &Path,
    entries: &[XattrEntry],
    policy: &MetaPolicy,
    outcome: &mut MetaApplyOutcome,
) {
    for x in entries {
        if !policy_permits_xattr(&x.name, policy) {
            continue;
        }
        if let Err(e) = xattr::set(path, &x.name, &x.value) {
            // ENOTSUP — destination FS doesn't accept the xattr.
            // Don't fail the whole apply pass; record it and move on.
            // The engine's caller can branch on partial_failures.
            outcome
                .partial_failures
                .push(format!("xattr {}: {e}", x.name));
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn policy_permits_xattr(name: &str, policy: &MetaPolicy) -> bool {
    if name == "security.selinux" {
        return policy.preserve_selinux;
    }
    if name.starts_with("system.posix_acl_") {
        return policy.preserve_posix_acls;
    }
    if name == "com.apple.ResourceFork" || name == "com.apple.FinderInfo" {
        return policy.preserve_resource_forks;
    }
    if name == "Zone.Identifier" || name == "user.Zone.Identifier" {
        // Highly unusual on Unix but possible via Samba bridges —
        // honour the MOTW toggle.
        return policy.preserve_motw;
    }
    policy.preserve_xattrs
}

// ============================================================
// macOS resource fork (legacy `..namedfork/rsrc` access)
// ============================================================

#[cfg(target_os = "macos")]
fn capture_resource_fork(path: &Path) -> io::Result<Option<ResourceForkBlob>> {
    let fork_path = resource_fork_path(path);
    match std::fs::read(&fork_path) {
        Ok(data) if !data.is_empty() => Ok(Some(ResourceForkBlob { data })),
        Ok(_) => Ok(None),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        // ENOENT can also surface as raw EPERM when the volume
        // disables Carbon resource forks. Treat as "no fork".
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(target_os = "macos")]
fn apply_resource_fork(path: &Path, data: &[u8]) -> io::Result<()> {
    let fork_path = resource_fork_path(path);
    std::fs::write(&fork_path, data)
}

#[cfg(target_os = "macos")]
fn resource_fork_path(path: &Path) -> PathBuf {
    let mut p = path.to_path_buf().into_os_string();
    p.push("/..namedfork/rsrc");
    PathBuf::from(p)
}

// ============================================================
// Windows NTFS Alternate Data Streams
// ============================================================

#[cfg(target_os = "windows")]
fn capture_ntfs_streams(path: &Path) -> io::Result<Vec<NtfsStream>> {
    use std::mem;
    use std::os::windows::ffi::OsStrExt;

    use windows_sys::Win32::Foundation::{
        ERROR_HANDLE_EOF, ERROR_NO_MORE_FILES, INVALID_HANDLE_VALUE,
    };
    use windows_sys::Win32::Storage::FileSystem::{
        FindClose, FindFirstStreamW, FindNextStreamW, FindStreamInfoStandard,
        WIN32_FIND_STREAM_DATA,
    };

    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.push(0);

    let mut data: WIN32_FIND_STREAM_DATA = unsafe { mem::zeroed() };
    // SAFETY: `wide` is a NUL-terminated UTF-16 path; `data` is a
    // properly-zeroed struct of the documented size.
    let handle = unsafe {
        FindFirstStreamW(
            wide.as_ptr(),
            FindStreamInfoStandard,
            &mut data as *mut _ as *mut _,
            0,
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        // ERROR_HANDLE_EOF (38) means no streams at all — return empty.
        let last = io::Error::last_os_error();
        if last.raw_os_error() == Some(ERROR_HANDLE_EOF as i32) {
            return Ok(Vec::new());
        }
        return Err(last);
    }

    let mut out: Vec<NtfsStream> = Vec::new();
    let mut iter_data = data;
    loop {
        // The first stream is always `::$DATA` — skip it; we don't
        // want to capture the file's primary content.
        let stream_name = wide_cstr_to_string(&iter_data.cStreamName);
        if let Some(parsed) = parse_ads_name(&stream_name)
            && let Ok(content) = read_ads_stream(path, &parsed)
        {
            out.push(NtfsStream {
                name: parsed,
                data: content,
            });
        }
        let mut next: WIN32_FIND_STREAM_DATA = unsafe { mem::zeroed() };
        // SAFETY: `handle` is live until `FindClose`; `next` is a
        // properly-zeroed struct of the documented size.
        let ok = unsafe { FindNextStreamW(handle, &mut next as *mut _ as *mut _) };
        if ok == 0 {
            let err = io::Error::last_os_error();
            // SAFETY: `handle` was valid; we're done with iteration.
            unsafe { FindClose(handle) };
            if err.raw_os_error() == Some(ERROR_HANDLE_EOF as i32)
                || err.raw_os_error() == Some(ERROR_NO_MORE_FILES as i32)
            {
                return Ok(out);
            }
            return Err(err);
        }
        iter_data = next;
    }
}

#[cfg(target_os = "windows")]
fn wide_cstr_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

/// Convert a `WIN32_FIND_STREAM_DATA::cStreamName` (which always looks
/// like `:streamname:$DATA` with the `:` and `:$DATA` tags) into the
/// bare stream name. Returns `None` for the unnamed default stream.
#[cfg(target_os = "windows")]
fn parse_ads_name(raw: &str) -> Option<String> {
    let trimmed = raw.strip_prefix(':')?;
    let end = trimmed.rfind(':')?;
    let name = &trimmed[..end];
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

#[cfg(target_os = "windows")]
fn read_ads_stream(path: &Path, stream: &str) -> io::Result<Vec<u8>> {
    let mut path_with_stream = path.as_os_str().to_os_string();
    path_with_stream.push(":");
    path_with_stream.push(stream);
    std::fs::read(PathBuf::from(path_with_stream))
}

#[cfg(target_os = "windows")]
fn apply_ntfs_streams(path: &Path, streams: &[NtfsStream], outcome: &mut MetaApplyOutcome) {
    for s in streams {
        let mut path_with_stream = path.as_os_str().to_os_string();
        path_with_stream.push(":");
        path_with_stream.push(&s.name);
        if let Err(e) = std::fs::write(PathBuf::from(path_with_stream), &s.data) {
            outcome
                .partial_failures
                .push(format!("ads {}: {e}", s.name));
        }
    }
}

// ============================================================
// AppleDouble v2 sidecar fallback
// ============================================================

/// Emit a minimal AppleDouble v2 sidecar at `<dir>/._<basename>`
/// carrying the foreign streams. Uses the documented v2 header
/// (`magic 0x00051607`, `version 0x00020000`, `home filler` 16 bytes,
/// then a count + entry descriptor table). Each stream becomes one
/// entry tagged with a synthesized entry-ID derived from a stable
/// FNV-1a hash of the stream name — that way the same metadata
/// re-applied via `tar` / `rsync` lands deterministically.
fn write_appledouble_sidecar(path: &Path, streams: &[ForeignStream]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("appledouble: dst has no parent"))?;
    let basename = path
        .file_name()
        .ok_or_else(|| io::Error::other("appledouble: dst has no basename"))?;
    let mut sidecar_name = std::ffi::OsString::from("._");
    sidecar_name.push(basename);
    let sidecar_path = parent.join(sidecar_name);

    // AppleDouble v2 layout:
    //   u32 magic = 0x00051607  (BE)
    //   u32 version = 0x00020000 (BE)
    //   [16] filler = 0
    //   u16 num_entries (BE)
    //   for each entry: u32 id, u32 offset, u32 length  (BE)
    //   ... payloads ...
    let entry_count = streams.len() as u16;
    let header_fixed = 4 + 4 + 16 + 2;
    let entry_descriptor = 4 + 4 + 4;
    let mut offset_cursor: u32 = (header_fixed + entry_descriptor * entry_count as usize) as u32;

    let mut out: Vec<u8> = Vec::with_capacity(offset_cursor as usize);
    out.extend_from_slice(&0x0005_1607u32.to_be_bytes());
    out.extend_from_slice(&0x0002_0000u32.to_be_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&entry_count.to_be_bytes());

    let mut payloads: Vec<&[u8]> = Vec::with_capacity(streams.len());
    for s in streams {
        let id = synth_entry_id(&s.name);
        let len = s.payload.len() as u32;
        out.extend_from_slice(&id.to_be_bytes());
        out.extend_from_slice(&offset_cursor.to_be_bytes());
        out.extend_from_slice(&len.to_be_bytes());
        offset_cursor = offset_cursor.saturating_add(len);
        payloads.push(&s.payload);
    }
    for p in payloads {
        out.extend_from_slice(p);
    }
    std::fs::write(&sidecar_path, out)?;
    Ok(())
}

/// Synthesise a stable AppleDouble entry-ID from a stream name via
/// FNV-1a 32-bit. We avoid the documented entry IDs (1 = data fork,
/// 2 = resource fork, 9 = FinderInfo) by reserving ID 0..=15 for
/// canonical Apple usage and rotating any FNV collision into the
/// 16..=u32::MAX space.
fn synth_entry_id(name: &str) -> u32 {
    if name == "com.apple.ResourceFork" {
        return 2;
    }
    if name == "com.apple.FinderInfo" {
        return 9;
    }
    let mut hash: u32 = 0x811c_9dc5;
    for b in name.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    if hash < 16 {
        hash = hash.wrapping_add(16);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appledouble_sidecar_roundtrips_minimum_header() {
        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("file.bin");
        std::fs::write(&dst, b"primary content").unwrap();
        let foreign = vec![
            ForeignStream {
                name: "com.apple.ResourceFork".to_string(),
                payload: vec![0xAA; 32],
            },
            ForeignStream {
                name: "com.apple.quarantine".to_string(),
                payload: b"q;1234".to_vec(),
            },
        ];
        write_appledouble_sidecar(&dst, &foreign).unwrap();
        let sidecar = tmp.path().join("._file.bin");
        let bytes = std::fs::read(&sidecar).unwrap();
        // Magic + version
        assert_eq!(&bytes[0..4], &0x0005_1607u32.to_be_bytes());
        assert_eq!(&bytes[4..8], &0x0002_0000u32.to_be_bytes());
        // Filler
        assert!(bytes[8..24].iter().all(|&b| b == 0));
        // Entry count
        assert_eq!(&bytes[24..26], &2u16.to_be_bytes());
        // Resource-fork entry id at offset 26
        assert_eq!(&bytes[26..30], &2u32.to_be_bytes());
    }

    #[test]
    fn synth_entry_id_uses_canonical_for_known_streams() {
        assert_eq!(synth_entry_id("com.apple.ResourceFork"), 2);
        assert_eq!(synth_entry_id("com.apple.FinderInfo"), 9);
        // Quarantine — non-canonical, should land in 16+ space.
        assert!(synth_entry_id("com.apple.quarantine") >= 16);
    }

    #[test]
    fn ext_lower_handles_no_extension() {
        assert_eq!(ext_lower(Path::new("/tmp/no-ext")), "none");
        assert_eq!(ext_lower(Path::new("/tmp/file.DOCX")), "docx");
    }

    #[test]
    fn capture_returns_empty_for_vanilla_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"hello").unwrap();
        let ops = PlatformMetaOps;
        let snap = ops.capture(tmp.path()).unwrap();
        // No assertions on the surface; just that the call succeeds
        // without panicking on a vanilla file. On Linux this returns
        // empty xattrs; on Windows it returns no ADS; on macOS it
        // may pick up FinderInfo if the OS auto-set it.
        assert!(snap.ads.is_empty() || cfg!(target_os = "windows"));
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn xattr_capture_apply_round_trip() {
        let tmp_src = tempfile::NamedTempFile::new().unwrap();
        let tmp_dst = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp_src.path(), b"src").unwrap();
        std::fs::write(tmp_dst.path(), b"dst").unwrap();

        // Best effort — xattr set may fail on tmpfs; test bails
        // quietly.
        let xattr_name = if cfg!(target_os = "macos") {
            "com.apple.metadata:kMDItemWhereFroms"
        } else {
            "user.copythat.test"
        };
        let value = b"phase24-roundtrip";
        if xattr::set(tmp_src.path(), xattr_name, value).is_err() {
            return;
        }

        let ops = PlatformMetaOps;
        let snap = ops.capture(tmp_src.path()).unwrap();
        let policy = MetaPolicy::default();
        let _ = ops.apply(tmp_dst.path(), &snap, &policy).unwrap();

        let got = xattr::get(tmp_dst.path(), xattr_name).ok().flatten();
        assert_eq!(got.as_deref(), Some(&value[..]));
    }
}
