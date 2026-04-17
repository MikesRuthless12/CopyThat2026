//! `xtask` — workspace automation.
//!
//! Subcommands:
//! - `i18n-lint`: verify Fluent key parity across every `locales/<code>/copythat.ftl`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

const SOURCE_LOCALE: &str = "en";

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
        "Usage: xtask <command>\n\nCommands:\n  i18n-lint   Verify key parity across all 18 locales/<code>/copythat.ftl files\n"
    );
}

fn i18n_lint() -> Result<(), String> {
    let root = repo_root().ok_or("could not locate repo root (Cargo.toml + locales/)")?;
    let locales_dir = root.join("locales");

    let mut per_locale: Vec<(String, BTreeSet<String>)> = Vec::with_capacity(LOCALES.len());
    for code in LOCALES {
        let path = locales_dir.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        per_locale.push(((*code).to_string(), parse_ftl_keys(&content)));
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

    println!(
        "i18n-lint: OK ({} locales, {} keys each)",
        per_locale.len(),
        reference.len()
    );
    Ok(())
}

/// Minimal Fluent parser: collect top-level message and term identifiers.
///
/// Recognised:
/// - `key = value`              → message identifier `key`
/// - `-term = value`            → term identifier `-term`
///
/// Skipped (continuations / attributes / variants / comments / blanks):
/// - lines starting with whitespace, `.`, `*`, `[`, `}`, or `#`.
fn parse_ftl_keys(content: &str) -> BTreeSet<String> {
    content
        .lines()
        .filter_map(|raw| {
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
            // A Fluent identifier is [A-Za-z][A-Za-z0-9_-]*; terms prepend `-`.
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
        })
        .collect()
}

fn repo_root() -> Option<PathBuf> {
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
        let src = "app-name = Copy That 2026\n# a comment\nfoo-bar = hi\n";
        let keys = parse_ftl_keys(src);
        assert!(keys.contains("app-name"));
        assert!(keys.contains("foo-bar"));
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn skips_attributes_and_continuations() {
        let src = "msg = Hello\n    .title = T\n    continuation line\n";
        let keys = parse_ftl_keys(src);
        assert_eq!(keys.len(), 1);
        assert!(keys.contains("msg"));
    }

    #[test]
    fn collects_term() {
        let src = "-brand = Copy That\n";
        let keys = parse_ftl_keys(src);
        assert!(keys.contains("-brand"));
    }
}
