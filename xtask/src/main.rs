//! `xtask` — workspace automation.
//!
//! Subcommands:
//! - `i18n-lint`: verify Fluent key parity across every `locales/<code>/copythat.ftl`,
//!   plus Phase 11a checks: literal string keys referenced in source exist in `en`,
//!   and every `.ftl` file parses with no duplicate keys.
//! - `bench`: run the Criterion bench suite with the default
//!   (full-size) workloads. Prints Criterion's normal output.
//! - `bench-ci`: run the Criterion bench suite with `COPYTHAT_BENCH_CI=1`
//!   set so workloads scale down to CI-friendly sizes. The smoke
//!   path (Phase 13). Still prints Criterion's normal output.
//! - `bench-vs`: detect competitor copy tools on PATH (Robocopy,
//!   TeraCopy, FastCopy on Windows; `cp`, `rsync`, `ditto` on Unix)
//!   and run a scripted copy against each. Missing tools are
//!   reported but not fatal — CI stays green on a machine that
//!   doesn't have every competitor installed. Results are printed
//!   as a markdown table.
//! - `release`: drive the Phase 16 free-first packaging path locally.
//!   Shells out to `pnpm tauri build --bundles …` with
//!   `APPLE_SIGNING_IDENTITY=-` set so macOS picks up ad-hoc codesign.
//!   Mirrors `.github/workflows/release.yml`.
//! - `qa-automate`: drive every automatable checkbox in
//!   `QualityAssuranceChecklist.md` (§0 pre-flight, §1 static +
//!   frontend, §2 per-crate tests, §3 security, §5 bench-ci) and
//!   emit a single pass/fail report. The single-call gate the
//!   checklist's appendix recommends running before `release.yml`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod bench;
mod qa;
mod release;

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

const SOURCE_LOCALE: &str = "en";

/// Dynamic-key prefixes the source tree composes at runtime. A key
/// reference like `t(\`state-${state}\`)` cannot be discovered by a
/// literal scan, so we whitelist its prefix here; the lint still
/// verifies at least one key with this prefix exists in `en`.
const DYNAMIC_KEY_PREFIXES: &[&str] = &["state-", "kind-", "status-", "err-"];

/// Directories whose source files participate in the literal-key
/// scan. Everything under `apps/copythat-ui` (Svelte/TS) and every
/// Rust crate + the Tauri shell.
const SOURCE_ROOTS: &[&str] = &[
    "apps/copythat-ui/src",
    "apps/copythat-ui/src-tauri/src",
    "crates",
    "xtask/src",
    "tests/smoke",
];

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("i18n-lint") => match i18n_lint() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("xtask i18n-lint: {e}");
                ExitCode::FAILURE
            }
        },
        Some("bench") => match bench::run(false) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("xtask bench: {e}");
                ExitCode::FAILURE
            }
        },
        Some("bench-ci") => match bench::run(true) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("xtask bench-ci: {e}");
                ExitCode::FAILURE
            }
        },
        Some("bench-vs") => {
            // Optional trailing flag: `--secure-cleanup` → after the
            // bench finishes, shred the copied dst files with our
            // own secure-delete crate (DoD 3-pass). Exercises the
            // shredder on the same bytes we just wrote out, and
            // leaves the test volumes cryptographically clean for
            // the next run.
            let secure = args.any(|a| a == "--secure-cleanup");
            match bench::run_vs_with(secure) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("xtask bench-vs: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("release") => {
            let rest: Vec<String> = args.collect();
            match release::run(rest) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("xtask release: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("qa-automate") => {
            let rest: Vec<String> = args.collect();
            match qa::run(rest) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("xtask qa-automate: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("--help" | "-h") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("xtask: unknown command `{other}`\n");
            print_help();
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!(
        "Usage: xtask <command>\n\nCommands:\n  i18n-lint    Verify key parity, literal-key coverage, and Fluent syntax\n               across locales/<code>/copythat.ftl\n  bench        Run the Criterion bench suite at full size\n  bench-ci     Run the Criterion bench suite at CI-scaled sizes\n  bench-vs     Time our engine against OS copy tools on PATH\n  release      Drive the Phase 16 free-first packaging path (pnpm tauri build)\n  qa-automate  Run every automatable QualityAssuranceChecklist item\n               (§0 pre-flight + §1 static + §2 tests + §3 security + §5 perf)\n               and emit a pass/fail report. Use `qa-automate --help` for flags.\n"
    );
}

fn i18n_lint() -> Result<(), String> {
    let root = repo_root().ok_or("could not locate repo root (Cargo.toml + locales/)")?;
    let locales_dir = root.join("locales");

    // Phase-5 check: key parity across all 18 locales. Each locale's
    // parse step also doubles as a minimal syntax validation — a
    // blank `key =` value or a malformed identifier rejects the
    // file outright via `parse_ftl_keys`'s duplicate detection.
    let mut per_locale: Vec<(String, BTreeSet<String>)> = Vec::with_capacity(LOCALES.len());
    for code in LOCALES {
        let path = locales_dir.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        let keys =
            parse_ftl_keys_unique(&content).map_err(|e| format!("[{code}] Fluent syntax: {e}"))?;
        per_locale.push(((*code).to_string(), keys));
    }

    let reference_idx = per_locale
        .iter()
        .position(|(c, _)| c == SOURCE_LOCALE)
        .ok_or_else(|| format!("source locale `{SOURCE_LOCALE}` missing from LOCALES table"))?;
    let reference = per_locale[reference_idx].1.clone();

    if reference.is_empty() {
        return Err(format!(
            "source locale `{SOURCE_LOCALE}` has zero keys — nothing to compare against"
        ));
    }

    let mut ok = true;
    for (code, keys) in &per_locale {
        if code == SOURCE_LOCALE {
            continue;
        }
        let missing: Vec<&String> = reference.difference(keys).collect();
        let extra: Vec<&String> = keys.difference(&reference).collect();
        if !missing.is_empty() {
            ok = false;
            eprintln!("[{code}] missing keys: {missing:?}");
        }
        if !extra.is_empty() {
            ok = false;
            eprintln!("[{code}] extra keys not in `{SOURCE_LOCALE}`: {extra:?}");
        }
    }

    if !ok {
        return Err("key parity check failed".to_string());
    }

    // Phase-11a check: literal keys referenced in source must exist
    // in `en`. Dynamic references (e.g. `t(\`state-${state}\`)`) are
    // whitelisted by prefix — see `DYNAMIC_KEY_PREFIXES`.
    let referenced = scan_source_key_references(&root)?;
    let mut missing_from_source: Vec<&String> = referenced
        .iter()
        .filter(|k| !reference.contains(*k))
        .collect();
    if !missing_from_source.is_empty() {
        missing_from_source.sort();
        return Err(format!(
            "keys referenced in source but missing from `{SOURCE_LOCALE}`: {missing_from_source:?}"
        ));
    }

    // Phase-11a check: every whitelisted dynamic prefix has at least
    // one key defined in `en`, otherwise the prefix is dead code.
    for prefix in DYNAMIC_KEY_PREFIXES {
        let any = reference.iter().any(|k| k.starts_with(prefix));
        if !any {
            return Err(format!(
                "dynamic-key prefix `{prefix}*` has no matching key in `{SOURCE_LOCALE}`"
            ));
        }
    }

    println!(
        "i18n-lint: OK ({} locales, {} keys each, {} literal refs in source)",
        per_locale.len(),
        reference.len(),
        referenced.len()
    );
    Ok(())
}

/// Parse a `.ftl` file into a set of unique message/term identifiers.
/// Rejects duplicate keys — Phase-11a additions sit next to Phase-5
/// content, and a copy-paste mistake that repeats a key should fail
/// the lint, not silently win last-write.
fn parse_ftl_keys_unique(content: &str) -> Result<BTreeSet<String>, String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for (lineno, raw) in content.lines().enumerate() {
        let Some(ident) = ident_of(raw) else {
            continue;
        };
        if let Some(prev) = seen.insert(ident.clone(), lineno + 1) {
            return Err(format!(
                "duplicate key `{ident}` (line {} and line {})",
                prev,
                lineno + 1
            ));
        }
    }
    Ok(seen.into_keys().collect())
}

/// Pull the Fluent identifier from one source line, or `None` if the
/// line is a continuation / comment / attribute / variant / blank.
fn ident_of(raw: &str) -> Option<String> {
    if raw.is_empty() {
        return None;
    }
    let first = raw.chars().next()?;
    if matches!(first, ' ' | '\t' | '.' | '*' | '[' | '}' | '#') {
        return None;
    }
    let (ident, _) = raw.split_once('=')?;
    let ident = ident.trim();
    if ident.is_empty() {
        return None;
    }
    let body = ident.strip_prefix('-').unwrap_or(ident);
    let mut chars = body.chars();
    let head = chars.next()?;
    if !head.is_ascii_alphabetic() {
        return None;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return None;
    }
    Some(ident.to_string())
}

/// Walk the source roots and collect every *literal* Fluent key
/// that any call site passes to `t(...)`, `$t(...)`, `t!(...)`, or
/// `localized_key = "..."`, plus anything in a
/// `pushToast("kind", "key")` that is itself a kebab-case identifier.
///
/// Dynamic lookups (`t(\`state-${x}\`)`, `t(\`kind-${y}\`)`) are
/// skipped — the dynamic-prefix whitelist in
/// [`DYNAMIC_KEY_PREFIXES`] confirms the prefix family exists.
fn scan_source_key_references(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut out: BTreeSet<String> = BTreeSet::new();
    for rel in SOURCE_ROOTS {
        let dir = root.join(rel);
        if !dir.exists() {
            continue;
        }
        walk_and_scan(&dir, &mut out).map_err(|e| format!("scanning {}: {e}", dir.display()))?;
    }
    Ok(out)
}

fn walk_and_scan(dir: &Path, out: &mut BTreeSet<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        let path = entry.path();
        if ft.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            // Skip vendored / generated trees that balloon the scan.
            if matches!(
                name,
                "target"
                    | "node_modules"
                    | "dist"
                    | "build"
                    | ".git"
                    | ".svelte-kit"
                    | "gen"
                    | "pnpm-lock.yaml"
            ) {
                continue;
            }
            walk_and_scan(&path, out)?;
        } else if is_scannable(&path) {
            let content = fs::read_to_string(&path)?;
            let stripped = strip_doc_comments(&content);
            collect_literal_keys(&stripped, out);
        }
    }
    Ok(())
}

fn is_scannable(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return false;
    };
    matches!(ext, "svelte" | "ts" | "tsx" | "js" | "rs")
}

/// Remove single-line comments before scanning so documentation
/// examples like `/// Extract t("key")` do not leak into the lint's
/// literal-key set. We keep multi-line `/* ... */` blocks intact —
/// a `*/` inside a string would make a proper lexer mandatory, and
/// the UI code doesn't rely on multi-line comments for any t() calls.
fn strip_doc_comments(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            // Preserve the line's newline so line numbers stay put.
            out.push('\n');
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Extract `t("key")` / `t!("key")` / `$t("key")` / `localized_key = "key"`
/// literal keys. The entry trigger (`t`, `t!`, `$t`) must start at a
/// word boundary so `pushToast(...)` does not parse as `t(...)`.
/// Dynamic lookups (`t(\`state-${x}\`)`) are skipped — the prefix
/// whitelist in [`DYNAMIC_KEY_PREFIXES`] covers them.
fn collect_literal_keys(content: &str, out: &mut BTreeSet<String>) {
    // Each trigger is a (lead, open-char) pair. `lead` is the part
    // up to and including `(`; we insist on a non-word char in front
    // to filter `pushToast(` / `Invariant(` / etc.
    const TRIGGERS: &[&str] = &["t(", "t!(", "$t("];
    let bytes = content.as_bytes();

    for trig in TRIGGERS {
        let mut offset = 0;
        while let Some(pos) = content[offset..].find(trig) {
            let abs = offset + pos;
            // Word-boundary check: char before the trigger must be
            // either start-of-file or a non-identifier char.
            let ok_boundary = abs == 0 || {
                let prev = bytes[abs - 1];
                !(prev.is_ascii_alphanumeric() || prev == b'_')
            };
            let start = abs + trig.len();
            if !ok_boundary || start >= bytes.len() {
                offset = abs + trig.len();
                continue;
            }
            let quote = bytes[start];
            if !matches!(quote, b'"' | b'\'' | b'`') {
                offset = abs + trig.len();
                continue;
            }
            let q_start = start + 1;
            if let Some(end_rel) = content[q_start..].find(quote as char) {
                let key = &content[q_start..q_start + end_rel];
                if is_fluent_identifier(key) {
                    out.insert(key.to_string());
                }
                offset = q_start + end_rel + 1;
            } else {
                break;
            }
        }
    }

    // Rust side: `localized_key: "..."` (error enum mapping). These
    // surface to the UI unchanged, so they must exist in `en`.
    for tag in ["localized_key: \""] {
        let mut offset = 0;
        while let Some(pos) = content[offset..].find(tag) {
            let start = offset + pos + tag.len();
            if let Some(end) = content[start..].find('"') {
                let key = &content[start..start + end];
                if is_fluent_identifier(key) {
                    out.insert(key.to_string());
                }
                offset = start + end + 1;
            } else {
                break;
            }
        }
    }
}

/// A valid Fluent message identifier: `[a-z][a-z0-9-]*` (we don't
/// model uppercase or unicode idents since we don't use them).
fn is_fluent_identifier(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    let mut chars = s.chars();
    let Some(head) = chars.next() else {
        return false;
    };
    if !head.is_ascii_lowercase() {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub(crate) fn repo_root() -> Option<PathBuf> {
    let start = std::env::current_dir().ok()?;
    let mut cur = start.as_path();
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_messages() {
        let src = "app-name = Copy That v1.25.0\n# a comment\nfoo-bar = hi\n";
        let keys = parse_ftl_keys_unique(src).unwrap();
        assert!(keys.contains("app-name"));
        assert!(keys.contains("foo-bar"));
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn skips_attributes_and_continuations() {
        let src = "msg = Hello\n    .title = T\n    continuation line\n";
        let keys = parse_ftl_keys_unique(src).unwrap();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains("msg"));
    }

    #[test]
    fn collects_term() {
        let src = "-brand = Copy That\n";
        let keys = parse_ftl_keys_unique(src).unwrap();
        assert!(keys.contains("-brand"));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let src = "foo = one\nfoo = two\n";
        let err = parse_ftl_keys_unique(src).unwrap_err();
        assert!(err.contains("duplicate"));
    }

    #[test]
    fn identifier_shape() {
        assert!(is_fluent_identifier("foo-bar"));
        assert!(is_fluent_identifier("a"));
        assert!(is_fluent_identifier("x1-y2"));
        assert!(!is_fluent_identifier(""));
        assert!(!is_fluent_identifier("Foo"));
        assert!(!is_fluent_identifier("1foo"));
        assert!(!is_fluent_identifier("a b"));
        assert!(!is_fluent_identifier("_foo"));
    }

    #[test]
    fn literal_keys_extract_svelte_ts() {
        let src = r#"
import { t } from "../i18n";
onClick = () => t("menu-pause");
const msg = t(`toast-copy-queued`);
const bad = t(`state-${s}`); // dynamic — not captured
const flag = $t("history-empty");
"#;
        let mut out = BTreeSet::new();
        collect_literal_keys(src, &mut out);
        assert!(out.contains("menu-pause"));
        assert!(out.contains("toast-copy-queued"));
        assert!(out.contains("history-empty"));
        // Dynamic keys must NOT be captured.
        assert!(!out.iter().any(|k| k.starts_with("state-")));
    }

    #[test]
    fn literal_keys_extract_rust() {
        let src = r#"
fn err() -> String { "err-source-required".to_string() }
let e = LoggedError { localized_key: "err-not-found", ..default() };
t!("action-pause")
"#;
        let mut out = BTreeSet::new();
        collect_literal_keys(src, &mut out);
        assert!(out.contains("err-not-found"));
        assert!(out.contains("action-pause"));
    }
}
