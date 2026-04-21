<!--
  Compact state pill used in both the header and each row. Colour
  and label come from the job/global state; `<code>` is kept out of
  the translation — localised labels live in the .ftl files under
  `state-<name>`.
-->
<script lang="ts">
  import { i18nVersion, t } from "../i18n";

  interface Props {
    state: string;
    size?: "sm" | "md";
  }

  let { state, size = "sm" }: Props = $props();

  const labelKey = $derived(`state-${state}`);
</script>

<span class="badge" data-state={state} data-size={size}>
  {#key $i18nVersion}{t(labelKey)}{/key}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 2px 8px;
    border-radius: 999px;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-transform: uppercase;
    background: var(--badge-bg, rgba(128, 128, 128, 0.14));
    color: var(--badge-fg, var(--fg, #1f1f1f));
    line-height: 1.6;
    white-space: nowrap;
  }

  .badge[data-size="md"] {
    font-size: 12px;
    padding: 3px 10px;
  }

  .badge[data-state="running"],
  .badge[data-state="copying"] {
    --badge-bg: rgba(79, 140, 255, 0.16);
    --badge-fg: var(--accent, #4f8cff);
  }
  .badge[data-state="succeeded"] {
    --badge-bg: rgba(63, 175, 106, 0.18);
    --badge-fg: var(--ok, #2f8c55);
  }
  .badge[data-state="paused"] {
    --badge-bg: rgba(228, 160, 64, 0.2);
    --badge-fg: var(--warn, #b4781b);
  }
  .badge[data-state="failed"],
  .badge[data-state="error"] {
    --badge-bg: rgba(217, 87, 87, 0.2);
    --badge-fg: var(--error, #c24141);
  }
  .badge[data-state="cancelled"] {
    --badge-bg: rgba(128, 128, 128, 0.18);
    --badge-fg: var(--muted-strong, #6a6a6a);
  }
  .badge[data-state="verifying"] {
    --badge-bg: rgba(140, 90, 200, 0.2);
    --badge-fg: var(--verify, #7a4fb3);
  }
</style>
