/**
 * §4.11g Phase 14f — queue-while-locked (Phase 38-followup-2).
 *
 * Volume-arrival / volume-departure handling lives at the engine
 * boundary. The CLI exposes `copythat queue --watch` which prints
 * a typed event stream. The frontend has no UI for this yet, so
 * the tests here shell out to the CLI like §4.10 / §4.11f.
 */

import { test } from "./fixtures/test";

test.describe("§4.11g Phase 14f queue-while-locked", () => {
  test.fixme(
    "Volume arrival: --watch surfaces VolumeArrival { root }",
    async () => {
      // Need a real removable volume to plug in. Realistic test:
      // mock the platform watcher via `COPYTHAT_TEST_VOLUME_BUS=1`
      // env var (engine-side stub) so the CLI re-emits a
      // synthetic VolumeArrival event. Assert the JSON line
      // shape on stdout.
    },
  );

  test.fixme(
    "Cancellation: Ctrl-C exits within 2 s",
    async () => {
      // Spawn `copythat queue --watch` with stdin piped. Wait
      // 1 s, send SIGINT (or generate Ctrl-C on Windows via
      // GenerateConsoleCtrlEvent). Assert the process exits
      // with code 130 (or whatever copythat picks for
      // user-cancellation) within 2 s of signal delivery.
    },
  );
});
