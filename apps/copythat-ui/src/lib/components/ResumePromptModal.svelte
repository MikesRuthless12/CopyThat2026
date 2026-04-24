<!--
  Phase 20 — resume prompt for unfinished jobs.

  Shown once per app launch when `pending_resumes()` returns a
  non-empty list and the user has not turned on
  `general.autoResumeInterrupted` in Settings → General. One row per
  unfinished job; the user can resume / discard each individually or
  use the header's bulk actions.

  Resume re-enqueues the job through the existing `start_copy` /
  `start_move` IPC commands; the runner's journal-sink wiring picks
  up the existing redb row by way of the engine's `decide_resume`
  probe (the prefix bytes already on disk are kept, only the tail
  gets copied).

  Discard calls `discard_resume(rowId)` which drops the row +
  per-file checkpoints from the journal; the prompt won't re-appear
  on the next launch.
-->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import type { PendingResumeDto } from "../types";
  import { t } from "../i18n";

  let { rows, autoResume = false, onClose }: {
    rows: PendingResumeDto[];
    autoResume?: boolean;
    onClose: () => void;
  } = $props();

  // When the user has flipped "Auto-resume" on, the modal short-
  // circuits: silently re-enqueue every row and never paint the
  // prompt. The caller (`+page.svelte`) checks the setting before
  // showing the modal at all, but we double-check here so a future
  // direct-mount case can't bypass the preference.
  $effect(() => {
    if (autoResume && rows.length > 0) {
      void resumeAll();
    }
  });

  function fmtBytes(n: number): string {
    if (n < 1024) return `${n} B`;
    if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KiB`;
    if (n < 1024 * 1024 * 1024) return `${(n / 1024 / 1024).toFixed(1)} MiB`;
    return `${(n / 1024 / 1024 / 1024).toFixed(2)} GiB`;
  }

  function fmtAge(ms: number): string {
    if (ms === 0) return "—";
    const elapsed = Date.now() - ms;
    if (elapsed < 60_000) return `${Math.floor(elapsed / 1000)}s ago`;
    if (elapsed < 3_600_000) return `${Math.floor(elapsed / 60_000)}m ago`;
    if (elapsed < 86_400_000) return `${Math.floor(elapsed / 3_600_000)}h ago`;
    return `${Math.floor(elapsed / 86_400_000)}d ago`;
  }

  function pct(row: PendingResumeDto): number {
    if (row.bytesTotal === 0) return 0;
    return Math.round((row.bytesDone / row.bytesTotal) * 100);
  }

  async function resume(row: PendingResumeDto) {
    if (!row.dstRoot) {
      // Delete / shred jobs have no resume strategy in Phase 20.
      // Discard them so the modal stops surfacing them.
      await discard(row);
      return;
    }
    const cmd = row.kind === "move" ? "start_move" : "start_copy";
    try {
      await invoke<number[]>(cmd, {
        sources: [row.srcRoot],
        destination: row.dstRoot,
      });
      // Drop the row from the modal once re-enqueued; the journal
      // entry stays alive so the new copy_file invocation's
      // `decide_resume` finds the existing checkpoint.
      rows = rows.filter((r) => r.rowId !== row.rowId);
      if (rows.length === 0) onClose();
    } catch (err) {
      console.error("[resume]", err);
    }
  }

  async function discard(row: PendingResumeDto) {
    await invoke("discard_resume", { rowId: row.rowId });
    rows = rows.filter((r) => r.rowId !== row.rowId);
    if (rows.length === 0) onClose();
  }

  async function resumeAll() {
    for (const r of [...rows]) await resume(r);
  }

  async function discardAll() {
    await invoke<string[]>("discard_all_resumes");
    rows = [];
    onClose();
  }
</script>

{#if rows.length > 0 && !autoResume}
  <div class="backdrop" role="presentation" onclick={onClose}>
    <div
      class="modal"
      role="dialog"
      aria-labelledby="resume-title"
      tabindex={-1}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => {
        if (e.key === "Escape") onClose();
      }}
    >
      <header>
        <h2 id="resume-title">{t("resume-prompt-title")}</h2>
        <p class="hint">{t("resume-prompt-body", { count: rows.length })}</p>
        <div class="bulk">
          <button type="button" onclick={() => void resumeAll()}>
            {t("resume-prompt-resume-all")}
          </button>
          <button type="button" class="danger" onclick={() => void discardAll()}>
            {t("resume-discard-all")}
          </button>
        </div>
      </header>

      <ul class="rows">
        {#each rows as row (row.rowId)}
          <li>
            <div class="path">
              <strong>{row.srcRoot}</strong>
              {#if row.dstRoot}
                <span class="arrow">→</span>
                <span class="dst">{row.dstRoot}</span>
              {/if}
            </div>
            <div class="meta">
              <span class="kind">{row.kind}</span>
              <span class="progress">
                {fmtBytes(row.bytesDone)} / {fmtBytes(row.bytesTotal)} ({pct(row)}%)
              </span>
              <span class="age">{fmtAge(row.lastCheckpointAtMs)}</span>
            </div>
            <div class="actions">
              <button type="button" onclick={() => void resume(row)}>
                {t("resume-prompt-resume")}
              </button>
              <button type="button" class="danger" onclick={() => void discard(row)}>
                {t("resume-discard-one")}
              </button>
            </div>
          </li>
        {/each}
      </ul>

      <footer>
        <button type="button" onclick={onClose}>{t("action-close")}</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .modal {
    background: var(--surface, #1a1a1a);
    color: var(--text, #fff);
    border-radius: 8px;
    padding: 1.5rem;
    max-width: 720px;
    width: 90%;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }
  header h2 {
    margin: 0 0 0.5rem 0;
  }
  .hint {
    margin: 0 0 0.75rem 0;
    color: var(--text-muted, #aaa);
    font-size: 0.9rem;
  }
  .bulk {
    display: flex;
    gap: 0.5rem;
  }
  .rows {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }
  .rows li {
    padding: 0.75rem 0;
    border-top: 1px solid var(--border, #333);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .rows li:first-child {
    border-top: none;
  }
  .path {
    font-family: var(--font-mono, monospace);
    font-size: 0.9rem;
    word-break: break-all;
  }
  .arrow {
    margin: 0 0.5em;
    color: var(--text-muted, #aaa);
  }
  .dst {
    color: var(--text-muted, #aaa);
  }
  .meta {
    display: flex;
    gap: 1em;
    font-size: 0.85rem;
    color: var(--text-muted, #aaa);
  }
  .kind {
    text-transform: uppercase;
    font-weight: 600;
  }
  .actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }
  button {
    padding: 0.4rem 0.8rem;
    border-radius: 4px;
    border: 1px solid var(--border, #444);
    background: var(--button-bg, #2a2a2a);
    color: var(--text, #fff);
    cursor: pointer;
    font-size: 0.85rem;
  }
  button:hover {
    background: var(--button-bg-hover, #353535);
  }
  button.danger {
    border-color: var(--danger, #c33);
    color: var(--danger, #c33);
  }
  button.danger:hover {
    background: var(--danger, #c33);
    color: var(--text-on-danger, #fff);
  }
  footer {
    display: flex;
    justify-content: flex-end;
  }
</style>
