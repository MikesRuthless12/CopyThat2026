<!--
  Phase 45.3 / 45.4 — named-queue tab strip with drag-merge.

  Renders one tab per `QueueSnapshotDto` from the Rust-side
  `QueueRegistry`, plus a synthesised "default" tab (id=0) covering
  jobs that flow through the legacy single-queue surface. Selecting a
  tab updates `selectedQueueIdStore`; the existing `JobList` reads
  `visibleJobs`, which is now filtered by that selection.

  Phase 45.4 — registry tabs are HTML5-draggable; dropping tab A on
  tab B fires `queue_merge(A, B)`. The synthesised default tab is
  neither a drag source nor a drop target (the legacy queue isn't
  in the registry, so `queue_merge` can't address it).

  The strip is hidden entirely when the registry holds zero queues —
  cold-launch UX is unchanged from pre-Phase-45 (one implicit queue,
  no chrome). It appears the moment the registry spawns its first
  queue (Phase 45.4+ runner reconciliation), at which point the
  default tab is needed to keep legacy-queue jobs reachable.

  i18n: `queue-tab-default`, `queue-tab-empty-state`,
  `queue-badge-tooltip`, `queue-drag-hint`, `queue-merge-confirm`,
  `queue-merge-toast`.
-->
<script lang="ts">
  import { i18nVersion, t } from "../i18n";
  import { queueMerge } from "../ipc";
  import {
    f2Mode,
    jobs,
    pushToast,
    queues,
    selectedQueueId,
    setSelectedQueue,
  } from "../stores";
  import type { JobDto, QueueSnapshotDto } from "../types";

  /// MIME-ish wire format for the drag payload. Custom string so the
  /// browser doesn't accidentally hand off the queue id to anything
  /// other than another `JobListTabs` tab.
  const DRAG_MIME = "application/x-copythat-queue";

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

  // Phase 45.4 — drag-merge state. `draggingId` is the source-queue
  // id while a drag is in flight; `dropTargetId` is the queue under
  // the cursor (used for the highlight class). Both reset on
  // `dragend` so an aborted drag (Esc, drop outside) leaves no
  // stuck highlight.
  let draggingId = $state<number | null>(null);
  let dropTargetId = $state<number | null>(null);

  function onClick(id: number) {
    setSelectedQueue(id);
  }

  function isValidDropTarget(tab: Tab): boolean {
    if (tab.isDefault) return false;
    if (draggingId === null) return false;
    if (draggingId === tab.id) return false;
    return true;
  }

  function onDragStart(e: DragEvent, tab: Tab) {
    if (tab.isDefault) {
      e.preventDefault();
      return;
    }
    draggingId = tab.id;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData(DRAG_MIME, String(tab.id));
    }
  }

  function onDragEnd() {
    draggingId = null;
    dropTargetId = null;
  }

  function onDragEnter(e: DragEvent, tab: Tab) {
    if (!isValidDropTarget(tab)) return;
    e.preventDefault();
    dropTargetId = tab.id;
  }

  function onDragOver(e: DragEvent, tab: Tab) {
    if (!isValidDropTarget(tab)) return;
    // `preventDefault` here is what tells the browser this element
    // *accepts* the drop — without it the cursor stays "no entry"
    // and `drop` never fires.
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
  }

  function onDragLeave(_e: DragEvent, tab: Tab) {
    if (dropTargetId === tab.id) dropTargetId = null;
  }

  async function onDrop(e: DragEvent, tab: Tab) {
    if (!isValidDropTarget(tab)) return;
    e.preventDefault();
    const raw = e.dataTransfer?.getData(DRAG_MIME);
    const srcId = raw && raw.length > 0 ? Number(raw) : draggingId;
    draggingId = null;
    dropTargetId = null;
    if (srcId === null || Number.isNaN(srcId) || srcId === tab.id) return;
    try {
      await queueMerge(srcId, tab.id);
      // Registry events (`queue-merged`, `queue-removed`) refresh
      // the tab strip; we only post the success toast here. If the
      // selected tab was the source, store-side reconcile flips it
      // back to the default — no extra logic in this component.
      pushToast("info", "queue-merge-toast");
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      pushToast("error", message);
    }
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
          class:dragging={draggingId === tab.id}
          class:drop-target={dropTargetId === tab.id}
          data-queue-tab-id={tab.id}
          aria-selected={$selectedQueueId === tab.id}
          tabindex={$selectedQueueId === tab.id ? 0 : -1}
          draggable={!tab.isDefault}
          title={tab.isDefault ? undefined : t("queue-drag-hint")}
          onclick={() => onClick(tab.id)}
          ondragstart={(e) => onDragStart(e, tab)}
          ondragend={onDragEnd}
          ondragenter={(e) => onDragEnter(e, tab)}
          ondragover={(e) => onDragOver(e, tab)}
          ondragleave={(e) => onDragLeave(e, tab)}
          ondrop={(e) => onDrop(e, tab)}
        >
          {#if $f2Mode && tab.running}
            <span
              class="f2-pulse"
              title={t("queue-f2-active-hint")}
              aria-label={t("queue-f2-active-hint")}
            ></span>
          {/if}
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
          {#if dropTargetId === tab.id}
            <span class="drop-hint" aria-live="polite">
              {t("queue-merge-confirm")}
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

  /* Phase 45.4 — drag-merge visual feedback. The native HTML5 drag
     ghost handles the moving thumbnail; these classes only style
     the source / target chrome that stays behind. */
  .tab.dragging {
    opacity: 0.55;
    cursor: grabbing;
  }

  .tab.drop-target {
    background: var(--row-selected, rgba(79, 140, 255, 0.18));
    border-color: var(--accent, #4f8cff);
    color: var(--fg-strong, #1f1f1f);
  }

  .drop-hint {
    margin-left: 6px;
    font-size: 10px;
    color: var(--accent, #4f8cff);
    font-style: italic;
    pointer-events: none;
  }

  /* Phase 45.5 — F2-mode pulse. Pinned to the leading edge of the
     running tab so the eye picks up which queue every fresh enqueue
     is about to land in. The animation uses a CSS keyframe rather
     than `prefers-reduced-motion`-blind transitions, and respects
     the user's reduced-motion preference. */
  .f2-pulse {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--ok, #3faf6a);
    box-shadow: 0 0 0 0 rgba(63, 175, 106, 0.55);
    animation: f2-pulse 1.4s ease-out infinite;
    flex: none;
  }

  @keyframes f2-pulse {
    0% {
      box-shadow: 0 0 0 0 rgba(63, 175, 106, 0.55);
    }
    70% {
      box-shadow: 0 0 0 6px rgba(63, 175, 106, 0);
    }
    100% {
      box-shadow: 0 0 0 0 rgba(63, 175, 106, 0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .f2-pulse {
      animation: none;
    }
  }
</style>
