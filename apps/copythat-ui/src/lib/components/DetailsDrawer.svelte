<!--
  Slide-in drawer showing source/destination/error details. Opened by
  double-clicking a row; closed via the X button, Escape, or clicking
  outside. Phase 5 keeps it read-only — Phase 8 adds retry/elevate
  actions here.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import {
    averageRateBps,
    formatBytes,
    formatEtaVerbose,
    formatRate,
  } from "../format";
  import { revealInFolder } from "../ipc";
  import type { JobDto } from "../types";

  interface Props {
    job: JobDto;
    onClose: () => void;
  }

  let { job, onClose }: Props = $props();

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
  const percentDisplay = $derived(
    job.bytesTotal > 0 ? `${(steps / 100).toFixed(2)}%` : "—",
  );
  const avgRate = $derived(averageRateBps(job.bytesDone, job.startedAtMs));

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onClose();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div
  class="drawer"
  role="dialog"
  tabindex="-1"
  aria-modal="true"
  aria-label={t("details-drawer-label")}
>
  {#key $i18nVersion}
  <header class="head">
    <div class="titlewrap">
      <h2>{job.name}</h2>
      {#if job.subpath}<p class="sub">{job.subpath}</p>{/if}
    </div>
    <button
      class="close"
      type="button"
      aria-label={t("action-close")}
      onclick={onClose}
    >
      <Icon name="x" size={16} />
    </button>
  </header>
  <dl class="grid">
    <dt>{t("details-source")}</dt>
    <dd class="path">{job.src}</dd>
    {#if job.dst}
      <dt>{t("details-destination")}</dt>
      <dd class="path">
        {job.dst}
        <button
          class="inline"
          type="button"
          onclick={() => revealInFolder(job.dst!)}
          aria-label={t("action-reveal")}
        >
          <Icon name="external-link" size={12} />
        </button>
      </dd>
    {/if}
    <dt>{t("details-state")}</dt>
    <dd>{t(`state-${job.state}`)}</dd>
    <dt>Progress</dt>
    <dd class="tabular">{percentDisplay}</dd>
    <dt>{t("details-bytes")}</dt>
    <dd>
      {formatBytes(job.bytesDone)} / {formatBytes(job.bytesTotal)}
    </dd>
    <dt>{t("details-files")}</dt>
    <dd>
      {job.filesDone.toLocaleString()} / {job.filesTotal.toLocaleString()}
    </dd>
    <dt>{t("details-speed")}</dt>
    <dd>{formatRate(job.rateBps)}</dd>
    <dt>Avg speed</dt>
    <dd>{avgRate > 0 ? formatRate(avgRate) : "—"}</dd>
    <dt>{t("details-eta")}</dt>
    <dd>{formatEtaVerbose(job.etaSeconds)}</dd>
    {#if job.lastError}
      <dt>{t("details-error")}</dt>
      <dd class="error">{job.lastError}</dd>
    {/if}
  </dl>
  {/key}
</div>

<style>
  .drawer {
    position: absolute;
    right: 0;
    top: 62px;
    bottom: 32px;
    width: min(360px, 75vw);
    padding: 12px 14px 16px;
    background: var(--surface, #ffffff);
    border-left: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    box-shadow: -8px 0 20px rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow: auto;
    z-index: 30;
  }

  .head {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    position: sticky;
    top: 0;
  }

  h2 {
    font-size: 14px;
    margin: 0;
    word-break: break-all;
  }

  .sub {
    margin: 2px 0 0;
    font-size: 11px;
    color: var(--fg-dim, #6a6a6a);
    word-break: break-all;
  }

  .close {
    margin-left: auto;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    width: 24px;
    height: 24px;
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .close:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .grid {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 6px 12px;
    margin: 0;
    font-size: 12px;
  }

  dt {
    color: var(--fg-dim, #6a6a6a);
    font-weight: 500;
  }

  dd {
    margin: 0;
    font-variant-numeric: tabular-nums;
  }

  .tabular {
    font-variant-numeric: tabular-nums;
  }

  .path {
    word-break: break-all;
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }

  .inline {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
  }

  .inline:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .error {
    color: var(--error, #c24141);
  }
</style>
