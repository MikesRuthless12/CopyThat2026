<!--
  Root component.

  Layout is a single-column flex: Header (56px) → ProgressBar (6px)
  → JobList (flex 1) → Footer (32px). The list container is also
  the drag-drop surface — actual paths arrive from the Rust side via
  the `drop-received` event, not from an HTML `drop` handler, because
  Tauri's webview intercepts drops at the window level.
-->
<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import Header from "./lib/components/Header.svelte";
  import Footer from "./lib/components/Footer.svelte";
  import ProgressBar from "./lib/components/ProgressBar.svelte";
  import JobList from "./lib/components/JobList.svelte";
  import ContextMenu from "./lib/components/ContextMenu.svelte";
  import DetailsDrawer from "./lib/components/DetailsDrawer.svelte";
  import DropStagingDialog from "./lib/components/DropStagingDialog.svelte";
  import ErrorModal from "./lib/components/ErrorModal.svelte";
  import CollisionModal from "./lib/components/CollisionModal.svelte";
  import ErrorLogDrawer from "./lib/components/ErrorLogDrawer.svelte";
  import HistoryDrawer from "./lib/components/HistoryDrawer.svelte";
  import Toast from "./lib/components/Toast.svelte";

  import { initI18n, t } from "./lib/i18n";
  import { initTheme } from "./lib/theme";
  import { dropped, initStores, jobs } from "./lib/stores";
  import {
    cancelJob,
    pauseJob,
    removeJob,
    resumeJob,
    revealInFolder,
  } from "./lib/ipc";
  import type { ContextMenuItem, JobDto } from "./lib/types";

  let selectedId: number | null = $state(null);
  let detailsJob: JobDto | null = $state(null);
  let contextMenu:
    | { job: JobDto; x: number; y: number; items: ContextMenuItem[] }
    | null = $state(null);

  let storesCleanup: (() => void) | null = null;
  let themeCleanup: (() => void) | null = null;

  onMount(async () => {
    themeCleanup = initTheme();
    await initI18n();
    storesCleanup = await initStores();
  });

  onDestroy(() => {
    storesCleanup?.();
    themeCleanup?.();
  });

  function contextItemsFor(job: JobDto): ContextMenuItem[] {
    const items: ContextMenuItem[] = [];
    const isRunning = job.state === "running";
    const isPaused = job.state === "paused";
    const isActive = isRunning || isPaused || job.state === "pending";
    items.push({
      id: "pause",
      label: t("menu-pause"),
      icon: "pause",
      disabled: !isRunning,
      onClick: () => void pauseJob(job.id),
    });
    items.push({
      id: "resume",
      label: t("menu-resume"),
      icon: "play",
      disabled: !isPaused,
      onClick: () => void resumeJob(job.id),
    });
    items.push({
      id: "cancel",
      label: t("menu-cancel"),
      icon: "x",
      disabled: !isActive,
      onClick: () => void cancelJob(job.id),
      tone: "danger",
    });
    items.push({
      id: "remove",
      label: t("menu-remove"),
      icon: "trash",
      onClick: () => void removeJob(job.id),
      tone: "danger",
    });
    items.push({
      id: "reveal-src",
      label: t("menu-reveal-source"),
      icon: "external-link",
      onClick: () => void revealInFolder(job.src),
    });
    if (job.dst) {
      items.push({
        id: "reveal-dst",
        label: t("menu-reveal-destination"),
        icon: "external-link",
        onClick: () => void revealInFolder(job.dst!),
      });
    }
    return items;
  }

  function openContextMenu(e: MouseEvent, job: JobDto) {
    const MENU_WIDTH = 180;
    const MENU_HEIGHT_ESTIMATE = 32 * 6;
    const x = Math.min(e.clientX, window.innerWidth - MENU_WIDTH - 8);
    const y = Math.min(e.clientY, window.innerHeight - MENU_HEIGHT_ESTIMATE - 8);
    contextMenu = { job, x, y, items: contextItemsFor(job) };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  // Re-emit the jobs store so TS knows it's used reactively.
  $effect(() => {
    void $jobs.length;
  });
</script>

<main class="app" aria-label={t("window-title")}>
  <Header />
  <ProgressBar />
  <JobList
    {selectedId}
    onSelect={(id) => {
      selectedId = id;
    }}
    onContextMenu={openContextMenu}
    onOpenDetails={(job) => {
      detailsJob = job;
    }}
  />
  <Footer />

  {#if detailsJob}
    <DetailsDrawer
      job={detailsJob}
      onClose={() => {
        detailsJob = null;
      }}
    />
  {/if}

  {#if contextMenu}
    <ContextMenu
      items={contextMenu.items}
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={closeContextMenu}
    />
  {/if}

  {#if $dropped.length > 0}
    <DropStagingDialog paths={$dropped} />
  {/if}

  <!-- Phase 8: prompt modals + error-log drawer -->
  <ErrorModal />
  <CollisionModal />
  <ErrorLogDrawer />

  <!-- Phase 9: SQLite history drawer -->
  <HistoryDrawer />

  <Toast />
</main>

<style>
  :root {
    --accent: #4f8cff;
    --ok: #3faf6a;
    --warn: #e4a040;
    --error: #d95757;
    --verify: #7a4fb3;
  }

  :global(html) {
    height: 100%;
  }

  :global(body) {
    margin: 0;
    height: 100%;
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      Roboto,
      "Helvetica Neue",
      Arial,
      sans-serif;
    font-size: 13px;
    background: var(--bg, #fafafa);
    color: var(--fg, #1f1f1f);
  }

  :global(#app) {
    height: 100%;
  }

  :global(html[data-theme="dark"]) {
    --bg: #18181a;
    --surface: #202024;
    --fg: #ececec;
    --fg-strong: #ffffff;
    --fg-dim: #a0a0a8;
    --border: rgba(255, 255, 255, 0.12);
    --hover: rgba(255, 255, 255, 0.06);
    --row-selected: rgba(79, 140, 255, 0.18);
  }

  :global(html:not([data-theme="dark"])) {
    --bg: #fafafa;
    --surface: #ffffff;
    --fg: #1f1f1f;
    --fg-strong: #1f1f1f;
    --fg-dim: #6a6a6a;
    --border: rgba(0, 0, 0, 0.1);
    --hover: rgba(0, 0, 0, 0.04);
    --row-selected: rgba(79, 140, 255, 0.1);
  }

  @media (prefers-contrast: more) {
    :root {
      --accent: #1f5bc8;
      --error: #a52020;
      --ok: #1e6b3c;
    }
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    position: relative;
    overflow: hidden;
  }
</style>
