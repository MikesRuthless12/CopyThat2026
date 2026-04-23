<!--
  Phase 22 — aggregate conflict dialog v2.

  Replaces the Phase 8 single-file CollisionModal with a richer dialog
  that opens ONCE per job and stays open as the engine streams
  collisions in.

  Layout:
  - Left rail (240 px, virtualized-ish): one row per conflict with
    64x64 thumbnail + filename + state badge.
  - Right pane: detail for the selected conflict — 240x240 side-by-side
    thumbnails + metadata + per-row action bar.
  - Bottom bar: bulk actions (apply-to-selected /
    apply-to-extension / apply-to-glob / apply-to-remaining) +
    "Save these rules as profile…".

  Data flow:
  - `$conflictBatch` is the full row set for the lifetime of the
    dialog; rows are added by `collision-raised` / `collision-auto-
    resolved` events and flipped by `collision-resolved`.
  - Per-row actions call `resolveCollision(id, resolution, rename, applyAll)`.
  - Bulk actions call `addConflictRule(jobId, pattern, resolution)`
    (installs the rule on the running job) plus, for every already-
    rendered pending row that matches the pattern, `resolveCollision`
    immediately so the UI unblocks the queued engine calls.
  - Save-as-profile calls `saveConflictProfile(name, profile)`.
-->
<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";

  import Icon from "../icons/Icon.svelte";
  import FileKindIcon from "../icons/FileKindIcon.svelte";
  import { i18nVersion, t } from "../i18n";
  import { formatBytes } from "../format";
  import {
    addConflictRule,
    currentConflictRules,
    resolveCollision,
    saveConflictProfile,
    thumbnailFor,
  } from "../ipc";
  import {
    clearConflictBatch,
    conflictBatch,
    pushToast,
    type ConflictBatchRow,
  } from "../stores";
  import type {
    ConflictRuleResolution,
    FileIconDto,
    ThumbnailDto,
  } from "../types";

  function iconDtoFor(thumb: ThumbnailDto | undefined): FileIconDto {
    return {
      kind: (thumb?.iconKind ?? "file") as FileIconDto["kind"],
      extension: thumb?.extension ?? null,
    };
  }

  let selectedIdx = $state(0);
  let busy = $state(false);
  let showGlobInput = $state(false);
  let globValue = $state("");
  let showProfileInput = $state(false);
  let profileName = $state("");
  // Thumbnail cache keyed by path — kept in component state so
  // right-pane re-renders don't re-invoke the IPC. Small (one
  // entry per rendered conflict) so no eviction needed.
  let thumbs = $state<Record<string, ThumbnailDto>>({});

  // Derivations off the store.
  const rows = $derived($conflictBatch);
  const selected = $derived<ConflictBatchRow | null>(
    rows[selectedIdx] ?? rows[0] ?? null,
  );
  const firstJobId = $derived(rows[0]?.jobId ?? null);
  const pendingCount = $derived(
    rows.filter((r) => r.state.phase === "pending").length,
  );
  const resolvedCount = $derived(rows.length - pendingCount);

  // ---- Helpers ----

  function basename(p: string): string {
    const idx = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    return idx >= 0 ? p.slice(idx + 1) : p;
  }

  function extOf(p: string): string {
    const name = basename(p);
    const dot = name.lastIndexOf(".");
    return dot > 0 ? name.slice(dot + 1).toLowerCase() : "";
  }

  function fmtDate(ms: number | null): string {
    if (ms === null) return "—";
    try {
      return new Date(ms).toLocaleString();
    } catch {
      return "—";
    }
  }

  function withSuffix(name: string): string {
    const dot = name.lastIndexOf(".");
    const hasExt = dot > 0 && dot < name.length - 1;
    const base = hasExt ? name.slice(0, dot) : name;
    const ext = hasExt ? name.slice(dot) : "";
    return `${base}_2${ext}`;
  }

  async function ensureThumb(path: string, maxDim: number): Promise<void> {
    if (thumbs[path]) return;
    try {
      const dto = await thumbnailFor(path, maxDim);
      thumbs = { ...thumbs, [path]: dto };
    } catch (e) {
      // Non-fatal — leave the slot empty so the icon fallback
      // renders from the classify() call at render time.
      console.warn("[thumbnail]", path, e);
    }
  }

  // Lazy-load thumbs for rows in view. On every row add / select,
  // prefetch the current + neighbours' thumbs.
  $effect(() => {
    const currentRows = $conflictBatch;
    const idx = selectedIdx;
    untrack(() => {
      currentRows
        .slice(Math.max(0, idx - 3), Math.min(currentRows.length, idx + 6))
        .forEach((r) => {
          void ensureThumb(r.dst, 240);
          void ensureThumb(r.src, 240);
        });
    });
  });

  // ---- Per-row actions ----

  async function resolveRow(
    row: ConflictBatchRow,
    resolution: "overwrite" | "skip" | "abort" | "rename",
    renameTo: string | null = null,
  ): Promise<void> {
    if (busy || row.id === null || row.state.phase !== "pending") return;
    busy = true;
    try {
      await resolveCollision(row.id, resolution, renameTo, false);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function resolveRowNewerWins(row: ConflictBatchRow): Promise<void> {
    const s = row.srcModifiedMs ?? 0;
    const d = row.dstModifiedMs ?? 0;
    await resolveRow(row, s > d ? "overwrite" : "skip");
  }

  async function resolveRowLargerWins(row: ConflictBatchRow): Promise<void> {
    const s = row.srcSize ?? 0;
    const d = row.dstSize ?? 0;
    await resolveRow(row, s > d ? "overwrite" : "skip");
  }

  async function resolveRowKeepBoth(row: ConflictBatchRow): Promise<void> {
    const fresh = withSuffix(basename(row.dst));
    await resolveRow(row, "rename", fresh);
  }

  // ---- Bulk actions ----

  /// Apply `resolution` to every currently-pending row matching
  /// `pattern` (basename-style glob); also install the rule on the
  /// running job so future collisions auto-resolve.
  async function applyPattern(
    pattern: string,
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    if (firstJobId === null || busy) return;
    busy = true;
    try {
      await addConflictRule(firstJobId, pattern, resolution);
      // Drain pending rows that match the pattern RIGHT NOW —
      // the runner's rule check only fires on future
      // collisions (already-registered prompts were parked
      // before the rule landed).
      const matchingPending = rows.filter(
        (r) => r.state.phase === "pending" && matchGlob(pattern, basename(r.src)),
      );
      for (const row of matchingPending) {
        if (row.id === null) continue;
        await applyRuleToRow(row, resolution);
      }
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function applyRuleToRow(
    row: ConflictBatchRow,
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    if (row.id === null) return;
    switch (resolution) {
      case "skip":
        await resolveCollision(row.id, "skip", null, false);
        break;
      case "overwrite":
        await resolveCollision(row.id, "overwrite", null, false);
        break;
      case "overwrite-if-newer": {
        const s = row.srcModifiedMs ?? 0;
        const d = row.dstModifiedMs ?? 0;
        await resolveCollision(row.id, s > d ? "overwrite" : "skip", null, false);
        break;
      }
      case "overwrite-if-larger": {
        const s = row.srcSize ?? 0;
        const d = row.dstSize ?? 0;
        await resolveCollision(row.id, s > d ? "overwrite" : "skip", null, false);
        break;
      }
      case "keep-both": {
        const fresh = withSuffix(basename(row.dst));
        await resolveCollision(row.id, "rename", fresh, false);
        break;
      }
    }
  }

  /// Simple glob matcher — `*` matches any chars except separators.
  /// Used only for "apply to already-rendered pending rows"; the
  /// authoritative rule engine lives in Rust.
  function matchGlob(pattern: string, subject: string): boolean {
    const escaped = pattern
      .split("")
      .map((ch) => {
        if (ch === "*") return "[^/]*";
        if (ch === "?") return "[^/]";
        if (".+^$()|{}[]\\".includes(ch)) return `\\${ch}`;
        return ch;
      })
      .join("");
    try {
      const re = new RegExp(`^${escaped}$`);
      return re.test(subject);
    } catch {
      return false;
    }
  }

  async function applySelected(
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    if (!selected) return;
    busy = true;
    try {
      await applyRuleToRow(selected, resolution);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function applyExtension(
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    if (!selected) return;
    const ext = extOf(selected.src);
    const pattern = ext ? `*.${ext}` : basename(selected.src);
    await applyPattern(pattern, resolution);
  }

  async function applyGlob(
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    if (!globValue.trim()) return;
    await applyPattern(globValue.trim(), resolution);
    showGlobInput = false;
    globValue = "";
  }

  async function applyRemaining(
    resolution: ConflictRuleResolution,
  ): Promise<void> {
    await applyPattern("*", resolution);
  }

  async function saveAsProfile(): Promise<void> {
    if (firstJobId === null || !profileName.trim()) return;
    busy = true;
    try {
      // Reconstruct the profile from the rules the user has been
      // clicking up — `addConflictRule` already pushed each one
      // into the job's live set, so re-fetching from the server
      // keeps the save atomic with the current state.
      const profile = await currentConflictRules(firstJobId);
      await saveConflictProfile(profileName.trim(), profile);
      pushToast("info", "conflict-batch-profile-saved");
      showProfileInput = false;
      profileName = "";
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  function closeDialog(): void {
    if (firstJobId !== null) clearConflictBatch(firstJobId);
    else clearConflictBatch();
  }

  // Clear selection when rows drop below the selected index.
  $effect(() => {
    if (selectedIdx >= rows.length && rows.length > 0) selectedIdx = rows.length - 1;
    if (rows.length === 0) selectedIdx = 0;
  });

  onMount(() => {
    // Preload selected-row thumbs at mount time so the first paint
    // has them ready.
    if (selected) {
      void ensureThumb(selected.src, 240);
      void ensureThumb(selected.dst, 240);
    }
  });

  onDestroy(() => {
    thumbs = {};
  });
</script>

{#if rows.length > 0}
  <div class="backdrop" role="presentation">
    {#key $i18nVersion}
      <div
        class="modal"
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="cbm-title"
        tabindex="-1"
      >
        <header class="head">
          <Icon name="info" size={18} />
          <h2 id="cbm-title">
            {t("conflict-batch-title", {
              count: String(rows.length),
              jobname: basename(rows[0]?.dst ?? ""),
            })}
          </h2>
          <span class="count-badges">
            <span class="badge pending">{t("conflict-batch-state-pending")}: {pendingCount}</span>
            <span class="badge resolved">{t("conflict-batch-state-resolved")}: {resolvedCount}</span>
          </span>
          <button
            class="icon-btn close"
            type="button"
            onclick={closeDialog}
            aria-label={t("conflict-batch-close")}
            title={t("conflict-batch-close")}
          >
            ✕
          </button>
        </header>

        <div class="body">
          <!-- Left rail -->
          <aside class="rail" aria-label={t("conflict-batch-source-vs-destination")}>
            <ul class="rail-list" role="listbox">
              {#each rows as row, i (row.src + "|" + row.dst)}
                <li
                  role="option"
                  aria-selected={selectedIdx === i}
                  class="rail-row"
                  class:selected={selectedIdx === i}
                  class:resolved={row.state.phase === "resolved"}
                >
                  <button
                    type="button"
                    class="rail-btn"
                    onclick={() => (selectedIdx = i)}
                  >
                    <div class="thumb-wrap small">
                      {#if thumbs[row.dst]?.kind === "image" && thumbs[row.dst].dataUrl}
                        <img src={thumbs[row.dst].dataUrl} alt="" />
                      {:else}
                        <FileKindIcon info={iconDtoFor(thumbs[row.dst])} size={24} />
                      {/if}
                    </div>
                    <div class="rail-meta">
                      <div class="rail-name" title={row.dst}>
                        {basename(row.dst)}
                      </div>
                      <div class="rail-sub">
                        {#if row.state.phase === "pending"}
                          <span class="badge-tiny pending">
                            {t("conflict-batch-state-pending")}
                          </span>
                        {:else}
                          <span class="badge-tiny resolved">
                            ✓ {row.state.action}
                          </span>
                          {#if row.state.matchedRulePattern}
                            <span class="rule-hint">
                              {t("conflict-batch-matched-rule", {
                                rule: row.state.matchedRulePattern,
                                action: row.state.action,
                              })}
                            </span>
                          {/if}
                        {/if}
                      </div>
                    </div>
                  </button>
                </li>
              {/each}
            </ul>
            {#if rows.length === 0}
              <p class="empty">{t("conflict-batch-empty")}</p>
            {/if}
          </aside>

          <!-- Right pane -->
          <section class="detail">
            {#if selected}
              <div class="panes">
                <div class="pane">
                  <h3>{t("conflict-batch-source-label")}</h3>
                  <div class="thumb-wrap large">
                    {#if thumbs[selected.src]?.kind === "image" && thumbs[selected.src].dataUrl}
                      <img src={thumbs[selected.src].dataUrl} alt="" />
                    {:else}
                      <FileKindIcon info={iconDtoFor(thumbs[selected.src])} size={64} />
                    {/if}
                  </div>
                  <dl>
                    <dt>{t("conflict-batch-size-label")}</dt>
                    <dd>{selected.srcSize !== null ? formatBytes(selected.srcSize) : "—"}</dd>
                    <dt>{t("conflict-batch-modified-label")}</dt>
                    <dd>{fmtDate(selected.srcModifiedMs)}</dd>
                  </dl>
                </div>
                <div class="pane">
                  <h3>{t("conflict-batch-destination-label")}</h3>
                  <div class="thumb-wrap large">
                    {#if thumbs[selected.dst]?.kind === "image" && thumbs[selected.dst].dataUrl}
                      <img src={thumbs[selected.dst].dataUrl} alt="" />
                    {:else}
                      <FileKindIcon info={iconDtoFor(thumbs[selected.dst])} size={64} />
                    {/if}
                  </div>
                  <dl>
                    <dt>{t("conflict-batch-size-label")}</dt>
                    <dd>{selected.dstSize !== null ? formatBytes(selected.dstSize) : "—"}</dd>
                    <dt>{t("conflict-batch-modified-label")}</dt>
                    <dd>{fmtDate(selected.dstModifiedMs)}</dd>
                  </dl>
                </div>
              </div>

              {#if selected.state.phase === "pending"}
                <div class="row-actions" aria-label={t("conflict-batch-state-pending")}>
                  <button
                    type="button"
                    class="btn"
                    disabled={busy}
                    onclick={() => resolveRow(selected, "overwrite")}
                  >{t("conflict-batch-action-overwrite")}</button>
                  <button
                    type="button"
                    class="btn"
                    disabled={busy}
                    onclick={() => resolveRow(selected, "skip")}
                  >{t("conflict-batch-action-skip")}</button>
                  <button
                    type="button"
                    class="btn"
                    disabled={busy}
                    onclick={() => resolveRowKeepBoth(selected)}
                  >{t("conflict-batch-action-keep-both")}</button>
                  <button
                    type="button"
                    class="btn"
                    disabled={busy}
                    onclick={() => resolveRowNewerWins(selected)}
                  >{t("conflict-batch-action-newer-wins")}</button>
                  <button
                    type="button"
                    class="btn"
                    disabled={busy}
                    onclick={() => resolveRowLargerWins(selected)}
                  >{t("conflict-batch-action-larger-wins")}</button>
                </div>
              {/if}
            {/if}
          </section>
        </div>

        <!-- Bulk action bar -->
        <footer class="bulk-bar">
          <div class="bulk-group">
            <label class="bulk-label" for="cbm-sel-action">
              {t("conflict-batch-bulk-apply-selected")}
            </label>
            <select
              id="cbm-sel-action"
              disabled={busy || !selected || selected.state.phase !== "pending"}
              onchange={(e) => {
                const res = (e.target as HTMLSelectElement).value as ConflictRuleResolution;
                if (res) void applySelected(res);
                (e.target as HTMLSelectElement).value = "";
              }}
            >
              <option value="">—</option>
              <option value="skip">{t("conflict-batch-action-skip")}</option>
              <option value="overwrite">{t("conflict-batch-action-overwrite")}</option>
              <option value="overwrite-if-newer">{t("conflict-batch-action-newer-wins")}</option>
              <option value="overwrite-if-larger">{t("conflict-batch-action-larger-wins")}</option>
              <option value="keep-both">{t("conflict-batch-action-keep-both")}</option>
            </select>
          </div>
          <div class="bulk-group">
            <label class="bulk-label" for="cbm-ext-action">
              {t("conflict-batch-bulk-apply-extension")}
            </label>
            <select
              id="cbm-ext-action"
              disabled={busy || !selected}
              onchange={(e) => {
                const res = (e.target as HTMLSelectElement).value as ConflictRuleResolution;
                if (res) void applyExtension(res);
                (e.target as HTMLSelectElement).value = "";
              }}
            >
              <option value="">—</option>
              <option value="skip">{t("conflict-batch-action-skip")}</option>
              <option value="overwrite">{t("conflict-batch-action-overwrite")}</option>
              <option value="overwrite-if-newer">{t("conflict-batch-action-newer-wins")}</option>
              <option value="overwrite-if-larger">{t("conflict-batch-action-larger-wins")}</option>
              <option value="keep-both">{t("conflict-batch-action-keep-both")}</option>
            </select>
          </div>
          <div class="bulk-group">
            <button
              type="button"
              class="btn"
              disabled={busy}
              onclick={() => (showGlobInput = !showGlobInput)}
            >{t("conflict-batch-bulk-apply-glob")}</button>
          </div>
          <div class="bulk-group">
            <label class="bulk-label" for="cbm-all-action">
              {t("conflict-batch-bulk-apply-remaining")}
            </label>
            <select
              id="cbm-all-action"
              disabled={busy || pendingCount === 0}
              onchange={(e) => {
                const res = (e.target as HTMLSelectElement).value as ConflictRuleResolution;
                if (res) void applyRemaining(res);
                (e.target as HTMLSelectElement).value = "";
              }}
            >
              <option value="">—</option>
              <option value="skip">{t("conflict-batch-action-skip")}</option>
              <option value="overwrite">{t("conflict-batch-action-overwrite")}</option>
              <option value="overwrite-if-newer">{t("conflict-batch-action-newer-wins")}</option>
              <option value="overwrite-if-larger">{t("conflict-batch-action-larger-wins")}</option>
              <option value="keep-both">{t("conflict-batch-action-keep-both")}</option>
            </select>
          </div>
          <div class="bulk-group push-right">
            <button
              type="button"
              class="btn"
              disabled={busy || rows.length === 0}
              onclick={() => (showProfileInput = !showProfileInput)}
            >{t("conflict-batch-save-profile")}</button>
          </div>
        </footer>

        {#if showGlobInput}
          <div class="glob-row">
            <input
              type="text"
              placeholder={t("conflict-batch-bulk-glob-placeholder")}
              bind:value={globValue}
              disabled={busy}
            />
            <select
              disabled={busy}
              onchange={(e) => {
                const res = (e.target as HTMLSelectElement).value as ConflictRuleResolution;
                if (res) void applyGlob(res);
                (e.target as HTMLSelectElement).value = "";
              }}
            >
              <option value="">—</option>
              <option value="skip">{t("conflict-batch-action-skip")}</option>
              <option value="overwrite">{t("conflict-batch-action-overwrite")}</option>
              <option value="overwrite-if-newer">{t("conflict-batch-action-newer-wins")}</option>
              <option value="overwrite-if-larger">{t("conflict-batch-action-larger-wins")}</option>
              <option value="keep-both">{t("conflict-batch-action-keep-both")}</option>
            </select>
          </div>
        {/if}

        {#if showProfileInput}
          <div class="profile-row">
            <input
              type="text"
              placeholder={t("conflict-batch-profile-placeholder")}
              bind:value={profileName}
              disabled={busy}
            />
            <button
              type="button"
              class="btn primary"
              disabled={busy || !profileName.trim()}
              onclick={saveAsProfile}
            >{t("conflict-batch-save-profile")}</button>
          </div>
        {/if}
      </div>
    {/key}
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.42);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 95;
  }
  .modal {
    width: min(1040px, 96vw);
    max-height: 86vh;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 12px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.28);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .head {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }
  .head h2 {
    margin: 0;
    flex: 1;
    font-size: 14px;
    font-weight: 600;
  }
  .count-badges {
    display: flex;
    gap: 6px;
  }
  .badge {
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 999px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.05));
  }
  .badge.pending {
    background: rgba(228, 160, 64, 0.18);
    color: var(--warn, #e4a040);
  }
  .badge.resolved {
    background: rgba(63, 175, 106, 0.18);
    color: var(--ok, #3faf6a);
  }
  .close {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    font-size: 16px;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .close:hover {
    background: var(--hover, rgba(0, 0, 0, 0.06));
  }
  .body {
    display: flex;
    flex: 1;
    min-height: 0;
  }
  .rail {
    width: 280px;
    min-width: 240px;
    border-right: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    overflow-y: auto;
    padding: 4px 0;
  }
  .rail-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .rail-row {
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.15));
  }
  .rail-row.selected {
    background: var(--row-selected, rgba(79, 140, 255, 0.14));
  }
  .rail-row.resolved {
    opacity: 0.72;
  }
  .rail-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    text-align: left;
    padding: 6px 10px;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
  }
  .rail-btn:hover {
    background: var(--hover, rgba(0, 0, 0, 0.04));
  }
  .thumb-wrap.small {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    border-radius: 4px;
    flex-shrink: 0;
    overflow: hidden;
  }
  .thumb-wrap.small img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .thumb-wrap.large {
    width: 200px;
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    border-radius: 8px;
    overflow: hidden;
  }
  .thumb-wrap.large img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }
  .rail-meta {
    flex: 1;
    min-width: 0;
  }
  .rail-name {
    font-size: 12px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .rail-sub {
    display: flex;
    gap: 6px;
    align-items: center;
    margin-top: 2px;
    font-size: 10.5px;
    color: var(--fg-dim, #6a6a6a);
  }
  .badge-tiny {
    font-size: 10px;
    padding: 1px 6px;
    border-radius: 999px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.05));
  }
  .badge-tiny.pending {
    background: rgba(228, 160, 64, 0.18);
    color: var(--warn, #e4a040);
  }
  .badge-tiny.resolved {
    background: rgba(63, 175, 106, 0.18);
    color: var(--ok, #3faf6a);
  }
  .rule-hint {
    font-size: 10px;
    color: var(--fg-dim, #6a6a6a);
    font-style: italic;
  }
  .detail {
    flex: 1;
    padding: 14px 16px;
    overflow-y: auto;
  }
  .panes {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
  }
  .pane {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 8px;
  }
  .pane h3 {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    color: var(--fg-dim, #6a6a6a);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .pane dl {
    margin: 0;
    display: grid;
    grid-template-columns: auto 1fr;
    column-gap: 8px;
    row-gap: 2px;
    font-size: 11.5px;
  }
  .pane dt {
    color: var(--fg-dim, #6a6a6a);
  }
  .pane dd {
    margin: 0;
  }
  .row-actions {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
    margin-top: 14px;
    justify-content: flex-end;
  }
  .bulk-bar {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
    padding: 10px 14px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: var(--surface-alt, rgba(0, 0, 0, 0.025));
  }
  .bulk-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .push-right {
    margin-left: auto;
  }
  .bulk-label {
    font-size: 11.5px;
    color: var(--fg-dim, #6a6a6a);
  }
  .glob-row,
  .profile-row {
    display: flex;
    gap: 6px;
    padding: 10px 14px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }
  .glob-row input,
  .profile-row input {
    flex: 1;
    padding: 4px 8px;
    font-size: 12px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: inherit;
  }
  select {
    font-size: 12px;
    padding: 3px 6px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: inherit;
  }
  .btn {
    font-size: 12px;
    padding: 5px 10px;
    border-radius: 5px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    color: inherit;
    cursor: pointer;
  }
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .btn.primary {
    background: var(--accent, #3261ff);
    color: white;
    border-color: transparent;
  }
  .empty {
    padding: 16px;
    font-size: 12px;
    color: var(--fg-dim, #6a6a6a);
    text-align: center;
  }
</style>
