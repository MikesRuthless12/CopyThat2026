// Lightweight theme tracker. Reads `prefers-color-scheme` from the
// webview and updates a `data-theme` attribute on `<html>` so CSS
// variables can switch in a single declaration block.

import { writable, type Readable } from "svelte/store";

export type ThemeName = "light" | "dark";

const store = writable<ThemeName>(detect());
export const theme: Readable<ThemeName> = { subscribe: store.subscribe };

export function initTheme(): () => void {
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const handler = () => {
    const next: ThemeName = media.matches ? "dark" : "light";
    store.set(next);
    document.documentElement.setAttribute("data-theme", next);
  };
  handler();
  media.addEventListener("change", handler);
  return () => media.removeEventListener("change", handler);
}

function detect(): ThemeName {
  if (typeof window === "undefined") return "light";
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}
