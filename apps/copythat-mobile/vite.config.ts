import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
  plugins: [
    svelte(),
    VitePWA({
      registerType: "autoUpdate",
      includeAssets: [
        "icons/icon-32.png",
        "icons/icon-64.png",
        "icons/icon-128.png",
        "icons/icon-512.png",
      ],
      manifest: {
        name: "Copy That",
        short_name: "Copy That",
        description:
          "Mobile companion for Copy That — connect over PeerJS WebRTC to drive copy/move/sync/secure-delete jobs running on your desktop.",
        theme_color: "#3b82f6",
        background_color: "#0f172a",
        display: "standalone",
        scope: "/",
        start_url: "/",
        icons: [
          {
            src: "icons/icon-32.png",
            sizes: "32x32",
            type: "image/png",
          },
          {
            src: "icons/icon-64.png",
            sizes: "64x64",
            type: "image/png",
          },
          {
            src: "icons/icon-128.png",
            sizes: "128x128",
            type: "image/png",
          },
          {
            src: "icons/icon-512.png",
            sizes: "512x512",
            type: "image/png",
            purpose: "any maskable",
          },
        ],
      },
    }),
  ],
});
