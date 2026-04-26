/**
 * §10 Release prep — version sync.
 *
 * Most §10 items are manual git operations (changelog edit, tag,
 * push, GitHub release notes, docs site rebuild, announcement).
 * The one mechanically-checkable item is workspace version
 * coherence: `Cargo.toml` `[workspace.package].version` must match
 * `tauri.conf.json`'s `version` so the binaries the user installs
 * carry the same string the changelog references.
 */

import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

import { expect, test } from "./fixtures/test";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const REPO_ROOT = resolve(__dirname, "../../..");

test.describe("§10 Release prep", () => {
  test("Cargo.toml workspace version matches tauri.conf.json", async () => {
    const cargoToml = readFileSync(resolve(REPO_ROOT, "Cargo.toml"), "utf8");
    const cargoMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);
    expect(cargoMatch, "Cargo.toml workspace version must be set").not.toBeNull();
    const cargoVersion = cargoMatch![1];

    const tauriJson = readFileSync(
      resolve(REPO_ROOT, "apps/copythat-ui/src-tauri/tauri.conf.json"),
      "utf8",
    );
    const tauri = JSON.parse(tauriJson) as { version?: string };
    expect(tauri.version, "tauri.conf.json must declare a version").toBeDefined();

    expect(tauri.version).toBe(cargoVersion);
  });

  test("App-name fluent key embeds the same version as Cargo.toml", async () => {
    const cargoToml = readFileSync(resolve(REPO_ROOT, "Cargo.toml"), "utf8");
    const cargoVersion = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)![1];

    const enFtl = readFileSync(
      resolve(REPO_ROOT, "locales/en/copythat.ftl"),
      "utf8",
    );
    const appName = enFtl.match(/^app-name\s*=\s*(.+)$/m);
    expect(appName, "en/copythat.ftl must define app-name").not.toBeNull();
    expect(appName![1]).toContain(cargoVersion);
  });

  test.fixme(
    "git tag v<version> and push --tags",
    async () => {
      // Manual git op — the release engineer runs `git tag` after
      // CHANGELOG.md, Cargo.toml, and tauri.conf.json are all
      // landed and the qa-automate report is clean.
    },
  );

  test.fixme(
    "GitHub Releases entry copies the CHANGELOG block + installer artifacts",
    async () => {
      // Driven by `release.yml` on tag push — a separate Computer
      // Use session can verify the Releases page renders correctly
      // (signed installers attached, notes rendered, etc.).
    },
  );

  test.fixme(
    "Public docs site (docs/site/) rebuilds + republishes",
    async () => {
      // Lives in a separate publishing pipeline; covered by the
      // docs-site repo's CI, not this harness.
    },
  );

  test.fixme(
    "Announce on CopyThat blog / Twitter / Reddit / HN",
    async () => {
      // Human-only.
    },
  );
});
