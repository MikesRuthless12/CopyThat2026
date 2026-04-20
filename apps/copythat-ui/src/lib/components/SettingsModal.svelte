<!--
  Phase 11b — Settings modal (minimal).

  Houses the permanent home for the language switcher that Phase 11a
  parked in the header as a temporary affordance. Phase 12 will grow
  this modal into a full Settings window with Transfer, Shell,
  Secure-delete, and Advanced tabs plus TOML persistence and named
  profiles. The skeleton here — tab strip, tab content container,
  close button, locale store wiring — is the seam Phase 12 will
  extend; every new tab adds one `<nav>` button and one
  `<section data-tab="…">` block.

  Hot-swap without restart: changing the language dropdown fires
  `setLocale(code)` which pushes a new state into the locale store
  and repaints the webview (via `applyHtmlAttributes` — flips
  `<html lang>` and `<html dir>` for RTL locales). No Tauri IPC
  round-trip; the Rust-side `translations(code)` call is cheap.
-->
<script lang="ts">
  import Icon from "../icons/Icon.svelte";
  import { locale, setLocale, t } from "../i18n";
  import { closeSettings, settingsOpen } from "../stores";

  // One tab today (General). Phase 12 adds the other four. Keeping
  // the constant here rather than inlining in the template so the
  // Phase-12 diff is a one-line append.
  type TabId = "general";
  let activeTab: TabId = $state("general");

  // Endonym fallback table. Used when the runtime webview lacks a
  // full CLDR dataset for `Intl.DisplayNames` (happens on some older
  // WebKit / webview2 builds — they return the BCP-47 tag verbatim
  // rather than a localized name). Each value is the language's
  // endonym (its name in itself) so the user can still identify it.
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

  /// Render one BCP-47 code as a human label in `uiLocale`.
  /// When the UI language is French, `en` reads as "anglais"; when
  /// Japanese, "英語". Falls back to the endonym if the webview's
  /// `Intl.DisplayNames` returns the tag verbatim (older WebKit
  /// builds) or throws.
  function displayName(code: string, uiLocale: string): string {
    try {
      const dn = new Intl.DisplayNames([uiLocale], { type: "language" });
      const name = dn.of(code);
      if (name && name !== code) return name;
    } catch {
      // Some runtimes throw on unknown `uiLocale` arguments; fall
      // through to the endonym.
    }
    return ENDONYMS[code] ?? code;
  }

  /// Sort order: English pinned to the top (always the reference
  /// locale), everything else alphabetized by its display name in
  /// the current UI locale. `localeCompare` respects the active
  /// locale's collation, so French in Russian-UI sorts among
  /// Cyrillic entries rather than Latin ones.
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

  async function onLocaleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    await setLocale(target.value);
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      closeSettings();
    }
  }
</script>

{#if $settingsOpen}
  <div
    class="backdrop"
    role="presentation"
    onclick={closeSettings}
    onkeydown={onKeydown}
  >
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

      <div class="body">
        <!-- Tab strip. Phase 12 appends to this list. -->
        <div class="tabs" role="tablist" aria-label={t("settings-title")}>
          <button
            type="button"
            role="tab"
            aria-selected={activeTab === "general"}
            aria-controls="tab-general"
            class:active={activeTab === "general"}
            onclick={() => (activeTab = "general")}
          >
            {t("settings-tab-general")}
          </button>
        </div>

        <!-- General tab. Contains the language switcher today; Phase
             12 adds theme (Auto/Light/Dark), start-with-OS,
             single-instance, and minimize-to-tray toggles. -->
        {#if activeTab === "general"}
          <div
            id="tab-general"
            class="tabpanel"
            role="tabpanel"
            tabindex="0"
            aria-labelledby="tab-general-label"
          >
            <label class="row">
              <span class="label">{t("settings-section-language")}</span>
              <select
                value={$locale.code}
                onchange={onLocaleChange}
                aria-label={t("header-language-label")}
              >
                {#each orderedLocales as code (code)}
                  <option value={code}>{displayName(code, $locale.code)}</option>
                {/each}
              </select>
            </label>
            <p class="hint">{t("settings-phase-12-hint")}</p>
          </div>
        {/if}
      </div>
    </div>
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
    width: min(520px, 94vw);
    max-height: 82vh;
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

  .body {
    display: flex;
    gap: 14px;
    min-height: 160px;
  }

  .tabs {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 120px;
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
    gap: 10px;
    padding: 2px;
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
  }

  .row .label {
    min-width: 96px;
    color: var(--fg-dim, #6a6a6a);
  }

  .row select {
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
</style>
