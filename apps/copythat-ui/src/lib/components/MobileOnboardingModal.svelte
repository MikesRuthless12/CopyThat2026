<!--
  Phase 37 follow-up #2 — first-launch mobile-companion onboarding
  modal.

  Shown once, on the first launch where:
    - settings.mobile?.pairEnabled is false (or unset)
    - settings.mobile.pairings is empty
    - settings.general.mobileOnboardingDismissed is not yet true

  Renders the desktop icon, the install-QR PNG (the QR scans the
  phone's camera straight to the deployed PWA URL — phone tap "Add
  to Home Screen" to install), and two actions:
    - "I have the app, pair now" → opens Settings → Mobile.
    - "Maybe later" → flips mobileOnboardingDismissed so the modal
       doesn't reappear.
-->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type Props = {
    open: boolean;
    onClose: () => void;
    onOpenSettings: () => void;
  };

  let { open, onClose, onOpenSettings }: Props = $props();

  let qrPngBase64 = $state<string | null>(null);
  let pwaUrl = $state<string | null>(null);
  let busy = $state(false);

  onMount(async () => {
    await loadQr();
  });

  async function loadQr() {
    try {
      const dto = (await invoke("mobile_onboarding_qr", {
        pwaUrl: null,
      })) as { url: string; qrPngBase64: string };
      qrPngBase64 = dto.qrPngBase64;
      pwaUrl = dto.url;
    } catch (e) {
      console.error("mobile_onboarding_qr", e);
    }
  }

  async function dismiss() {
    if (busy) return;
    busy = true;
    try {
      await invoke("mobile_onboarding_dismiss");
    } finally {
      busy = false;
      onClose();
    }
  }

  async function openSettings() {
    if (busy) return;
    busy = true;
    try {
      await invoke("mobile_onboarding_dismiss");
    } finally {
      busy = false;
      onOpenSettings();
      onClose();
    }
  }
</script>

{#if open}
  <div
    class="backdrop"
    role="dialog"
    aria-modal="true"
    aria-labelledby="mobile-onboarding-title"
    tabindex="-1"
    onkeydown={(e) => {
      if (e.key === "Escape") dismiss();
    }}
  >
    <div class="modal">
      <img src="/icons/icon-128.png" alt="Copy That" class="logo" />
      <h2 id="mobile-onboarding-title">Get the Copy That mobile companion</h2>
      <p class="hint">
        Drive your desktop's copy / move / sync / secure-delete jobs
        from your phone over a private WebRTC link. Scan the QR
        below with your phone's camera to open the install URL in
        your browser, then tap "Add to Home Screen" — no App Store
        needed.
      </p>
      {#if qrPngBase64}
        <img
          class="qr"
          src={`data:image/png;base64,${qrPngBase64}`}
          alt="Install QR"
        />
        <p class="addr">{pwaUrl}</p>
      {:else}
        <p class="hint muted">Loading QR…</p>
      {/if}
      <div class="actions">
        <button
          type="button"
          class="primary"
          onclick={openSettings}
          disabled={busy}
        >
          I have the app, pair now
        </button>
        <button
          type="button"
          class="secondary"
          onclick={dismiss}
          disabled={busy}
        >
          Maybe later
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }
  .modal {
    background: var(--color-panel, #1e293b);
    color: var(--color-fg, #f1f5f9);
    padding: 1.5rem;
    border-radius: 0.75rem;
    max-width: 420px;
    width: 100%;
    text-align: center;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
  }
  .logo {
    width: 64px;
    height: 64px;
    margin-bottom: 0.75rem;
    border-radius: 12px;
  }
  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
  }
  .hint {
    margin: 0.5rem 0;
    font-size: 0.95rem;
    line-height: 1.4;
    opacity: 0.85;
  }
  .qr {
    margin: 1rem auto;
    width: 220px;
    height: 220px;
    image-rendering: pixelated;
    border: 4px solid var(--color-border, #475569);
    border-radius: 8px;
  }
  .addr {
    font-family: var(--font-mono, monospace);
    font-size: 0.75rem;
    opacity: 0.6;
    word-break: break-all;
    margin: 0.25rem 0;
  }
  .actions {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  .actions button {
    width: 100%;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    border: 0;
    cursor: pointer;
    font-size: 1rem;
  }
  .actions .primary {
    background: var(--color-accent, #3b82f6);
    color: white;
  }
  .actions .secondary {
    background: transparent;
    color: var(--color-fg, #f1f5f9);
    border: 1px solid var(--color-border, #475569);
  }
</style>
