// Humanise byte counts, bit rates, and durations.
//
// Deliberately locale-agnostic: ICU-backed formatting lands in
// Phase 11. These helpers emit ASCII digits with a thousands-separator
// coming from `Intl.NumberFormat(locale)` so at least the grouping
// adapts.

const BINARY_UNITS = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"] as const;

export function formatBytes(bytes: number, locale = "en"): string {
  if (!Number.isFinite(bytes) || bytes < 0) return "—";
  if (bytes === 0) return `0 ${BINARY_UNITS[0]}`;
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < BINARY_UNITS.length - 1) {
    value /= 1024;
    unit += 1;
  }
  const fractionDigits = unit === 0 ? 0 : value >= 100 ? 0 : value >= 10 ? 1 : 2;
  return `${formatNumber(value, locale, fractionDigits)} ${BINARY_UNITS[unit]}`;
}

export function formatRate(bytesPerSecond: number, locale = "en"): string {
  if (!Number.isFinite(bytesPerSecond) || bytesPerSecond <= 0) return "—";
  return `${formatBytes(bytesPerSecond, locale)}/s`;
}

export function formatNumber(
  value: number,
  locale: string,
  fractionDigits = 0,
): string {
  try {
    return new Intl.NumberFormat(locale, {
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
  locale = "en",
): string {
  if (total <= 0) return "—";
  const ratio = Math.min(1, Math.max(0, done / total));
  const pct = ratio * 100;
  const fractionDigits = pct < 10 ? 1 : 0;
  return `${formatNumber(pct, locale, fractionDigits)}%`;
}

export function formatEta(
  seconds: number | null | undefined,
  t: (key: string) => string,
  locale = "en",
): string {
  if (seconds === null || seconds === undefined) {
    return t("eta-calculating");
  }
  if (!Number.isFinite(seconds)) {
    return t("eta-unknown");
  }
  if (seconds < 1) return `< 1s`;
  if (seconds < 60) return `${Math.round(seconds)}s`;
  if (seconds < 3600) {
    const m = Math.floor(seconds / 60);
    const s = Math.round(seconds % 60);
    return `${m}m ${s}s`;
  }
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return `${h}h ${m}m`;
  // Locale param reserved for Phase 11's ICU relative-time formatter.
  void locale;
}

export function progressRatio(done: number, total: number): number {
  if (total <= 0) return 0;
  return Math.min(1, Math.max(0, done / total));
}
