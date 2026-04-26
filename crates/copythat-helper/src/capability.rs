//! Capability allowlist for the helper.
//!
//! Phase 17d intentionally errs on the side of fewer capabilities
//! than the Windows / Unix elevation tokens technically grant. The
//! main UI process declares what it needs at spawn time (via the
//! command-line arg `--capabilities=elevated_retry,install_shell`)
//! and the helper refuses every request not on the list. The
//! security property: even if an attacker tricks the user into a
//! UAC consent for one capability (say, "install shell extension"),
//! the helper won't accept a different request kind on the same
//! session.
//!
//! This is coarser than per-call dynamic policy — the assumption
//! is that the consent flow is the moat, not the IPC layer. The
//! IPC layer just keeps an attacker from pivoting to *other*
//! elevated paths after the consent has fired.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::rpc::Request;

/// Coarse capability tag the caller declares at spawn time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// `Request::Hello` and `Request::Shutdown` always implicitly
    /// allowed; every other capability is opt-in. Listed here for
    /// documentation only — `Capability::is_allowed_for` short-
    /// circuits Hello + Shutdown without consulting the set.
    Lifecycle,
    /// `Request::ElevatedRetry`.
    ElevatedRetry,
    /// `Request::InstallShellExtension` + `UninstallShellExtension`.
    ShellExtension,
    /// `Request::HardwareErase`.
    HardwareErase,
}

impl Capability {
    /// Wire string used in the `--capabilities=...` arg.
    pub fn wire_label(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::ElevatedRetry => "elevated_retry",
            Self::ShellExtension => "shell_extension",
            Self::HardwareErase => "hardware_erase",
        }
    }

    /// Inverse — used by the helper's argv parser. Unknown labels
    /// fail at spawn time so the helper never starts in a half-
    /// configured state.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "lifecycle" => Some(Self::Lifecycle),
            "elevated_retry" => Some(Self::ElevatedRetry),
            "shell_extension" => Some(Self::ShellExtension),
            "hardware_erase" => Some(Self::HardwareErase),
            _ => None,
        }
    }

    /// Required capability for a given request. `Hello` and
    /// `Shutdown` always return `None` because they're lifecycle
    /// management — gating them would deadlock the protocol.
    pub fn required_for(req: &Request) -> Option<Self> {
        match req {
            // Hello / Shutdown / GrantCapabilities are lifecycle —
            // gating them would deadlock the protocol (the grant
            // itself can't require a capability it's about to
            // grant). The binary's run-loop handles
            // GrantCapabilities specially anyway, so this branch
            // is defence-in-depth: even if a future caller routes
            // GrantCapabilities through `handle_request`, the
            // capability gate doesn't refuse it.
            Request::Hello { .. } | Request::Shutdown | Request::GrantCapabilities { .. } => None,
            Request::ElevatedRetry { .. } => Some(Self::ElevatedRetry),
            Request::InstallShellExtension { .. } | Request::UninstallShellExtension { .. } => {
                Some(Self::ShellExtension)
            }
            Request::HardwareErase { .. } => Some(Self::HardwareErase),
        }
    }
}

/// Error variant raised when a request fails the capability check.
/// Callers map this onto `Response::CapabilityDenied`.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum CapabilityError {
    #[error("capability {} not granted to this helper session", capability.wire_label())]
    NotGranted { capability: Capability },
}

impl CapabilityError {
    pub fn reason(&self) -> String {
        self.to_string()
    }
}

/// Parse the helper's `--capabilities=cap1,cap2,...` argv arm into
/// a typed set. An unknown label fails the whole parse — the
/// helper exits before reading any IPC.
pub fn parse_capability_list(raw: &str) -> Result<Vec<Capability>, String> {
    let mut out = Vec::new();
    for token in raw.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let cap = Capability::parse(token)
            .ok_or_else(|| format!("unknown capability `{token}` in --capabilities argv"))?;
        if !out.contains(&cap) {
            out.push(cap);
        }
    }
    Ok(out)
}

/// Per-request gate. Returns `Ok(())` if the request is allowed,
/// `Err(CapabilityError)` otherwise. Hello / Shutdown bypass.
pub fn check(request: &Request, granted: &[Capability]) -> Result<(), CapabilityError> {
    match Capability::required_for(request) {
        None => Ok(()),
        Some(needed) => {
            if granted.contains(&needed) {
                Ok(())
            } else {
                Err(CapabilityError::NotGranted { capability: needed })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::ShellExtensionKind;
    use std::path::PathBuf;

    #[test]
    fn lifecycle_requests_bypass_capabilities() {
        let granted: Vec<Capability> = vec![];
        check(&Request::Hello { version: 1 }, &granted).unwrap();
        check(&Request::Shutdown, &granted).unwrap();
    }

    #[test]
    fn elevated_retry_requires_capability() {
        let req = Request::ElevatedRetry {
            src: PathBuf::from("/a"),
            dst: PathBuf::from("/b"),
        };
        assert!(check(&req, &[]).is_err());
        check(&req, &[Capability::ElevatedRetry]).unwrap();
    }

    #[test]
    fn shell_extension_requires_capability() {
        let req = Request::InstallShellExtension {
            target: ShellExtensionKind::LinuxNautilus,
        };
        assert!(check(&req, &[]).is_err());
        assert!(check(&req, &[Capability::ElevatedRetry]).is_err());
        check(&req, &[Capability::ShellExtension]).unwrap();
    }

    #[test]
    fn parse_list_handles_dedup_and_unknown() {
        let parsed =
            parse_capability_list("elevated_retry,shell_extension,elevated_retry").unwrap();
        assert_eq!(parsed.len(), 2);
        assert!(parse_capability_list("elevated_retry,bogus").is_err());
    }

    #[test]
    fn wire_labels_round_trip() {
        for cap in [
            Capability::Lifecycle,
            Capability::ElevatedRetry,
            Capability::ShellExtension,
            Capability::HardwareErase,
        ] {
            assert_eq!(Capability::parse(cap.wire_label()), Some(cap));
        }
    }
}
