/**
 * §4.8 Audit log (Phase 34) — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.8 Audit log (Phase 34)", () => {
  test.fixme(
    "Settings → Advanced → enable JSON-Lines audit → run a copy → file gains records",
    async ({ page: _page, tauri: _tauri }) => {
      // Open SettingsModal → Advanced. Toggle "Audit log" on,
      // pick format = "jsonl", confirm. Assert
      // `update_settings` invoked with
      // `audit.enabled = true, audit.format = "jsonl"`. The
      // record-content half is engine-side; this test verifies
      // the settings round-trip.
    },
  );

  test.fixme(
    "WORM mode on → truncate refused (chattr +a / read-only attr)",
    async ({ page: _page, tauri: _tauri }) => {
      // Toggle WORM. Assert the explanatory hint renders ("OS
      // append-only attribute will be set on the audit file").
      // Mock `audit_set_worm` to succeed; assert the
      // confirmation toast lands. The actual chattr / Win32
      // attribute set is engine-side.
    },
  );

  test.fixme(
    "Verify chain → green for untampered, red after dd overwrites a record",
    async ({ page: _page, tauri: _tauri }) => {
      // Click "Verify chain". Mock `audit_verify` first to
      // return `{ ok: true, lastSeq: 1234 }` → assert green
      // checkmark. Re-run with the mock returning
      // `{ ok: false, brokenAtSeq: 500 }` → assert red banner
      // surfaces with the broken-record number.
    },
  );
});
