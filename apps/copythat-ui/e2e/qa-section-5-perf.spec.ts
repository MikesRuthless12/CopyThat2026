/**
 * §5 Performance + benchmarks.
 *
 * Most §5 items run via `xtask bench-ci` / `xtask bench-vs` / per-
 * crate cargo tests. Re-running them here would just duplicate
 * coverage. The one frontend-shaped check is that the Phase 13c
 * parallel-chunk gate stays env-var-gated by default — exposing a
 * Playwright-driven smoke that shells out to the cargo test that
 * enforces the invariant.
 */

import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

import { expect, test } from "./fixtures/test";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const REPO_ROOT = resolve(__dirname, "../../..");

test.describe("§5 Performance + benchmarks", () => {
  test("Phase 13c parallel-chunk path stays env-var-gated by default", async () => {
    // The smoke at `tests/smoke/phase_13c_parallel.rs` (in
    // copythat-platform) verifies the parallel chunk path is OFF
    // unless `COPYTHAT_PARALLEL_CHUNKS=N` is set. Run it via cargo;
    // exit 0 = invariant holds.
    const r = spawnSync(
      "cargo",
      [
        "test",
        "-p",
        "copythat-platform",
        "--test",
        "phase_13c_parallel",
        "--quiet",
      ],
      {
        stdio: "pipe",
        cwd: REPO_ROOT,
        timeout: 180_000,
      },
    );
    expect(r.status).toBe(0);
  });

  test.fixme(
    "xtask bench-ci finishes under 90s vs docs/BENCHMARKS.md baseline",
    async () => {
      // Already runs in `xtask qa-automate` (without `--skip-bench`)
      // so re-running here would burn the same minutes twice.
      // The QA report is the canonical surface for this check.
    },
  );

  test.fixme(
    "xtask bench-vs head-to-head against Robocopy / TeraCopy / FastCopy",
    async () => {
      // Needs a Windows host with the three competitors installed,
      // physical disks for the C→C / C→D / C→E scenarios. Manual
      // pre-tag pass per the QualityAssuranceChecklist appendix.
    },
  );

  test.fixme(
    "Per-volume buffer-size sweep matches Phase 13b 1 MiB optimum",
    async () => {
      // Engine-side; runs as part of `xtask bench-vs` on Windows.
    },
  );

  test.fixme(
    "Memory: 5 M-file scan database peak RSS < 200 MiB",
    async () => {
      // Long-running; needs a 5 M-file synthetic tree. Engine-side
      // smoke lives at `cargo test -p copythat-history --test
      // phase_19a_scan_db`.
    },
  );

  test.fixme(
    "Phase 13c research-vs-reality (CopyFileExW within 5%)",
    async () => {
      // Manual eyeball pass on `xtask bench-vs` output against the
      // committed COMPETITOR-TEST.md table.
    },
  );

  test.fixme(
    "Phase 17g fat LTO release-build under GitHub Actions 45 min cap",
    async () => {
      // CI-only — measured in release.yml's build job. Local timing
      // differs too much from the GHA Windows runners to be a
      // useful gate here.
    },
  );
});
