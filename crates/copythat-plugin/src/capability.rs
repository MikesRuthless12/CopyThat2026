//! Phase 46.4 — capability gating.
//!
//! Plugins declare the host resources they need in their
//! `plugin.toml` manifest. The user (Settings UI in 46.6) grants or
//! denies each capability. The host turns the granted set into a
//! [`CapabilityGrant`] and hands it to every
//! [`crate::PluginHandle::call_hook`] dispatch. If the manifest
//! declares a capability the grant does not contain, dispatch
//! rejects the call with [`crate::PluginError::CapabilityDenied`]
//! before the WASM instance is even built — "Plugins that ask for
//! capabilities the user denies still load but their hooks can't
//! perform the gated operation" (the load succeeds; only `call_hook`
//! refuses).
//!
//! 46.4 only carries the *declarative* layer: the types, the
//! manifest grammar, and the pre-call gate. Wiring `wasmtime-wasi`
//! preview1 imports against the granted capabilities — preopened
//! dirs for `read_fs:<scope>` / `write_fs:<scope>` and a
//! socket-allowlist for `network` — lands in 46.5 alongside the
//! sample plugins that actually import those functions.

use std::collections::BTreeSet;

/// One host-mediated permission a plugin may need.
///
/// The string forms (`read_fs:<scope>`, `write_fs:<scope>`,
/// `network`) are what plugins write in `plugin.toml` and what the
/// Settings UI surfaces; [`Capability::parse`] / [`Capability::as_str`]
/// round-trip between strings and the typed enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capability {
    /// Read access under a logical scope. Conventional scopes for
    /// the post-copy hook ABI are `source` (the job's source root)
    /// and `dest` (the job's destination root); the manifest accepts
    /// any non-empty scope string so plugins can declare
    /// project-specific scopes too.
    ReadFs { scope: String },

    /// Write access under a logical scope. Same scope grammar as
    /// [`Capability::ReadFs`].
    WriteFs { scope: String },

    /// Outbound network access. No scope sub-grammar at 46.4 —
    /// 46.7's wrap pass adds an allowlist (`network:<host>:<port>`)
    /// once the sample plugins exercise it.
    Network,
}

impl Capability {
    /// Parse a manifest-string capability declaration.
    ///
    /// Accepted grammar:
    /// - `read_fs:<scope>` — `<scope>` non-empty, no embedded `:`.
    /// - `write_fs:<scope>` — same shape.
    /// - `network` — bare keyword.
    ///
    /// Anything else is rejected with a descriptive error string the
    /// manifest layer wraps in [`crate::PluginError::Manifest`].
    pub fn parse(raw: &str) -> Result<Self, String> {
        if raw == "network" {
            return Ok(Capability::Network);
        }
        if let Some(scope) = raw.strip_prefix("read_fs:") {
            return parse_scope(scope).map(|s| Capability::ReadFs { scope: s });
        }
        if let Some(scope) = raw.strip_prefix("write_fs:") {
            return parse_scope(scope).map(|s| Capability::WriteFs { scope: s });
        }
        Err(format!(
            "unknown capability `{raw}` (expected `read_fs:<scope>`, `write_fs:<scope>`, or `network`)"
        ))
    }

    /// Render back to the manifest-string form. Round-trips with
    /// [`Capability::parse`].
    pub fn as_str(&self) -> String {
        match self {
            Capability::ReadFs { scope } => format!("read_fs:{scope}"),
            Capability::WriteFs { scope } => format!("write_fs:{scope}"),
            Capability::Network => "network".to_owned(),
        }
    }
}

fn parse_scope(scope: &str) -> Result<String, String> {
    if scope.is_empty() {
        return Err("capability scope must not be empty".to_owned());
    }
    if scope.contains(':') {
        return Err(format!(
            "capability scope `{scope}` must not contain `:` (only one `:` separator is allowed)"
        ));
    }
    Ok(scope.to_owned())
}

/// Per-plugin set of capabilities the user has granted.
///
/// `BTreeSet` (rather than a `Vec`) gives O(log n) membership checks
/// for the pre-call gate and a deterministic iteration order for
/// any UI that wants to render the granted set.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilityGrant {
    granted: BTreeSet<Capability>,
}

impl CapabilityGrant {
    /// Empty grant — no capabilities approved. The default an
    /// initially-loaded plugin starts with before the user reviews
    /// its manifest.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Add one capability to the grant.
    pub fn grant(&mut self, capability: Capability) {
        self.granted.insert(capability);
    }

    /// `true` iff this grant covers `wanted`. Used by the pre-call
    /// gate in [`crate::PluginHandle::call_hook`].
    pub fn allows(&self, wanted: &Capability) -> bool {
        self.granted.contains(wanted)
    }

    /// Iterate the granted capabilities in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.granted.iter()
    }

    /// First capability in `required` that this grant does not
    /// cover, or `None` if every required capability is granted.
    /// The pre-call gate uses this to fail fast with a single
    /// [`crate::PluginError::CapabilityDenied`] diagnostic.
    pub fn first_missing<'a, I>(&self, required: I) -> Option<&'a Capability>
    where
        I: IntoIterator<Item = &'a Capability>,
    {
        required.into_iter().find(|cap| !self.allows(cap))
    }
}

/// Build a [`CapabilityGrant`] from an iterator of [`Capability`]
/// values. Duplicates collapse via the underlying `BTreeSet`. Lets
/// callers write `CapabilityGrant::from_iter([cap_a, cap_b])` or
/// `caps_iter.collect::<CapabilityGrant>()`.
impl FromIterator<Capability> for CapabilityGrant {
    fn from_iter<I: IntoIterator<Item = Capability>>(iter: I) -> Self {
        Self {
            granted: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_round_trips_through_as_str() {
        let cases = [
            "read_fs:source",
            "read_fs:dest",
            "write_fs:dest",
            "write_fs:user-cache",
            "network",
        ];
        for raw in cases {
            let parsed = Capability::parse(raw).unwrap_or_else(|e| panic!("{raw}: {e}"));
            assert_eq!(parsed.as_str(), raw, "{raw}");
        }
    }

    #[test]
    fn unknown_capability_is_rejected() {
        let err = Capability::parse("read_socket:tcp").expect_err("must reject");
        assert!(err.contains("unknown capability"), "{err}");
    }

    #[test]
    fn empty_scope_is_rejected() {
        let err = Capability::parse("read_fs:").expect_err("empty scope must reject");
        assert!(err.contains("must not be empty"), "{err}");
    }

    #[test]
    fn double_colon_scope_is_rejected() {
        let err =
            Capability::parse("read_fs:source:extra").expect_err("double colon must reject");
        assert!(err.contains("must not contain `:`"), "{err}");
    }

    #[test]
    fn grant_membership() {
        let grant = CapabilityGrant::from_iter([
            Capability::ReadFs {
                scope: "source".into(),
            },
            Capability::Network,
        ]);
        assert!(grant.allows(&Capability::Network));
        assert!(grant.allows(&Capability::ReadFs {
            scope: "source".into()
        }));
        assert!(!grant.allows(&Capability::ReadFs {
            scope: "dest".into()
        }));
        assert!(!grant.allows(&Capability::WriteFs {
            scope: "source".into()
        }));
    }

    #[test]
    fn first_missing_returns_first_ungranted() {
        let grant = CapabilityGrant::from_iter([Capability::Network]);
        let required = [
            Capability::Network,
            Capability::ReadFs {
                scope: "source".into(),
            },
            Capability::WriteFs {
                scope: "dest".into(),
            },
        ];
        let missing = grant
            .first_missing(required.iter())
            .expect("at least one missing");
        assert_eq!(
            *missing,
            Capability::ReadFs {
                scope: "source".into()
            }
        );
    }

    #[test]
    fn first_missing_none_when_all_granted() {
        let grant = CapabilityGrant::from_iter([
            Capability::Network,
            Capability::ReadFs {
                scope: "source".into(),
            },
        ]);
        let required = [
            Capability::Network,
            Capability::ReadFs {
                scope: "source".into(),
            },
        ];
        assert!(grant.first_missing(required.iter()).is_none());
    }
}
