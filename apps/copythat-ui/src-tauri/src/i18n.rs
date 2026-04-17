//! Fluent-lite translations for the Tauri shell.
//!
//! All 18 `copythat.ftl` files are `include_str!`'d at compile time
//! so the packaged binary is self-contained. The parser is the same
//! "first `key = value` token per line, skip continuations and
//! comments" shape used by `xtask i18n-lint`; Phase 11 swaps the
//! body for real `fluent-rs` so placeables, plural rules, and
//! attributes start working.

use std::collections::HashMap;

const LOCALES: &[(&str, &str)] = &[
    ("en", include_str!("../../../../locales/en/copythat.ftl")),
    ("es", include_str!("../../../../locales/es/copythat.ftl")),
    (
        "zh-CN",
        include_str!("../../../../locales/zh-CN/copythat.ftl"),
    ),
    ("hi", include_str!("../../../../locales/hi/copythat.ftl")),
    ("ar", include_str!("../../../../locales/ar/copythat.ftl")),
    (
        "pt-BR",
        include_str!("../../../../locales/pt-BR/copythat.ftl"),
    ),
    ("ru", include_str!("../../../../locales/ru/copythat.ftl")),
    ("ja", include_str!("../../../../locales/ja/copythat.ftl")),
    ("de", include_str!("../../../../locales/de/copythat.ftl")),
    ("fr", include_str!("../../../../locales/fr/copythat.ftl")),
    ("ko", include_str!("../../../../locales/ko/copythat.ftl")),
    ("it", include_str!("../../../../locales/it/copythat.ftl")),
    ("tr", include_str!("../../../../locales/tr/copythat.ftl")),
    ("vi", include_str!("../../../../locales/vi/copythat.ftl")),
    ("pl", include_str!("../../../../locales/pl/copythat.ftl")),
    ("nl", include_str!("../../../../locales/nl/copythat.ftl")),
    ("id", include_str!("../../../../locales/id/copythat.ftl")),
    ("uk", include_str!("../../../../locales/uk/copythat.ftl")),
];

const FALLBACK_LOCALE: &str = "en";

pub fn translations(locale: &str) -> HashMap<String, String> {
    let content = lookup(locale).unwrap_or_else(|| {
        lookup(FALLBACK_LOCALE).expect("built-in `en` locale is always present")
    });
    parse(content)
}

pub fn available_locales() -> Vec<String> {
    LOCALES.iter().map(|(c, _)| (*c).to_string()).collect()
}

/// Read the OS's current locale preference. Tauri 2.x exposes this
/// via `sys_locale` transitively; since we want zero extra deps in
/// Phase 5 we use the `LANG` / `LC_ALL` / `USERLANGUAGE` env vars,
/// falling back to `en` when nothing is set.
pub fn system_locale() -> String {
    fn from_env(var: &str) -> Option<String> {
        std::env::var(var).ok().and_then(|raw| {
            let trimmed = raw.trim();
            if trimmed.is_empty() || trimmed == "C" || trimmed == "POSIX" {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
    }
    // Priority order matches POSIX: LC_ALL > LC_MESSAGES > LANG.
    // Windows exposes UI language via `LANG` in modern WSL / MSYS
    // bashes; the webview's `navigator.language` is the authoritative
    // source anyway, and the frontend uses that first.
    for var in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Some(raw) = from_env(var) {
            return normalise(&raw);
        }
    }
    FALLBACK_LOCALE.to_string()
}

/// Normalise a locale string like `es_MX.UTF-8` → `es`. Matches the
/// directory names we ship under `locales/`, falling back to the
/// language code when we don't have an exact match.
fn normalise(raw: &str) -> String {
    // Strip encoding suffix (`es_MX.UTF-8` → `es_MX`).
    let without_encoding = raw.split('.').next().unwrap_or(raw);
    // Convert POSIX underscore to BCP-47 dash (`es_MX` → `es-MX`).
    let bcp47 = without_encoding.replace('_', "-");

    // Exact match (case-sensitive for BCP-47 region; the shipped
    // directory names are `pt-BR` and `zh-CN`).
    if LOCALES.iter().any(|(c, _)| *c == bcp47) {
        return bcp47;
    }
    // Case-insensitive region match (`pt-br` → `pt-BR`).
    for (code, _) in LOCALES {
        if code.eq_ignore_ascii_case(&bcp47) {
            return (*code).to_string();
        }
    }
    // Language-only fallback: `es-MX` → `es`.
    if let Some((language, _)) = bcp47.split_once('-')
        && LOCALES.iter().any(|(c, _)| *c == language)
    {
        return language.to_string();
    }
    FALLBACK_LOCALE.to_string()
}

fn lookup(code: &str) -> Option<&'static str> {
    LOCALES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, content)| *content)
}

/// Minimal Fluent parser: collects `key = value` lines.
///
/// - Lines starting with whitespace, `.`, `*`, `[`, `}`, or `#` are
///   skipped (continuations / attributes / variants / comments).
/// - Terms (`-name = ...`) are emitted with the leading `-`.
/// - Values are captured verbatim, including any trailing Fluent
///   continuation syntax — Phase 11 replaces this with real
///   `fluent-rs` parsing.
fn parse(content: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_locale_has_app_name() {
        for (code, _) in LOCALES {
            let t = translations(code);
            assert!(t.contains_key("app-name"), "{code} missing app-name");
        }
    }

    #[test]
    fn unknown_locale_falls_back_to_en() {
        let t = translations("klingon");
        assert!(t.contains_key("app-name"));
    }

    #[test]
    fn normalise_maps_posix_to_bcp47() {
        assert_eq!(normalise("es_MX.UTF-8"), "es");
        assert_eq!(normalise("pt_BR.UTF-8"), "pt-BR");
        assert_eq!(normalise("zh_CN"), "zh-CN");
        assert_eq!(normalise("fr-FR"), "fr");
        assert_eq!(normalise("de"), "de");
    }

    #[test]
    fn normalise_unknown_falls_back() {
        assert_eq!(normalise("kl-KLI"), FALLBACK_LOCALE);
    }
}
