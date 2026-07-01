<!--
  Phase 49c — Backup sources sub-view, slotted into the Library drawer.

  Lists the folders the user designated for backup, lets them add one
  (native folder picker + optional comma-separated exclude globs), run
  "Back up now" (snapshots into the repository as a Backup), and remove
  sources. A live progress bar + toasts are driven by the `backup-*`
  events; on completion the parent Library `refresh` runs so the new
  snapshot + the updated dedup hero appear immediately.
-->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { t } from "../i18n";
  import {
    backupNow,
    backupSourcesStatus,
    notificationsGet,
    notificationsSet,
    notificationsTest,
    repositoryPrune,
    sourcesAdd,
    sourcesList,
    sourcesRemove,
    sourcesSetRetention,
    sourcesSetSchedule,
    type RetentionSettings,
    type SourceConfigDto,
  } from "../ipc";
  import { pushToast } from "../stores";

  let { refresh }: { refresh: () => void | Promise<void> } = $props();

  type Progress = { filesDone: number; filesTotal: number };

  let sources = $state<SourceConfigDto[]>([]);
  let progress = $state<Record<string, Progress | undefined>>({});
  let newExcludes = $state("");
  let newIncludes = $state("");
  let newSkipHidden = $state(false);
  let notifyOnSuccess = $state(false);
  let notifyOnFailure = $state(false);
  let unlistenFns: UnlistenFn[] = [];

  async function reload() {
    try {
      sources = await sourcesList();
      await loadStatus();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function clearProgress(id: string) {
    const next = { ...progress };
    delete next[id];
    progress = next;
  }

  onMount(() => {
    void reload();
    void loadNotifications();
    (async () => {
      unlistenFns.push(
        await listen<{ id: string; filesDone: number; filesTotal: number }>(
          "backup-progress",
          (evt) => {
            progress = {
              ...progress,
              [evt.payload.id]: {
                filesDone: evt.payload.filesDone,
                filesTotal: evt.payload.filesTotal,
              },
            };
          },
        ),
      );
      unlistenFns.push(
        await listen<{ id: string; files: number }>(
          "backup-completed",
          (evt) => {
            clearProgress(evt.payload.id);
            pushToast(
              "success",
              t("backup-toast-completed", {
                label: labelOf(evt.payload.id),
                files: evt.payload.files,
              }),
            );
            void reload();
            void refresh();
          },
        ),
      );
      // Failures clear the progress bar here; the user-facing toast is
      // raised by `runBackup`'s catch so there is exactly one per failure.
      unlistenFns.push(
        await listen<{ id: string }>("backup-failed", (evt) => {
          clearProgress(evt.payload.id);
        }),
      );
    })().catch((e) => console.error("[backup-listen]", e));
  });

  onDestroy(() => {
    for (const f of unlistenFns) {
      try {
        f();
      } catch {
        // Unlisten errors are best-effort; swallow.
      }
    }
  });

  function labelOf(id: string): string {
    return sources.find((s) => s.id === id)?.label ?? id;
  }

  function baseName(path: string): string {
    const parts = path.split(/[\\/]/).filter(Boolean);
    return parts.length ? parts[parts.length - 1] : path;
  }

  async function addSource() {
    const picked = await openDialog({ directory: true, multiple: false });
    if (typeof picked !== "string") return;
    const splitGlobs = (s: string) =>
      s.split(",").map((g) => g.trim()).filter(Boolean);
    try {
      await sourcesAdd(
        baseName(picked),
        picked,
        splitGlobs(newExcludes),
        splitGlobs(newIncludes),
        newSkipHidden,
      );
      newExcludes = "";
      newIncludes = "";
      newSkipHidden = false;
      await reload();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function runBackup(id: string) {
    pushToast("info", t("backup-toast-started", { label: labelOf(id) }));
    try {
      await backupNow(id);
    } catch (e) {
      const reason = e instanceof Error ? e.message : String(e);
      pushToast("error", t("backup-toast-failed", { label: labelOf(id), reason }));
    }
  }

  async function removeSource(id: string) {
    try {
      await sourcesRemove(id);
      await reload();
      void refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function fmtDate(iso: string): string {
    if (!iso) return "";
    const d = new Date(iso);
    return Number.isNaN(d.getTime()) ? iso : d.toLocaleString();
  }

  // --- Phase 49e: per-source retention ---------------------------------

  // Preset retention policies surfaced in the dropdown. The backend
  // supports arbitrary Last-N / Older-than-days / GFS values; the GUI
  // offers the common ones (GFS uses a sensible rolling default).
  const PRESETS: Record<string, RetentionSettings> = {
    "keep-all": { kind: "keep-all" },
    "last-5": { kind: "last-n", n: 5 },
    "last-10": { kind: "last-n", n: 10 },
    "last-30": { kind: "last-n", n: 30 },
    "older-30": { kind: "older-than-days", days: 30 },
    "older-90": { kind: "older-than-days", days: 90 },
    gfs: { kind: "gfs", hourly: 24, daily: 7, weekly: 4, monthly: 12 },
  };

  function policyToValue(r: RetentionSettings): string {
    if (r.kind === "last-n") return r.n <= 5 ? "last-5" : r.n <= 10 ? "last-10" : "last-30";
    if (r.kind === "older-than-days") return r.days <= 30 ? "older-30" : "older-90";
    if (r.kind === "gfs") return "gfs";
    return "keep-all";
  }

  function humanBytes(n: number): string {
    if (n < 1024) return `${n} B`;
    const units = ["KiB", "MiB", "GiB", "TiB"];
    let v = n / 1024;
    let i = 0;
    while (v >= 1024 && i < units.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(1)} ${units[i]}`;
  }

  async function setRetention(id: string, value: string) {
    const policy = PRESETS[value] ?? { kind: "keep-all" };
    try {
      await sourcesSetRetention(id, policy);
      await reload();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function pruneNow(id: string) {
    try {
      const r = await repositoryPrune(id, Date.now());
      if (r.snapshotsRemoved === 0) {
        pushToast("info", t("backup-prune-none"));
      } else {
        pushToast(
          "success",
          t("backup-prune-result", {
            removed: r.snapshotsRemoved,
            bytes: humanBytes(r.bytesReclaimed),
          }),
        );
      }
      await reload();
      void refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // --- Phase 49f: scheduling -------------------------------------------

  let nextRun = $state<Record<string, number | null>>({});

  async function loadStatus() {
    try {
      const rows = await backupSourcesStatus();
      const m: Record<string, number | null> = {};
      for (const r of rows) m[r.id] = r.nextRunMs;
      nextRun = m;
    } catch {
      // Status is advisory (e.g. repository unavailable); ignore.
    }
  }

  async function setSchedule(id: string, spec: string) {
    try {
      // A non-Manual cadence implies auto-run enabled.
      await sourcesSetSchedule(id, spec, spec !== "");
      await reload();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // --- Phase 49q: notifications ----------------------------------------

  async function loadNotifications() {
    try {
      const n = await notificationsGet();
      notifyOnSuccess = n.onSuccess;
      notifyOnFailure = n.onFailure;
    } catch {
      // advisory; ignore
    }
  }

  async function saveNotifications() {
    try {
      await notificationsSet(notifyOnSuccess, notifyOnFailure);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function testNotify() {
    try {
      const n = await notificationsTest();
      pushToast("info", t("notify-test-sent", { n }));
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }
</script>

<div class="notify-bar">
  <span class="notify-title">{t("notify-title")}</span>
  <label class="notify-toggle">
    <input type="checkbox" bind:checked={notifyOnSuccess} onchange={saveNotifications} />
    {t("notify-on-success")}
  </label>
  <label class="notify-toggle">
    <input type="checkbox" bind:checked={notifyOnFailure} onchange={saveNotifications} />
    {t("notify-on-failure")}
  </label>
  <button type="button" onclick={testNotify}>{t("notify-test")}</button>
</div>
<div class="add-row">
  <input
    type="text"
    placeholder={t("backup-exclude-ph")}
    bind:value={newExcludes}
  />
  <input
    type="text"
    placeholder={t("backup-include-ph")}
    bind:value={newIncludes}
  />
  <label class="skip-hidden">
    <input type="checkbox" bind:checked={newSkipHidden} />
    {t("backup-skip-hidden")}
  </label>
  <button type="button" onclick={addSource}>{t("backup-add-source")}</button>
</div>

{#if sources.length === 0}
  <p class="empty">{t("backup-empty")}</p>
{:else}
  <ul class="list">
    {#each sources as s (s.id)}
      <li>
        <div class="row">
          <div class="info">
            <span class="label">{s.label}</span>
            <span class="meta" title={s.path}>{s.path}</span>
            <span class="meta">
              {#if s.lastRunAt}
                {t("backup-last-run", { when: fmtDate(s.lastRunAt) })}
              {:else}
                {t("backup-never-run")}
              {/if}
            </span>
          </div>
          <div class="actions">
            <button
              type="button"
              disabled={!!progress[s.id]}
              onclick={() => runBackup(s.id)}
            >
              {t("backup-now")}
            </button>
            <button
              type="button"
              class="danger"
              disabled={!!progress[s.id]}
              onclick={() => removeSource(s.id)}
            >
              {t("backup-remove")}
            </button>
          </div>
        </div>
        <div class="schedule">
          <label class="sched-label">
            {t("backup-schedule")}
            <select
              value={s.schedule}
              disabled={!!progress[s.id]}
              onchange={(e) => setSchedule(s.id, e.currentTarget.value)}
            >
              <option value="">{t("backup-schedule-manual")}</option>
              <option value="@hourly">{t("backup-schedule-hourly")}</option>
              <option value="@daily">{t("backup-schedule-daily")}</option>
              <option value="@weekly">{t("backup-schedule-weekly")}</option>
            </select>
          </label>
          {#if s.enabled && nextRun[s.id]}
            <span class="meta">
              {t("backup-next-run", {
                when: new Date(nextRun[s.id] as number).toLocaleString(),
              })}
            </span>
          {:else}
            <span class="meta">{t("backup-not-scheduled")}</span>
          {/if}
        </div>
        <div class="retention">
          <label class="ret-label">
            {t("backup-retention")}
            <select
              value={policyToValue(s.retention)}
              disabled={!!progress[s.id]}
              onchange={(e) => setRetention(s.id, e.currentTarget.value)}
            >
              <option value="keep-all">{t("backup-retention-keep-all")}</option>
              <option value="last-5">{t("backup-retention-last", { n: 5 })}</option>
              <option value="last-10">{t("backup-retention-last", { n: 10 })}</option>
              <option value="last-30">{t("backup-retention-last", { n: 30 })}</option>
              <option value="older-30">{t("backup-retention-days", { days: 30 })}</option>
              <option value="older-90">{t("backup-retention-days", { days: 90 })}</option>
              <option value="gfs">{t("backup-retention-gfs")}</option>
            </select>
          </label>
          <button
            type="button"
            class="prune"
            disabled={!!progress[s.id]}
            onclick={() => pruneNow(s.id)}
          >
            {t("backup-prune-now")}
          </button>
        </div>
        {#if progress[s.id]}
          {@const p = progress[s.id]}
          <div class="progress">
            <div
              class="bar"
              style:width={p && p.filesTotal > 0
                ? `${Math.round((p.filesDone / p.filesTotal) * 100)}%`
                : "10%"}
            ></div>
          </div>
          <span class="meta">
            {t("backup-running", { files: p ? p.filesTotal : 0 })}
          </span>
        {/if}
      </li>
    {/each}
  </ul>
{/if}

<style>
  .notify-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px 0;
    flex-wrap: wrap;
    font-size: 0.85rem;
  }
  .notify-title {
    font-weight: 600;
  }
  .notify-toggle {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .add-row {
    display: flex;
    gap: 8px;
    padding: 12px 16px;
  }
  .add-row input[type="text"] {
    flex: 1;
    padding: 6px 8px;
  }
  .skip-hidden {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.8rem;
    color: var(--muted, #666666);
    white-space: nowrap;
  }
  .list {
    list-style: none;
    margin: 0;
    padding: 8px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .list li {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.2));
    border-radius: 6px;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .label {
    font-weight: 600;
  }
  .meta {
    color: var(--muted, #666666);
    font-size: 0.8rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .actions {
    display: flex;
    gap: 6px;
    flex-shrink: 0;
  }
  .danger {
    color: var(--danger, #dc2626);
  }
  .progress {
    height: 6px;
    background: var(--border, rgba(128, 128, 128, 0.2));
    border-radius: 3px;
    overflow: hidden;
  }
  .bar {
    height: 100%;
    background: var(--accent, #3b82f6);
    transition: width 0.2s ease;
  }
  .schedule {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    flex-wrap: wrap;
  }
  .sched-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    color: var(--muted, #666666);
  }
  .sched-label select {
    padding: 4px 6px;
  }
  .retention {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    flex-wrap: wrap;
  }
  .ret-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    color: var(--muted, #666666);
  }
  .ret-label select {
    padding: 4px 6px;
  }
  .empty {
    padding: 24px 16px;
    color: var(--muted, #666666);
    text-align: center;
  }
</style>
