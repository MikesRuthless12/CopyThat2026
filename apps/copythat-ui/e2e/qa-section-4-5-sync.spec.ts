/**
 * §4.5 Sync (Phase 25) — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.5 Sync (Phase 25)", () => {
  test.fixme(
    "Add sync pair → toggle live-mirror → right tree updates",
    async ({ page: _page, tauri: _tauri }) => {
      // 1. Open SyncDrawer. Click "Add pair" → modal renders the
      //    left/right path pickers. Stub `pick_folders` to return
      //    /tmp/left and /tmp/right. Submit → assert
      //    `sync_pair_create` invoked with both paths.
      // 2. Toggle live-mirror on. Assert
      //    `sync_pair_set_live { id, enabled: true }` invoked.
      // 3. Emit `sync-event { side: "left", path: "...", op: "modify" }`
      //    and verify the right column shows the matching update
      //    row within 1 s (debounce window).
    },
  );

  test.fixme(
    "Modify same file on both sides → vector-clock conflict modal",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `sync-conflict { id, path, leftVersion, rightVersion }`.
      // Assert the conflict modal opens; click "Keep left" →
      // `resolve_sync_conflict { id, resolution: "keep-left" }`
      // invoked. Repeat for "Keep right" and "Keep both".
    },
  );
});
