<!--
  Phase 29 — destination picker with spring-load cascade.

  A one-level-at-a-time folder browser that can be used in any flow that
  asks "where should we copy this?". Key behavior:

  - Left column = breadcrumb (root → current).
  - Right column = immediate children of the current folder.
  - Each child row is a `DropTarget`. Hovering with a drag for
    `springLoadDelayMs` (default 650 ms) navigates into it — the
    cascade continues arbitrarily deep until the user either drops or
    drags out.
  - Children marked not-writable paint the error border + tooltip.
  - `onPick(path)` fires when the user either clicks "Use this
    folder" on the current breadcrumb or drops onto a writable row.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import DropTarget from "./DropTarget.svelte";
  import { listDirectory, listRoots } from "../ipc";
  import { t } from "../i18n";
  import type { DirChildDto } from "../types";

  interface Props {
    /** Starting directory. When null, the picker shows roots. */
    initialPath?: string | null;
    /** Spring-load delay in ms. 0 disables cascade. */
    springLoadDelayMs?: number;
    /** Paint the red border + tooltip on invalid targets. */
    highlightInvalid?: boolean;
    /** Fires with the chosen absolute path. */
    onPick?: (path: string) => void;
    /** Fires when the picker is closed without a choice. */
    onCancel?: () => void;
  }

  let {
    initialPath = null,
    springLoadDelayMs = 650,
    highlightInvalid = true,
    onPick,
    onCancel,
  }: Props = $props();

  // svelte-ignore state_referenced_locally
  let currentPath = $state<string | null>(initialPath);
  let crumbs = $state<string[]>([]);
  let children = $state<DirChildDto[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  function computeCrumbs(p: string | null): string[] {
    if (p === null) return [];
    // Split on either separator; preserve drive letter on Windows.
    const norm = p.replace(/\\/g, "/");
    const parts = norm.split("/").filter((x) => x.length > 0);
    const out: string[] = [];
    for (let i = 0; i < parts.length; i++) {
      if (i === 0 && /^[A-Za-z]:$/.test(parts[0])) {
        out.push(parts[0] + "\\");
      } else {
        const prev = out.length > 0 ? out[out.length - 1] : "";
        const sep = prev.endsWith("\\") || prev.endsWith("/") ? "" : "/";
        out.push(prev + sep + parts[i]);
      }
    }
    return out;
  }

  async function refresh() {
    loading = true;
    error = null;
    try {
      if (currentPath === null) {
        children = await listRoots();
        crumbs = [];
      } else {
        children = await listDirectory(currentPath);
        crumbs = computeCrumbs(currentPath);
      }
    } catch (e) {
      error = String(e);
      children = [];
    } finally {
      loading = false;
    }
  }

  function navigateInto(path: string) {
    currentPath = path;
    void refresh();
  }

  function navigateUp() {
    if (currentPath === null) return;
    const norm = currentPath.replace(/\\/g, "/");
    const idx = norm.lastIndexOf("/");
    if (idx <= 0) {
      // Already at a root.
      currentPath = null;
    } else {
      let next = norm.slice(0, idx);
      // Windows: "C:" alone isn't a valid path; restore trailing slash.
      if (/^[A-Za-z]:$/.test(next)) next = next + "\\";
      currentPath = next;
    }
    void refresh();
  }

  function pickCurrent() {
    if (currentPath !== null) onPick?.(currentPath);
  }

  function handleChildDrop(child: DirChildDto) {
    if (!child.writable && highlightInvalid) return;
    onPick?.(child.path);
  }

  onMount(() => {
    void refresh();
  });
</script>

<div class="destpicker" role="dialog" aria-label={t("dropzone-picker-title")}>
  <header>
    <button type="button" class="up" onclick={navigateUp} disabled={currentPath === null}>
      ← {t("dropzone-picker-up")}
    </button>
    <nav class="crumbs" aria-label={t("dropzone-picker-path")}>
      {#if currentPath === null}
        <span class="crumb root">{t("dropzone-picker-root")}</span>
      {:else}
        {#each crumbs as c, i (c)}
          <button
            type="button"
            class="crumb"
            onclick={() => {
              currentPath = c;
              void refresh();
            }}
            disabled={i === crumbs.length - 1}
          >
            {c.split(/[\\/]/).filter(Boolean).pop() || c}
          </button>
          {#if i < crumbs.length - 1}<span class="sep">/</span>{/if}
        {/each}
      {/if}
    </nav>
    <button
      type="button"
      class="use"
      onclick={pickCurrent}
      disabled={currentPath === null}
    >
      {t("dropzone-picker-use-this")}
    </button>
  </header>

  {#if loading}
    <p class="status">…</p>
  {:else if error}
    <p class="error" role="alert">{error}</p>
  {:else if children.length === 0}
    <p class="empty">{t("dropzone-picker-empty")}</p>
  {:else}
    <ul class="children">
      {#each children as c (c.path)}
        <li>
          <DropTarget
            invalid={highlightInvalid && !c.writable}
            invalidReason={t("dropzone-invalid-readonly")}
            springLoadMs={springLoadDelayMs}
            onSpringLoad={() => navigateInto(c.path)}
            onDrop={() => handleChildDrop(c)}
          >
            <button
              class="child"
              type="button"
              ondblclick={() => navigateInto(c.path)}
              onclick={() => navigateInto(c.path)}
            >
              <span class="icon" aria-hidden="true">📁</span>
              <span class="name">{c.name}</span>
            </button>
          </DropTarget>
        </li>
      {/each}
    </ul>
  {/if}

  <footer>
    <button type="button" onclick={() => onCancel?.()} class="cancel">
      {t("dropzone-picker-cancel")}
    </button>
  </footer>
</div>

<style>
  .destpicker {
    display: flex;
    flex-direction: column;
    min-width: 360px;
    max-height: 480px;
    background: var(--bg, #fff);
    color: var(--fg, #222);
    border-radius: 6px;
    overflow: hidden;
  }
  header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-bottom: 1px solid var(--border, #e5e7eb);
  }
  .crumbs {
    display: flex;
    align-items: center;
    gap: 4px;
    flex: 1;
    overflow-x: auto;
    font-size: 12px;
  }
  .crumb {
    background: none;
    border: none;
    color: var(--accent, #3b82f6);
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 3px;
  }
  .crumb:disabled {
    color: var(--fg, #222);
    cursor: default;
  }
  .sep {
    opacity: 0.5;
  }
  .up,
  .use {
    font-size: 12px;
    padding: 4px 10px;
    border: 1px solid var(--border, #d1d5db);
    background: var(--bg, #fff);
    border-radius: 4px;
    cursor: pointer;
  }
  .up:disabled,
  .use:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .children {
    flex: 1;
    list-style: none;
    margin: 0;
    padding: 4px;
    overflow-y: auto;
  }
  .children li {
    margin: 2px 0;
  }
  .child {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 10px;
    background: none;
    border: none;
    color: inherit;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
    border-radius: 3px;
  }
  .child:hover {
    background: color-mix(in srgb, var(--accent, #3b82f6) 8%, transparent);
  }
  footer {
    padding: 6px 10px;
    border-top: 1px solid var(--border, #e5e7eb);
    display: flex;
    justify-content: flex-end;
  }
  .cancel {
    font-size: 12px;
    padding: 4px 10px;
    border: 1px solid var(--border, #d1d5db);
    background: var(--bg, #fff);
    border-radius: 4px;
    cursor: pointer;
  }
  .status,
  .empty {
    padding: 16px;
    text-align: center;
    opacity: 0.6;
    font-size: 12px;
    margin: 0;
  }
  .error {
    padding: 12px;
    color: var(--danger, #dc2626);
    font-size: 12px;
    margin: 0;
  }
</style>
