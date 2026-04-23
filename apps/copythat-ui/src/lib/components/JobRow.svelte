<!--
  Two-line job row. Line 1 shows the ring (with %), filename,
  live metrics (avg rate, verbose ETA) and state. Line 2 shows
  the truncated source → destination paths and the files-done
  count so the user can see exactly what is moving where and
  how much is left — without having to open the details drawer.
-->
<script lang="ts">
  import CircularProgress from "./CircularProgress.svelte";
  import FileKindIcon from "../icons/FileKindIcon.svelte";
  import StateBadge from "./StateBadge.svelte";
  import { i18nVersion, t } from "../i18n";
  import { fileIcon } from "../ipc";
  import {
    averageRateBps,
    formatBytes,
    formatEtaVerbose,
    formatRate,
    truncatePath,
  } from "../format";
  import type { FileIconDto, JobDto } from "../types";

  interface Props {
    job: JobDto;
    selected: boolean;
    onSelect: () => void;
    onContextMenu: (e: MouseEvent) => void;
    onOpenDetails: () => void;
  }

  let { job, selected, onSelect, onContextMenu, onOpenDetails }: Props = $props();

  // See Footer.svelte — we bind `$locale.code` on the root element
  // so the template re-evaluates `t(...)` on language switch.

  let iconInfo: FileIconDto = $state({ kind: "file", extension: null });

  $effect(() => {
    const path = job.src;
    fileIcon(path)
      .then((info) => {
        iconInfo = info;
      })
      .catch(() => {
        // Best-effort — stay on the default `file` icon.
      });
  });

  // 10 000-step progress model: quantize bytesDone/bytesTotal to
  // 0.01 % increments so the ring fill and the text readout advance
  // together and both hit an honest 100.00 % at step 10 000. The
  // ring's `dashOffset` is computed from `ratio` (= steps / 10 000)
  // so it visibly ticks in 1/10 000 increments rather than smooth
  // floating-point, which keeps "99.99 %" from visually looking like
  // a completed ring.
  const PROGRESS_STEPS = 10_000;
  const steps = $derived(
    job.bytesTotal > 0
      ? Math.min(
          PROGRESS_STEPS,
          Math.max(
            0,
            Math.round((job.bytesDone / job.bytesTotal) * PROGRESS_STEPS),
          ),
        )
      : 0,
  );
  const ratio = $derived(steps / PROGRESS_STEPS);
  const percentDisplay = $derived(
    job.bytesTotal > 0 ? `${(steps / 100).toFixed(2)}%` : "—",
  );
  const sizeDisplay = $derived(
    job.bytesTotal > 0
      ? `${formatBytes(job.bytesDone)} / ${formatBytes(job.bytesTotal)}`
      : formatBytes(job.bytesDone),
  );
  const avgRate = $derived(
    job.state === "running" || job.state === "paused"
      ? averageRateBps(job.bytesDone, job.startedAtMs)
      : 0,
  );
  const rateDisplay = $derived(
    job.state === "running" || job.state === "paused"
      ? formatRate(avgRate)
      : "—",
  );
  const etaDisplay = $derived(
    job.state === "running" || job.state === "paused"
      ? formatEtaVerbose(job.etaSeconds)
      : "—",
  );
  const filesDisplay = $derived(
    job.filesTotal > 0
      ? `${job.filesDone.toLocaleString()} / ${job.filesTotal.toLocaleString()}`
      : "—",
  );
  const srcDisplay = $derived(truncatePath(job.src ?? "", 46));
  const dstDisplay = $derived(truncatePath(job.dst ?? "", 46));
</script>

<div
  class="row"
  role="row"
  class:selected
  tabindex="0"
  onclick={onSelect}
  oncontextmenu={(e) => {
    e.preventDefault();
    onContextMenu(e);
  }}
  ondblclick={onOpenDetails}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onOpenDetails();
    }
  }}
>
  {#key $i18nVersion}
    <div
      class="cell ring"
      aria-label={`${job.name} — ${t(`state-${job.state}`)} — ${percentDisplay}`}
    >
      <CircularProgress
        {ratio}
        status={job.state}
        size={48}
        stroke={5}
        showLabel={false}
      />
      <span class="ring-label tabular" aria-hidden="true">{percentDisplay}</span>
    </div>
    <div class="cell icon">
      <FileKindIcon info={iconInfo} size={20} />
    </div>
    <div class="cell name" role="cell">
      <div class="pri" title={job.src}>{job.name}</div>
      <div class="paths">
        <span class="path src" title={job.src}>{srcDisplay}</span>
        {#if job.dst}
          <span class="arrow" aria-hidden="true">→</span>
          <span class="path dst" title={job.dst}>{dstDisplay}</span>
        {/if}
      </div>
    </div>
    <div class="cell size tabular" role="cell" title={sizeDisplay}>
      {sizeDisplay}
    </div>
    <div class="cell files tabular" role="cell" aria-label={t("details-files")}>
      {filesDisplay}
    </div>
    <div class="cell rate tabular" role="cell" aria-label={t("details-speed")}>
      {rateDisplay}
    </div>
    <div class="cell eta tabular" role="cell" aria-label={t("details-eta")}>
      {etaDisplay}
    </div>
    <div class="cell status" role="cell">
      <StateBadge state={job.state} />
    </div>
  {/key}
</div>

<style>
  .row {
    display: grid;
    grid-template-columns:
      56px
      28px
      minmax(180px, 1fr)
      150px
      80px
      110px
      140px
      96px;
    align-items: center;
    gap: 10px;
    min-height: 64px;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.1));
    cursor: default;
    outline: none;
  }

  .row:hover {
    background: var(--hover, rgba(128, 128, 128, 0.06));
  }

  .row:focus-visible,
  .row.selected {
    background: var(--row-selected, rgba(79, 140, 255, 0.1));
  }

  .cell {
    min-width: 0;
    font-size: 12px;
    color: var(--fg, #1f1f1f);
  }

  .ring {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
  }

  .ring-label {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 600;
    color: var(--fg-strong, #1f1f1f);
    pointer-events: none;
    letter-spacing: -0.01em;
  }

  .icon {
    color: var(--fg-dim, #6a6a6a);
    display: flex;
    align-items: center;
  }

  .name {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 2px;
  }

  .name .pri {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .paths {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 10.5px;
    color: var(--fg-dim, #6a6a6a);
    min-width: 0;
  }

  .path {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-variant-numeric: tabular-nums;
  }

  .arrow {
    flex-shrink: 0;
    color: var(--fg-dim, #8a8a8a);
  }

  .tabular {
    font-variant-numeric: tabular-nums;
    color: var(--fg-dim, #6a6a6a);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* ETA values like "1m 44s" or "2h 10m 38s" must never clip — the
     trailing `s` is semantically load-bearing. Give the cell extra
     breathing room and suppress ellipsis. */
  .cell.eta {
    overflow: visible;
    text-overflow: clip;
    padding-right: 2px;
  }

  .status {
    display: flex;
    justify-content: flex-end;
  }
</style>
