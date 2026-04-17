<!--
  36px SVG progress ring with rounded caps.

  - Shows a percentage label inside when the progress is ≥10% so
    short-lived jobs don't strobe numbers.
  - Otherwise renders a `from 0deg` spin animation so the row still
    looks alive during enumeration.
  - Colour derives from `status`: accent while running, green on
    verify-ok, amber on paused, red on error, muted on idle.
-->
<script lang="ts">
  import type { JobState } from "../types";

  interface Props {
    ratio: number;
    status: JobState;
    size?: number;
    stroke?: number;
    showLabel?: boolean;
  }

  let {
    ratio,
    status,
    size = 36,
    stroke = 4,
    showLabel = true,
  }: Props = $props();

  const radius = $derived((size - stroke) / 2);
  const circumference = $derived(2 * Math.PI * radius);
  const clamped = $derived(Math.max(0, Math.min(1, ratio)));
  const dashOffset = $derived(circumference * (1 - clamped));
  const percent = $derived(Math.round(clamped * 100));
  const showPercent = $derived(
    showLabel &&
      clamped >= 0.1 &&
      status !== "cancelled" &&
      status !== "failed",
  );
  const indeterminate = $derived(
    status === "pending" ||
      (status === "running" && clamped < 0.01 && clamped >= 0),
  );
</script>

<div
  class="ring"
  data-status={status}
  class:indeterminate
  style:--ring-size="{size}px"
  style:--ring-stroke="{stroke}px"
  role="progressbar"
  aria-valuenow={percent}
  aria-valuemin="0"
  aria-valuemax="100"
>
  <svg width={size} height={size} viewBox="0 0 {size} {size}">
    <circle
      class="track"
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke-width={stroke}
    />
    {#if status === "failed"}
      <circle
        class="bar done"
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke-width={stroke}
        stroke-dasharray={circumference}
        stroke-dashoffset="0"
        stroke-linecap="round"
      />
    {:else if status === "succeeded"}
      <circle
        class="bar succeeded"
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke-width={stroke}
        stroke-dasharray={circumference}
        stroke-dashoffset="0"
        stroke-linecap="round"
      />
    {:else}
      <circle
        class="bar"
        class:spinner={indeterminate}
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke-width={stroke}
        stroke-dasharray={circumference}
        stroke-dashoffset={indeterminate ? circumference * 0.7 : dashOffset}
        stroke-linecap="round"
      />
    {/if}
  </svg>
  {#if showPercent}
    <span class="label">{percent}</span>
  {/if}
</div>

<style>
  .ring {
    --ring-accent: var(--accent, #4f8cff);
    position: relative;
    width: var(--ring-size);
    height: var(--ring-size);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .ring[data-status="paused"] {
    --ring-accent: var(--warn, #e4a040);
  }
  .ring[data-status="failed"] {
    --ring-accent: var(--error, #d95757);
  }
  .ring[data-status="cancelled"] {
    --ring-accent: var(--muted-strong, #7a7a7a);
  }
  .ring[data-status="succeeded"] {
    --ring-accent: var(--ok, #3faf6a);
  }

  svg {
    transform: rotate(-90deg);
    overflow: visible;
  }

  .track {
    stroke: var(--ring-track, rgba(128, 128, 128, 0.2));
  }

  .bar {
    stroke: var(--ring-accent);
    transition:
      stroke-dashoffset 120ms linear,
      stroke 160ms ease;
  }

  .bar.spinner {
    animation: spin 1.1s linear infinite;
    transform-origin: 50% 50%;
  }

  .label {
    position: absolute;
    font-size: 10px;
    font-variant-numeric: tabular-nums;
    font-weight: 600;
    color: var(--fg-strong, #1f1f1f);
    line-height: 1;
    pointer-events: none;
  }

  @keyframes spin {
    from {
      transform: rotate(0);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
