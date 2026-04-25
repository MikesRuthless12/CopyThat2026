<!--
  Shown when the desktop is offline. Per the design contract, the
  PWA can't drive any control surface without an active data
  channel — this is the only screen the user can interact with
  in that state.
-->
<script lang="ts">
  let { message, onRetry }: { message: string; onRetry: () => void } =
    $props();
</script>

<div class="panel">
  <h2>Desktop unreachable</h2>
  <p class="muted">{message}</p>
  <p>
    Open Copy That on your desktop or laptop and confirm Settings →
    Mobile shows the toggle as <em>active</em>, then retry below.
  </p>
  <button type="button" onclick={onRetry}>Retry connection</button>
</div>

<style>
  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
  }
</style>
