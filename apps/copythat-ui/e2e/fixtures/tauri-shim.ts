/**
 * Tauri 2.x IPC shim, browser-side.
 *
 * Loaded via Playwright's `page.addInitScript()` before the Vite dev
 * bundle boots. Replaces `window.__TAURI_INTERNALS__` — the global
 * the `@tauri-apps/api` package reads — with a stand-in whose
 * `invoke()` and `transformCallback()` route through a registry the
 * test fixture controls from Node.
 *
 * Why a browser shim instead of `tauri-driver` against the real
 * binary?
 *
 * - The canonical Tauri 2.x WebDriver harness pairs with
 *   WebdriverIO, not Playwright (Playwright doesn't speak the
 *   classic WebDriver protocol). Wiring Playwright into a real
 *   tauri-driver bridge is a research project; the docs/E2E.md
 *   "deferred work" section spells out the path.
 * - Frontend coverage of the §4 golden path checkboxes — clicks,
 *   modals, drop-stack drags, IPC payload shapes — is what's
 *   tractable today, and the IPC mock makes it deterministic.
 *   The Rust side is exercised by the per-crate smoke tests
 *   (`cargo test -p <crate>`); this harness covers the half that
 *   sits above the IPC boundary.
 *
 * The shim is a self-contained IIFE that installs the globals; the
 * Node-side fixture (e2e/fixtures/test.ts) injects helpers to
 * register handlers, dispatch events, and inspect the call log.
 */

declare global {
  interface Window {
    /**
     * Test-only escape hatch the fixture uses to manage the IPC
     * mock from Node. Production code never touches this — the
     * real Tauri runtime never installs it.
     */
    __copythat_e2e__?: CopyThatE2EHandle;

    __TAURI_INTERNALS__?: TauriInternals;
  }
}

interface InvokeOptions {
  headers?: Record<string, string>;
}

interface TauriInternals {
  invoke: (
    cmd: string,
    args?: Record<string, unknown>,
    options?: InvokeOptions,
  ) => Promise<unknown>;
  transformCallback: (
    callback: (response: unknown) => void,
    once?: boolean,
  ) => number;
  metadata: {
    currentWindow: { label: string };
    currentWebview: { label: string };
  };
  // `convertFileSrc` round-trips a host file path into an asset URL.
  // Most frontend code uses it for thumbnails; the shim returns the
  // raw path so tests can assert on what the UI tried to load.
  convertFileSrc: (filePath: string, protocol?: string) => string;
  /**
   * Tauri 2.x exposes the runtime auth token used by IPC; we keep a
   * stable test value so any header-mirror code paths see something
   * non-empty.
   */
  runtimeAuthToken?: string;
}

export interface InvokeRecord {
  cmd: string;
  args: Record<string, unknown> | undefined;
  at: number;
}

export type InvokeHandler = (
  args: Record<string, unknown> | undefined,
) => unknown | Promise<unknown>;

export interface CopyThatE2EHandle {
  /** Register or replace the handler for a single Tauri command. */
  setHandler: (cmd: string, handler: InvokeHandler) => void;
  /** Remove a registered handler so a later call falls back to the default. */
  clearHandler: (cmd: string) => void;
  /** Wipe every handler — called between tests. */
  reset: () => void;
  /** Dispatch a Tauri event to whatever listeners the UI registered. */
  emit: (event: string, payload: unknown) => void;
  /** Read-only view of every invoke() that's fired since the last reset. */
  calls: () => InvokeRecord[];
  /** Number of registered listeners for the named event. Useful as a sanity probe. */
  listenerCount: (event: string) => number;
  /** Default handler fired when no specific one is registered. */
  setDefaultHandler: (handler: InvokeHandler) => void;
}

/**
 * Install the shim on `window`. Called from Playwright via
 * `page.addInitScript()` so the override beats every module that
 * imports `@tauri-apps/api`.
 */
export function installTauriShim(): void {
  if (window.__copythat_e2e__) {
    // Hot reload during `pnpm dev` re-runs init scripts; keep the
    // existing handle so handlers registered for the in-progress
    // test don't vanish.
    return;
  }

  const handlers = new Map<string, InvokeHandler>();
  const callbacks = new Map<number, (response: unknown) => void>();
  const calls: InvokeRecord[] = [];
  const listenersByEvent = new Map<string, Set<number>>();
  let nextCallbackId = 1;
  let defaultHandler: InvokeHandler | null = (_args) => undefined;

  const transformCallback = (
    callback: (response: unknown) => void,
    once?: boolean,
  ): number => {
    const id = nextCallbackId++;
    if (once) {
      callbacks.set(id, (response: unknown) => {
        callbacks.delete(id);
        callback(response);
      });
    } else {
      callbacks.set(id, callback);
    }
    return id;
  };

  const invoke = async (
    cmd: string,
    args?: Record<string, unknown>,
    _options?: InvokeOptions,
  ): Promise<unknown> => {
    calls.push({ cmd, args, at: Date.now() });

    // Tauri's `listen()` translates to invoke('plugin:event|listen').
    // We snoop the registration so `emit()` can find the right
    // callback id when the test dispatches an event.
    if (cmd === "plugin:event|listen" && args) {
      const event = args.event as string | undefined;
      const handlerId = args.handler as number | undefined;
      if (typeof event === "string" && typeof handlerId === "number") {
        let set = listenersByEvent.get(event);
        if (!set) {
          set = new Set();
          listenersByEvent.set(event, set);
        }
        set.add(handlerId);
        // Tauri returns the handler id so callers can pass it to
        // `unlisten`. We mirror that contract.
        return handlerId;
      }
    }
    if (cmd === "plugin:event|unlisten" && args) {
      const event = args.event as string | undefined;
      const handlerId = args.eventId as number | undefined;
      if (typeof event === "string" && typeof handlerId === "number") {
        listenersByEvent.get(event)?.delete(handlerId);
        callbacks.delete(handlerId);
      }
      return undefined;
    }

    // `plugin:dialog|open` fires the file picker. Tests should stub
    // it to return a synthetic path list; the default returns null
    // (= user cancelled).
    const handler = handlers.get(cmd) ?? defaultHandler;
    if (!handler) {
      throw new Error(
        `[copythat e2e] no handler for invoke('${cmd}'). Call __copythat_e2e__.setHandler() first.`,
      );
    }
    return await handler(args);
  };

  window.__TAURI_INTERNALS__ = {
    invoke,
    transformCallback,
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { label: "main" },
    },
    convertFileSrc: (filePath: string, _protocol?: string) => filePath,
    runtimeAuthToken: "copythat-e2e-shim",
  };

  window.__copythat_e2e__ = {
    setHandler(cmd, handler) {
      handlers.set(cmd, handler);
    },
    clearHandler(cmd) {
      handlers.delete(cmd);
    },
    reset() {
      handlers.clear();
      callbacks.clear();
      listenersByEvent.clear();
      calls.length = 0;
      nextCallbackId = 1;
      defaultHandler = (_args) => undefined;
    },
    emit(event, payload) {
      const set = listenersByEvent.get(event);
      if (!set) return;
      // Tauri delivers `{ event, id, payload }` — shape required by
      // `@tauri-apps/api/event`'s internal handler. Mirror it exactly.
      const message = {
        event,
        id: 0,
        payload,
      };
      for (const id of set) {
        const cb = callbacks.get(id);
        cb?.(message);
      }
    },
    calls() {
      return calls.slice();
    },
    listenerCount(event) {
      return listenersByEvent.get(event)?.size ?? 0;
    },
    setDefaultHandler(handler) {
      defaultHandler = handler;
    },
  };
}

// Auto-install when this module is loaded directly in the browser
// via `addInitScript({ path })`. Playwright's init-script mode
// evaluates the file as a script (no exports), so the IIFE has to
// run at module top level.
if (typeof window !== "undefined") {
  installTauriShim();
}
