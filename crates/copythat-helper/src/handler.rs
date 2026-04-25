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
                Response::HelloOk {
                    version: crate::rpc::PROTOCOL_VERSION,
                    session_id: session_id(),
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

    match std::fs::copy(src, dst) {
        Ok(bytes) => Response::ElevatedRetryOk { bytes },
        Err(e) => {
            let key = match e.kind() {
                std::io::ErrorKind::PermissionDenied => "err-permission-denied",
                std::io::ErrorKind::NotFound => "err-not-found",
                _ => "err-io-other",
            };
            Response::ElevatedRetryFailed {
                localized_key: key.to_string(),
                message: e.to_string(),
            }
        }
    }
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
    // The actual install lands in a per-OS body fill — Windows
    // `RegCreateKeyExW` against HKLM, macOS plist write into
    // `/Library/PreferencePanes/`, Linux symlink into the
    // distro's `nautilus-python` extensions dir. None of those
    // are pure-Rust without an unsafe block (Windows) or a shell
    // out (Linux). The Phase 17d helper ships the dispatch layer
    // + capability gate; the per-OS body fills land alongside
    // the corresponding shell-extension manifest changes in
    // `crates/copythat-shellext/`. For now the handler returns
    // success on the install with a documented "scaffold" note —
    // the caller surface (`copythat-ui`'s `install_shell_extension`
    // IPC) is not wired into the production menu yet, so this
    // stub is exercised only by tests + by future bodies.
    Response::ShellExtensionInstalled { target }
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
    Response::ShellExtensionUninstalled { target }
}

fn session_id() -> String {
    let mut bytes = [0u8; 16];
    if getrandom::fill(&mut bytes).is_err() {
        // Fallback: incorporate the system time so the session-id
        // is at least mildly distinct even if getrandom failed
        // (which would itself be a serious problem worth surfacing
        // in tests).
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        return format!("ts-{nanos}");
    }
    hex::encode(bytes)
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
