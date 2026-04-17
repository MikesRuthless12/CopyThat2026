<!--
  Modal that appears after a drop: lists the dropped paths and
  asks the user to pick a destination. Clicking "Pick destination"
  opens the Tauri dialog plugin's directory picker; once chosen,
  the command layer enqueues one `copy` job per source.
-->
<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { startCopy, startMove } from "../ipc";
  import { clearDropped, pushToast } from "../stores";

  interface Props {
    paths: string[];
  }

  let { paths }: Props = $props();

  let destination: string | null = $state(null);
  let kind: "copy" | "move" = $state("copy");
  let busy = $state(false);

  async function pick() {
    const chosen = await open({ directory: true, multiple: false });
    if (typeof chosen === "string" && chosen.trim().length > 0) {
      destination = chosen;
    }
  }

  async function confirm() {
    if (!destination) return;
    busy = true;
    try {
      if (kind === "copy") {
        await startCopy(paths, destination);
      } else {
        await startMove(paths, destination);
      }
      pushToast("info", kind === "copy" ? "toast-copy-queued" : "toast-move-queued");
      clearDropped();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  function cancel() {
    clearDropped();
  }
</script>

<div
  class="backdrop"
  role="presentation"
  onclick={cancel}
  onkeydown={(e) => {
    if (e.key === "Escape") cancel();
  }}
>
  <div
    class="modal"
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-label={t("drop-dialog-title")}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <header>
      <h2>{t("drop-dialog-title")}</h2>
      <button
        type="button"
        class="close"
        aria-label={t("action-close")}
        onclick={cancel}
      >
        <Icon name="x" size={16} />
      </button>
    </header>
    <p class="sub">{t("drop-dialog-subtitle", { count: paths.length })}</p>
    <ul class="sources">
      {#each paths as p}
        <li>{p}</li>
      {/each}
    </ul>

    <div class="mode" role="radiogroup" aria-label={t("drop-dialog-mode")}>
      <label>
        <input
          type="radio"
          name="kind"
          value="copy"
          bind:group={kind}
        />
        {t("drop-dialog-copy")}
      </label>
      <label>
        <input
          type="radio"
          name="kind"
          value="move"
          bind:group={kind}
        />
        {t("drop-dialog-move")}
      </label>
    </div>

    <div class="dest">
      <button type="button" class="pick" onclick={pick} disabled={busy}>
        <Icon name="folder" size={14} />
        {destination
          ? t("drop-dialog-change-destination")
          : t("drop-dialog-pick-destination")}
      </button>
      {#if destination}
        <span class="path" title={destination}>{destination}</span>
      {/if}
    </div>

    <div class="actions">
      <button class="secondary" type="button" onclick={cancel} disabled={busy}>
        {t("action-cancel")}
      </button>
      <button
        class="primary"
        type="button"
        onclick={confirm}
        disabled={busy || !destination}
      >
        {kind === "copy"
          ? t("drop-dialog-start-copy")
          : t("drop-dialog-start-move")}
      </button>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 90;
  }

  .modal {
    width: min(440px, 92vw);
    max-height: 82vh;
    padding: 14px 16px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  h2 {
    margin: 0;
    font-size: 14px;
  }

  .close {
    margin-left: auto;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .sub {
    margin: 0;
    font-size: 12px;
    color: var(--fg-dim, #6a6a6a);
  }

  .sources {
    margin: 0;
    padding: 0 0 0 16px;
    max-height: 120px;
    overflow: auto;
    font-size: 11px;
    color: var(--fg-dim, #6a6a6a);
  }

  .mode {
    display: flex;
    gap: 16px;
    font-size: 12px;
  }

  .mode label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
  }

  .dest {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
    font-size: 12px;
  }

  .pick {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: var(--hover, rgba(128, 128, 128, 0.12));
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 6px;
    color: inherit;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .pick:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .path {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--fg-dim, #6a6a6a);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  button.primary,
  button.secondary {
    padding: 6px 12px;
    border-radius: 6px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
    border: 1px solid transparent;
  }

  button.secondary {
    background: transparent;
    border-color: var(--border, rgba(128, 128, 128, 0.3));
    color: inherit;
  }

  button.primary {
    background: var(--accent, #4f8cff);
    color: #ffffff;
  }

  button.primary:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
</style>
