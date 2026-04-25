<!--
  First-time pairing screen. The phone reaches this when there's no
  stored pair record in localStorage.

  Flow:
  1. User pastes the `cthat-pair://<peer>?sas=…&dpk=…` URL from the
     desktop's Settings → Mobile QR.
  2. PWA loads (or mints + persists) its 32-byte stable identity from
     `identity.ts`.
  3. PWA computes the four-emoji SAS via SHA-256(seed || desktop ||
     phone) and renders it.
  4. User compares the four glyphs against the desktop's Settings →
     Mobile panel. Tapping "Match" stores the pair record, hands the
     pubkey to App.svelte, and the dashboard takes over.

  The pubkey-as-identity pattern (instead of a real X25519 keypair)
  is sound here because the WebRTC data channel already encrypts /
  authenticates the transport via DTLS — the SAS protects against
  PeerJS-broker MITM, and the pubkey is the stable per-device tag
  the desktop matches against `MobileSettings::pairings`.
-->
<script lang="ts">
  import { computeSasEmoji, getOrMintPhoneIdentity } from "../identity";

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
    sasSeedB32: string;
    desktopPubkeyB32: string;
  } | null>(null);
  let sasEmoji = $state<string | null>(null);
  let phonePubkeyHex = $state<string | null>(null);
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

  /**
   * Decode a Crockford-base32 string the desktop encoded with
   * the `base32` crate. The desktop emits uppercase, no padding;
   * we accept any case + the standard Crockford symbol set.
   */
  function decodeCrockford(input: string): Uint8Array {
    const ALPHABET = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    const trimmed = input.trim().toUpperCase().replace(/[OIL]/g, (c) =>
      c === "O" ? "0" : "1",
    );
    let bits = 0;
    let value = 0;
    const out: number[] = [];
    for (const ch of trimmed) {
      const idx = ALPHABET.indexOf(ch);
      if (idx < 0) {
        throw new Error(`bad Crockford-base32 char: ${ch}`);
      }
      value = (value << 5) | idx;
      bits += 5;
      if (bits >= 8) {
        bits -= 8;
        out.push((value >> bits) & 0xff);
      }
    }
    return new Uint8Array(out);
  }

  function bytesToHex(bytes: Uint8Array): string {
    let s = "";
    for (let i = 0; i < bytes.length; i++) {
      s += bytes[i].toString(16).padStart(2, "0");
    }
    return s;
  }

  async function tryParse() {
    error = null;
    sasEmoji = null;
    const trimmed = pairUrl.trim();
    const match = trimmed.match(
      /^cthat-pair:\/\/([^?]+)\?(.+)$/,
    );
    if (!match) {
      error = "Pairing URL must look like cthat-pair://<peer-id>?sas=…&dpk=…";
      return;
    }
    const peerId = match[1];
    const queryRaw = match[2];
    let sasSeedB32: string | null = null;
    let desktopPubkeyB32: string | null = null;
    let sawSas = 0;
    let sawDpk = 0;
    for (const kv of queryRaw.split("&")) {
      const [k, v] = kv.split("=", 2);
      if (k === "sas") {
        sawSas++;
        sasSeedB32 = v;
      } else if (k === "dpk") {
        sawDpk++;
        desktopPubkeyB32 = v;
      }
    }
    if (sawSas !== 1 || sawDpk !== 1 || !sasSeedB32 || !desktopPubkeyB32) {
      error = "Pairing URL must carry exactly one `sas` and one `dpk` parameter.";
      return;
    }
    let seedHex: string;
    let dpkHex: string;
    try {
      seedHex = bytesToHex(decodeCrockford(sasSeedB32));
      dpkHex = bytesToHex(decodeCrockford(desktopPubkeyB32));
    } catch (e) {
      error = `Pairing URL contained malformed base32: ${e}`;
      return;
    }
    if (seedHex.length !== 64 || dpkHex.length !== 64) {
      error = "Pairing URL has wrong-sized seed or desktop key.";
      return;
    }
    let phoneHex: string;
    try {
      phoneHex = getOrMintPhoneIdentity();
    } catch (e) {
      error = `Phone identity unavailable: ${e}`;
      return;
    }
    let emoji: string;
    try {
      emoji = await computeSasEmoji(seedHex, dpkHex, phoneHex);
    } catch (e) {
      error = `SAS computation failed: ${e}`;
      return;
    }
    parsing = {
      peerId,
      sasSeedB32,
      desktopPubkeyB32,
    };
    phonePubkeyHex = phoneHex;
    sasEmoji = emoji;
  }

  function confirmPair() {
    if (!parsing || !phonePubkeyHex) return;
    onPaired({
      desktopPeerId: parsing.peerId,
      deviceLabel,
      phonePubkeyHex,
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
    <input bind:value={pairUrl} placeholder="cthat-pair://<peer-id>?sas=…&dpk=…" />
  </label>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if !parsing}
    <button type="button" onclick={tryParse} disabled={!pairUrl.trim()}>
      Continue
    </button>
  {:else}
    <div class="sas-block">
      <p class="muted">Compare these four glyphs against the desktop's Settings → Mobile panel:</p>
      <div class="sas-emoji">{sasEmoji ?? "…"}</div>
    </div>
    <p class="muted">
      Only tap "Match" when the four glyphs are identical on both
      screens. A mismatch means the pairing is being intercepted —
      tap "Cancel" and start over.
    </p>
    <div class="row">
      <button type="button" onclick={confirmPair}>Match</button>
      <button type="button" class="secondary" onclick={() => { parsing = null; sasEmoji = null; }}>
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
  .sas-block {
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    padding: 1rem;
    text-align: center;
    margin: 0.5rem 0;
  }
  .sas-emoji {
    font-size: 2.5rem;
    letter-spacing: 0.5rem;
    margin-top: 0.25rem;
  }
</style>
