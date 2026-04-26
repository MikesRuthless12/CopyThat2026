/**
 * Extended Playwright test fixture.
 *
 * Boots every spec with the Tauri IPC shim already installed and
 * exposes a `tauri` handle that lets tests register invoke
 * responders, dispatch events, and inspect the call log without
 * dropping into raw `page.evaluate()` boilerplate.
 *
 * Usage:
 *
 * ```ts
 * import { expect, test } from "./fixtures/test";
 *
 * test("Drop Stack lights up after a drop", async ({ page, tauri }) => {
 *   await tauri.handle("start_copy", () => [42]);
 *   await tauri.handle("globals", () => ({ ... }));
 *   await page.goto("/");
 *   await tauri.emit("drop-received", { paths: ["/tmp/foo.bin"] });
 *   await expect(page.getByRole("dialog")).toBeVisible();
 * });
 * ```
 *
 * The `tauri` fixture is auto-reset between tests, so handlers and
 * the call log don't leak. See `apps/copythat-ui/e2e/README.md` for
 * the design notes and the deferred true-end-to-end tauri-driver
 * path.
 */

import { test as base, expect, type Page } from "@playwright/test";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import type {
  CopyThatE2EHandle,
  InvokeHandler,
  InvokeRecord,
} from "./tauri-shim";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const SHIM_PATH = resolve(__dirname, "tauri-shim.ts");

declare global {
  interface Window {
    __copythat_e2e__?: CopyThatE2EHandle;
  }
}

/**
 * Public surface of the `tauri` test fixture. All methods round-trip
 * through `page.evaluate()` so the call sites stay readable in the
 * specs.
 */
export interface TauriFixture {
  /**
   * Register a handler for a single Tauri command. Replaces any
   * previously installed handler for the same name. The handler can
   * return a value or a promise; Tauri's `invoke()` resolves to it.
   */
  handle: (
    cmd: string,
    handler: (args: Record<string, unknown> | undefined) => unknown,
  ) => Promise<void>;

  /**
   * Bulk-register a stable set of handlers in one round-trip. Useful
   * for the "default" set of IPC responders most tests need
   * (globals, list_jobs, get_settings, etc.).
   */
  handles: (
    map: Record<
      string,
      (args: Record<string, unknown> | undefined) => unknown
    >,
  ) => Promise<void>;

  /**
   * Dispatch a Tauri event to whatever listeners the UI has
   * registered. Returns the number of listeners that received it
   * (0 means nobody was listening, which is usually a test bug).
   */
  emit: (event: string, payload: unknown) => Promise<number>;

  /** Read-only view of every invoke() the page has made so far. */
  calls: () => Promise<InvokeRecord[]>;

  /** Calls filtered to a single command name, in order. */
  callsFor: (cmd: string) => Promise<InvokeRecord[]>;

  /** Wait until at least one invoke() of the named command has fired. */
  waitForCall: (cmd: string, timeoutMs?: number) => Promise<InvokeRecord>;

  /** Set the default handler used when no specific handler matches. */
  setDefault: (
    handler: (args: Record<string, unknown> | undefined) => unknown,
  ) => Promise<void>;
}

interface Fixtures {
  tauri: TauriFixture;
}

export const test = base.extend<Fixtures>({
  tauri: async ({ page }, use) => {
    // Install the shim before any of the page's own scripts run.
    // `addInitScript({ path })` evaluates the file as a classic
    // script in the page's main world, which is what we need for
    // the shim's `installTauriShim()` IIFE to land before the
    // Vite bundle imports `@tauri-apps/api/core`.
    await page.addInitScript({ path: SHIM_PATH });

    // Per-test default handlers cover the boot path so a spec
    // that only asserts on, say, the drop-stack flow doesn't have
    // to pre-register `globals`, `list_jobs`, etc. Specs that need
    // a different shape override these by name.
    await page.addInitScript(() => {
      const reg = window.__copythat_e2e__;
      if (!reg) return;
      const noop = () => undefined;
      reg.setHandler("globals", () => ({
        state: "idle",
        activeJobs: 0,
        queuedJobs: 0,
        pausedJobs: 0,
        failedJobs: 0,
        succeededJobs: 0,
        bytesDone: 0,
        bytesTotal: 0,
        rateBps: 0,
        etaSeconds: null,
        errors: 0,
      }));
      reg.setHandler("list_jobs", () => []);
      reg.setHandler("error_log", () => []);
      reg.setHandler("history_search", () => []);
      reg.setHandler("history_totals", () => ({
        bytes: 0,
        files: 0,
        jobs: 0,
        avgRateBps: 0,
        peakRateBps: 0,
      }));
      reg.setHandler("history_daily", () => []);
      reg.setHandler("available_locales", () => [
        "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de",
        "fr", "ko", "it", "tr", "vi", "pl", "nl", "id", "uk",
      ]);
      reg.setHandler("system_locale", () => "en");
      reg.setHandler("translations", () => ({}));
      reg.setHandler("get_settings", () => ({
        general: {
          locale: "en",
          theme: "system",
          autoResumeInterrupted: false,
          mobileOnboardingDismissed: true,
          errorPromptStyle: "modal",
        },
        transfer: {
          verifyAlgo: "blake3",
          dedupMode: "auto-ladder",
          hardlinkPolicy: "off",
          encryption: { recipients: [], recipientsFile: null },
          compression: { mode: "off", level: 3 },
        },
        mobile: { pairings: [], desktopPeerId: "test-peer" },
        network: { rateBps: null, scheduleEnabled: false },
        power: { policy: "always" },
      }));
      reg.setHandler("pending_resumes", () => []);
      reg.setHandler("plugin:dialog|open", () => null);
      // Default fallback so a stray invoke doesn't reject — tests
      // that need stricter assertions can override.
      reg.setDefaultHandler(noop);
    });

    const fixture: TauriFixture = {
      async handle(cmd, handler) {
        const fnSrc = handler.toString();
        await page.evaluate(
          ({ cmd, fnSrc }) => {
            // Reconstitute the function in the page context.
            const fn = new Function("args", `return (${fnSrc})(args);`) as InvokeHandler;
            window.__copythat_e2e__?.setHandler(cmd, fn);
          },
          { cmd, fnSrc },
        );
      },
      async handles(map) {
        const entries = Object.entries(map).map(([cmd, fn]) => ({
          cmd,
          fnSrc: fn.toString(),
        }));
        await page.evaluate((entries) => {
          for (const { cmd, fnSrc } of entries) {
            const fn = new Function("args", `return (${fnSrc})(args);`) as InvokeHandler;
            window.__copythat_e2e__?.setHandler(cmd, fn);
          }
        }, entries);
      },
      async emit(event, payload) {
        return await page.evaluate(
          ({ event, payload }) => {
            const reg = window.__copythat_e2e__;
            if (!reg) return 0;
            const before = reg.listenerCount(event);
            reg.emit(event, payload);
            return before;
          },
          { event, payload },
        );
      },
      async calls() {
        return await page.evaluate(() => window.__copythat_e2e__?.calls() ?? []);
      },
      async callsFor(cmd) {
        return await page.evaluate(
          (cmd) =>
            (window.__copythat_e2e__?.calls() ?? []).filter(
              (r: InvokeRecord) => r.cmd === cmd,
            ),
          cmd,
        );
      },
      async waitForCall(cmd, timeoutMs = 5000) {
        const handle = await page.waitForFunction(
          (cmd) => {
            const calls = window.__copythat_e2e__?.calls() ?? [];
            return calls.find((r: InvokeRecord) => r.cmd === cmd) ?? null;
          },
          cmd,
          { timeout: timeoutMs },
        );
        return (await handle.jsonValue()) as InvokeRecord;
      },
      async setDefault(handler) {
        const fnSrc = handler.toString();
        await page.evaluate((fnSrc) => {
          const fn = new Function("args", `return (${fnSrc})(args);`) as InvokeHandler;
          window.__copythat_e2e__?.setDefaultHandler(fn);
        }, fnSrc);
      },
    };

    await use(fixture);

    // Reset between tests so handler state doesn't bleed across
    // specs in the same worker.
    await resetSafely(page);
  },
});

async function resetSafely(page: Page): Promise<void> {
  if (page.isClosed()) return;
  try {
    await page.evaluate(() => window.__copythat_e2e__?.reset());
  } catch {
    // Page may have navigated to about:blank during teardown; the
    // reset is best-effort.
  }
}

export { expect };
