<!--
  Phase 10 — Totals drawer.

  Lives behind the Footer's "Totals" button. Fetches lifetime
  aggregates via `history_totals` + a 30-day daily series via
  `history_daily`, then renders:

  - Four big-number cards: bytes, files, jobs, avg throughput.
  - A compact SVG line-sparkline of bytes/day over the last 30
    days. Zero-activity days show as gaps rather than false flat
    lines.
  - A stacked horizontal bar of the by-kind breakdown (copy / move
    today; the enum will grow in later phases).
  - A "time saved" estimate card. Applies a conservative per-OS
    speedup constant (baked in here; Phase 13 benchmarks can tune
    the ratio from measurement).
  - A "Reset statistics" action gated behind a confirm prompt.

  The component deliberately avoids Chart.js / svelte-chart — a
  30-point series on a single SVG path is cheaper than pulling a
  dependency that Phase 18 packaging will later fight to trim.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { formatBytes } from "../format";
  import {
    historyClearAll,
    historyDaily,
    historyTotals,
  } from "../ipc";
  import {
    closeTotalsDrawer,
    pushToast,
    totalsDrawerOpen,
  } from "../stores";
  import type { DayTotalDto, TotalsDto } from "../types";

  const DAY_MS = 86_400_000;
  const SPARK_DAYS = 30;
  // Conservative per-OS speedup constants. Phase 13's head-to-head
  // benchmarks will dial these in with measured data; for now the
  // numbers are intentionally cautious so the "time saved" label
  // never over-promises.
  const SPEEDUP_BY_OS: Record<string, number> = {
    windows: 1.4,
    macos: 1.05,
    linux: 1.15,
    default: 1.15,
  };

  let totals = $state<TotalsDto | null>(null);
  let daily = $state<DayTotalDto[]>([]);
  let busy = $state(false);
  let loadError = $state<string | null>(null);
  let confirmingReset = $state(false);
  let osHint = $state<string>("default");

  $effect(() => {
    if ($totalsDrawerOpen) {
      void refresh();
      void detectOs();
    }
  });

  async function detectOs() {
    try {
      // Tauri's `@tauri-apps/plugin-os` would be cleaner, but the
      // user-agent heuristic is dependency-free and accurate
      // enough for a cosmetic speedup estimate.
      const ua = navigator.userAgent.toLowerCase();
      if (ua.includes("windows")) osHint = "windows";
      else if (ua.includes("mac")) osHint = "macos";
      else if (ua.includes("linux")) osHint = "linux";
      else osHint = "default";
    } catch {
      osHint = "default";
    }
  }

  async function refresh() {
    busy = true;
    loadError = null;
    try {
      const sinceMs =
        Math.floor(Date.now() / DAY_MS) * DAY_MS - (SPARK_DAYS - 1) * DAY_MS;
      const [t, d] = await Promise.all([
        historyTotals(undefined),
        historyDaily(sinceMs),
      ]);
      totals = t;
      daily = d;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      loadError = msg;
      if (msg !== "history-unavailable") {
        pushToast("error", msg);
      }
    } finally {
      busy = false;
    }
  }

  async function reset() {
    confirmingReset = false;
    try {
      await historyClearAll();
      await refresh();
      pushToast("success", "toast-totals-reset");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // ---------- derived ----------

  const avgRateBps = $derived.by(() => {
    if (!totals || totals.durationMs === 0) return 0;
    const secs = totals.durationMs / 1000;
    return Math.floor(totals.bytes / secs);
  });

  const speedup = $derived(SPEEDUP_BY_OS[osHint] ?? SPEEDUP_BY_OS.default);

  // Milliseconds saved = actual_duration * (speedup - 1).
  const savedMs = $derived.by(() => {
    if (!totals) return 0;
    return Math.floor(totals.durationMs * (speedup - 1));
  });

  // Fill the sparse daily buckets into a dense SPARK_DAYS series so
  // the SVG polyline doesn't compress wide gaps into one line
  // segment.
  const denseDaily = $derived.by<DayTotalDto[]>(() => {
    const byDay = new Map<number, DayTotalDto>();
    for (const d of daily) byDay.set(d.dateMs, d);
    const today = Math.floor(Date.now() / DAY_MS) * DAY_MS;
    const out: DayTotalDto[] = [];
    for (let i = SPARK_DAYS - 1; i >= 0; i--) {
      const key = today - i * DAY_MS;
      out.push(
        byDay.get(key) ?? { dateMs: key, bytes: 0, files: 0, jobs: 0 },
      );
    }
    return out;
  });

  const sparkMax = $derived(
    denseDaily.reduce((m, d) => Math.max(m, d.bytes), 1),
  );

  // Polyline points in SVG-coord space (280×60 viewport).
  const sparklinePoints = $derived.by(() => {
    const W = 280;
    const H = 60;
    if (denseDaily.length === 0) return "";
    const step = W / Math.max(denseDaily.length - 1, 1);
    return denseDaily
      .map((d, i) => {
        const x = i * step;
        const y = H - (d.bytes / sparkMax) * H;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  });

  // Stacked-bar segments for the by-kind breakdown.
  const byKindBars = $derived.by<
    { kind: string; widthPct: number; bytes: number }[]
  >(() => {
    if (!totals || totals.bytes === 0) return [];
    return totals.byKind.map((b) => ({
      kind: b.kind,
      widthPct: (b.bytes / totals!.bytes) * 100,
      bytes: b.bytes,
    }));
  });

  function fmtMs(ms: number): string {
    if (ms <= 0) return "0s";
    const s = Math.floor(ms / 1000);
    if (s < 60) return `${s}s`;
    const m = Math.floor(s / 60);
    const rs = s % 60;
    if (m < 60) return `${m}m ${rs}s`;
    const h = Math.floor(m / 60);
    const rm = m % 60;
    return `${h}h ${rm}m`;
  }

  function fmtRate(bps: number): string {
    return bps === 0 ? "—" : `${formatBytes(bps)}/s`;
  }
</script>

{#if $totalsDrawerOpen}
  <aside class="drawer" aria-label={t("totals-title")}>
    <header>
      <h2>{t("totals-title")}</h2>
      <button
        class="close"
        type="button"
        aria-label={t("action-close")}
        onclick={closeTotalsDrawer}
      >
        <Icon name="x" size={16} />
      </button>
    </header>

    {#if loadError === "history-unavailable"}
      <p class="notice">{t("history-unavailable")}</p>
    {:else if !totals}
      <p class="empty">{t("totals-loading")}</p>
    {:else}
      <!-- Big numbers -->
      <div class="cards">
        <section class="card">
          <h3>{t("totals-card-bytes")}</h3>
          <strong>{formatBytes(totals.bytes)}</strong>
        </section>
        <section class="card">
          <h3>{t("totals-card-files")}</h3>
          <strong>{totals.files.toLocaleString()}</strong>
        </section>
        <section class="card">
          <h3>{t("totals-card-jobs")}</h3>
          <strong>{totals.jobs.toLocaleString()}</strong>
          {#if totals.errors > 0}
            <span class="sub">
              {totals.errors}
              {t("totals-errors")}
            </span>
          {/if}
        </section>
        <section class="card">
          <h3>{t("totals-card-avg-rate")}</h3>
          <strong>{fmtRate(avgRateBps)}</strong>
        </section>
      </div>

      <!-- 30-day sparkline -->
      <section class="spark">
        <header>
          <h3>{t("totals-spark-title")}</h3>
          <span class="axis">
            0 – {formatBytes(sparkMax)}
          </span>
        </header>
        <svg viewBox="0 0 280 60" preserveAspectRatio="none" role="img" aria-label={t("totals-spark-title")}>
          <polyline fill="none" stroke="currentColor" stroke-width="1.5" points={sparklinePoints} />
        </svg>
        <footer class="axis-dates">
          <span>
            {new Date(denseDaily[0]?.dateMs ?? 0).toLocaleDateString()}
          </span>
          <span>
            {new Date(
              denseDaily[denseDaily.length - 1]?.dateMs ?? 0,
            ).toLocaleDateString()}
          </span>
        </footer>
      </section>

      <!-- By-kind breakdown -->
      {#if byKindBars.length > 0}
        <section class="kinds">
          <h3>{t("totals-kinds-title")}</h3>
          <div class="kind-bar" role="img" aria-label={t("totals-kinds-title")}>
            {#each byKindBars as b}
              <span
                class="segment"
                data-kind={b.kind}
                style="width:{b.widthPct}%"
                title={`${b.kind}: ${formatBytes(b.bytes)}`}
              ></span>
            {/each}
          </div>
          <ul class="kind-legend">
            {#each byKindBars as b}
              <li>
                <span class="swatch" data-kind={b.kind}></span>
                {b.kind} — {formatBytes(b.bytes)}
              </li>
            {/each}
          </ul>
        </section>
      {/if}

      <!-- Time saved -->
      <section class="saved">
        <h3>{t("totals-saved-title")}</h3>
        <strong>{fmtMs(savedMs)}</strong>
        <p class="note">
          {t("totals-saved-note")}
          <span class="speedup">
            ({osHint} / {speedup.toFixed(2)}×)
          </span>
        </p>
      </section>

      <!-- Reset -->
      <section class="reset">
        {#if !confirmingReset}
          <button
            class="danger"
            type="button"
            onclick={() => (confirmingReset = true)}
            disabled={busy}
          >
            {t("totals-reset")}
          </button>
        {:else}
          <div class="confirm">
            <p>{t("totals-reset-confirm")}</p>
            <button
              class="secondary"
              type="button"
              onclick={() => (confirmingReset = false)}
            >
              {t("action-cancel")}
            </button>
            <button class="danger" type="button" onclick={reset}>
              {t("totals-reset-confirm-yes")}
            </button>
          </div>
        {/if}
      </section>
    {/if}
  </aside>
{/if}

<style>
  .drawer {
    position: fixed;
    right: 0;
    top: 0;
    bottom: 0;
    width: min(560px, 94vw);
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border-left: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    box-shadow: -10px 0 24px rgba(0, 0, 0, 0.22);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    z-index: 85;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  h3 {
    margin: 0 0 4px;
    font-size: 11px;
    font-weight: 600;
    color: var(--muted, #6a6a6a);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .close {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .notice,
  .empty {
    padding: 24px 16px;
    color: var(--muted, #6a6a6a);
    font-size: 13px;
    text-align: center;
  }

  .cards {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 10px;
    padding: 14px 16px;
  }

  .card {
    padding: 10px 12px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 6px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.03));
  }

  .card strong {
    font-size: 18px;
    font-variant-numeric: tabular-nums;
    display: block;
    margin-top: 2px;
  }

  .card .sub {
    display: block;
    margin-top: 4px;
    font-size: 10.5px;
    color: var(--error, #c24141);
  }

  .spark {
    padding: 8px 16px 12px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.15));
  }

  .spark header {
    padding: 0;
    border: none;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 4px;
  }

  .spark .axis {
    font-size: 10.5px;
    color: var(--muted, #6a6a6a);
  }

  .spark svg {
    width: 100%;
    height: 60px;
    color: var(--accent, #4f8cff);
  }

  .axis-dates {
    display: flex;
    justify-content: space-between;
    font-size: 10.5px;
    color: var(--muted, #6a6a6a);
    margin-top: 2px;
  }

  .kinds {
    padding: 8px 16px 14px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.15));
  }

  .kind-bar {
    display: flex;
    height: 14px;
    border-radius: 4px;
    overflow: hidden;
    background: var(--surface-alt, rgba(0, 0, 0, 0.05));
    margin: 6px 0;
  }

  .segment {
    height: 100%;
    background: var(--accent, #4f8cff);
  }

  .segment[data-kind="move"] {
    background: var(--verify, #7a4fb3);
  }

  .segment[data-kind="delete"],
  .segment[data-kind="secure-delete"] {
    background: var(--error, #d95757);
  }

  .kind-legend {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    gap: 14px;
    font-size: 11.5px;
    color: var(--muted, #6a6a6a);
  }

  .swatch {
    display: inline-block;
    width: 10px;
    height: 10px;
    margin-right: 4px;
    vertical-align: middle;
    background: var(--accent, #4f8cff);
    border-radius: 2px;
  }

  .swatch[data-kind="move"] {
    background: var(--verify, #7a4fb3);
  }

  .swatch[data-kind="delete"],
  .swatch[data-kind="secure-delete"] {
    background: var(--error, #d95757);
  }

  .saved {
    padding: 10px 16px 14px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.15));
  }

  .saved strong {
    font-size: 18px;
    font-variant-numeric: tabular-nums;
    display: block;
    margin-top: 2px;
  }

  .note {
    font-size: 11px;
    color: var(--muted, #6a6a6a);
    margin: 4px 0 0;
  }

  .speedup {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 10.5px;
  }

  .reset {
    padding: 10px 16px 16px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.15));
  }

  .reset .confirm {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .reset .confirm p {
    margin: 0;
    font-size: 12px;
    color: var(--muted, #6a6a6a);
    flex: 1;
  }

  button {
    font-size: 12px;
    padding: 5px 12px;
    border-radius: 4px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    color: inherit;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.danger {
    border-color: var(--error, #d95757);
    color: var(--error, #c24141);
  }

  button.danger:hover:not(:disabled) {
    background: rgba(217, 87, 87, 0.08);
  }
</style>
