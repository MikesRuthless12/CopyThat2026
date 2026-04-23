<!--
  Fixed 56 px strip along the top. Carries the global state pill,
  throughput, ETA, and Pause-all / Resume-all / Cancel-all controls.

  Binds directly to the `globals` store so the rate is always the
  sum of live per-job rates (see `stores.ts::liveRateBps`).
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import StateBadge from "./StateBadge.svelte";
  import { i18nVersion, t } from "../i18n";
  import {
    globals,
    liveRateBps,
    openSettings,
    pushToast,
    setDropped,
  } from "../stores";

  // Bind `$locale.code` to the root's `data-locale` attribute below
  // so the template re-evaluates every `t(...)` call on language
  // switch or initial hydration.
  import {
    pauseAll,
    resumeAll,
    cancelAll,
    currentShapeRate,
    onEvent,
    pickFiles,
    pickFolders,
  } from "../ipc";
  import { formatEta, formatRate } from "../format";
  import type { ShapeRateDto } from "../types";

  let g = $derived($globals);
  let rate = $derived($liveRateBps);
  let hasWork = $derived(
    g.state !== "idle" && g.activeJobs + g.queuedJobs + g.pausedJobs > 0,
  );

  // Phase 21 — bandwidth shape badge. `null` rate = unlimited (no
  // badge); 0 = paused; positive = active cap. Initial value pulled
  // on mount; subsequent changes ride on the `shape-rate-changed`
  // event from the runner / Settings update / minute-tick poller.
  let shapeRate = $state<ShapeRateDto | null>(null);
  let badgeLabel = $derived.by(() => {
    if (!shapeRate || shapeRate.bytesPerSecond === null) return "";
    if (shapeRate.bytesPerSecond === 0) return t("shape-badge-paused");
    return formatRate(shapeRate.bytesPerSecond);
  });
  let badgeSourceLabel = $derived.by(() => {
    if (!shapeRate || shapeRate.bytesPerSecond === null) return "";
    switch (shapeRate.source) {
      case "schedule":
        return t("shape-badge-source-schedule");
      case "auto-metered":
        return t("shape-badge-source-metered");
      case "auto-battery":
        return t("shape-badge-source-battery");
      case "auto-cellular":
        return t("shape-badge-source-cellular");
      default:
        return t("shape-badge-source-settings");
    }
  });

  $effect(() => {
    let unlistenFn: (() => void) | undefined;
    void (async () => {
      try {
        shapeRate = await currentShapeRate();
      } catch {
        shapeRate = null;
      }
      try {
        unlistenFn = await onEvent<ShapeRateDto>(
          "shape-rate-changed",
          (payload) => {
            shapeRate = payload;
          },
        );
      } catch {
        // Listener registration is best-effort; the badge stays on the
        // last-known value if the event bus rejects us.
      }
    })();
    return () => {
      if (unlistenFn) unlistenFn();
    };
  });
  // Only show a live ETA when there's real work in flight. When the
  // app is idle (no queued / running / paused job) we hide the ETA
  // entirely — otherwise the header sits on a permanent
  // "calculating…" that confuses users into thinking the app is
  // doing something. `formatEta` itself already handles the inside-
  // a-copy transitions (null etaSeconds with state=running → the
  // "calculating" string while bytes < 1 %; live numbers after).
  let etaDisplay = $derived(hasWork ? formatEta(g.etaSeconds, t) : "");
  let rateDisplay = $derived(hasWork ? formatRate(rate) : "");

  async function onAddFiles() {
    try {
      const paths = await pickFiles();
      if (paths.length > 0) setDropped(paths);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onAddFolders() {
    try {
      const paths = await pickFolders();
      if (paths.length > 0) setDropped(paths);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }
</script>

<header class="header">
  {#key $i18nVersion}
    <div class="brand" aria-hidden="true" aria-label={t("window-title")}>
      <Icon name="rocket" size={22} />
    </div>
    <div class="summary">
      <StateBadge state={g.state} size="md" />
      <div class="metrics">
        {#if rateDisplay}
          <span class="metric rate">{rateDisplay}</span>
        {/if}
        {#if etaDisplay}
          <span class="metric eta" aria-label={t("header-eta-label")}>
            {etaDisplay}
          </span>
        {/if}
        {#if badgeLabel}
          <button
            type="button"
            class="shape-badge"
            title={t("shape-badge-tooltip")}
            aria-label={t("shape-badge-tooltip")}
            onclick={() => openSettings()}
          >
            <span class="shape-icon" aria-hidden="true">▼</span>
            <span class="shape-rate">{badgeLabel}</span>
            {#if badgeSourceLabel}
              <span class="shape-source">· {badgeSourceLabel}</span>
            {/if}
          </button>
        {/if}
      </div>
    </div>
    <div class="actions" role="toolbar" aria-label={t("header-toolbar-label")}>
      <button
        type="button"
        class="pill-btn"
        aria-label={t("action-add-files")}
        title={t("action-add-files")}
        onclick={onAddFiles}
      >
        <Icon name="file" size={16} />
        <span>{t("action-add-files")}</span>
      </button>
      <button
        type="button"
        class="pill-btn"
        aria-label={t("action-add-folders")}
        title={t("action-add-folders")}
        onclick={onAddFolders}
      >
        <Icon name="folder" size={16} />
        <span>{t("action-add-folders")}</span>
      </button>
      <span class="divider" aria-hidden="true"></span>
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
  {/key}
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

  .pill-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: var(--hover, rgba(128, 128, 128, 0.12));
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 6px;
    color: inherit;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .pill-btn:hover {
    background: var(--row-selected, rgba(79, 140, 255, 0.14));
    border-color: var(--accent, #4f8cff);
  }

  .pill-btn:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: 2px;
  }

  .divider {
    width: 1px;
    height: 20px;
    background: var(--border, rgba(128, 128, 128, 0.3));
    margin: 0 4px;
  }

  .shape-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    background: var(--row-selected, rgba(79, 140, 255, 0.14));
    border: 1px solid var(--accent, #4f8cff);
    border-radius: 999px;
    color: var(--fg, #1f1f1f);
    font: inherit;
    font-size: 12px;
    line-height: 1.2;
    cursor: pointer;
  }

  .shape-badge:hover {
    background: var(--accent, #4f8cff);
    color: #ffffff;
  }

  .shape-icon {
    font-size: 10px;
    color: var(--accent, #4f8cff);
  }

  .shape-badge:hover .shape-icon {
    color: #ffffff;
  }

  .shape-rate {
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .shape-source {
    color: var(--fg-dim, #5f5f5f);
  }

  .shape-badge:hover .shape-source {
    color: rgba(255, 255, 255, 0.85);
  }
</style>
