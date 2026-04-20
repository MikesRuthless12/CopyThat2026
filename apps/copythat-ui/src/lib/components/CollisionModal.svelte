<!--
  Phase 8 — collision prompt modal.

  Rendered when `$collisionQueue` is non-empty. Shows the source and
  destination previews (name / size / mtime) side-by-side; the user
  picks Overwrite / Skip / Keep both / Rename. "Apply to all" caches
  the resolution per job_id in the Rust registry so a tree with many
  collisions doesn't nag every time.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { t } from "../i18n";
  import { formatBytes } from "../format";
  import { resolveCollision } from "../ipc";
  import { collisionQueue, pushToast } from "../stores";

  let applyToAll = $state(false);
  let busy = $state(false);
  let renameMode = $state(false);
  let renameValue = $state("");

  const head = $derived($collisionQueue[0] ?? null);

  function basename(p: string): string {
    const idx = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    return idx >= 0 ? p.slice(idx + 1) : p;
  }

  function fmtDate(ms: number | null): string {
    if (ms === null) return "—";
    try {
      return new Date(ms).toLocaleString();
    } catch {
      return "—";
    }
  }

  async function run(
    resolution: "overwrite" | "skip" | "abort",
  ): Promise<void> {
    if (!head || busy) return;
    busy = true;
    try {
      await resolveCollision(head.id, resolution, null, applyToAll);
      applyToAll = false;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function keepBoth() {
    // Keep-both is not a direct engine resolution — it's the caller
    // picking a unique filename. We surface it by auto-generating
    // `name (1).ext` and sending Rename. The engine's `KeepBoth`
    // policy lives at the TreeOptions level; this is the per-prompt
    // override.
    if (!head || busy) return;
    busy = true;
    try {
      const fresh = withSuffix(basename(head.dst));
      await resolveCollision(head.id, "rename", fresh, applyToAll);
      applyToAll = false;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function confirmRename() {
    if (!head || busy || !renameValue.trim()) return;
    busy = true;
    try {
      await resolveCollision(head.id, "rename", renameValue.trim(), applyToAll);
      renameMode = false;
      renameValue = "";
      applyToAll = false;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  function withSuffix(name: string): string {
    const dot = name.lastIndexOf(".");
    const base = dot > 0 ? name.slice(0, dot) : name;
    const ext = dot > 0 ? name.slice(dot) : "";
    return `${base} (1)${ext}`;
  }

  function toggleRename() {
    renameMode = !renameMode;
    if (renameMode && head && !renameValue) {
      renameValue = withSuffix(basename(head.dst));
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
    <div
      class="modal"
      role="alertdialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="coll-title"
    >
      <header>
        <Icon name="info" size={16} />
        <h2 id="coll-title">{t("collision-modal-title")}</h2>
      </header>

      <div class="panes">
        <section>
          <h3>{t("collision-modal-source")}</h3>
          <dl>
            <dt>{t("error-modal-path-label")}</dt>
            <dd class="path" title={head.src}>{head.src}</dd>
            <dt>{t("collision-modal-size")}</dt>
            <dd>{head.srcSize === null ? "—" : formatBytes(head.srcSize)}</dd>
            <dt>{t("collision-modal-modified")}</dt>
            <dd>{fmtDate(head.srcModifiedMs)}</dd>
          </dl>
        </section>
        <section>
          <h3>{t("collision-modal-destination")}</h3>
          <dl>
            <dt>{t("error-modal-path-label")}</dt>
            <dd class="path" title={head.dst}>{head.dst}</dd>
            <dt>{t("collision-modal-size")}</dt>
            <dd>{head.dstSize === null ? "—" : formatBytes(head.dstSize)}</dd>
            <dt>{t("collision-modal-modified")}</dt>
            <dd>{fmtDate(head.dstModifiedMs)}</dd>
          </dl>
        </section>
      </div>

      {#if renameMode}
        <div class="rename">
          <label>
            {t("collision-modal-rename-placeholder")}
            <input
              type="text"
              bind:value={renameValue}
              disabled={busy}
              aria-label={t("collision-modal-rename-placeholder")}
            />
          </label>
          <button
            class="primary"
            type="button"
            onclick={confirmRename}
            disabled={busy || !renameValue.trim()}
          >
            {t("collision-modal-confirm-rename")}
          </button>
        </div>
      {/if}

      <label class="apply-all">
        <input type="checkbox" bind:checked={applyToAll} disabled={busy} />
        {t("collision-modal-apply-to-all")}
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
          onclick={toggleRename}
          disabled={busy}
        >
          {t("collision-modal-rename")}
        </button>
        <button
          class="secondary"
          type="button"
          onclick={keepBoth}
          disabled={busy}
        >
          {t("collision-modal-keep-both")}
        </button>
        <button
          class="secondary"
          type="button"
          onclick={() => run("skip")}
          disabled={busy}
        >
          {t("collision-modal-skip")}
        </button>
        <button
          class="primary"
          type="button"
          onclick={() => run("overwrite")}
          disabled={busy}
        >
          {t("collision-modal-overwrite")}
        </button>
      </div>
    </div>
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
    z-index: 94;
  }

  .modal {
    width: min(680px, 96vw);
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
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .panes {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    margin: 10px 0;
  }

  .panes section {
    padding: 8px 10px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 6px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.03));
  }

  h3 {
    margin: 0 0 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--muted, #6a6a6a);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  dl {
    margin: 0;
    display: grid;
    grid-template-columns: auto 1fr;
    column-gap: 8px;
    row-gap: 3px;
    font-size: 11.5px;
  }

  dt {
    color: var(--muted, #6a6a6a);
  }

  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }

  dd.path {
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 10.5px;
  }

  .rename {
    display: flex;
    align-items: end;
    gap: 8px;
    padding: 8px 0;
  }

  .rename label {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 11.5px;
  }

  .rename input {
    padding: 4px 6px;
    font-size: 12px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: inherit;
  }

  .apply-all {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    margin: 6px 0 10px;
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
