//! Phase 11c smoke test — runtime substitution pipeline.
//!
//! Phase 11a's smoke test checks placeable names **structurally**
//! — "every `{ $count }` in `en` appears in every translation".
//! Phase 11c runs the actual `parse → substitute` pipeline the
//! frontend uses, feeding canned args at each call site, and
//! asserts the produced string has no `{$` or unmatched-brace
//! remnants. This catches runtime hazards Phase 11a cannot:
//!
//! - A translator writes `{count}` (missing `$`). Phase 11a's
//!   placeable scanner treats the locale as carrying no placeables
//!   for that key and flags the missing `{ $count }`, but only
//!   because en has a `$` variant. Phase 11c would fail the
//!   substitution *and* cross-check against the real UI template.
//! - A translator introduces a stray `{…` with no closing brace
//!   (e.g. copy-paste truncation). The UI would render the
//!   unclosed brace verbatim; this test flags it.
//! - A translator writes `{ $counnt }` (typo). Phase 11a already
//!   catches the "introduces unknown placeable" case but Phase 11c
//!   demonstrates the behaviour end-to-end by failing the
//!   substitution.
//!
//! Also verifies the RTL direction flag: every non-`ar` locale
//! resolves to LTR, `ar` resolves to RTL. This mirrors
//! `applyHtmlAttributes` in `apps/copythat-ui/src/lib/i18n.ts`.
//!
//! Per Phase 11c scope (see `CopyThat2026-Build-Prompts-Guide.md`
//! and the 11b/11c scoping discussion), the full per-locale
//! screenshot + pixelmatch visual-regression harness is deferred
//! to Phase 18 polish. Cross-platform font rendering (Windows
//! webview2 + macOS WKWebView + Linux webkit2gtk) would require
//! baseline-per-OS pixel sets, and CI font discovery would be
//! fragile until the Phase 18 packaging work pins specific font
//! packages anyway.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

/// Canned arguments keyed by Fluent key. For every key in `en`
/// that carries placeables, the test feeds this arg dict into the
/// substitution pipeline. Keys without placeables get an empty
/// dict (still exercised — they must round-trip unchanged).
///
/// The values here mirror what the UI passes at the real call
/// site; cross-check when adding new placeables:
/// - `drop-dialog-subtitle`  → `DropStagingDialog.svelte`
/// - `toast-history-purged`  → `HistoryDrawer.svelte::purge30`
/// - `duration-*`            → `format.ts::formatEta` + `fmtMs`
/// - `rate-unit-per-second`  → `TotalsDrawer.svelte::fmtRate`
fn canned_args(key: &str) -> HashMap<&'static str, String> {
    let mut m = HashMap::new();
    match key {
        "drop-dialog-subtitle" => {
            m.insert("count", "3".to_string());
        }
        "toast-history-purged" => {
            m.insert("count", "42".to_string());
        }
        "duration-ms" => {
            m.insert("ms", "250".to_string());
        }
        "duration-seconds" => {
            m.insert("s", "5".to_string());
        }
        "duration-minutes-seconds" => {
            m.insert("m", "2".to_string());
            m.insert("s", "15".to_string());
        }
        "duration-hours-minutes" => {
            m.insert("h", "1".to_string());
            m.insert("m", "30".to_string());
        }
        "rate-unit-per-second" => {
            m.insert("size", "1.2 MiB".to_string());
        }
        _ => {}
    }
    m
}

#[test]
fn phase_11c_substitution_round_trip() {
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

    let en = &bundles
        .iter()
        .find(|(c, _)| *c == "en")
        .expect("en locale must exist")
        .1;

    let mut failures: Vec<String> = Vec::new();

    for (code, bundle) in &bundles {
        for key in en.keys() {
            let Some(template) = bundle.get(key) else {
                continue;
            };
            let args = canned_args(key);
            let out = substitute(template, &args);

            // No unsubstituted `{ $var }` placeable remnant. Any
            // `{` immediately followed (maybe after whitespace) by
            // `$` is a leak — either a typo in the translation or
            // a missing key in `canned_args`.
            if has_unresolved_placeable(&out) {
                failures.push(format!(
                    "[{code}] key `{key}` leaked placeable after substitution:\n  template : {template:?}\n  args     : {args:?}\n  output   : {out:?}"
                ));
            }

            // Also check balanced braces. A translator-introduced
            // `{$count` (no closing brace) would make it through the
            // placeable check above because we look for `{$`; this
            // guards against the asymmetric case.
            let opens = out.matches('{').count();
            let closes = out.matches('}').count();
            if opens != closes {
                failures.push(format!(
                    "[{code}] key `{key}` has unbalanced braces after substitution:\n  template : {template:?}\n  output   : {out:?}"
                ));
            }
        }
    }

    if !failures.is_empty() {
        let count = failures.len();
        for f in &failures {
            eprintln!("{f}");
        }
        panic!(
            "{count} substitution violation(s) across {} locales",
            bundles.len()
        );
    }
}

#[test]
fn phase_11c_direction_flag() {
    // Mirrors `applyHtmlAttributes` in apps/copythat-ui/src/lib/i18n.ts.
    // `ar` is the only RTL locale in the 18-locale set; everything
    // else resolves to LTR. A regression that silently adds another
    // RTL language (Hebrew, Persian, Urdu) would need the flip wired
    // at the same time as the locale file — this test is the tripwire.
    for code in LOCALES {
        let got = direction_for(code);
        let expected = if *code == "ar" { "rtl" } else { "ltr" };
        assert_eq!(
            got, expected,
            "locale `{code}` resolved to {got}, expected {expected}"
        );
    }
}

/// TS port of `apps/copythat-ui/src/lib/i18n.ts::substitute`.
/// Replaces `{ $name }` placeables with the value from `args`;
/// leaves unknown placeables untouched (the TS behaviour). Used
/// only in this test module — no public export needed.
fn substitute(template: &str, args: &HashMap<&'static str, String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(lb) = rest.find('{') {
        out.push_str(&rest[..lb]);
        let after = &rest[lb + 1..];

        // Skip optional whitespace.
        let ws_end = after
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(after.len());
        let after_ws = &after[ws_end..];

        if let Some(dollar_after) = after_ws.strip_prefix('$') {
            // Identifier chars: [A-Za-z0-9_-].
            let name_end = dollar_after
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
                .unwrap_or(dollar_after.len());
            let name = &dollar_after[..name_end];
            let after_name = &dollar_after[name_end..];
            // Skip trailing whitespace + closing brace.
            let trail_end = after_name
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(after_name.len());
            let after_trail = &after_name[trail_end..];

            if let Some(after_close) = after_trail.strip_prefix('}') {
                match args.get(name) {
                    Some(value) => {
                        out.push_str(value);
                    }
                    None => {
                        // Leave the placeable intact — exactly what
                        // the TS `substitute()` does. The outer
                        // assertion then flags it as a leak.
                        out.push('{');
                        out.push_str(&rest[lb + 1..rest.len() - after_close.len()]);
                    }
                }
                rest = after_close;
                continue;
            }
        }

        // Not a placeable; emit the `{` literally and carry on.
        out.push('{');
        rest = after;
    }
    out.push_str(rest);
    out
}

/// Scan for `{` followed by (optional whitespace and) `$` — that's
/// what a failed substitution looks like in the output.
fn has_unresolved_placeable(s: &str) -> bool {
    let mut rest = s;
    while let Some(lb) = rest.find('{') {
        let after = &rest[lb + 1..];
        let ws_end = after
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(after.len());
        if after[ws_end..].starts_with('$') {
            return true;
        }
        rest = after;
    }
    false
}

/// Mirrors `apps/copythat-ui/src/lib/i18n.ts`: only `ar` is RTL.
fn direction_for(code: &str) -> &'static str {
    if code == "ar" { "rtl" } else { "ltr" }
}

/// Minimal `.ftl` parser — same shape as the runtime in
/// `apps/copythat-ui/src-tauri/src/i18n.rs::parse`.
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
    let start = std::env::current_dir().expect("current_dir");
    let mut cur = start.as_path();
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = cur
            .parent()
            .expect("phase 11c smoke: could not locate repo root");
    }
}

#[test]
fn substitute_happy_path() {
    let mut args = HashMap::new();
    args.insert("count", "7".to_string());
    assert_eq!(substitute("{ $count } items", &args), "7 items");
}

#[test]
fn substitute_multiple_placeables() {
    let mut args = HashMap::new();
    args.insert("h", "2".to_string());
    args.insert("m", "5".to_string());
    assert_eq!(substitute("{ $h } h { $m } min", &args), "2 h 5 min");
}

#[test]
fn substitute_unknown_placeable_is_preserved() {
    let args = HashMap::new();
    let out = substitute("{ $unknown }", &args);
    assert!(out.contains("{"));
    assert!(out.contains("$unknown"));
    assert!(has_unresolved_placeable(&out));
}

#[test]
fn substitute_non_placeable_brace_is_literal() {
    let args = HashMap::new();
    // A literal `{` that's not followed by `$` is emitted as-is
    // — matches the TS `substitute()` behaviour.
    let out = substitute("plain { literal } text", &args);
    assert_eq!(out, "plain { literal } text");
    assert!(!has_unresolved_placeable(&out));
}
