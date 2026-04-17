<!--
  Simple context menu popover. Keyboard-navigable: Up/Down moves
  focus across enabled items, Enter/Space activates, Escape closes.

  The parent passes a list of items with `label`, `icon`, `onClick`,
  `tone`, and `disabled`. Only the render/layout lives here.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import type { ContextMenuItem } from "../types";

  interface Props {
    items: ContextMenuItem[];
    x: number;
    y: number;
    onClose: () => void;
  }

  let { items, x, y, onClose }: Props = $props();

  let menuEl: HTMLElement | null = $state(null);

  $effect(() => {
    if (!menuEl) return;
    const first = menuEl.querySelector<HTMLElement>(
      'button:not([disabled])',
    );
    first?.focus();
  });

  function onKeydown(e: KeyboardEvent) {
    if (!menuEl) return;
    const buttons = Array.from(
      menuEl.querySelectorAll<HTMLButtonElement>("button:not([disabled])"),
    );
    const idx = buttons.indexOf(document.activeElement as HTMLButtonElement);
    if (e.key === "ArrowDown") {
      e.preventDefault();
      buttons[(idx + 1) % buttons.length]?.focus();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      buttons[(idx - 1 + buttons.length) % buttons.length]?.focus();
    } else if (e.key === "Escape") {
      e.preventDefault();
      onClose();
    } else if (e.key === "Tab") {
      // Let focus escape naturally but dismiss so we don't leave a
      // floating menu behind.
      onClose();
    }
  }

  function onBackdropClick(e: MouseEvent) {
    if (menuEl && !menuEl.contains(e.target as Node)) {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} onclick={onBackdropClick} />

<div
  bind:this={menuEl}
  class="menu"
  role="menu"
  style:--x="{x}px"
  style:--y="{y}px"
>
  {#each items as item (item.id)}
    <button
      type="button"
      role="menuitem"
      class="item"
      data-tone={item.tone ?? "default"}
      disabled={item.disabled}
      onclick={() => {
        item.onClick();
        onClose();
      }}
    >
      {#if item.icon}
        <Icon name={item.icon} size={14} />
      {/if}
      <span>{item.label}</span>
    </button>
  {/each}
</div>

<style>
  .menu {
    position: fixed;
    left: var(--x);
    top: var(--y);
    z-index: 50;
    min-width: 180px;
    padding: 4px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 8px;
    box-shadow:
      0 2px 4px rgba(0, 0, 0, 0.08),
      0 8px 24px rgba(0, 0, 0, 0.12);
    font-size: 12px;
  }

  .item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 10px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }

  .item:hover:not(:disabled),
  .item:focus-visible {
    background: var(--hover, rgba(128, 128, 128, 0.14));
    outline: none;
  }

  .item:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .item[data-tone="danger"] {
    color: var(--error, #c24141);
  }
</style>
