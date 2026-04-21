<!--
  Phase 14e — subset picker.

  Opens when the preflight check determined the full source set
  won't fit on the destination (with the user's free-space
  reserve applied). Lists each top-level source with its recursive
  size, a checkbox, and a live "fits / doesn't fit" indicator at
  the bottom. The caller's promise resolves with the selected
  subset on confirm, or `null` on cancel.

  Per-path sizes come from `path_sizes_individual`; any path that
  was too expensive to count renders as "too large to count" and
  is treated as "would fill the disk" for budget math (so the user
  can't accidentally pick a giant tree that breaks the preflight
  promise).
-->
<script lang="ts">
  import { i18nVersion, t } from "../i18n";
  import { formatBytes } from "../format";
  import { pathSizesIndividual } from "../ipc";

  interface Props {
    sources: string[];
    freeBytes: number;
    reserveBytes: number;
    onResolve: (picked: string[] | null) => void;
  }

  let { sources, freeBytes, reserveBytes, onResolve }: Props = $props();

  // `sources` is a prop — reading its length during state init only
  // captures the initial value, which is fine here because the array
  // identity is fixed for the lifetime of this modal (the parent
  // renders a fresh modal if the source list changes). Suppress the
  // state_referenced_locally warning with a targeted comment.
  // svelte-ignore state_referenced_locally
  let sizes = $state<number[]>(new Array(sources.length).fill(0));
  // svelte-ignore state_referenced_locally
  let checked = $state<boolean[]>(new Array(sources.length).fill(false));
  let loading = $state(true);

  const budget = $derived(Math.max(0, freeBytes - reserveBytes));
  const pickedTotal = $derived(
    sizes.reduce(
      (acc, n, i) => (checked[i] ? acc + (n === Number.MAX_SAFE_INTEGER ? Infinity : n) : acc),
      0,
    ),
  );
  const fits = $derived(pickedTotal <= budget);
  const anyChecked = $derived(checked.some(Boolean));

  $effect(() => {
    void probe();
  });

  async function probe() {
    try {
      const raw = await pathSizesIndividual(sources);
      // Rust returns u64::MAX for "too large to count" — map it to
      // JS's safest huge value so the math stays well-defined.
      sizes = raw.map((n) =>
        n >= Number.MAX_SAFE_INTEGER ? Number.MAX_SAFE_INTEGER : n,
      );
      // Auto-tick entries from the top until we'd exceed the budget.
      // Users see a sensible starting selection and can adjust from
      // there rather than staring at an all-blank list.
      let running = 0;
      const next = sources.map(() => false);
      for (let i = 0; i < sizes.length; i++) {
        if (sizes[i] === Number.MAX_SAFE_INTEGER) continue;
        if (running + sizes[i] <= budget) {
          next[i] = true;
          running += sizes[i];
        }
      }
      checked = next;
    } finally {
      loading = false;
    }
  }

  function toggle(i: number) {
    const next = checked.slice();
    next[i] = !next[i];
    checked = next;
  }

  function basename(p: string): string {
    const idx = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    return idx >= 0 ? p.slice(idx + 1) || p : p;
  }

  function confirm() {
    if (!fits || !anyChecked) return;
    const picked = sources.filter((_, i) => checked[i]);
    onResolve(picked);
  }
</script>

<div class="backdrop" role="presentation">
  {#key $i18nVersion}
    <div
      class="modal"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="subset-title"
    >
      <header>
        <h2 id="subset-title">{t("subset-title")}</h2>
      </header>
      <p class="sub">{t("subset-subtitle")}</p>

      {#if loading}
        <p class="loading">{t("subset-loading")}</p>
      {:else}
        <ul class="rows">
          {#each sources as src, i (src)}
            <li class="row" class:picked={checked[i]}>
              <label>
                <input
                  type="checkbox"
                  checked={checked[i]}
                  onchange={() => toggle(i)}
                />
                <span class="name" title={src}>{basename(src)}</span>
                <span class="size tabular">
                  {sizes[i] === Number.MAX_SAFE_INTEGER
                    ? t("subset-too-large")
                    : formatBytes(sizes[i])}
                </span>
              </label>
            </li>
          {/each}
        </ul>

        <dl class="tally">
          <dt>{t("preflight-required")}</dt>
          <dd class="tabular">
            {pickedTotal === Infinity
              ? t("subset-too-large")
              : formatBytes(pickedTotal)}
          </dd>
          <dt>{t("subset-budget")}</dt>
          <dd class="tabular">{formatBytes(budget)}</dd>
          <dt class:bad={!fits}>{t("subset-remaining")}</dt>
          <dd class:bad={!fits} class="tabular">
            {pickedTotal === Infinity || !fits
              ? `−${formatBytes(Math.max(0, pickedTotal - budget))}`
              : formatBytes(budget - pickedTotal)}
          </dd>
        </dl>
      {/if}

      <div class="actions">
        <button class="secondary" type="button" onclick={() => onResolve(null)}>
          {t("action-cancel")}
        </button>
        <button
          class="primary"
          type="button"
          onclick={confirm}
          disabled={loading || !fits || !anyChecked}
        >
          {t("subset-confirm")}
        </button>
      </div>
    </div>
  {/key}
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 99;
  }

  .modal {
    width: min(520px, 94vw);
    max-height: 82vh;
    padding: 14px 16px 12px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.24);
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow: hidden;
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .sub {
    margin: 0;
    font-size: 12px;
    color: var(--fg-dim, #6a6a6a);
  }

  .loading {
    padding: 12px 0;
    text-align: center;
    color: var(--fg-dim, #6a6a6a);
    font-size: 12px;
  }

  .rows {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 240px;
    overflow: auto;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 6px;
  }

  .row {
    padding: 0;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.1));
  }

  .row:last-child {
    border-bottom: none;
  }

  .row.picked {
    background: var(--row-selected, rgba(79, 140, 255, 0.08));
  }

  .row label {
    display: grid;
    grid-template-columns: 20px 1fr auto;
    align-items: center;
    gap: 8px;
    padding: 7px 10px;
    cursor: pointer;
    font-size: 12px;
  }

  .name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .size {
    color: var(--fg-dim, #6a6a6a);
  }

  .tally {
    display: grid;
    grid-template-columns: max-content 1fr;
    column-gap: 12px;
    row-gap: 3px;
    margin: 0;
    font-size: 12px;
    padding: 6px 8px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.03));
    border-radius: 6px;
  }

  dt {
    color: var(--fg-dim, #6a6a6a);
  }

  dd {
    margin: 0;
  }

  .bad {
    color: var(--error, #c24141);
    font-weight: 600;
  }

  .tabular {
    font-variant-numeric: tabular-nums;
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

  button.primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.secondary {
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
  }
</style>
