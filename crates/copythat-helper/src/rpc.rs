//! Wire shape between `copythat-ui` and the elevated helper.
//!
//! Newline-delimited JSON over a single pipe / socket. Each line
//! is one of:
//!
//! - `Request` — what the caller wants done.
//! - `Response` — what the helper did (or refused to do).
//!
//! The shape is narrowly typed so a malformed payload surfaces as
//! a parse error before any privileged action runs.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Helper protocol version. Bumped only when the wire shape
/// changes incompatibly. The handshake `Request::Hello` carries
/// the caller's expected version; a mismatch surfaces as
/// `Response::ProtocolMismatch`.
pub const PROTOCOL_VERSION: u32 = 1;

/// Every incoming request is one of these. The `tag = "kind"` +
/// `rename_all = "snake_case"` shape means the wire form is
/// idiomatic JSON: `{"kind": "elevated_retry", "src": "...", ...}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    /// First request the caller sends after the pipe connects.
    /// Echoes the version + a session-id back; lets the caller
    /// confirm the helper is the build it expects before it dispatches
    /// any privileged work.
    Hello { version: u32 },
    /// Phase 8 elevated-retry path. The unprivileged engine hit a
    /// `PermissionDenied` on a per-file copy; the helper attempts
    /// the same copy with an Administrator / root token. Path
    /// arguments must be absolute + traversal-free.
    ElevatedRetry { src: PathBuf, dst: PathBuf },
    /// Phase 17d shell-extension install. Writes to system
    /// directories (HKLM, `/Library/PreferencePanes`, system
    /// `.desktop` files). The field is named `target` (not
    /// `kind`) so it doesn't collide with the serde discriminator.
    InstallShellExtension {
        #[serde(rename = "target")]
        target: ShellExtensionKind,
    },
    /// Symmetric undo of `InstallShellExtension`.
    UninstallShellExtension {
        #[serde(rename = "target")]
        target: ShellExtensionKind,
    },
    /// Phase 4 `Nist80088Purge` follow-up. Phase 44 wires the
    /// actual ioctls; today the helper acknowledges the request
    /// and returns `Response::HardwareEraseUnavailable` with a
    /// clear pointer at Clear + FDE rotation.
    HardwareErase { device: PathBuf },
    /// Drop the connection cleanly. Optional — EOF on the pipe is
    /// the same shape, just less polite.
    Shutdown,
}

/// What the helper sends back. One per request. The wire shape
/// mirrors `Request` so a JSON consumer can correlate without
/// having to thread a request-id (the protocol is strictly
/// ping-pong; the caller blocks until it gets a response).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Response {
    /// Match for `Request::Hello`. Carries the helper's protocol
    /// version + a randomly-generated session id the caller can
    /// use to correlate subsequent responses.
    HelloOk { version: u32, session_id: String },
    /// `Hello` arrived with a version the helper doesn't speak.
    ProtocolMismatch {
        helper_version: u32,
        caller_version: u32,
    },
    /// `ElevatedRetry` succeeded. Helper performed the copy; caller
    /// can update history / events accordingly.
    ElevatedRetryOk { bytes: u64 },
    /// `ElevatedRetry` failed at the helper too — even with
    /// Administrator privileges the OS refused. `localized_key`
    /// maps to the same Fluent table the engine uses.
    ElevatedRetryFailed {
        localized_key: String,
        message: String,
    },
    /// `InstallShellExtension` succeeded for the named kind.
    ShellExtensionInstalled {
        #[serde(rename = "target")]
        target: ShellExtensionKind,
    },
    /// Symmetric.
    ShellExtensionUninstalled {
        #[serde(rename = "target")]
        target: ShellExtensionKind,
    },
    /// Helper has no platform implementation for this kind on this
    /// host. Caller decides whether to surface a "not supported on
    /// this OS" toast or fall back to a per-user variant.
    ShellExtensionUnsupported {
        #[serde(rename = "target")]
        target: ShellExtensionKind,
        reason: String,
    },
    /// `HardwareErase` is in scope but not yet implemented. The
    /// helper refuses with this variant + a pointer at the
    /// `Clear + FDE rotation` workflow that does work today.
    HardwareEraseUnavailable { reason: String },
    /// Phase 17a path-safety bar fired. Caller sent a request with
    /// a `..`-laden / NUL-laden / empty path. Same shape as
    /// `CopyErrorKind::PathEscape`; the `localized_key` is
    /// `err-path-escape`.
    PathRejected {
        offending: PathBuf,
        localized_key: String,
    },
    /// Capability check refused this request. Either the request
    /// kind isn't allowed at all (helper builds with a narrower
    /// allowlist) or the per-request bounds (e.g. shell-extension
    /// scope) didn't satisfy.
    CapabilityDenied { reason: String },
    /// Generic fallback for IO / serialization / OS-level failures
    /// the helper can't classify into a more specific variant. The
    /// caller surfaces this as a typed error in the UI; the
    /// `message` field is meant for diagnostic display, not
    /// programmatic branching.
    Failed {
        localized_key: String,
        message: String,
    },
    /// Match for `Request::Shutdown`. Helper exits cleanly after
    /// flushing the response.
    ShuttingDown,
}

/// Which shell extension flavour the caller wants installed /
/// uninstalled. Mirrors the platform-side surfaces the
/// `copythat-shellext` crate already ships — the helper just
/// performs the elevated half.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellExtensionKind {
    /// Windows COM `IExplorerCommand` registration under HKLM.
    WindowsExplorerCommand,
    /// macOS Finder Sync `.appex` registration under
    /// `/Library/PreferencePanes/`.
    MacosFinderSync,
    /// Linux Nautilus Python extension at
    /// `/usr/share/nautilus-python/extensions/`.
    LinuxNautilus,
    /// Linux KDE Dolphin ServiceMenu under `/usr/share/kio/servicemenus/`.
    LinuxDolphin,
    /// Linux XFCE Thunar UCA file under `/etc/xdg/Thunar/`.
    LinuxThunar,
}

impl ShellExtensionKind {
    pub fn wire_label(self) -> &'static str {
        match self {
            Self::WindowsExplorerCommand => "windows-explorer-command",
            Self::MacosFinderSync => "macos-finder-sync",
            Self::LinuxNautilus => "linux-nautilus",
            Self::LinuxDolphin => "linux-dolphin",
            Self::LinuxThunar => "linux-thunar",
        }
    }

    /// Best-effort guess at whether this kind is meaningful on the
    /// current host OS. Lets the caller pre-filter the menu before
    /// asking the helper for permission.
    pub fn is_native_to_current_host(self) -> bool {
        match self {
            Self::WindowsExplorerCommand => cfg!(target_os = "windows"),
            Self::MacosFinderSync => cfg!(target_os = "macos"),
            Self::LinuxNautilus | Self::LinuxDolphin | Self::LinuxThunar => {
                cfg!(target_os = "linux")
            }
        }
    }
}

/// Failure modes for parsing / dispatching a helper request.
/// Distinct from `Response::Failed` — this enum captures the
/// errors that surface BEFORE the helper has a request to act on
/// (parse failure, unknown variant, etc.).
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum HelperError {
    #[error("invalid JSON: {0}")]
    InvalidJson(String),
    #[error("protocol mismatch: helper={helper}, caller={caller}")]
    ProtocolMismatch { helper: u32, caller: u32 },
    #[error("path rejected: {0}")]
    PathRejected(String),
}

impl HelperError {
    pub fn localized_key(&self) -> &'static str {
        match self {
            Self::InvalidJson(_) => "err-helper-invalid-json",
            Self::ProtocolMismatch { .. } => "err-helper-protocol-mismatch",
            Self::PathRejected(_) => "err-path-escape",
        }
    }
}

/// Generate a per-launch random pipe / socket name. 256 bits from
/// `getrandom`; the caller embeds this in the spawn argv so a
/// concurrent attacker can't predict the pipe / socket. The shape
/// is `<prefix><hex>` where `prefix` distinguishes the platform's
/// pipe / socket convention.
pub fn generate_pipe_name(prefix: &str) -> Result<String, std::io::Error> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|e| std::io::Error::other(format!("getrandom failed: {e}")))?;
    let suffix = hex::encode(bytes);
    Ok(format!("{prefix}{suffix}"))
}

/// Inverse of `generate_pipe_name` — strips the prefix and returns
/// the random suffix. Used by the helper's argv parser to
/// validate the caller-supplied name has the expected shape
/// before opening the pipe (defence in depth — an attacker can't
/// trick the helper into opening, say, a system pipe).
pub fn parse_pipe_name<'a>(prefix: &str, full: &'a str) -> Option<&'a str> {
    let suffix = full.strip_prefix(prefix)?;
    // 32 random bytes hex-encoded == 64 chars.
    if suffix.len() != 64 || !suffix.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_round_trip_through_serde() {
        let r = Request::ElevatedRetry {
            src: PathBuf::from("/tmp/src"),
            dst: PathBuf::from("/tmp/dst"),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: Request = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }

    #[test]
    fn responses_round_trip_through_serde() {
        let r = Response::ElevatedRetryOk { bytes: 42 };
        let json = serde_json::to_string(&r).unwrap();
        let back: Response = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }

    #[test]
    fn shell_extension_kind_wire_labels_unique() {
        let labels: Vec<&str> = [
            ShellExtensionKind::WindowsExplorerCommand,
            ShellExtensionKind::MacosFinderSync,
            ShellExtensionKind::LinuxNautilus,
            ShellExtensionKind::LinuxDolphin,
            ShellExtensionKind::LinuxThunar,
        ]
        .iter()
        .map(|k| k.wire_label())
        .collect();
        let mut seen = std::collections::HashSet::new();
        for l in &labels {
            assert!(seen.insert(*l), "duplicate wire label: {l}");
        }
    }

    #[test]
    fn pipe_name_round_trip() {
        let n = generate_pipe_name(r"\\.\pipe\copythat-helper-").unwrap();
        let suffix = parse_pipe_name(r"\\.\pipe\copythat-helper-", &n).unwrap();
        assert_eq!(suffix.len(), 64);
        assert!(suffix.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn pipe_name_rejects_wrong_shape() {
        // Wrong prefix.
        assert!(parse_pipe_name(r"\\.\pipe\copythat-helper-", "totally-different").is_none());
        // Right prefix, wrong-length suffix.
        let too_short = format!(r"\\.\pipe\copythat-helper-{}", "ab");
        assert!(parse_pipe_name(r"\\.\pipe\copythat-helper-", &too_short).is_none());
        // Right prefix + right length but non-hex characters.
        let non_hex = format!(r"\\.\pipe\copythat-helper-{}", "z".repeat(64));
        assert!(parse_pipe_name(r"\\.\pipe\copythat-helper-", &non_hex).is_none());
    }
}
