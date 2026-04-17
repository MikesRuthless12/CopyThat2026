<!--
  Horizontal cumulative progress bar — thin 6 px strip between the
  header and the job list. Binds to the globals store so it reflects
  live totals across every active job.
-->
<script lang="ts">
  import { globals } from "../stores";
  import { progressRatio } from "../format";

  let g = $derived($globals);
  let ratio = $derived(progressRatio(g.bytesDone, g.bytesTotal));
  let indeterminate = $derived(
    g.bytesTotal === 0 && g.activeJobs + g.queuedJobs > 0,
  );
</script>

<div
  class="bar"
  role="progressbar"
  aria-valuemin="0"
  aria-valuemax="100"
  aria-valuenow={Math.round(ratio * 100)}
  data-state={g.state}
  class:indeterminate
>
  <div class="fill" style:--ratio={ratio}></div>
</div>

<style>
  .bar {
    position: relative;
    height: 6px;
    background: var(--border, rgba(128, 128, 128, 0.14));
    overflow: hidden;
    flex-shrink: 0;
  }

  .fill {
    height: 100%;
    width: calc(var(--ratio) * 100%);
    background: var(--accent, #4f8cff);
    transition: width 140ms linear;
  }

  .bar[data-state="paused"] .fill {
    background: var(--warn, #e4a040);
  }
  .bar[data-state="error"] .fill {
    background: var(--error, #d95757);
  }
  .bar[data-state="idle"] .fill {
    background: transparent;
  }

  .bar.indeterminate .fill {
    width: 40% !important;
    animation: slide 1.6s ease-in-out infinite;
  }

  @keyframes slide {
    0% {
      margin-left: -40%;
    }
    100% {
      margin-left: 100%;
    }
  }
</style>
