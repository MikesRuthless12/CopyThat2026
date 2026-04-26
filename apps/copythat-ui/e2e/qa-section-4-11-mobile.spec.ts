/**
 * §4.11 Mobile companion (Phase 37) — Manual UI golden path.
 *
 * Most of §4.11 lives on the PWA host, not the desktop Tauri shell.
 * Tests here assert the **desktop side** of each flow:
 *  - Onboarding modal renders with the right install QR.
 *  - Settings → Mobile pairing flow surfaces the SAS emojis.
 *  - Pause / resume / cancel events from the PWA arrive at the
 *    desktop and update the JobRow state.
 *
 * The PWA-side checkboxes (install QR scan, "Add to Home Screen",
 * etc.) need a real phone + browser. Document them as deferred —
 * a Computer Use session per release is the realistic path.
 */

import { test } from "./fixtures/test";

test.describe("§4.11 Mobile companion (Phase 37) — desktop side", () => {
  test.fixme(
    "First launch shows onboarding modal with PWA install QR",
    async ({ page: _page, tauri: _tauri }) => {
      // Override `get_settings` so `mobileOnboardingDismissed = false`
      // and `mobile.pairings = []`. Reload page. Assert
      // MobileOnboardingModal is visible and its <img> has a
      // `src` shaped like a QR data-url. Click "Maybe later" →
      // assert `update_settings` invoked with
      // `general.mobileOnboardingDismissed = true`.
    },
  );

  test.fixme(
    "Pairing flow → desktop Settings → Mobile shows QR + matching SAS emojis",
    async ({ page: _page, tauri: _tauri }) => {
      // Open Settings → Mobile tab. Click "Start pairing" →
      // mock `mobile_pairing_start` to return
      // `{ qr: "data:image/png;base64,...", sas: ["🐱","🐶","🦊","🐻"] }`.
      // Assert the QR + the four emojis render. Click "Confirm"
      // → assert `mobile_pairing_confirm` invoked with the
      // session id surfaced earlier.
    },
  );

  test.fixme(
    "PWA → Pause invokes pause_job; desktop reflects state",
    async ({ page: _page, tauri: _tauri }) => {
      // The PWA → desktop control surface is a `mobile-control`
      // event the desktop translates to a local IPC. Emit
      // `mobile-control { id, action: "pause" }` → assert the
      // JobRow visibly switches to paused state. Repeat for
      // "resume" and "cancel".
    },
  );

  test.fixme(
    "PWA Collisions panel → tap 'Overwrite all' → tree completes under that policy",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `mobile-control { action: "collision-resolve-all",
      // resolution: "overwrite" }`. Assert all queued
      // CollisionPrompts in the desktop UI receive
      // `resolve_collision` with overwrite + applyToAll = true.
    },
  );

  test.fixme(
    "PWA History → Re-run fires a new desktop job",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `mobile-control { action: "history-rerun", rowId }`.
      // Assert `history_rerun` invoked with the row id; assert
      // a new JobRow appears in the desktop list.
    },
  );

  test.fixme(
    "Kill desktop while PWA is connected → PWA shows reachability error",
    async ({ page: _page, tauri: _tauri }) => {
      // Desktop-only assertion: emit `mobile-disconnect
      // { peerId }` (reverse direction). Assert the Mobile
      // settings tab shows "phone offline" badge for the named
      // peer. The PWA-side reachability flip is browser-side,
      // covered by the deferred Computer Use sweep.
    },
  );

  // PWA-only checkboxes — defer to manual / Computer Use:
  //   - "Scan QR with iPhone → Safari opens the PWA"
  //   - "Add to Home Screen appears → installed icon matches"
  //   - "Open installed PWA → 'Pair with desktop'"
  //   - "PWA Home shows live globals while desktop runs a copy"
  //   - "PWA Pause / Resume / Cancel buttons drive desktop"
  //   - "PWA Exit button cleanly disconnects"
});
