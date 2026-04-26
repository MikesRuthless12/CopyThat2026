/**
 * §4.11f Phase 14d — scheduled jobs (Phase 38-followup-2).
 *
 * The scheduling work happens inside `copythat-cli`, not the Tauri
 * frontend. The CLI render checkboxes are checked by spawning the
 * binary the same way §4.10 does. The Phase 17a guard checkbox
 * also lives at the CLI surface.
 */

import { spawnSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { test } from "./fixtures/test";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const REPO_ROOT = resolve(__dirname, "../../..");

function copythatBin(): string {
  const candidates = [
    resolve(REPO_ROOT, "target/release/copythat.exe"),
    resolve(REPO_ROOT, "target/release/copythat"),
    resolve(REPO_ROOT, "target/debug/copythat.exe"),
    resolve(REPO_ROOT, "target/debug/copythat"),
  ];
  for (const candidate of candidates) {
    try {
      const r = spawnSync(candidate, ["--version"], { stdio: "pipe" });
      if (r.status === 0) return candidate;
    } catch {
      // not built / not on this OS
    }
  }
  return "";
}

test.describe("§4.11f Phase 14d scheduled jobs", () => {
  test.fixme("CLI render — Windows: schtasks /Create form", async () => {
    // Need a `sample.toml` fixture committed under
    // `tests/smoke/fixtures/scheduled-job.toml`. Once that
    // lands: resolve `copythatBin()`, spawn `copythat schedule
    // --spec sample.toml --host windows`, capture stdout, assert
    // it begins with `schtasks /Create` and contains the
    // expected /SC, /TN, /TR, /ST flags.
    void copythatBin;
  });

  test.fixme(
    "CLI render — macOS: launchd plist form",
    async () => {
      // `copythat schedule --spec sample.toml --host macos`.
      // Assert stdout is a valid plist (parses XML, root
      // element is `<plist>`, has `<dict>` with `Label` +
      // `ProgramArguments` keys).
    },
  );

  test.fixme(
    "CLI render — Linux: systemd .service + .timer pair",
    async () => {
      // `copythat schedule --spec sample.toml --host linux`.
      // Assert stdout contains a `[Unit]` block, an `[Install]`
      // block, and the timer body contains an `OnCalendar=`
      // directive matching the spec's cadence.
    },
  );

  test.fixme(
    "Phase 17a guard — `..`-laden source rejected with err-path-escape",
    async () => {
      // Spawn `copythat schedule --spec spec.toml` where
      // spec.toml carries `source = "/some/../etc/passwd"`.
      // Assert exit code 2 and stderr contains
      // `err-path-escape`. Already covered by
      // `cargo test -p copythat-cli --test phase_17_security`;
      // this test is parity for the §4 list.
    },
  );
});
