import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright config for the §4 golden-path harness.
 *
 * Targets the Vite dev server with a `window.__TAURI_INTERNALS__`
 * shim — see `e2e/README.md` for the trade-offs vs. a real
 * tauri-driver bridge. The Tauri binary itself isn't booted; the
 * frontend half of every §4 checkbox is what these tests cover.
 *
 * Run modes:
 * - `pnpm exec playwright test` — full suite, headless.
 * - `pnpm exec playwright test --ui` — interactive runner.
 * - `pnpm exec playwright test e2e/qa-section-4-1-copy.spec.ts` —
 *   single subsection.
 */
export default defineConfig({
  testDir: "./e2e",
  testMatch: "**/*.spec.ts",
  // Parallelism is fine — every test resets the IPC mock + opens
  // its own page, so cross-test bleed is bounded.
  fullyParallel: true,
  // Fail fast in CI so a green-on-flake doesn't slip through.
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 2 : undefined,
  reporter: process.env.CI
    ? [["github"], ["html", { outputFolder: "../../target/playwright-report" }]]
    : [["list"], ["html", { outputFolder: "../../target/playwright-report", open: "never" }]],
  use: {
    baseURL: "http://localhost:1420",
    trace: "retain-on-failure",
    video: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  webServer: {
    // `pnpm dev` (vite) is what the human normally runs; the test
    // harness boots a fresh copy on the same port. Reuse an
    // existing server in dev so a developer iterating on a spec
    // doesn't cycle Vite each run.
    command: "pnpm dev",
    url: "http://localhost:1420",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
    stdout: "ignore",
    stderr: "pipe",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    // WebKit + Firefox land here once the §4 specs are filled in
    // and we want cross-engine coverage. Today the Tauri 2.x
    // production runtime is WebView2 / WKWebView / WebKitGTK, so
    // Chromium covers the largest single share.
  ],
});
