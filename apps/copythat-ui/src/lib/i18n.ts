// Fluent-lite runtime loader for the frontend.
//
// Phase 5 ships a minimal `t(key, args?)` that does `$variable`
// substitution against the translations returned by the Rust
// `translations(locale)` command. Phase 11 replaces this with the
// real `@fluent/bundle` + `@fluent/langneg` stack.

import { get, writable, type Readable } from "svelte/store";

import { availableLocales, systemLocale, translations } from "./ipc";

const FALLBACK_LOCALE = "en";

interface LocaleState {
  code: string;
  direction: "ltr" | "rtl";
  table: Record<string, string>;
  fallback: Record<string, string>;
  available: string[];
}

const initial: LocaleState = {
  code: FALLBACK_LOCALE,
  direction: "ltr",
  table: {},
  fallback: {},
  available: [FALLBACK_LOCALE],
};

const store = writable<LocaleState>(initial);

/// Monotonic counter bumped on every `initI18n` / `setLocale`. Bound
/// by components to a `data-locale` attribute so static `t(...)`
/// calls in their templates re-evaluate when the translation table
/// hydrates (on first load, `store.code` stays `"en"` but the table
/// goes from `{}` to populated — binding to `.code` would miss that
/// transition because Svelte compares by value).
const versionStore = writable<number>(0);

/// Read-only view used by components; `t()` reads directly from the
/// store under the hood so components can bind to it reactively by
/// subscribing or by re-invoking `t("key")` inside a reactive block.
export const locale: Readable<LocaleState> = { subscribe: store.subscribe };
export const i18nVersion: Readable<number> = {
  subscribe: versionStore.subscribe,
};

/// Determine and load the user's preferred locale. Order of sources:
///   1. `navigator.language` (webview-advertised system locale)
///   2. Rust-side `LC_ALL` / `LANG` (`system_locale` command)
///   3. `en` fallback
export async function initI18n(): Promise<LocaleState> {
  const available = await availableLocales();
  const preferred = await pickPreferred(available);
  const [table, fallback] = await Promise.all([
    translations(preferred),
    preferred === FALLBACK_LOCALE
      ? Promise.resolve<Record<string, string>>({})
      : translations(FALLBACK_LOCALE),
  ]);
  const state: LocaleState = {
    code: preferred,
    direction: preferred === "ar" ? "rtl" : "ltr",
    table,
    fallback,
    available,
  };
  store.set(state);
  versionStore.update((n) => n + 1);
  applyHtmlAttributes(state);
  return state;
}

export async function setLocale(code: string): Promise<void> {
  const current = get(store);
  if (!current.available.includes(code)) return;
  const [table, fallback] = await Promise.all([
    translations(code),
    code === FALLBACK_LOCALE
      ? Promise.resolve<Record<string, string>>({})
      : translations(FALLBACK_LOCALE),
  ]);
  const state: LocaleState = {
    code,
    direction: code === "ar" ? "rtl" : "ltr",
    table,
    fallback,
    available: current.available,
  };
  store.set(state);
  versionStore.update((n) => n + 1);
  applyHtmlAttributes(state);
}

/// Translate a key with optional `$variable` substitutions.
///
/// Missing keys return the key itself in curly braces (e.g.
/// `{missing-key}`) so a regression pops out visually in both dev
/// and prod; Phase 11's audit pass will replace this behaviour with
/// dev-only console warnings.
export function t(
  key: string,
  args?: Record<string, string | number>,
): string {
  const { table, fallback } = get(store);
  const template = table[key] ?? fallback[key];
  if (template === undefined) return `{${key}}`;
  return substitute(template, args);
}

function substitute(
  template: string,
  args?: Record<string, string | number>,
): string {
  if (!args) return template;
  return template.replace(/\{\s*\$([a-zA-Z_][\w-]*)\s*\}/g, (match, name) => {
    const value = args[name];
    return value === undefined ? match : String(value);
  });
}

function applyHtmlAttributes(state: LocaleState) {
  const html = document.documentElement;
  html.setAttribute("lang", state.code);
  html.setAttribute("dir", state.direction);
}

async function pickPreferred(available: string[]): Promise<string> {
  const candidates = navigatorCandidates();
  candidates.push(...(await rustCandidates()));
  candidates.push(FALLBACK_LOCALE);
  for (const raw of candidates) {
    const hit = match(raw, available);
    if (hit) return hit;
  }
  return FALLBACK_LOCALE;
}

function navigatorCandidates(): string[] {
  if (typeof navigator === "undefined") return [];
  const list: string[] = [];
  if (navigator.languages) list.push(...navigator.languages);
  if (navigator.language) list.push(navigator.language);
  return list;
}

async function rustCandidates(): Promise<string[]> {
  try {
    return [await systemLocale()];
  } catch {
    return [];
  }
}

/// Case-insensitive locale matcher. `pt-br` → `pt-BR`, `es-MX` → `es`.
function match(raw: string, available: string[]): string | null {
  if (!raw) return null;
  const normalised = raw.replace("_", "-");
  const exact = available.find((c) => c === normalised);
  if (exact) return exact;
  const caseInsensitive = available.find(
    (c) => c.toLowerCase() === normalised.toLowerCase(),
  );
  if (caseInsensitive) return caseInsensitive;
  const [language] = normalised.split("-");
  return available.find((c) => c === language) ?? null;
}
