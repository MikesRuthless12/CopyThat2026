<!--
  Phase 25 — Sync drawer (two-way sync pairs).

  One row per configured `SyncPair`. Each row shows the label, both
  sides' roots, the mode, last-run summary (`+3 / −1 / !2`), and a
  Run / Pause / Cancel button cluster. Clicking a pair with
  conflicts opens the conflict detail view where the user resolves
  per-file (Keep left / Keep right / Keep both).

  The "Add pair" button opens a small inline form for label + left +
  right. Path pickers would be a future polish; for now the user
  types or pastes absolute paths.

  Tauri events drive live updates: `sync-started`, `sync-walk-*`,
  `sync-action`, `sync-conflict`, `sync-completed`, `sync-failed`,
  and `sync-cancelled`. The drawer listens when mounted and detaches
  on unmount so a closed drawer does no work.
-->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { t } from "../i18n";
  import type {
    SyncPairDto,
    SyncConflictDto,
    SyncCompletedDto,
    SyncFailedDto,
    SyncStartedDto,
  } from "../types";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";

  let { onClose }: { onClose: () => void } = $props();

  let pairs: SyncPairDto[] = $state([]);
  let loading: boolean = $state(true);
  let error: string | null = $state(null);

  // Per-pair conflict lists keyed by pair id. Refreshed on every
  // `sync-conflict` event; cleared on a successful `sync-completed`
  // with zero conflicts.
  let conflictsByPair: Record<string, SyncConflictDto[]> = $state({});

  // "Add pair" form state.
  let addFormOpen: boolean = $state(false);
  let newLabel: string = $state("");
  let newLeft: string = $state("");
  let newRight: string = $state("");
  let newMode: SyncModeWire = $state("two-way");
  let saving: boolean = $state(false);

  // Focused pair for the conflict detail view.
  let focusedPairId: string | null = $state(null);

  let unlistenFns: UnlistenFn[] = [];

  onMount(() => {
    void refresh();
    (async () => {
      unlistenFns.push(
        await listen<SyncStartedDto>("sync-started", (evt) => {
          const id = evt.payload.pairId;
          pairs = pairs.map((p) => (p.id === id ? { ...p, running: true } : p));
          // Starting a new run clears the prior conflict list for this pair.
          conflictsByPair = { ...conflictsByPair, [id]: [] };
        }),
      );
      unlistenFns.push(
        await listen<SyncConflictDto>("sync-conflict", (evt) => {
          const c = evt.payload;
          const existing = conflictsByPair[c.pairId] ?? [];
          conflictsByPair = {
            ...conflictsByPair,
            [c.pairId]: [...existing, c],
          };
        }),
      );
      unlistenFns.push(
        await listen<SyncCompletedDto>("sync-completed", (evt) => {
          const payload = evt.payload;
          pairs = pairs.map((p) =>
            p.id === payload.pairId
              ? {
                  ...p,
                  running: false,
                  lastRunAt: new Date().toISOString(),
                  lastRunSummary: summaryOf(payload),
                }
              : p,
          );
        }),
      );
      unlistenFns.push(
        await listen<SyncFailedDto>("sync-failed", (evt) => {
          const id = evt.payload.pairId;
          pairs = pairs.map((p) => (p.id === id ? { ...p, running: false } : p));
          error = `${t("sync-error-prefix")}: ${evt.payload.message}`;
        }),
      );
      unlistenFns.push(
        await listen<SyncFailedDto>("sync-cancelled", (evt) => {
          const id = evt.payload.pairId;
          pairs = pairs.map((p) => (p.id === id ? { ...p, running: false } : p));
        }),
      );
      unlistenFns.push(
        await listen<{ pairId: string }>("live-mirror-started", (evt) => {
          const id = evt.payload.pairId;
          pairs = pairs.map((p) => (p.id === id ? { ...p, liveMirror: true } : p));
        }),
      );
      unlistenFns.push(
        await listen<{ pairId: string }>("live-mirror-stopped", (evt) => {
          const id = evt.payload.pairId;
          pairs = pairs.map((p) => (p.id === id ? { ...p, liveMirror: false } : p));
        }),
      );
    })().catch((e) => {
      console.error("[sync-listen]", e);
    });
  });

  onDestroy(() => {
    for (const f of unlistenFns) {
      try {
        f();
      } catch {
        // Unlisten errors are best-effort; swallow.
      }
    }
    unlistenFns = [];
  });

  async function refresh() {
    loading = true;
    error = null;
    try {
      pairs = await invoke<SyncPairDto[]>("list_sync_pairs");
    } catch (e) {
      error = String(e);
    }
    loading = false;
  }

  async function pickPath(prompt: string): Promise<string | null> {
    try {
      const selected = await openDialog({
        directory: true,
        multiple: false,
        title: prompt,
      });
      if (typeof selected === "string") return selected;
      return null;
    } catch {
      return null;
    }
  }

  async function addPair() {
    if (!newLabel.trim() || !newLeft.trim() || !newRight.trim()) {
      error = t("sync-add-missing-fields");
      return;
    }
    saving = true;
    error = null;
    try {
      await invoke<SyncPairDto>("add_sync_pair", {
        label: newLabel.trim(),
        left: newLeft.trim(),
        right: newRight.trim(),
        mode: newMode,
      });
      newLabel = "";
      newLeft = "";
      newRight = "";
      newMode = "two-way";
      addFormOpen = false;
      await refresh();
    } catch (e) {
      error = String(e);
    }
    saving = false;
  }

  async function removePair(id: string) {
    if (!confirm(t("sync-remove-confirm"))) return;
    try {
      await invoke("remove_sync_pair", { pairId: id });
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  async function runPair(id: string) {
    error = null;
    try {
      await invoke<string>("start_sync", { pairId: id });
    } catch (e) {
      error = String(e);
    }
  }

  async function cancelPair(id: string) {
    try {
      await invoke("cancel_sync", { pairId: id });
    } catch (e) {
      error = String(e);
    }
  }

  async function toggleLiveMirror(id: string, active: boolean) {
    error = null;
    try {
      if (active) {
        await invoke("stop_live_mirror", { pairId: id });
      } else {
        await invoke<string>("start_live_mirror", { pairId: id });
      }
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  function summaryOf(r: SyncCompletedDto): string {
    const applied = r.appliedLeft + r.appliedRight;
    const deleted = r.deletedLeft + r.deletedRight;
    return `+${applied} / −${deleted} / !${r.conflicts}`;
  }

  function conflictsFor(pairId: string): SyncConflictDto[] {
    return conflictsByPair[pairId] ?? [];
  }

  function focusPair(id: string) {
    focusedPairId = id;
  }

  function unfocus() {
    focusedPairId = null;
  }

  type SyncModeWire =
    | "two-way"
    | "mirror-left-to-right"
    | "mirror-right-to-left"
    | "contribute-left-to-right";
</script>

<div class="backdrop" role="presentation" onclick={onClose}>
  <div
    class="drawer"
    role="dialog"
    aria-labelledby="sync-title"
    tabindex={-1}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => {
      if (e.key === "Escape") onClose();
    }}
  >
    <header>
      <h2 id="sync-title">{t("sync-drawer-title")}</h2>
      <p class="hint">{t("sync-drawer-hint")}</p>
      <div class="toolbar">
        <button type="button" onclick={() => (addFormOpen = !addFormOpen)}>
          {addFormOpen ? t("sync-add-cancel") : t("sync-add-pair")}
        </button>
        <button type="button" onclick={() => void refresh()}>
          {t("sync-refresh")}
        </button>
      </div>
    </header>

    {#if error}
      <div class="error" role="alert">{error}</div>
    {/if}

    {#if addFormOpen}
      <form
        class="add-form"
        onsubmit={(e) => {
          e.preventDefault();
          void addPair();
        }}
      >
        <label class="field">
          <span>{t("sync-field-label")}</span>
          <input
            type="text"
            bind:value={newLabel}
            placeholder={t("sync-field-label-placeholder")}
          />
        </label>
        <label class="field">
          <span>{t("sync-field-left")}</span>
          <div class="path-row">
            <input
              type="text"
              bind:value={newLeft}
              placeholder={t("sync-field-left-placeholder")}
            />
            <button
              type="button"
              onclick={async () => {
                const p = await pickPath(t("sync-field-left"));
                if (p) newLeft = p;
              }}
            >
              …
            </button>
          </div>
        </label>
        <label class="field">
          <span>{t("sync-field-right")}</span>
          <div class="path-row">
            <input
              type="text"
              bind:value={newRight}
              placeholder={t("sync-field-right-placeholder")}
            />
            <button
              type="button"
              onclick={async () => {
                const p = await pickPath(t("sync-field-right"));
                if (p) newRight = p;
              }}
            >
              …
            </button>
          </div>
        </label>
        <label class="field">
          <span>{t("sync-field-mode")}</span>
          <select bind:value={newMode}>
            <option value="two-way">{t("sync-mode-two-way")}</option>
            <option value="mirror-left-to-right"
              >{t("sync-mode-mirror-left-to-right")}</option
            >
            <option value="mirror-right-to-left"
              >{t("sync-mode-mirror-right-to-left")}</option
            >
            <option value="contribute-left-to-right"
              >{t("sync-mode-contribute-left-to-right")}</option
            >
          </select>
        </label>
        <div class="form-actions">
          <button type="submit" class="primary" disabled={saving}>
            {saving ? t("sync-add-saving") : t("sync-add-save")}
          </button>
        </div>
      </form>
    {/if}

    {#if focusedPairId}
      {@const focused = pairs.find((p) => p.id === focusedPairId)}
      {@const focusedConflicts = conflictsFor(focusedPairId)}
      <section class="conflict-detail">
        <header>
          <button type="button" onclick={unfocus} class="back">
            ← {t("action-close")}
          </button>
          <h3>
            {focused?.label ?? focusedPairId}: {t("sync-conflicts-heading")}
            ({focusedConflicts.length})
          </h3>
        </header>
        {#if focusedConflicts.length === 0}
          <p class="empty">{t("sync-no-conflicts")}</p>
        {:else}
          <ul class="conflicts">
            {#each focusedConflicts as c (c.relpath)}
              <li>
                <div class="relpath"><strong>{c.relpath}</strong></div>
                <div class="meta">
                  <span class="kind">{t(`sync-conflict-kind-${c.kind}`)}</span>
                  <span class="winner">
                    {t("sync-winner")}:
                    {t(`sync-side-${c.winnerSide}`)}
                  </span>
                </div>
                <div class="preservation">
                  <code>{c.loserPreservationPath}</code>
                </div>
                <div class="actions">
                  <button type="button" disabled>
                    {t("sync-resolve-keep-left")}
                  </button>
                  <button type="button" disabled>
                    {t("sync-resolve-keep-right")}
                  </button>
                  <button type="button" disabled>
                    {t("sync-resolve-keep-both")}
                  </button>
                  <button
                    type="button"
                    disabled
                    title={t("sync-resolve-phase-53-tooltip")}
                  >
                    {t("sync-resolve-three-way")}
                  </button>
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {:else if loading}
      <p class="empty">{t("sync-loading")}</p>
    {:else if pairs.length === 0}
      <p class="empty">{t("sync-no-pairs")}</p>
    {:else}
      <ul class="pairs">
        {#each pairs as p (p.id)}
          {@const cs = conflictsFor(p.id)}
          <li class:running={p.running}>
            <div class="pair-head">
              <strong>{p.label}</strong>
              <span class="mode">{t(`sync-mode-${p.mode}`)}</span>
              {#if p.liveMirror}
                <span class="pulse live" aria-label={t("live-mirror-watching")}
                ></span>
                <span class="live-label">{t("live-mirror-watching")}</span>
              {:else if p.running}
                <span class="pulse" aria-label={t("sync-running")}></span>
              {/if}
            </div>
            <div class="paths">
              <span class="side">{p.left}</span>
              <span class="arrow">↔</span>
              <span class="side">{p.right}</span>
            </div>
            <div class="summary">
              {#if p.lastRunSummary}
                <span class="last">{p.lastRunSummary}</span>
              {:else}
                <span class="last">{t("sync-never-run")}</span>
              {/if}
              {#if cs.length > 0}
                <button
                  type="button"
                  class="conflict-link"
                  onclick={() => focusPair(p.id)}
                >
                  {t("sync-view-conflicts", { count: cs.length })}
                </button>
              {/if}
            </div>
            <div class="actions">
              {#if p.running}
                <button type="button" onclick={() => void cancelPair(p.id)}>
                  {t("sync-cancel")}
                </button>
              {:else}
                <button
                  type="button"
                  class="primary"
                  onclick={() => void runPair(p.id)}
                >
                  {t("sync-run-now")}
                </button>
              {/if}
              <button
                type="button"
                class={p.liveMirror ? "live-on" : "live-off"}
                onclick={() => void toggleLiveMirror(p.id, p.liveMirror)}
                title={t("live-mirror-toggle-hint")}
              >
                {p.liveMirror
                  ? t("live-mirror-stop")
                  : t("live-mirror-start")}
              </button>
              <button
                type="button"
                class="danger"
                onclick={() => void removePair(p.id)}
              >
                {t("sync-remove-pair")}
              </button>
            </div>
          </li>
        {/each}
      </ul>
    {/if}

    <footer>
      <button type="button" onclick={onClose}>{t("action-close")}</button>
    </footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .drawer {
    background: var(--surface, #1a1a1a);
    color: var(--text, #fff);
    border-radius: 8px;
    padding: 1.5rem;
    max-width: 820px;
    width: 92%;
    max-height: 82vh;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }
  header h2 {
    margin: 0 0 0.25rem 0;
  }
  .hint {
    margin: 0 0 0.75rem 0;
    color: var(--text-muted, #aaa);
    font-size: 0.9rem;
  }
  .toolbar {
    display: flex;
    gap: 0.5rem;
  }
  .error {
    background: rgba(220, 60, 60, 0.15);
    color: #f88;
    padding: 0.5rem 0.75rem;
    border-radius: 4px;
    font-size: 0.85rem;
  }
  .add-form {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.75rem;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--border, #333);
    border-radius: 4px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.85rem;
  }
  .field > span {
    color: var(--text-muted, #aaa);
  }
  .field input,
  .field select {
    background: var(--input-bg, #0f0f0f);
    border: 1px solid var(--border, #333);
    border-radius: 4px;
    padding: 0.4rem 0.6rem;
    color: var(--text, #fff);
    font-family: var(--font-mono, monospace);
  }
  .path-row {
    display: flex;
    gap: 0.4rem;
  }
  .path-row input {
    flex: 1;
  }
  .form-actions {
    display: flex;
    justify-content: flex-end;
  }
  .pairs,
  .conflicts {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }
  .pairs li {
    padding: 0.75rem 0;
    border-top: 1px solid var(--border, #333);
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }
  .pairs li:first-child {
    border-top: none;
  }
  .pair-head {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .mode {
    color: var(--text-muted, #aaa);
    font-size: 0.8rem;
    text-transform: lowercase;
  }
  .pulse {
    width: 8px;
    height: 8px;
    background: #4caf50;
    border-radius: 50%;
    animation: pulse 1.2s ease-in-out infinite;
  }
  .pulse.live {
    background: #22c55e;
    box-shadow: 0 0 8px rgba(34, 197, 94, 0.7);
  }
  .live-label {
    color: #22c55e;
    font-size: 0.78rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  @keyframes pulse {
    0%,
    100% {
      opacity: 0.3;
    }
    50% {
      opacity: 1;
    }
  }
  .paths {
    font-family: var(--font-mono, monospace);
    font-size: 0.85rem;
    display: flex;
    gap: 0.4rem;
    flex-wrap: wrap;
    color: var(--text-muted, #aaa);
  }
  .arrow {
    color: var(--text, #fff);
  }
  .side {
    word-break: break-all;
  }
  .summary {
    font-size: 0.85rem;
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  .last {
    color: var(--text-muted, #aaa);
  }
  .actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }
  .conflict-link {
    color: #f0b940;
    background: transparent;
    border: 1px solid #f0b940;
    padding: 0.2rem 0.6rem;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
  }
  .conflict-link:hover {
    background: rgba(240, 185, 64, 0.12);
  }
  .conflict-detail header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .conflict-detail header h3 {
    margin: 0;
    font-size: 1rem;
  }
  .back {
    padding: 0.25rem 0.6rem;
    font-size: 0.85rem;
  }
  .conflicts li {
    padding: 0.6rem 0;
    border-top: 1px solid var(--border, #333);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .conflicts .relpath {
    font-family: var(--font-mono, monospace);
  }
  .conflicts .meta {
    font-size: 0.8rem;
    color: var(--text-muted, #aaa);
    display: flex;
    gap: 1rem;
  }
  .conflicts .preservation {
    font-size: 0.75rem;
    color: var(--text-muted, #aaa);
    word-break: break-all;
  }
  .empty {
    color: var(--text-muted, #aaa);
    text-align: center;
    padding: 1.5rem;
  }
  button {
    padding: 0.4rem 0.8rem;
    border-radius: 4px;
    border: 1px solid var(--border, #444);
    background: var(--button-bg, #2a2a2a);
    color: var(--text, #fff);
    cursor: pointer;
    font-size: 0.85rem;
  }
  button:hover {
    background: var(--button-bg-hover, #353535);
  }
  button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  button.primary {
    background: var(--accent, #3b82f6);
    border-color: var(--accent, #3b82f6);
    color: white;
  }
  button.primary:hover {
    background: var(--accent-hover, #2563eb);
  }
  button.live-on {
    border-color: #22c55e;
    color: #22c55e;
  }
  button.live-on:hover {
    background: rgba(34, 197, 94, 0.12);
  }
  button.live-off {
    border-color: var(--border, #444);
  }
  button.danger {
    border-color: var(--danger, #c33);
    color: var(--danger, #c33);
  }
  button.danger:hover {
    background: var(--danger, #c33);
    color: white;
  }
  footer {
    display: flex;
    justify-content: flex-end;
  }
</style>
