/**
 * §4.3 Verify — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.3 Verify", () => {
  test.fixme(
    "Settings → Verify = blake3 → green checkmark on row after copy",
    async ({ page: _page, tauri: _tauri }) => {
      // 1. Open SettingsModal (via Header settings button or
      //    `openSettings` store flag), pick Transfer tab,
      //    flip Verify to "blake3". Assert `update_settings` is
      //    invoked with `transfer.verifyAlgo = "blake3"`.
      // 2. Trigger a copy job; emit `verify-passed { id }` after
      //    `job-finished`. Assert the row's verify badge shows
      //    the green checkmark and the algo label "BLAKE3".
    },
  );

  test.fixme(
    "Verify mismatch → VerifyFailed surfaced, partial dst removed",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `verify-failed { id, expected, actual }`. Assert
      // ErrorModal opens with localized `err-verify-mismatch`.
      // Click "Dismiss" → `resolve_error` invoked. The "partial
      // dst removed" half is engine-side; covered by
      // `cargo test -p copythat-core --test phase_03_verify`.
    },
  );
});
