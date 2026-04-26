/**
 * §4.2 Move — Manual UI golden path.
 *
 * One `test()` per checkbox in `QualityAssuranceChecklist.md` §4.2.
 * Every test is a fixme stub; fill in by registering IPC handlers
 * via `tauri.handles({ ... })` and asserting on the resulting DOM.
 */

import { test } from "./fixtures/test";

test.describe("§4.2 Move", () => {
  test.fixme(
    "Same-volume move → atomic rename, source disappears",
    async ({ page: _page, tauri: _tauri }) => {
      // Mock `start_move` to return a job id. Emit
      // `job-progress` and `job-finished` (ok=true). Assert the
      // staging dialog's "Move" button surfaced; assert the row
      // shows `kind = "move"` and finishes successfully. The
      // engine's atomic-rename vs copy-and-delete decision is
      // backend-only; this test covers the wire-up.
    },
  );

  test.fixme(
    "Cross-volume move → falls back to copy + delete (EXDEV)",
    async ({ page: _page, tauri: _tauri }) => {
      // Same as above plus a `move-strategy` event with
      // `strategy = "CopyThenDelete"` should render as the
      // long-form label in the row's tooltip.
    },
  );

  test.fixme(
    "Cancel a long-running move → source intact, partial dst cleaned",
    async ({ page: _page, tauri: _tauri }) => {
      // Wire `cancel_job` handler. Drive a partial progress
      // sequence, click the row's cancel button, assert
      // `cancel_job` invoked with the right id; emit
      // `job-cancelled` and verify the row state transitions.
    },
  );
});
