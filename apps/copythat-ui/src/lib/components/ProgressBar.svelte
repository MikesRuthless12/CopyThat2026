<!--
  Flat aggregate progress bar — thin 6 px strip directly above the
  footer. Sums across every active job via the globals store and
  ticks in the same 10 000-step model as each row's CircularProgress
  so the visible "%" readout matches the bar fill exactly.
-->
<script lang="ts">
  import { globals, liveBytes } from "../stores";

  const PROGRESS_STEPS = 10_000;
  let g = $derived($globals);
  // Read aggregate bytes from the derived jobs-store snapshot instead
  // of `GlobalsDto` so the bottom bar can't disagree with the per-row
  // ring (they share the same source).
  let bytes = $derived($liveBytes);
  let steps = $derived(
    bytes.total > 0
      ? Math.min(
          PROGRESS_STEPS,
          Math.max(
            0,
            Math.round((bytes.done / bytes.total) * PROGRESS_STEPS),
          ),
        )
      : 0,
  );
  let ratio = $derived(steps / PROGRESS_STEPS);
  let percentLabel = $derived(
    bytes.total > 0 ? `${(steps / 100).toFixed(2)}%` : "",
  );
  let indeterminate = $derived(
    bytes.total === 0 && g.activeJobs + g.queuedJobs > 0,
  );
</script>

<div
  class="bar"
  role="progressbar"
  aria-valuemin="0"
  aria-valuemax="100"
  aria-valuenow={Math.round(ratio * 100)}
  aria-valuetext={percentLabel}
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
