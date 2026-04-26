# §4 golden-path harness

Playwright suite that drives every checkbox in
`QualityAssuranceChecklist.md` §4 (`Manual UI golden path`) at the
frontend layer.

## How it works

The Tauri app's IPC layer is shimmed from the browser side. A
Playwright `addInitScript` injects a stand-in for
`window.__TAURI_INTERNALS__` before the Vite bundle boots. Tests
register per-command responders via the `tauri` fixture:

```ts
import { expect, test } from "./fixtures/test";

test("Drop Stack lights up after a drop", async ({ page, tauri }) => {
  await tauri.handle("start_copy", () => [42]);
  await page.goto("/");
  await tauri.emit("drop-received", { paths: ["/tmp/foo.bin"] });
  await expect(page.getByRole("dialog", { name: /staging/i })).toBeVisible();
});
```

This means **the Rust backend never runs** during a Playwright test.
Per-crate `cargo test -p <crate>` runs (`xtask qa-automate` covers
those) exercise the Rust side. This harness covers everything that
sits above the IPC boundary — clicks, modals, forms, drop targets,
i18n switches, error rendering, etc.

## Running

```bash
# from repo root
cd apps/copythat-ui
pnpm install                    # installs @playwright/test
pnpm exec playwright install    # downloads the Chromium runtime
pnpm test:e2e                   # full run
pnpm test:e2e:ui                # interactive UI mode
pnpm test:e2e:list              # list every test (no run)
```

The `webServer` block in `playwright.config.ts` boots `pnpm dev`
automatically; in local dev a developer's already-running Vite
server is reused.

Reports land in `target/playwright-report/` (HTML) and console
output uses Playwright's `list` reporter (`github` reporter in CI).

## Why a browser shim instead of `tauri-driver`?

The QA checklist's appendix calls for a "Playwright + tauri-driver
harness". In Tauri 2.x, the canonical end-to-end path actually
pairs **WebdriverIO** with `tauri-driver` — Playwright doesn't
speak the classic WebDriver protocol the bridge exposes. There are
ways to coerce Playwright onto WebDriver (selenium-grid, custom
adapters), but they're brittle and they don't give you Playwright's
locator / trace ergonomics.

The pragmatic split:

| Layer                          | Coverage path                                        |
| ------------------------------ | ---------------------------------------------------- |
| Rust engine + IPC dispatch     | `cargo test -p <crate>` (driven by `xtask qa-automate`) |
| Tauri shell ↔ engine wiring    | `cargo test -p copythat-ui` smoke tests              |
| Frontend UI golden path (§4)   | This Playwright suite                                |
| Real binary on a real desktop  | Manual / Computer-Use sessions before tagging        |

The frontend half is the layer that actually moves between
releases; the Rust side is largely covered by typed unit + smoke
tests already.

## Deferred: real-binary end-to-end

A future phase can promote a subset of these specs to run against
the real Tauri binary via `tauri-driver` + WebdriverIO. The work is:

1. Add `tauri-driver` install to `release.yml`'s test matrix.
2. Stand up a WebdriverIO config alongside this Playwright config.
3. Pick the dozen or so §4 checkboxes whose value depends on real
   IPC behavior (drag-drop wiring, mount visibility, OAuth flows)
   and port them. Most checkboxes — settings flips, modal flows,
   error rendering — stay in this Playwright suite where the IPC
   mock keeps them deterministic.

Don't port wholesale. The IPC mock is the right tool for ~90% of
the §4 list and a real-binary harness costs minutes per run plus
matrix breadth.

## File layout

```
e2e/
├── README.md                          (this file)
├── tsconfig.json                      (separate from src/ tsconfig)
├── fixtures/
│   ├── tauri-shim.ts                  (browser-side __TAURI_INTERNALS__)
│   └── test.ts                        (Playwright extends() wrapper)
├── qa-section-4-1-copy.spec.ts        (§4.1 — copy)
├── qa-section-4-2-move.spec.ts        (§4.2 — move)
├── qa-section-4-3-verify.spec.ts      (§4.3 — verify)
├── qa-section-4-4-secure-delete.spec.ts (§4.4 — secure delete)
├── qa-section-4-5-sync.spec.ts        (§4.5 — sync, Phase 25)
├── qa-section-4-6-cloud.spec.ts       (§4.6 — cloud, Phase 32)
├── qa-section-4-7-mount.spec.ts       (§4.7 — mount, Phase 33)
├── qa-section-4-8-audit.spec.ts       (§4.8 — audit log, Phase 34)
├── qa-section-4-9-encryption.spec.ts  (§4.9 — encryption + compression, Phase 35)
├── qa-section-4-10-cli.spec.ts        (§4.10 — CLI, Phase 36)
├── qa-section-4-11-mobile.spec.ts     (§4.11 — mobile companion, Phase 37)
├── qa-section-4-11a-phase37-followup.spec.ts (§4.11a — Phase 37 follow-up #2)
├── qa-section-4-11b-locale.spec.ts    (§4.11b — locale sync, Phase 38)
├── qa-section-4-11c-dedup.spec.ts     (§4.11c — destination dedup ladder, Phase 38)
├── qa-section-4-11d-partials.spec.ts  (§4.11d — Phase 8 partials, Phase 38-followup-3)
├── qa-section-4-11e-power.spec.ts     (§4.11e — real OS power probes, Phase 31b)
├── qa-section-4-11f-schedule.spec.ts  (§4.11f — scheduled jobs, Phase 14d)
└── qa-section-4-11g-queue-locked.spec.ts (§4.11g — queue-while-locked, Phase 14f)
```

Each spec mirrors its checklist subsection one-to-one: one
`test()` per checkbox. Empty checklist items become `test.fixme()`
stubs with the intended actions, IPC mocks, and assertions
documented in the body. Filling a stub means deleting the
`fixme` and writing the asserts — the harness around it is ready.

## Conventions

- **One test per checkbox.** Test name is the checkbox text,
  shortened if necessary. The spec file's `test.describe()` block
  names the QA subsection (`§4.1 Copy`).
- **Every spec resets the IPC mock between tests.** The fixture
  handles this; you never need an explicit `afterEach`.
- **Default IPC handlers cover the boot path.** `globals`,
  `list_jobs`, `get_settings`, and the i18n calls all return
  empty/idle states by default. Override only what your test
  changes.
- **Wait for invokes, not for arbitrary timeouts.**
  `tauri.waitForCall("start_copy")` is the right hook for "did the
  click reach the backend?" assertions.
- **`test.fixme(true, "<why>")` is preferred over `test.skip()`**
  for unimplemented checkboxes — fixme keeps the test in the
  report so the gap stays visible.

## Adding a new §4 subsection

When a new phase lands:

1. Append the subsection to `QualityAssuranceChecklist.md` §4.
2. Create `e2e/qa-section-4-<n>-<topic>.spec.ts` using the
   existing files as a template.
3. One `test.fixme()` per checkbox initially. Fill them in as the
   underlying flow stabilises.
4. Update this README's "File layout" table.
