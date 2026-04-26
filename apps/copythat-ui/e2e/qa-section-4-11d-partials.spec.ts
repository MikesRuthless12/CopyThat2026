/**
 * §4.11d Phase 8 partials (Phase 38-followup-3).
 */

import { test } from "./fixtures/test";

test.describe("§4.11d Phase 8 partials", () => {
  test.fixme(
    "Settings → Error prompt style: Modal vs Drawer survives restart",
    async ({ page: _page, tauri: _tauri }) => {
      // Open Settings → General. Find "Error prompt style"
      // dropdown. Switch to "Drawer" → assert update_settings
      // invoked with `general.errorPromptStyle = "drawer"`.
      // Emit an `error-prompt` event → assert
      // `ErrorPromptDrawer` (corner panel) renders, NOT
      // `ErrorModal`. Switch back to "Modal" → next
      // `error-prompt` should render `ErrorModal`. Reload page
      // mocking get_settings to return the persisted value →
      // the choice survives.
    },
  );

  test.fixme(
    "Collision modal → Quick hash (SHA-256) renders both digests",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `collision-prompt` for two paths. Click the
      // SHA-256 button on each side. Mock `quick_hash
      // { path, algo: "sha256" }` to return distinct hex
      // strings. Assert both digest strings render in the
      // modal within 1 s. Confirm the strings differ
      // (sanity check that the mock returned two different
      // values for two different paths).
    },
  );

  test.fixme(
    "Retry with elevated permissions surfaces err-permission-denied",
    async ({ page: _page, tauri: _tauri }) => {
      // Emit `error-prompt { id, kind: "permission-denied",
      // path: "C:\\Windows\\System32\\..." }`. Assert
      // ErrorModal shows "Retry with elevated permissions"
      // button. Click it → assert `retry_elevated` invoked.
      // Mock the response to re-emit the same
      // err-permission-denied (today's behaviour — the helper
      // runs in-process). Assert the modal shows the failure
      // again. The future UAC / sudo / polkit consent surface
      // is OS-level and out of scope for this test.
    },
  );
});
