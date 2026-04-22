<!--
  Modal that appears after a drop: lists the dropped paths and
  asks the user to pick a destination. Clicking "Pick destination"
  opens the Tauri dialog plugin's directory picker; once chosen,
  the command layer enqueues one `copy` job per source.
-->
<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  import Icon from "../icons/Icon.svelte";
  import PreflightModal from "./PreflightModal.svelte";
  import SubsetPickerModal from "./SubsetPickerModal.svelte";
  import { i18nVersion, t } from "../i18n";
  import { onEvent, pathMetadata, startCopy, startMove } from "../ipc";
  import { formatBytes } from "../format";
  import { clearDropped, preseedBeforeEnqueue, pushToast } from "../stores";
  import type { CollisionPolicyWire } from "../types";

  interface Props {
    paths: string[];
  }

  let { paths }: Props = $props();

  // Per-path metadata (isDir + recursive size) powers the sort
  // and the file/folder grouping. Populated once on mount; null
  // size is rendered as "…" while the walk is in flight.
  type Meta = { isDir: boolean; size: number | null };
  // svelte-ignore state_referenced_locally
  let metas = $state<Meta[]>(paths.map(() => ({ isDir: false, size: null })));
  // `ordered` is the working copy of indices into `paths`. All
  // sort / reorder operations mutate this array, and the final
  // enqueue pulls `finalPaths = ordered.map(i => paths[i])`.
  // svelte-ignore state_referenced_locally
  let ordered = $state<number[]>(paths.map((_, i) => i));

  type SortMode = "custom" | "name-asc" | "name-desc" | "size-asc" | "size-desc";
  let sortMode: SortMode = $state(
    (typeof localStorage !== "undefined"
      ? (localStorage.getItem("copythat-sort-mode") as SortMode | null)
      : null) ?? "custom",
  );

  let destination: string | null = $state(null);
  let kind: "copy" | "move" = $state("copy");
  let busy = $state(false);
  // What the modal is currently doing so the Start button doesn't
  // look frozen on big trees. `null` = idle (button reads the
  // localised "Start copy" text); anything else wins the label.
  let busyLabel = $state<string | null>(null);
  // Live file count streamed from the Rust-side enumerator's
  // `enumeration-progress` event. When the button is in
  // `Counting files…` state, we append this number so the user
  // sees the walk actually making progress instead of a silent
  // spinner.
  let enumerationCount = $state<number>(0);
  // Collision policy picked up-front so "paste into the same spot"
  // auto-renames via `keep-both` without a modal per file. Default
  // is `keep-both` because that's the least-destructive option for
  // the common "copy + paste here" case: neither the original nor
  // the new copy is ever overwritten or silently dropped.
  const COLLISION_KEY = "copythat-collision-policy";
  let collision: CollisionPolicyWire = $state(
    (typeof localStorage !== "undefined"
      ? (localStorage.getItem(COLLISION_KEY) as CollisionPolicyWire | null)
      : null) ?? "keep-both",
  );
  function onCollisionChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    collision = target.value as CollisionPolicyWire;
    try {
      localStorage.setItem(COLLISION_KEY, collision);
    } catch {
      // ignore
    }
  }

  $effect(() => {
    void loadMetadata();
  });

  async function loadMetadata() {
    try {
      const raw = await pathMetadata(paths);
      metas = raw.map((m) => ({
        isDir: m.isDir,
        size: m.size >= Number.MAX_SAFE_INTEGER ? Number.MAX_SAFE_INTEGER : m.size,
      }));
      // If the user had a non-custom sort selected last session,
      // apply it now that sizes are known.
      if (sortMode !== "custom") applySort(sortMode);
    } catch {
      // Non-fatal — the reorder buttons still work, only the sort
      // presets are unavailable until metadata loads.
    }
  }

  function basename(p: string): string {
    const idx = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    return idx >= 0 ? p.slice(idx + 1) || p : p;
  }

  // The files-before-folders invariant is non-negotiable: any
  // reorder operation has to preserve `[...files, ...folders]`.
  // `groupBounds` returns the [min, max] range for whichever group
  // the caller's row belongs to, so up/down moves can clamp to
  // their own group and never hop across the fence.
  function groupBounds(row: number): { min: number; max: number } {
    const isFolder = metas[ordered[row]].isDir;
    let min = 0;
    let max = ordered.length - 1;
    for (let i = 0; i < ordered.length; i++) {
      if (metas[ordered[i]].isDir === isFolder) {
        if (min === 0 && i > 0) min = min === 0 ? i : min;
        max = i;
      }
    }
    // First pass of `min` above is fiddly; recompute cleanly:
    min = ordered.findIndex((idx) => metas[idx].isDir === isFolder);
    if (min < 0) min = 0;
    return { min, max };
  }

  function applySort(mode: SortMode) {
    sortMode = mode;
    try {
      localStorage.setItem("copythat-sort-mode", mode);
    } catch {
      // localStorage can reject in private mode; ignore.
    }
    if (mode === "custom") {
      // Even in custom mode, enforce the files-first invariant so
      // a fresh drop lands in a predictable shape.
      reapplyFilesBeforeFolders();
      return;
    }
    const compareWithin = (a: number, b: number): number => {
      switch (mode) {
        case "name-asc":
          return basename(paths[a]).localeCompare(basename(paths[b]));
        case "name-desc":
          return basename(paths[b]).localeCompare(basename(paths[a]));
        case "size-asc":
          return (metas[a].size ?? 0) - (metas[b].size ?? 0);
        case "size-desc":
          return (metas[b].size ?? 0) - (metas[a].size ?? 0);
        default:
          return 0;
      }
    };
    const files = ordered.filter((i) => !metas[i].isDir).sort(compareWithin);
    const folders = ordered.filter((i) => metas[i].isDir).sort(compareWithin);
    ordered = [...files, ...folders];
  }

  function reapplyFilesBeforeFolders() {
    // Stable partition — keep the user's in-group order, just
    // pull all files ahead of all folders.
    const files = ordered.filter((i) => !metas[i].isDir);
    const folders = ordered.filter((i) => metas[i].isDir);
    const next = [...files, ...folders];
    // Only mutate if something actually changed so Svelte doesn't
    // pay for unnecessary re-renders.
    if (next.some((v, i) => v !== ordered[i])) ordered = next;
  }

  function onSortChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    applySort(target.value as SortMode);
  }

  function moveIdx(from: number, to: number) {
    if (from < 0 || from >= ordered.length) return;
    const { min, max } = groupBounds(from);
    const clamped = Math.max(min, Math.min(max, to));
    if (clamped === from) return;
    const next = ordered.slice();
    const [x] = next.splice(from, 1);
    next.splice(clamped, 0, x);
    ordered = next;
    sortMode = "custom";
    try {
      localStorage.setItem("copythat-sort-mode", "custom");
    } catch {
      // ignore
    }
    // Defensive — a manual reorder should never violate the
    // invariant, but re-partitioning is cheap and guarantees it.
    reapplyFilesBeforeFolders();
    // Keep the focus on the row that just moved.
    selectedRow = ordered.indexOf(x);
  }

  function moveTop(from: number) {
    const { min } = groupBounds(from);
    moveIdx(from, min);
  }
  function moveUp(from: number) {
    moveIdx(from, from - 1);
  }
  function moveDown(from: number) {
    moveIdx(from, from + 1);
  }
  function moveBottom(from: number) {
    const { max } = groupBounds(from);
    moveIdx(from, max);
  }

  // Keyboard nav — Ctrl-Up/Down = one step, Shift-Up/Down =
  // jump to the group's top/bottom edge. Matches the on-screen
  // button set one-for-one.
  let selectedRow = $state<number>(0);
  function onListKeydown(e: KeyboardEvent) {
    if (ordered.length === 0) return;
    const row = selectedRow;
    if (e.key === "ArrowUp" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      moveUp(row);
    } else if (e.key === "ArrowDown" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      moveDown(row);
    } else if (e.key === "ArrowUp" && e.shiftKey) {
      e.preventDefault();
      moveTop(row);
    } else if (e.key === "ArrowDown" && e.shiftKey) {
      e.preventDefault();
      moveBottom(row);
    } else if (e.key === "Home") {
      // Home = jump the selected row to its group's top edge. The
      // files-before-folders invariant keeps files inside the files
      // half and folders inside the folders half.
      e.preventDefault();
      moveTop(row);
    } else if (e.key === "End") {
      // End = jump to the group's bottom edge.
      e.preventDefault();
      moveBottom(row);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedRow = Math.max(0, row - 1);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedRow = Math.min(ordered.length - 1, row + 1);
    }
  }
  interface PreflightResult {
    proceed: boolean;
    pickedSubset?: string[];
  }
  // Preflight — populated on confirm(); when non-null, PreflightModal
  // renders on top of this dialog and runs the free-space probes.
  let preflight = $state<null | { resolve: (r: PreflightResult) => void }>(null);
  // Subset picker — opens when the user picks "pick which ones" in
  // a preflight-block state. The resolver bubbles the chosen paths
  // back through the preflight promise.
  let subset = $state<null | { freeBytes: number }>(null);

  const RESERVE_BYTES = (() => {
    try {
      const raw = localStorage.getItem("copythat-reserve-bytes");
      const n = raw ? Number(raw) : 0;
      return Number.isFinite(n) && n >= 0 ? n : 0;
    } catch {
      return 0;
    }
  })();

  async function pick() {
    const chosen = await open({ directory: true, multiple: false });
    if (typeof chosen === "string" && chosen.trim().length > 0) {
      destination = chosen;
    }
  }

  async function confirm() {
    if (!destination) return;
    busy = true;
    busyLabel = t("drop-dialog-busy-checking");
    enumerationCount = 0;
    // Subscribe to the streaming enumerator's progress events for
    // the duration of this confirm() — we unlisten in the finally
    // block so the subscription doesn't outlive the dialog.
    const unlisten = await onEvent<{
      count: number;
      done: boolean;
      overflow: boolean;
    }>("enumeration-progress", (p) => {
      enumerationCount = p.count;
    });
    try {
      // Phase 14 preflight — pause here until PreflightModal resolves
      // (or resolves auto-immediately when everything fits). The
      // result can also carry a picked subset from SubsetPickerModal.
      const result = await new Promise<PreflightResult>((resolve) => {
        preflight = { resolve };
      });
      preflight = null;
      subset = null;
      if (!result.proceed) return;
      // Respect the user's ordered list. If the preflight picker
      // returned a subset, filter the ordered list down to just
      // those paths (keeping the user's chosen order for them).
      const orderedPaths = ordered.map((i) => paths[i]);
      const picked = result.pickedSubset;
      const finalPaths = picked
        ? orderedPaths.filter((p) => picked.includes(p))
        : orderedPaths;
      if (finalPaths.length === 0) return;
      // Pre-seed the Activity list *before* enqueueing, so the user
      // sees the full file list the instant the engine starts copy.
      // For whole-drive / millions-of-files cases the enumeration
      // has a wall-clock budget — if it times out, we skip the
      // pre-seed and let the engine's own pre-walk populate the
      // list lazily rather than leaving the Start button frozen.
      busyLabel = t("drop-dialog-busy-enumerating");
      const seed = await preseedBeforeEnqueue(finalPaths, { timeoutMs: 8_000 });
      if (!seed.completed) {
        // User-visible nudge so nobody thinks the app has stalled
        // when the Start button disables for the engine's own walk.
        pushToast("info", "toast-enumeration-deferred");
      }
      busyLabel = t("drop-dialog-busy-starting");
      if (kind === "copy") {
        await startCopy(finalPaths, destination, { collision });
      } else {
        await startMove(finalPaths, destination, { collision });
      }
      pushToast("info", kind === "copy" ? "toast-copy-queued" : "toast-move-queued");
      clearDropped();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      unlisten();
      busy = false;
      busyLabel = null;
      enumerationCount = 0;
    }
  }

  function onPreflightOpenSubset(freeBytes: number) {
    subset = { freeBytes };
  }

  function onSubsetResolve(picked: string[] | null) {
    const resolver = preflight?.resolve;
    if (!resolver) return;
    if (picked === null || picked.length === 0) {
      resolver({ proceed: false });
    } else {
      resolver({ proceed: true, pickedSubset: picked });
    }
    subset = null;
  }

  function cancel() {
    clearDropped();
  }
</script>

<div
  class="backdrop"
  role="presentation"
  onclick={cancel}
  onkeydown={(e) => {
    if (e.key === "Escape") cancel();
  }}
>
  {#key $i18nVersion}
  <div
    class="modal"
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-label={t("drop-dialog-title")}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <header>
      <h2>{t("drop-dialog-title")}</h2>
      <button
        type="button"
        class="close"
        aria-label={t("action-close")}
        onclick={cancel}
      >
        <Icon name="x" size={16} />
      </button>
    </header>
    <p class="sub">{t("drop-dialog-subtitle", { count: paths.length })}</p>

    <div class="sort-row">
      <label>
        <span>{t("drop-dialog-sort-label")}</span>
        <select value={sortMode} onchange={onSortChange}>
          <option value="custom">{t("sort-custom")}</option>
          <option value="name-asc">{t("sort-name-asc")}</option>
          <option value="name-desc">{t("sort-name-desc")}</option>
          <option value="size-asc">{t("sort-size-asc")}</option>
          <option value="size-desc">{t("sort-size-desc")}</option>
        </select>
      </label>
    </div>

    <ul
      class="sources"
      tabindex="0"
      role="listbox"
      aria-label={t("drop-dialog-sort-label")}
      aria-activedescendant={`src-row-${selectedRow}`}
      onkeydown={onListKeydown}
    >
      {#each ordered as srcIdx, row (srcIdx)}
        {@const m = metas[srcIdx]}
        {@const bounds = groupBounds(row)}
        {@const isAtTop = row === bounds.min}
        {@const isAtBottom = row === bounds.max}
        <li
          class="source-row"
          id={`src-row-${row}`}
          role="option"
          aria-selected={row === selectedRow}
          class:selected={row === selectedRow}
          data-kind={m.isDir ? "folder" : "file"}
          onclick={() => (selectedRow = row)}
          onkeydown={() => { /* a11y: parent <ul> owns keyboard nav */ }}
        >
          <Icon name={m.isDir ? "folder" : "file"} size={13} />
          <span class="path" title={paths[srcIdx]}>{paths[srcIdx]}</span>
          <span class="size tabular">
            {m.size === null
              ? "…"
              : m.size >= Number.MAX_SAFE_INTEGER
                ? t("subset-too-large")
                : formatBytes(m.size)}
          </span>
          <div class="move-buttons" role="toolbar" aria-label={t("sort-reorder")}>
            <button
              type="button"
              class="move"
              onclick={() => moveTop(row)}
              disabled={isAtTop}
              title={t("sort-move-top")}
              aria-label={t("sort-move-top")}
            >⤒</button>
            <button
              type="button"
              class="move"
              onclick={() => moveUp(row)}
              disabled={isAtTop}
              title={t("sort-move-up")}
              aria-label={t("sort-move-up")}
            >↑</button>
            <button
              type="button"
              class="move"
              onclick={() => moveDown(row)}
              disabled={isAtBottom}
              title={t("sort-move-down")}
              aria-label={t("sort-move-down")}
            >↓</button>
            <button
              type="button"
              class="move"
              onclick={() => moveBottom(row)}
              disabled={isAtBottom}
              title={t("sort-move-bottom")}
              aria-label={t("sort-move-bottom")}
            >⤓</button>
          </div>
        </li>
      {/each}
    </ul>

    <div class="collision-row">
      <label>
        <span>{t("drop-dialog-collision-label")}</span>
        <select value={collision} onchange={onCollisionChange}>
          <option value="keep-both">{t("collision-policy-keep-both")}</option>
          <option value="skip">{t("collision-policy-skip")}</option>
          <option value="overwrite">{t("collision-policy-overwrite")}</option>
          <option value="overwrite-if-newer">
            {t("collision-policy-overwrite-if-newer")}
          </option>
          <option value="prompt">{t("collision-policy-prompt")}</option>
        </select>
      </label>
    </div>

    <div class="mode" role="radiogroup" aria-label={t("drop-dialog-mode")}>
      <label>
        <input
          type="radio"
          name="kind"
          value="copy"
          bind:group={kind}
        />
        {t("drop-dialog-copy")}
      </label>
      <label>
        <input
          type="radio"
          name="kind"
          value="move"
          bind:group={kind}
        />
        {t("drop-dialog-move")}
      </label>
    </div>

    <div class="dest">
      <button type="button" class="pick" onclick={pick} disabled={busy}>
        <Icon name="folder" size={14} />
        {destination
          ? t("drop-dialog-change-destination")
          : t("drop-dialog-pick-destination")}
      </button>
      {#if destination}
        <span class="path" title={destination}>{destination}</span>
      {/if}
    </div>

    <div class="actions">
      <button class="secondary" type="button" onclick={cancel} disabled={busy}>
        {t("action-cancel")}
      </button>
      <button
        class="primary"
        type="button"
        onclick={confirm}
        disabled={busy || !destination}
      >
        {busy && busyLabel
          ? enumerationCount > 0
            ? `${busyLabel} ${enumerationCount.toLocaleString()}`
            : busyLabel
          : kind === "copy"
            ? t("drop-dialog-start-copy")
            : t("drop-dialog-start-move")}
      </button>
    </div>
  </div>
  {/key}
</div>

{#if preflight && destination && !subset}
  <PreflightModal
    sources={paths}
    {destination}
    reserveBytes={RESERVE_BYTES}
    onResolve={preflight.resolve}
    onOpenSubset={onPreflightOpenSubset}
  />
{/if}

{#if subset && destination}
  <SubsetPickerModal
    sources={paths}
    freeBytes={subset.freeBytes}
    reserveBytes={RESERVE_BYTES}
    onResolve={onSubsetResolve}
  />
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 90;
  }

  .modal {
    width: min(440px, 92vw);
    max-height: 82vh;
    padding: 14px 16px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  h2 {
    margin: 0;
    font-size: 14px;
  }

  .close {
    margin-left: auto;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .sub {
    margin: 0;
    font-size: 12px;
    color: var(--fg-dim, #6a6a6a);
  }

  .sources {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 200px;
    overflow: auto;
    font-size: 11.5px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 6px;
  }

  .source-row {
    display: grid;
    grid-template-columns: 16px 1fr auto auto;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.1));
    color: var(--fg, #1f1f1f);
  }

  .source-row:last-child {
    border-bottom: none;
  }

  .source-row[data-kind="folder"] {
    background: var(--surface-alt, rgba(0, 0, 0, 0.02));
  }

  .source-row.selected {
    background: var(--row-selected, rgba(79, 140, 255, 0.14));
  }

  .sources:focus {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: -2px;
  }

  .source-row .path {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    direction: rtl;
    text-align: left;
  }

  .source-row .size {
    color: var(--fg-dim, #6a6a6a);
    min-width: 72px;
    text-align: right;
  }

  .tabular {
    font-variant-numeric: tabular-nums;
  }

  .move-buttons {
    display: inline-flex;
    gap: 2px;
  }

  button.move {
    padding: 2px 6px;
    font-size: 12px;
    border: 1px solid transparent;
    background: transparent;
    color: var(--fg-dim, #6a6a6a);
    border-radius: 4px;
    cursor: pointer;
    font-family: system-ui, sans-serif;
    line-height: 1;
  }

  button.move:hover:not(:disabled) {
    background: var(--hover, rgba(128, 128, 128, 0.14));
    color: var(--fg-strong, #1f1f1f);
  }

  button.move:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .sort-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11.5px;
    color: var(--fg-dim, #6a6a6a);
  }

  .sort-row select {
    padding: 3px 6px;
    font-size: 11.5px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    cursor: pointer;
  }

  .collision-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11.5px;
    color: var(--fg-dim, #6a6a6a);
  }

  .collision-row select {
    padding: 3px 6px;
    font-size: 11.5px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    cursor: pointer;
  }

  .mode {
    display: flex;
    gap: 16px;
    font-size: 12px;
  }

  .mode label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
  }

  .dest {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
    font-size: 12px;
  }

  .pick {
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

  .pick:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .path {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--fg-dim, #6a6a6a);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  button.primary,
  button.secondary {
    padding: 6px 12px;
    border-radius: 6px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
    border: 1px solid transparent;
  }

  button.secondary {
    background: transparent;
    border-color: var(--border, rgba(128, 128, 128, 0.3));
    color: inherit;
  }

  button.primary {
    background: var(--accent, #4f8cff);
    color: #ffffff;
  }

  button.primary:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
</style>
