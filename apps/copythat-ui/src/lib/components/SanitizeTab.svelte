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
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import { t } from "../i18n";
  import { pushToast } from "../stores";

  type Mode =
    | "nvme-format"
    | "nvme-sanitize-block"
    | "nvme-sanitize-crypto"
    | "ata-secure-erase"
    | "opal-crypto-erase";

  type CapabilitiesDto = {
    trim: boolean;
    modes: Mode[];
    bus: string;
    model: string;
    hasGuaranteedCryptoErase: boolean;
  };

  type ReportDto = {
    device: string;
    mode: string;
    durationMs: number;
  };

  type ProgressEvt = {
    device: string;
    mode: string;
    percent: number;
  };

  let devicePath = $state("");
  let modelTyped = $state("");
  let driveModel = $state("(probe to populate)");
  let availableDevices = $state<string[]>([]);
  let availableModes = $state<Mode[]>([]);
  let busLabel = $state<string>("");
  let mode = $state<Mode>("nvme-sanitize-crypto");
  let confirm1 = $state(false);
  let confirm2 = $state(false);
  let progressPercent = $state<number | null>(null);
  let running = $state(false);

  // The third confirmation requires typing the drive's model name
  // exactly. Defense in depth: the Tauri command also enforces this
  // on the Rust side so a hostile frontend cannot bypass it.
  let modelMatches = $derived(
    modelTyped.trim().length > 0
      && driveModel !== "(probe to populate)"
      && modelTyped.trim() === driveModel
  );
  let runReady = $derived(
    confirm1
      && confirm2
      && modelMatches
      && devicePath.trim().length > 0
      && !running
      && availableModes.includes(mode)
  );

  const MODE_LABEL_KEYS: Record<Mode, string> = {
    "nvme-format": "sanitize-mode-nvme-format",
    "nvme-sanitize-block": "sanitize-mode-nvme-sanitize-block",
    "nvme-sanitize-crypto": "sanitize-mode-nvme-sanitize-crypto",
    "ata-secure-erase": "sanitize-mode-ata-secure-erase",
    "opal-crypto-erase": "sanitize-mode-opal-crypto-erase",
  };

  // Track active event listeners so we can clean up on unmount.
  let unlistenProgress: UnlistenFn | null = null;
  let unlistenCompleted: UnlistenFn | null = null;
  let unlistenFailed: UnlistenFn | null = null;

  onMount(async () => {
    try {
      availableDevices = await invoke<string[]>("sanitize_list_devices");
    } catch (e) {
      // Empty list is fine — the user types the path manually.
      availableDevices = [];
    }
    // Phase 44.2 post-review (C1) — listener stale-event guard.
    // SettingsModal mounts/unmounts SanitizeTab on tab switches; a
    // sanitize that completes between unmount and re-mount can
    // deliver its event to a fresh listener that has no in-flight
    // run. Filter on `evt.payload.device === devicePath` AND
    // `running` so stale-but-matching events from a prior run
    // can't trigger a misleading toast.
    unlistenProgress = await listen<ProgressEvt>("sanitize-progress", (evt) => {
      if (running && evt.payload.device === devicePath.trim()) {
        progressPercent = evt.payload.percent;
      }
    });
    unlistenCompleted = await listen<ReportDto>("sanitize-completed", (evt) => {
      if (running && evt.payload.device === devicePath.trim()) {
        running = false;
        progressPercent = 100;
        pushToast("success", `sanitize completed (${evt.payload.mode})`);
      }
    });
    unlistenFailed = await listen<string>("sanitize-failed", (evt) => {
      if (running) {
        running = false;
        progressPercent = null;
        pushToast("error", evt.payload || "sanitize failed");
      }
    });
  });

  onDestroy(() => {
    unlistenProgress?.();
    unlistenCompleted?.();
    unlistenFailed?.();
  });

  async function onProbeCapabilities() {
    if (!devicePath.trim()) return;
    try {
      const caps = await invoke<CapabilitiesDto>("sanitize_capabilities_cmd", {
        device: devicePath.trim(),
      });
      driveModel = caps.model || "(unknown)";
      availableModes = caps.modes;
      busLabel = caps.bus;
      // Snap mode to the first available if the current selection
      // isn't supported.
      if (!caps.modes.includes(mode) && caps.modes.length > 0) {
        mode = caps.modes[0];
      }
    } catch (e) {
      pushToast("error", String(e));
    }
  }

  async function onRunSanitize() {
    if (!runReady) return;
    running = true;
    progressPercent = 0;
    try {
      await invoke<ReportDto>("sanitize_run", {
        device: devicePath.trim(),
        mode,
        modelTyped: modelTyped.trim(),
      });
      // The "sanitize-completed" event listener flips `running` to
      // false; the awaited `invoke` returning OK is the same signal.
      running = false;
    } catch (e) {
      running = false;
      progressPercent = null;
      pushToast("error", String(e));
    }
  }

  async function onFreeSpaceTrim() {
    if (!devicePath.trim()) return;
    try {
      await invoke<ReportDto>("sanitize_free_space_trim", {
        device: devicePath.trim(),
      });
      pushToast("success", "sanitize-completed");
    } catch (e) {
      pushToast("error", String(e));
    }
  }
</script>

<section class="tab-body">
  <h3>{t("sanitize-heading")}</h3>
  <p class="hint">{t("sanitize-hint")}</p>

  <label class="field">
    <span>{t("sanitize-pick-device")}</span>
    {#if availableDevices.length > 0}
      <select bind:value={devicePath}>
        <option value="">— pick a device —</option>
        {#each availableDevices as dev (dev)}
          <option value={dev}>{dev}</option>
        {/each}
      </select>
    {:else}
      <input
        type="text"
        placeholder="/dev/nvme0   |   \\.\\PhysicalDrive2   |   /dev/disk2"
        bind:value={devicePath}
      />
    {/if}
    <button type="button" class="secondary" onclick={onProbeCapabilities}>
      Probe capabilities
    </button>
    {#if busLabel}
      <small class="hint">Bus: {busLabel} · Model: {driveModel}</small>
    {/if}
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

  <hr />

  <h4>Free-space TRIM</h4>
  <p class="hint">
    macOS-only in this build. TRIMs the unallocated regions on a flash
    drive without touching the live filesystem. Linux + Windows return
    NotImplemented (see helper docstring).
  </p>
  <button
    type="button"
    class="secondary"
    onclick={onFreeSpaceTrim}
    disabled={!devicePath.trim() || running}
  >
    Run free-space TRIM
  </button>
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
