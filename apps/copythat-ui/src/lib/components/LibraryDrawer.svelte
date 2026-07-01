<!--
  Phase 49 — Library drawer.

  A unified view over the content-addressed chunk Repository, behind the
  Footer's "Library" button. Additive: the History / Sync / Totals
  drawers are unchanged. Three sub-views plus a dedup "hero":

  - Live      — the repository at a glance (stored vs effective, chunks).
  - Snapshots — the unified snapshot timeline (copy / sync / version /
                backup), newest first.
  - Versions  — per-file rolling versions (Phase 42), looked up by path.

  Reads are advisory: a "repository-unavailable" error renders the
  empty/unavailable state rather than surfacing a toast.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import BackupSourcesView from "./BackupSourcesView.svelte";
  import RestoreModal from "./RestoreModal.svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { i18nVersion, t } from "../i18n";
  import { formatBytes } from "../format";
  import {
    listVersions,
    repositoryCompact,
    repositoryCompressionGet,
    repositoryCompressionSet,
    repositoryDiff,
    repositoryExportReport,
    repositoryForget,
    repositoryGc,
    repositoryPrunePolicy,
    repositoryReport,
    repositorySetPinned,
    repositoryRepair,
    repositorySnapshots,
    repositorySources,
    repositoryStats,
    repositoryVerify,
    type GrowthPointDto,
    type RepoCompressionSettings,
    type RepoReportDto,
    type RepoSourceDto,
    type RepositorySnapshotDto,
    type RepositoryStatsDto,
    type SnapshotDiffDto,
    type VersionRecordDto,
  } from "../ipc";
  import {
    closeLibraryDrawer,
    libraryDrawerOpen,
    openRepositoryWizard,
    openTaskCenter,
    pushToast,
  } from "../stores";

  type SubView =
    | "overview"
    | "live"
    | "snapshots"
    | "compare"
    | "reports"
    | "versions"
    | "sources";
  const SUB_VIEWS: SubView[] = [
    "overview",
    "live",
    "snapshots",
    "compare",
    "reports",
    "versions",
    "sources",
  ];

  let subView = $state<SubView>("overview");
  let stats = $state<RepositoryStatsDto | null>(null);
  let sources = $state<RepoSourceDto[]>([]);
  let snapshots = $state<RepositorySnapshotDto[]>([]);
  let versions = $state<VersionRecordDto[]>([]);
  let versionPath = $state("");
  let unavailable = $state(false);
  let restoreSnapshotId = $state<number | null>(null);
  let compareFrom = $state<number | null>(null);
  let compareTo = $state<number | null>(null);
  let diffResult = $state<SnapshotDiffDto | null>(null);
  let report = $state<RepoReportDto | null>(null);
  let pruneKeepLast = $state(10);
  let compressionMode = $state("off");

  // Refresh stats + snapshots whenever the drawer opens.
  $effect(() => {
    if ($libraryDrawerOpen) {
      void refresh();
      void loadCompression();
      void loadSources();
    }
  });

  // Lazily compute the report the first time the Reports tab is opened.
  $effect(() => {
    if ($libraryDrawerOpen && subView === "reports" && report === null) {
      void loadReport();
    }
  });

  async function refresh() {
    unavailable = false;
    try {
      const [s, snaps] = await Promise.all([
        repositoryStats(),
        repositorySnapshots(),
      ]);
      stats = s;
      // Newest first for the timeline.
      snapshots = snaps.slice().reverse();
      // Default the compare pickers to "previous → newest".
      if (snapshots.length >= 2 && compareFrom === null) {
        compareFrom = snapshots[1].id;
        compareTo = snapshots[0].id;
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      if (msg === "repository-unavailable") {
        unavailable = true;
      } else {
        pushToast("error", msg);
      }
    }
  }

  async function loadVersions() {
    const path = versionPath.trim();
    if (!path) {
      versions = [];
      return;
    }
    try {
      versions = await listVersions(path);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function fmtDate(ms: number): string {
    return new Date(ms).toLocaleString();
  }

  function savedPct(ratio: number): string {
    return `${Math.round(ratio * 100)}%`;
  }

  async function forget(id: number) {
    try {
      await repositoryForget(id);
      pushToast("info", t("snapshot-forget-toast"));
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function reclaimSpace() {
    try {
      const r = await repositoryGc();
      pushToast(
        "success",
        t("repo-gc-done", {
          chunks: r.chunksSwept,
          bytes: formatBytes(r.bytesReclaimed),
        }),
      );
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function fullCompact() {
    try {
      await repositoryCompact();
      openTaskCenter();
      pushToast("info", t("library-compact-started"));
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function doVerify(deep: boolean) {
    try {
      const r = await repositoryVerify(null, deep);
      if (r.isClean) {
        pushToast("success", t("repo-verify-clean", { files: r.filesChecked, chunks: r.chunksChecked }));
      } else {
        pushToast("error", t("repo-verify-damaged", { missing: r.missing, corrupt: r.corrupt }));
      }
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function doRepair() {
    try {
      const dry = await repositoryRepair(true, false);
      if (dry.removedIds.length === 0) {
        pushToast("success", t("repo-repair-none"));
        return;
      }
      if (!window.confirm(t("repo-repair-confirm", { n: dry.removedIds.length }))) return;
      const done = await repositoryRepair(true, true);
      pushToast("success", t("repo-repair-removed", { n: done.removedIds.length }));
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function doCompare() {
    if (compareFrom === null || compareTo === null) return;
    try {
      diffResult = await repositoryDiff(compareFrom, compareTo);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function loadReport() {
    try {
      report = await repositoryReport(10);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      if (msg !== "repository-unavailable") pushToast("error", msg);
    }
  }

  async function exportReport() {
    const path = await save({
      filters: [
        { name: "Markdown", extensions: ["md"] },
        { name: "JSON", extensions: ["json"] },
      ],
    });
    if (typeof path !== "string") return;
    const format: "md" | "json" = path.toLowerCase().endsWith(".json") ? "json" : "md";
    try {
      await repositoryExportReport(path, 10, format);
      pushToast("success", t("report-exported", { path }));
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // Map the growth series to an SVG polyline over a 100×30 viewBox.
  function growthPoints(growth: GrowthPointDto[]): string {
    const max = Math.max(...growth.map((g) => g.cumulativeUniqueBytes), 1);
    const n = growth.length;
    return growth
      .map((g, i) => {
        const x = n > 1 ? (i / (n - 1)) * 100 : 0;
        const y = 29 - (g.cumulativeUniqueBytes / max) * 28;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  }

  async function togglePin(id: number, pinned: boolean) {
    try {
      await repositorySetPinned(id, pinned);
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function doPrune() {
    try {
      const removed = await repositoryPrunePolicy(pruneKeepLast, null, Date.now());
      if (removed.length === 0) {
        pushToast("info", t("repo-prune-none"));
      } else {
        await repositoryGc(); // reclaim the now-orphaned chunks
        pushToast("success", t("repo-prune-removed", { n: removed.length }));
      }
      await refresh();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function loadCompression() {
    try {
      compressionMode = (await repositoryCompressionGet()).mode;
    } catch {
      // advisory; ignore
    }
  }

  async function loadSources() {
    try {
      sources = await repositorySources();
    } catch {
      // advisory; ignore
    }
  }

  async function setCompression(mode: string) {
    const c: RepoCompressionSettings =
      mode === "auto"
        ? { mode: "auto", level: 3 }
        : mode === "always"
          ? { mode: "always", level: 3 }
          : { mode: "off" };
    try {
      await repositoryCompressionSet(c);
      compressionMode = mode;
      pushToast("info", t("storage-compression-restart"));
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }
</script>

{#if $libraryDrawerOpen}
  <aside class="drawer" aria-label={t("library-title")}>
    {#key $i18nVersion}
      <header>
        <h2>{t("library-title")}</h2>
        <button
          class="repo-switch"
          type="button"
          onclick={openRepositoryWizard}
          style="background:none;border:1px solid var(--border,#444);color:inherit;border-radius:4px;font-size:0.75rem;padding:2px 8px;cursor:pointer;"
        >
          {t("repo-switcher-label")}
        </button>
        <button
          class="close"
          type="button"
          aria-label={t("action-close")}
          onclick={closeLibraryDrawer}
        >
          <Icon name="x" size={16} />
        </button>
      </header>

      {#if unavailable}
        <p class="notice">{t("library-unavailable")}</p>
      {:else}
        {#if stats}
          <div class="hero">
            <span class="hero-num">💾 {formatBytes(stats.storedBytes)}</span>
            <span class="hero-sub">
              {#if stats.effectiveBytes > 0}
                {t("library-hero-savings", {
                  effective: formatBytes(stats.effectiveBytes),
                  pct: savedPct(stats.savedRatio),
                })}
              {:else}
                {t("library-hero-empty", { chunks: stats.chunkCount })}
              {/if}
            </span>
          </div>
        {/if}

        <div class="tabs" role="tablist">
          {#each SUB_VIEWS as id (id)}
            <button
              type="button"
              role="tab"
              class="tab"
              class:active={subView === id}
              aria-selected={subView === id}
              onclick={() => (subView = id)}
            >
              {t(`library-tab-${id}`)}
            </button>
          {/each}
        </div>

        {#if subView === "overview"}
          {#if sources.length === 0}
            <p class="empty">{t("library-source-empty")}</p>
          {:else}
            <ul class="source-cards">
              {#each sources as s (s.source)}
                <li>
                  <div class="src-head">
                    <span class="src-path">{s.source || t("library-source-unknown")}</span>
                    <span class="kind-chip">{t(`repo-kind-${s.latestKind}`)}</span>
                  </div>
                  <div class="src-meta">
                    <span>{t("library-source-snapshots", { n: s.snapshotCount })}</span>
                    <span>{formatBytes(s.latestSize)}</span>
                    <span
                      >{t("library-source-latest", {
                        when: new Date(s.latestMs).toLocaleDateString(),
                      })}</span
                    >
                  </div>
                </li>
              {/each}
            </ul>
          {/if}
        {:else if subView === "live"}
          {#if stats}
            <dl class="grid">
              <div>
                <dt>{t("library-stat-stored")}</dt>
                <dd>{formatBytes(stats.storedBytes)}</dd>
              </div>
              <div>
                <dt>{t("library-stat-effective")}</dt>
                <dd>{formatBytes(stats.effectiveBytes)}</dd>
              </div>
              <div>
                <dt>{t("library-stat-snapshots")}</dt>
                <dd>{stats.snapshotCount}</dd>
              </div>
              <div>
                <dt>{t("library-stat-chunks")}</dt>
                <dd>{stats.chunkCount}</dd>
              </div>
              {#if stats.compressionRatio > 0}
                <div>
                  <dt>{t("library-stat-compression")}</dt>
                  <dd>{savedPct(stats.compressionRatio)}</dd>
                </div>
              {/if}
            </dl>
            <div class="compression-ctl">
              <label class="ret-label">
                {t("storage-compression")}
                <select
                  value={compressionMode}
                  onchange={(e) => setCompression(e.currentTarget.value)}
                >
                  <option value="off">{t("storage-compression-off")}</option>
                  <option value="auto">{t("storage-compression-auto")}</option>
                  <option value="always">{t("storage-compression-always")}</option>
                </select>
              </label>
              <span class="meta">{t("storage-compression-restart")}</span>
            </div>
            <div class="reclaim">
              <button type="button" onclick={reclaimSpace}>
                {t("library-reclaim")}
              </button>
              <button type="button" onclick={fullCompact}>
                {t("library-compact")}
              </button>
              <button type="button" onclick={() => doVerify(false)}>
                {t("repo-action-verify")}
              </button>
              <button type="button" onclick={() => doVerify(true)}>
                {t("repo-action-verify-deep")}
              </button>
              <button type="button" onclick={doRepair}>
                {t("repo-action-repair")}
              </button>
            </div>
          {:else}
            <p class="empty">{t("library-loading")}</p>
          {/if}
        {:else if subView === "snapshots"}
          {#if snapshots.length === 0}
            <p class="empty">{t("library-snapshot-empty")}</p>
          {:else}
            <div class="prune-bar">
              <label class="ret-label">
                {t("repo-prune-keep-last")}
                <input type="number" min="1" bind:value={pruneKeepLast} />
              </label>
              <button type="button" onclick={doPrune}>{t("repo-prune-title")}</button>
            </div>
            <ul class="list">
              {#each snapshots as s (s.id)}
                <li>
                  <span class="snap-head">
                    <span class="kind kind-{s.kind}">{t(`repo-kind-${s.kind}`)}</span>
                    {#if s.pinned}
                      <span class="pin-badge">📌 {t("repo-pinned-badge")}</span>
                    {/if}
                  </span>
                  <span class="label">{s.label}</span>
                  {#if s.description}
                    <span class="meta">{s.description}</span>
                  {/if}
                  <span class="meta">
                    {fmtDate(s.createdAtMs)} ·
                    {t("library-snapshot-files", { n: s.fileCount })} ·
                    {formatBytes(s.totalSize)}
                  </span>
                  <span class="snap-actions">
                    <button type="button" onclick={() => togglePin(s.id, !s.pinned)}>
                      {s.pinned ? t("repo-unpin") : t("repo-pin")}
                    </button>
                    <button type="button" onclick={() => (restoreSnapshotId = s.id)}>
                      {t("restore-browse")}
                    </button>
                    <button type="button" class="danger" onclick={() => forget(s.id)}>
                      {t("snapshot-forget")}
                    </button>
                  </span>
                </li>
              {/each}
            </ul>
          {/if}
        {:else if subView === "compare"}
          {#if snapshots.length < 2}
            <p class="empty">{t("repo-diff-pick-two")}</p>
          {:else}
            <div class="compare-bar">
              <select bind:value={compareFrom}>
                {#each snapshots as s (s.id)}
                  <option value={s.id}>
                    {fmtDate(s.createdAtMs)} · {t(`repo-kind-${s.kind}`)}
                  </option>
                {/each}
              </select>
              <span aria-hidden="true">→</span>
              <select bind:value={compareTo}>
                {#each snapshots as s (s.id)}
                  <option value={s.id}>
                    {fmtDate(s.createdAtMs)} · {t(`repo-kind-${s.kind}`)}
                  </option>
                {/each}
              </select>
              <button type="button" onclick={doCompare}>{t("library-tab-compare")}</button>
            </div>
            {#if diffResult}
              <div class="diff-summary">
                <span>
                  {t("repo-diff-summary", {
                    added: diffResult.added,
                    removed: diffResult.removed,
                    modified: diffResult.modified,
                  })}
                </span>
                <span class="meta">
                  {t("repo-diff-bytes-added", {
                    bytes: formatBytes(diffResult.bytesAdded),
                  })}
                </span>
              </div>
              {@const changed = diffResult.files.filter((f) => f.change !== "unchanged")}
              {#if changed.length === 0}
                <p class="empty">{t("repo-change-unchanged")}</p>
              {:else}
                <ul class="list">
                  {#each changed as f (f.path)}
                    <li>
                      <span class="change change-{f.change}">{t(`repo-change-${f.change}`)}</span>
                      <span class="label" title={f.path}>{f.path}</span>
                    </li>
                  {/each}
                </ul>
              {/if}
            {/if}
          {/if}
        {:else if subView === "reports"}
          {#if report === null}
            <p class="empty">{t("library-loading")}</p>
          {:else}
            <div class="report">
              <p class="report-dedup">
                {t("report-dedup-ratio", { pct: Math.round(report.dedupRatio * 100) })}
              </p>
              {#if report.growth.length > 1}
                <h3>{t("report-growth-title")}</h3>
                <svg
                  class="spark"
                  viewBox="0 0 100 30"
                  preserveAspectRatio="none"
                  aria-hidden="true"
                >
                  <polyline
                    points={growthPoints(report.growth)}
                    fill="none"
                    stroke="var(--accent, #3b82f6)"
                    stroke-width="1.5"
                  />
                </svg>
              {/if}
              <h3>{t("report-by-kind-title")}</h3>
              <ul class="list">
                {#each report.byKind as k (k.kind)}
                  <li class="report-row">
                    <span class="kind kind-{k.kind}">{t(`repo-kind-${k.kind}`)}</span>
                    <span class="meta">{k.count} · {formatBytes(k.effectiveBytes)}</span>
                  </li>
                {/each}
              </ul>
              {#if report.topFiles.length > 0}
                <h3>{t("report-top-files-title")}</h3>
                <ul class="list">
                  {#each report.topFiles as f (f.path)}
                    <li class="report-row">
                      <span class="label" title={f.path}>{f.path}</span>
                      <span class="meta">
                        {t("report-file-versions", { n: f.versions })} · {formatBytes(f.maxSize)}
                      </span>
                    </li>
                  {/each}
                </ul>
              {/if}
              <div class="reclaim">
                <button type="button" onclick={exportReport}>{t("report-export")}</button>
              </div>
            </div>
          {/if}
        {:else if subView === "sources"}
          <BackupSourcesView {refresh} />
        {:else}
          <div class="version-bar">
            <input
              type="text"
              placeholder={t("library-version-path-ph")}
              bind:value={versionPath}
              onkeydown={(e) => e.key === "Enter" && loadVersions()}
            />
            <button type="button" onclick={loadVersions}>
              {t("library-version-load")}
            </button>
          </div>
          {#if versions.length === 0}
            <p class="empty">{t("library-version-empty")}</p>
          {:else}
            <ul class="list">
              {#each versions as v (v.rowId)}
                <li>
                  <span class="label">{fmtDate(v.tsMs)}</span>
                  <span class="meta">{formatBytes(v.size)}</span>
                </li>
              {/each}
            </ul>
          {/if}
        {/if}
      {/if}

      {#if restoreSnapshotId !== null}
        <RestoreModal
          snapshotId={restoreSnapshotId}
          onClose={() => (restoreSnapshotId = null)}
          {refresh}
        />
      {/if}
    {/key}
  </aside>
{/if}

<style>
  .drawer {
    position: fixed;
    inset: 0;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    z-index: 85;
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
    font-size: 1.1rem;
  }
  .close {
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
    padding: 4px;
  }
  .hero {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .hero-num {
    font-size: 1.5rem;
    font-weight: 700;
  }
  .hero-sub {
    color: var(--muted, #666666);
    font-size: 0.9rem;
  }
  .tabs {
    display: flex;
    gap: 4px;
    padding: 0 16px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.3));
  }
  .tab {
    background: none;
    border: none;
    padding: 8px 12px;
    cursor: pointer;
    color: var(--muted, #666666);
    border-bottom: 2px solid transparent;
  }
  .tab.active {
    color: var(--fg, #1f1f1f);
    border-bottom-color: var(--accent, #3b82f6);
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    padding: 16px;
    margin: 0;
  }
  .grid dt {
    color: var(--muted, #666666);
    font-size: 0.85rem;
  }
  .grid dd {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
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
    gap: 2px;
    padding: 8px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.2));
    border-radius: 6px;
  }
  .kind {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent, #3b82f6);
  }
  .meta {
    color: var(--muted, #666666);
    font-size: 0.8rem;
  }
  .empty,
  .notice {
    padding: 24px 16px;
    color: var(--muted, #666666);
    text-align: center;
  }
  .version-bar {
    display: flex;
    gap: 8px;
    padding: 12px 16px;
  }
  .version-bar input {
    flex: 1;
    padding: 6px 8px;
  }
  .compare-bar {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 12px 16px;
    flex-wrap: wrap;
  }
  .compare-bar select {
    flex: 1;
    min-width: 0;
    padding: 4px 6px;
  }
  .diff-summary {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    padding: 0 16px 8px;
    flex-wrap: wrap;
    font-size: 0.85rem;
  }
  .change {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .change-added {
    color: #16a34a;
  }
  .change-removed {
    color: var(--danger, #dc2626);
  }
  .change-modified {
    color: #d97706;
  }
  .report {
    padding: 8px 16px;
  }
  .report h3 {
    font-size: 0.9rem;
    margin: 12px 0 6px;
  }
  .report-dedup {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 8px 0;
  }
  .spark {
    width: 100%;
    height: 40px;
    display: block;
  }
  .list li.report-row {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }
  .snap-actions {
    display: flex;
    gap: 6px;
    margin-top: 4px;
  }
  .prune-bar {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 12px 16px;
    flex-wrap: wrap;
  }
  .prune-bar input {
    width: 64px;
    padding: 4px 6px;
  }
  .ret-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.85rem;
  }
  .snap-head {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .pin-badge {
    font-size: 0.7rem;
    color: #d97706;
  }
  .snap-actions button {
    font-size: 0.8rem;
    padding: 2px 8px;
    cursor: pointer;
  }
  .danger {
    color: var(--danger, #dc2626);
  }
  .source-cards {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .source-cards li {
    padding: 10px 16px;
    border-bottom: 1px solid var(--border, #2a2a2a);
  }
  .src-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .src-path {
    font-size: 0.9rem;
    word-break: break-all;
  }
  .kind-chip {
    font-size: 0.7rem;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--border, #2a2a2a);
    white-space: nowrap;
  }
  .src-meta {
    display: flex;
    gap: 12px;
    margin-top: 4px;
    font-size: 0.78rem;
    opacity: 0.7;
  }
  .compression-ctl {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 16px 8px;
    flex-wrap: wrap;
    font-size: 0.85rem;
  }
  .reclaim {
    padding: 0 16px 12px;
  }
  .reclaim button {
    cursor: pointer;
    padding: 6px 12px;
  }
</style>
