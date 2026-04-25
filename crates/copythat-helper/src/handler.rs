//! Per-request handler — translates a `Request` into a
//! `Response`. Pure-Rust + `#![forbid(unsafe_code)]`-clean (the
//! lib already declares this); the actual elevated work
//! (`std::fs::copy` for elevated retry, `RegSetKeyValueW` for
//! shell-extension install) runs through std-only surfaces. When
//! the OS surface itself needs unsafe (e.g. NVMe Sanitize ioctl
//! for `HardwareErase`), the handler returns a typed
//! "unavailable" response and lets the caller fall through —
//! Phase 44 wires the actual ioctls behind a separate feature
//! gate.
//!
//! The handler is deliberately stateless. Per-session bookkeeping
//! (which shell extensions the helper installed during this
//! session, etc.) lives one layer up in the binary
//! (`bin/helper.rs`) so this module is unit-testable without a
//! pipe.

use copythat_core::safety::no_follow_open_flags;
use copythat_core::validate_path_no_traversal;

use crate::capability::{Capability, check};
use crate::rpc::{Request, Response, ShellExtensionKind};

/// Dispatch a single request. Returns the response the binary
/// will write back over the pipe.
pub fn handle_request(request: &Request, granted: &[Capability]) -> Response {
    // Phase 17a — every path-typed request gets a lexical traversal
    // guard before the handler dispatches. The IPC layer in the
    // main app already gates input, but defence-in-depth: never
    // trust a path arriving from an external process, even one we
    // spawned ourselves.
    if let Some(off) = path_to_validate(request) {
        if let Err(err) = validate_path_no_traversal(off) {
            return Response::PathRejected {
                offending: off.to_path_buf(),
                localized_key: err.localized_key().to_string(),
            };
        }
    }

    // Capability gate.
    if let Err(e) = check(request, granted) {
        return Response::CapabilityDenied { reason: e.reason() };
    }

    match request {
        Request::Hello { version } => {
            if *version == crate::rpc::PROTOCOL_VERSION {
                match session_id() {
                    Ok(sid) => Response::HelloOk {
                        version: crate::rpc::PROTOCOL_VERSION,
                        session_id: sid,
                    },
                    Err(e) => Response::Failed {
                        localized_key: "err-randomness-unavailable".to_string(),
                        message: format!("session-id mint failed: {e}"),
                    },
                }
            } else {
                Response::ProtocolMismatch {
                    helper_version: crate::rpc::PROTOCOL_VERSION,
                    caller_version: *version,
                }
            }
        }
        Request::Shutdown => Response::ShuttingDown,
        Request::ElevatedRetry { src, dst } => handle_elevated_retry(src, dst),
        Request::InstallShellExtension { target } => handle_install_shell_extension(*target),
        Request::UninstallShellExtension { target } => handle_uninstall_shell_extension(*target),
        Request::HardwareErase { device } => Response::HardwareEraseUnavailable {
            reason: format!(
                "Phase 44 will wire the per-OS ioctls (NVMe Sanitize / OPAL Crypto Erase / \
                 ATA Secure Erase) for {}. Until then, use Clear-method shred + FDE \
                 rotation — see docs/SECURITY.md § Phase 4 for the workflow.",
                device.display()
            ),
        },
    }
}

fn path_to_validate(req: &Request) -> Option<&std::path::Path> {
    match req {
        Request::ElevatedRetry { src, .. } => Some(src.as_path()),
        Request::HardwareErase { device } => Some(device.as_path()),
        _ => None,
    }
}

fn handle_elevated_retry(src: &std::path::Path, dst: &std::path::Path) -> Response {
    // Phase 17a — re-validate dst (the path-to-validate helper
    // returns src; dst gets its own pass).
    if let Err(err) = validate_path_no_traversal(dst) {
        return Response::PathRejected {
            offending: dst.to_path_buf(),
            localized_key: err.localized_key().to_string(),
        };
    }

    // Phase 17c — refuse to follow a symlink at `src`. `std::fs::copy`
    // would silently follow whatever the link points at, which under
    // an elevated token means an attacker who can swap `src` for a
    // symlink between the unprivileged main app's path validation and
    // the helper's copy can read any file the elevated token can. Open
    // src with O_NOFOLLOW (Unix) / FILE_FLAG_OPEN_REPARSE_POINT
    // (Windows), then byte-copy into dst.
    match copy_no_follow(src, dst) {
        Ok(bytes) => Response::ElevatedRetryOk { bytes },
        Err(e) => {
            let key = if copythat_core::safety::is_no_follow_rejection(&e) {
                "err-path-escape"
            } else {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => "err-permission-denied",
                    std::io::ErrorKind::NotFound => "err-not-found",
                    _ => "err-io-other",
                }
            };
            Response::ElevatedRetryFailed {
                localized_key: key.to_string(),
                message: e.to_string(),
            }
        }
    }
}

/// Open both `src` and `dst` with the platform's no-follow flag set,
/// then stream src's bytes into dst. Returns the number of bytes
/// copied. Refuses to follow a symlink at *either* end so an
/// attacker who races a symlink onto the destination path between
/// the caller's lexical validation and this open can't redirect an
/// elevated write to a victim file (e.g. `/etc/sudoers`,
/// `C:\Windows\System32\drivers\etc\hosts`). The src side closes the
/// equivalent read-redirect race.
///
/// Today the helper runs in-process at the unprivileged caller's
/// privilege level — but the per-OS spawn helper (Phase 17d body
/// fill) will activate this trust boundary, and the no-follow flag
/// must already be there when it does.
fn copy_no_follow(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<u64> {
    use std::io::{Read, Write};

    let flags = no_follow_open_flags();
    let mut src_opts = std::fs::OpenOptions::new();
    src_opts.read(true);
    let mut dst_opts = std::fs::OpenOptions::new();
    dst_opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        src_opts.custom_flags(flags as i32);
        dst_opts.custom_flags(flags as i32);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        src_opts.custom_flags(flags);
        dst_opts.custom_flags(flags);
    }
    let mut sf = src_opts.open(src)?;
    let mut df = dst_opts.open(dst)?;
    let mut buf = [0u8; 64 * 1024];
    let mut total: u64 = 0;
    loop {
        let n = sf.read(&mut buf)?;
        if n == 0 {
            break;
        }
        df.write_all(&buf[..n])?;
        total = total.saturating_add(n as u64);
    }
    df.flush()?;
    Ok(total)
}

fn handle_install_shell_extension(target: ShellExtensionKind) -> Response {
    if !target.is_native_to_current_host() {
        return Response::ShellExtensionUnsupported {
            target,
            reason: format!(
                "{} is not a native shell extension for {} — refused at the helper",
                target.wire_label(),
                std::env::consts::OS
            ),
        };
    }
    // The per-OS body fills (Windows `RegCreateKeyExW` against HKLM,
    // macOS plist write under `/Library/PreferencePanes/`, Linux
    // symlink into the distro's extensions dir) are not yet
    // implemented. Returning `ShellExtensionInstalled` here would lie
    // to the caller — the UI would flip to "installed" while the
    // registry/plist/symlink remained absent. Surface as
    // `ShellExtensionUnsupported` with a clear "scaffold" reason so
    // the caller treats it as a no-op until the body fill lands.
    Response::ShellExtensionUnsupported {
        target,
        reason: format!(
            "{} install scaffolded but not yet implemented — the per-OS body fill (Phase 17d follow-up) lands alongside the shell-extension manifest changes in `crates/copythat-shellext/`",
            target.wire_label()
        ),
    }
}

fn handle_uninstall_shell_extension(target: ShellExtensionKind) -> Response {
    if !target.is_native_to_current_host() {
        return Response::ShellExtensionUnsupported {
            target,
            reason: format!(
                "{} is not a native shell extension for {} — refused at the helper",
                target.wire_label(),
                std::env::consts::OS
            ),
        };
    }
    // Symmetric with install — until the per-OS body fill lands the
    // helper cannot honestly claim to have uninstalled anything.
    Response::ShellExtensionUnsupported {
        target,
        reason: format!(
            "{} uninstall scaffolded but not yet implemented — the per-OS body fill (Phase 17d follow-up) lands alongside `crates/copythat-shellext/`",
            target.wire_label()
        ),
    }
}

fn session_id() -> Result<String, std::io::Error> {
    let mut bytes = [0u8; 16];
    // Phase 17d — fail closed when CSPRNG entropy is unavailable.
    // The previous fallback synthesised a `ts-{nanos}` id from the
    // wall clock, giving an attacker who can observe (or roughly
    // approximate) the time-of-spawn a directly-guessable session
    // token. If a future caller branches replay-protection or
    // capability-correlation on the session id, that fallback
    // becomes a forgery primitive against an elevated helper.
    getrandom::fill(&mut bytes)
        .map_err(|e| std::io::Error::other(format!("getrandom failed: {e}")))?;
    Ok(hex::encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn all_caps() -> Vec<Capability> {
        vec![
            Capability::Lifecycle,
            Capability::ElevatedRetry,
            Capability::ShellExtension,
            Capability::HardwareErase,
        ]
    }

    #[test]
    fn hello_with_correct_version_returns_hello_ok() {
        let r = handle_request(
            &Request::Hello {
                version: crate::rpc::PROTOCOL_VERSION,
            },
            &all_caps(),
        );
        match r {
            Response::HelloOk { version, .. } => assert_eq!(version, crate::rpc::PROTOCOL_VERSION),
            other => panic!("expected HelloOk, got {other:?}"),
        }
    }

    #[test]
    fn hello_with_wrong_version_returns_protocol_mismatch() {
        let r = handle_request(&Request::Hello { version: 9999 }, &all_caps());
        assert!(matches!(r, Response::ProtocolMismatch { .. }));
    }

    #[test]
    fn shutdown_is_acknowledged() {
        let r = handle_request(&Request::Shutdown, &all_caps());
        assert_eq!(r, Response::ShuttingDown);
    }

    #[test]
    fn capability_denied_for_request_without_grant() {
        let r = handle_request(
            &Request::ElevatedRetry {
                src: PathBuf::from("/a"),
                dst: PathBuf::from("/b"),
            },
            &[],
        );
        assert!(matches!(r, Response::CapabilityDenied { .. }));
    }

    #[test]
    fn traversal_path_rejected_before_capability_check() {
        // Even with full caps, a traversal-laden request fails at
        // the Phase 17a gate.
        let r = handle_request(
            &Request::ElevatedRetry {
                src: PathBuf::from("foo/../etc/passwd"),
                dst: PathBuf::from("/tmp/dst"),
            },
            &all_caps(),
        );
        match r {
            Response::PathRejected { localized_key, .. } => {
                assert_eq!(localized_key, "err-path-escape");
            }
            other => panic!("expected PathRejected, got {other:?}"),
        }
    }

    #[test]
    fn hardware_erase_returns_unavailable_with_pointer() {
        let r = handle_request(
            &Request::HardwareErase {
                device: PathBuf::from("/dev/nvme0n1"),
            },
            &all_caps(),
        );
        match r {
            Response::HardwareEraseUnavailable { reason } => {
                assert!(reason.contains("Phase 44"));
                assert!(reason.contains("FDE rotation") || reason.contains("Clear"));
            }
            other => panic!("expected HardwareEraseUnavailable, got {other:?}"),
        }
    }

    #[test]
    fn install_unsupported_kind_for_host_surfaces_unsupported() {
        // A Windows host asked to install the macOS Finder Sync
        // bundle should refuse rather than pretending to succeed.
        #[cfg(not(target_os = "macos"))]
        {
            let r = handle_request(
                &Request::InstallShellExtension {
                    target: ShellExtensionKind::MacosFinderSync,
                },
                &all_caps(),
            );
            assert!(matches!(r, Response::ShellExtensionUnsupported { .. }));
        }
    }

    #[test]
    fn elevated_retry_round_trips_a_real_file() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");
        std::fs::write(&src, b"hello").unwrap();
        let r = handle_request(
            &Request::ElevatedRetry {
                src: src.clone(),
                dst: dst.clone(),
            },
            &all_caps(),
        );
        match r {
            Response::ElevatedRetryOk { bytes } => assert_eq!(bytes, 5),
            other => panic!("expected ElevatedRetryOk, got {other:?}"),
        }
        assert_eq!(std::fs::read(&dst).unwrap(), b"hello");
    }
}
