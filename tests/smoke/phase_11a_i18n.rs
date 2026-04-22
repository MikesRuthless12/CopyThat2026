//! Phase 11a smoke test — i18n core.
//!
//! Per Phase 11a scope (see `CopyThat2026-Build-Prompts-Guide.md`):
//! the full per-locale screenshot + pixelmatch visual-regression
//! harness is deferred to Phase 18 polish. This smoke test instead
//! verifies three cheap-but-load-bearing invariants:
//!
//! 1. **Placeholder integrity.** For every locale, every translation
//!    of a key with `{ $var }` placeables in `en` carries the same
//!    placeable names. A missing `{ $count }` leaks the brace-var
//!    literal to the end user; catching this in CI beats catching it
//!    in a translator review.
//!
//! 2. **No cross-locale placeholder drift.** A locale is not allowed
//!    to *introduce* a placeable (`{ $foo }`) that `en` does not
//!    declare — the runtime would leave it unsubstituted.
//!
//! 3. **Required Phase 11a keys present in `en`.** A guard against
//!    accidentally deleting the keys that `format.ts` and the
//!    Svelte components depend on. Phase 11a's ICU-like duration
//!    rendering would silently fall back to `{duration-seconds}` if
//!    a key vanished.
//!
//! The test is deterministic, has no filesystem side effects beyond
//! reading `locales/*/copythat.ftl`, and exits non-zero on any
//! violation. `cargo test -p copythat-ui phase_11a_i18n` runs it.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

/// Keys that the Phase 11a UI and format layer depend on. Adding
/// one here means: if it disappears from `en`, this smoke test
/// fails before any locale-drift check even runs.
const REQUIRED_KEYS: &[&str] = &[
    "header-language-label",
    "header-language-title",
    "kind-copy",
    "kind-move",
    "status-running",
    "status-succeeded",
    "status-failed",
    "status-cancelled",
    "history-search-placeholder",
    "toast-history-purged",
    "err-source-required",
    "err-destination-empty",
    "err-source-empty",
    "duration-lt-1s",
    "duration-ms",
    "duration-seconds",
    "duration-minutes-seconds",
    "duration-hours-minutes",
    "duration-zero",
    "rate-unit-per-second",
];

#[test]
fn phase_11a_i18n_invariants() {
    let locales_root = repo_root().join("locales");
    let bundles: Vec<(&str, BTreeMap<String, String>)> = LOCALES
        .iter()
        .map(|code| {
            let path = locales_root.join(code).join("copythat.ftl");
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            (*code, parse_ftl(&content))
        })
        .collect();

    // -- Invariant 3: required Phase 11a keys present in `en`.
    let en = &bundles
        .iter()
        .find(|(c, _)| *c == "en")
        .expect("en locale missing from LOCALES table")
        .1;
    for key in REQUIRED_KEYS {
        assert!(
            en.contains_key(*key),
            "Phase 11a required key `{key}` missing from locales/en/copythat.ftl"
        );
    }

    // -- Invariants 1 + 2: placeable parity.
    let en_placeables: BTreeMap<String, Vec<String>> =
        en.iter().map(|(k, v)| (k.clone(), placeables(v))).collect();

    let mut violations: Vec<String> = Vec::new();
    for (code, bundle) in &bundles {
        if *code == "en" {
            continue;
        }
        for (key, expected) in &en_placeables {
            let Some(translated) = bundle.get(key) else {
                // Key-parity is checked by `xtask i18n-lint`; this test
                // focuses on placeable behaviour, not presence.
                continue;
            };
            let got = placeables(translated);
            // Every `en` placeable must appear in the translation.
            for name in expected {
                if !got.contains(name) {
                    violations.push(format!(
                        "[{code}] key `{key}` missing placeable `{{ ${name} }}` (value: {translated:?})"
                    ));
                }
            }
            // The translation must not introduce a placeable `en`
            // doesn't declare — the frontend wouldn't substitute it.
            for name in &got {
                if !expected.contains(name) {
                    violations.push(format!(
                        "[{code}] key `{key}` introduces unknown placeable `{{ ${name} }}` not in `en` (value: {translated:?})"
                    ));
                }
            }
        }
    }

    if !violations.is_empty() {
        let count = violations.len();
        for v in &violations {
            eprintln!("{v}");
        }
        panic!(
            "{count} placeable-parity violation(s) across {} locales",
            bundles.len() - 1
        );
    }
}

/// Collect every placeable variable name (the `xxx` in `{ $xxx }`)
/// referenced by one translation value. Empty vec means no runtime
/// interpolation needed; equal vecs across locales mean every call
/// site produces the same substituted output regardless of language.
fn placeables(value: &str) -> Vec<String> {
    // A Fluent placeable is `{ $name }` — with optional whitespace
    // around the `$name`. This matcher accepts what our runtime
    // accepts (see `substitute()` in `apps/copythat-ui/src/lib/i18n.ts`).
    let mut out = Vec::new();
    let mut rest = value;
    while let Some(lb) = rest.find('{') {
        rest = &rest[lb + 1..];
        // Skip whitespace.
        let start = rest
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(rest.len());
        let after_ws = &rest[start..];
        if !after_ws.starts_with('$') {
            continue;
        }
        let name_start = &after_ws[1..];
        let name_end = name_start
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .unwrap_or(name_start.len());
        let name = &name_start[..name_end];
        if !name.is_empty() {
            out.push(name.to_string());
        }
        rest = &name_start[name_end..];
    }
    out.sort();
    out.dedup();
    out
}

/// Minimal `.ftl` parser: accept `key = value` lines, reject
/// continuations / attributes / comments. Matches the parser in
/// `apps/copythat-ui/src-tauri/src/i18n.rs` so this test mirrors
/// what the runtime sees.
fn parse_ftl(content: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for raw in content.lines() {
        if raw.is_empty() {
            continue;
        }
        let first = match raw.chars().next() {
            Some(c) => c,
            None => continue,
        };
        if matches!(first, ' ' | '\t' | '.' | '*' | '[' | '}' | '#') {
            continue;
        }
        let Some((ident, value)) = raw.split_once('=') else {
            continue;
        };
        let key = ident.trim();
        if key.is_empty() {
            continue;
        }
        out.insert(key.to_string(), value.trim().to_string());
    }
    out
}

fn repo_root() -> PathBuf {
    // The test is invoked from `apps/copythat-ui/src-tauri`. Walk up
    // until we see both `Cargo.toml` and `locales/`.
    let start = std::env::current_dir().expect("current_dir");
    let mut cur = start.as_path();
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = cur
            .parent()
            .expect("phase 11a smoke: could not locate repo root");
    }
}

#[test]
fn placeable_scanner_self_check() {
    assert_eq!(placeables("hello"), Vec::<String>::new());
    assert_eq!(placeables("{ $count } items"), vec!["count".to_string()]);
    assert_eq!(
        placeables("{ $h } h { $m } min"),
        vec!["h".to_string(), "m".to_string()]
    );
    // Duplicates collapse.
    assert_eq!(placeables("{ $x } and { $x }"), vec!["x".to_string()]);
    // Non-variable placeables (e.g. select/term refs) are ignored.
    assert_eq!(placeables("{ foo }"), Vec::<String>::new());
}
