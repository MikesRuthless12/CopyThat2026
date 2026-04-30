<!--
  Phase 45.3 — named-queue tab strip.

  Renders one tab per `QueueSnapshotDto` from the Rust-side
  `QueueRegistry`, plus a synthesised "default" tab (id=0) covering
  jobs that flow through the legacy single-queue surface. Selecting a
  tab updates `selectedQueueIdStore`; the existing `JobList` reads
  `visibleJobs`, which is now filtered by that selection.

  The strip is hidden entirely when the registry holds zero queues —
  cold-launch UX is unchanged from pre-Phase-45 (one implicit queue,
  no chrome). It appears the moment the registry spawns its first
  queue (Phase 45.4+ runner reconciliation), at which point the
  default tab is needed to keep legacy-queue jobs reachable.

  i18n: `queue-tab-default`, `queue-tab-empty-state`,
  `queue-badge-tooltip`.
-->
<script lang="ts">
  import { i18nVersion, t } from "../i18n";
  import {
    jobs,
    queues,
    selectedQueueId,
    setSelectedQueue,
  } from "../stores";
  import type { JobDto, QueueSnapshotDto } from "../types";

  interface Tab {
    id: number;
    label: string;
    badge: number;
    running: boolean;
    isDefault: boolean;
  }

  function defaultBadge(allJobs: JobDto[]): number {
    let count = 0;
    for (const j of allJobs) {
      if ((j.queueId ?? 0) !== 0) continue;
      if (j.state === "pending" || j.state === "running") count += 1;
    }
    return count;
  }

  function defaultRunning(allJobs: JobDto[]): boolean {
    return allJobs.some(
      (j) => (j.queueId ?? 0) === 0 && j.state === "running",
    );
  }

  let tabs = $derived.by<Tab[]>(() => {
    const queueList = $queues;
    const jobList = $jobs;
    if (queueList.length === 0) return [];
    const out: Tab[] = [];
    // Synthesised default tab — covers legacy-queue jobs (queueId=0)
    // until Phase 45.4+ migrates them into registry queues. Always
    // first in the strip so muscle memory matches "the queue I've
    // always had".
    out.push({
      id: 0,
      label: t("queue-tab-default"),
      badge: defaultBadge(jobList),
      running: defaultRunning(jobList),
      isDefault: true,
    });
    for (const q of queueList as QueueSnapshotDto[]) {
      out.push({
        id: q.id,
        label: q.name,
        badge: q.badgeCount,
        running: q.running,
        isDefault: false,
      });
    }
    return out;
  });

  function onClick(id: number) {
    setSelectedQueue(id);
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
    e.preventDefault();
    const list = tabs;
    if (list.length === 0) return;
    const sel = $selectedQueueId;
    const idx = list.findIndex((tab) => tab.id === sel);
    const fallback = idx >= 0 ? idx : 0;
    const delta = e.key === "ArrowRight" ? 1 : -1;
    const next = list[(fallback + delta + list.length) % list.length];
    setSelectedQueue(next.id);
    // Move focus to the newly-selected tab so the cursor follows the
    // selection — standard ARIA tablist behaviour.
    const el = document.querySelector<HTMLButtonElement>(
      `[data-queue-tab-id="${next.id}"]`,
    );
    el?.focus();
  }
</script>

{#if tabs.length > 0}
  {#key $i18nVersion}
    <div
      class="tabs"
      role="tablist"
      tabindex="-1"
      aria-label={t("queue-tab-empty-state")}
      onkeydown={onKeydown}
    >
      {#each tabs as tab (tab.id)}
        <button
          type="button"
          role="tab"
          class="tab"
          class:active={$selectedQueueId === tab.id}
          class:running={tab.running}
          data-queue-tab-id={tab.id}
          aria-selected={$selectedQueueId === tab.id}
          tabindex={$selectedQueueId === tab.id ? 0 : -1}
          onclick={() => onClick(tab.id)}
        >
          <span class="label">{tab.label}</span>
          {#if tab.badge > 0}
            <span
              class="badge"
              title={t("queue-badge-tooltip")}
              aria-label={t("queue-badge-tooltip")}
            >
              {tab.badge}
            </span>
          {/if}
        </button>
      {/each}
    </div>
  {/key}
{/if}

<style>
  .tabs {
    display: flex;
    align-items: stretch;
    gap: 2px;
    padding: 4px 8px 0;
    background: var(--surface, #ffffff);
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    overflow-x: auto;
    scrollbar-width: thin;
  }

  .tab {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    font: inherit;
    font-size: 12px;
    color: var(--fg-dim, #5f5f5f);
    background: transparent;
    border: 1px solid transparent;
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    white-space: nowrap;
  }

  .tab:hover {
    background: var(--hover, rgba(128, 128, 128, 0.08));
    color: var(--fg, #1f1f1f);
  }

  .tab.active {
    color: var(--fg-strong, #1f1f1f);
    background: var(--bg, #fafafa);
    border-color: var(--border, rgba(128, 128, 128, 0.18));
    /* Align with the row content below — pull down 1 px to cover the
       bottom border so the active tab visually fuses with JobList. */
    margin-bottom: -1px;
    padding-bottom: 7px;
  }

  .tab:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: -2px;
  }

  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 16px;
    height: 16px;
    padding: 0 5px;
    font-size: 10px;
    font-weight: 600;
    line-height: 1;
    color: var(--fg-strong, #1f1f1f);
    background: var(--hover, rgba(128, 128, 128, 0.18));
    border-radius: 8px;
    font-variant-numeric: tabular-nums;
  }

  .tab.active .badge {
    background: var(--accent, #4f8cff);
    color: #ffffff;
  }
</style>
