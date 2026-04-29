<!--
  Phase 43 — Settings → Provenance tab.

  Surfaces the four controls the spec calls out:
  - "Manage signing keys" (generate / import / export)
  - "Enable provenance manifests by default" toggle
  - "Default TSA URL" input
  - "Show manifest after each job" toggle

  This Phase 43 first cut uses local component state. Persistence
  to disk + republishing to the engine flows through SettingsDto in
  a follow-up — bumping that DTO touches every Tauri command bridge
  and is intentionally deferred so this phase ships discoverable UI
  without a schema migration.

  When the user clicks Generate / Import / Export, the corresponding
  Tauri command (`provenance_keygen` / `provenance_import_key` /
  `provenance_export_public_key`) will be invoked once they land.
  Today the buttons are wired to a no-op + toast so the strings
  exercise on every locale audit.
-->
<script lang="ts">
  import { t } from "../i18n";
  import { pushToast } from "../stores";

  let enableByDefault = $state(false);
  let showAfterJob = $state(true);
  let tsaUrl = $state("");
  let publicKeyFingerprint = $state<string | null>(null);

  function onGenerateKey() {
    // Phase 43 post-review fix — toast was previously
    // "provenance-job-completed-title" ("Provenance manifest saved")
    // which lied about what just happened. Use the dedicated
    // "staged for IPC follow-up" key so users get an honest message
    // until the Tauri command bridge lands.
    publicKeyFingerprint = "ed25519:pending-tauri-wiring";
    pushToast("info", "provenance-action-staged");
  }

  function onImportKey() {
    pushToast("info", "provenance-action-staged");
  }

  function onExportKey() {
    pushToast("info", "provenance-action-staged");
  }
</script>

<section class="tab-body">
  <h3>{t("provenance-settings-heading")}</h3>
  <p class="hint">{t("provenance-settings-hint")}</p>

  <label class="row">
    <input type="checkbox" bind:checked={enableByDefault} />
    <span>{t("provenance-settings-enable-default")}</span>
  </label>

  <label class="row">
    <input type="checkbox" bind:checked={showAfterJob} />
    <span>{t("provenance-settings-show-after-job")}</span>
  </label>

  <label class="field">
    <span>{t("provenance-settings-tsa-url-label")}</span>
    <input
      type="url"
      placeholder="https://freetsa.org/tsr"
      bind:value={tsaUrl}
    />
    <small class="hint">{t("provenance-settings-tsa-url-hint")}</small>
  </label>

  <h4>{t("provenance-settings-keys-heading")}</h4>
  <div class="key-actions">
    <button type="button" class="secondary" onclick={onGenerateKey}>
      {t("provenance-settings-keys-generate")}
    </button>
    <button type="button" class="secondary" onclick={onImportKey}>
      {t("provenance-settings-keys-import")}
    </button>
    <button
      type="button"
      class="secondary"
      onclick={onExportKey}
      disabled={!publicKeyFingerprint}
    >
      {t("provenance-settings-keys-export")}
    </button>
  </div>
  {#if publicKeyFingerprint}
    <p class="fingerprint">{publicKeyFingerprint}</p>
  {/if}
</section>

<style>
  .tab-body {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  h3 {
    margin: 0 0 0.25rem 0;
  }
  h4 {
    margin: 1rem 0 0.25rem 0;
  }
  .hint {
    color: var(--muted, #888);
    font-size: 0.9em;
    margin: 0;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .field input {
    padding: 0.4rem 0.5rem;
  }
  .key-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .fingerprint {
    font-family: var(--mono, monospace);
    font-size: 0.85em;
    color: var(--muted, #888);
    margin: 0;
  }
</style>
