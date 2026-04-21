// Humanise byte counts, rates, percentages, and durations.
//
// Phase 11a wires these helpers to the active locale via `currentLocale()`
// (pulled from the Fluent store) so the thousands separator, decimal
// separator, and percent character all adapt automatically. The binary
// unit *symbols* (KiB, MiB, GiB, ...) stay fixed — they are IEC 80000-13
// standard and deliberately not localized. Duration words like "min",
// "h", "ms" are routed through Fluent so translators can pick the
// per-language abbreviation.
//
// All helpers accept an optional explicit `locale` override for places
// where the active locale is not reachable (pure-logic unit tests,
// legacy call sites). When omitted, they pull the current Fluent
// locale from the store — a zero-allocation `get()` read, not a
// subscription.
//
// Binary units follow IEC 80000-13 (`1 KiB = 1024 B`), which both
// `formatBytes` and `formatRate` reuse; `PiB` is the largest bucket
// any one file or one day of throughput is ever going to need in 2026.

import { get } from "svelte/store";

import { locale } from "./i18n";

const BINARY_UNITS = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"] as const;

/// The locale-parameter default is `undefined` (pull from the i18n
/// store). Callers can pass an explicit string (e.g. `"en"`) to pin
/// the output — handy for determinism in tests.
function currentLocale(): string {
  try {
    return get(locale).code;
  } catch {
    return "en";
  }
}

export function formatBytes(bytes: number, loc?: string): string {
  if (!Number.isFinite(bytes) || bytes < 0) return "—";
  if (bytes === 0) return `0 ${BINARY_UNITS[0]}`;
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < BINARY_UNITS.length - 1) {
    value /= 1024;
    unit += 1;
  }
  const fractionDigits = unit === 0 ? 0 : value >= 100 ? 0 : value >= 10 ? 1 : 2;
  return `${formatNumber(value, loc ?? currentLocale(), fractionDigits)} ${BINARY_UNITS[unit]}`;
}

export function formatRate(bytesPerSecond: number, loc?: string): string {
  if (!Number.isFinite(bytesPerSecond) || bytesPerSecond <= 0) return "—";
  return `${formatBytes(bytesPerSecond, loc)}/s`;
}

export function formatNumber(
  value: number,
  loc: string,
  fractionDigits = 0,
): string {
  try {
    return new Intl.NumberFormat(loc, {
      maximumFractionDigits: fractionDigits,
      minimumFractionDigits: fractionDigits,
    }).format(value);
  } catch {
    return value.toFixed(fractionDigits);
  }
}

export function formatPercent(
  done: number,
  total: number,
  loc?: string,
): string {
  if (total <= 0) return "—";
  const ratio = Math.min(1, Math.max(0, done / total));
  const pct = ratio * 100;
  const fractionDigits = pct < 10 ? 1 : 0;
  return `${formatNumber(pct, loc ?? currentLocale(), fractionDigits)}%`;
}

/// Two-decimal percentage for the main progress readout. Always
/// renders with exactly two decimals ("38.95%", "0.00%", "100.00%")
/// so the text doesn't visually jitter as digits roll over.
export function formatPercent2(
  done: number,
  total: number,
  loc?: string,
): string {
  if (total <= 0) return "—";
  const ratio = Math.min(1, Math.max(0, done / total));
  return `${formatNumber(ratio * 100, loc ?? currentLocale(), 2)}%`;
}

/// Compact ETA in `3h 10m 38s` shape. Deliberately does NOT route
/// through Fluent — the requirement is a fixed visual form for every
/// locale. Leading zero components are dropped: `0h 4m 12s` renders
/// as `4m 12s`; sub-second renders as `<1s`.
export function formatEtaVerbose(
  seconds: number | null | undefined,
): string {
  if (seconds === null || seconds === undefined) return "calculating…";
  if (!Number.isFinite(seconds)) return "—";
  if (seconds < 0) return "—";
  if (seconds < 1) return "<1s";
  const total = Math.round(seconds);
  const hours = Math.floor(total / 3600);
  const mins = Math.floor((total % 3600) / 60);
  const secs = total % 60;
  const parts: string[] = [];
  if (hours > 0) parts.push(`${hours}h`);
  if (mins > 0 || hours > 0) parts.push(`${mins}m`);
  parts.push(`${secs}s`);
  return parts.join(" ");
}

/// Running average bytes/second since the job started. Complements
/// `rateBps` (the instantaneous sample) — the average is much less
/// twitchy for user-visible UI and is what most users mentally
/// mean by "how fast is it going?"
export function averageRateBps(
  bytesDone: number,
  startedAtMs: number | null | undefined,
  nowMs: number = Date.now(),
): number {
  if (!startedAtMs || bytesDone <= 0) return 0;
  const elapsedSec = (nowMs - startedAtMs) / 1000;
  if (elapsedSec <= 0) return 0;
  return bytesDone / elapsedSec;
}

/// Middle-ellipsis truncation for path displays. Keeps the drive /
/// prefix and the basename visible — `C:\Users\mike\...\file.bin` —
/// so the user can still tell *what* is being copied even when the
/// row doesn't have room for the whole path.
export function truncatePath(path: string, maxLen: number = 40): string {
  if (!path) return "";
  if (path.length <= maxLen) return path;
  const keepFront = Math.max(3, Math.floor((maxLen - 1) / 2));
  const keepBack = Math.max(3, maxLen - 1 - keepFront);
  return `${path.slice(0, keepFront)}…${path.slice(path.length - keepBack)}`;
}

/// Translator-facing ETA renderer.
///
/// Routes seconds → minutes → hours buckets through the `duration-*`
/// Fluent keys so an RTL or Asian locale can substitute its own unit
/// abbreviations (e.g. Japanese "分" / "時間", Chinese "分" / "小时").
/// `t` is passed in so this module stays dependency-free at its call
/// sites.
export function formatEta(
  seconds: number | null | undefined,
  t: (key: string, args?: Record<string, string | number>) => string,
): string {
  if (seconds === null || seconds === undefined) {
    return t("eta-calculating");
  }
  if (!Number.isFinite(seconds)) {
    return t("eta-unknown");
  }
  if (seconds < 1) return t("duration-lt-1s");
  if (seconds < 60) {
    return t("duration-seconds", { s: Math.round(seconds) });
  }
  if (seconds < 3600) {
    const m = Math.floor(seconds / 60);
    const s = Math.round(seconds % 60);
    return t("duration-minutes-seconds", { m, s });
  }
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return t("duration-hours-minutes", { h, m });
}

export function progressRatio(done: number, total: number): number {
  if (total <= 0) return 0;
  return Math.min(1, Math.max(0, done / total));
}
