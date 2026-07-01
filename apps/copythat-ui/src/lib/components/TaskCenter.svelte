<script lang="ts">
  // Phase 49j — Tasks & progress center. Lists running tasks (with a live
  // progress bar + Cancel) and recently-finished ones. State is fed by the
  // `tasks` store, which the app keeps live via `initTaskListeners`.
  import Icon from "../icons/Icon.svelte";
  import { i18nVersion, t } from "../i18n";
  import { taskCancel } from "../ipc";
  import { closeTaskCenter, pushToast, taskCenterOpen, tasks } from "../stores";

  const running = $derived($tasks.filter((x) => x.state === "running"));
  const recent = $derived($tasks.filter((x) => x.state !== "running"));

  async function cancel(id: number) {
    try {
      await taskCancel(id);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function pct(p: number): string {
    return `${Math.round(Math.max(0, Math.min(1, p)) * 100)}%`;
  }
</script>

{#if $taskCenterOpen}
  <aside class="drawer" aria-label={t("tasks-title")}>
    {#key $i18nVersion}
      <header>
        <h2>{t("tasks-title")}</h2>
        <button
          class="close"
          type="button"
          aria-label={t("action-close")}
          onclick={closeTaskCenter}
        >
          <Icon name="x" size={16} />
        </button>
      </header>

      <div class="body">
        {#if $tasks.length === 0}
          <p class="empty">{t("tasks-empty")}</p>
        {:else}
          {#if running.length > 0}
            <h3>{t("tasks-running")}</h3>
            <ul>
              {#each running as task (task.id)}
                <li class="task">
                  <div class="row">
                    <span class="label">{task.label}</span>
                    <button type="button" class="cancel" onclick={() => cancel(task.id)}>
                      {t("tasks-cancel")}
                    </button>
                  </div>
                  <div class="bar">
                    <div class="fill" style="width:{pct(task.progress)}"></div>
                  </div>
                  {#if task.detail}<p class="detail">{task.detail}</p>{/if}
                </li>
              {/each}
            </ul>
          {/if}

          {#if recent.length > 0}
            <h3>{t("tasks-recent")}</h3>
            <ul>
              {#each recent as task (task.id)}
                <li class="task">
                  <div class="row">
                    <span class="label">{task.label}</span>
                    <span class="state {task.state}">{t(`task-state-${task.state}`)}</span>
                  </div>
                  {#if task.error}
                    <p class="detail err">{task.error}</p>
                  {:else if task.detail}
                    <p class="detail">{task.detail}</p>
                  {/if}
                </li>
              {/each}
            </ul>
          {/if}
        {/if}
      </div>
    {/key}
  </aside>
{/if}

<style>
  .drawer {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: min(420px, 92vw);
    background: var(--surface, #1e1e1e);
    color: var(--text, #eee);
    box-shadow: -4px 0 16px rgba(0, 0, 0, 0.4);
    z-index: 41;
    display: flex;
    flex-direction: column;
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border, #333);
  }
  header h2 {
    margin: 0;
    font-size: 1rem;
  }
  .close {
    background: none;
    border: 0;
    color: inherit;
    cursor: pointer;
    line-height: 1;
    padding: 4px;
  }
  .body {
    overflow-y: auto;
    padding: 8px 16px 16px;
  }
  .body h3 {
    font-size: 0.8rem;
    text-transform: uppercase;
    opacity: 0.6;
    margin: 14px 0 6px;
  }
  .empty {
    opacity: 0.6;
    font-size: 0.9rem;
    padding: 24px 0;
    text-align: center;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .task {
    padding: 8px 0;
    border-bottom: 1px solid var(--border, #2a2a2a);
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .label {
    font-size: 0.9rem;
  }
  .bar {
    height: 4px;
    background: var(--border, #333);
    border-radius: 2px;
    margin: 6px 0 2px;
    overflow: hidden;
  }
  .fill {
    height: 100%;
    background: var(--accent, #4c8bf5);
    transition: width 0.2s ease;
  }
  .detail {
    margin: 4px 0 0;
    font-size: 0.78rem;
    opacity: 0.7;
  }
  .detail.err {
    color: var(--danger, #e06c6c);
    opacity: 1;
  }
  .cancel {
    background: none;
    border: 1px solid var(--border, #444);
    color: inherit;
    border-radius: 4px;
    font-size: 0.75rem;
    padding: 2px 8px;
    cursor: pointer;
  }
  .state {
    font-size: 0.75rem;
    padding: 1px 6px;
    border-radius: 4px;
    opacity: 0.85;
  }
  .state.completed {
    color: var(--success, #5fce72);
  }
  .state.failed {
    color: var(--danger, #e06c6c);
  }
  .state.cancelled {
    opacity: 0.55;
  }
</style>
