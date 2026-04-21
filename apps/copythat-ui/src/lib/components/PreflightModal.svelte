<!--
  Phase 14 — preflight free-space dialog.

  Appears after the user picks a destination (via drop dialog or CLI
  enqueue) but *before* the copy is actually queued. Runs two probes
  in parallel — `destination_free_bytes` on the destination volume
  and `path_total_bytes` recursively on the source set — then picks
  one of three outcomes:

  - **ok**: required + reserve ≤ free → resolve silently, caller
    proceeds to enqueue. Modal never renders.
  - **warn**: required ≤ free but required would eat into the
    reserve → show a yellow "low space" warning with numbers and a
    "Continue anyway / Cancel" pair.
  - **block**: required > free → show a red "not enough space"
    error with the shortfall and a "Cancel" button. A follow-up
    phase adds the subset-picker here; for now the user has to
    deselect sources manually.

  Caller resolves a promise with the user's answer (true = proceed,
  false = cancel) so the existing enqueue flow stays shape-for-shape
  the same as before.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import { formatBytes } from "../format";
  import { destinationFreeBytes, pathTotalBytes } from "../ipc";

  interface PreflightResult {
    proceed: boolean;
    pickedSubset?: string[];
  }

  interface Props {
    sources: string[];
    destination: string;
    reserveBytes: number;
    onResolve: (result: PreflightResult) => void;
    onOpenSubset: (freeBytes: number) => void;
  }

  let { sources, destination, reserveBytes, onResolve, onOpenSubset }: Props =
    $props();

  let free = $state<number | null>(null);
  let required = $state<number | null>(null);
  let status = $state<"loading" | "ok" | "warn" | "block" | "unknown">(
    "loading",
  );

  $effect(() => {
    void probe();
  });

  async function probe() {
    status = "loading";
    try {
      const [f, r] = await Promise.all([
        destinationFreeBytes(destination).catch(() => 0),
        pathTotalBytes(sources).catch((e) => {
          const msg = e instanceof Error ? e.message : String(e);
          if (msg.includes("too-large")) return -1;
          throw e;
        }),
      ]);
      free = f;
      required = r;
      if (r < 0) {
        // Source tree too big to count quickly. Let the user proceed;
        // the engine's reserve guard (if enabled) is the safety net.
        status = "unknown";
        if (reserveBytes > 0 && f < reserveBytes) {
          // Even an empty source would already breach the reserve.
          status = "block";
        }
        return;
      }
      if (f === 0) {
        // Volume probe failed — couldn't tell. Proceed with warning.
        status = "unknown";
        return;
      }
      if (r > f) {
        status = "block";
      } else if (reserveBytes > 0 && r > f - reserveBytes) {
        status = "warn";
      } else {
        // Fits comfortably — resolve immediately without ever
        // showing the modal. The parent can treat this as "go".
        status = "ok";
        onResolve({ proceed: true });
      }
    } catch {
      // Probe explosion — let the user proceed with a "couldn't
      // determine" notice rather than blocking on a fluky probe.
      status = "unknown";
    }
  }

  function shortfall(): number {
    if (required === null || free === null) return 0;
    if (status === "block") return required - free;
    if (status === "warn") return required - (free - reserveBytes);
    return 0;
  }
</script>

{#if status !== "loading" && status !== "ok"}
  <div class="backdrop" role="presentation">
    {#key $i18nVersion}
      <div
        class="modal"
        role="alertdialog"
        tabindex="-1"
        aria-modal="true"
        aria-labelledby="pre-title"
      >
        <header class="head" data-tone={status}>
          <Icon
            name={status === "block" ? "alert-triangle" : "info"}
            size={18}
          />
          <h2 id="pre-title">
            {status === "block"
              ? t("preflight-block-title")
              : status === "warn"
                ? t("preflight-warn-title")
                : t("preflight-unknown-title")}
          </h2>
        </header>

        {#if status === "unknown"}
          <p class="msg">{t("preflight-unknown-body")}</p>
        {:else}
          <dl class="fields">
            <dt>{t("preflight-required")}</dt>
            <dd class="tabular">
              {required !== null ? formatBytes(required) : "—"}
            </dd>
            <dt>{t("preflight-free")}</dt>
            <dd class="tabular">
              {free !== null ? formatBytes(free) : "—"}
            </dd>
            {#if reserveBytes > 0}
              <dt>{t("preflight-reserve")}</dt>
              <dd class="tabular">{formatBytes(reserveBytes)}</dd>
            {/if}
            {#if shortfall() > 0}
              <dt class="warn">{t("preflight-shortfall")}</dt>
              <dd class="warn tabular">{formatBytes(shortfall())}</dd>
            {/if}
          </dl>
        {/if}

        <div class="actions">
          <button
            class="secondary"
            type="button"
            onclick={() => onResolve({ proceed: false })}
          >
            {t("action-cancel")}
          </button>
          {#if status === "block" && sources.length > 1 && free !== null}
            <button
              class="primary"
              type="button"
              onclick={() => onOpenSubset(free ?? 0)}
            >
              {t("preflight-pick-subset")}
            </button>
          {:else if status !== "block"}
            <button
              class="primary"
              type="button"
              onclick={() => onResolve({ proceed: true })}
            >
              {t("preflight-continue")}
            </button>
          {/if}
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
    z-index: 98;
  }

  .modal {
    width: min(460px, 92vw);
    padding: 14px 16px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.24);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .head[data-tone="block"] {
    color: var(--error, #c24141);
  }

  .head[data-tone="warn"] {
    color: var(--warn, #b4781b);
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .fields {
    display: grid;
    grid-template-columns: max-content 1fr;
    column-gap: 12px;
    row-gap: 4px;
    margin: 4px 0;
    font-size: 12px;
  }

  dt {
    color: var(--fg-dim, #6a6a6a);
  }

  dd {
    margin: 0;
  }

  .tabular {
    font-variant-numeric: tabular-nums;
  }

  .warn {
    color: var(--error, #c24141);
    font-weight: 600;
  }

  .msg {
    margin: 4px 0;
    font-size: 12px;
    color: var(--fg-dim, #4a4a4a);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
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

  button.primary {
    background: var(--accent, #3261ff);
    color: white;
    border-color: transparent;
  }

  button.secondary {
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
  }
</style>
