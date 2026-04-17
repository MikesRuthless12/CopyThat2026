<!--
  Scrollable list of job rows. Uses `content-visibility: auto` for
  offscreen skipping instead of a full virtual list — sufficient
  through ~5000 rows, and a real virtual scroller can land later
  without touching the row component.

  Keyboard nav: Up/Down moves focus across rows, Enter/Space opens
  details for the focused row (handled on the row itself).
-->
<script lang="ts">
  import JobRow from "./JobRow.svelte";
  import EmptyState from "./EmptyState.svelte";
  import { visibleJobs } from "../stores";
  import type { JobDto } from "../types";

  interface Props {
    selectedId: number | null;
    onSelect: (id: number) => void;
    onContextMenu: (e: MouseEvent, job: JobDto) => void;
    onOpenDetails: (job: JobDto) => void;
  }

  let { selectedId, onSelect, onContextMenu, onOpenDetails }: Props = $props();

  function onKeydown(e: KeyboardEvent) {
    if (!(e.target instanceof HTMLElement)) return;
    if (!e.target.classList.contains("row")) return;
    if (e.key === "ArrowDown") {
      e.preventDefault();
      const next = e.target.nextElementSibling as HTMLElement | null;
      next?.focus();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      const prev = e.target.previousElementSibling as HTMLElement | null;
      prev?.focus();
    }
  }
</script>

<div
  class="list"
  role="grid"
  tabindex="-1"
  aria-rowcount={$visibleJobs.length}
  onkeydown={onKeydown}
>
  {#if $visibleJobs.length === 0}
    <EmptyState />
  {:else}
    {#each $visibleJobs as job (job.id)}
      <div class="holder">
        <JobRow
          {job}
          selected={selectedId === job.id}
          onSelect={() => onSelect(job.id)}
          onContextMenu={(e) => onContextMenu(e, job)}
          onOpenDetails={() => onOpenDetails(job)}
        />
      </div>
    {/each}
  {/if}
</div>

<style>
  .list {
    flex: 1;
    min-height: 0;
    overflow: auto;
    background: var(--list-bg, var(--bg, #ffffff));
  }

  .holder {
    /* Let the browser skip offscreen row layout — cheap "virtual
       list" for modest queue sizes. */
    content-visibility: auto;
    contain-intrinsic-size: 44px;
  }
</style>
