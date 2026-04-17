<!--
  Zero-state shown when the queue is empty. Doubles as the drop
  target: the `<main>` wrapper listens for the Rust-side
  `drop-received` event which is what actually populates the queue.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";

  interface Props {
    title?: string;
    hint?: string;
  }

  let { title, hint }: Props = $props();
</script>

<div class="empty" role="region" aria-label={t("empty-region-label")}>
  <div class="glyph" aria-hidden="true">
    <Icon name="upload" size={32} />
  </div>
  <p class="title">{title ?? t("empty-title")}</p>
  <p class="hint">{hint ?? t("empty-hint")}</p>
</div>

<style>
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 32px 16px;
    color: var(--fg-dim, #6a6a6a);
    min-height: 240px;
  }

  .glyph {
    color: var(--accent, #4f8cff);
    opacity: 0.8;
  }

  .title {
    margin: 4px 0 0;
    font-size: 14px;
    color: var(--fg, #1f1f1f);
    font-weight: 600;
  }

  .hint {
    margin: 0;
    font-size: 12px;
  }
</style>
