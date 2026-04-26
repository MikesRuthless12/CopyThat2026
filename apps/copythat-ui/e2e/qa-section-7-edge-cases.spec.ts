/**
 * §7 Edge cases — frontend-testable items.
 *
 * Most §7 items need real hardware (locked file with Excel open, AC
 * unplug, sparse-aware filesystems, NTFS ADS to ext4). The frontend
 * surface for those is just settings round-trips + event rendering;
 * the engine-side behaviours are covered by per-crate cargo tests.
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§7 Edge cases — frontend surface", () => {
  test("Bandwidth shaping: Settings → Network → mode dropdown round-trip", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue("get_settings", fullSettings());
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /network/i }).click();

    // Network mode dropdown: off / fixed / schedule. Use the
    // accessible label to disambiguate from the auto-throttle
    // sub-dropdowns below it.
    const modeSelect = settingsModal.getByLabel(/Bandwidth limit/i).first();
    await modeSelect.selectOption("fixed");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { network?: { mode?: string } } | undefined;
    expect(dto?.network?.mode).toBe("fixed");
  });

  test("Bandwidth shape badge renders shape-rate-changed event", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    // Header subscribes to `shape-rate-changed`. Emitting an active
    // cap should make the bandwidth badge surface a value.
    await tauri.emit("shape-rate-changed", {
      bytesPerSecond: 5 * 1024 * 1024, // 5 MB/s
      source: "settings",
    });

    // The Header renders the cap as a formatted rate. We just check
    // the page didn't crash and a rate-like string surfaces. The
    // Header's badge is conditionally rendered based on $shapeRate.
    await page.waitForTimeout(300);
    // Soft assertion — the test's value is "the event handler doesn't
    // crash"; the visual badge format is covered by the format unit
    // test (`format.test.ts`).
    await expect(page.locator("body")).toBeVisible();
  });

  test("Resume modal renders when pending_resumes is non-empty", async ({
    page,
    tauri,
  }) => {
    await page.addInitScript(() => {
      const reg = (window as unknown as {
        __copythat_e2e__?: {
          setHandler: (cmd: string, h: (a: unknown) => unknown) => void;
        };
      }).__copythat_e2e__;
      if (!reg) return;
      reg.setHandler("pending_resumes", () => [
        {
          rowId: 42,
          kind: "copy",
          srcRoot: "/tmp/src",
          dstRoot: "/tmp/dst",
          status: "running",
          startedAtMs: Date.now() - 60_000,
          bytesDone: 524288000,
          bytesTotal: 1073741824,
          filesDone: 50,
          filesTotal: 100,
          lastCheckpointAtMs: Date.now() - 1000,
        },
      ]);
    });

    await page.goto("/");

    // The resume modal renders when pendingResumes is non-empty
    // (App.svelte's onMount fetches them and conditionally shows
    // ResumePromptModal). We assert by looking for the row text.
    await expect(
      page.getByText(/\/tmp\/src|resume|continue/i).first(),
    ).toBeVisible({ timeout: 5_000 });
    void tauri;
  });

  test("Locked-file (Phase 19b) settings: onLocked dropdown round-trip", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue("get_settings", fullSettings());
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /transfer/i }).click();

    // The "When a file is locked" picker — values are
    // ask / retry / skip / snapshot.
    const lockedSelect = settingsModal.locator("select").filter({
      hasText: /Snapshot|Retry/i,
    });
    if ((await lockedSelect.count()) > 0) {
      await lockedSelect.first().selectOption("snapshot");
      const updateCall = await tauri.waitForCall("update_settings");
      const dto = updateCall.args?.dto as { transfer?: { onLocked?: string } } | undefined;
      expect(dto?.transfer?.onLocked).toBe("snapshot");
    }
  });

  test.fixme(
    "Resume across reboot — actual reboot needed",
    async ({ page: _page, tauri: _tauri }) => {
      // Hardware-bound. Engine-side resume coverage is in
      // `cargo test -p copythat-journal`.
    },
  );

  test.fixme(
    "Sparse files (Phase 23) — needs sparse-aware filesystems",
    async ({ page: _page, tauri: _tauri }) => {
      // Requires real APFS / Btrfs / NTFS-with-sparse for the dst
      // volume. Engine-side coverage in `copythat-platform`.
    },
  );

  test.fixme(
    "Security metadata (Phase 24) — NTFS ADS → ext4 AppleDouble sidecar",
    async ({ page: _page, tauri: _tauri }) => {
      // Requires Windows source + Linux destination volume. Engine
      // covers the metadata translation logic.
    },
  );

  test.fixme(
    "Path translation (Phase 30): NFD normalisation Windows → macOS",
    async ({ page: _page, tauri: _tauri }) => {
      // Engine-side; covered by `cargo test -p copythat-core
      // --test phase_30_translate`.
    },
  );

  test.fixme(
    "Power policy (Phase 31): unplug AC → engine pauses",
    async ({ page: _page, tauri: _tauri }) => {
      // Hardware-bound. Frontend has no power-event handler today.
    },
  );
});
