<!--
  Copy That mobile companion — root component.

  States:
  - "unpaired" — show pair-from-QR screen.
  - "connecting" — PeerJS handshake in flight.
  - "unreachable" — desktop offline; offer Retry.
  - "connected" — Dashboard (stats + jobs + history + exit).
-->
<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import Dashboard from "./views/Dashboard.svelte";
  import Pairing from "./views/Pairing.svelte";
  import Unreachable from "./views/Unreachable.svelte";
  import { PeerLink, type PeerStatus } from "./peer";

  type StoredPair = {
    desktopPeerId: string;
    deviceLabel: string;
    phonePubkeyHex: string;
    pairedAt: number;
  };

  const STORAGE_KEY = "copythat.pair.v1";

  let link = new PeerLink();
  let status = $state<PeerStatus>({ kind: "idle" });
  let pair = $state<StoredPair | null>(loadStoredPair());
  /// Set when the desktop emits a `server_shutting_down` event
  /// just before exiting. Surfaces the explicit "Desktop exited
  /// — reconnect when Copy That is running again" message instead
  /// of the generic disconnect copy.
  let shutdownReason = $state<string | null>(null);

  $effect(() => {
    const off = link.onStatus((s) => {
      status = s;
    });
    return () => {
      off();
    };
  });

  $effect(() => {
    const off = link.onEvent((evt) => {
      if (evt.kind === "server_shutting_down") {
        shutdownReason = evt.reason || "Desktop is exiting.";
      }
    });
    return () => {
      off();
    };
  });

  onMount(() => {
    if (pair) {
      link.connect(pair.desktopPeerId);
    }
  });

  onDestroy(() => {
    void link.disconnect();
  });

  function loadStoredPair(): StoredPair | null {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (!raw) return null;
      return JSON.parse(raw) as StoredPair;
    } catch {
      return null;
    }
  }

  function persistPair(value: StoredPair | null): void {
    if (value === null) {
      localStorage.removeItem(STORAGE_KEY);
    } else {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
    }
  }

  function handlePaired(stored: StoredPair) {
    pair = stored;
    persistPair(stored);
    link.connect(stored.desktopPeerId);
  }

  async function handleExit() {
    await link.disconnect();
    pair = null;
    persistPair(null);
  }

  function retryConnection() {
    shutdownReason = null;
    if (pair) {
      link.connect(pair.desktopPeerId);
    }
  }
</script>

<header class="header">
  <img src="/icons/icon-128.png" alt="Copy That" class="logo" />
  <span class="title">Copy That</span>
  {#if pair}
    <button type="button" class="secondary exit" onclick={handleExit}>
      Exit
    </button>
  {/if}
</header>

<main>
  {#if !pair}
    <Pairing onPaired={handlePaired} />
  {:else if status.kind === "connecting" || status.kind === "idle"}
    <p class="muted">Connecting to {pair.desktopPeerId}…</p>
  {:else if status.kind === "connected"}
    <Dashboard {link} />
  {:else if status.kind === "error" || status.kind === "disconnected"}
    <Unreachable
      message={shutdownReason
        ? `Desktop exited (${shutdownReason}). Reconnect to your Copy That desktop app to resume control.`
        : status.kind === "error"
          ? status.message
          : "Disconnected from desktop."}
      onRetry={retryConnection}
    />
  {/if}
</main>

<style>
  .header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border);
    margin-bottom: 1rem;
  }
  .logo {
    width: 32px;
    height: 32px;
    border-radius: 6px;
  }
  .title {
    font-size: 1.1rem;
    font-weight: 600;
    flex: 1;
  }
  .exit {
    font-size: 0.85rem;
    padding: 0.4rem 0.8rem;
  }
  main {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
