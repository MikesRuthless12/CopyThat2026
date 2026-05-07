<script lang="ts">
  // Phase 46.6 — Settings → Plugins tab.
  //
  // Lists installed plugins under <config_dir>/plugins/ with:
  //  - enable/disable toggle
  //  - manifest viewer (name, version, hooks)
  //  - capability grant matrix (one row per manifest-declared capability,
  //    with a per-row toggle bound to plugin_grant_capability /
  //    plugin_revoke_capability)
  //  - "Install from file" picker (uses Tauri dialog to pick a .wasm)
  //  - "Install from URL" two-phase flow (preview -> confirm)
  //
  // The IPC layer (apps/copythat-ui/src-tauri/src/plugin_commands.rs)
  // owns the on-disk store; this component is purely a thin renderer.
  import { onDestroy, onMount } from "svelte";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";

  import { t } from "../i18n";
  import {
    pluginDisable,
    pluginEnable,
    pluginGrantCapability,
    pluginInstallFromFile,
    pluginInstallFromUrl,
    pluginList,
    pluginRevokeCapability,
    type PluginEntryDto,
    type PluginInstallPreviewDto,
  } from "../ipc";

  let plugins = $state<PluginEntryDto[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // URL install state machine. `null` for the idle state, an object
  // once the user has typed URLs and clicked "Preview", and the same
  // object updated with `installed = true` after they confirm.
  let urlFormShow = $state(false);
  let urlWasm = $state("");
  let urlManifest = $state("");
  let urlPreview = $state<PluginInstallPreviewDto | null>(null);
  let urlBusy = $state(false);

  // Per-(plugin, capability) in-flight map so the user can't fire
  // overlapping grant/revoke IPC for the same row by clicking the
  // checkbox repeatedly. Keyed by `${name}:${capability}`.
  let capPending = $state<Record<string, boolean>>({});
  // Reset-form timer tracked across re-renders so onDestroy can
  // cancel it. Without the cancel, the closure can fire against an
  // already-unmounted component when the user navigates away from
  // the Plugins tab during the post-install confirmation hold.
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  /// Render any thrown value into a stable string for the inline
  /// error banner. Tauri rejects with the Rust `Result<_, String>`
  /// payload as a plain string, but transport errors arrive as
  /// `Error` instances and a defensive fallback covers anything
  /// else without yielding `"[object Object]"`.
  function formatError(e: unknown): string {
    if (typeof e === "string") return e;
    if (e instanceof Error) return e.message;
    try {
      return JSON.stringify(e);
    } catch {
      return String(e);
    }
  }

  async function refresh(showSpinner: boolean = true) {
    if (showSpinner) loading = true;
    error = null;
    try {
      plugins = await pluginList();
    } catch (e) {
      error = formatError(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    void refresh();
  });

  onDestroy(() => {
    if (resetTimer !== null) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }
  });

  async function onToggleEnabled(p: PluginEntryDto) {
    try {
      const updated = p.enabled
        ? await pluginDisable(p.name)
        : await pluginEnable(p.name);
      // Replace the entry in-place rather than refresh()ing the whole
      // list — keeps any expanded-row state stable.
      plugins = plugins.map((x) => (x.name === updated.name ? updated : x));
    } catch (e) {
      error = formatError(e);
    }
  }

  async function onToggleCapability(p: PluginEntryDto, capability: string) {
    const key = `${p.name}:${capability}`;
    if (capPending[key]) return;
    capPending = { ...capPending, [key]: true };
    const granted = p.grantedCapabilities.includes(capability);
    try {
      const updated = granted
        ? await pluginRevokeCapability(p.name, capability)
        : await pluginGrantCapability(p.name, capability);
      plugins = plugins.map((x) => (x.name === updated.name ? updated : x));
    } catch (e) {
      error = formatError(e);
    } finally {
      const next = { ...capPending };
      delete next[key];
      capPending = next;
    }
  }

  async function onInstallFromFile() {
    error = null;
    try {
      // `multiple: false` returns `string | null` per the
      // @tauri-apps/plugin-dialog v2 types. The user cancelling
      // returns null; the Array.isArray() defensive branch from
      // the v1 typings is no longer needed.
      const wasmPath = await openDialog({
        multiple: false,
        directory: false,
        filters: [{ name: "WASM plugin", extensions: ["wasm"] }],
      });
      if (!wasmPath) return;
      await pluginInstallFromFile({ wasmPath });
      await refresh(false);
    } catch (e) {
      error = formatError(e);
    }
  }

  async function onPreviewFromUrl(e: Event) {
    e.preventDefault();
    if (!urlWasm.trim() || !urlManifest.trim()) return;
    urlBusy = true;
    error = null;
    try {
      urlPreview = await pluginInstallFromUrl({
        wasmUrl: urlWasm.trim(),
        manifestUrl: urlManifest.trim(),
        expectedHash: null,
      });
    } catch (e) {
      error = formatError(e);
      urlPreview = null;
    } finally {
      urlBusy = false;
    }
  }

  async function onConfirmFromUrl() {
    if (!urlPreview) return;
    urlBusy = true;
    error = null;
    try {
      const committed = await pluginInstallFromUrl({
        wasmUrl: urlWasm.trim(),
        manifestUrl: urlManifest.trim(),
        expectedHash: urlPreview.hash,
      });
      urlPreview = committed;
      await refresh(false);
      // Reset the form on success — `installed = true` keeps the
      // confirmation card visible for one render cycle so the user
      // sees the success state before the form collapses.
      if (resetTimer !== null) clearTimeout(resetTimer);
      resetTimer = setTimeout(() => {
        resetTimer = null;
        urlFormShow = false;
        urlPreview = null;
        urlWasm = "";
        urlManifest = "";
      }, 1500);
    } catch (e) {
      error = formatError(e);
    } finally {
      urlBusy = false;
    }
  }

  function cancelUrlInstall() {
    if (resetTimer !== null) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }
    urlFormShow = false;
    urlPreview = null;
    urlWasm = "";
    urlManifest = "";
    error = null;
  }

  function toggleUrlForm() {
    if (urlFormShow) {
      cancelUrlInstall();
    } else {
      urlFormShow = true;
      urlPreview = null;
      error = null;
    }
  }
</script>

<div class="plugins-tab">
  <h3>{t("plugin-heading")}</h3>
  <p class="hint">{t("plugin-hint")}</p>

  {#if loading}
    <p>{t("settings-loading")}</p>
  {:else}
    {#if error}
      <p class="error" role="alert">{error}</p>
    {/if}

    {#if plugins.length === 0}
      <p class="empty">{t("plugin-list-empty")}</p>
    {:else}
      <ul class="plugin-list">
        {#each plugins as p (p.name)}
          <li class="plugin-row">
            <div class="head">
              <div class="ident">
                <strong>{p.name}</strong>
                <span class="version">v{p.version}</span>
              </div>
              <label class="enabled-toggle">
                <input
                  type="checkbox"
                  checked={p.enabled}
                  aria-label={`${p.name}: ${p.enabled ? t("plugin-enabled") : t("plugin-disabled")}`}
                  onchange={() => onToggleEnabled(p)}
                />
                <span>{p.enabled ? t("plugin-enabled") : t("plugin-disabled")}</span>
              </label>
            </div>
            <div class="meta">
              <span class="meta-label">{t("plugin-hooks")}:</span>
              <span class="meta-value">{p.hooks.join(", ")}</span>
            </div>
            {#if p.manifestCapabilities.length > 0}
              <div class="capabilities">
                <span class="meta-label">{t("plugin-capabilities")}:</span>
                <ul class="capability-grid">
                  {#each p.manifestCapabilities as cap (cap)}
                    <li class="capability-row">
                      <label>
                        <input
                          type="checkbox"
                          checked={p.grantedCapabilities.includes(cap)}
                          disabled={capPending[`${p.name}:${cap}`]}
                          aria-label={`${p.name}: ${cap}`}
                          onchange={() => onToggleCapability(p, cap)}
                        />
                        <code>{cap}</code>
                      </label>
                    </li>
                  {/each}
                </ul>
              </div>
            {:else}
              <div class="meta">
                <span class="meta-label">{t("plugin-capabilities")}:</span>
                <span class="meta-value">{t("plugin-no-capabilities")}</span>
              </div>
            {/if}
            <div class="path">
              <span class="meta-label">{t("plugin-directory")}:</span>
              <code>{p.directory}</code>
            </div>
          </li>
        {/each}
      </ul>
    {/if}

    <div class="install-actions">
      <button type="button" class="primary" onclick={onInstallFromFile}>
        {t("plugin-install-from-file")}
      </button>
      <button type="button" onclick={toggleUrlForm}>
        {t("plugin-install-from-url")}
      </button>
    </div>

    {#if urlFormShow}
      <form class="url-form" onsubmit={onPreviewFromUrl}>
        <label>
          <span>{t("plugin-url-wasm")}</span>
          <input
            type="url"
            bind:value={urlWasm}
            required
            disabled={urlPreview !== null}
            placeholder="https://example.com/my-plugin.wasm"
          />
        </label>
        <label>
          <span>{t("plugin-url-manifest")}</span>
          <input
            type="url"
            bind:value={urlManifest}
            required
            disabled={urlPreview !== null}
            placeholder="https://example.com/plugin.toml"
          />
        </label>
        {#if !urlPreview}
          <div class="form-actions">
            <button type="submit" class="primary" disabled={urlBusy}>
              {t("plugin-url-preview")}
            </button>
            <button type="button" onclick={cancelUrlInstall} disabled={urlBusy}>
              {t("remote-cancel")}
            </button>
          </div>
        {:else}
          <div class="preview-card">
            <p class="preview-line">
              <strong>{urlPreview.name}</strong>
              <span class="version">v{urlPreview.version}</span>
            </p>
            <p class="preview-line">
              <span class="meta-label">{t("plugin-url-hash")}:</span>
              <code class="hash">{urlPreview.hash}</code>
            </p>
            <p class="preview-line">
              <span class="meta-label">{t("plugin-hooks")}:</span>
              <span>{urlPreview.hooks.join(", ")}</span>
            </p>
            <p class="preview-line">
              <span class="meta-label">{t("plugin-capabilities")}:</span>
              <span>
                {urlPreview.capabilities.length === 0
                  ? t("plugin-no-capabilities")
                  : urlPreview.capabilities.join(", ")}
              </span>
            </p>
            {#if urlPreview.installed}
              <p class="preview-success" role="status">{t("plugin-enabled")}</p>
            {:else}
              <div class="form-actions">
                <button
                  type="button"
                  class="primary"
                  onclick={onConfirmFromUrl}
                  disabled={urlBusy}
                >
                  {t("plugin-url-confirm")}
                </button>
                <button type="button" onclick={cancelUrlInstall} disabled={urlBusy}>
                  {t("remote-cancel")}
                </button>
              </div>
            {/if}
          </div>
        {/if}
      </form>
    {/if}
  {/if}
</div>

<style>
  .plugins-tab {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .hint {
    margin: 0;
    color: var(--fg-dim, var(--muted, #5f5f5f));
    font-size: 12px;
  }
  .empty {
    color: var(--muted, #666);
    font-style: italic;
  }
  .error {
    color: var(--error, #a00);
  }
  .plugin-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .plugin-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px;
    border: 1px solid var(--border, #d0d0d0);
    border-radius: 6px;
    background: var(--surface, #f8f8f8);
  }
  .plugin-row .head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 12px;
  }
  .ident {
    display: flex;
    gap: 8px;
    align-items: baseline;
  }
  .version {
    color: var(--muted, #666);
    font-size: 0.85em;
    font-family: var(--mono, ui-monospace, monospace);
  }
  .enabled-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.9em;
  }
  .meta,
  .capabilities,
  .path {
    display: flex;
    gap: 8px;
    align-items: baseline;
    flex-wrap: wrap;
  }
  .meta-label {
    font-weight: 600;
    font-size: 0.85em;
    color: var(--muted, #666);
  }
  .meta-value {
    font-size: 0.9em;
  }
  .capability-grid {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-wrap: wrap;
    gap: 6px 14px;
  }
  .capability-row label {
    display: flex;
    gap: 4px;
    align-items: center;
    font-size: 0.85em;
  }
  .capability-row code {
    font-size: 0.9em;
  }
  .path code {
    font-size: 0.8em;
    color: var(--muted, #666);
    word-break: break-all;
  }
  .install-actions {
    display: flex;
    gap: 8px;
  }
  .url-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border: 1px solid var(--border, #d0d0d0);
    border-radius: 6px;
  }
  .url-form label {
    display: grid;
    grid-template-columns: 140px 1fr;
    gap: 8px;
    align-items: center;
  }
  .form-actions {
    display: flex;
    gap: 8px;
    margin-top: 6px;
  }
  .preview-card {
    padding: 10px;
    background: var(--surface-2, rgba(128, 128, 128, 0.05));
    border-left: 3px solid var(--accent, #4a90e2);
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .preview-line {
    margin: 0;
    font-size: 0.9em;
    display: flex;
    gap: 6px;
    align-items: baseline;
    flex-wrap: wrap;
  }
  .preview-success {
    margin: 6px 0 0;
    padding: 8px;
    background: rgba(0, 128, 0, 0.08);
    border-left: 3px solid #1a6b1a;
    color: var(--ok, #1a6b1a);
    font-size: 0.9em;
  }
  .hash {
    font-size: 0.75em;
    word-break: break-all;
  }
  .primary {
    font-weight: 600;
  }
</style>
