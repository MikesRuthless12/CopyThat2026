// Phase 38 — minimal i18n for the PWA. Mirrors the desktop's
// 18-locale set; the PWA queries the desktop's selected locale via
// `RemoteCommand::GetLocale` on connect and falls back to
// `navigator.language` when the desktop replies with an empty
// string (auto-detect).
//
// Translations are inline JSON objects so the PWA doesn't have to
// fetch additional bundles at runtime. Strings are intentionally
// minimal — only what the PWA UI actually shows. The desktop's
// `locales/<code>/copythat.ftl` carries the full localization set.

import { derived, get, writable, type Readable, type Writable } from "svelte/store";

export type Locale =
  | "en"
  | "es"
  | "zh-CN"
  | "hi"
  | "ar"
  | "pt-BR"
  | "ru"
  | "ja"
  | "de"
  | "fr"
  | "ko"
  | "it"
  | "tr"
  | "vi"
  | "pl"
  | "nl"
  | "id"
  | "uk";

export const SUPPORTED_LOCALES: readonly Locale[] = [
  "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr",
  "ko", "it", "tr", "vi", "pl", "nl", "id", "uk",
] as const;

type Bundle = Record<string, string>;

const BUNDLES: Record<Locale, Bundle> = {
  en: {
    "title": "Copy That",
    "header.exit": "Exit",
    "pair.heading": "Pair with desktop",
    "pair.continue": "Continue",
    "pair.connect": "Connect",
    "pair.cancel": "Cancel",
    "unreachable.heading": "Desktop unreachable",
    "unreachable.retry": "Retry connection",
    "dashboard.live-progress": "Live progress",
    "dashboard.bytes": "Bytes",
    "dashboard.files": "Files",
    "dashboard.rate": "Rate",
    "dashboard.copied": "Copied",
    "dashboard.moved": "Moved",
    "dashboard.securely-deleted": "Securely deleted",
    "dashboard.active-jobs": "Active jobs",
    "dashboard.no-jobs": "Nothing running right now.",
    "dashboard.pause": "Pause",
    "dashboard.resume": "Resume",
    "dashboard.cancel": "Cancel",
    "dashboard.files-being-processed": "Files being processed",
    "dashboard.recent-history": "Recent history",
    "dashboard.no-history": "No recent jobs.",
    "dashboard.rerun": "Re-run",
    "dashboard.keep-awake": "Keep desktop awake while paired",
    "dashboard.loading": "Desktop is loading files",
  },
  // Translations follow the desktop's MT-flagged convention —
  // English placeholders today, human-translated in a follow-up
  // tracked under docs/I18N_TODO.md.
  es: {} as Bundle,
  "zh-CN": {} as Bundle,
  hi: {} as Bundle,
  ar: {} as Bundle,
  "pt-BR": {} as Bundle,
  ru: {} as Bundle,
  ja: {} as Bundle,
  de: {} as Bundle,
  fr: {} as Bundle,
  ko: {} as Bundle,
  it: {} as Bundle,
  tr: {} as Bundle,
  vi: {} as Bundle,
  pl: {} as Bundle,
  nl: {} as Bundle,
  id: {} as Bundle,
  uk: {} as Bundle,
};

const localeStore: Writable<Locale> = writable(detectBrowserLocale());
export const locale: Readable<Locale> = { subscribe: localeStore.subscribe };

/// Apply the desktop-supplied BCP-47 tag if it's one of our
/// supported locales; otherwise fall back to the browser default.
export function applyDesktopLocale(bcp47: string): void {
  if (!bcp47) return; // Empty = auto-detect; keep browser default.
  if ((SUPPORTED_LOCALES as readonly string[]).includes(bcp47)) {
    localeStore.set(bcp47 as Locale);
    return;
  }
  // Best-effort prefix match — `en-US` falls back to `en`.
  const prefix = bcp47.split("-")[0];
  const match = (SUPPORTED_LOCALES as readonly string[]).find(
    (l) => l === prefix || l.split("-")[0] === prefix,
  );
  if (match) {
    localeStore.set(match as Locale);
  }
}

function detectBrowserLocale(): Locale {
  if (typeof navigator === "undefined") return "en";
  const tag = navigator.language || "en";
  if ((SUPPORTED_LOCALES as readonly string[]).includes(tag)) {
    return tag as Locale;
  }
  const prefix = tag.split("-")[0];
  const match = (SUPPORTED_LOCALES as readonly string[]).find(
    (l) => l === prefix || l.split("-")[0] === prefix,
  );
  return (match as Locale) ?? "en";
}

type TranslateKey = keyof typeof BUNDLES.en;

function lookup(active: Locale, key: TranslateKey): string {
  const bundle = BUNDLES[active];
  return bundle[key] ?? BUNDLES.en[key] ?? key;
}

/// Reactive translator store. Components subscribe with
/// `$translator("dashboard.bytes")` and Svelte re-runs the
/// expression whenever `localeStore` changes — so flipping the
/// locale at runtime (via `applyDesktopLocale`) updates every
/// translated string in place.
export const translator: Readable<(key: TranslateKey) => string> = derived(
  localeStore,
  (active) => (key: TranslateKey) => lookup(active, key),
);

/// One-shot translation lookup. Reads the current locale via
/// `get(localeStore)` (no subscribe-then-unsubscribe dance); the
/// previous shape leaked the closed-over `active` variable as a
/// stale snapshot that never updated when the locale changed.
/// Components that need reactive re-rendering should use
/// `$translator(key)` instead. `t()` is fine for non-reactive
/// callers (one-time toast text, console diagnostics, etc.).
export function t(key: TranslateKey): string {
  return lookup(get(localeStore), key);
}
