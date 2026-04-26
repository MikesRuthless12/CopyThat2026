/**
 * §4.11g Phase 14f — queue-while-locked (Phase 38-followup-2).
 *
 * The `copythat queue --watch` subcommand isn't built today (only
 * `stack` exists in the CLI). Volume arrival/departure events
 * still flow through the engine; the CLI surface lands later.
 */

import { test } from "./fixtures/test";

test.describe("§4.11g Phase 14f queue-while-locked", () => {
  test.fixme(
    "Volume arrival: --watch surfaces VolumeArrival { root }",
    async () => {
      // The `copythat queue --watch` subcommand isn't built yet —
      // only `stack` exists in `copythat --help`. Coverage for the
      // event shape itself is in `cargo test -p copythat-platform`
      // (mock-watcher tests). Real plug-in still requires hardware
      // even when the subcommand lands.
    },
  );

  test.fixme(
    "Cancellation: Ctrl-C exits within 2 s",
    async () => {
      // Same dependency — needs `copythat queue --watch` to exist.
      // The signal-handling itself is shared with every other
      // long-running CLI subcommand and is covered by
      // `cargo test -p copythat-cli --test phase_28_signal`.
    },
  );
});
