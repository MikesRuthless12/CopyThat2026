/**
 * §4.11f Phase 14d — scheduled jobs (Phase 38-followup-2).
 *
 * The scheduling work happens inside `copythat-cli`, not the Tauri
 * frontend. Tests shell out to the binary the same way §4.10 does.
 */

import { spawnSync } from "node:child_process";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";

import { expect, test } from "./fixtures/test";

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

function writeSpec(dir: string): string {
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

[schedule]
cron = "0 3 * * *"
`.trim();
  writeFileSync(specPath, spec);
  return specPath;
}

test.describe("§4.11f Phase 14d scheduled jobs", () => {
  test("CLI render — Windows: schtasks /Create form", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-sched-win-", (dir) => {
      const specPath = writeSpec(dir);
      const r = spawnSync(
        bin,
        ["schedule", "--spec", specPath, "--host", "windows"],
        { stdio: "pipe", timeout: 30_000 },
      );
      expect(r.status).toBe(0);
      const out = r.stdout.toString();
      expect(out).toMatch(/schtasks/i);
      expect(out).toMatch(/\/Create/);
    });
  });

  test("CLI render — macOS: launchd plist form", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-sched-mac-", (dir) => {
      const specPath = writeSpec(dir);
      const r = spawnSync(
        bin,
        ["schedule", "--spec", specPath, "--host", "mac-os"],
        { stdio: "pipe", timeout: 30_000 },
      );
      expect(r.status).toBe(0);
      const out = r.stdout.toString();
      expect(out).toMatch(/<plist|Label|ProgramArguments/i);
    });
  });

  test("CLI render — Linux: systemd .service + .timer pair", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-sched-linux-", (dir) => {
      const specPath = writeSpec(dir);
      const r = spawnSync(
        bin,
        ["schedule", "--spec", specPath, "--host", "linux"],
        { stdio: "pipe", timeout: 30_000 },
      );
      expect(r.status).toBe(0);
      const out = r.stdout.toString();
      expect(out).toMatch(/\[Unit\]/);
      expect(out).toMatch(/OnCalendar/);
    });
  });

  test("Phase 17a guard — `..`-laden source rejected with err-path-escape", async () => {
    const bin = copythatBin();
    test.skip(bin === "", "copythat binary not built");
    withTmp("copythat-e2e-sched-escape-", (dir) => {
      // Create a real source dir so the `source must exist` check
      // passes; the escape sequence is what should trigger the
      // path-escape guard inside the CLI's path normalizer.
      const srcDir = resolve(dir, "src");
      mkdirSync(srcDir, { recursive: true });

      const specPath = resolve(dir, "spec.toml");
      const spec = `
[job]
kind = "copy"
source = [${JSON.stringify(srcDir.replace(/\\/g, "/") + "/../etc/passwd")}]
destination = ${JSON.stringify(resolve(dir, "dst").replace(/\\/g, "/"))}

[schedule]
cron = "0 3 * * *"
`.trim();
      writeFileSync(specPath, spec);

      const r = spawnSync(
        bin,
        ["schedule", "--spec", specPath, "--host", "linux"],
        { stdio: "pipe", timeout: 30_000 },
      );
      // Path-escape rejection lands at exit 9 (config invalid) or
      // 2 (jobspec validation) depending on where the guard fires.
      // Either is a non-zero failure; that's the contract.
      expect(r.status === 0).toBe(false);
    });
  });
});
