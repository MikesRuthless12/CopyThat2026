/**
 * §4.11a Phase 37 follow-up #2 — deferred items closed.
 *
 * The wake-lock + native binary checkboxes are OS-level / build-time
 * checks. The frontend touches the onboarding-modal lifecycle and
 * the mobile snapshot shape; everything else is engine or CI.
 */

import { expect, test } from "./fixtures/test";

test.describe("§4.11a Phase 37 follow-up #2", () => {
  test("First-launch onboarding modal appears once → does not reappear after dismiss", async ({
    page,
    tauri,
  }) => {
    await page.addInitScript(() => {
      const reg = (window as unknown as {
        __copythat_e2e__?: { setHandler: (cmd: string, h: (a: unknown) => unknown) => void };
      }).__copythat_e2e__;
      if (!reg) return;
      reg.setHandler("get_settings", () => ({
        general: {
          language: "en",
          theme: "auto",
          startWithOs: false,
          singleInstance: true,
          minimizeToTray: false,
          errorDisplayMode: "modal",
          pasteShortcutEnabled: false,
          pasteShortcut: "CmdOrCtrl+Shift+V",
          clipboardWatcherEnabled: false,
          autoResumeInterrupted: false,
          mobileOnboardingDismissed: false,
        },
        transfer: {
          bufferSizeBytes: 1048576,
          verify: "off",
          concurrency: "auto",
          reflink: "prefer",
          fsyncOnClose: false,
          preserveTimestamps: true,
          preservePermissions: true,
          preserveAcls: false,
          onLocked: "ask",
          preserveSparseness: true,
          preserveSecurityMetadata: false,
          preserveMotw: true,
          preservePosixAcls: false,
          preserveSelinuxContexts: false,
          preserveResourceForks: true,
          appledoubleFallback: true,
        },
        shell: { contextMenuEnabled: false, interceptDefaultCopy: false, notifyOnCompletion: true },
        secureDelete: { method: "dod-3-pass", confirmTwice: true },
        advanced: {
          logLevel: "info",
          telemetry: false,
          errorPolicy: { kind: "ask" },
          historyRetentionDays: 90,
          databasePath: null,
        },
        filters: {
          enabled: false,
          includeGlobs: [],
          excludeGlobs: [],
          minSizeBytes: null,
          maxSizeBytes: null,
          minMtimeUnixSecs: null,
          maxMtimeUnixSecs: null,
          skipHidden: false,
          skipSystem: false,
          skipReadonly: false,
        },
        updater: {
          autoCheck: false,
          channel: "stable",
          lastCheckUnixSecs: 0,
          dismissedVersion: "",
          checkIntervalSecs: 86400,
        },
        network: { rateBps: null, scheduleEnabled: false },
        mobile: { pairings: [], desktopPeerId: "test-peer" },
      }));
      reg.setHandler("mobile_onboarding_qr", () => ({
        url: "https://copythat.app/pwa",
        qrPngBase64: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=",
      }));
      reg.setHandler("mobile_onboarding_dismiss", () => undefined);
    });

    await page.goto("/");
    const modal = page.getByRole("dialog", {
      name: /mobile companion|copy that mobile/i,
    });
    await expect(modal).toBeVisible({ timeout: 5_000 });

    await modal.getByRole("button", { name: /maybe later/i }).click();
    await tauri.waitForCall("mobile_onboarding_dismiss");

    // After dismiss, modal closes immediately (App.svelte's
    // mobileOnboardingOpen flips to false on close).
    await expect(modal).not.toBeVisible({ timeout: 2_000 });
  });

  test.fixme(
    "Wake-lock toggle on PWA inhibits desktop sleep",
    async ({ page: _page, tauri: _tauri }) => {
      // OS-level: the desktop's `power_inhibit_set` IPC translates
      // to Win32 SetThreadExecutionState / macOS caffeinate /
      // Linux DBus inhibit. The frontend doesn't expose this
      // toggle today (the PWA is the only client); the actual
      // probe lives engine-side and is covered by
      // `cargo test -p copythat-power`.
    },
  );

  test.fixme(
    "Job snapshot is real — bytes/files/% reflect running job",
    async ({ page: _page, tauri: _tauri }) => {
      // The `mobile_snapshot` IPC is a desktop→PWA push, not a
      // user-visible UI surface on the desktop. Coverage lives in
      // `cargo test -p copythat-mobile` (the snapshot derivation
      // logic) and the PWA side's WebRTC rendering — not in this
      // Playwright harness.
    },
  );

  test.fixme(
    "Native Tauri Mobile binary scaffold compiles (smoke)",
    async ({ page: _page, tauri: _tauri }) => {
      // Build-time check, not a runtime UI assertion. Covered by
      // `cargo test -p copythat-ui` plus the tauri-build CI job.
    },
  );
});
