<!--
  Phase 28 — tray-resident Drop Stack.

  Standalone Tauri window (label `dropstack`) mounted by
  `dropstack-main.ts`. Shows the persistent list of staged paths and
  offers the two dispatch verbs (Copy all to… / Move all to…) plus
  per-row Remove + global Clear.

  State flows:
  - Initial list: `invoke("dropstack_list")` on mount.
  - Live updates: listen for `dropstack-changed` (payload is the
    whole list, already-DTO'd) and `dropstack-path-missing` (toast
    per dropped path).
  - Drag in: the Tauri window-level drag-drop event delivers
    absolute file paths when the user drops onto this window;
    pushed into the registry via `dropstack_add`.
  - Dispatch: destination picker → `dropstack_copy_all_to` /
    `dropstack_move_all_to`.

  Drag-out-to-another-app is a Phase 29 native-DnD polish follow-up;
  for Phase 28 the dispatch verbs are the only out-path.
-->
<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { t } from "../i18n";
  import type { DropStackEntryDto, DropStackPathMissingDto } from "../types";
  import { applyDragThumbnail } from "../dragThumbnail";

  let entries: DropStackEntryDto[] = $state([]);
  let toasts: string[] = $state([]);
  let loading: boolean = $state(true);
  let busy: boolean = $state(false);
  let error: string | null = $state(null);

  const unlistenFns: UnlistenFn[] = [];

  function basename(p: string): string {
    const m = p.match(/[^\\/]+$/);
    return m ? m[0] : p;
  }

  function parentOf(p: string): string {
    const idx = Math.max(p.lastIndexOf("\\"), p.lastIndexOf("/"));
    return idx > 0 ? p.slice(0, idx) : "";
  }

  async function refresh() {
    try {
      entries = await invoke<DropStackEntryDto[]>("dropstack_list");
      error = null;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function removePath(path: string) {
    try {
      await invoke("dropstack_remove", { path });
    } catch (e) {
      error = String(e);
    }
  }

  async function clearAll() {
    if (!confirm(t("dropstack-clear"))) return;
    try {
      await invoke("dropstack_clear");
    } catch (e) {
      error = String(e);
    }
  }

  async function pickDestinationAnd(kind: "copy" | "move") {
    const dst = await openDialog({ directory: true, multiple: false });
    if (typeof dst !== "string" || dst.length === 0) return;
    busy = true;
    error = null;
    try {
      if (kind === "copy") {
        await invoke("dropstack_copy_all_to", { destination: dst });
      } else {
        await invoke("dropstack_move_all_to", { destination: dst });
      }
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function pushToast(message: string) {
    toasts = [...toasts, message];
    setTimeout(() => {
      toasts = toasts.slice(1);
    }, 4500);
  }

  onMount(() => {
    void refresh();
    (async () => {
      unlistenFns.push(
        await listen<DropStackEntryDto[]>("dropstack-changed", (evt) => {
          entries = evt.payload ?? [];
        }),
      );
      unlistenFns.push(
        await listen<DropStackPathMissingDto>(
          "dropstack-path-missing",
          (evt) => {
            pushToast(
              t("dropstack-path-missing-toast", { path: evt.payload.path }),
            );
          },
        ),
      );

      // Tauri 2 surfaces drag-drop events on the window itself;
      // `drag-drop` fires with the list of absolute paths once the
      // user lets go. We push them into the registry; the
      // `dropstack-changed` listener above does the rest.
      unlistenFns.push(
        await getCurrentWindow().onDragDropEvent((evt) => {
          const payload = evt.payload as unknown as {
            type: string;
            paths?: string[];
          };
          if (payload.type === "drop" && Array.isArray(payload.paths)) {
            void invoke("dropstack_add", { paths: payload.paths });
          }
        }),
      );
    })();
  });

  onDestroy(() => {
    for (const un of unlistenFns) {
      try {
        un();
      } catch {
        /* ignore */
      }
    }
  });
</script>

<main class="dropstack">
  <header>
    <h1>{t("dropstack-window-title")}</h1>
    <span class="count"
      >{t("dropstack-count", { count: String(entries.length) })}</span
    >
  </header>

  {#if loading}
    <p class="status">…</p>
  {:else if entries.length === 0}
    <div class="empty">
      <p class="empty-title">{t("dropstack-empty-title")}</p>
      <p class="empty-hint">{t("dropstack-empty-hint")}</p>
    </div>
  {:else}
    <ul class="entries">
      {#each entries as e (e.path)}
        <li
          draggable="true"
          ondragstart={(ev) => {
            // Phase 29 — in-app drag source. HTML5 DataTransfer can't
            // create real OS file drags (that's `tauri-plugin-drag`,
            // deferred to Phase 29b); this lets in-window DropTarget
            // components receive the row and renders a themed preview
            // while the user drags. External apps will show the
            // fallback "text" preview until the plugin attaches.
            if (!ev.dataTransfer) return;
            ev.dataTransfer.effectAllowed = "copy";
            ev.dataTransfer.setData("text/plain", e.path);
            ev.dataTransfer.setData(
              "application/x-copythat-paths",
              JSON.stringify([e.path]),
            );
            applyDragThumbnail(ev, [{ label: e.path }]);
          }}
        >
          <div class="row">
            <span class="name" title={e.path}>{basename(e.path)}</span>
            <span class="parent" title={e.path}>{parentOf(e.path)}</span>
          </div>
          <button
            class="remove"
            type="button"
            aria-label={t("dropstack-remove-row")}
            onclick={() => removePath(e.path)}>×</button
          >
        </li>
      {/each}
    </ul>
  {/if}

  <footer>
    <button
      type="button"
      disabled={busy || entries.length === 0}
      onclick={() => pickDestinationAnd("copy")}
      >{t("dropstack-copy-all-to")}</button
    >
    <button
      type="button"
      disabled={busy || entries.length === 0}
      onclick={() => pickDestinationAnd("move")}
      >{t("dropstack-move-all-to")}</button
    >
    <button
      type="button"
      class="secondary"
      disabled={busy || entries.length === 0}
      onclick={clearAll}>{t("dropstack-clear")}</button
    >
  </footer>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <div class="toast-host" aria-live="polite">
    {#each toasts as msg (msg)}
      <div class="toast">{msg}</div>
    {/each}
  </div>
</main>

<style>
  :global(html, body, #app) {
    height: 100%;
    margin: 0;
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      sans-serif;
  }
  .dropstack {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 8px 12px 12px;
    box-sizing: border-box;
    gap: 8px;
    color: var(--fg, #222);
    background: var(--bg, #fff);
  }
  @media (prefers-color-scheme: dark) {
    .dropstack {
      color: #eaeaea;
      background: #1e1e1e;
    }
    .parent {
      color: #a0a0a0;
    }
    button {
      background: #2b2b2b;
      color: #eaeaea;
      border-color: #3a3a3a;
    }
    button:disabled {
      color: #666;
    }
  }
  header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
  }
  header h1 {
    font-size: 14px;
    font-weight: 600;
    margin: 0;
  }
  .count {
    font-size: 12px;
    opacity: 0.7;
  }
  .empty {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 16px;
    gap: 6px;
  }
  .empty-title {
    font-weight: 600;
    font-size: 13px;
    margin: 0;
  }
  .empty-hint {
    font-size: 12px;
    opacity: 0.75;
    margin: 0;
  }
  .entries {
    list-style: none;
    margin: 0;
    padding: 0;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .entries li {
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: 4px;
  }
  .entries li:hover {
    background: rgba(0, 0, 0, 0.05);
  }
  @media (prefers-color-scheme: dark) {
    .entries li:hover {
      background: rgba(255, 255, 255, 0.06);
    }
  }
  .row {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .name {
    font-size: 13px;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .parent {
    font-size: 11px;
    color: #666;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .remove {
    background: transparent;
    border: none;
    color: #888;
    font-size: 16px;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 3px;
  }
  .remove:hover {
    background: rgba(255, 0, 0, 0.1);
    color: #c00;
  }
  footer {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }
  button {
    font-size: 12px;
    padding: 6px 10px;
    border: 1px solid rgba(0, 0, 0, 0.15);
    border-radius: 4px;
    background: #f4f4f4;
    cursor: pointer;
  }
  button.secondary {
    margin-left: auto;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .error {
    font-size: 12px;
    color: #c00;
    margin: 0;
  }
  .toast-host {
    position: fixed;
    bottom: 12px;
    left: 12px;
    right: 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    pointer-events: none;
  }
  .toast {
    background: #333;
    color: white;
    padding: 6px 10px;
    border-radius: 4px;
    font-size: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }
</style>
