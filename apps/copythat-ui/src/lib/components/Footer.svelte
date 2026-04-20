<!--
  Fixed 32 px strip along the bottom. Three counters + a History
  button wired to the Phase 9 drawer.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import {
    globals,
    openErrorLogDrawer,
    openHistoryDrawer,
    openSettings,
    openTotalsDrawer,
  } from "../stores";
  import { formatBytes } from "../format";

  let g = $derived($globals);
  let total = $derived(formatBytes(g.bytesTotal));
</script>

<footer class="footer">
  <span class="stat">
    <strong>{g.queuedJobs + g.activeJobs + g.pausedJobs}</strong>
    {t("footer-queued")}
  </span>
  <span class="stat" aria-label={t("footer-total-bytes")}>
    <strong>{total}</strong>
    {t("footer-total-bytes")}
  </span>
  <!--
    Phase 8: clicking the error counter opens the log drawer so the
    user can inspect every logged failure and export CSV / TXT.
  -->
  <button
    class="stat errors"
    data-tone={g.errors > 0 ? "error" : "muted"}
    type="button"
    onclick={openErrorLogDrawer}
    aria-label={t("error-log-title")}
  >
    <strong>{g.errors}</strong>
    {t("footer-errors")}
  </button>
  <span class="spacer"></span>
  <button
    class="history"
    type="button"
    onclick={openTotalsDrawer}
    aria-label={t("footer-totals")}
  >
    <Icon name="info" size={14} />
    {t("footer-totals")}
  </button>
  <button
    class="history"
    type="button"
    onclick={openHistoryDrawer}
    aria-label={t("footer-history")}
  >
    <Icon name="external-link" size={14} />
    {t("footer-history")}
  </button>
  <!--
    Phase 11b — Settings entry point. Icon-only button; the Footer
    is tight on horizontal room so the gear sits without a text
    label. Phase 12 can promote it to a labelled button if user
    research shows people miss it.
  -->
  <button
    class="history icon-only"
    type="button"
    onclick={openSettings}
    aria-label={t("settings-title")}
    title={t("settings-title")}
  >
    <Icon name="settings" size={14} />
  </button>
</footer>

<style>
  .footer {
    height: 32px;
    min-height: 32px;
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 0 14px;
    font-size: 11px;
    color: var(--fg-dim, #5f5f5f);
    background: var(--footer-bg, var(--surface, #fafafa));
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    box-sizing: border-box;
  }

  .stat {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-variant-numeric: tabular-nums;
  }

  .stat strong {
    color: var(--fg-strong, #1f1f1f);
    font-weight: 600;
  }

  .stat[data-tone="error"] strong {
    color: var(--error, #c24141);
  }

  button.stat.errors {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    font: inherit;
    padding: 2px 6px;
    border-radius: 4px;
    cursor: pointer;
  }

  button.stat.errors:hover {
    background: var(--surface-alt, rgba(0, 0, 0, 0.05));
  }

  .spacer {
    flex: 1;
  }

  .history {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: inherit;
    font: inherit;
    cursor: pointer;
  }

  .history:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .history.icon-only {
    padding: 4px 6px;
    color: var(--fg-dim, #5f5f5f);
  }

  .history.icon-only:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
    color: var(--fg-strong, #1f1f1f);
  }
</style>
