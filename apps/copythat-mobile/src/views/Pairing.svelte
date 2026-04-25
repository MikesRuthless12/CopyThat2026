<!--
  First-time pairing screen. The phone reaches this when there's no
  stored pair record in localStorage.

  Flow:
  1. User parses the QR they scanned with the system camera (or
     pastes the cthat-pair:// URL into the text field).
  2. Phone derives its own X25519 keypair (long-term, persisted in
     localStorage); displays the SAS-emoji that the desktop's
     Settings → Mobile panel will also display.
  3. User confirms the four glyphs match → phone records the pair
     record in localStorage and bubbles it up to App.svelte.

  The actual SAS-emoji derivation requires X25519 keys + a matching
  client-side crypto library; the wire-up here lays out the UX +
  the StoredPair shape. The cryptography itself ships in the
  follow-up alongside a `@noble/curves`-based key generator.
-->
<script lang="ts">
  type StoredPair = {
    desktopPeerId: string;
    deviceLabel: string;
    phonePubkeyHex: string;
    pairedAt: number;
  };

  let { onPaired }: { onPaired: (pair: StoredPair) => void } = $props();

  let pairUrl = $state("");
  let deviceLabel = $state(autoLabel());
  let parsing = $state<{
    peerId: string;
    sasSeed: string;
  } | null>(null);
  let error = $state<string | null>(null);

  function autoLabel(): string {
    if (typeof navigator !== "undefined") {
      const ua = navigator.userAgent;
      if (/iphone/i.test(ua)) return "iPhone";
      if (/ipad/i.test(ua)) return "iPad";
      if (/android/i.test(ua)) return "Android phone";
    }
    return "Phone";
  }

  function tryParse() {
    error = null;
    const trimmed = pairUrl.trim();
    const match = trimmed.match(/^cthat-pair:\/\/([^?]+)\?sas=([^&]+)/);
    if (!match) {
      error = "Pairing URL must look like cthat-pair://<peer-id>?sas=…";
      return;
    }
    parsing = { peerId: match[1], sasSeed: match[2] };
  }

  function confirmPair() {
    if (!parsing) return;
    onPaired({
      desktopPeerId: parsing.peerId,
      deviceLabel,
      // Stub pubkey — real one arrives with the noble-curves wiring
      // in the follow-up. Stored verbatim so the desktop can match
      // against MobileSettings::pairings on subsequent sessions.
      phonePubkeyHex: "00".repeat(32),
      pairedAt: Date.now(),
    });
  }
</script>

<div class="panel">
  <h2>Pair with desktop</h2>
  <p class="muted">
    Open Copy That on your desktop, click <em>Settings → Mobile → Start
    pairing</em>, and scan the QR with your camera. Then paste or scan
    the resulting URL here.
  </p>

  <label class="col">
    <span class="muted">Device label (shown on desktop)</span>
    <input bind:value={deviceLabel} />
  </label>

  <label class="col">
    <span class="muted">Pairing URL</span>
    <input bind:value={pairUrl} placeholder="cthat-pair://<peer-id>?sas=…" />
  </label>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if !parsing}
    <button type="button" onclick={tryParse} disabled={!pairUrl.trim()}>
      Continue
    </button>
  {:else}
    <p class="muted">
      Confirm the four-emoji code on your desktop matches the one
      shown after this connection establishes. If they don't match,
      tap "Cancel" and try again — someone may be impersonating
      your desktop.
    </p>
    <div class="row">
      <button type="button" onclick={confirmPair}>Connect</button>
      <button type="button" class="secondary" onclick={() => (parsing = null)}>
        Cancel
      </button>
    </div>
  {/if}
</div>

<style>
  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
  }
  input {
    background: var(--bg);
    color: var(--fg);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    padding: 0.5rem 0.75rem;
    font-size: 1rem;
    font-family: var(--font-system);
    width: 100%;
  }
</style>
