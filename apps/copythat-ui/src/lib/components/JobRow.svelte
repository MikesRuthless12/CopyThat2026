<!--
  One row of the job list. 44 px tall, seven visual columns:
  [ring 36] [icon 18] [name + subpath] [size] [speed] [ETA] [state].

  Right-click opens a context menu; double-click (or Enter) opens
  the details drawer. Keyboard Up/Down moves focus (handled by the
  parent list).
-->
<script lang="ts">
  import CircularProgress from "./CircularProgress.svelte";
  import FileKindIcon from "../icons/FileKindIcon.svelte";
  import StateBadge from "./StateBadge.svelte";
  import { t } from "../i18n";
  import { fileIcon } from "../ipc";
  import { formatBytes, formatEta, formatRate, progressRatio } from "../format";
  import type { FileIconDto, JobDto } from "../types";

  interface Props {
    job: JobDto;
    selected: boolean;
    onSelect: () => void;
    onContextMenu: (e: MouseEvent) => void;
    onOpenDetails: () => void;
  }

  let { job, selected, onSelect, onContextMenu, onOpenDetails }: Props = $props();

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

  const ratio = $derived(progressRatio(job.bytesDone, job.bytesTotal));
  const sizeDisplay = $derived(
    job.bytesTotal > 0
      ? `${formatBytes(job.bytesDone)} / ${formatBytes(job.bytesTotal)}`
      : formatBytes(job.bytesDone),
  );
  const rateDisplay = $derived(
    job.state === "running" ? formatRate(job.rateBps) : "—",
  );
  const etaDisplay = $derived(
    job.state === "running" || job.state === "paused"
      ? formatEta(job.etaSeconds, t)
      : "—",
  );
</script>

<div
  class="row"
  role="row"
  class:selected
  tabindex="0"
  aria-label={`${job.name} — ${t(`state-${job.state}`)}`}
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
  <div class="cell ring">
    <CircularProgress {ratio} status={job.state} />
  </div>
  <div class="cell icon">
    <FileKindIcon info={iconInfo} size={18} />
  </div>
  <div class="cell name" role="cell">
    <div class="pri" title={job.src}>{job.name}</div>
    {#if job.subpath}
      <div class="sub" title={job.subpath}>{job.subpath}</div>
    {/if}
  </div>
  <div class="cell size tabular" role="cell">{sizeDisplay}</div>
  <div class="cell rate tabular" role="cell">{rateDisplay}</div>
  <div class="cell eta tabular" role="cell">{etaDisplay}</div>
  <div class="cell status" role="cell">
    <StateBadge state={job.state} />
  </div>
</div>

<style>
  .row {
    display: grid;
    grid-template-columns: 46px 30px minmax(120px, 1fr) 130px 90px 80px 96px;
    align-items: center;
    gap: 8px;
    height: 44px;
    padding: 0 12px;
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
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .icon {
    color: var(--fg-dim, #6a6a6a);
    display: flex;
    align-items: center;
  }

  .name .pri {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .name .sub {
    font-size: 10.5px;
    color: var(--fg-dim, #6a6a6a);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tabular {
    font-variant-numeric: tabular-nums;
    color: var(--fg-dim, #6a6a6a);
  }

  .status {
    display: flex;
    justify-content: flex-end;
  }
</style>
