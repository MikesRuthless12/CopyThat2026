<!--
  Phase 41 (initial scaffold) — pre-execution tree-diff preview.

  This is the v1 modal: shows the rolled-up plan before the user
  clicks Run, with counts + a flat list per category. The full
  virtualized side-by-side trees + animated transition described in
  the build-prompt-guide land in a follow-up — virtualization is
  necessary at >10k rows to keep DOM commits fast, and the animation
  needs the runner-side hooks for the post-Run morph.

  Usage:
    <TreeDiffPreview
      src={srcPath}
      dst={dstPath}
      onRun={() => actuallyStartCopy(...)}
      onCancel={() => closeModal()}
    />
-->
<script lang="ts">
  import { t } from "../i18n";
  import { computeTreeDiff, type TreeDiffDto } from "../ipc";

  let {
    src = "",
    dst = "",
    forceOverwrite = false,
    trustSizeMtime = true,
    onRun = () => {},
    onCancel = () => {},
  }: {
    src: string;
    dst: string;
    forceOverwrite?: boolean;
    trustSizeMtime?: boolean;
    onRun?: () => void;
    onCancel?: () => void;
  } = $props();

  let diff = $state<TreeDiffDto | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Format a byte count into a human-readable string. Quick mirror of
  // the recovery server's `human_bytes` helper so the preview reads
  // consistent units across the desktop + web surfaces.
  function humanBytes(n: number): string {
    if (n < 1024) return `${n} B`;
    const units = ["KiB", "MiB", "GiB", "TiB"];
    let value = n / 1024;
    let i = 0;
    while (value >= 1024 && i < units.length - 1) {
      value /= 1024;
      i++;
    }
    return `${value.toFixed(2)} ${units[i]}`;
  }

  $effect(() => {
    let cancelled = false;
    void (async () => {
      loading = true;
      error = null;
      try {
        const dto = await computeTreeDiff(src, dst, {
          forceOverwrite,
          trustSizeMtime,
        });
        if (!cancelled) {
          diff = dto;
        }
      } catch (e) {
        if (!cancelled) {
          error = String(e);
        }
      } finally {
        if (!cancelled) {
          loading = false;
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  function reasonLabel(reason: string): string {
    switch (reason) {
      case "source-newer":
        return t("preview-reason-source-newer");
      case "destination-newer":
        return t("preview-reason-dest-newer");
      case "content-different":
      case "force-overwrite-older":
        return t("preview-reason-content-different");
      case "identical-content":
        return t("preview-reason-identical");
      default:
        return reason;
    }
  }
</script>

<div class="tree-diff-preview" role="dialog" aria-modal="true" aria-labelledby="preview-title">
  <h2 id="preview-title">{t("preview-modal-title")}</h2>

  {#if loading}
    <p class="preview-loading">…</p>
  {:else if error}
    <p class="preview-error">{error}</p>
  {:else if diff}
    <h3>{t("preview-summary-header")}</h3>
    <ul class="preview-summary">
      <li class="cat-add">{t("preview-category-additions", { count: diff.additions.length })}</li>
      <li class="cat-replace">
        {t("preview-category-replacements", { count: diff.replacements.length })}
      </li>
      <li class="cat-skip">{t("preview-category-skips", { count: diff.skips.length })}</li>
      <li class="cat-conflict">
        {t("preview-category-conflicts", { count: diff.conflicts.length })}
      </li>
      <li class="cat-unchanged">
        {t("preview-category-unchanged", { count: diff.unchanged.length })}
      </li>
    </ul>
    <p class="preview-bytes">
      {t("preview-bytes-to-transfer", { bytes: humanBytes(diff.bytesToTransfer) })}
    </p>

    {#if diff.additions.length > 0}
      <details class="preview-rows">
        <summary>{t("preview-category-additions", { count: diff.additions.length })}</summary>
        <ul>
          {#each diff.additions as p}
            <li class="row-add">{p}</li>
          {/each}
        </ul>
      </details>
    {/if}
    {#if diff.replacements.length > 0}
      <details class="preview-rows">
        <summary>{t("preview-category-replacements", { count: diff.replacements.length })}</summary>
        <ul>
          {#each diff.replacements as r}
            <li class="row-replace">{r.relPath} — {reasonLabel(r.reason)}</li>
          {/each}
        </ul>
      </details>
    {/if}
    {#if diff.skips.length > 0}
      <details class="preview-rows">
        <summary>{t("preview-category-skips", { count: diff.skips.length })}</summary>
        <ul>
          {#each diff.skips as s}
            <li class="row-skip">{s.relPath} — {reasonLabel(s.reason)}</li>
          {/each}
        </ul>
      </details>
    {/if}
    {#if diff.conflicts.length > 0}
      <details class="preview-rows" open>
        <summary class="conflict-summary">
          {t("preview-category-conflicts", { count: diff.conflicts.length })}
        </summary>
        <ul>
          {#each diff.conflicts as c}
            <li class="row-conflict">{c.relPath} — {c.kind}</li>
          {/each}
        </ul>
      </details>
    {/if}

    <div class="preview-actions">
      <button
        type="button"
        class="primary"
        disabled={diff.hasBlockingConflicts}
        onclick={onRun}
      >
        {t("preview-button-run")}
      </button>
      <button type="button" onclick={onCancel}>
        {t("preview-button-reduce")}
      </button>
    </div>
  {/if}
</div>

<style>
  .tree-diff-preview {
    padding: 16px;
    background: var(--surface, #fff);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 8px;
    max-width: 720px;
    margin: 0 auto;
    font-family: inherit;
    color: var(--fg, #1f1f1f);
  }
  .tree-diff-preview h2 {
    font-size: 16px;
    margin: 0 0 12px;
  }
  .tree-diff-preview h3 {
    font-size: 13px;
    margin: 12px 0 4px;
    color: var(--fg-dim, #5f5f5f);
  }
  .preview-summary {
    list-style: none;
    padding: 0;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 4px;
    margin: 0;
  }
  .preview-summary li {
    padding: 6px 10px;
    border-radius: 4px;
    font-size: 12px;
  }
  .cat-add { background: rgba(46, 160, 67, 0.14); border-left: 3px solid #2ea043; }
  .cat-replace { background: rgba(255, 196, 77, 0.14); border-left: 3px solid #ffc44d; }
  .cat-skip { background: rgba(128, 128, 128, 0.10); border-left: 3px solid #888; }
  .cat-conflict { background: rgba(220, 53, 69, 0.14); border-left: 3px solid #dc3545; }
  .cat-unchanged { background: rgba(128, 128, 128, 0.06); border-left: 3px solid #aaa; }
  .preview-bytes {
    font-weight: 600;
    margin: 8px 0;
  }
  .preview-rows {
    margin: 4px 0;
    font-size: 12px;
  }
  .preview-rows summary {
    cursor: pointer;
    padding: 4px 0;
  }
  .conflict-summary {
    color: #dc3545;
    font-weight: 600;
  }
  .row-add { color: #2ea043; }
  .row-replace { color: #b8860b; }
  .row-skip { color: var(--fg-dim, #5f5f5f); }
  .row-conflict { color: #dc3545; font-weight: 600; }
  .preview-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }
  .preview-loading {
    color: var(--fg-dim, #5f5f5f);
  }
  .preview-error {
    color: #dc3545;
  }
</style>
