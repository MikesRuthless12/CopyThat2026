//! Phase 46.4 — `plugin.toml` manifest parser.
//!
//! Each plugin ships an adjacent `plugin.toml` declaring the hooks
//! it implements and the capabilities it needs. The host reads it
//! when [`crate::PluginHost::load_plugin`] runs, asks the user (via
//! the Settings UI in 46.6) to approve each capability, and stores
//! the resulting [`crate::CapabilityGrant`] beside the
//! [`crate::PluginHandle`] for the per-call gate to consult.
//!
//! Manifest grammar:
//!
//! ```toml
//! name = "exif-rename"
//! version = "0.1.0"
//! hooks = ["after_file"]
//! capabilities = ["read_fs:source", "write_fs:dest"]
//! ```
//!
//! Validation enforced here (errors flow as
//! [`crate::PluginError::Manifest`]):
//! - `name` non-empty.
//! - `version` semver-shaped (`MAJOR.MINOR.PATCH` plus optional
//!   `-prerelease`/`+build`). We do not pull in the `semver` crate
//!   because we never compare versions inside the runtime — strict
//!   shape is enough to surface fat-finger typos.
//! - `hooks` non-empty subset of [`crate::HookKind`].
//! - `capabilities` matches [`crate::Capability::parse`] grammar.

use serde::Deserialize;

use crate::{Capability, HookKind, PluginError};

/// Parsed and validated `plugin.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    /// Human-readable plugin name. Surfaced in the Settings UI and
    /// in [`PluginError::CapabilityDenied`] diagnostics.
    pub name: String,

    /// Semver-shaped version string. Stored as-is; not compared at
    /// runtime in 46.4.
    pub version: String,

    /// Hooks the plugin implements. Non-empty, no duplicates after
    /// validation.
    pub hooks: Vec<HookKind>,

    /// Capabilities the plugin needs. May be empty; duplicates are
    /// collapsed during validation.
    pub capabilities: Vec<Capability>,
}

impl PluginManifest {
    /// Parse and validate a `plugin.toml` payload.
    pub fn parse(toml_src: &str) -> Result<Self, PluginError> {
        let raw: RawManifest = toml::from_str(toml_src)
            .map_err(|e| PluginError::Manifest(format!("invalid TOML: {e}")))?;
        raw.validate()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    name: String,
    version: String,
    hooks: Vec<HookKind>,
    #[serde(default)]
    capabilities: Vec<String>,
}

/// Hard cap on the number of capabilities one manifest may declare.
/// Prevents a hostile plugin.toml with millions of entries from
/// allocating an unbounded `Vec<Capability>` (and from rendering an
/// infinite list in the Settings UI).
const MAX_MANIFEST_CAPABILITIES: usize = 64;

/// Hard cap on the length of a single capability scope string.
/// `read_fs:<scope>` / `write_fs:<scope>` declarations are rejected
/// past this bound.
const MAX_CAPABILITY_SCOPE_LEN: usize = 256;

impl RawManifest {
    fn validate(self) -> Result<PluginManifest, PluginError> {
        if self.name.trim().is_empty() {
            return Err(PluginError::Manifest("`name` must not be empty".into()));
        }
        if !is_semver_shaped(&self.version) {
            return Err(PluginError::Manifest(format!(
                "`version` must be semver-shaped (MAJOR.MINOR.PATCH[-prerelease][+build]), got `{}`",
                self.version
            )));
        }
        if self.hooks.is_empty() {
            return Err(PluginError::Manifest(
                "`hooks` must declare at least one hook".into(),
            ));
        }
        let mut hooks = self.hooks;
        // Deterministic order + duplicate collapse without imposing
        // serde's input order on round-trips. `HookKind` derives
        // `Ord` via its discriminant ordering.
        hooks.sort_by_key(|h| *h as u8);
        hooks.dedup();

        if self.capabilities.len() > MAX_MANIFEST_CAPABILITIES {
            return Err(PluginError::Manifest(format!(
                "`capabilities`: too many entries ({}; max {})",
                self.capabilities.len(),
                MAX_MANIFEST_CAPABILITIES
            )));
        }
        let mut capabilities = Vec::with_capacity(self.capabilities.len());
        for raw_cap in &self.capabilities {
            if raw_cap.len() > MAX_CAPABILITY_SCOPE_LEN {
                return Err(PluginError::Manifest(format!(
                    "`capabilities`: entry too long ({} chars; max {})",
                    raw_cap.len(),
                    MAX_CAPABILITY_SCOPE_LEN
                )));
            }
            let cap = Capability::parse(raw_cap)
                .map_err(|e| PluginError::Manifest(format!("`capabilities`: {e}")))?;
            if !capabilities.contains(&cap) {
                capabilities.push(cap);
            }
        }

        Ok(PluginManifest {
            name: self.name,
            version: self.version,
            hooks,
            capabilities,
        })
    }
}

/// `MAJOR.MINOR.PATCH` with optional `-prerelease` and/or `+build`
/// suffix. The three numeric components must be non-empty digit
/// runs; nothing else about the suffix is enforced beyond
/// non-emptiness when present (the `semver` crate's full grammar is
/// overkill for fat-finger detection).
fn is_semver_shaped(s: &str) -> bool {
    // Split off `+build` first so it can't poison the prerelease check.
    let (core_and_pre, build) = match s.split_once('+') {
        Some((head, tail)) => (head, Some(tail)),
        None => (s, None),
    };
    if matches!(build, Some(b) if b.is_empty()) {
        return false;
    }
    let (core, pre) = match core_and_pre.split_once('-') {
        Some((head, tail)) => (head, Some(tail)),
        None => (core_and_pre, None),
    };
    if matches!(pre, Some(p) if p.is_empty()) {
        return false;
    }
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
name = "exif-rename"
version = "0.1.0"
hooks = ["after_file"]
capabilities = ["read_fs:source", "write_fs:dest"]
"#;

    #[test]
    fn round_trip_happy_path() {
        let m = PluginManifest::parse(VALID).expect("must parse");
        assert_eq!(m.name, "exif-rename");
        assert_eq!(m.version, "0.1.0");
        assert_eq!(m.hooks, vec![HookKind::AfterFile]);
        assert_eq!(
            m.capabilities,
            vec![
                Capability::ReadFs {
                    scope: "source".into()
                },
                Capability::WriteFs {
                    scope: "dest".into()
                },
            ]
        );
    }

    #[test]
    fn missing_capabilities_field_defaults_to_empty() {
        let src = r#"
name = "noop"
version = "0.1.0"
hooks = ["before_job"]
"#;
        let m = PluginManifest::parse(src).expect("must parse without capabilities");
        assert!(m.capabilities.is_empty());
    }

    #[test]
    fn empty_name_is_rejected() {
        let src = r#"
name = ""
version = "0.1.0"
hooks = ["before_job"]
"#;
        let err = PluginManifest::parse(src).expect_err("empty name must reject");
        assert!(matches!(err, PluginError::Manifest(ref m) if m.contains("`name`")), "{err:?}");
    }

    #[test]
    fn empty_hooks_is_rejected() {
        let src = r#"
name = "x"
version = "0.1.0"
hooks = []
"#;
        let err = PluginManifest::parse(src).expect_err("empty hooks must reject");
        assert!(matches!(err, PluginError::Manifest(ref m) if m.contains("`hooks`")), "{err:?}");
    }

    #[test]
    fn malformed_version_is_rejected() {
        for bad in ["1.0", "v1.0.0", "1.0.x", "1..0", "1.0.0-", "1.0.0+"] {
            let src = format!(
                r#"
name = "x"
version = "{bad}"
hooks = ["before_job"]
"#
            );
            assert!(
                PluginManifest::parse(&src).is_err(),
                "`{bad}` must reject but parsed successfully"
            );
        }
    }

    #[test]
    fn semver_with_prerelease_and_build_accepted() {
        for good in ["1.0.0", "1.2.3", "0.0.0", "1.0.0-rc.1", "1.0.0+build5", "1.0.0-alpha+meta"] {
            assert!(is_semver_shaped(good), "{good} must be accepted");
        }
    }

    #[test]
    fn unknown_hook_string_is_rejected_by_serde() {
        let src = r#"
name = "x"
version = "0.1.0"
hooks = ["never_heard_of_it"]
"#;
        let err = PluginManifest::parse(src).expect_err("unknown hook must reject");
        assert!(matches!(err, PluginError::Manifest(ref m) if m.contains("invalid TOML")), "{err:?}");
    }

    #[test]
    fn unknown_capability_is_rejected() {
        let src = r#"
name = "x"
version = "0.1.0"
hooks = ["before_job"]
capabilities = ["read_socket:tcp"]
"#;
        let err = PluginManifest::parse(src).expect_err("unknown capability must reject");
        assert!(
            matches!(err, PluginError::Manifest(ref m) if m.contains("capabilities")),
            "{err:?}"
        );
    }

    #[test]
    fn duplicate_hooks_are_collapsed() {
        let src = r#"
name = "x"
version = "0.1.0"
hooks = ["before_job", "before_job", "after_job"]
"#;
        let m = PluginManifest::parse(src).expect("must parse");
        assert_eq!(m.hooks, vec![HookKind::BeforeJob, HookKind::AfterJob]);
    }

    #[test]
    fn unknown_top_level_keys_are_rejected() {
        // Pre-46.7 a typo'd `capabilites` (missing `i`) parsed
        // successfully with zero capabilities. With
        // `deny_unknown_fields` the parser surfaces the typo so the
        // user can correct the manifest before the runtime decides
        // an unintentionally-empty grant means "no host I/O needed".
        let src = r#"
name = "x"
version = "0.1.0"
hooks = ["before_job"]
capabilites = ["read_fs:source"]
"#;
        let err = PluginManifest::parse(src).expect_err("typo'd key must reject");
        assert!(matches!(err, PluginError::Manifest(_)), "{err:?}");
    }

    #[test]
    fn capability_count_above_cap_is_rejected() {
        let mut caps = String::from("[");
        for i in 0..MAX_MANIFEST_CAPABILITIES + 1 {
            if i > 0 {
                caps.push(',');
            }
            caps.push_str(&format!("\"read_fs:scope{i}\""));
        }
        caps.push(']');
        let src = format!(
            r#"
name = "x"
version = "0.1.0"
hooks = ["before_job"]
capabilities = {caps}
"#
        );
        let err = PluginManifest::parse(&src).expect_err("oversized cap list must reject");
        assert!(
            matches!(err, PluginError::Manifest(ref m) if m.contains("too many")),
            "{err:?}"
        );
    }

    #[test]
    fn capability_scope_above_cap_is_rejected() {
        let long = "x".repeat(MAX_CAPABILITY_SCOPE_LEN + 1);
        let src = format!(
            r#"
name = "x"
version = "0.1.0"
hooks = ["before_job"]
capabilities = ["read_fs:{long}"]
"#
        );
        let err = PluginManifest::parse(&src).expect_err("oversized scope must reject");
        assert!(
            matches!(err, PluginError::Manifest(ref m) if m.contains("too long")),
            "{err:?}"
        );
    }
}
