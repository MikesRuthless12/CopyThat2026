/**
 * §4.9 Encryption + compression (Phase 35) — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.9 Encryption + compression (Phase 35)", () => {
  test.fixme(
    "Encryption recipients flow → destination is age-encrypted",
    async ({ page: _page, tauri: _tauri }) => {
      // SettingsModal → Transfer → Encryption section. Click
      // "Add recipient" → enter `age1...` string. Save → assert
      // `update_settings` with
      // `transfer.encryption.recipients = ["age1..."]`. Then
      // start a copy; assert `start_copy` carries the
      // recipients in `options.encryption`. The age round-trip
      // itself is engine-side (`cargo test -p copythat-crypt`).
    },
  );

  test.fixme(
    "Smart compression — txt shrinks, jpg unchanged",
    async ({ page: _page, tauri: _tauri }) => {
      // Settings → Transfer → Compression = Smart, level 3.
      // Save → assert update_settings with
      // `transfer.compression.mode = "smart", level = 3`.
      // Drive two synthetic per-file `compression-decision`
      // events — one for `.txt` (`{ applied: true,
      // ratio: 0.6 }`) and one for `.jpg` (`{ applied: false,
      // reason: "deny-listed" }`). Assert the per-file rows
      // render the matching badges (compressed vs skipped).
    },
  );
});
