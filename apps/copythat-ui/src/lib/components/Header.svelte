<!--
  Fixed 56 px strip along the top. Carries the global state pill,
  throughput, ETA, and Pause-all / Resume-all / Cancel-all controls.

  Binds directly to the `globals` store so the rate is always the
  sum of live per-job rates (see `stores.ts::liveRateBps`).
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import StateBadge from "./StateBadge.svelte";
  import { locale, setLocale, t } from "../i18n";
  import { globals, liveRateBps } from "../stores";
  import { pauseAll, resumeAll, cancelAll } from "../ipc";
  import { formatEta, formatRate } from "../format";

  let g = $derived($globals);
  let rate = $derived($liveRateBps);
  let etaDisplay = $derived(formatEta(g.etaSeconds, t));
  let hasWork = $derived(
    g.state !== "idle" && g.activeJobs + g.queuedJobs + g.pausedJobs > 0,
  );

  // Phase 11a — temporary language dropdown. Folds into Settings →
  // General when Phase 12 lands the Settings window.
  const LANGUAGE_LABELS: Record<string, string> = {
    en: "English",
    es: "Español",
    "zh-CN": "中文 (简)",
    hi: "हिन्दी",
    ar: "العربية",
    "pt-BR": "Português (BR)",
    ru: "Русский",
    ja: "日本語",
    de: "Deutsch",
    fr: "Français",
    ko: "한국어",
    it: "Italiano",
    tr: "Türkçe",
    vi: "Tiếng Việt",
    pl: "Polski",
    nl: "Nederlands",
    id: "Bahasa Indonesia",
    uk: "Українська",
  };

  async function onLocaleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    await setLocale(target.value);
  }
</script>

<header class="header" aria-label={t("window-title")}>
  <div class="brand" aria-hidden="true">
    <Icon name="rocket" size={22} />
  </div>
  <div class="summary">
    <StateBadge state={g.state} size="md" />
    <div class="metrics">
      <span class="metric rate">{formatRate(rate)}</span>
      <span class="metric eta" aria-label={t("header-eta-label")}>
        {etaDisplay}
      </span>
    </div>
  </div>
  <div class="actions" role="toolbar" aria-label={t("header-toolbar-label")}>
    <!--
      Phase 11a — temporary language dropdown. Lives here until the
      Phase 12 Settings window can host a proper Appearance → Language
      control. Keep the markup minimal: a <select> hot-swaps the
      locale without a restart via `setLocale`.
    -->
    <label
      class="lang"
      title={t("header-language-title")}
      aria-label={t("header-language-label")}
    >
      <Icon name="info" size={14} />
      <select
        value={$locale.code}
        onchange={onLocaleChange}
        aria-label={t("header-language-label")}
      >
        {#each $locale.available as code (code)}
          <option value={code}>{LANGUAGE_LABELS[code] ?? code}</option>
        {/each}
      </select>
    </label>
    <button
      type="button"
      class="icon-btn"
      aria-label={t("action-pause-all")}
      title={t("action-pause-all")}
      disabled={!hasWork}
      onclick={() => pauseAll()}
    >
      <Icon name="pause" size={18} />
    </button>
    <button
      type="button"
      class="icon-btn"
      aria-label={t("action-resume-all")}
      title={t("action-resume-all")}
      disabled={g.pausedJobs === 0}
      onclick={() => resumeAll()}
    >
      <Icon name="play" size={18} />
    </button>
    <button
      type="button"
      class="icon-btn danger"
      aria-label={t("action-cancel-all")}
      title={t("action-cancel-all")}
      disabled={!hasWork}
      onclick={() => cancelAll()}
    >
      <Icon name="x" size={18} />
    </button>
  </div>
</header>

<style>
  .header {
    height: 56px;
    min-height: 56px;
    display: grid;
    grid-template-columns: 28px 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 0 14px;
    background: var(--header-bg, var(--surface, #fafafa));
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    box-sizing: border-box;
  }

  .brand {
    color: var(--accent, #4f8cff);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .summary {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .metrics {
    display: flex;
    align-items: baseline;
    gap: 10px;
    min-width: 0;
    font-variant-numeric: tabular-nums;
  }

  .metric {
    font-size: 13px;
    line-height: 1.2;
    color: var(--fg-strong, #1f1f1f);
  }

  .metric.rate {
    font-weight: 600;
  }

  .metric.eta {
    color: var(--fg-dim, #5f5f5f);
  }

  .actions {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  /* Phase 11a temporary locale picker — neutral styling so it
     doesn't compete with the primary action buttons. */
  .lang {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 6px;
    margin-right: 4px;
    border-radius: 6px;
    color: var(--fg-dim, #6a6a6a);
    cursor: pointer;
  }

  .lang select {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    font: inherit;
    font-size: 12px;
    padding: 2px 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .lang select:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .lang select:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: 2px;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--fg, #1f1f1f);
    cursor: pointer;
    padding: 0;
    transition:
      background 120ms,
      color 120ms,
      border-color 120ms;
  }

  .icon-btn:hover:not(:disabled) {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: 2px;
  }

  .icon-btn.danger:hover:not(:disabled) {
    color: var(--error, #c24141);
  }

  .icon-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }
</style>
