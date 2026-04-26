/**
 * §4.10 CLI (Phase 36) — Manual checklist verification.
 *
 * The CLI is a separate binary — `copythat` from `copythat-cli`.
 * The frontend isn't involved, so these tests shell out via
 * `child_process` instead of driving the Svelte app. They live in
 * the e2e directory so the qa-automate harness has a single
 * `pnpm test:e2e` entry point that covers every §4 subsection.
 *
 * The CLI smoke (`cargo test -p copythat-cli --test phase_36_cli`)
 * already exercises the same surface from Rust; this file is the
 * checkbox-shaped wrapper for parity with the rest of §4. The first
 * test (--json shape) is filled in; the rest are fixme stubs because
 * they need a known-good fixture file the harness doesn't ship yet.
 */

import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

import { expect, test } from "./fixtures/test";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
// repo root is e2e/ → apps/copythat-ui/ → apps/ → repo root
const REPO_ROOT = resolve(__dirname, "../../..");

function copythatBin(): string {
  // Prefer the release binary (faster, what users get); fall back
  // to debug if release isn't built yet.
  const release = resolve(REPO_ROOT, "target/release/copythat.exe");
  const releaseUnix = resolve(REPO_ROOT, "target/release/copythat");
  const debug = resolve(REPO_ROOT, "target/debug/copythat.exe");
  const debugUnix = resolve(REPO_ROOT, "target/debug/copythat");
  for (const candidate of [release, releaseUnix, debug, debugUnix]) {
    try {
      const r = spawnSync(candidate, ["--version"], { stdio: "pipe" });
      if (r.status === 0) return candidate;
    } catch {
      // not built / not on this OS
    }
  }
  return "";
}

test.describe("§4.10 CLI (Phase 36)", () => {
  test("`copythat version --json` emits a parseable JSON object", async () => {
    const bin = copythatBin();
    test.skip(
      bin === "",
      "copythat binary not built — run `cargo build -p copythat-cli` first",
    );
    const r = spawnSync(bin, ["version", "--json"], { stdio: "pipe" });
    expect(r.status).toBe(0);
    const body = r.stdout.toString();
    const parsed = JSON.parse(body);
    expect(parsed).toMatchObject({ version: expect.any(String) });
    // Spot-check a couple of fields the CLI manual mentions —
    // adjust if the schema lands with a different shape.
    expect(typeof parsed.version).toBe("string");
  });

  test.fixme(
    "`copythat copy <src> <dst> --json` emits one event per line",
    async () => {
      // Spawn with a small synthetic source dir + dst tempdir.
      // Read the stdout stream line by line; assert each line is
      // valid JSON and the sequence shape matches the
      // documented event types (job-started, job-progress,
      // job-finished). The synthetic tree fixture is what blocks
      // this from being filled in today.
    },
  );

  test.fixme(
    "`copythat plan --spec sample.toml` reports actions + exits 2",
    async () => {
      // Need a `sample.toml` fixture that documents a couple of
      // actions. Run plan; assert exit code 2 (pending actions
      // present) and that stderr contains the formatted action
      // list. The Rust smoke `cargo test -p copythat-cli`
      // already covers the underlying logic.
    },
  );

  test.fixme(
    "`copythat apply --spec sample.toml` runs once, second run exits 0",
    async () => {
      // Idempotency check. First apply: exit 0 with N actions.
      // Second apply on the same tree: exit 0 with 0 new
      // actions.
    },
  );

  test.fixme(
    "`copythat verify <file> --algo blake3 --against <sidecar>` exits 4 on mismatch",
    async () => {
      // Stage a known-good file + matching sidecar (exit 0),
      // then a mutated file with the same sidecar (exit 4).
      // Need fixture files committed before this can fill in.
    },
  );
});
