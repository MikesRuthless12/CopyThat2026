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
    let granted = match resolve_capabilities(&args) {
        Ok(caps) => caps,
        Err(e) => {
            eprintln!("copythat-helper: {e}");
            std::process::exit(2);
        }
    };

    let mut reader = buf_reader(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let exit_code = match run_loop(&mut reader, &mut writer, &granted) {
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

fn run_loop<R: std::io::BufRead, W: std::io::Write>(
    reader: &mut R,
    writer: &mut W,
    granted: &[Capability],
) -> Result<(), TransportError> {
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
        let response = handle_request(&request, granted);
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
}
