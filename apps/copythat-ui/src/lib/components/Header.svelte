<!--
  Fixed 56 px strip along the top. Carries the global state pill,
  throughput, ETA, and Pause-all / Resume-all / Cancel-all controls.

  Binds directly to the `globals` store so the rate is always the
  sum of live per-job rates (see `stores.ts::liveRateBps`).
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import StateBadge from "./StateBadge.svelte";
  import { t } from "../i18n";
  import { globals, liveRateBps } from "../stores";
  import { pauseAll, resumeAll, cancelAll } from "../ipc";
  import { formatEta, formatRate } from "../format";

  let g = $derived($globals);
  let rate = $derived($liveRateBps);
  let etaDisplay = $derived(formatEta(g.etaSeconds, t));
  let hasWork = $derived(
    g.state !== "idle" && g.activeJobs + g.queuedJobs + g.pausedJobs > 0,
  );
</script>

<header class="header" aria-label={t("window-title")}>
  <div class="brand" aria-hidden="true">
    <Icon name="rocket" size={22} />
  </div>
  <div class="summary">
    <StateBadge state={g.state} size="md" />
    <div class="metrics">
      <span class="metric rate">{formatRate(rate)}</span>
      <span class="metric eta" aria-label={t("header-eta-label")}>
        {etaDisplay}
      </span>
    </div>
  </div>
  <div class="actions" role="toolbar" aria-label={t("header-toolbar-label")}>
    <button
      type="button"
      class="icon-btn"
      aria-label={t("action-pause-all")}
      title={t("action-pause-all")}
      disabled={!hasWork}
      onclick={() => pauseAll()}
    >
      <Icon name="pause" size={18} />
    </button>
    <button
      type="button"
      class="icon-btn"
      aria-label={t("action-resume-all")}
      title={t("action-resume-all")}
      disabled={g.pausedJobs === 0}
      onclick={() => resumeAll()}
    >
      <Icon name="play" size={18} />
    </button>
    <button
      type="button"
      class="icon-btn danger"
      aria-label={t("action-cancel-all")}
      title={t("action-cancel-all")}
      disabled={!hasWork}
      onclick={() => cancelAll()}
    >
      <Icon name="x" size={18} />
    </button>
  </div>
</header>

<style>
  .header {
    height: 56px;
    min-height: 56px;
    display: grid;
    grid-template-columns: 28px 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 0 14px;
    background: var(--header-bg, var(--surface, #fafafa));
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    box-sizing: border-box;
  }

  .brand {
    color: var(--accent, #4f8cff);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .summary {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .metrics {
    display: flex;
    align-items: baseline;
    gap: 10px;
    min-width: 0;
    font-variant-numeric: tabular-nums;
  }

  .metric {
    font-size: 13px;
    line-height: 1.2;
    color: var(--fg-strong, #1f1f1f);
  }

  .metric.rate {
    font-weight: 600;
  }

  .metric.eta {
    color: var(--fg-dim, #5f5f5f);
  }

  .actions {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--fg, #1f1f1f);
    cursor: pointer;
    padding: 0;
    transition:
      background 120ms,
      color 120ms,
      border-color 120ms;
  }

  .icon-btn:hover:not(:disabled) {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: 2px;
  }

  .icon-btn.danger:hover:not(:disabled) {
    color: var(--error, #c24141);
  }

  .icon-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }
</style>
