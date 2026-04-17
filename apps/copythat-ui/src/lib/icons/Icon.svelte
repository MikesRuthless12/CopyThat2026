<!--
  Minimal, dependency-free Lucide-style SVG icon set.

  Each icon is a 24x24 viewBox with `currentColor` stroke so callers
  style via CSS `color`. We hand-inline the paths from the Lucide
  ISC-licensed source rather than shipping the full package; that
  keeps the bundle small and avoids a new dev dep.

  Adding an icon: drop a new `case` below with the literal Lucide
  SVG body. Keep the alphabetical order.
-->
<script lang="ts">
  type IconName =
    | "alert-triangle"
    | "archive"
    | "chevron-down"
    | "chevron-up"
    | "copy"
    | "download"
    | "external-link"
    | "file"
    | "file-audio"
    | "file-code"
    | "file-image"
    | "file-text"
    | "file-video"
    | "file-x"
    | "folder"
    | "folder-symlink"
    | "info"
    | "loader"
    | "pause"
    | "play"
    | "refresh"
    | "rocket"
    | "square"
    | "trash"
    | "upload"
    | "x";

  interface Props {
    name: IconName;
    size?: number;
    strokeWidth?: number;
    label?: string;
  }

  let { name, size = 16, strokeWidth = 2, label }: Props = $props();
</script>

<svg
  class="icon"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width={strokeWidth}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-label={label}
  role={label ? "img" : "presentation"}
  aria-hidden={label ? undefined : true}
>
  {#if name === "alert-triangle"}
    <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
    <line x1="12" x2="12" y1="9" y2="13" />
    <line x1="12" x2="12.01" y1="17" y2="17" />
  {:else if name === "archive"}
    <rect width="20" height="5" x="2" y="3" rx="1" />
    <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" />
    <path d="M10 12h4" />
  {:else if name === "chevron-down"}
    <polyline points="6 9 12 15 18 9" />
  {:else if name === "chevron-up"}
    <polyline points="18 15 12 9 6 15" />
  {:else if name === "copy"}
    <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
    <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
  {:else if name === "download"}
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
    <polyline points="7 10 12 15 17 10" />
    <line x1="12" x2="12" y1="15" y2="3" />
  {:else if name === "external-link"}
    <path d="M15 3h6v6" />
    <path d="M10 14 21 3" />
    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
  {:else if name === "file"}
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="M14 2v5h6" />
  {:else if name === "file-audio"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="M8 13v5" />
    <path d="M12 11v9" />
    <path d="M16 13v5" />
  {:else if name === "file-code"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="m10 13-2 2 2 2" />
    <path d="m14 17 2-2-2-2" />
  {:else if name === "file-image"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <circle cx="10" cy="13" r="2" />
    <path d="m20 17-1.09-1.09a2 2 0 0 0-2.82 0L10 22" />
  {:else if name === "file-text"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="M10 9H8" />
    <path d="M16 13H8" />
    <path d="M16 17H8" />
  {:else if name === "file-video"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="m10 11 5 3-5 3v-6Z" />
  {:else if name === "file-x"}
    <path d="M14 2v5h6" />
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="m9 13 4 4" />
    <path d="m13 13-4 4" />
  {:else if name === "folder"}
    <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  {:else if name === "folder-symlink"}
    <path d="M2 9a2 2 0 0 1 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2Z" />
    <path d="m8 16 3-3-3-3" />
  {:else if name === "info"}
    <circle cx="12" cy="12" r="10" />
    <path d="M12 16v-4" />
    <path d="M12 8h.01" />
  {:else if name === "loader"}
    <path d="M21 12a9 9 0 1 1-6.219-8.56" />
  {:else if name === "pause"}
    <rect x="6" y="4" width="4" height="16" rx="1" />
    <rect x="14" y="4" width="4" height="16" rx="1" />
  {:else if name === "play"}
    <polygon points="6 3 20 12 6 21 6 3" />
  {:else if name === "refresh"}
    <path d="M3 12a9 9 0 0 1 15-6.7L21 8" />
    <path d="M21 3v5h-5" />
    <path d="M21 12a9 9 0 0 1-15 6.7L3 16" />
    <path d="M3 21v-5h5" />
  {:else if name === "rocket"}
    <path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z" />
    <path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z" />
    <path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" />
    <path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" />
  {:else if name === "square"}
    <rect width="18" height="18" x="3" y="3" rx="2" />
  {:else if name === "trash"}
    <path d="M3 6h18" />
    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
    <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
  {:else if name === "upload"}
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
    <polyline points="17 8 12 3 7 8" />
    <line x1="12" x2="12" y1="3" y2="15" />
  {:else if name === "x"}
    <path d="M18 6 6 18" />
    <path d="m6 6 12 12" />
  {/if}
</svg>

<style>
  .icon {
    display: inline-block;
    flex-shrink: 0;
    vertical-align: middle;
  }
</style>
