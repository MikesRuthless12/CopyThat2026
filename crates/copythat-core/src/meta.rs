//! Phase 24 — security-metadata preservation.
//!
//! Engine-side surface for capturing and re-applying out-of-band file
//! metadata that the Phase 1–6 byte/permission/timestamp passes do not
//! cover:
//!
//! - **Windows NTFS Alternate Data Streams**, including the
//!   `Zone.Identifier` Mark-of-the-Web stream that SmartScreen and
//!   Office Protected View key off.
//! - **Linux extended attributes** (`user.*`, `security.*`, `trusted.*`).
//!   The two POSIX-ACL xattrs (`system.posix_acl_access` /
//!   `system.posix_acl_default`) and the SELinux context
//!   (`security.selinux`) and the file-capabilities blob
//!   (`security.capability`) are broken out into structured
//!   sub-blobs so the UI can render them without a re-parse.
//! - **macOS extended attributes** (`com.apple.*` family — Gatekeeper
//!   `com.apple.quarantine`, Spotlight provenance
//!   `com.apple.metadata:kMDItemWhereFroms`, FinderInfo, ResourceFork).
//! - **macOS resource fork** (the legacy `<file>/..namedfork/rsrc`
//!   stream — kept distinct from the xattr surface because some
//!   tools refuse to round-trip it through xattrs).
//!
//! This module stays `#![forbid(unsafe_code)]`-clean: the platform
//! syscalls (`FindFirstStreamW` / `BackupRead` on Windows, `xattr` /
//! `getxattr` on Unix) live in `copythat-platform::meta` behind the
//! [`MetaOps`] trait object. Tests can plug in an in-memory
//! implementation; the engine calls the trait without any FFI.
//!
//! # Wire model
//!
//! [`MetaSnapshot`] is a *neutral* representation: every per-OS hook
//! captures whatever it can read on the source side, the engine carries
//! the snapshot to the destination, and the destination-side hook tries
//! to apply it. When the destination's filesystem can't hold the foreign
//! metadata (e.g. macOS resource fork on a Windows SMB share) and the
//! caller opted into [`MetaPolicy::appledouble_fallback`], the platform
//! hook serialises the foreign streams into an AppleDouble sidecar
//! (`._<filename>`) and surfaces a
//! [`CopyEvent::MetaTranslatedToAppleDouble`](crate::CopyEvent::MetaTranslatedToAppleDouble)
//! on the events channel.

use std::path::Path;
use std::sync::Arc;

/// One Windows NTFS Alternate Data Stream attached to a file.
///
/// `name` is the stream name *without* the leading `:` and the trailing
/// `:$DATA` type — e.g. `"Zone.Identifier"`, not
/// `":Zone.Identifier:$DATA"`. The default unnamed stream (the file's
/// primary content) is never represented here; capturing it would
/// double-write the bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsStream {
    pub name: String,
    pub data: Vec<u8>,
}

/// One extended attribute (Linux / macOS).
///
/// `name` is the full namespaced key as the kernel sees it
/// (`user.foo`, `security.selinux`, `com.apple.quarantine`); the engine
/// does not strip namespaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XattrEntry {
    pub name: String,
    pub value: Vec<u8>,
}

/// POSIX ACL blob — opaque on the wire (the kernel's binary
/// `system.posix_acl_access` / `system.posix_acl_default` payload),
/// plus the human-readable rendering the UI shows.
///
/// Both fields are `Option` because a file may carry only the access
/// ACL (most common) or only the default ACL (directories with default
/// inheritance). When both are `None` the snapshot did not capture an
/// ACL at all.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PosixAclBlob {
    pub access_acl: Option<Vec<u8>>,
    pub default_acl: Option<Vec<u8>>,
}

/// SELinux security context — the value of `security.selinux`, kept as
/// a UTF-8 string so the UI can show it verbatim
/// (`unconfined_u:object_r:user_home_t:s0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeLinuxContext {
    pub context: String,
}

/// Linux file capabilities — the raw `security.capability` xattr.
/// Kept opaque so we don't have to track every kernel `vfs_cap_data`
/// version; the apply side just writes the bytes back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileCaps {
    pub raw: Vec<u8>,
}

/// macOS resource fork — the bytes of `<file>/..namedfork/rsrc`.
/// Empty / `None` for the vast majority of modern files (resource
/// forks are largely a Carbon-era artifact).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceForkBlob {
    pub data: Vec<u8>,
}

/// macOS Finder info — the fixed 32-byte `com.apple.FinderInfo`
/// xattr. Carries Finder color tags, kind/creator, and other Carbon
/// metadata. We keep the canonical 32-byte width even though some
/// xattr surfaces present it as a variable-length blob.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FinderInfoBlob {
    pub data: [u8; 32],
}

/// Aggregate of every out-of-band metadata stream the engine knows
/// about.
///
/// Every field is independently optional / empty. A snapshot from a
/// vanilla Linux file will populate `xattrs` and possibly
/// `posix_acl` / `selinux`, leaving the `mac_*` and `ads` fields
/// empty. A snapshot from a Windows file with MOTW populates `ads`
/// with a single `Zone.Identifier` entry and leaves the rest empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MetaSnapshot {
    pub ads: Vec<NtfsStream>,
    pub xattrs: Vec<XattrEntry>,
    pub posix_acl: Option<PosixAclBlob>,
    pub selinux: Option<SeLinuxContext>,
    pub linux_caps: Option<FileCaps>,
    pub mac_resource_fork: Option<ResourceForkBlob>,
    pub mac_finder_info: Option<FinderInfoBlob>,
}

impl MetaSnapshot {
    /// `true` when the snapshot has no captured metadata of any kind.
    /// The engine uses this to skip the apply call entirely on
    /// vanilla files (the common case for fresh downloads on Linux).
    pub fn is_empty(&self) -> bool {
        self.ads.is_empty()
            && self.xattrs.is_empty()
            && self.posix_acl.is_none()
            && self.selinux.is_none()
            && self.linux_caps.is_none()
            && self.mac_resource_fork.is_none()
            && self.mac_finder_info.is_none()
    }

    /// `true` when the snapshot includes a Windows
    /// `Zone.Identifier` ADS — the Mark-of-the-Web stream that
    /// SmartScreen / Office Protected View key off.
    pub fn has_motw(&self) -> bool {
        self.ads
            .iter()
            .any(|s| s.name.eq_ignore_ascii_case("Zone.Identifier"))
    }
}

/// Per-toggle policy that gates which metadata streams the apply pass
/// will write back to the destination.
///
/// Defaults to "preserve everything" — the engine and the Settings UI
/// surface explicit opt-outs for the user. The MOTW toggle is
/// security-sensitive: turning it off lets a downloaded EXE shed its
/// Mark-of-the-Web on copy, which bypasses SmartScreen. The Settings
/// UI carries an explicit warning tooltip on that one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaPolicy {
    pub preserve_motw: bool,
    pub preserve_xattrs: bool,
    pub preserve_posix_acls: bool,
    pub preserve_selinux: bool,
    pub preserve_resource_forks: bool,
    /// On a cross-FS copy where the destination filesystem can't hold
    /// the foreign metadata (e.g. SMB / FAT / exFAT / ext4 receiving a
    /// macOS resource fork), serialise the foreign streams into an
    /// `._<filename>` AppleDouble sidecar so the data is recoverable
    /// when the file lands back on a capable FS.
    pub appledouble_fallback: bool,
}

impl Default for MetaPolicy {
    fn default() -> Self {
        Self {
            preserve_motw: true,
            preserve_xattrs: true,
            preserve_posix_acls: true,
            preserve_selinux: true,
            preserve_resource_forks: true,
            appledouble_fallback: true,
        }
    }
}

impl MetaPolicy {
    /// Filter `snapshot` in place down to only the streams this
    /// policy permits. Used by the engine before handing the snapshot
    /// to the apply hook so the per-OS code can stay policy-agnostic.
    pub fn filter(&self, snapshot: &mut MetaSnapshot) {
        if !self.preserve_motw {
            snapshot
                .ads
                .retain(|s| !s.name.eq_ignore_ascii_case("Zone.Identifier"));
        }
        if !self.preserve_xattrs {
            // user.* and com.apple.* and trusted.* are dropped, but
            // SELinux / POSIX ACLs / file caps continue to follow
            // their dedicated toggles.
            snapshot.xattrs.retain(|x| {
                let n = &x.name;
                n.starts_with("system.posix_acl_")
                    || n == "security.selinux"
                    || n == "security.capability"
            });
        }
        if !self.preserve_posix_acls {
            snapshot.posix_acl = None;
            snapshot
                .xattrs
                .retain(|x| !x.name.starts_with("system.posix_acl_"));
        }
        if !self.preserve_selinux {
            snapshot.selinux = None;
            snapshot.xattrs.retain(|x| x.name != "security.selinux");
        }
        if !self.preserve_resource_forks {
            snapshot.mac_resource_fork = None;
            snapshot.mac_finder_info = None;
            snapshot
                .xattrs
                .retain(|x| x.name != "com.apple.ResourceFork" && x.name != "com.apple.FinderInfo");
        }
    }
}

/// Outcome the apply hook reports back to the engine.
///
/// `translated_to_appledouble` flips to `true` when the hook fell
/// through to the `._<filename>` sidecar fallback because the
/// destination FS could not hold the foreign metadata. The engine
/// emits one [`CopyEvent::MetaTranslatedToAppleDouble`](crate::CopyEvent::MetaTranslatedToAppleDouble)
/// in that case so the UI can render an info badge on the row.
#[derive(Debug, Clone, Default)]
pub struct MetaApplyOutcome {
    pub translated_to_appledouble: bool,
    /// Stable wire string for the AppleDouble translation event:
    /// the source file's extension (lowercase, no leading dot —
    /// `"docx"`, `"jpg"`). `"none"` for files with no extension.
    /// Empty when no translation happened.
    pub translated_extension: String,
    /// Per-stream errors encountered during apply. The hook
    /// continues past individual stream failures so a single
    /// permission-denied xattr write does not lose the rest of
    /// the snapshot. The engine drops these into the structured
    /// log when present and surfaces a single rolled-up info
    /// event to the UI.
    pub partial_failures: Vec<String>,
}

/// Bridge contract for the platform's security-metadata capture and
/// apply syscalls.
///
/// Implemented by `copythat_platform::PlatformMetaOps`. The engine
/// holds an `Arc<dyn MetaOps>` so callers (the Tauri shell, the CLI,
/// test harnesses) can plug in alternate backends without pulling in
/// the platform crate's unsafe FFI.
///
/// All methods run on a `tokio::task::spawn_blocking` worker — the
/// underlying syscalls are blocking by design (`xattr` listing on
/// Linux walks a kernel buffer; `FindFirstStreamW` on Windows opens a
/// handle and iterates).
pub trait MetaOps: Send + Sync + std::fmt::Debug + 'static {
    /// Capture every metadata stream the platform exposes for `path`.
    /// Returns an empty snapshot when the file has no out-of-band
    /// metadata (the common case for vanilla Linux files).
    fn capture(&self, path: &Path) -> std::io::Result<MetaSnapshot>;

    /// Apply `snapshot` to `path`, honouring `policy`. Streams the
    /// platform doesn't recognise (e.g. macOS resource fork on a
    /// Linux destination) are routed into an AppleDouble sidecar
    /// when `policy.appledouble_fallback` is `true`; otherwise they
    /// are silently dropped.
    fn apply(
        &self,
        path: &Path,
        snapshot: &MetaSnapshot,
        policy: &MetaPolicy,
    ) -> std::io::Result<MetaApplyOutcome>;
}

/// A trivial [`MetaOps`] that captures nothing and applies nothing.
/// Useful as a test stub and as the zero-config default when the
/// caller doesn't wire in the platform hook.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopMetaOps;

impl MetaOps for NoopMetaOps {
    fn capture(&self, _path: &Path) -> std::io::Result<MetaSnapshot> {
        Ok(MetaSnapshot::default())
    }

    fn apply(
        &self,
        _path: &Path,
        _snapshot: &MetaSnapshot,
        _policy: &MetaPolicy,
    ) -> std::io::Result<MetaApplyOutcome> {
        Ok(MetaApplyOutcome::default())
    }
}

/// Convenience: capture + apply via an arc'd hook in one shot. Used
/// by the engine after the byte copy finishes; saves the per-call site
/// from juggling two spawn_blocking shells.
pub async fn transfer(
    ops: Arc<dyn MetaOps>,
    src: std::path::PathBuf,
    dst: std::path::PathBuf,
    policy: MetaPolicy,
) -> std::io::Result<MetaApplyOutcome> {
    tokio::task::spawn_blocking(move || {
        let mut snap = ops.capture(&src)?;
        if snap.is_empty() {
            return Ok(MetaApplyOutcome::default());
        }
        policy.filter(&mut snap);
        if snap.is_empty() {
            return Ok(MetaApplyOutcome::default());
        }
        ops.apply(&dst, &snap, &policy)
    })
    .await
    .map_err(|e| std::io::Error::other(format!("meta transfer join: {e}")))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_is_empty() {
        let s = MetaSnapshot::default();
        assert!(s.is_empty());
        assert!(!s.has_motw());
    }

    #[test]
    fn motw_detected_case_insensitive() {
        let s = MetaSnapshot {
            ads: vec![NtfsStream {
                name: "zone.identifier".to_string(),
                data: b"[ZoneTransfer]\nZoneId=3".to_vec(),
            }],
            ..MetaSnapshot::default()
        };
        assert!(s.has_motw());
        assert!(!s.is_empty());
    }

    #[test]
    fn policy_default_preserves_everything() {
        let p = MetaPolicy::default();
        assert!(p.preserve_motw);
        assert!(p.preserve_xattrs);
        assert!(p.preserve_posix_acls);
        assert!(p.preserve_selinux);
        assert!(p.preserve_resource_forks);
        assert!(p.appledouble_fallback);
    }

    #[test]
    fn policy_filter_drops_motw_when_off() {
        let mut s = MetaSnapshot {
            ads: vec![
                NtfsStream {
                    name: "Zone.Identifier".to_string(),
                    data: b"[ZoneTransfer]\nZoneId=3".to_vec(),
                },
                NtfsStream {
                    name: "OtherStream".to_string(),
                    data: vec![0xAB; 16],
                },
            ],
            ..MetaSnapshot::default()
        };
        let p = MetaPolicy {
            preserve_motw: false,
            ..MetaPolicy::default()
        };
        p.filter(&mut s);
        assert_eq!(s.ads.len(), 1);
        assert_eq!(s.ads[0].name, "OtherStream");
    }

    #[test]
    fn policy_filter_drops_xattrs_but_keeps_acls_selinux_caps() {
        let mut s = MetaSnapshot {
            xattrs: vec![
                XattrEntry {
                    name: "user.foo".to_string(),
                    value: b"bar".to_vec(),
                },
                XattrEntry {
                    name: "system.posix_acl_access".to_string(),
                    value: vec![0xAA; 4],
                },
                XattrEntry {
                    name: "security.selinux".to_string(),
                    value: b"unconfined_u:object_r:user_home_t:s0".to_vec(),
                },
                XattrEntry {
                    name: "security.capability".to_string(),
                    value: vec![0xBB; 8],
                },
            ],
            ..MetaSnapshot::default()
        };
        let p = MetaPolicy {
            preserve_xattrs: false,
            ..MetaPolicy::default()
        };
        p.filter(&mut s);
        // user.foo dropped; the three structured ones stay.
        assert_eq!(s.xattrs.len(), 3);
        assert!(s.xattrs.iter().all(|x| x.name != "user.foo"));
    }

    #[test]
    fn policy_filter_drops_resource_forks_via_xattrs_too() {
        let mut s = MetaSnapshot {
            mac_resource_fork: Some(ResourceForkBlob {
                data: vec![0xAB; 16],
            }),
            mac_finder_info: Some(FinderInfoBlob::default()),
            xattrs: vec![
                XattrEntry {
                    name: "com.apple.ResourceFork".to_string(),
                    value: vec![0xAB; 16],
                },
                XattrEntry {
                    name: "com.apple.quarantine".to_string(),
                    value: b"q;1234".to_vec(),
                },
            ],
            ..MetaSnapshot::default()
        };
        let p = MetaPolicy {
            preserve_resource_forks: false,
            ..MetaPolicy::default()
        };
        p.filter(&mut s);
        assert!(s.mac_resource_fork.is_none());
        assert!(s.mac_finder_info.is_none());
        // ResourceFork xattr drops, quarantine survives (it's a
        // separate concept — Gatekeeper provenance).
        assert!(s.xattrs.iter().any(|x| x.name == "com.apple.quarantine"));
        assert!(s.xattrs.iter().all(|x| x.name != "com.apple.ResourceFork"));
    }

    #[tokio::test]
    async fn transfer_via_noop_returns_default_outcome() {
        let ops: Arc<dyn MetaOps> = Arc::new(NoopMetaOps);
        let outcome = transfer(
            ops,
            std::path::PathBuf::from("/tmp/src"),
            std::path::PathBuf::from("/tmp/dst"),
            MetaPolicy::default(),
        )
        .await
        .unwrap();
        assert!(!outcome.translated_to_appledouble);
        assert!(outcome.partial_failures.is_empty());
    }
}
