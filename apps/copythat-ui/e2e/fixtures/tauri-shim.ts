/**
 * Tauri 2.x IPC shim, browser-side.
 *
 * The actual install runs as an inline `addInitScript()` callback in
 * `fixtures/test.ts` so the browser doesn't try to parse this file's
 * TypeScript syntax. This file keeps the shared type declarations.
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
 */

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
 * The body of the IPC shim, returned as a plain function so
 * `page.addInitScript()` can serialize it. Self-contained: no
 * TypeScript-only syntax, no imports, no module exports.
 *
 * Browser-side notes:
 * - Hot reload during `pnpm dev` re-runs init scripts. We keep the
 *   existing handle so handlers registered for the in-progress
 *   test don't vanish.
 * - `plugin:event|listen` / `unlisten` are snooped so `emit()` can
 *   route to the right callback id without going through the real
 *   Tauri event bus.
 */
export function shimInstaller(): () => void {
  return function install() {
    if ((window as any).__copythat_e2e__) return;

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
    ): Promise<unknown> => {
      calls.push({ cmd, args, at: Date.now() });
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
      const handler = handlers.get(cmd) ?? defaultHandler;
      if (!handler) {
        throw new Error("[copythat e2e] no handler for invoke('" + cmd + "')");
      }
      return await handler(args);
    };

    (window as any).__TAURI_INTERNALS__ = {
      invoke,
      transformCallback,
      metadata: {
        currentWindow: { label: "main" },
        currentWebview: { label: "main" },
      },
      convertFileSrc: (filePath: string, _protocol?: string) => filePath,
      runtimeAuthToken: "copythat-e2e-shim",
    };

    (window as any).__copythat_e2e__ = {
      setHandler(cmd: string, handler: InvokeHandler) {
        handlers.set(cmd, handler);
      },
      clearHandler(cmd: string) {
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
      emit(event: string, payload: unknown) {
        const set = listenersByEvent.get(event);
        if (!set) return;
        const message = { event, id: 0, payload };
        for (const id of set) {
          const cb = callbacks.get(id);
          cb?.(message);
        }
      },
      calls() {
        return calls.slice();
      },
      listenerCount(event: string) {
        return listenersByEvent.get(event)?.size ?? 0;
      },
      setDefaultHandler(handler: InvokeHandler) {
        defaultHandler = handler;
      },
    };
  };
}
