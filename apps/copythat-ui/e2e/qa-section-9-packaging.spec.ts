/**
 * §9 Packaging + signing.
 *
 * The release.yml CI workflow handles every checkbox here on tag
 * push: it builds installers per OS, signs them via the platform's
 * signing key, and notarizes the macOS bundle. Local Playwright
 * runs can't reproduce the CI signing keys (and shouldn't), so
 * every check defers to the manual / CI surface.
 */

import { test } from "./fixtures/test";

test.describe("§9 Packaging + signing", () => {
  test.fixme(
    "xtask release produces installers on every target",
    async () => {
      // Driven by `release.yml` matrix on tag push. Local
      // qa-automate doesn't run the packaging stage.
    },
  );

  test.fixme(
    "Windows MSI installs without admin (per-user install)",
    async () => {
      // Manual install verification on a clean Windows host.
    },
  );

  test.fixme(
    "macOS DMG opens; .app runs without Gatekeeper blocks",
    async () => {
      // Requires a notarized build + a macOS host. Verified once
      // per release on the signing engineer's Mac.
    },
  );

  test.fixme(
    "Linux AppImage / deb / rpm install via package manager",
    async () => {
      // Manual install on Ubuntu / Debian / Fedora / openSUSE.
    },
  );

  test.fixme(
    "Auto-updater pings the manifest endpoint and detects bumped version",
    async () => {
      // Needs a deployed update manifest and a real network round-
      // trip. Pre-tag dress rehearsal item — engineering tests the
      // Phase 15 throttle separately in `cargo test -p
      // copythat-ui --test phase_15_updater`.
    },
  );
});
