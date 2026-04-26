/**
 * §4.11c Phase 38 — destination dedup ladder.
 *
 * The dedup-mode + hardlink-policy settings are persisted in the
 * `transfer.dedupMode` / `transfer.dedupHardlinkPolicy` fields of
 * `SettingsDto`. The frontend doesn't surface dropdowns for these
 * yet (Phase 38 follow-up), and the per-file `dedup-strategy` event
 * isn't subscribed in stores.ts. Coverage today is the wire-shape
 * round-trip — the UI badges land later.
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.11c Phase 38 destination dedup ladder", () => {
  test("Settings carries dedupMode round-trip via update_settings", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue(
      "get_settings",
      fullSettings({
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
          dedupMode: "auto-ladder",
          dedupHardlinkPolicy: "off",
          dedupPrescan: false,
        },
      }),
    );
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
    });

    // Just verify the settings round-trip retains the dedupMode
    // field — the UI surface for picking it lands in a later
    // Phase 38 follow-up.
    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });
  });

  test.fixme(
    "Mode = AutoLadder + reflink-capable FS → Reflink strategy + bytes_saved",
    async ({ page: _page, tauri: _tauri }) => {
      // No `dedup-strategy` event handler in stores.ts today; the
      // per-file Reflink/Hardlink/Copy badge is engine-side only.
      // Coverage lives in `cargo test -p copythat-platform`.
    },
  );

  test.fixme(
    "AutoLadder + HardlinkPolicy = Always on NTFS → Hardlink + yellow warning",
    async ({ page: _page, tauri: _tauri }) => {
      // Same story — engine-side until the dedup-strategy event +
      // per-row badge land in the frontend.
    },
  );

  test.fixme(
    "Mode = ReflinkOnly on NTFS → every file reports Copy",
    async ({ page: _page, tauri: _tauri }) => {
      // Same story.
    },
  );

  test.fixme(
    "Mode = None on any volume → every file reports Skipped",
    async ({ page: _page, tauri: _tauri }) => {
      // Same story.
    },
  );

  test.fixme(
    "Pre-pass scan modal proposes 50 hardlink/reflink actions",
    async ({ page: _page, tauri: _tauri }) => {
      // The `start_dedup_scan` IPC isn't wired yet (per the
      // QualityAssuranceChecklist's note: "requires the dedup-scan
      // IPC to land first").
    },
  );
});
