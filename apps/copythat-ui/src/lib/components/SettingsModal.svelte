<!--
  Phase 12 — full Settings modal. Six tabs mirroring the Phase 12
  build-prompt spec:

  - General       : language, theme, start-with-OS, single-instance, minimize-to-tray
  - Transfer      : buffer size, verify, concurrency, reflink, fsync, preserve *
  - Shell         : context menu, intercept default copy (Win), notify on completion
  - Secure delete : method, confirm twice
  - Advanced      : log level, telemetry (always off, displayed as read-only),
                    error policy, history retention, database path
  - Profiles      : save / load / delete / export / import named configs

  Wire-shape: a single `SettingsDto` flows both ways through IPC.
  `get_settings` loads on open; every control change invokes
  `update_settings` so the Rust side both persists to TOML and
  republishes to the live `AppState.settings` lock — the engine then
  picks up new values on the next enqueue without a restart.

  Phase 11b shipped the skeleton (one General tab); Phase 12 fills
  in the remaining five + profile management. Language switcher
  semantics (Intl.DisplayNames rendering + English-pinned-first
  ordering) carried over from Phase 11b.
-->
<script lang="ts">
  import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";

  import Icon from "../icons/Icon.svelte";
  import RemotesTab from "./RemotesTab.svelte";
  import { i18nVersion, locale, setLocale, t } from "../i18n";
  import {
    closeSettings,
    pushToast,
    settingsOpen,
    setErrorDisplayMode,
  } from "../stores";
  import {
    deleteProfile,
    exportProfile,
    getSettings,
    importProfile,
    listProfiles,
    loadProfile,
    resetSettings,
    saveProfile,
    updateSettings,
    updaterCheckNow,
    updaterDismissVersion,
  } from "../ipc";
  import type { ProfileInfoDto, SettingsDto, UpdateCheckDto } from "../types";

  type TabId =
    | "general"
    | "transfer"
    | "filters"
    | "shell"
    | "secure-delete"
    | "advanced"
    | "updater"
    | "network"
    | "remotes"
    | "profiles";

  let activeTab: TabId = $state("general");
  let settings = $state<SettingsDto | null>(null);
  let profiles = $state<ProfileInfoDto[]>([]);
  let profileNameInput = $state("");
  let busy = $state(false);

  // Endonym fallback — used when `Intl.DisplayNames` doesn't return
  // a localized name (older WebKit / webview2 builds return the
  // BCP-47 tag verbatim). Each value is the language's own name.
  const ENDONYMS: Record<string, string> = {
    en: "English",
    es: "Español",
    "zh-CN": "中文 (简)",
    hi: "हिन्दी",
    ar: "العربية",
    "pt-BR": "Português (BR)",
    ru: "Русский",
    ja: "日本語",
    de: "Deutsch",
    fr: "Français",
    ko: "한국어",
    it: "Italiano",
    tr: "Türkçe",
    vi: "Tiếng Việt",
    pl: "Polski",
    nl: "Nederlands",
    id: "Bahasa Indonesia",
    uk: "Українська",
  };

  function displayName(code: string, uiLocale: string): string {
    try {
      const dn = new Intl.DisplayNames([uiLocale], { type: "language" });
      const name = dn.of(code);
      if (name && name !== code) return name;
    } catch {
      // Fallback to endonym below.
    }
    return ENDONYMS[code] ?? code;
  }

  // Phase 14a — HTML `<input type="date">` yields "yyyy-mm-dd" in the
  // user's local calendar; we store and compare against mtime in UTC
  // seconds so "everything before 2026-01-01" means midnight UTC
  // regardless of where the user lives. `null` on either side of the
  // boundary means "unbounded on that end".
  function secsToDateInput(secs: number | null): string {
    if (secs === null) return "";
    const d = new Date(secs * 1000);
    const y = d.getUTCFullYear().toString().padStart(4, "0");
    const m = (d.getUTCMonth() + 1).toString().padStart(2, "0");
    const day = d.getUTCDate().toString().padStart(2, "0");
    return `${y}-${m}-${day}`;
  }

  function dateInputToSecs(input: string): number | null {
    if (!input) return null;
    const parts = input.split("-");
    if (parts.length !== 3) return null;
    const y = parseInt(parts[0], 10);
    const m = parseInt(parts[1], 10);
    const d = parseInt(parts[2], 10);
    if (!Number.isFinite(y) || !Number.isFinite(m) || !Number.isFinite(d)) return null;
    return Math.floor(Date.UTC(y, m - 1, d) / 1000);
  }

  // English pinned first; rest sorted by localised display name with
  // `localeCompare` respecting the active locale's collation.
  const orderedLocales = $derived.by<string[]>(() => {
    const all = $locale.available;
    const ui = $locale.code;
    const withoutEn = all.filter((c) => c !== "en");
    withoutEn.sort((a, b) =>
      displayName(a, ui).localeCompare(displayName(b, ui), ui),
    );
    const head = all.includes("en") ? ["en"] : [];
    return [...head, ...withoutEn];
  });

  // Load settings + profiles whenever the modal opens. Skip if we
  // already have them — a modal close/re-open shouldn't cost a
  // full IPC round-trip unless the user explicitly reset.
  $effect(() => {
    if ($settingsOpen && settings === null) {
      void refresh();
    }
  });

  async function refresh() {
    busy = true;
    try {
      const [s, p] = await Promise.all([getSettings(), listProfiles()]);
      settings = s;
      profiles = p;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  /// Commit the current in-memory `settings` back to Rust. Called
  /// after every control change (the controls write through `bind:`
  /// to the `settings` state and then we push). We don't debounce
  /// at this layer — the Rust `update_settings` handler is cheap and
  /// TOML writes are atomic, so an 80 ms slider scrub triggers maybe
  /// a dozen writes worst case.
  async function pushSettings() {
    if (!settings) return;
    try {
      const next = await updateSettings(settings);
      settings = next;
      // Mirror persisted UI-render preferences into the live store
      // so components outside this modal (ErrorModal / ErrorPromptDrawer
      // in App.svelte) re-render against the new value without a
      // follow-up IPC round-trip.
      setErrorDisplayMode(next.general.errorDisplayMode);
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onLocaleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    const code = target.value;
    if (!settings) return;
    settings = { ...settings, general: { ...settings.general, language: code } };
    // Hot-swap the webview BEFORE the server push so the rest of the
    // modal re-renders in the new language immediately.
    await setLocale(code);
    await pushSettings();
  }

  async function onResetAll() {
    if (!confirm(t("settings-reset-confirm"))) return;
    try {
      const next = await resetSettings();
      settings = next;
      setErrorDisplayMode(next.general.errorDisplayMode);
      pushToast("success", "toast-settings-reset");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // ---- Profiles ------------------------------------------------------

  async function onSaveProfile() {
    const name = profileNameInput.trim();
    if (!name) return;
    try {
      await saveProfile(name);
      profileNameInput = "";
      profiles = await listProfiles();
      pushToast("success", "toast-profile-saved");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onLoadProfile(name: string) {
    try {
      const next = await loadProfile(name);
      settings = next;
      setErrorDisplayMode(next.general.errorDisplayMode);
      pushToast("info", "toast-profile-loaded");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onDeleteProfile(name: string) {
    try {
      await deleteProfile(name);
      profiles = await listProfiles();
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onExportProfile(name: string) {
    const dest = await saveDialog({
      defaultPath: `${name}.json`,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (typeof dest !== "string") return;
    try {
      await exportProfile(name, dest);
      pushToast("success", "toast-profile-exported");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onImportProfile() {
    const src = await openDialog({
      multiple: false,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (typeof src !== "string") return;
    const name = prompt(t("settings-profile-import-prompt"));
    if (!name) return;
    try {
      await importProfile(name, src);
      profiles = await listProfiles();
      pushToast("success", "toast-profile-imported");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  // ---- Error-policy helper ------------------------------------------
  // The ErrorPolicy enum's three inline-args shape doesn't bind cleanly
  // with `<select>`, so we carry the variant choice in a plain string
  // and write back a structured object on change.
  function errorPolicyKind(
    ep: SettingsDto["advanced"]["errorPolicy"],
  ): string {
    return ep.kind;
  }

  function setErrorPolicy(kind: string) {
    if (!settings) return;
    let next: SettingsDto["advanced"]["errorPolicy"];
    switch (kind) {
      case "skip":
        next = { kind: "skip" };
        break;
      case "abort":
        next = { kind: "abort" };
        break;
      case "retryN":
        next = { kind: "retryN", maxAttempts: 3, backoffMs: 250 };
        break;
      default:
        next = { kind: "ask" };
    }
    settings = {
      ...settings,
      advanced: { ...settings.advanced, errorPolicy: next },
    };
    void pushSettings();
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      closeSettings();
    }
  }

  // ---- Phase 15 updater -----------------------------------------------
  // Most-recent check result; populated on demand. Rendered below the
  // channel selector when non-null.
  let lastCheck = $state<UpdateCheckDto | null>(null);
  let checking = $state(false);

  async function onCheckForUpdatesNow() {
    checking = true;
    try {
      // `force: true` bypasses the 24 h throttle — the UI button is
      // an explicit user action so the throttle doesn't apply.
      const res = await updaterCheckNow(true, null);
      lastCheck = res;
      // The backend bumped `lastCheckUnixSecs` on its side; re-pull
      // settings so the displayed timestamp stays in sync without the
      // user having to close+reopen the modal.
      if (settings) {
        const s = await getSettings();
        settings = s;
      }
      if (res.isNewer) {
        pushToast("info", "toast-update-available");
      } else if (res.availableVersion) {
        pushToast("success", "toast-update-up-to-date");
      }
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    } finally {
      checking = false;
    }
  }

  /**
   * Phase 34 — Settings → Advanced → Audit log actions.
   * `onAuditTestWrite` emits a synthetic LoginEvent through the live
   * sink so the user can confirm the log file is reachable; the
   * backend refuses when audit is disabled or the sink failed to
   * open. `onAuditVerifyChain` recomputes the BLAKE3 chain hash end-
   * to-end and surfaces the summary as a toast.
   */
  async function onAuditTestWrite() {
    try {
      await invoke("audit_test_write");
      pushToast("success", "toast-audit-test-write-ok");
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onAuditVerifyChain() {
    try {
      const report = (await invoke("audit_verify")) as {
        total: number;
        matches: number;
        mismatches: number;
        missing: number;
      };
      if (report.mismatches === 0 && report.missing === 0) {
        pushToast("success", "toast-audit-verify-ok");
      } else {
        pushToast("error", "toast-audit-verify-failed");
      }
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  async function onDismissAvailable() {
    if (!lastCheck || !lastCheck.availableVersion) return;
    try {
      await updaterDismissVersion(lastCheck.availableVersion);
      if (settings) {
        settings = {
          ...settings,
          updater: {
            ...settings.updater,
            dismissedVersion: lastCheck.availableVersion,
          },
        };
      }
      lastCheck = null;
    } catch (e) {
      pushToast("error", e instanceof Error ? e.message : String(e));
    }
  }

  function formatLastCheckLabel(unixSecs: number): string {
    if (!unixSecs || unixSecs <= 0) return t("settings-updater-last-never");
    const d = new Date(unixSecs * 1000);
    return d.toLocaleString();
  }
</script>

{#if $settingsOpen}
  <div
    class="backdrop"
    role="presentation"
    onclick={closeSettings}
    onkeydown={onKeydown}
  >
    {#key $i18nVersion}
    <div
      class="modal"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="settings-title"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <header>
        <h2 id="settings-title">{t("settings-title")}</h2>
        <button
          class="close"
          type="button"
          aria-label={t("action-close")}
          onclick={closeSettings}
        >
          <Icon name="x" size={16} />
        </button>
      </header>

      {#if settings === null}
        <p class="loading">{t("settings-loading")}</p>
      {:else}
        <div class="body">
          <div class="tabs" role="tablist" aria-label={t("settings-title")}>
            {#each [["general", "settings-tab-general"], ["transfer", "settings-tab-transfer"], ["filters", "settings-tab-filters"], ["shell", "settings-tab-shell"], ["secure-delete", "settings-tab-secure-delete"], ["advanced", "settings-tab-advanced"], ["updater", "settings-tab-updater"], ["network", "settings-tab-network"], ["remotes", "settings-tab-remotes"], ["profiles", "settings-tab-profiles"]] as const as [id, key] (id)}
              <button
                type="button"
                role="tab"
                aria-selected={activeTab === id}
                aria-controls={`tab-${id}`}
                class:active={activeTab === id}
                onclick={() => (activeTab = id)}
              >
                {t(key)}
              </button>
            {/each}
          </div>

          <div class="tabpanel">
            {#if activeTab === "general"}
              <label class="row">
                <span class="label">{t("settings-section-language")}</span>
                <select value={$locale.code} onchange={onLocaleChange}>
                  {#each orderedLocales as code (code)}
                    <option value={code}>{displayName(code, $locale.code)}</option>
                  {/each}
                </select>
              </label>

              <label class="row">
                <span class="label">{t("settings-section-theme")}</span>
                <select
                  bind:value={settings.general.theme}
                  onchange={pushSettings}
                >
                  <option value="auto">{t("settings-theme-auto")}</option>
                  <option value="light">{t("settings-theme-light")}</option>
                  <option value="dark">{t("settings-theme-dark")}</option>
                </select>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.startWithOs}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-start-with-os")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.singleInstance}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-single-instance")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.minimizeToTray}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-minimize-to-tray")}</span>
              </label>

              <label class="row">
                <span class="label">{t("settings-error-display-mode")}</span>
                <select
                  bind:value={settings.general.errorDisplayMode}
                  onchange={pushSettings}
                >
                  <option value="modal">{t("settings-error-display-modal")}</option>
                  <option value="drawer">{t("settings-error-display-drawer")}</option>
                </select>
              </label>
              <p class="hint">{t("settings-error-display-mode-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.pasteShortcutEnabled}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-paste-shortcut")}</span>
              </label>
              <label class="row">
                <span class="label">{t("settings-paste-shortcut-combo")}</span>
                <input
                  type="text"
                  bind:value={settings.general.pasteShortcut}
                  onchange={pushSettings}
                  disabled={!settings.general.pasteShortcutEnabled}
                  placeholder="CmdOrCtrl+Shift+V"
                  spellcheck={false}
                />
              </label>
              <p class="hint">{t("settings-paste-shortcut-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.clipboardWatcherEnabled}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-clipboard-watcher")}</span>
              </label>
              <p class="hint">{t("settings-clipboard-watcher-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.general.autoResumeInterrupted}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-auto-resume")}</span>
              </label>
              <p class="hint">{t("settings-auto-resume-hint")}</p>

              {#if settings.dnd}
                <h4 class="subheading">{t("settings-dnd-heading")}</h4>

                <label class="row check">
                  <input
                    type="checkbox"
                    bind:checked={settings.dnd.springLoadEnabled}
                    onchange={pushSettings}
                  />
                  <span class="label">{t("settings-dnd-spring-load")}</span>
                </label>

                <label class="row">
                  <span class="label">{t("settings-dnd-spring-delay")}</span>
                  <input
                    type="number"
                    min="200"
                    max="2000"
                    step="50"
                    bind:value={settings.dnd.springLoadDelayMs}
                    onchange={pushSettings}
                    disabled={!settings.dnd.springLoadEnabled}
                  />
                </label>

                <label class="row check">
                  <input
                    type="checkbox"
                    bind:checked={settings.dnd.showDragThumbnails}
                    onchange={pushSettings}
                  />
                  <span class="label">{t("settings-dnd-thumbnails")}</span>
                </label>

                <label class="row check">
                  <input
                    type="checkbox"
                    bind:checked={settings.dnd.highlightInvalidTargets}
                    onchange={pushSettings}
                  />
                  <span class="label">{t("settings-dnd-invalid-highlight")}</span>
                </label>
              {/if}
            {:else if activeTab === "transfer"}
              <label class="row">
                <span class="label">{t("settings-buffer-size")}</span>
                <select
                  value={String(settings.transfer.bufferSizeBytes)}
                  onchange={(e) => {
                    if (!settings) return;
                    const v = parseInt((e.currentTarget as HTMLSelectElement).value, 10);
                    settings = { ...settings, transfer: { ...settings.transfer, bufferSizeBytes: v } };
                    void pushSettings();
                  }}
                >
                  <option value="65536">64 KiB</option>
                  <option value="262144">256 KiB</option>
                  <option value="1048576">1 MiB</option>
                  <option value="4194304">4 MiB</option>
                  <option value="8388608">8 MiB</option>
                  <option value="16777216">16 MiB</option>
                </select>
              </label>

              <label class="row">
                <span class="label">{t("settings-verify")}</span>
                <select
                  bind:value={settings.transfer.verify}
                  onchange={pushSettings}
                >
                  <option value="off">{t("settings-verify-off")}</option>
                  <option value="crc32">CRC32</option>
                  <option value="md5">MD5</option>
                  <option value="sha1">SHA-1</option>
                  <option value="sha256">SHA-256</option>
                  <option value="sha512">SHA-512</option>
                  <option value="xxhash3-64">xxHash3-64</option>
                  <option value="xxhash3-128">xxHash3-128</option>
                  <option value="blake3">BLAKE3</option>
                </select>
              </label>

              <label class="row">
                <span class="label">{t("settings-concurrency")}</span>
                <select
                  bind:value={settings.transfer.concurrency}
                  onchange={pushSettings}
                >
                  <option value="auto">{t("settings-concurrency-auto")}</option>
                  {#each [1, 2, 4, 8, 12, 16] as n (n)}
                    <option value={`manual-${n}`}>{n}</option>
                  {/each}
                </select>
              </label>

              <label class="row">
                <span class="label">{t("settings-reflink")}</span>
                <select
                  bind:value={settings.transfer.reflink}
                  onchange={pushSettings}
                >
                  <option value="prefer">{t("settings-reflink-prefer")}</option>
                  <option value="avoid">{t("settings-reflink-avoid")}</option>
                  <option value="disabled">{t("settings-reflink-disabled")}</option>
                </select>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.transfer.fsyncOnClose}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-fsync-on-close")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.transfer.preserveTimestamps}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-preserve-timestamps")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.transfer.preservePermissions}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-preserve-permissions")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.transfer.preserveAcls}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-preserve-acls")}</span>
              </label>

              <label class="row check" title={t("settings-preserve-sparseness-hint")}>
                <input
                  type="checkbox"
                  bind:checked={settings.transfer.preserveSparseness}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-preserve-sparseness")}</span>
              </label>

              <!-- Phase 24 — Security-metadata subsection (5 toggles) -->
              <div class="meta-subsection">
                <label
                  class="row check"
                  title={t("settings-preserve-security-metadata-hint")}
                >
                  <input
                    type="checkbox"
                    bind:checked={settings.transfer.preserveSecurityMetadata}
                    onchange={pushSettings}
                  />
                  <span class="label section-header"
                    >{t("settings-preserve-security-metadata")}</span
                  >
                </label>

                {#if settings.transfer.preserveSecurityMetadata}
                  <label
                    class="row check meta-child"
                    title={t("settings-preserve-motw-hint")}
                  >
                    <input
                      type="checkbox"
                      bind:checked={settings.transfer.preserveMotw}
                      onchange={pushSettings}
                    />
                    <span class="label">
                      {t("settings-preserve-motw")}
                      <span class="motw-warning" aria-hidden="true">⚠</span>
                    </span>
                  </label>

                  <label
                    class="row check meta-child"
                    title={t("settings-preserve-posix-acls-hint")}
                  >
                    <input
                      type="checkbox"
                      bind:checked={settings.transfer.preservePosixAcls}
                      onchange={pushSettings}
                    />
                    <span class="label">{t("settings-preserve-posix-acls")}</span>
                  </label>

                  <label
                    class="row check meta-child"
                    title={t("settings-preserve-selinux-hint")}
                  >
                    <input
                      type="checkbox"
                      bind:checked={settings.transfer.preserveSelinuxContexts}
                      onchange={pushSettings}
                    />
                    <span class="label">{t("settings-preserve-selinux")}</span>
                  </label>

                  <label
                    class="row check meta-child"
                    title={t("settings-preserve-resource-forks-hint")}
                  >
                    <input
                      type="checkbox"
                      bind:checked={settings.transfer.preserveResourceForks}
                      onchange={pushSettings}
                    />
                    <span class="label"
                      >{t("settings-preserve-resource-forks")}</span
                    >
                  </label>

                  <label class="row check meta-child">
                    <input
                      type="checkbox"
                      bind:checked={settings.transfer.appledoubleFallback}
                      onchange={pushSettings}
                    />
                    <span class="label">
                      {t("settings-appledouble-fallback")}
                    </span>
                  </label>
                {/if}
              </div>

              <label class="row">
                <span class="label">{t("settings-on-locked")}</span>
                <select
                  bind:value={settings.transfer.onLocked}
                  onchange={pushSettings}
                >
                  <option value="ask">{t("settings-on-locked-ask")}</option>
                  <option value="retry">{t("settings-on-locked-retry")}</option>
                  <option value="skip">{t("settings-on-locked-skip")}</option>
                  <option value="snapshot">{t("settings-on-locked-snapshot")}</option>
                </select>
              </label>
              <p class="hint">{t("settings-on-locked-hint")}</p>
            {:else if activeTab === "filters"}
              <p class="hint">{t("settings-filters-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.filters.enabled}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-filters-enabled")}</span>
              </label>

              <label class="row stacked">
                <span class="label">{t("settings-filters-include-globs")}</span>
                <textarea
                  rows="3"
                  placeholder={t("settings-filters-include-globs-placeholder")}
                  value={settings.filters.includeGlobs.join("\n")}
                  onchange={(e) => {
                    if (!settings) return;
                    const lines = (e.currentTarget as HTMLTextAreaElement).value
                      .split(/\r?\n/)
                      .map((s) => s.trim())
                      .filter(Boolean);
                    settings = { ...settings, filters: { ...settings.filters, includeGlobs: lines } };
                    void pushSettings();
                  }}
                ></textarea>
              </label>
              <p class="hint">{t("settings-filters-include-globs-hint")}</p>

              <label class="row stacked">
                <span class="label">{t("settings-filters-exclude-globs")}</span>
                <textarea
                  rows="3"
                  placeholder={t("settings-filters-exclude-globs-placeholder")}
                  value={settings.filters.excludeGlobs.join("\n")}
                  onchange={(e) => {
                    if (!settings) return;
                    const lines = (e.currentTarget as HTMLTextAreaElement).value
                      .split(/\r?\n/)
                      .map((s) => s.trim())
                      .filter(Boolean);
                    settings = { ...settings, filters: { ...settings.filters, excludeGlobs: lines } };
                    void pushSettings();
                  }}
                ></textarea>
              </label>
              <p class="hint">{t("settings-filters-exclude-globs-hint")}</p>

              <div class="row">
                <span class="label">{t("settings-filters-size-range")}</span>
              </div>
              <label class="row">
                <span class="label">{t("settings-filters-min-size-bytes")}</span>
                <input
                  type="number"
                  min="0"
                  value={settings.filters.minSizeBytes ?? ""}
                  onchange={(e) => {
                    if (!settings) return;
                    const v = (e.currentTarget as HTMLInputElement).value.trim();
                    const n = v === "" ? null : Math.max(0, parseInt(v, 10) || 0);
                    settings = { ...settings, filters: { ...settings.filters, minSizeBytes: n } };
                    void pushSettings();
                  }}
                />
              </label>
              <label class="row">
                <span class="label">{t("settings-filters-max-size-bytes")}</span>
                <input
                  type="number"
                  min="0"
                  value={settings.filters.maxSizeBytes ?? ""}
                  onchange={(e) => {
                    if (!settings) return;
                    const v = (e.currentTarget as HTMLInputElement).value.trim();
                    const n = v === "" ? null : Math.max(0, parseInt(v, 10) || 0);
                    settings = { ...settings, filters: { ...settings.filters, maxSizeBytes: n } };
                    void pushSettings();
                  }}
                />
              </label>

              <div class="row">
                <span class="label">{t("settings-filters-date-range")}</span>
              </div>
              <label class="row">
                <span class="label">{t("settings-filters-min-mtime")}</span>
                <input
                  type="date"
                  value={secsToDateInput(settings.filters.minMtimeUnixSecs)}
                  onchange={(e) => {
                    if (!settings) return;
                    const secs = dateInputToSecs((e.currentTarget as HTMLInputElement).value);
                    settings = { ...settings, filters: { ...settings.filters, minMtimeUnixSecs: secs } };
                    void pushSettings();
                  }}
                />
              </label>
              <label class="row">
                <span class="label">{t("settings-filters-max-mtime")}</span>
                <input
                  type="date"
                  value={secsToDateInput(settings.filters.maxMtimeUnixSecs)}
                  onchange={(e) => {
                    if (!settings) return;
                    const secs = dateInputToSecs((e.currentTarget as HTMLInputElement).value);
                    settings = { ...settings, filters: { ...settings.filters, maxMtimeUnixSecs: secs } };
                    void pushSettings();
                  }}
                />
              </label>

              <div class="row">
                <span class="label">{t("settings-filters-attributes")}</span>
              </div>
              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.filters.skipHidden}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-filters-skip-hidden")}</span>
              </label>
              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.filters.skipSystem}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-filters-skip-system")}</span>
              </label>
              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.filters.skipReadonly}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-filters-skip-readonly")}</span>
              </label>
            {:else if activeTab === "shell"}
              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.shell.contextMenuEnabled}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-context-menu")}</span>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.shell.interceptDefaultCopy}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-intercept-copy")}</span>
              </label>
              <p class="hint">{t("settings-intercept-copy-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.shell.notifyOnCompletion}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-notify-completion")}</span>
              </label>
            {:else if activeTab === "secure-delete"}
              <label class="row">
                <span class="label">{t("settings-shred-method")}</span>
                <select
                  bind:value={settings.secureDelete.method}
                  onchange={pushSettings}
                >
                  <option value="zero">{t("settings-shred-zero")}</option>
                  <option value="random">{t("settings-shred-random")}</option>
                  <option value="dod-3-pass">{t("settings-shred-dod3")}</option>
                  <option value="dod-7-pass">{t("settings-shred-dod7")}</option>
                  <option value="gutmann">{t("settings-shred-gutmann")}</option>
                  <option value="nist-800-88">{t("settings-shred-nist")}</option>
                </select>
              </label>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.secureDelete.confirmTwice}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-shred-confirm-twice")}</span>
              </label>
            {:else if activeTab === "advanced"}
              <label class="row">
                <span class="label">{t("settings-log-level")}</span>
                <select
                  bind:value={settings.advanced.logLevel}
                  onchange={pushSettings}
                >
                  <option value="off">{t("settings-log-off")}</option>
                  <option value="error">Error</option>
                  <option value="warn">Warn</option>
                  <option value="info">Info</option>
                  <option value="debug">Debug</option>
                  <option value="trace">Trace</option>
                </select>
              </label>

              <div class="row">
                <span class="label">{t("settings-telemetry")}</span>
                <span class="muted">{t("settings-telemetry-never")}</span>
              </div>

              <label class="row">
                <span class="label">{t("settings-error-policy")}</span>
                <select
                  value={errorPolicyKind(settings.advanced.errorPolicy)}
                  onchange={(e) => setErrorPolicy((e.currentTarget as HTMLSelectElement).value)}
                >
                  <option value="ask">{t("settings-error-policy-ask")}</option>
                  <option value="skip">{t("settings-error-policy-skip")}</option>
                  <option value="retryN">{t("settings-error-policy-retry")}</option>
                  <option value="abort">{t("settings-error-policy-abort")}</option>
                </select>
              </label>

              <label class="row">
                <span class="label">{t("settings-history-retention")}</span>
                <input
                  type="number"
                  min="0"
                  max="3650"
                  bind:value={settings.advanced.historyRetentionDays}
                  onchange={pushSettings}
                />
              </label>
              <p class="hint">{t("settings-history-retention-hint")}</p>

              <div class="row">
                <span class="label">{t("settings-database-path")}</span>
                <span class="muted">
                  {settings.advanced.databasePath ?? t("settings-database-path-default")}
                </span>
              </div>

              <h4 class="subheading">{t("settings-mount-heading")}</h4>
              <p class="hint">{t("settings-mount-hint")}</p>

              {#if settings.mount}
                <label class="row check">
                  <input
                    type="checkbox"
                    bind:checked={settings.mount.mountOnLaunch}
                    onchange={pushSettings}
                  />
                  <span class="label">{t("settings-mount-on-launch")}</span>
                </label>

                <label class="row">
                  <span class="label">{t("settings-mount-on-launch-path")}</span>
                  <input
                    type="text"
                    bind:value={settings.mount.mountOnLaunchPath}
                    placeholder={t("settings-mount-on-launch-path-placeholder")}
                    disabled={!settings.mount.mountOnLaunch}
                    onchange={pushSettings}
                  />
                </label>
              {/if}

              <h4 class="subheading">{t("settings-audit-heading")}</h4>
              <p class="hint">{t("settings-audit-hint")}</p>

              {#if settings.audit}
                <label class="row check">
                  <input
                    type="checkbox"
                    bind:checked={settings.audit.enabled}
                    onchange={pushSettings}
                  />
                  <span class="label">{t("settings-audit-enable")}</span>
                </label>

                <label class="row">
                  <span class="label">{t("settings-audit-format")}</span>
                  <select
                    bind:value={settings.audit.format}
                    disabled={!settings.audit.enabled}
                    onchange={pushSettings}
                  >
                    <option value="json-lines">{t("settings-audit-format-json-lines")}</option>
                    <option value="csv">{t("settings-audit-format-csv")}</option>
                    <option value="syslog">{t("settings-audit-format-syslog")}</option>
                    <option value="cef">{t("settings-audit-format-cef")}</option>
                    <option value="leef">{t("settings-audit-format-leef")}</option>
                  </select>
                </label>

                <label class="row">
                  <span class="label">{t("settings-audit-file-path")}</span>
                  <input
                    type="text"
                    bind:value={settings.audit.filePath}
                    placeholder={t("settings-audit-file-path-placeholder")}
                    disabled={!settings.audit.enabled}
                    onchange={pushSettings}
                  />
                </label>

                <label class="row">
                  <span class="label">{t("settings-audit-max-size")}</span>
                  <input
                    type="number"
                    min="0"
                    step="1048576"
                    bind:value={settings.audit.maxSizeBytes}
                    disabled={!settings.audit.enabled}
                    onchange={pushSettings}
                  />
                </label>

                <label class="row check">
                  <input
                    type="checkbox"
                    checked={settings.audit.worm === "on"}
                    disabled={!settings.audit.enabled}
                    onchange={(e) => {
                      if (settings && settings.audit) {
                        settings.audit.worm = (e.currentTarget as HTMLInputElement).checked
                          ? "on"
                          : "off";
                        pushSettings();
                      }
                    }}
                  />
                  <span class="label">{t("settings-audit-worm")}</span>
                </label>
                <p class="hint">{t("settings-audit-worm-hint")}</p>

                <div class="row end">
                  <button
                    type="button"
                    class="secondary"
                    disabled={!settings.audit.enabled}
                    onclick={onAuditTestWrite}
                  >
                    {t("settings-audit-test-write")}
                  </button>
                  <button
                    type="button"
                    class="secondary"
                    disabled={!settings.audit.enabled}
                    onclick={onAuditVerifyChain}
                  >
                    {t("settings-audit-verify-chain")}
                  </button>
                </div>
              {/if}

              <div class="row end">
                <button class="danger" type="button" onclick={onResetAll} disabled={busy}>
                  {t("settings-reset-all")}
                </button>
              </div>
            {:else if activeTab === "updater"}
              <p class="hint">{t("settings-updater-hint")}</p>

              <label class="row check">
                <input
                  type="checkbox"
                  bind:checked={settings.updater.autoCheck}
                  onchange={pushSettings}
                />
                <span class="label">{t("settings-updater-auto-check")}</span>
              </label>

              <label class="row">
                <span class="label">{t("settings-updater-channel")}</span>
                <select
                  bind:value={settings.updater.channel}
                  onchange={pushSettings}
                >
                  <option value="stable">{t("settings-updater-channel-stable")}</option>
                  <option value="beta">{t("settings-updater-channel-beta")}</option>
                </select>
              </label>

              <div class="row">
                <span class="label">{t("settings-updater-last-check")}</span>
                <span class="muted">
                  {formatLastCheckLabel(settings.updater.lastCheckUnixSecs)}
                </span>
              </div>

              <div class="row end">
                <button
                  type="button"
                  class="secondary"
                  onclick={onCheckForUpdatesNow}
                  disabled={checking}
                >
                  {checking
                    ? t("settings-updater-checking")
                    : t("settings-updater-check-now")}
                </button>
              </div>

              {#if lastCheck && lastCheck.availableVersion}
                <div class="row update-summary" data-tone={lastCheck.isNewer ? "available" : "up-to-date"}>
                  {#if lastCheck.isNewer}
                    <span class="label">
                      {t("settings-updater-available")} —
                      <strong>{lastCheck.availableVersion}</strong>
                    </span>
                  {:else}
                    <span class="label">{t("settings-updater-up-to-date")}</span>
                  {/if}
                </div>
                {#if lastCheck.notes}
                  <p class="hint notes">{lastCheck.notes}</p>
                {/if}
                {#if lastCheck.isNewer}
                  <div class="row end">
                    <button
                      type="button"
                      class="tiny"
                      onclick={onDismissAvailable}
                    >
                      {t("settings-updater-dismiss")}
                    </button>
                  </div>
                {/if}
              {/if}

              {#if settings.updater.dismissedVersion}
                <p class="hint">
                  {t("settings-updater-dismissed")}:
                  <strong>{settings.updater.dismissedVersion}</strong>
                </p>
              {/if}
            {:else if activeTab === "network"}
              <p class="hint">{t("settings-network-hint")}</p>

              <label class="row">
                <span class="label">{t("settings-network-mode")}</span>
                <select
                  bind:value={settings.network.mode}
                  onchange={pushSettings}
                >
                  <option value="off">{t("settings-network-mode-off")}</option>
                  <option value="fixed">{t("settings-network-mode-fixed")}</option>
                  <option value="schedule">{t("settings-network-mode-schedule")}</option>
                </select>
              </label>

              {#if settings.network.mode === "fixed"}
                <label class="row">
                  <span class="label">{t("settings-network-cap-mbps")}</span>
                  <input
                    type="number"
                    min="0"
                    step="1"
                    value={Math.round(
                      settings.network.fixedBytesPerSecond / (1024 * 1024),
                    )}
                    onchange={(e) => {
                      if (!settings) return;
                      const mb = parseInt(
                        (e.currentTarget as HTMLInputElement).value,
                        10,
                      );
                      const bps = Number.isFinite(mb) && mb > 0
                        ? mb * 1024 * 1024
                        : 0;
                      settings = {
                        ...settings,
                        network: { ...settings.network, fixedBytesPerSecond: bps },
                      };
                      void pushSettings();
                    }}
                  />
                </label>
              {/if}

              {#if settings.network.mode === "schedule"}
                <label class="row column">
                  <span class="label">{t("settings-network-schedule")}</span>
                  <textarea
                    rows="3"
                    spellcheck={false}
                    placeholder="08:00,512k 12:00,off 13:00,512k 18:00,10M Sat-Sun,unlimited"
                    bind:value={settings.network.scheduleSpec}
                    onchange={pushSettings}
                  ></textarea>
                </label>
                <p class="hint">{t("settings-network-schedule-hint")}</p>
              {/if}

              <p class="section-header">{t("settings-network-auto-header")}</p>

              {#each [["autoOnMetered", "settings-network-auto-metered"], ["autoOnBattery", "settings-network-auto-battery"], ["autoOnCellular", "settings-network-auto-cellular"]] as const as [field, key] (field)}
                <label class="row">
                  <span class="label">{t(key)}</span>
                  <select
                    value={settings.network[field].kind}
                    onchange={(e) => {
                      if (!settings) return;
                      const kind = (e.currentTarget as HTMLSelectElement).value as
                        | "unchanged"
                        | "pause"
                        | "cap";
                      const next = kind === "cap"
                        ? { kind: "cap" as const, value: 1024 * 1024 }
                        : { kind } as { kind: "unchanged" | "pause" };
                      settings = {
                        ...settings,
                        network: { ...settings.network, [field]: next },
                      };
                      void pushSettings();
                    }}
                  >
                    <option value="unchanged">{t("settings-network-auto-unchanged")}</option>
                    <option value="pause">{t("settings-network-auto-pause")}</option>
                    <option value="cap">{t("settings-network-auto-cap")}</option>
                  </select>
                </label>
              {/each}
            {:else if activeTab === "remotes"}
              <RemotesTab />
            {:else if activeTab === "profiles"}
              <p class="hint">{t("settings-profiles-hint")}</p>
              <div class="row">
                <input
                  type="text"
                  placeholder={t("settings-profile-name-placeholder")}
                  bind:value={profileNameInput}
                />
                <button type="button" class="secondary" onclick={onSaveProfile} disabled={busy || !profileNameInput.trim()}>
                  {t("settings-profile-save")}
                </button>
                <button type="button" class="secondary" onclick={onImportProfile} disabled={busy}>
                  {t("settings-profile-import")}
                </button>
              </div>

              {#if profiles.length === 0}
                <p class="empty">{t("settings-profile-empty")}</p>
              {:else}
                <ul class="profile-list">
                  {#each profiles as p (p.name)}
                    <li>
                      <span class="profile-name" title={p.path}>{p.name}</span>
                      <button
                        type="button"
                        class="tiny"
                        onclick={() => onLoadProfile(p.name)}
                        disabled={busy}
                      >
                        {t("settings-profile-load")}
                      </button>
                      <button
                        type="button"
                        class="tiny"
                        onclick={() => onExportProfile(p.name)}
                        disabled={busy}
                      >
                        {t("settings-profile-export")}
                      </button>
                      <button
                        type="button"
                        class="tiny danger"
                        onclick={() => onDeleteProfile(p.name)}
                        disabled={busy}
                      >
                        {t("settings-profile-delete")}
                      </button>
                    </li>
                  {/each}
                </ul>
              {/if}
            {/if}
          </div>
        </div>
      {/if}
    </div>
    {/key}
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 92;
  }

  .modal {
    width: min(720px, 96vw);
    max-height: 86vh;
    padding: 12px 14px 14px;
    background: var(--surface, #ffffff);
    color: var(--fg, #1f1f1f);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 10px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.24);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 4px;
    border-bottom: 1px solid var(--border, rgba(128, 128, 128, 0.2));
  }

  h2 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
  }

  .close {
    background: transparent;
    border: 1px solid transparent;
    color: inherit;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .close:hover {
    background: var(--hover, rgba(128, 128, 128, 0.14));
  }

  .loading {
    padding: 24px 16px;
    color: var(--muted, #6a6a6a);
    font-size: 13px;
    text-align: center;
  }

  .body {
    display: flex;
    gap: 14px;
    min-height: 280px;
    max-height: calc(86vh - 48px);
    overflow: hidden;
  }

  .tabs {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 160px;
    padding-right: 10px;
    border-right: 1px solid var(--border, rgba(128, 128, 128, 0.18));
  }

  .tabs button {
    text-align: left;
    padding: 6px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: inherit;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .tabs button:hover:not(.active) {
    background: var(--hover, rgba(128, 128, 128, 0.12));
  }

  .tabs button.active {
    background: var(--row-selected, rgba(79, 140, 255, 0.12));
    color: var(--accent, #4f8cff);
    font-weight: 600;
  }

  .tabpanel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 2px 6px 2px 2px;
    overflow-y: auto;
    outline: none;
  }

  .tabpanel:focus-visible {
    outline: 2px solid var(--accent, #4f8cff);
    outline-offset: 2px;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 12px;
    flex-wrap: wrap;
  }

  .row.check {
    justify-content: flex-start;
  }

  .row.end {
    justify-content: flex-end;
    margin-top: 10px;
  }

  .row .label {
    min-width: 140px;
    color: var(--fg-dim, #6a6a6a);
  }

  .row select,
  .row input[type="number"],
  .row input[type="text"] {
    flex: 1;
    padding: 4px 6px;
    font: inherit;
    font-size: 12px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    border-radius: 4px;
    background: var(--surface, #ffffff);
    color: inherit;
  }

  .hint {
    margin: 0;
    font-size: 11px;
    color: var(--fg-dim, #6a6a6a);
  }

  .subheading {
    margin: 10px 0 4px;
    font-size: 12px;
    font-weight: 600;
    color: var(--fg-dim, #6a6a6a);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .empty {
    padding: 12px 0;
    color: var(--muted, #6a6a6a);
    font-size: 12px;
    text-align: center;
  }

  .muted {
    color: var(--fg-dim, #6a6a6a);
    font-family: var(--mono, ui-monospace, SFMono-Regular, Menlo, monospace);
    font-size: 11px;
    overflow-wrap: anywhere;
  }

  button {
    font-size: 12px;
    padding: 5px 12px;
    border-radius: 4px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.3));
    background: var(--surface-alt, rgba(0, 0, 0, 0.04));
    color: inherit;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.tiny {
    padding: 3px 8px;
    font-size: 11px;
  }

  button.danger {
    border-color: var(--error, #d95757);
    color: var(--error, #c24141);
  }

  button.danger:hover:not(:disabled) {
    background: rgba(217, 87, 87, 0.08);
  }

  .profile-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .profile-list li {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    border: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 4px;
    background: var(--surface-alt, rgba(0, 0, 0, 0.02));
  }

  .profile-name {
    flex: 1;
    font-weight: 500;
    font-size: 12px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Phase 24 — Security-metadata subsection. The header row carries
     the master toggle; nested toggles indent under it and only render
     when the master is on so a user opting out doesn't have to look at
     five irrelevant sub-checkboxes. */
  .meta-subsection {
    margin-top: 8px;
    padding: 8px 0 4px 0;
    border-top: 1px solid var(--border, rgba(127, 127, 127, 0.2));
  }
  .meta-subsection .section-header {
    font-weight: 600;
  }
  .meta-child {
    padding-left: 18px;
  }
  .motw-warning {
    color: var(--warning, #d97706);
    margin-left: 4px;
    font-size: 0.9em;
  }
</style>
