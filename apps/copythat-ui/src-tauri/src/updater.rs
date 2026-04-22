//! Phase 15 — auto-update manifest + throttle surface.
//!
//! Tauri 2.x ships its own updater plugin (`tauri-plugin-updater`),
//! which handles the signed-artifact download + install. We layer
//! three responsibilities on top of it:
//!
//! 1. **Endpoint formatting.** The manifest URL template carries
//!    `{{target}}`, `{{arch}}`, `{{current_version}}`, and our own
//!    `{{channel}}` placeholder so a single deploy can serve both
//!    `stable` and `beta` channels.
//! 2. **Manifest parsing / comparison.** A tiny dependency-free JSON
//!    parse that understands the same schema the updater plugin
//!    consumes, plus a semver-ish "strictly greater" comparison so
//!    the UI can show "update available" without booting the plugin
//!    against a real server. The smoke test exercises this surface
//!    against a local HTTP fixture.
//! 3. **24 h throttle.** `UpdaterSettings::due_for_check` decides
//!    whether the automatic launch check fires; this module is where
//!    the callsite lives. The setting persists across restarts, so a
//!    launch within 24 h of the last successful check stays silent.
//!
//! Everything here is synchronous / blocking and free of Tauri
//! dependencies, which keeps the smoke test short and CI-friendly.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// A single platform-keyed artifact record from the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformArtifact {
    /// Minisign signature of the artifact, base64-encoded.
    pub signature: String,
    /// Direct URL to the signed artifact.
    pub url: String,
}

/// The JSON payload Tauri's updater plugin consumes. We re-parse it
/// ourselves (rather than call into the plugin) so the UI can show a
/// "version X.Y.Z is available" banner without kicking off an install.
///
/// Example:
/// ```json
/// {
///   "version": "1.2.3",
///   "notes": "Bug fixes + perf.",
///   "pub_date": "2026-04-22T10:00:00Z",
///   "platforms": {
///     "windows-x86_64": {
///       "signature": "<base64>",
///       "url": "https://…/CopyThat_1.2.3_x64-setup.nsis.zip"
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub pub_date: String,
    #[serde(default)]
    pub platforms: std::collections::BTreeMap<String, PlatformArtifact>,
}

/// Narrow error shape for the pure-Rust manifest helpers. The Tauri
/// commands translate this to a String at the IPC boundary.
#[derive(Debug, thiserror::Error)]
pub enum UpdaterError {
    #[error("manifest url is malformed: {0}")]
    BadUrl(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("server returned status {0}")]
    BadStatus(u16),
    #[error("manifest JSON parse failed: {0}")]
    BadJson(String),
}

/// Substitute the `{{target}}`, `{{arch}}`, `{{current_version}}`,
/// and `{{channel}}` placeholders in an endpoint template. Tauri's
/// own plugin handles the first three at download time; we do the
/// same substitution here so the manifest pre-fetch path uses the
/// exact same URL the plugin will later hit.
pub fn format_endpoint(
    template: &str,
    channel: &str,
    target: &str,
    arch: &str,
    current_version: &str,
) -> String {
    template
        .replace("{{channel}}", channel)
        .replace("{{target}}", target)
        .replace("{{arch}}", arch)
        .replace("{{current_version}}", current_version)
}

/// Identify the current OS/arch pair the way Tauri's updater plugin
/// keys manifest platforms. `windows-x86_64`, `darwin-aarch64`, etc.
pub fn current_target_platform() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    };
    format!("{os}-{arch}")
}

/// Fetch and parse a manifest over plain HTTP or HTTPS. Written with
/// a hand-rolled minimal HTTP client so the smoke test doesn't need
/// `reqwest` in dev-deps, but the real auto-check path is happy to
/// delegate to `tauri-plugin-updater` instead — this helper exists
/// primarily for the pre-fetch "is there an update?" banner logic
/// the UI shows before the user opts into the install.
///
/// Only supports `http://` for the smoke test (local fixture).
/// Production code paths call through the Tauri plugin's own fetch,
/// which uses `reqwest` under the hood with full TLS.
pub fn fetch_manifest_http(url: &str, timeout: Duration) -> Result<UpdateManifest, UpdaterError> {
    let (host, port, path) = parse_http_url(url)?;
    let addr = format!("{host}:{port}");
    let sa = addr
        .to_socket_addrs()
        .map_err(|e| UpdaterError::Network(format!("resolve {addr}: {e}")))?
        .next()
        .ok_or_else(|| UpdaterError::Network(format!("no addresses for {addr}")))?;
    let mut sock = TcpStream::connect_timeout(&sa, timeout)
        .map_err(|e| UpdaterError::Network(format!("connect {addr}: {e}")))?;
    sock.set_read_timeout(Some(timeout))
        .map_err(|e| UpdaterError::Network(format!("set_read_timeout: {e}")))?;
    sock.set_write_timeout(Some(timeout))
        .map_err(|e| UpdaterError::Network(format!("set_write_timeout: {e}")))?;

    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: copythat/{}\r\nConnection: close\r\nAccept: application/json\r\n\r\n",
        env!("CARGO_PKG_VERSION"),
    );
    sock.write_all(req.as_bytes())
        .map_err(|e| UpdaterError::Network(format!("write: {e}")))?;

    let mut reader = BufReader::new(sock);
    let status_line = read_line(&mut reader)?;
    let status = parse_status_code(&status_line)?;
    if !(200..300).contains(&status) {
        return Err(UpdaterError::BadStatus(status));
    }

    // Drain headers (through the blank separator).
    loop {
        let line = read_line(&mut reader)?;
        if line == "\r\n" || line.is_empty() {
            break;
        }
    }

    let mut body = String::new();
    reader
        .read_to_string(&mut body)
        .map_err(|e| UpdaterError::Network(format!("read body: {e}")))?;

    // Some HTTP responses use chunked transfer encoding; a dead-simple
    // "first { to last }" window survives both chunked and identity.
    let start = body
        .find('{')
        .ok_or_else(|| UpdaterError::BadJson("no opening brace".into()))?;
    let end = body
        .rfind('}')
        .ok_or_else(|| UpdaterError::BadJson("no closing brace".into()))?;
    let slice = &body[start..=end];

    serde_json::from_str::<UpdateManifest>(slice).map_err(|e| UpdaterError::BadJson(e.to_string()))
}

fn read_line<R: BufRead>(reader: &mut R) -> Result<String, UpdaterError> {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|e| UpdaterError::Network(format!("read_line: {e}")))?;
    Ok(line)
}

fn parse_status_code(line: &str) -> Result<u16, UpdaterError> {
    // "HTTP/1.1 200 OK\r\n"
    let mut parts = line.split_whitespace();
    let _version = parts.next();
    let code = parts
        .next()
        .ok_or_else(|| UpdaterError::Network(format!("malformed status line: {line:?}")))?;
    code.parse::<u16>()
        .map_err(|e| UpdaterError::Network(format!("bad status code {code:?}: {e}")))
}

fn parse_http_url(url: &str) -> Result<(String, u16, String), UpdaterError> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| UpdaterError::BadUrl(format!("unsupported scheme: {url}")))?;
    let (authority, path) = match rest.find('/') {
        Some(idx) => (&rest[..idx], &rest[idx..]),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rfind(':') {
        Some(idx) => {
            let h = &authority[..idx];
            let p = authority[idx + 1..]
                .parse::<u16>()
                .map_err(|e| UpdaterError::BadUrl(format!("bad port in {authority:?}: {e}")))?;
            (h.to_string(), p)
        }
        None => (authority.to_string(), 80),
    };
    Ok((host, port, path.to_string()))
}

/// Strictly greater semver-ish comparison. Only the three leading
/// dotted numeric groups are compared; any `-pre` / `+build` tail is
/// stripped. Non-numeric groups are treated as 0 — a malformed
/// manifest version silently loses against anything, which is the
/// safer failure mode for "should we prompt an update?" decisions.
pub fn is_strictly_newer(candidate: &str, current: &str) -> bool {
    let a = split_numeric3(candidate);
    let b = split_numeric3(current);
    a > b
}

fn split_numeric3(v: &str) -> (u32, u32, u32) {
    let head = v.split(['-', '+']).next().unwrap_or(v);
    let mut it = head.split('.');
    let a = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let b = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let c = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    (a, b, c)
}

/// Summary a Tauri command returns to the UI: what manifest said plus
/// whether it's actually newer than the running binary.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckDto {
    /// Version the manifest advertises; empty string when no manifest
    /// was reachable.
    pub available_version: String,
    /// Release notes from the manifest (free-form markdown).
    pub notes: String,
    /// Pub date from the manifest.
    pub pub_date: String,
    /// True iff `available_version` is strictly newer than the
    /// running binary's version.
    pub is_newer: bool,
    /// Unix epoch seconds at which the check fired.
    pub checked_at_unix_secs: i64,
    /// Indicates the check was gated by the 24 h throttle and no
    /// network request was made. When `true`, the other fields echo
    /// the last stored snapshot — useful for the UI to show "last
    /// checked: …" without lying about having just hit the network.
    pub skipped_by_throttle: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_endpoint_substitutes_all_placeholders() {
        let url = format_endpoint(
            "https://rel.example.com/{{channel}}/{{target}}/{{arch}}/{{current_version}}",
            "beta",
            "windows",
            "x86_64",
            "0.1.0",
        );
        assert_eq!(url, "https://rel.example.com/beta/windows/x86_64/0.1.0");
    }

    #[test]
    fn format_endpoint_handles_missing_placeholders() {
        let url = format_endpoint(
            "https://rel.example.com/latest.json",
            "stable",
            "linux",
            "x86_64",
            "1.0.0",
        );
        assert_eq!(url, "https://rel.example.com/latest.json");
    }

    #[test]
    fn is_strictly_newer_basic() {
        assert!(is_strictly_newer("1.0.1", "1.0.0"));
        assert!(is_strictly_newer("2.0.0", "1.99.99"));
        assert!(is_strictly_newer("0.2.0", "0.1.99"));
        assert!(!is_strictly_newer("1.0.0", "1.0.0"));
        assert!(!is_strictly_newer("0.9.9", "1.0.0"));
    }

    #[test]
    fn is_strictly_newer_ignores_prerelease_tail() {
        // Tail is stripped, so "1.0.0-beta" == "1.0.0" — current is
        // not strictly newer than a pre-release of the same number,
        // matching how we want the auto-update banner to behave.
        assert!(!is_strictly_newer("1.0.0-beta", "1.0.0"));
        assert!(!is_strictly_newer("1.0.0", "1.0.0-beta"));
        assert!(is_strictly_newer("1.0.1-beta", "1.0.0"));
    }

    #[test]
    fn is_strictly_newer_malformed_loses() {
        // Garbage parses to (0,0,0); anything real beats it.
        assert!(is_strictly_newer("0.0.1", "garbage"));
        assert!(!is_strictly_newer("garbage", "0.0.1"));
    }

    #[test]
    fn parse_http_url_handles_port_and_path() {
        let (h, p, path) = parse_http_url("http://127.0.0.1:5432/manifest.json").unwrap();
        assert_eq!(h, "127.0.0.1");
        assert_eq!(p, 5432);
        assert_eq!(path, "/manifest.json");

        let (h, p, path) = parse_http_url("http://releases.example.com/latest.json").unwrap();
        assert_eq!(h, "releases.example.com");
        assert_eq!(p, 80);
        assert_eq!(path, "/latest.json");

        let (h, p, path) = parse_http_url("http://127.0.0.1:8080").unwrap();
        assert_eq!(h, "127.0.0.1");
        assert_eq!(p, 8080);
        assert_eq!(path, "/");
    }

    #[test]
    fn parse_http_url_rejects_https() {
        let err = parse_http_url("https://example.com/foo.json").unwrap_err();
        assert!(matches!(err, UpdaterError::BadUrl(_)));
    }

    #[test]
    fn parse_manifest_from_valid_json() {
        let body = r#"{
            "version": "9.9.9",
            "notes": "World-changing stuff.",
            "pub_date": "2026-04-22T00:00:00Z",
            "platforms": {
                "windows-x86_64": {
                    "signature": "sig-here",
                    "url": "https://example.com/x64.zip"
                }
            }
        }"#;
        let m: UpdateManifest = serde_json::from_str(body).unwrap();
        assert_eq!(m.version, "9.9.9");
        assert_eq!(m.notes, "World-changing stuff.");
        assert!(m.platforms.contains_key("windows-x86_64"));
        let art = m.platforms.get("windows-x86_64").unwrap();
        assert_eq!(art.signature, "sig-here");
    }
}
