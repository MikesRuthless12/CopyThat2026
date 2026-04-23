import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Tauri prefers a fixed port and clean output.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },

  // Expose only `VITE_` and `TAURI_ENV_*` to the frontend.
  envPrefix: ["VITE_", "TAURI_ENV_*"],

  build: {
    target:
      process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    // Phase 28 — the Drop Stack lives in its own Tauri window.
    // Multi-page entry so Vite emits both `index.html` (main) and
    // `dropstack.html` (stack) into `dist/`.
    rollupOptions: {
      input: {
        main: "index.html",
        dropstack: "dropstack.html",
      },
    },
  },
}));
