/**
 * §8 i18n — every locale renders translated text + CLI stays English.
 *
 * Two flavours of coverage:
 *  - For each of the 18 shipped locales, switch the desktop's
 *    language and verify a known UI string renders in that locale's
 *    translation rather than the English fallback. This catches
 *    "no English strings leak through" regressions without scraping
 *    the whole DOM.
 *  - The CLI's --help output stays English regardless of $LANG —
 *    the engineering-accessibility constraint documented in
 *    `crates/copythat-cli/src/cli.rs`'s `after_help` block.
 *
 * The Rust-side `xtask i18n-lint` already enforces key parity across
 * locales; this harness covers the runtime rendering.
 */

import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

import { ALL_LOCALES, expect, test, translationsFor } from "./fixtures/test";

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

test.describe("§8 i18n — locale rendering", () => {
  for (const locale of ALL_LOCALES) {
    if (locale === "en") continue;
    test(`Locale ${locale} renders translated empty-state text`, async ({
      page,
      tauri,
    }) => {
      const bundle = translationsFor(locale);
      const expected = bundle["empty-title"];
      // If a locale ships without the empty-title key, surface that
      // as a hard failure here — i18n-lint already enforces parity
      // across files, but this is the runtime second-line check.
      expect(
        expected,
        `locale ${locale} must define empty-title`,
      ).toBeDefined();

      // Override translations + system_locale BEFORE the page boots
      // so initI18n picks up the right bundle on first render.
      // Also override navigator.language since i18n.ts's pickPreferred
      // checks the navigator's locale list FIRST — without this the
      // webview's en-US would always win over the IPC-side override.
      await page.addInitScript(
        (seed: { locale: string; bundle: Record<string, string> }) => {
          Object.defineProperty(navigator, "language", {
            get: () => seed.locale,
            configurable: true,
          });
          Object.defineProperty(navigator, "languages", {
            get: () => [seed.locale],
            configurable: true,
          });
          const reg = (window as unknown as {
            __copythat_e2e__?: {
              setHandler: (cmd: string, h: (a: unknown) => unknown) => void;
            };
          }).__copythat_e2e__;
          if (!reg) return;
          reg.setHandler("system_locale", () => seed.locale);
          reg.setHandler("translations", (args: unknown) => {
            const a = args as { locale?: string } | undefined;
            return a?.locale === seed.locale ? seed.bundle : {};
          });
        },
        { locale, bundle },
      );

      await page.goto("/");
      await expect(page.getByText(expected)).toBeVisible({ timeout: 5_000 });

      // Sanity-check: the English version should NOT be rendered.
      // Skip locales whose translation happens to coincide with
      // English (rare but possible for tiny strings).
      const enBundle = translationsFor("en");
      const enText = enBundle["empty-title"];
      if (enText && enText !== expected) {
        await expect(page.getByText(enText)).toHaveCount(0);
      }

      // RTL check: ar should set html dir="rtl"; everything else
      // should be ltr. i18n.ts derives this from the locale code.
      const expectedDir = locale === "ar" ? "rtl" : "ltr";
      expect(
        await page.evaluate(() => document.documentElement.getAttribute("dir")),
      ).toBe(expectedDir);
    });
  }
});

test.describe("§8 i18n — CLI stays English", () => {
  for (const lang of ["fr_FR.UTF-8", "ja_JP.UTF-8", "ar_SA.UTF-8"]) {
    test(`copythat --help stays English under LANG=${lang}`, async () => {
      const bin = copythatBin();
      test.skip(bin === "", "copythat binary not built");

      const r = spawnSync(bin, ["--help"], {
        stdio: "pipe",
        timeout: 30_000,
        env: { ...process.env, LANG: lang, LC_ALL: lang, LC_MESSAGES: lang },
      });
      expect(r.status).toBe(0);
      const out = r.stdout.toString();
      // The CLI's help block is pinned to English regardless of
      // $LANG (per `crates/copythat-cli/src/cli.rs`'s after_help
      // policy). We assert by checking for English-only tokens
      // that wouldn't survive translation: the command names
      // themselves (`copy`, `move`, `sync`) and `Usage:`.
      expect(out).toMatch(/Usage:/);
      expect(out).toMatch(/Commands:/);
      // Spot-check a couple of subcommand names.
      expect(out).toMatch(/\bcopy\b/);
      expect(out).toMatch(/\bsync\b/);
    });
  }
});
