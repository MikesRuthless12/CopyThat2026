//! Wire transport — newline-delimited JSON over a single bi-directional
//! pipe / socket. Phase 17d uses this for both directions: caller
//! writes a `Request` line, helper writes a `Response` line, repeat.
//!
//! The transport is intentionally synchronous + std-only on the
//! helper side: we don't want to depend on tokio inside the
//! elevated binary because that pulls a heavyweight runtime that
//! makes the attack surface bigger. The caller side (which lives
//! in `copythat-ui`) wraps tokio's `NamedPipeClient` /
//! `UnixStream` in async, and bridges the two via
//! `spawn_blocking`. Both sides go through `Read`/`Write` traits
//! so unit tests can use `Cursor` / `Vec` round-trips.

use std::io::{BufRead, BufReader, Read, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("EOF on transport")]
    Eof,
}

/// Encode a `Serialize`-able value to a single newline-terminated
/// JSON line and write it to `out`. Flushes after the newline so
/// the receiver doesn't have to wait for a buffer fill.
pub fn write_line<T: Serialize, W: Write>(out: &mut W, value: &T) -> Result<(), TransportError> {
    let mut buf = serde_json::to_vec(value)?;
    buf.push(b'\n');
    out.write_all(&buf)?;
    out.flush()?;
    Ok(())
}

/// Read a single newline-delimited JSON line from `r` and decode
/// it as `T`. Returns `Eof` when the upstream closed the pipe
/// without a trailing newline (the polite shutdown path).
pub fn read_line<T: DeserializeOwned, R: BufRead>(r: &mut R) -> Result<T, TransportError> {
    let mut line = String::new();
    let n = r.read_line(&mut line)?;
    if n == 0 {
        return Err(TransportError::Eof);
    }
    let trimmed = line.trim_end_matches(['\r', '\n']);
    let value = serde_json::from_str(trimmed)?;
    Ok(value)
}

/// Helper to build a `BufReader` so callers don't have to remember
/// which adapter to use.
pub fn buf_reader<R: Read>(r: R) -> BufReader<R> {
    BufReader::new(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::{Request, Response};
    use std::io::Cursor;

    #[test]
    fn write_then_read_round_trips() {
        let mut wire: Vec<u8> = Vec::new();
        write_line(
            &mut wire,
            &Request::Hello {
                version: crate::rpc::PROTOCOL_VERSION,
            },
        )
        .unwrap();
        // Body should end in exactly one newline.
        assert_eq!(wire.last(), Some(&b'\n'));
        // Multiple writes — second message in the stream.
        write_line(&mut wire, &Request::Shutdown).unwrap();

        let mut cursor = BufReader::new(Cursor::new(wire));
        let r1: Request = read_line(&mut cursor).unwrap();
        let r2: Request = read_line(&mut cursor).unwrap();
        assert!(matches!(r1, Request::Hello { .. }));
        assert!(matches!(r2, Request::Shutdown));
    }

    #[test]
    fn read_line_surfaces_eof_on_empty_stream() {
        let empty: Vec<u8> = Vec::new();
        let mut cursor = BufReader::new(Cursor::new(empty));
        let err = read_line::<Request, _>(&mut cursor).unwrap_err();
        assert!(matches!(err, TransportError::Eof));
    }

    #[test]
    fn read_line_handles_crlf_terminator() {
        // A Windows newline pair must not break the JSON parse.
        let bytes = b"{\"kind\":\"shutdown\"}\r\n".to_vec();
        let mut cursor = BufReader::new(Cursor::new(bytes));
        let r: Request = read_line(&mut cursor).unwrap();
        assert!(matches!(r, Request::Shutdown));
    }

    #[test]
    fn responses_round_trip_too() {
        let mut wire: Vec<u8> = Vec::new();
        write_line(
            &mut wire,
            &Response::HelloOk {
                version: 1,
                session_id: "abc".into(),
            },
        )
        .unwrap();
        let mut cursor = BufReader::new(Cursor::new(wire));
        let r: Response = read_line(&mut cursor).unwrap();
        match r {
            Response::HelloOk {
                version,
                session_id,
            } => {
                assert_eq!(version, 1);
                assert_eq!(session_id, "abc");
            }
            other => panic!("expected HelloOk, got {other:?}"),
        }
    }
}
