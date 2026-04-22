//! JSON-RPC wire protocol shared by `copythat-snapshot` (main process)
//! and `copythat-helper-vss` (the elevated child).
//!
//! The protocol is newline-delimited JSON over stdin/stdout. One
//! request per line, one response per line. No batching, no
//! pipelining — the helper serves one request at a time. Fields are
//! snake-case. Every request that isn't a shutdown returns an
//! `ok: bool`; on `ok: false`, a `message: string` explains.

use serde::{Deserialize, Serialize};

/// Wire-version header. The helper refuses requests with a mismatched
/// version so the main process can detect a stale helper on disk after
/// an upgrade and re-spawn.
pub const PROTOCOL_VERSION: u32 = 1;

/// Request from main → helper.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum Request {
    /// Probe the helper: returns its version + elevation status.
    Hello { version: u32 },
    /// Create a VSS snapshot covering the volume that `volume_letter`
    /// names. `volume_letter` is a single letter with a trailing
    /// backslash (`"C:\\"`).
    Create { volume: String },
    /// Release the snapshot identified by `shadow_id`.
    Release { shadow_id: String },
    /// Orderly shutdown — helper flushes any pending snapshots and
    /// exits with code 0.
    Shutdown,
}

/// Response from helper → main.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub shadow_id: Option<String>,
    /// The `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN` device path.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub device_path: Option<String>,
    /// Original volume letter the shadow covers (e.g. `"C:\\"`).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub volume: Option<String>,
}

impl Response {
    pub fn ok() -> Self {
        Self {
            ok: true,
            message: None,
            version: None,
            shadow_id: None,
            device_path: None,
            volume: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            message: Some(message.into()),
            version: None,
            shadow_id: None,
            device_path: None,
            volume: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trip_create() {
        let req = Request::Create {
            volume: "C:\\".to_string(),
        };
        let line = serde_json::to_string(&req).unwrap();
        assert!(line.contains("\"method\":\"create\""));
        assert!(line.contains("\"volume\":\"C:"));
        let back: Request = serde_json::from_str(&line).unwrap();
        match back {
            Request::Create { volume } => assert_eq!(volume, "C:\\"),
            _ => panic!("round-trip produced wrong variant"),
        }
    }

    #[test]
    fn response_ok_serializes_without_null_fields() {
        let s = serde_json::to_string(&Response::ok()).unwrap();
        // We strip None fields so a success response is compact.
        assert_eq!(s, r#"{"ok":true}"#);
    }

    #[test]
    fn response_err_carries_message() {
        let s = serde_json::to_string(&Response::err("boom")).unwrap();
        assert!(s.contains(r#""ok":false"#));
        assert!(s.contains(r#""message":"boom""#));
    }

    #[test]
    fn hello_request_has_version_field() {
        let req = Request::Hello {
            version: PROTOCOL_VERSION,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("\"method\":\"hello\""));
        assert!(s.contains("\"version\":1"));
    }

    #[test]
    fn shutdown_request_is_tag_only() {
        let req = Request::Shutdown;
        let s = serde_json::to_string(&req).unwrap();
        assert_eq!(s, r#"{"method":"shutdown"}"#);
    }
}
