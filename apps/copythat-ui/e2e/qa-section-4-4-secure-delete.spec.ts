/**
 * §4.4 Secure delete — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.4 Secure delete", () => {
  test.fixme(
    "Right-click → Secure Delete (DoD 3-pass) → confirmation → file gone",
    async ({ page: _page, tauri: _tauri }) => {
      // Seed `list_jobs` with a completed copy; right-click the
      // row to surface ContextMenu; click "Secure Delete". Assert
      // the confirmation modal renders with the DoD-3 label;
      // confirm → `start_secure_delete` invoked with
      // `method = "dod3"` and the file's source path. Drive a
      // shred-progress event and a shred-finished event; assert
      // the row transitions to a "secure-delete" kind row.
    },
  );

  test.fixme(
    "On a CoW filesystem → SSD-aware refusal explanation",
    async ({ page: _page, tauri: _tauri }) => {
      // Mock `start_secure_delete` to reject with the
      // localized `err-shred-cow-refusal` key. Assert ErrorModal
      // surfaces the long-form copy ("Btrfs/APFS/ReFS shadow
      // copies prevent reliable overwrite — use Trash + zero-fill
      // instead"). Confirm a "Learn more" link routes to the
      // engineering doc URL.
    },
  );
});
