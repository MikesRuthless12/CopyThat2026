<!--
  Phase 44.1f — Settings → Drive sanitize tab.

  Whole-drive sanitization via firmware (NVMe Sanitize Crypto Erase,
  OPAL Crypto Erase, ATA Secure Erase). The actual privileged
  command runs through a `SanitizeHelper` impl in Rust; this UI is
  the human-side gate.

  Spec contract: hammer the user with three confirmations before
  invoking. Confirmation 1 = checkbox. Confirmation 2 = checkbox.
  Confirmation 3 = type the drive's model name. The Run button is
  disabled until ALL THREE have flipped.

  Phase 44.1 first cut uses local component state. Tauri command
  bridges (`sanitize_capabilities`, `sanitize_run`,
  `sanitize_listen_progress`) land in Phase 44.2 — until then the
  Run button surfaces a "staged" toast. The mode picker is
  populated from the static SsdSanitizeMode list rather than the
  live capability probe; once the IPC lands, the picker filters
  to only modes the device reports.
-->
<script lang="ts">
  import { t } from "../i18n";
  import { pushToast } from "../stores";

  type Mode =
    | "nvme-format"
    | "nvme-sanitize-block"
    | "nvme-sanitize-crypto"
    | "ata-secure-erase"
    | "opal-crypto-erase";

  let devicePath = $state("");
  let modelTyped = $state("");
  let driveModel = $state("(unknown — capability probe pending IPC wiring)");
  let mode = $state<Mode>("nvme-sanitize-crypto");
  let confirm1 = $state(false);
  let confirm2 = $state(false);
  let progressPercent = $state<number | null>(null);
  let running = $state(false);

  // The third confirmation is "type the drive's model name to
  // proceed." It's enabled only when both checkboxes are flipped
  // AND the typed text matches the drive model the helper
  // reported. Today drive model is a placeholder; once the
  // capabilities IPC lands, this binds to live data.
  let modelMatches = $derived(
    modelTyped.trim().length > 0
      && driveModel !== "(unknown — capability probe pending IPC wiring)"
      && modelTyped.trim() === driveModel
  );
  let runReady = $derived(
    confirm1 && confirm2 && modelMatches && devicePath.trim().length > 0 && !running
  );

  const MODE_LABEL_KEYS: Record<Mode, string> = {
    "nvme-format": "sanitize-mode-nvme-format",
    "nvme-sanitize-block": "sanitize-mode-nvme-sanitize-block",
    "nvme-sanitize-crypto": "sanitize-mode-nvme-sanitize-crypto",
    "ata-secure-erase": "sanitize-mode-ata-secure-erase",
    "opal-crypto-erase": "sanitize-mode-opal-crypto-erase",
  };

  function onProbeCapabilities() {
    // Phase 44.1 — placeholder. Real impl invokes
    // `invoke("sanitize_capabilities", { device: devicePath })`
    // and binds `driveModel = result.model`.
    pushToast("info", "sanitize-action-staged");
  }

  function onRunSanitize() {
    if (!runReady) return;
    // Phase 44.1 — placeholder. Real impl invokes
    // `invoke("sanitize_run", { device: devicePath, mode })`
    // and listens to "sanitize-progress" / "sanitize-completed"
    // events to drive `progressPercent` + `running`.
    running = true;
    progressPercent = 0;
    pushToast("info", "sanitize-action-staged");
    // Auto-clear after 2s for the demo.
    setTimeout(() => {
      running = false;
      progressPercent = null;
    }, 2000);
  }
</script>

<section class="tab-body">
  <h3>{t("sanitize-heading")}</h3>
  <p class="hint">{t("sanitize-hint")}</p>

  <label class="field">
    <span>{t("sanitize-pick-device")}</span>
    <input
      type="text"
      placeholder="/dev/nvme0   |   \\.\\PhysicalDrive2   |   /dev/disk2"
      bind:value={devicePath}
    />
    <button type="button" class="secondary" onclick={onProbeCapabilities}>
      Probe capabilities
    </button>
  </label>

  <label class="field">
    <span>{t("sanitize-mode-label")}</span>
    <select bind:value={mode}>
      <option value="nvme-format">{t("sanitize-mode-nvme-format")}</option>
      <option value="nvme-sanitize-block">{t("sanitize-mode-nvme-sanitize-block")}</option>
      <option value="nvme-sanitize-crypto">{t("sanitize-mode-nvme-sanitize-crypto")}</option>
      <option value="ata-secure-erase">{t("sanitize-mode-ata-secure-erase")}</option>
      <option value="opal-crypto-erase">{t("sanitize-mode-opal-crypto-erase")}</option>
    </select>
  </label>

  <div class="confirm-block">
    <p class="warn">
      {#if devicePath.trim().length > 0}
        {t("sanitize-confirm-1", { device: devicePath })}
      {:else}
        {t("sanitize-confirm-1", { device: "(no device chosen)" })}
      {/if}
    </p>

    <label class="row">
      <input type="checkbox" bind:checked={confirm1} />
      <span>{t("sanitize-confirm-2")}</span>
    </label>

    <label class="row">
      <input type="checkbox" bind:checked={confirm2} />
      <span>I confirm I have a backup of any data I want to keep</span>
    </label>

    <label class="field">
      <span>{t("sanitize-confirm-3", { model: driveModel })}</span>
      <input type="text" bind:value={modelTyped} placeholder={driveModel} />
      {#if modelTyped.trim().length > 0 && !modelMatches}
        <small class="error">Drive model name does not match.</small>
      {/if}
    </label>
  </div>

  <button
    type="button"
    class="danger"
    onclick={onRunSanitize}
    disabled={!runReady}
  >
    Run sanitize
  </button>

  {#if running}
    <p class="status">
      {t("sanitize-running", { device: devicePath, mode: t(MODE_LABEL_KEYS[mode]) })}
    </p>
    {#if progressPercent !== null}
      <progress max="100" value={progressPercent}></progress>
      <small>{progressPercent}%</small>
    {/if}
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
  .hint {
    color: var(--muted, #888);
    font-size: 0.9em;
    margin: 0;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .field input,
  .field select {
    padding: 0.4rem 0.5rem;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .confirm-block {
    border: 1px solid var(--border, #c00);
    border-radius: 4px;
    padding: 0.75rem;
    background: var(--danger-bg, rgba(192, 0, 0, 0.04));
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .warn {
    color: var(--danger, #c00);
    font-weight: 600;
    margin: 0;
  }
  .error {
    color: var(--danger, #c00);
  }
  .status {
    color: var(--muted, #888);
    margin-top: 0.5rem;
  }
  button.danger {
    background: var(--danger, #c00);
    color: white;
  }
  button.danger:disabled {
    background: var(--muted, #888);
    color: white;
    opacity: 0.5;
  }
</style>
