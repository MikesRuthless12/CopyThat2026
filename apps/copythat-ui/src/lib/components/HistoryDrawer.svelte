<!--
  Phase 9 — History drawer.

  Lives behind the Footer's "History" button. Lists every completed /
  running job recorded in the SQLite store, with a filter bar on top
  (kind, status, text search) and per-row actions (re-run + open
  detail + export CSV for the full set). Detail view is an inline
  sub-drawer showing the per-item rows.
-->
<script lang="ts">
  import { save } from "@tauri-apps/plugin-dialog";

  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { formatBytes } from "../format";
  import {
    historyExportCsv,
    historyItems,
    historyPurge,
    historyRerun,
    historySearch,
  } from "../ipc";
  import {
    closeHistoryDetail,
    closeHistoryDrawer,
    historyDetailRow,
    historyDrawerOpen,
    openHistoryDetail,
    pushToast,
  } from "../stores";
  import type {
    HistoryFilterDto,
    HistoryItemDto,
    HistoryJobDto,
  } from "../types";

  let rows = $state<HistoryJobDto[]>([]);
  let items = $state<HistoryItemDto[]>([]);
  let busy = $state(false);
  let loadError = $state<string | null>(null);

  // Filter state — the refresh function re-queries when any changes.
  let kindFilter = $state<string>("");
  let statusFilter = $state<string>("");
  let textFilter = $state<string>("");

  async function refresh() {
    busy = true;
    loadError = null;
    try {
      const filter: HistoryFilterDto = {};
      if (kindFilter) filter.kind = kindFilter;
      if (statusFilter) filter.status = statusFilter;
      if (textFilter.trim()) filter.text = textFilter.trim();
      rows = await historySearch(filter);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      loadError = msg;
      if (msg === "history-unavailable") {
        // Leave row list empty; drawer shows the "unavailable"
        // banner below.
      } else {
        pushToast("error", msg);
      }
    } finally {
      busy = false;
    }
  }

  // Refresh when the drawer opens or any filter changes.
  $effect(() => {
    if ($historyDrawerOpen) {
      void refresh();
    }
  });

  // When detail row changes, fetch its items.
  $effect(() => {
    const id = $historyDetailRow;
    if (id !== null) {
      void loadItems(id);
    } else {
      items = [];
    }
  });

  async function loadItems(rowId: number) {
    try {
      items = await historyItems(rowId);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function exportCsv() {
    const dest = await save({
      defaultPath: "copythat-history.csv",
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!dest || typeof dest !== "string") return;
    try {
      await historyExportCsv(dest);
      pushToast("success", "toast-history-exported");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function rerun(row: HistoryJobDto) {
    try {
      await historyRerun(row.rowId);
      pushToast("info", "toast-history-rerun-queued");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function purge30() {
    try {
      const n = await historyPurge(30);
      pushToast("info", t("toast-history-purged", { count: n }));
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function fmtDate(ms: number | null): string {
    if (ms === null) return "—";
    try {
      return new Date(ms).toLocaleString();
    } catch {
      return String(ms);
    }
  }

  function fmtDuration(start: number, end: number | null): string {
    if (end === null) return "—";
    const ms = Math.max(0, end - start);
    if (ms < 1000) return t("duration-ms", { ms });
    const s = Math.floor(ms / 1000);
    if (s < 60) return t("duration-seconds", { s });
    const m = Math.floor(s / 60);
    const r = s % 60;
    return t("duration-minutes-seconds", { m, s: r });
  }

  // Map history wire-format `kind` / `status` strings to localized
  // labels. Unknown values fall back to the raw string so operator-
  // introduced kinds (e.g. a future "verify") still render.
  function localizedKind(kind: string): string {
    const key = `kind-${kind}`;
    const out = t(key);
    return out.startsWith("{") ? kind : out;
  }
  function localizedStatus(status: string): string {
    const key = `status-${status}`;
    const out = t(key);
    return out.startsWith("{") ? status : out;
  }
</script>

{#if $historyDrawerOpen}
  <aside class="drawer" aria-label={t("history-title")}>
    <header>
      <h2>{t("history-title")}</h2>
      <button
        class="close"
        type="button"
        aria-label={t("action-close")}
        onclick={closeHistoryDrawer}
      >
        <Icon name="x" size={16} />
      </button>
    </header>

    <div class="filters">
      <label>
        {t("history-filter-kind")}
        <select bind:value={kindFilter}>
          <option value="">{t("history-filter-any")}</option>
          <option value="copy">{t("kind-copy")}</option>
          <option value="move">{t("kind-move")}</option>
        </select>
      </label>
      <label>
        {t("history-filter-status")}
        <select bind:value={statusFilter}>
          <option value="">{t("history-filter-any")}</option>
          <option value="running">{t("status-running")}</option>
          <option value="succeeded">{t("status-succeeded")}</option>
          <option value="failed">{t("status-failed")}</option>
          <option value="cancelled">{t("status-cancelled")}</option>
        </select>
      </label>
      <label class="text">
        {t("history-filter-text")}
        <input
          type="text"
          bind:value={textFilter}
          placeholder={t("history-search-placeholder")}
          onkeydown={(e) => {
            if (e.key === "Enter") void refresh();
          }}
        />
      </label>
      <button class="secondary" type="button" onclick={refresh} disabled={busy}>
        {t("history-refresh")}
      </button>
      <button
        class="secondary"
        type="button"
        onclick={exportCsv}
        disabled={busy || rows.length === 0}
      >
        {t("history-export-csv")}
      </button>
      <button class="secondary" type="button" onclick={purge30} disabled={busy}>
        {t("history-purge-30")}
      </button>
    </div>

    {#if loadError === "history-unavailable"}
      <p class="notice">{t("history-unavailable")}</p>
    {:else if rows.length === 0 && !busy}
      <p class="empty">{t("history-empty")}</p>
    {:else}
      <table>
        <thead>
          <tr>
            <th>{t("history-col-date")}</th>
            <th>{t("history-col-kind")}</th>
            <th>{t("history-col-src")}</th>
            <th>{t("history-col-dst")}</th>
            <th>{t("history-col-files")}</th>
            <th>{t("history-col-size")}</th>
            <th>{t("history-col-status")}</th>
            <th>{t("history-col-duration")}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {#each rows as row (row.rowId)}
            <tr>
              <td>{fmtDate(row.startedAtMs)}</td>
              <td>{localizedKind(row.kind)}</td>
              <td class="path" title={row.srcRoot}>{row.srcRoot}</td>
              <td class="path" title={row.dstRoot}>{row.dstRoot}</td>
              <td>
                {row.filesOk}
                {#if row.filesFailed > 0}
                  <span class="failed">/ {row.filesFailed}</span>
                {/if}
              </td>
              <td>{formatBytes(row.totalBytes)}</td>
              <td>
                <span class="status" data-status={row.status}>
                  {localizedStatus(row.status)}
                </span>
              </td>
              <td>{fmtDuration(row.startedAtMs, row.finishedAtMs)}</td>
              <td class="actions">
                <button
                  type="button"
                  class="tiny"
                  onclick={() => openHistoryDetail(row.rowId)}
                  aria-label={t("history-detail-open")}
                >
                  <Icon name="info" size={13} />
                </button>
                <button
                  type="button"
                  class="tiny"
                  onclick={() => rerun(row)}
                  aria-label={t("history-rerun")}
                  disabled={row.kind !== "copy" && row.kind !== "move"}
                >
                  <Icon name="refresh" size={13} />
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </aside>

  {#if $historyDetailRow !== null}
    <aside class="detail" aria-label={t("history-detail-title")}>
      <header>
        <h3>{t("history-detail-title")}</h3>
        <button
          class="close"
          type="button"
          aria-label={t("action-close")}
          onclick={closeHistoryDetail}
        >
          <Icon name="x" size={16} />
        </button>
      </header>
      {#if items.length === 0}
        <p class="empty">{t("history-detail-empty")}</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>{t("history-col-src")}</th>
              <th>{t("history-col-size")}</th>
              <th>{t("history-col-status")}</th>
              <th>{t("history-col-error")}</th>
            </tr>
          </thead>
          <tbody>
            {#each items as it}
              <tr>
                <td class="path" title={it.src}>{it.src}</td>
                <td>{formatBytes(it.size)}</td>
                <td>
                  <span class="status" data-status={it.status}>
                    {localizedStatus(it.status)}
                  </span>
                </td>
                <td class="err">{it.errorMsg ?? "—"}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </aside>
  {/if}
{/if}

<style>
  .drawer {
    position: fixed;
    right: 0;
    top: 0;
    bottom: 0;
    width: min(820px, 96vw);
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border-left: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    box-shadow: -10px 0 24px rgba(0, 0, 0, 0.22);
    display: flex;
    flex-direction: column;
    z-index: 86;
  }

  .detail {
    position: fixed;
    right: 0;
    top: 0;
    bottom: 0;
    width: min(620px, 88vw);
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border-left: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    box-shadow: -10px 0 24px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    z-index: 87;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }

  h2,
  h3 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .close {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .filters {
    display: flex;
    align-items: end;
    gap: 8px;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    flex-wrap: wrap;
  }

  .filters label {
    display: flex;
    flex-direction: column;
    gap: 3px;
    font-size: 11px;
    color: var(--muted, #6a6a6a);
  }

  .filters label.text {
    flex: 1;
    min-width: 140px;
  }

  .filters select,
  .filters input {
    padding: 4px 6px;
    font-size: 12px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: inherit;
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

  button.tiny {
    padding: 3px 4px;
    background: transparent;
    border: 1px solid transparent;
  }

  button.tiny:hover:not(:disabled) {
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
  }

  .notice,
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

  td.path {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 10.5px;
    max-width: 180px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  td.err {
    color: var(--error, #c24141);
    font-size: 10.5px;
  }

  td.actions {
    white-space: nowrap;
    display: flex;
    gap: 4px;
  }

  .failed {
    color: var(--error, #c24141);
  }

  .status {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 3px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .status[data-status="succeeded"],
  .status[data-status="ok"] {
    background: rgba(63, 175, 106, 0.16);
    color: var(--ok, #2e7a4a);
  }

  .status[data-status="failed"] {
    background: rgba(217, 87, 87, 0.16);
    color: var(--error, #c24141);
  }

  .status[data-status="cancelled"] {
    background: rgba(228, 160, 64, 0.16);
    color: var(--warn, #b87b1e);
  }
</style>
