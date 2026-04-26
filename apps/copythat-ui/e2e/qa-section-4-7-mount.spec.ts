/**
 * §4.7 Mount (Phase 33) — Manual UI golden path.
 *
 * Drives the History → Mount snapshot flow. Random-access reads
 * inside the mount are engine-side (covered by
 * `cargo test -p copythat-mount`).
 */

import { expect, test } from "./fixtures/test";

test.describe("§4.7 Mount (Phase 33)", () => {
  test("History → Mount snapshot → mount_snapshot invoked with mountpoint", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      history_search: () => [
        {
          rowId: 7,
          kind: "copy",
          status: "succeeded",
          startedAtMs: 1_700_000_000_000,
          finishedAtMs: 1_700_000_001_000,
          srcRoot: "/tmp/src",
          dstRoot: "/tmp/dst",
          totalBytes: 1024,
          filesOk: 1,
          filesFailed: 0,
          verifyAlgo: "blake3",
          optionsJson: null,
        },
      ],
      "plugin:dialog|open": () => "/mnt/cp-7",
      mount_snapshot: () => ({ mountpoint: "/mnt/cp-7" }),
    });

    // Open the History drawer via the Footer button.
    await page.getByRole("button", { name: /history/i }).first().click();
    await expect(
      page.getByText(/\/tmp\/src|\/tmp\/dst|history/i).first(),
    ).toBeVisible({ timeout: 5_000 });

    // Click the Mount action on the first history row.
    await page.getByRole("button", { name: /mount/i }).first().click();

    const mountCall = await tauri.waitForCall("mount_snapshot");
    expect(mountCall.args?.jobRowId).toBe(7);
    expect(mountCall.args?.mountpoint).toBe("/mnt/cp-7");
  });

  test.fixme("Unmount → mountpoint disappears", async ({ page: _page, tauri: _tauri }) => {
    // The Unmount button isn't wired in HistoryDrawer today —
    // mounting writes the path via a toast and the user runs the
    // OS-level unmount themselves. When the per-row Unmount lands,
    // this test becomes:
    //   1. Same setup as Mount.
    //   2. Click Unmount on the row → assert `unmount_snapshot`
    //      fires with the rowId.
    //   3. Emit `mount-state-changed { id, mounted: false }` →
    //      assert the mount badge clears.
  });
});
