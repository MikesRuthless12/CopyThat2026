//! Phase 17d smoke test — privilege-separated helper.
//!
//! Drives the helper's request/response surface end-to-end via
//! in-process transport (`Cursor`-backed reader + `Vec`-backed
//! writer). No real elevation; the protocol layer is what's
//! under test.
//!
//! Coverage:
//! 1. Hello handshake — version match + protocol mismatch.
//! 2. Capability gate — `ElevatedRetry` rejects without grant.
//! 3. Phase 17a — traversal-laden src or dst gets `PathRejected`.
//! 4. `ElevatedRetry` round-trips a real file copy.
//! 5. `HardwareErase` always returns `HardwareEraseUnavailable`
//!    (Phase 44 wires the actual ioctls).
//! 6. Pipe-name generator + parser refuse mismatched shapes
//!    — defence-in-depth against argv tampering.
//! 7. Wire labels round-trip (`ShellExtensionKind` /
//!    `Capability`).

use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use copythat_helper::capability::{Capability, parse_capability_list};
use copythat_helper::handler::handle_request;
use copythat_helper::rpc::{
    PROTOCOL_VERSION, Request, Response, ShellExtensionKind, generate_pipe_name, parse_pipe_name,
};
use copythat_helper::transport::{read_line, write_line};
use tempfile::TempDir;

fn full_caps() -> Vec<Capability> {
    parse_capability_list("elevated_retry,shell_extension,hardware_erase").unwrap()
}

#[test]
fn hello_with_correct_version_returns_session_id() {
    let r = handle_request(
        &Request::Hello {
            version: PROTOCOL_VERSION,
        },
        &[],
    );
    match r {
        Response::HelloOk {
            version,
            session_id,
        } => {
            assert_eq!(version, PROTOCOL_VERSION);
            assert_eq!(session_id.len(), 32);
            assert!(session_id.chars().all(|c| c.is_ascii_hexdigit()));
        }
        other => panic!("expected HelloOk, got {other:?}"),
    }
}

#[test]
fn hello_with_wrong_version_returns_protocol_mismatch() {
    let r = handle_request(&Request::Hello { version: 999 }, &[]);
    assert!(matches!(r, Response::ProtocolMismatch { .. }));
}

#[test]
fn elevated_retry_without_capability_is_denied() {
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
fn traversal_in_request_gets_path_rejected() {
    let r = handle_request(
        &Request::ElevatedRetry {
            src: PathBuf::from("foo/../etc/passwd"),
            dst: PathBuf::from("/tmp/dst"),
        },
        &full_caps(),
    );
    match r {
        Response::PathRejected { localized_key, .. } => {
            assert_eq!(localized_key, "err-path-escape");
        }
        other => panic!("expected PathRejected, got {other:?}"),
    }
}

#[test]
fn elevated_retry_copies_a_real_file() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    std::fs::write(&src, b"hello world").unwrap();

    let r = handle_request(
        &Request::ElevatedRetry {
            src: src.clone(),
            dst: dst.clone(),
        },
        &full_caps(),
    );
    match r {
        Response::ElevatedRetryOk { bytes } => assert_eq!(bytes, 11),
        other => panic!("expected ElevatedRetryOk, got {other:?}"),
    }
    assert_eq!(std::fs::read(&dst).unwrap(), b"hello world");
}

#[test]
fn hardware_erase_returns_unavailable_with_pointer() {
    let r = handle_request(
        &Request::HardwareErase {
            device: PathBuf::from(if cfg!(windows) {
                r"\\.\PhysicalDrive0"
            } else {
                "/dev/nvme0n1"
            }),
        },
        &full_caps(),
    );
    match r {
        Response::HardwareEraseUnavailable { reason } => {
            assert!(reason.contains("Phase 44"));
        }
        other => panic!("expected HardwareEraseUnavailable, got {other:?}"),
    }
}

#[test]
fn shell_extension_kind_native_to_current_host_is_correct() {
    #[cfg(target_os = "windows")]
    {
        assert!(ShellExtensionKind::WindowsExplorerCommand.is_native_to_current_host());
        assert!(!ShellExtensionKind::MacosFinderSync.is_native_to_current_host());
    }
    #[cfg(target_os = "linux")]
    {
        assert!(ShellExtensionKind::LinuxNautilus.is_native_to_current_host());
        assert!(!ShellExtensionKind::WindowsExplorerCommand.is_native_to_current_host());
    }
    #[cfg(target_os = "macos")]
    {
        assert!(ShellExtensionKind::MacosFinderSync.is_native_to_current_host());
    }
}

#[test]
fn pipe_name_generator_round_trips_through_parser() {
    let prefix = if cfg!(windows) {
        r"\\.\pipe\copythat-helper-"
    } else {
        "/tmp/copythat-helper-"
    };
    let name = generate_pipe_name(prefix).unwrap();
    let suffix = parse_pipe_name(prefix, &name).unwrap();
    assert_eq!(suffix.len(), 64);
}

#[test]
fn pipe_name_parser_rejects_mismatched_shape() {
    let prefix = r"\\.\pipe\copythat-helper-";
    assert!(parse_pipe_name(prefix, "not-a-helper-pipe").is_none());
    let too_short = format!("{prefix}deadbeef");
    assert!(parse_pipe_name(prefix, &too_short).is_none());
}

#[test]
fn capability_wire_labels_unique_and_round_trip() {
    let caps = [
        Capability::Lifecycle,
        Capability::ElevatedRetry,
        Capability::ShellExtension,
        Capability::HardwareErase,
    ];
    let labels: Vec<&str> = caps.iter().map(|c| c.wire_label()).collect();
    let mut seen = std::collections::HashSet::new();
    for l in &labels {
        assert!(seen.insert(*l), "duplicate label {l}");
    }
    for c in caps {
        assert_eq!(Capability::parse(c.wire_label()), Some(c));
    }
}

#[test]
fn transport_round_trips_request_and_response() {
    let mut wire: Vec<u8> = Vec::new();
    write_line(
        &mut wire,
        &Request::Hello {
            version: PROTOCOL_VERSION,
        },
    )
    .unwrap();
    let mut reader = BufReader::new(Cursor::new(wire));
    let req: Request = read_line(&mut reader).unwrap();
    assert_eq!(
        req,
        Request::Hello {
            version: PROTOCOL_VERSION,
        }
    );
}
