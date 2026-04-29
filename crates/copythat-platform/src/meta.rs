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
//!   structured [`copythat_core::meta::PosixAclBlob`] so the UI can render them; the
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
    // `Path::extension` would happily return `"bashrc"` for a leading-
    // dot file like `.bashrc` because the OS path API treats the dot
    // as a separator. The translated-extension wire string should
    // report `"none"` for dotfiles since they have no real extension —
    // detect a leading dot with no internal dot in the basename and
    // bail to `"none"` before calling `extension()`.
    if let Some(stem) = path.file_name().and_then(|s| s.to_str())
        && stem.starts_with('.')
        && !stem[1..].contains('.')
    {
        return "none".to_string();
    }
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
    // if the matching individual policy toggle is on. The capture side
    // never produces them on a Windows source — but the snapshot we
    // were handed may have come from Linux/macOS over SMB.
    for x in &snapshot.xattrs {
        if !foreign_xattr_permitted(&x.name, policy) {
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
    policy: &MetaPolicy,
    _outcome: &mut MetaApplyOutcome,
    foreign: &mut Vec<ForeignStream>,
) -> io::Result<()> {
    // Unknown host — every stream is "foreign". Push them to the
    // AppleDouble fallback so the data still survives the trip,
    // gating on the per-family policy toggle so disabled streams
    // are dropped rather than smuggled through.
    for s in &snapshot.ads {
        let gate = if s.name.eq_ignore_ascii_case("Zone.Identifier") {
            policy.preserve_motw
        } else {
            policy.preserve_xattrs
        };
        if !gate {
            continue;
        }
        foreign.push(ForeignStream {
            name: s.name.clone(),
            payload: s.data.clone(),
        });
    }
    for x in &snapshot.xattrs {
        if !foreign_xattr_permitted(&x.name, policy) {
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
    Ok(())
}

#[allow(dead_code)]
fn is_structured_xattr(name: &str) -> bool {
    name.starts_with("system.posix_acl_")
        || name == "security.selinux"
        || name == "security.capability"
}

/// Per-toggle gate for routing a *foreign* xattr (one captured on a
/// Linux/macOS source that landed on a non-Unix destination) into the
/// AppleDouble sidecar. Each named family is keyed off its own policy
/// bit so disabling `preserve_xattrs` doesn't accidentally smuggle a
/// `security.selinux` or `security.capability` payload through, and
/// disabling `preserve_selinux` actually drops the SELinux xattr.
#[allow(dead_code)]
fn foreign_xattr_permitted(name: &str, policy: &MetaPolicy) -> bool {
    if name.starts_with("system.posix_acl_") {
        return policy.preserve_posix_acls;
    }
    if name == "security.selinux" {
        return policy.preserve_selinux;
    }
    if name == "security.capability" {
        // No dedicated `preserve_capability` bit yet; gate it under
        // the generic xattr toggle. Documenting the assumption here
        // so a future audit doesn't have to grep for the link.
        return policy.preserve_xattrs;
    }
    if name == "com.apple.ResourceFork" || name == "com.apple.FinderInfo" {
        return policy.preserve_resource_forks;
    }
    if name == "Zone.Identifier" || name == "user.Zone.Identifier" {
        return policy.preserve_motw;
    }
    policy.preserve_xattrs
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
            // list and get. Skip; it's harmless. `libc::ENODATA` is
            // 61 on Linux but 96 on macOS, so source it portably.
            Err(e) if e.raw_os_error() == Some(libc::ENODATA) => {}
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
///
/// The trailing `:$DATA` tag is matched as the exact 6-byte literal at
/// the end of the string (rather than a generic `rfind(':')`), so a
/// stream name that itself contains a colon is not mis-parsed. Names
/// that retain an embedded colon after stripping the trailing tag are
/// rejected as malformed — NTFS rejects raw colons in stream names but
/// the kernel could in principle hand us a buffer crafted by a hostile
/// driver, so we refuse to mint a misleading [`NtfsStream::name`] from
/// it.
#[cfg(target_os = "windows")]
fn parse_ads_name(raw: &str) -> Option<String> {
    let trimmed = raw.strip_prefix(':')?;
    // The trailing tag is always exactly `:$DATA` (6 bytes). Strip it
    // by suffix match instead of `rfind(':')` so a stream name that
    // itself contains a colon is not truncated at the wrong boundary.
    let name = trimmed.strip_suffix(":$DATA")?;
    if name.is_empty() {
        return None;
    }
    // Reject embedded colons — they should never appear in a valid
    // NTFS stream name and signal either a parser bug here or a
    // malformed kernel-supplied buffer.
    if name.contains(':') {
        return None;
    }
    Some(name.to_string())
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
///
/// Two safeties layered on top of the FNV hash:
///
/// 1. Collision detection: if two stream names hash to the same
///    entry-ID, the second entry linear-probes (incrementing the ID,
///    skipping the canonical 0..=15 reserved range and wrapping past
///    `u32::MAX`) until it finds a free slot. Without this an
///    overwrite at read time would silently lose one of the streams.
/// 2. Offset arithmetic runs in `u64` and is checked at write time
///    before being narrowed to `u32` — the AppleDouble v2 entry
///    descriptor stores a `u32` offset, so payloads totalling more
///    than 4 GiB cannot fit and we surface the error rather than
///    silently truncating.
fn write_appledouble_sidecar(path: &Path, streams: &[ForeignStream]) -> io::Result<()> {
    use std::collections::HashSet;

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
    let header_fixed = 4u64 + 4 + 16 + 2;
    let entry_descriptor = 4u64 + 4 + 4;
    // Run offset arithmetic in u64 so a >4 GiB total payload is
    // detected at narrowing time rather than silently wrapping.
    let mut offset_cursor: u64 = header_fixed + entry_descriptor * entry_count as u64;

    let initial_capacity = usize::try_from(offset_cursor).unwrap_or(0);
    let mut out: Vec<u8> = Vec::with_capacity(initial_capacity);
    out.extend_from_slice(&0x0005_1607u32.to_be_bytes());
    out.extend_from_slice(&0x0002_0000u32.to_be_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&entry_count.to_be_bytes());

    // Track entry IDs that have already been allocated so a FNV-1a
    // collision (or an explicit 2 / 9 reuse) doesn't overwrite an
    // earlier entry.
    let mut used_ids: HashSet<u32> = HashSet::with_capacity(streams.len());
    let mut payloads: Vec<&[u8]> = Vec::with_capacity(streams.len());
    for s in streams {
        let id = synth_entry_id(&s.name, &mut used_ids);
        let len_u64 = s.payload.len() as u64;
        let len_u32 = u32::try_from(len_u64).map_err(|_| {
            io::Error::other(format!(
                "appledouble: payload for {} exceeds 4 GiB ({} bytes)",
                s.name,
                s.payload.len()
            ))
        })?;
        let offset_u32 = u32::try_from(offset_cursor).map_err(|_| {
            io::Error::other(format!(
                "appledouble: total payload offset exceeds 4 GiB at entry {} ({} bytes)",
                s.name, offset_cursor
            ))
        })?;
        out.extend_from_slice(&id.to_be_bytes());
        out.extend_from_slice(&offset_u32.to_be_bytes());
        out.extend_from_slice(&len_u32.to_be_bytes());
        offset_cursor = offset_cursor.saturating_add(len_u64);
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
///
/// `used` carries every ID we've already handed out for *this*
/// sidecar; on a collision we linear-probe by incrementing (and
/// skipping the canonical 0..=15 range) until we find a free slot.
/// The canonical IDs `2` (ResourceFork) and `9` (FinderInfo) are
/// always granted to the matching stream name — duplicates of those
/// names also probe so a misbehaving snapshot can't shadow itself.
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
fn synth_entry_id(name: &str, used: &mut std::collections::HashSet<u32>) -> u32 {
    let preferred = if name == "com.apple.ResourceFork" {
        2
    } else if name == "com.apple.FinderInfo" {
        9
    } else {
        let mut hash: u32 = 0x811c_9dc5;
        for b in name.bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x0100_0193);
        }
        if hash < 16 {
            hash = hash.wrapping_add(16);
        }
        hash
    };
    let id = next_free_entry_id(preferred, used);
    used.insert(id);
    id
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn synth_entry_id(name: &str, used: &mut std::collections::HashSet<u32>) -> u32 {
    let preferred = if name == "com.apple.ResourceFork" {
        2
    } else if name == "com.apple.FinderInfo" {
        9
    } else {
        let mut hash: u32 = 0x811c_9dc5;
        for b in name.bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x0100_0193);
        }
        if hash < 16 {
            hash = hash.wrapping_add(16);
        }
        hash
    };
    let id = next_free_entry_id(preferred, used);
    used.insert(id);
    id
}

/// Linear-probe `start` against the `used` set, returning the first
/// unused ID. Skips the canonical Apple range `0..=15` for non-canonical
/// streams (the caller is allowed to seed with `2` or `9` for the
/// known names) and wraps via `wrapping_add` past `u32::MAX`.
fn next_free_entry_id(start: u32, used: &std::collections::HashSet<u32>) -> u32 {
    // The canonical IDs 2 and 9 are explicitly handed back to the
    // matching stream names by the caller; we honour that on the
    // first try only. After that we skip the reserved 0..=15 band.
    if !used.contains(&start) {
        return start;
    }
    let mut probe = start;
    loop {
        probe = probe.wrapping_add(1);
        if probe < 16 {
            // Skip the canonical reserved range. Wrap into 16+.
            probe = 16;
        }
        if !used.contains(&probe) {
            return probe;
        }
        // u32 has 4 billion slots; running out means the caller is
        // pushing >2^32 streams which is well outside what AppleDouble
        // can describe (entry-count is u16). Defensive bail.
        if probe == start {
            return probe;
        }
    }
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
        let mut used = std::collections::HashSet::new();
        assert_eq!(synth_entry_id("com.apple.ResourceFork", &mut used), 2);
        let mut used = std::collections::HashSet::new();
        assert_eq!(synth_entry_id("com.apple.FinderInfo", &mut used), 9);
        // Quarantine — non-canonical, should land in 16+ space.
        let mut used = std::collections::HashSet::new();
        assert!(synth_entry_id("com.apple.quarantine", &mut used) >= 16);
    }

    #[test]
    fn synth_entry_id_resolves_collision_via_linear_probe() {
        // Pre-seed the used set with a hash that we'll observe falls
        // out of the FNV pass for "user.foo", forcing the linear
        // probe.
        let mut used = std::collections::HashSet::new();
        let first = synth_entry_id("user.foo", &mut used);
        // Asking again with the same name MUST mint a different ID
        // even though the hash is identical, because `used` already
        // contains the first one.
        let second = synth_entry_id("user.foo", &mut used);
        assert_ne!(first, second);
        assert!(second >= 16);
    }

    #[test]
    fn appledouble_collision_does_not_overwrite_entry() {
        // Two streams whose hashes collide cannot share an ID; the
        // second one must end up with a different one.
        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("file.bin");
        std::fs::write(&dst, b"primary").unwrap();
        // Crafted: the second name has the same FNV-1a output by
        // construction is hard, so we synthesize the situation by
        // forcing the same name twice. Same name → same hash → second
        // one would otherwise overwrite the first.
        let foreign = vec![
            ForeignStream {
                name: "user.dup".to_string(),
                payload: vec![0xAA; 4],
            },
            ForeignStream {
                name: "user.dup".to_string(),
                payload: vec![0xBB; 4],
            },
        ];
        write_appledouble_sidecar(&dst, &foreign).unwrap();
        let sidecar = tmp.path().join("._file.bin");
        let bytes = std::fs::read(&sidecar).unwrap();
        // Two entries → two distinct IDs.
        let id1 = u32::from_be_bytes(bytes[26..30].try_into().unwrap());
        let id2 = u32::from_be_bytes(bytes[38..42].try_into().unwrap());
        assert_ne!(id1, id2, "collision must be resolved into distinct IDs");
    }

    #[test]
    fn parse_ads_name_strips_dollar_data_suffix() {
        #[cfg(target_os = "windows")]
        {
            assert_eq!(
                parse_ads_name(":Zone.Identifier:$DATA").as_deref(),
                Some("Zone.Identifier")
            );
            // Default unnamed stream is rejected.
            assert!(parse_ads_name("::$DATA").is_none());
            // Missing trailing tag → rejected.
            assert!(parse_ads_name(":Zone.Identifier").is_none());
        }
    }

    #[test]
    fn ext_lower_handles_no_extension() {
        assert_eq!(ext_lower(Path::new("/tmp/no-ext")), "none");
        assert_eq!(ext_lower(Path::new("/tmp/file.DOCX")), "docx");
    }

    #[test]
    fn ext_lower_treats_dotfiles_as_no_extension() {
        // `.bashrc` should report "none", not "bashrc".
        assert_eq!(ext_lower(Path::new("/tmp/.bashrc")), "none");
        assert_eq!(ext_lower(Path::new("/tmp/.gitignore")), "none");
        // A dotfile WITH an extension still surfaces the extension.
        assert_eq!(ext_lower(Path::new("/tmp/.config.json")), "json");
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

    // ============================================================
    // Wave-2 adversarial-input regressions for the wave-1 hardening
    // (AppleDouble offset overflow, ADS embedded colon, FinderInfo
    // classifier name+length, foreign-xattr policy AND→OR routing,
    // FNV-1a collision linear probe).
    // ============================================================

    /// Reference FNV-1a 32-bit implementation. Kept in lock-step with
    /// the production hash inside `synth_entry_id` so the collision
    /// search below is meaningful.
    fn fnv1a_32(name: &str) -> u32 {
        let mut h: u32 = 0x811c_9dc5;
        for b in name.bytes() {
            h ^= b as u32;
            h = h.wrapping_mul(0x0100_0193);
        }
        if h < 16 { h.wrapping_add(16) } else { h }
    }

    #[test]
    fn appledouble_payload_under_4gib_round_trips() {
        // The wave-1 fix narrowed offset/length arithmetic from
        // unchecked `as u32` (silent truncation on overflow) to
        // `u64 + u32::try_from`. Verify the success path still
        // round-trips for normal payloads well below the 4 GiB
        // boundary (the common case).
        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("file.bin");
        std::fs::write(&dst, b"primary").unwrap();
        let foreign = vec![ForeignStream {
            name: "user.smallish".to_string(),
            // 1 KiB payload — well under the u32 ceiling.
            payload: vec![0x42; 1024],
        }];
        write_appledouble_sidecar(&dst, &foreign).expect("small payload should round-trip");
        let sidecar = tmp.path().join("._file.bin");
        let bytes = std::fs::read(&sidecar).unwrap();
        // entry_count == 1
        assert_eq!(&bytes[24..26], &1u16.to_be_bytes());
        // entry length field == 1024
        assert_eq!(&bytes[34..38], &1024u32.to_be_bytes());
    }

    #[test]
    fn appledouble_payload_over_4gib_returns_err() {
        // Wave-1 changed `s.payload.len() as u32` (silent truncation)
        // to `u32::try_from(s.payload.len() as u64)?`. Triggering the
        // overflow path needs a Vec that *reports* `len > u32::MAX`.
        // We can't physically allocate 4 GiB and Rust's `Vec::set_len`
        // enforces `new_len <= capacity()` in debug builds, so we
        // build an unowned Vec via `Vec::from_raw_parts` whose
        // capacity is also spoofed to `u32::MAX + 1`. The Vec is
        // never dropped (`mem::forget` after the call) so the
        // allocator never sees the lie.
        //
        // The function under test only reads `s.payload.len()` and
        // returns Err on `u32::try_from` *before* it dereferences any
        // bytes through `extend_from_slice`, so no code touches the
        // spoofed pointer range.
        //
        // On 32-bit targets `u32::MAX + 1` overflows `usize`, so
        // bail.
        if usize::BITS < 64 {
            return;
        }

        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("file.bin");
        std::fs::write(&dst, b"primary").unwrap();

        // 16 bytes of legitimately-allocated buffer to back the Vec
        // pointer, in case any miri/sanitizer path probes the first
        // bytes. We never read them; they're just so the pointer is
        // a valid object.
        let real_buf: Box<[u8; 16]> = Box::new([0u8; 16]);
        let real_ptr = Box::into_raw(real_buf) as *mut u8;

        // SAFETY: we synthesise a `Vec<u8>` that *claims* a
        // length/capacity beyond `u32::MAX`. The wave-1 code path
        // checks `u32::try_from(s.payload.len())?` and returns Err
        // *before* doing anything that reads the pointer's data.
        // The Vec is `mem::forget`-ed below so its `Drop` never runs,
        // meaning the allocator never tries to free a buffer that
        // doesn't really span the spoofed capacity. We separately
        // free the underlying 16 bytes by reconstituting an honest
        // `Box` over `real_ptr`.
        let huge: usize = u32::MAX as usize + 1;
        let lying_payload: Vec<u8> = unsafe { Vec::from_raw_parts(real_ptr, huge, huge) };
        let lying_stream = ForeignStream {
            name: "user.huge".to_string(),
            payload: lying_payload,
        };
        let foreign = vec![lying_stream];

        let result = write_appledouble_sidecar(&dst, &foreign);

        // SAFETY: we MUST avoid dropping the lying Vec because its
        // (ptr, len=huge, cap=huge) tuple would cause the allocator
        // to deallocate a layout it never allocated. Decompose
        // `foreign` and `mem::forget` the lying Vec, then free the
        // real 16-byte buffer through an honest Box.
        let mut foreign = foreign;
        let lying_stream = foreign.pop().unwrap();
        let lying_vec = lying_stream.payload;
        std::mem::forget(lying_vec);
        // SAFETY: real_ptr was the result of `Box::into_raw` over a
        // Box<[u8; 16]>; reconstituting the same `Box<[u8; 16]>`
        // here lets the allocator free exactly the layout it
        // originally received.
        unsafe {
            let _real_box: Box<[u8; 16]> = Box::from_raw(real_ptr as *mut [u8; 16]);
        }

        assert!(
            result.is_err(),
            "payload >4 GiB must surface an error rather than silently truncate"
        );
        let err_str = format!("{}", result.unwrap_err());
        assert!(
            err_str.contains("4 GiB") || err_str.contains("appledouble"),
            "error message should mention the AppleDouble overflow ceiling, got: {err_str}"
        );
        // The sidecar must NOT have been written.
        assert!(!tmp.path().join("._file.bin").exists());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn parse_ads_name_rejects_embedded_colon_inside_name() {
        // Wave-1 swapped `rfind(':')` for `strip_suffix(":$DATA")` and
        // added an explicit reject on embedded colons. Cover the
        // hostile inputs that the old `rfind` parser would have
        // accepted (and mis-parsed).

        // A stream name containing internal colons after the leading
        // ':' marker. Old rfind code would have returned
        // "file.txt:my:weird:name" by stripping at the wrong colon;
        // wave-1 rejects it because the post-suffix-strip name still
        // contains ':'.
        assert!(parse_ads_name(":file.txt:my:weird:name:$DATA").is_none());

        // Empty stream name between two colons (`":file.txt::$DATA"`):
        // after stripping the leading ':' we get "file.txt::$DATA";
        // strip_suffix ":$DATA" yields "file.txt:" which still
        // contains ':' → rejected.
        assert!(parse_ads_name(":file.txt::$DATA").is_none());

        // Default unnamed stream — empty name after suffix strip.
        assert!(parse_ads_name("::$DATA").is_none());

        // No colon at all — strip_prefix(':') fails first.
        assert!(parse_ads_name("file.txt").is_none());

        // Has a colon but no ":$DATA" suffix — strip_suffix fails.
        assert!(parse_ads_name(":file.txt:stream").is_none());

        // Missing leading colon — strip_prefix fails before we even
        // see the suffix.
        assert!(parse_ads_name("file.txt:stream:$DATA").is_none());
        assert!(parse_ads_name("file.txt:$DATA").is_none());

        // Sanity: a well-formed valid name still parses. The leading
        // ':' is the WIN32_FIND_STREAM_DATA tag; "file.txt" here is
        // the stream-name slot (the field always has the form
        // `:<name>:$DATA`), so this is a synthetic case where the
        // stream name happens to look like a filename — accepted.
        assert_eq!(
            parse_ads_name(":file.txt:$DATA").as_deref(),
            Some("file.txt")
        );

        // The canonical Mark-of-the-Web stream still parses.
        assert_eq!(
            parse_ads_name(":Zone.Identifier:$DATA").as_deref(),
            Some("Zone.Identifier")
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn decompose_mac_xattrs_classifier_name_and_length() {
        // Wave-1 verified the AND check (`name == "com.apple.FinderInfo"
        // && len == 32`) was already correct. Lock that in: a 32-byte
        // *non-FinderInfo* xattr must NOT be classified, and a
        // *wrong-length* FinderInfo entry must NOT be classified.

        // (a) user.foo of length 32 — wrong NAME, must NOT classify.
        let entries = vec![XattrEntry {
            name: "user.foo".to_string(),
            value: vec![0xCC; 32],
        }];
        let mut snap = MetaSnapshot::default();
        decompose_mac_xattrs(entries, &mut snap);
        assert!(
            snap.mac_finder_info.is_none(),
            "user.foo of length 32 must not be classified as FinderInfo"
        );
        // The entry survives in the foreign_xattrs path.
        assert_eq!(snap.xattrs.len(), 1);

        // (b) com.apple.FinderInfo of length 16 — wrong LENGTH, must
        // NOT classify.
        let entries = vec![XattrEntry {
            name: "com.apple.FinderInfo".to_string(),
            value: vec![0xDD; 16],
        }];
        let mut snap = MetaSnapshot::default();
        decompose_mac_xattrs(entries, &mut snap);
        assert!(
            snap.mac_finder_info.is_none(),
            "com.apple.FinderInfo of length 16 must not be classified as FinderInfo"
        );
        assert_eq!(snap.xattrs.len(), 1);

        // (c) com.apple.FinderInfo of length 32 — correct, MUST
        // classify into the structured slot.
        let entries = vec![XattrEntry {
            name: "com.apple.FinderInfo".to_string(),
            value: vec![0xEE; 32],
        }];
        let mut snap = MetaSnapshot::default();
        decompose_mac_xattrs(entries, &mut snap);
        let fi = snap
            .mac_finder_info
            .as_ref()
            .expect("32-byte FinderInfo must be classified");
        assert_eq!(fi.data, [0xEE; 32]);
        // The raw entry is also retained in `xattrs` so the apply
        // side can replay the binary form unchanged.
        assert_eq!(snap.xattrs.len(), 1);
    }

    #[test]
    fn foreign_xattr_permitted_routes_per_toggle_and_not_per_and() {
        // Wave-1 swapped the old AND expression
        //   `policy.preserve_xattrs && !is_structured_xattr(name)`
        // for a per-toggle gate (`foreign_xattr_permitted`). The
        // regression here: with `preserve_xattrs == false` and
        // `preserve_selinux == true`, a `security.selinux` xattr MUST
        // still be routed through the AppleDouble fallback. And vice
        // versa — disabling `preserve_selinux` while keeping
        // `preserve_xattrs == true` MUST drop SELinux.

        // (a) Generic xattrs off, SELinux on → SELinux still routed.
        let policy = MetaPolicy {
            preserve_motw: false,
            preserve_xattrs: false,
            preserve_posix_acls: false,
            preserve_selinux: true,
            preserve_resource_forks: false,
            appledouble_fallback: true,
        };
        assert!(
            foreign_xattr_permitted("security.selinux", &policy),
            "selinux must pass through when preserve_selinux is on, even with preserve_xattrs off"
        );
        // A regular user.* xattr is dropped under that policy.
        assert!(!foreign_xattr_permitted("user.metadata", &policy));

        // (b) Generic xattrs on, SELinux off → SELinux dropped.
        let policy = MetaPolicy {
            preserve_motw: false,
            preserve_xattrs: true,
            preserve_posix_acls: false,
            preserve_selinux: false,
            preserve_resource_forks: false,
            appledouble_fallback: true,
        };
        assert!(
            !foreign_xattr_permitted("security.selinux", &policy),
            "selinux must drop when preserve_selinux is off, even with preserve_xattrs on"
        );
        // user.* still flows because preserve_xattrs is on.
        assert!(foreign_xattr_permitted("user.metadata", &policy));

        // (c) POSIX ACLs gate independently of preserve_xattrs.
        let policy = MetaPolicy {
            preserve_motw: false,
            preserve_xattrs: false,
            preserve_posix_acls: true,
            preserve_selinux: false,
            preserve_resource_forks: false,
            appledouble_fallback: true,
        };
        assert!(foreign_xattr_permitted("system.posix_acl_access", &policy));
        assert!(foreign_xattr_permitted("system.posix_acl_default", &policy));

        // (d) Resource forks gate the FinderInfo / ResourceFork
        // routes independently of preserve_xattrs.
        let policy = MetaPolicy {
            preserve_motw: false,
            preserve_xattrs: false,
            preserve_posix_acls: false,
            preserve_selinux: false,
            preserve_resource_forks: true,
            appledouble_fallback: true,
        };
        assert!(foreign_xattr_permitted("com.apple.ResourceFork", &policy));
        assert!(foreign_xattr_permitted("com.apple.FinderInfo", &policy));

        // (e) MOTW gates the Zone.Identifier route independently.
        let policy = MetaPolicy {
            preserve_motw: true,
            preserve_xattrs: false,
            preserve_posix_acls: false,
            preserve_selinux: false,
            preserve_resource_forks: false,
            appledouble_fallback: true,
        };
        assert!(foreign_xattr_permitted("Zone.Identifier", &policy));
        assert!(foreign_xattr_permitted("user.Zone.Identifier", &policy));
    }

    #[test]
    fn synth_entry_id_distinct_for_two_fnv_colliding_names() {
        // The wave-1 fix added `next_free_entry_id` linear probing so
        // two distinct names whose FNV-1a 32 hashes happen to alias
        // get distinct entry IDs (rather than the second silently
        // overwriting the first). Find a real collision via a short
        // brute-force search over 2-character ASCII strings, then
        // assert the probe.
        let chars: Vec<u8> = (b'a'..=b'z').chain(b'0'..=b'9').collect();
        let mut seen: std::collections::HashMap<u32, String> = std::collections::HashMap::new();
        let mut pair: Option<(String, String, u32)> = None;
        'outer: for length in [2usize, 3, 4] {
            seen.clear();
            // Iterate length-N tuples over the alphabet.
            let n = chars.len();
            let total = n.pow(length as u32);
            for i in 0..total {
                let mut s = Vec::with_capacity(length);
                let mut idx = i;
                for _ in 0..length {
                    s.push(chars[idx % n]);
                    idx /= n;
                }
                let s = String::from_utf8(s).unwrap();
                let h = fnv1a_32(&s);
                if let Some(other) = seen.get(&h) {
                    if other != &s {
                        pair = Some((other.clone(), s.clone(), h));
                        break 'outer;
                    }
                } else {
                    seen.insert(h, s);
                }
            }
        }
        let (a, b, h) =
            pair.expect("expected to find an FNV-1a collision among short alphanumeric strings");
        // Confirm the collision really is one — distinct names, same
        // hash.
        assert_ne!(a, b, "collision pair must be distinct strings");
        assert_eq!(fnv1a_32(&a), fnv1a_32(&b));
        assert_eq!(fnv1a_32(&a), h);

        // Now drive synth_entry_id and verify the second mints a
        // different ID via the linear probe.
        let mut used = std::collections::HashSet::new();
        let id_a = synth_entry_id(&a, &mut used);
        let id_b = synth_entry_id(&b, &mut used);
        assert_ne!(
            id_a, id_b,
            "two FNV-colliding names ({a:?}, {b:?}) must mint distinct IDs after linear probing"
        );
        // Both should be in the canonical-safe range.
        assert!(id_a >= 16);
        assert!(id_b >= 16);
    }

    #[test]
    fn appledouble_table_has_distinct_ids_for_fnv_colliding_names() {
        // End-to-end: write a sidecar with two FNV-colliding stream
        // names and verify the 12-byte entry-descriptor table at
        // offset 26 carries two distinct IDs.
        let chars: Vec<u8> = (b'a'..=b'z').chain(b'0'..=b'9').collect();
        let mut seen: std::collections::HashMap<u32, String> = std::collections::HashMap::new();
        let mut pair: Option<(String, String)> = None;
        'outer: for length in [2usize, 3, 4] {
            seen.clear();
            let n = chars.len();
            let total = n.pow(length as u32);
            for i in 0..total {
                let mut s = Vec::with_capacity(length);
                let mut idx = i;
                for _ in 0..length {
                    s.push(chars[idx % n]);
                    idx /= n;
                }
                let s = String::from_utf8(s).unwrap();
                let h = fnv1a_32(&s);
                if let Some(other) = seen.get(&h) {
                    if other != &s {
                        pair = Some((other.clone(), s));
                        break 'outer;
                    }
                } else {
                    seen.insert(h, s);
                }
            }
        }
        let (a, b) =
            pair.expect("expected to find an FNV-1a collision pair for the round-trip test");

        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("file.bin");
        std::fs::write(&dst, b"primary").unwrap();
        let foreign = vec![
            ForeignStream {
                name: a.clone(),
                payload: vec![0xAA; 4],
            },
            ForeignStream {
                name: b.clone(),
                payload: vec![0xBB; 4],
            },
        ];
        write_appledouble_sidecar(&dst, &foreign).unwrap();

        let bytes = std::fs::read(tmp.path().join("._file.bin")).unwrap();
        // Header = 4+4+16+2 = 26 bytes, then entry descriptors of 12
        // bytes each. The first descriptor's ID is at [26..30], the
        // second at [38..42].
        let id1 = u32::from_be_bytes(bytes[26..30].try_into().unwrap());
        let id2 = u32::from_be_bytes(bytes[38..42].try_into().unwrap());
        assert_ne!(
            id1, id2,
            "FNV-colliding names ({a:?}, {b:?}) must land in distinct AppleDouble entry IDs"
        );
    }
}
