<!--
  Phase 8 — error prompt modal.

  Rendered when `$errorQueue` is non-empty. The head of the queue
  drives the modal contents; clicking an action fires
  `resolve_error` + the Rust side pops the queue through the
  `error-resolved` echo event. A single checkbox toggles
  "skip all errors of this kind" which caches the Skip decision
  per (job_id, error_kind) in the Rust registry.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import { resolveError, retryElevated } from "../ipc";
  import { errorQueue, pushToast } from "../stores";

  let applyToAll = $state(false);
  let busy = $state(false);

  const head = $derived($errorQueue[0] ?? null);

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
  <div
    class="backdrop"
    role="presentation"
    onkeydown={(e) => {
      if (e.key === "Escape") run("skip");
    }}
  >
    {#key $i18nVersion}
    <div
      class="modal"
      role="alertdialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="err-title"
    >
      <header>
        <Icon name="x" size={16} />
        <h2 id="err-title">{t("error-modal-title")}</h2>
      </header>

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
    </div>
    {/key}
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 95;
  }

  .modal {
    width: min(520px, 94vw);
    padding: 14px 16px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.24);
  }

  header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--danger, #c43030);
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .fields {
    display: grid;
    grid-template-columns: auto 1fr;
    column-gap: 10px;
    row-gap: 4px;
    margin: 10px 0 8px;
    font-size: 12px;
  }

  dt {
    color: var(--muted, #6a6a6a);
    font-weight: 500;
  }

  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }

  dd.path {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 11px;
  }

  .os {
    color: var(--muted, #6a6a6a);
  }

  .msg {
    margin: 4px 0 10px;
    font-size: 12px;
    color: var(--muted, #4a4a4a);
    padding: 6px 8px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.03));
    border-radius: 6px;
  }

  .apply-all {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    margin: 4px 0 10px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: 8px;
  }

  button {
    font-size: 12px;
    padding: 6px 12px;
    border-radius: 6px;
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
