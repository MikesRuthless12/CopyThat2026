/**
 * §4.11e Phase 31b — real OS power probes.
 *
 * The OS-level probes (Focus Assist, fullscreen detection, dbus
 * screensaver) are platform-specific and run inside the engine.
 * The frontend only sees `power-event` notifications and a
 * paused-because-presentation badge on the JobRow. These tests
 * cover the event → UI half; the probe accuracy itself is
 * engine-side.
 */

import { test } from "./fixtures/test";

test.describe("§4.11e Phase 31b power probes", () => {
  test.fixme(
    "Windows presentation mode → engine pauses within 5 s",
    async ({ page: _page, tauri: _tauri }) => {
      // Settings → General → Power policy = "Pause on
      // presentation". Drive a job to running state. Emit
      // `power-event { kind: "presentation-on" }`. Assert the
      // active JobRow's state badge transitions to "paused
      // (presentation)" within 5 s. Emit
      // `power-event { kind: "presentation-off" }` → assert
      // the row resumes.
    },
  );

  test.fixme(
    "Windows fullscreen → same pause/resume contract",
    async ({ page: _page, tauri: _tauri }) => {
      // Same as above with kind: "fullscreen-on" /
      // "fullscreen-off". Engine probe details are covered by
      // `cargo test -p copythat-power`; this test is the wire-up.
    },
  );

  test.fixme(
    "Linux DBus screensaver inhibit → engine pauses",
    async ({ page: _page, tauri: _tauri }) => {
      // Same shape, kind: "dbus-inhibit-on" /
      // "dbus-inhibit-off". The dbus probe lives in
      // copythat-power and is covered by
      // `cargo test -p copythat-power`.
    },
  );

  test.fixme(
    "macOS — presentation/fullscreen probe stays a stub",
    async ({ page: _page, tauri: _tauri }) => {
      // Settings → General → Power policy dropdown should still
      // show "Pause on presentation" as a selectable option even
      // though the macOS probe isn't wired. Assert the option
      // exists and can be picked + saved (documentation-only
      // for now).
    },
  );
});
