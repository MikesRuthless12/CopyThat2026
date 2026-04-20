<!--
  Phase 8 — error-log drawer.

  Shown when `$errorLogDrawerOpen` is true (Footer's "errors" link
  flips it on). Fetches the log via `error_log()` on mount and when
  explicitly refreshed; exports CSV / TXT via the Tauri dialog
  plugin's save dialog.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";

  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { clearErrorLog, errorLog, errorLogExport } from "../ipc";
  import {
    closeErrorLogDrawer,
    errorLogDrawerOpen,
    pushToast,
  } from "../stores";
  import type { LoggedErrorDto } from "../types";

  let entries = $state<LoggedErrorDto[]>([]);
  let busy = $state(false);

  async function refresh() {
    busy = true;
    try {
      entries = await errorLog();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  onMount(refresh);

  // Refresh when the drawer is toggled on.
  $effect(() => {
    if ($errorLogDrawerOpen) {
      refresh();
    }
  });

  async function exportAs(format: "csv" | "txt") {
    const dest = await save({
      defaultPath:
        format === "csv"
          ? "copythat-errors.csv"
          : "copythat-errors.txt",
      filters: [
        {
          name: format === "csv" ? "CSV" : "Text",
          extensions: [format],
        },
      ],
    });
    if (!dest || typeof dest !== "string") return;
    try {
      await errorLogExport(format, dest);
      pushToast("success", "toast-error-log-exported");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function clear() {
    try {
      await clearErrorLog();
      entries = [];
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function fmtDate(ms: number): string {
    try {
      return new Date(ms).toLocaleString();
    } catch {
      return String(ms);
    }
  }
</script>

{#if $errorLogDrawerOpen}
  <aside class="drawer" aria-label={t("error-log-title")}>
    <header>
      <h2>{t("error-log-title")}</h2>
      <button
        class="close"
        type="button"
        aria-label={t("action-close")}
        onclick={closeErrorLogDrawer}
      >
        <Icon name="x" size={16} />
      </button>
    </header>

    <div class="bar">
      <button
        type="button"
        class="secondary"
        onclick={() => exportAs("csv")}
        disabled={busy || entries.length === 0}
      >
        {t("error-log-export-csv")}
      </button>
      <button
        type="button"
        class="secondary"
        onclick={() => exportAs("txt")}
        disabled={busy || entries.length === 0}
      >
        {t("error-log-export-txt")}
      </button>
      <button
        type="button"
        class="secondary"
        onclick={clear}
        disabled={busy || entries.length === 0}
      >
        {t("error-log-clear")}
      </button>
    </div>

    {#if entries.length === 0}
      <p class="empty">{t("error-log-empty")}</p>
    {:else}
      <table>
        <thead>
          <tr>
            <th>{t("error-log-col-time")}</th>
            <th>{t("error-log-col-job")}</th>
            <th>{t("error-log-col-code")}</th>
            <th>{t("error-log-col-path")}</th>
            <th>{t("error-log-col-resolution")}</th>
            <th>{t("error-log-col-message")}</th>
          </tr>
        </thead>
        <tbody>
          {#each entries as e (e.id)}
            <tr>
              <td>{fmtDate(e.timestampMs)}</td>
              <td>{e.jobId}</td>
              <td>{t(e.localizedKey)}</td>
              <td class="path" title={e.src}>{e.src}</td>
              <td>{e.resolution ?? "—"}</td>
              <td class="msg">{e.message}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </aside>
{/if}

<style>
  .drawer {
    position: fixed;
    right: 0;
    top: 0;
    bottom: 0;
    width: min(720px, 94vw);
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border-left: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    box-shadow: -10px 0 24px rgba(0, 0, 0, 0.22);
    display: flex;
    flex-direction: column;
    z-index: 88;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  button.close {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .bar {
    display: flex;
    gap: 8px;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }

  button {
    font-size: 12px;
    padding: 4px 10px;
    border-radius: 4px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    color: inherit;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .empty {
    padding: 24px 16px;
    color: var(--muted, #6a6a6a);
    font-size: 13px;
    text-align: center;
  }

  table {
    flex: 1;
    width: 100%;
    border-collapse: collapse;
    font-size: 11.5px;
    overflow-y: auto;
    display: block;
  }

  thead {
    position: sticky;
    top: 0;
    background: var(--surface, #ffffff);
    z-index: 1;
  }

  th,
  td {
    text-align: left;
    padding: 5px 10px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.15));
    vertical-align: top;
  }

  td.path,
  td.msg {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 10.5px;
    overflow-wrap: anywhere;
  }
</style>
