<!--
  Phase 13d — collapsible live file-activity feed. Every file the
  engine touches appears as one row, with an icon reflecting its
  current phase (copying / done / error / folder) and an inline
  progress bar for the file that is actively mid-copy. Auto-scrolls
  to the newest row as long as the user hasn't scrolled up to
  inspect history.

  Collapsible via the header caret so users who want a minimal UI
  can hide the feed entirely; the state survives a restart via
  localStorage (wired in `stores.ts::toggleActivity`).
-->
<script lang="ts">
  import CircularProgress from "./CircularProgress.svelte";
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import {
    activityOpen,
    activitySort,
    clearActivity,
    globals,
    setActivitySort,
    sortedFileActivity,
    toggleActivity,
    type ActivitySortMode,
  } from "../stores";
  import { formatBytes, truncatePath } from "../format";

  let rows = $derived($sortedFileActivity);
  let open = $derived($activityOpen);
  let sortMode = $derived($activitySort);
  // Sort is only valid before a copy begins or after it ends.
  // While a job is actively running / paused / queued the dropdown
  // freezes so the row order can't churn mid-copy.
  let g = $derived($globals);
  let sortLocked = $derived(
    g.activeJobs + g.queuedJobs + g.pausedJobs > 0,
  );

  function onSortChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    setActivitySort(target.value as ActivitySortMode);
  }

  // Auto-scroll: jump to the bottom whenever the list grows, but
  // only if the user hasn't scrolled up to read history. Tracked
  // via an `autoStick` flag that flips to `false` the moment the
  // user scrolls above the tail and back to `true` when they reach
  // the bottom again.
  let listEl: HTMLDivElement | null = $state(null);
  let autoStick = $state(true);

  $effect(() => {
    // Subscribe to rows so this reruns whenever the feed mutates.
    rows.length;
    if (!open || !listEl || !autoStick) return;
    // rAF so the scroll lands after the new row has rendered.
    requestAnimationFrame(() => {
      if (listEl) listEl.scrollTop = listEl.scrollHeight;
    });
  });

  function onScroll() {
    if (!listEl) return;
    const nearBottom =
      listEl.scrollHeight - listEl.scrollTop - listEl.clientHeight < 24;
    autoStick = nearBottom;
  }

  function ratioFor(r: (typeof rows)[number]): number {
    if (r.phase === "done") return 1;
    if (r.phase === "error") return 0;
    if (r.bytesTotal <= 0) return 0;
    const v = r.bytesDone / r.bytesTotal;
    return Math.max(0, Math.min(1, v));
  }

  function rowStatus(
    r: (typeof rows)[number],
  ): "running" | "succeeded" | "failed" | "pending" {
    if (r.phase === "done") return "succeeded";
    if (r.phase === "error") return "failed";
    if (r.phase === "progress" || r.phase === "start") return "running";
    return "pending";
  }
</script>

<section class="activity" class:collapsed={!open}>
  {#key $i18nVersion}
    <header aria-label={t("activity-title")}>
      <button
        class="toggle"
        type="button"
        onclick={toggleActivity}
        aria-expanded={open}
        aria-label={t("activity-title")}
      >
        <Icon name={open ? "chevron-down" : "chevron-up"} size={14} />
        <span class="title">{t("activity-title")}</span>
        <span class="count">{rows.length}</span>
      </button>
      {#if open}
        <label
          class="sort"
          class:locked={sortLocked}
          title={sortLocked ? t("activity-sort-locked") : ""}
        >
          <span>{t("drop-dialog-sort-label")}</span>
          <select
            value={sortMode}
            onchange={onSortChange}
            disabled={sortLocked}
          >
            <option value="insertion">{t("sort-custom")}</option>
            <option value="name-asc">{t("sort-name-asc-simple")}</option>
            <option value="name-desc">{t("sort-name-desc-simple")}</option>
            <option value="size-asc">{t("sort-size-asc-simple")}</option>
            <option value="size-desc">{t("sort-size-desc-simple")}</option>
          </select>
        </label>
      {/if}
      {#if open && rows.length > 0}
        <button
          class="clear"
          type="button"
          onclick={clearActivity}
          aria-label={t("activity-clear")}
          title={t("activity-clear")}
        >
          <Icon name="trash" size={12} />
        </button>
      {/if}
    </header>
  {/key}

  {#if open}
    <div
      class="rows"
      bind:this={listEl}
      onscroll={onScroll}
      role="list"
    >
      {#if rows.length === 0}
        <div class="empty">{#key $i18nVersion}{t("activity-empty")}{/key}</div>
      {:else}
        {#each rows as r (r.key)}
          <div class="row" role="listitem" data-phase={r.phase}>
            <span class="ring" aria-hidden="true">
              {#if r.phase === "error"}
                <span class="glyph err"><Icon name="x" size={12} /></span>
              {:else if r.phase === "dir" || r.isDir}
                <span class="glyph folder"><Icon name="folder" size={12} /></span>
              {:else}
                <CircularProgress
                  ratio={ratioFor(r)}
                  status={rowStatus(r)}
                  size={18}
                  stroke={2.5}
                  showLabel={false}
                />
              {/if}
            </span>
            <span class="paths">
              <span class="path src" title={r.src}>
                {truncatePath(r.src, 48)}
              </span>
              {#if r.dst}
                <span class="arrow" aria-hidden="true">→</span>
                <span class="path dst" title={r.dst}>
                  {truncatePath(r.dst, 48)}
                </span>
              {/if}
            </span>
            <span class="meta tabular">
              {#if r.phase === "error"}
                <span class="pct err" title={r.message ?? ""}>error</span>
              {:else if r.phase === "dir"}
                <span class="pct muted">dir</span>
              {:else if r.phase === "done"}
                <span class="pct done" aria-label="done">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
                {#if r.bytesTotal > 0}
                  <span class="pct muted size-readout">
                    {formatBytes(r.bytesTotal)} / {formatBytes(r.bytesTotal)}
                  </span>
                {/if}
              {:else if r.bytesTotal > 0}
                <span class="pct muted size-readout">
                  {formatBytes(r.bytesDone)} / {formatBytes(r.bytesTotal)}
                </span>
              {/if}
            </span>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</section>

<style>
  .activity {
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: var(--surface, #ffffff);
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.18));
  }

  .activity.collapsed {
    flex: 0 0 auto;
  }

  .activity:not(.collapsed) {
    flex: 0 0 30%;
    min-height: 120px;
    max-height: 50%;
  }

  header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 10px;
    height: 26px;
    background: var(--footer-bg, var(--surface-alt, rgba(0, 0, 0, 0.04)));
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    font-size: 11px;
    color: var(--fg-dim, #5f5f5f);
  }

  .toggle {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    font: inherit;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .toggle:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .title {
    font-weight: 600;
    color: var(--fg-strong, #1f1f1f);
    letter-spacing: 0.02em;
    text-transform: uppercase;
    font-size: 10.5px;
  }

  .count {
    padding: 0 6px;
    background: var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 8px;
    font-variant-numeric: tabular-nums;
    min-width: 18px;
    text-align: center;
  }

  .sort {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 10.5px;
    color: var(--fg-dim, #6a6a6a);
  }

  .sort select {
    font: inherit;
    font-size: 10.5px;
    padding: 1px 4px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 3px;
    cursor: pointer;
  }

  .sort select:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .sort.locked {
    opacity: 0.85;
  }

  .clear {
    margin-left: auto;
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
  }

  .clear:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .rows {
    flex: 1;
    min-height: 0;
    overflow: auto;
    font-size: 11.5px;
  }

  .empty {
    padding: 10px 12px;
    color: var(--fg-dim, #7a7a7a);
    font-style: italic;
    font-size: 11.5px;
  }

  .row {
    display: grid;
    grid-template-columns: 22px 1fr minmax(140px, 220px);
    align-items: center;
    gap: 8px;
    padding: 3px 10px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.07));
    /* Rendering escape hatch — tells the browser to skip offscreen
       rows' layout / paint until they're near the viewport. Keeps
       a 10 000-row list scrolling at 60 fps and prevents the OOM
       that a full-drive enumeration hit previously. */
    content-visibility: auto;
    contain-intrinsic-size: 24px;
  }

  /* Highlight every row that's currently in flight. A tree copy
     with concurrency=4 lights up four rows at once — each in its
     own active state with its own spinning ring. Completed /
     errored / folder-creation rows sit quietly behind. */
  .row[data-phase="progress"],
  .row[data-phase="start"] {
    background: var(--row-selected, rgba(79, 140, 255, 0.14));
    border-left: 3px solid var(--accent, #4f8cff);
    padding-left: 7px;
  }

  .row[data-phase="progress"] .path,
  .row[data-phase="start"] .path {
    color: var(--fg-strong, #1f1f1f);
    font-weight: 600;
  }

  .ring {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
  }

  .glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .glyph.err {
    color: var(--error, #c24141);
  }

  .glyph.folder {
    color: var(--fg-dim, #6a6a6a);
  }

  .paths {
    display: flex;
    align-items: center;
    gap: 5px;
    min-width: 0;
  }

  .path {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--fg, #1f1f1f);
    max-width: 50%;
  }

  .path.dst {
    color: var(--fg-dim, #6a6a6a);
  }

  .arrow {
    flex-shrink: 0;
    color: var(--fg-dim, #8a8a8a);
  }

  .meta {
    display: flex;
    align-items: center;
    gap: 6px;
    justify-content: flex-end;
    font-variant-numeric: tabular-nums;
    font-size: 10.5px;
    color: var(--fg-dim, #6a6a6a);
  }

  /* Size readout sits next to the checkmark / ring so users always
     see `<bytes> / <total>` — completed rows included. Keeps a
     min-width so rows stay column-aligned even when the number
     formatting yanks the string length around. */
  .size-readout {
    min-width: 120px;
    text-align: right;
  }

  .pct {
    min-width: 56px;
    text-align: right;
    display: inline-flex;
    align-items: center;
    justify-content: flex-end;
  }

  .pct.done {
    color: var(--ok, #3faf6a);
    font-weight: 600;
  }

  .pct.err {
    color: var(--error, #c24141);
    font-weight: 600;
  }

  .pct.muted {
    color: var(--fg-dim, #8a8a8a);
  }

  .tabular {
    font-variant-numeric: tabular-nums;
  }

  @keyframes spin {
    from {
      transform: rotate(0);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
