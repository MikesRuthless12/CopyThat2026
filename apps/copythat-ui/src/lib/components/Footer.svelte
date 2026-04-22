<!--
  Fixed 32 px strip along the bottom. Three counters + a History
  button wired to the Phase 9 drawer.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import {
    globals,
    openErrorLogDrawer,
    openHistoryDrawer,
    openSettings,
    openTotalsDrawer,
    pushToast,
  } from "../stores";

  // `t()` reads from the locale store non-reactively. Svelte only
  // re-runs a template expression when a reactive value it actually
  // *uses* changes, so we bind `$locale.code` to a `data-locale`
  // attribute on the root below — that forces the whole template
  // (and every `t(...)` call inside it) to re-evaluate on language
  // switch or initial hydration.
  import { formatBytes } from "../format";
  import { postCompletionAction, type PostCompletionAction } from "../ipc";

  let g = $derived($globals);
  let total = $derived(formatBytes(g.bytesTotal));

  // When-done action. Persisted to localStorage so it survives a
  // full app restart; defaults to keep-open so nothing surprising
  // fires unless the user opts in.
  const ACTION_KEY = "copythat-after-done";
  let afterDone: PostCompletionAction = $state(
    (typeof localStorage !== "undefined"
      ? (localStorage.getItem(ACTION_KEY) as PostCompletionAction | null)
      : null) ?? "keep-open",
  );
  // Track whether we've ever seen work, so a fresh launch with no
  // jobs doesn't immediately fire a "done" event.
  let sawWork = $state(false);
  // Fire-once latch per "run". Reset when new work arrives.
  let alreadyFired = $state(false);

  $effect(() => {
    const active = g.activeJobs + g.queuedJobs + g.pausedJobs;
    if (active > 0) {
      sawWork = true;
      alreadyFired = false;
      return;
    }
    if (!sawWork || alreadyFired) return;
    if (afterDone === "keep-open") {
      alreadyFired = true;
      return;
    }
    alreadyFired = true;
    postCompletionAction(afterDone).catch((e) => {
      pushToast("error", e instanceof Error ? e.message : String(e));
    });
  });

  function onAfterDoneChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    afterDone = target.value as PostCompletionAction;
    try {
      localStorage.setItem(ACTION_KEY, afterDone);
    } catch {
      // Best-effort — a private-mode browser can reject setItem.
    }
    // A fresh pick resets the fire-once latch so if the user flips
    // the selector *while* jobs are already done, the new action
    // still fires when/if the queue empties again.
    alreadyFired = false;
  }
</script>

<footer class="footer">
  {#key $i18nVersion}
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
    <label class="after-done" title={t("activity-after-done")}>
      <span>{t("activity-after-done")}</span>
      <select value={afterDone} onchange={onAfterDoneChange}>
        <option value="keep-open">{t("activity-keep-open")}</option>
        <option value="exit">{t("activity-close-app")}</option>
        <option value="shutdown">{t("activity-shutdown")}</option>
        <option value="logoff">{t("activity-logoff")}</option>
        <option value="sleep">{t("activity-sleep")}</option>
      </select>
    </label>
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
  {/key}
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

  .after-done {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--fg-dim, #5f5f5f);
  }

  .after-done select {
    font: inherit;
    font-size: 11px;
    padding: 2px 4px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    cursor: pointer;
  }
</style>
