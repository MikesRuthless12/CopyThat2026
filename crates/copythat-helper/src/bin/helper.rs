//! `copythat-helper` binary entry point.
//!
//! Spawned by the main `copythat-ui` process via the OS-native
//! elevation flow. Reads JSON-RPC requests from stdin and writes
//! responses to stdout — pipe / socket plumbing happens on the
//! caller side, the helper just speaks line-delimited JSON over
//! the standard streams.
//!
//! This binary is **never user-facing** — running it directly is
//! a no-op that reads from a tty and exits as soon as stdin
//! closes. The CLAUDE.md "executing actions with care" rule is
//! enforced by the capability allowlist + the Phase 17a path
//! safety bar; both run before any privileged action.

#![forbid(unsafe_code)]

use std::io::{BufWriter, stdin, stdout};

use copythat_helper::capability::{Capability, parse_capability_list};
use copythat_helper::handler::handle_request;
use copythat_helper::rpc::{Request, Response};
use copythat_helper::transport::{TransportError, buf_reader, read_line, write_line};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argv_requested = match resolve_capabilities(&args) {
        Ok(caps) => caps,
        Err(e) => {
            eprintln!("copythat-helper: {e}");
            std::process::exit(2);
        }
    };

    let mut reader = buf_reader(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let exit_code = match run_loop(&mut reader, &mut writer, &argv_requested) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("copythat-helper: transport error: {e}");
            3
        }
    };
    std::process::exit(exit_code);
}

fn resolve_capabilities(args: &[String]) -> Result<Vec<Capability>, String> {
    let raw = args
        .iter()
        .find_map(|a| a.strip_prefix("--capabilities=").map(|s| s.to_string()));
    match raw {
        Some(list) => parse_capability_list(&list),
        // Default-empty grants only Hello + Shutdown (lifecycle).
        // The caller MUST explicitly opt in to elevated paths.
        None => Ok(Vec::new()),
    }
}

/// Intersect `argv_requested` (the upper bound) with
/// `pipe_granted` (the lower bound) so a `GrantCapabilities`
/// request can never widen beyond what the spawn argv asked for.
fn effective_capabilities(
    argv_requested: &[Capability],
    pipe_granted: &[Capability],
) -> Vec<Capability> {
    let mut out: Vec<Capability> = Vec::with_capacity(pipe_granted.len());
    for cap in pipe_granted {
        if argv_requested.contains(cap) {
            out.push(*cap);
        }
    }
    out
}

fn run_loop<R: std::io::BufRead, W: std::io::Write>(
    reader: &mut R,
    writer: &mut W,
    argv_requested: &[Capability],
) -> Result<(), TransportError> {
    // Phase 17j — runtime-granted starts empty. The caller must
    // send `Request::GrantCapabilities` over the (DACL-restricted)
    // pipe before any capability-bearing request is accepted.
    // argv `--capabilities=` is the upper bound; this is the
    // lower bound. Effective = argv ∩ pipe.
    let mut pipe_granted: Vec<Capability> = Vec::new();
    loop {
        let request: Request = match read_line(reader) {
            Ok(r) => r,
            Err(TransportError::Eof) => {
                // Caller closed the pipe — exit cleanly.
                return Ok(());
            }
            Err(TransportError::Serde(e)) => {
                // Malformed JSON. Surface a typed Failed response so
                // the caller knows the helper saw the line; do NOT
                // propagate the parse error on its own — that would
                // tear down the connection on the first hiccup.
                let resp = Response::Failed {
                    localized_key: "err-helper-invalid-json".into(),
                    message: e.to_string(),
                };
                write_line(writer, &resp)?;
                continue;
            }
            Err(other) => return Err(other),
        };

        let is_shutdown = matches!(request, Request::Shutdown);
        // GrantCapabilities is special-cased here in the binary
        // because it mutates the per-session granted state; every
        // other request gates through the stateless
        // `handle_request`.
        let response = match &request {
            Request::GrantCapabilities { capabilities } => {
                pipe_granted = capabilities.clone();
                let granted = effective_capabilities(argv_requested, &pipe_granted);
                Response::CapabilitiesGranted { granted }
            }
            _ => {
                let effective = effective_capabilities(argv_requested, &pipe_granted);
                handle_request(&request, &effective)
            }
        };
        write_line(writer, &response)?;
        if is_shutdown {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    // Integration-style test that pumps a synthetic request stream
    // through the run-loop without spawning the binary. Exercises
    // the malformed-line recovery path.
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn run_loop_handles_malformed_then_valid_line() {
        let request_line = serde_json::to_string(&Request::Shutdown).unwrap();
        let stream = format!("not json\n{request_line}\n");
        let mut reader = BufReader::new(Cursor::new(stream.into_bytes()));
        let mut wire: Vec<u8> = Vec::new();
        run_loop(&mut reader, &mut wire, &[]).unwrap();
        // Response stream should carry Failed (for the bad line) +
        // ShuttingDown (for the valid Shutdown).
        let body = String::from_utf8(wire).unwrap();
        let mut lines = body.lines();
        let r1: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let r2: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert!(matches!(r1, Response::Failed { .. }));
        assert!(matches!(r2, Response::ShuttingDown));
    }

    /// Phase 17j — capability-bearing requests sent before
    /// `GrantCapabilities` must surface as `CapabilityDenied`,
    /// regardless of what `--capabilities=` argv said. The argv
    /// is the upper bound; the lower bound starts at zero.
    #[test]
    fn run_loop_denies_capability_request_before_grant() {
        let req = Request::ElevatedRetry {
            src: std::path::PathBuf::from("/tmp/src"),
            dst: std::path::PathBuf::from("/tmp/dst"),
        };
        let req_line = serde_json::to_string(&req).unwrap();
        let shut_line = serde_json::to_string(&Request::Shutdown).unwrap();
        let stream = format!("{req_line}\n{shut_line}\n");
        let mut reader = BufReader::new(Cursor::new(stream.into_bytes()));
        let mut wire: Vec<u8> = Vec::new();
        // argv-requested = ElevatedRetry, but no GrantCapabilities
        // landed yet, so effective = ∅.
        run_loop(&mut reader, &mut wire, &[Capability::ElevatedRetry]).unwrap();
        let body = String::from_utf8(wire).unwrap();
        let mut lines = body.lines();
        let r1: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let r2: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert!(
            matches!(r1, Response::CapabilityDenied { .. }),
            "expected CapabilityDenied before GrantCapabilities, got {r1:?}"
        );
        assert!(matches!(r2, Response::ShuttingDown));
    }

    /// Phase 17j — `GrantCapabilities` populates the per-session
    /// state; subsequent requests use the intersection of argv-
    /// requested and pipe-granted.
    #[test]
    fn run_loop_grants_then_serves_capability() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");
        std::fs::write(&src, b"payload").unwrap();
        let grant = Request::GrantCapabilities {
            capabilities: vec![Capability::ElevatedRetry],
        };
        let retry = Request::ElevatedRetry {
            src: src.clone(),
            dst: dst.clone(),
        };
        let stream = format!(
            "{}\n{}\n{}\n",
            serde_json::to_string(&grant).unwrap(),
            serde_json::to_string(&retry).unwrap(),
            serde_json::to_string(&Request::Shutdown).unwrap(),
        );
        let mut reader = BufReader::new(Cursor::new(stream.into_bytes()));
        let mut wire: Vec<u8> = Vec::new();
        run_loop(&mut reader, &mut wire, &[Capability::ElevatedRetry]).unwrap();
        let body = String::from_utf8(wire).unwrap();
        let mut lines = body.lines();
        let r1: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let r2: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let r3: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert!(matches!(r1, Response::CapabilitiesGranted { .. }));
        assert!(
            matches!(r2, Response::ElevatedRetryOk { bytes: 7 }),
            "expected ElevatedRetryOk after grant, got {r2:?}"
        );
        assert!(matches!(r3, Response::ShuttingDown));
    }

    /// Phase 17j — pipe-granted set wider than argv-requested is
    /// silently clamped to argv (you can't widen the grant beyond
    /// what the spawn argv asked for). A request for the
    /// non-argv-requested capability after such a clamp is denied.
    #[test]
    fn run_loop_clamps_grant_to_argv_upper_bound() {
        let grant = Request::GrantCapabilities {
            capabilities: vec![Capability::ElevatedRetry, Capability::HardwareErase],
        };
        let erase = Request::HardwareErase {
            device: std::path::PathBuf::from("/dev/nvme0n1"),
        };
        let stream = format!(
            "{}\n{}\n{}\n",
            serde_json::to_string(&grant).unwrap(),
            serde_json::to_string(&erase).unwrap(),
            serde_json::to_string(&Request::Shutdown).unwrap(),
        );
        let mut reader = BufReader::new(Cursor::new(stream.into_bytes()));
        let mut wire: Vec<u8> = Vec::new();
        // argv-requested = ElevatedRetry only. The grant asked for
        // HardwareErase too but the intersection drops it.
        run_loop(&mut reader, &mut wire, &[Capability::ElevatedRetry]).unwrap();
        let body = String::from_utf8(wire).unwrap();
        let mut lines = body.lines();
        let r1: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let r2: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        let _r3: Response = serde_json::from_str(lines.next().unwrap()).unwrap();
        match r1 {
            Response::CapabilitiesGranted { granted } => {
                assert_eq!(granted, vec![Capability::ElevatedRetry]);
            }
            other => panic!("expected CapabilitiesGranted, got {other:?}"),
        }
        assert!(
            matches!(r2, Response::CapabilityDenied { .. }),
            "HardwareErase must be denied even after over-broad grant: {r2:?}"
        );
    }

    /// Helper unit-test: `effective_capabilities` is intersection.
    #[test]
    fn effective_capabilities_is_intersection() {
        let argv = [Capability::ElevatedRetry, Capability::ShellExtension];
        let pipe = vec![Capability::ElevatedRetry, Capability::HardwareErase];
        let eff = effective_capabilities(&argv, &pipe);
        assert_eq!(eff, vec![Capability::ElevatedRetry]);
    }
}
