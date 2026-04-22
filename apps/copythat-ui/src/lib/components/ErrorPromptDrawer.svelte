<!--
  Phase 8 — non-blocking error prompt drawer.

  Alternative render path for the error queue, picked when
  `general.error_display_mode == "drawer"`. Same actions and
  apply-to-all shape as ErrorModal; the difference is visual: a
  corner panel docks in the bottom-right without a backdrop, so
  the queue keeps advancing while the user triages. When more than
  one error is pending, a count chip surfaces behind the head entry.

  Rendered in parallel with the rest of the UI — no `aria-modal`,
  no focus trap, no Esc-to-skip. Esc hides the drawer without
  resolving, letting the user come back to it later.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import { resolveError, retryElevated } from "../ipc";
  import { errorQueue, pushToast } from "../stores";

  let applyToAll = $state(false);
  let busy = $state(false);
  let collapsed = $state(false);

  const head = $derived($errorQueue[0] ?? null);
  const pendingMore = $derived(Math.max(0, $errorQueue.length - 1));

  async function run(action: "retry" | "skip" | "abort") {
    if (!head || busy) return;
    busy = true;
    try {
      await resolveError(head.id, action, applyToAll && action === "skip");
      applyToAll = false;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function elevated() {
    if (!head || busy) return;
    try {
      await retryElevated(head.id);
    } catch {
      pushToast("info", "toast-elevated-unavailable");
    }
  }
</script>

{#if head}
  {#key $i18nVersion}
    <aside
      class="drawer"
      role="region"
      aria-label={t("error-modal-title")}
      aria-live="polite"
      class:collapsed
    >
      <header>
        <Icon name="x" size={14} />
        <h2>{t("error-modal-title")}</h2>
        {#if pendingMore > 0}
          <span class="badge" title={t("error-drawer-pending-count")}>
            +{pendingMore}
          </span>
        {/if}
        <button
          class="collapse"
          type="button"
          aria-label={collapsed ? t("action-close") : t("action-close")}
          onclick={() => (collapsed = !collapsed)}
          title={t("error-drawer-toggle")}
        >
          <Icon name={collapsed ? "info" : "x"} size={12} />
        </button>
      </header>

      {#if !collapsed}
        <dl class="fields">
          <dt>{t("error-modal-path-label")}</dt>
          <dd class="path" title={head.src}>{head.src}</dd>

          <dt>{t("error-modal-code-label")}</dt>
          <dd>
            {t(head.localizedKey)}
            {#if head.rawOsError !== null}
              <span class="os">(os {head.rawOsError})</span>
            {/if}
          </dd>
        </dl>

        {#if head.message}
          <p class="msg">{head.message}</p>
        {/if}

        <label class="apply-all">
          <input type="checkbox" bind:checked={applyToAll} disabled={busy} />
          {t("error-modal-skip-all-kind")}
        </label>

        <div class="actions">
          <button
            class="secondary"
            type="button"
            onclick={() => run("abort")}
            disabled={busy}
          >
            {t("error-modal-abort")}
          </button>
          <button
            class="secondary"
            type="button"
            onclick={elevated}
            disabled={busy}
          >
            {t("error-modal-retry-elevated")}
          </button>
          <button
            class="secondary"
            type="button"
            onclick={() => run("skip")}
            disabled={busy}
          >
            {t("error-modal-skip")}
          </button>
          <button
            class="primary"
            type="button"
            onclick={() => run("retry")}
            disabled={busy}
          >
            {t("error-modal-retry")}
          </button>
        </div>
      {/if}
    </aside>
  {/key}
{/if}

<style>
  .drawer {
    position: fixed;
    right: 16px;
    bottom: 16px;
    width: min(420px, calc(100vw - 32px));
    padding: 10px 12px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--error, #d95757);
    border-left-width: 3px;
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
    /* Sit above the progress bar / footer but below the blocking
       collision modal (z-index 94) so a pending collision always
       wins the visual stack. */
    z-index: 80;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .drawer.collapsed {
    gap: 0;
  }

  header {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--error, #c43030);
  }

  h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    flex: 1;
  }

  .badge {
    font-size: 11px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 999px;
    background: var(--error, #d95757);
    color: white;
  }

  .collapse {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    padding: 2px 4px;
    border-radius: 4px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
  }

  .collapse:hover {
    background: var(--hover, rgba(0, 0, 0, 0.06));
  }

  .fields {
    display: grid;
    grid-template-columns: auto 1fr;
    column-gap: 8px;
    row-gap: 3px;
    margin: 2px 0 4px;
    font-size: 11.5px;
  }

  dt {
    color: var(--fg-dim, #6a6a6a);
    font-weight: 500;
  }

  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }

  dd.path {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 10.5px;
  }

  .os {
    color: var(--fg-dim, #6a6a6a);
  }

  .msg {
    margin: 2px 0 6px;
    font-size: 11px;
    color: var(--fg-dim, #4a4a4a);
    padding: 4px 6px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.03));
    border-radius: 4px;
  }

  .apply-all {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    margin: 2px 0 6px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: 6px;
  }

  button {
    font-size: 11.5px;
    padding: 4px 10px;
    border-radius: 5px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.primary {
    background: var(--accent, #3261ff);
    color: white;
    border-color: transparent;
  }

  button.secondary {
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
  }
</style>
