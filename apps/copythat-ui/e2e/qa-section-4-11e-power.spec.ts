/**
 * §4.11e Phase 31b — real OS power probes.
 *
 * The frontend doesn't currently wire a `power-event` listener nor
 * expose a power-policy dropdown in SettingsModal. The probe layer
 * lives engine-side (covered by `cargo test -p copythat-power`).
 * Every checkbox here is engine + manual until the frontend grows
 * a power surface.
 */

import { test } from "./fixtures/test";

test.describe("§4.11e Phase 31b power probes", () => {
  test.fixme(
    "Windows presentation mode → engine pauses within 5 s",
    async ({ page: _page, tauri: _tauri }) => {
      // No `power-event` handler in stores.ts today. The engine
      // emits the pause/resume; the frontend just sees the
      // job-state flip via the standard `job-paused` /
      // `job-resumed` events. Real Focus Assist flip requires a
      // Windows host with the toggle.
    },
  );

  test.fixme(
    "Windows fullscreen → same pause/resume contract",
    async ({ page: _page, tauri: _tauri }) => {
      // Same shape — engine probe + standard pause/resume events.
      // Coverage: `cargo test -p copythat-power`.
    },
  );

  test.fixme(
    "Linux DBus screensaver inhibit → engine pauses",
    async ({ page: _page, tauri: _tauri }) => {
      // Same shape. The `dbus-inhibit-on/-off` events flow through
      // the engine's pause path; frontend renders the standard
      // job-paused state.
    },
  );

  test.fixme(
    "macOS — presentation/fullscreen probe stays a stub",
    async ({ page: _page, tauri: _tauri }) => {
      // No power-policy dropdown in SettingsModal today; the
      // probe never fires on macOS regardless. Becomes testable
      // when the General tab gains a "Pause on presentation"
      // option.
    },
  );
});
