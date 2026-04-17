<!--
  Stacked toast notifications anchored bottom-right. Clicking a
  toast dismisses it early; auto-dismiss fires after the timeout
  declared when the toast was pushed.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { toasts, dismissToast } from "../stores";
</script>

<div class="stack" aria-live="polite" role="status">
  {#each $toasts as toast (toast.id)}
    <button
      class="toast"
      type="button"
      data-kind={toast.kind}
      onclick={() => dismissToast(toast.id)}
    >
      <span class="icon">
        {#if toast.kind === "success"}
          <Icon name="rocket" size={14} />
        {:else if toast.kind === "error"}
          <Icon name="alert-triangle" size={14} />
        {:else}
          <Icon name="info" size={14} />
        {/if}
      </span>
      <span class="body">{isKey(toast.message) ? t(toast.message) : toast.message}</span>
    </button>
  {/each}
</div>

<script module lang="ts">
  // Treat messages that look like Fluent keys (`kebab-case-lowercase`)
  // as lookup requests; pass arbitrary text through unchanged. That
  // lets the store push either Fluent keys or engine error strings
  // and each renders sensibly.
  export function isKey(s: string): boolean {
    return /^[a-z][a-z0-9-]*$/.test(s);
  }
</script>

<style>
  .stack {
    position: fixed;
    right: 14px;
    bottom: 44px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    z-index: 80;
    pointer-events: none;
  }

  .toast {
    pointer-events: auto;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 6px;
    font: inherit;
    font-size: 12px;
    box-shadow: 0 3px 12px rgba(0, 0, 0, 0.12);
    max-width: 320px;
    text-align: left;
    cursor: pointer;
    animation: slideIn 180ms ease-out;
  }

  .toast[data-kind="success"] {
    border-color: rgba(63, 175, 106, 0.45);
    color: var(--ok, #2f8c55);
  }
  .toast[data-kind="error"] {
    border-color: rgba(217, 87, 87, 0.5);
    color: var(--error, #c24141);
  }

  .icon {
    display: inline-flex;
    align-items: center;
  }

  .body {
    flex: 1;
    word-break: break-word;
  }

  @keyframes slideIn {
    from {
      transform: translateY(12px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
</style>
