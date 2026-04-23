//! Phase 22 — reusable per-pattern collision rules.
//!
//! A [`ConflictProfile`] is an ordered list of `(glob, resolution)`
//! rules plus an optional fallback. When the Phase 22 aggregate
//! conflict dialog asks "what should happen to `foo.docx`?", the
//! profile is consulted in order — first match wins, fallback
//! resolves anything that didn't match. The UI layer also
//! accumulates rules into an in-memory "live" profile during a
//! running job (via "Apply to all of this extension" bulk actions)
//! and can offer to persist that live profile under a user-chosen
//! name at job end.
//!
//! Persistence: [`ConflictProfileSettings`] is attached as a
//! [`crate::Settings`] field so it round-trips through the
//! existing `settings.toml` + profile `settings-profiles/*.json`
//! storage with zero extra IO. The `active` field names the
//! currently-selected profile; `None` means "no rules, always
//! prompt".
//!
//! The match target is the file's *basename* (everything after
//! the last path separator), because collision rules key off
//! filename shape (e.g. `*.docx`) far more often than full path
//! shape. Callers who care about directory globs pass the relative
//! source path in `ConflictProfile::match_basename_or_path` — the
//! helper tries the basename first, then the whole path. Kept as
//! two string views rather than a `&Path` so this module stays
//! free of platform `PathBuf` / separator concerns (the IPC
//! boundary hands us forward-slash-normalised strings already).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Resolution the rule engine should apply when a pattern matches.
///
/// Mirrors [`crate::Settings`]-adjacent wire enums: kebab-case on
/// disk and across the Tauri IPC boundary. A superset of the
/// engine's `CollisionResolution` because the UI layer can express
/// "newer wins" / "larger wins" which it resolves to Overwrite or
/// Skip by comparing source + destination metadata *before* the
/// engine sees the decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictRuleResolution {
    /// Leave destination alone; skip the source file.
    Skip,
    /// Truncate + overwrite destination.
    Overwrite,
    /// Overwrite only when source mtime > destination mtime; else
    /// skip. Resolved against metadata by the runner.
    OverwriteIfNewer,
    /// Overwrite only when source size > destination size; else
    /// skip. Resolved against metadata by the runner.
    OverwriteIfLarger,
    /// Keep both — runner generates a `_2` / `_3` suffix path and
    /// issues a Rename to the engine.
    KeepBoth,
}

impl ConflictRuleResolution {
    /// Stable wire string for IPC / CSV / logs. Kept small and
    /// lowercase so it slots into an existing wire-enum translator
    /// without a second mapping.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Skip => "skip",
            Self::Overwrite => "overwrite",
            Self::OverwriteIfNewer => "overwrite-if-newer",
            Self::OverwriteIfLarger => "overwrite-if-larger",
            Self::KeepBoth => "keep-both",
        }
    }

    /// Parse the wire string. Unknown values resolve to `Skip` so a
    /// settings-file written by a newer binary loads without
    /// panicking on an older one — the safer-by-default fallback.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "overwrite" => Self::Overwrite,
            "overwrite-if-newer" => Self::OverwriteIfNewer,
            "overwrite-if-larger" => Self::OverwriteIfLarger,
            "keep-both" => Self::KeepBoth,
            _ => Self::Skip,
        }
    }
}

/// One pattern → resolution binding. `pattern` is a `globset`-
/// compatible shell glob (e.g. `*.docx`, `**/tmp/*`, `backup-*.tar.gz`).
/// The engine does *not* validate the pattern here — rule entry is a
/// UI concern, and the runner's match helper falls through on an
/// unparseable pattern so a bad rule can't wedge a job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConflictRule {
    pub pattern: String,
    pub resolution: ConflictRuleResolution,
}

/// Ordered rule set + optional fallback. Order matters — the first
/// pattern that matches wins, so users should list more-specific
/// patterns first and leave broad catch-alls at the bottom.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ConflictProfile {
    /// Rules evaluated top-to-bottom. Empty is valid — callers treat
    /// an empty profile with `None` fallback as "no rules".
    pub rules: Vec<ConflictRule>,
    /// Applied when no rule matches. `None` leaves the collision
    /// unresolved (falls through to the interactive prompt).
    pub fallback: Option<ConflictRuleResolution>,
}

impl ConflictProfile {
    /// Try matching `basename` first, then `rel_path`, returning the
    /// winning rule's `(pattern, resolution)` and the index in
    /// `rules`. Falls through to `fallback` if nothing matches.
    ///
    /// `basename` is the filename-only view (post-last-slash);
    /// `rel_path` is the path-relative-to-src-root view (forward-
    /// slashes regardless of platform). Supplying both lets rules
    /// like `*.docx` match filenames anywhere in the tree without
    /// needing `**/*.docx`, while path-shaped rules like
    /// `cache/**` still work.
    pub fn match_basename_or_path(
        &self,
        basename: &str,
        rel_path: &str,
    ) -> Option<ConflictMatch<'_>> {
        for (idx, rule) in self.rules.iter().enumerate() {
            if glob_matches(&rule.pattern, basename) || glob_matches(&rule.pattern, rel_path) {
                return Some(ConflictMatch {
                    index: idx,
                    pattern: &rule.pattern,
                    resolution: rule.resolution,
                    is_fallback: false,
                });
            }
        }
        self.fallback.map(|res| ConflictMatch {
            index: self.rules.len(),
            pattern: "*",
            resolution: res,
            is_fallback: true,
        })
    }

    /// Convenience constructor. Keeps smoke tests and unit tests
    /// legible — the struct-with-two-fields form is verbose.
    pub fn with_rules(rules: Vec<ConflictRule>) -> Self {
        Self {
            rules,
            fallback: None,
        }
    }

    /// Builder — set the fallback without rebuilding the rule vec.
    pub fn with_fallback(mut self, fallback: ConflictRuleResolution) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// True when this profile has nothing to apply — empty rules and
    /// no fallback. The runner skips the per-collision match call in
    /// that case so a "no rules" active profile has zero overhead.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.fallback.is_none()
    }
}

/// One hit from [`ConflictProfile::match_basename_or_path`]. Carries
/// the pattern that matched + the resolution; consumers wire this
/// into the `matched_rule` field on the Phase 22 auto-resolved
/// IPC event so the UI can render "✓ via rule '*.docx → newer'".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConflictMatch<'a> {
    /// Index into `rules` that matched, or `rules.len()` when the
    /// fallback fired. Useful for rule-order debugging + UI
    /// highlighting.
    pub index: usize,
    /// The pattern string that matched. Borrowed from the profile
    /// so tests can assert on the literal rule text.
    pub pattern: &'a str,
    pub resolution: ConflictRuleResolution,
    /// `true` when the match came from `fallback`, `false` when an
    /// explicit rule matched.
    pub is_fallback: bool,
}

/// `Settings::conflict_profiles` — persisted named profiles +
/// which one is currently active.
///
/// Default: no profiles, no active selection. Forward-compatible:
/// `#[serde(default)]` on every field so an older `settings.toml`
/// missing this section loads cleanly.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ConflictProfileSettings {
    /// Name of the profile applied to new jobs by default. `None`
    /// means "no profile active" — every collision falls through
    /// to the aggregate dialog for manual resolution.
    pub active: Option<String>,
    /// User-saved named profiles. `BTreeMap` so the UI sees a
    /// stable alphabetical order when listing.
    pub profiles: BTreeMap<String, ConflictProfile>,
}

impl ConflictProfileSettings {
    /// Look up the currently-active profile. `None` when no profile
    /// is selected OR the selected name is not in the map (stale
    /// handle from a deleted profile, etc.).
    pub fn active_profile(&self) -> Option<&ConflictProfile> {
        self.active
            .as_deref()
            .and_then(|name| self.profiles.get(name))
    }
}

/// Compile + evaluate a single glob pattern against `subject`.
/// Returns `false` on any compile error — the UI validates at save
/// time, so a runtime match against a malformed rule is a
/// "shouldn't happen" path that still must not crash the runner.
fn glob_matches(pattern: &str, subject: &str) -> bool {
    match globset::Glob::new(pattern) {
        Ok(g) => g.compile_matcher().is_match(subject),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(pattern: &str, resolution: ConflictRuleResolution) -> ConflictRule {
        ConflictRule {
            pattern: pattern.to_string(),
            resolution,
        }
    }

    #[test]
    fn first_match_wins_over_later_rules() {
        let profile = ConflictProfile::with_rules(vec![
            rule("*.docx", ConflictRuleResolution::OverwriteIfNewer),
            rule("*", ConflictRuleResolution::Skip),
        ]);
        let m = profile
            .match_basename_or_path("report.docx", "docs/report.docx")
            .unwrap();
        assert_eq!(m.index, 0);
        assert_eq!(m.resolution, ConflictRuleResolution::OverwriteIfNewer);
        assert!(!m.is_fallback);
        assert_eq!(m.pattern, "*.docx");
    }

    #[test]
    fn fallback_applies_only_when_no_rule_matches() {
        let profile = ConflictProfile {
            rules: vec![rule("*.txt", ConflictRuleResolution::Skip)],
            fallback: Some(ConflictRuleResolution::KeepBoth),
        };
        let m = profile.match_basename_or_path("a.jpg", "a.jpg").unwrap();
        assert!(m.is_fallback);
        assert_eq!(m.resolution, ConflictRuleResolution::KeepBoth);

        let m = profile.match_basename_or_path("a.txt", "a.txt").unwrap();
        assert!(!m.is_fallback);
        assert_eq!(m.resolution, ConflictRuleResolution::Skip);
    }

    #[test]
    fn empty_profile_with_no_fallback_returns_none() {
        let profile = ConflictProfile::default();
        assert!(profile.is_empty());
        assert!(profile.match_basename_or_path("x.txt", "x.txt").is_none());
    }

    #[test]
    fn path_shaped_glob_matches_rel_path_when_basename_misses() {
        let profile =
            ConflictProfile::with_rules(vec![rule("cache/**", ConflictRuleResolution::Skip)]);
        // Basename alone doesn't carry the path prefix, but rel_path does.
        let m = profile
            .match_basename_or_path("thumb.png", "cache/thumb.png")
            .unwrap();
        assert_eq!(m.resolution, ConflictRuleResolution::Skip);
    }

    #[test]
    fn malformed_pattern_does_not_panic_and_does_not_match() {
        let profile = ConflictProfile::with_rules(vec![rule(
            "[unterminated",
            ConflictRuleResolution::Overwrite,
        )]);
        assert!(
            profile
                .match_basename_or_path("foo.bar", "foo.bar")
                .is_none()
        );
    }

    #[test]
    fn resolution_wire_strings_round_trip() {
        for res in [
            ConflictRuleResolution::Skip,
            ConflictRuleResolution::Overwrite,
            ConflictRuleResolution::OverwriteIfNewer,
            ConflictRuleResolution::OverwriteIfLarger,
            ConflictRuleResolution::KeepBoth,
        ] {
            assert_eq!(ConflictRuleResolution::from_wire(res.as_str()), res);
        }
        // Unknown wire string → Skip (safe default).
        assert_eq!(
            ConflictRuleResolution::from_wire("bogus"),
            ConflictRuleResolution::Skip
        );
    }

    #[test]
    fn active_profile_resolves_via_name() {
        let mut cps = ConflictProfileSettings::default();
        cps.profiles.insert(
            "imports".to_string(),
            ConflictProfile::with_rules(vec![rule("*.tmp", ConflictRuleResolution::Skip)]),
        );
        assert!(cps.active_profile().is_none());
        cps.active = Some("imports".to_string());
        assert_eq!(cps.active_profile().unwrap().rules.len(), 1);
        cps.active = Some("missing".to_string());
        assert!(cps.active_profile().is_none());
    }
}
