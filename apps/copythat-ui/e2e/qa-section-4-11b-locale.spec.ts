/**
 * §4.11b Locale sync (Phase 38 PWA i18n).
 */

import { test } from "./fixtures/test";

test.describe("§4.11b Locale sync (Phase 38 PWA i18n)", () => {
  test.fixme(
    "Switch desktop to French → PWA strings flip to French within 1 s",
    async ({ page: _page, tauri: _tauri }) => {
      // Open SettingsModal → General → Language. Pick French.
      // Assert `update_settings` invoked with
      // `general.locale = "fr"`. Mock the `mobile_locale_push`
      // IPC and assert it fires with `locale: "fr"` after the
      // update_settings round-trip — that's the desktop→PWA
      // sync edge.
    },
  );

  test.fixme(
    "Repeat for ja, ar (RTL), zh — PWA loads matching bundle",
    async ({ page: _page, tauri: _tauri }) => {
      // Same as above for each locale. For ar, assert the
      // settings dialog itself flips `dir = "rtl"` (Svelte
      // store wires this through `theme.ts`). MT-flagged
      // strings fall back to en — that's a Fluent runtime
      // behaviour and is covered by the i18n unit tests; this
      // checkbox just verifies the locale push.
    },
  );
});
