<main>
  <h1>Copy That 2026</h1>
</main>

<style>
  :global(body) {
    margin: 0;
    background: var(--bg, #fafafa);
    color: var(--fg, #1f1f1f);
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      Roboto,
      sans-serif;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      --bg: #1a1a1a;
      --fg: #f0f0f0;
    }
  }

  main {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
  }

  h1 {
    margin: 0;
    font-weight: 500;
    letter-spacing: -0.02em;
  }
</style>
