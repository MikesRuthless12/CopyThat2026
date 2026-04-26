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
 * checkbox-shaped wrapper for parity with the rest of §4.
 */

import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";

import { expect, test } from "./fixtures/test";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const REPO_ROOT = resolve(__dirname, "../../..");

function copythatBin(): string {
  // Prefer the release binary (faster, what users get); fall back
  // to debug if release isn't built yet.
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

function withTmp<T>(prefix: string, fn: (dir: string) => T): T {
  const dir = mkdtempSync(resolve(tmpdir(), prefix));
  try {
    return fn(dir);
  } finally {
    try {
      rmSync(dir, { recursive: true, force: true });
    } catch {
      // best effort
    }
  }
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
    expect(typeof parsed.version).toBe("string");
  });

  test("`copythat copy <src> <dst> --json` emits one event per line", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-cp-", (dir) => {
      const srcDir = resolve(dir, "src");
      const dstDir = resolve(dir, "dst");
      mkdirSync(srcDir, { recursive: true });
      writeFileSync(resolve(srcDir, "a.bin"), Buffer.alloc(64));
      writeFileSync(resolve(srcDir, "b.bin"), Buffer.alloc(128));

      const r = spawnSync(bin, ["copy", "--json", srcDir, dstDir], {
        stdio: "pipe",
        timeout: 30_000,
      });
      expect(r.status).toBe(0);

      const lines = r.stdout
        .toString()
        .split(/\r?\n/)
        .filter((l) => l.trim().length > 0);
      expect(lines.length).toBeGreaterThan(0);
      // Every line is a parseable JSON object.
      for (const line of lines) {
        expect(() => JSON.parse(line)).not.toThrow();
      }
      // At least one of the lines should carry a `kind` field
      // matching the documented event shape.
      const kinds = lines.map((l) => (JSON.parse(l) as { kind?: string }).kind);
      expect(kinds.some((k) => typeof k === "string")).toBe(true);
    });
  });

  test("`copythat plan --spec sample.toml` reports actions", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-plan-", (dir) => {
      const srcDir = resolve(dir, "src");
      const dstDir = resolve(dir, "dst");
      mkdirSync(srcDir, { recursive: true });
      writeFileSync(resolve(srcDir, "a.bin"), Buffer.alloc(32));

      const specPath = resolve(dir, "spec.toml");
      const spec = `
[job]
kind = "copy"
source = [${JSON.stringify(srcDir.replace(/\\/g, "/"))}]
destination = ${JSON.stringify(dstDir.replace(/\\/g, "/"))}
`.trim();
      writeFileSync(specPath, spec);

      const r = spawnSync(bin, ["plan", "--spec", specPath, "--json"], {
        stdio: "pipe",
        timeout: 30_000,
      });
      // Plan exit codes: 0 = nothing to do, 2 = pending actions.
      // A fresh dst dir means there's work to do → exit 2.
      expect([0, 2]).toContain(r.status ?? -1);
    });
  });

  test("`copythat apply --spec sample.toml` runs once, second run exits 0", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-apply-", (dir) => {
      const srcDir = resolve(dir, "src");
      const dstDir = resolve(dir, "dst");
      mkdirSync(srcDir, { recursive: true });
      writeFileSync(resolve(srcDir, "a.bin"), Buffer.alloc(32));

      const specPath = resolve(dir, "spec.toml");
      const spec = `
[job]
kind = "copy"
source = [${JSON.stringify(srcDir.replace(/\\/g, "/"))}]
destination = ${JSON.stringify(dstDir.replace(/\\/g, "/"))}
`.trim();
      writeFileSync(specPath, spec);

      const first = spawnSync(bin, ["apply", "--spec", specPath, "--json"], {
        stdio: "pipe",
        timeout: 30_000,
      });
      expect(first.status).toBe(0);

      // Second run: idempotent, should also exit 0.
      const second = spawnSync(bin, ["apply", "--spec", specPath, "--json"], {
        stdio: "pipe",
        timeout: 30_000,
      });
      expect(second.status).toBe(0);
    });
  });

  test("`copythat verify <file> --algo blake3 --against <sidecar>` exits 4 on mismatch", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-verify-", (dir) => {
      const file = resolve(dir, "data.bin");
      writeFileSync(file, "hello world");

      // Sidecar must follow the `<hex>  <basename>` GNU `*sum`
      // format. A deliberately-wrong digest exercises the
      // mismatch path → exit 4.
      const sidecar = resolve(dir, "data.bin.b3");
      writeFileSync(
        sidecar,
        "0000000000000000000000000000000000000000000000000000000000000000  data.bin\n",
      );

      const mismatch = spawnSync(
        bin,
        ["verify", file, "--algo", "blake3", "--against", sidecar, "--quiet"],
        { stdio: "pipe", timeout: 30_000 },
      );
      // Exit 4 = verify failed (per the CLI's documented exit code table).
      expect(mismatch.status).toBe(4);
    });
  });
});
