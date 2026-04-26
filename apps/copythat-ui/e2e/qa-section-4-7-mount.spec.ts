/**
 * §4.7 Mount (Phase 33) — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.7 Mount (Phase 33)", () => {
  test.fixme(
    "History → Mount snapshot → Explorer opens read-only mount",
    async ({ page: _page, tauri: _tauri }) => {
      // Open HistoryDrawer; right-click a row → "Mount snapshot".
      // Mock `mount_snapshot` to return `{ mountpoint: "/mnt/cp-..." }`.
      // Assert `reveal_in_folder` invoked with the mountpoint.
      // Random-access correctness is engine-side; this test
      // covers the wire-up + the user-visible mountpoint string.
    },
  );

  test.fixme("Unmount → mountpoint disappears", async ({ page: _page, tauri: _tauri }) => {
    // Click "Unmount" on the same row → `unmount_snapshot`
    // invoked with the row id. Emit `mount-state-changed
    // { id, mounted: false }` and assert the row's mount badge
    // toggles off.
  });
});
