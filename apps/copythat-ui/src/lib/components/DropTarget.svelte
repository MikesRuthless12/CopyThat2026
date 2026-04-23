<!--
  Phase 29 — reusable drop-target wrapper.

  Renders a bordered region that:
  - Shows the accent border + 5% tint while a drag hovers over a valid
    target.
  - Shows an error border + tooltip when `invalid` is truthy (read-only
    target, insufficient permission, disk full — the caller decides).
  - Optionally spring-loads after `springLoadMs` (default 650) by
    calling `onSpringLoad`. Omit `onSpringLoad` to disable spring
    loading on this specific target.
  - Forwards the actual drop to `onDrop(paths, event)`. Paths come from
    the HTML5 dataTransfer.files list when the drag is a browser-level
    file drag; for Tauri's OS-native window drag, the caller is
    expected to listen on `onDragDropEvent` and route paths through a
    different code path (this component is for in-window drag sources
    like DropStack rows → DestinationPicker rows).
-->
<script lang="ts">
  import type { Snippet } from "svelte";
  import { springLoad } from "../springLoad";
  import { t } from "../i18n";

  interface Props {
    /** Accent tint + border trigger. Cosmetic only. */
    dimmed?: boolean;
    /** Mark the target invalid — renders error border + tooltip. */
    invalid?: boolean;
    /** Human-readable reason shown in the invalid tooltip. */
    invalidReason?: string;
    /** Spring-load delay (ms). Omit `onSpringLoad` to disable. */
    springLoadMs?: number;
    /** Cascade hook — fires after a sustained hover. */
    onSpringLoad?: () => void;
    /** Fires on drop. `paths` are extracted from dataTransfer when
     *  the drag carries OS files; otherwise empty (in-app drag). */
    onDrop?: (paths: string[], ev: DragEvent) => void;
    /** Content placed inside the target. */
    children?: Snippet;
    /** Extra CSS class forwarded onto the root. */
    className?: string;
  }

  let {
    dimmed = false,
    invalid = false,
    invalidReason = "",
    springLoadMs = 650,
    onSpringLoad,
    onDrop,
    children,
    className = "",
  }: Props = $props();

  let hovering = $state(false);

  function handleDragOver(ev: DragEvent) {
    if (invalid) return;
    ev.preventDefault();
    hovering = true;
  }

  function handleDragLeave(_ev: DragEvent) {
    hovering = false;
  }

  function handleDrop(ev: DragEvent) {
    hovering = false;
    if (invalid) return;
    ev.preventDefault();
    const paths: string[] = [];
    const files = ev.dataTransfer?.files;
    if (files) {
      for (let i = 0; i < files.length; i++) {
        const f = files.item(i);
        // `File` in a browser context has no real path; Tauri's
        // enhanced drag exposes it on a non-standard field. Try both.
        const maybePath =
          // @ts-expect-error — Tauri 2 attaches `.path` to File in webview
          typeof f?.path === "string" ? (f.path as string) : null;
        if (maybePath) paths.push(maybePath);
      }
    }
    onDrop?.(paths, ev);
  }

  // Build the springLoad options lazily — omitting the action entirely
  // when `onSpringLoad` is undefined keeps the DOM tree simpler for
  // targets that don't cascade.
  function makeSpringOpts() {
    return {
      delayMs: springLoadMs,
      onTrigger: () => onSpringLoad?.(),
      disabled: invalid || !onSpringLoad,
    };
  }
</script>

<div
  class={["drop-target", className, {
    "drop-target-hover": hovering && !invalid,
    "drop-target-invalid": invalid,
  }]
    .filter(Boolean)
    .join(" ")}
  role="region"
  title={invalid ? invalidReason || t("dropzone-invalid-title") : undefined}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  use:springLoad={makeSpringOpts()}
>
  {#if invalid}
    <span class="drop-target-badge" role="status">
      {invalidReason || t("dropzone-invalid-title")}
    </span>
  {/if}
  {#if children}{@render children()}{/if}
  {#if dimmed && !hovering && !invalid}
    <!-- reserved for caller-driven visual tweaks -->
  {/if}
</div>

<style>
  .drop-target {
    position: relative;
    border: 2px solid transparent;
    border-radius: 4px;
    transition: border-color 80ms ease, background-color 80ms ease;
  }
  .drop-target-hover {
    border-color: var(--accent, #3b82f6);
    background-color: color-mix(in srgb, var(--accent, #3b82f6) 5%, transparent);
  }
  .drop-target-invalid {
    border-color: var(--danger, #dc2626);
    background-color: color-mix(in srgb, var(--danger, #dc2626) 5%, transparent);
    cursor: not-allowed;
  }
  .drop-target-badge {
    position: absolute;
    top: 4px;
    right: 4px;
    background: var(--danger, #dc2626);
    color: #fff;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 3px;
    pointer-events: none;
  }
</style>
