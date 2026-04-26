/**
 * §6 Cross-platform — every checkbox needs a non-current OS host.
 *
 * The GitHub Actions matrix covers tests + builds on Windows /
 * macOS / Linux. Manual UI golden path on each OS needs human
 * eyes (or a Computer Use session per OS) per the
 * QualityAssuranceChecklist appendix. Mobile checks need real
 * iOS / Android devices.
 */

import { test } from "./fixtures/test";

test.describe("§6 Cross-platform", () => {
  test.fixme(
    "Windows 10 NTFS — repeat §4.1–4.10",
    async ({ page: _page, tauri: _tauri }) => {
      // CI matrix in `release.yml` covers cargo test + build on
      // windows-latest. UI golden path per release needs human or
      // Computer Use session on Win10 NTFS.
    },
  );

  test.fixme(
    "Windows 11 NTFS + ReFS Dev Drive — repeat §4.1–4.10",
    async ({ page: _page, tauri: _tauri }) => {
      // Same coverage shape; ReFS-specific checks need a Dev Drive
      // host (Win11 only) — Computer Use session per release.
    },
  );

  test.fixme(
    "macOS 12 / 14 APFS — repeat §4.1–4.10",
    async ({ page: _page, tauri: _tauri }) => {
      // CI matrix covers macOS-latest + macOS-13. UI golden path
      // needs a Mac host or Computer Use session.
    },
  );

  test.fixme(
    "Ubuntu 22.04 ext4 — repeat §4.1–4.10",
    async ({ page: _page, tauri: _tauri }) => {
      // CI matrix covers ubuntu-latest. Manual / Computer Use for
      // visual UI verification.
    },
  );

  test.fixme(
    "Fedora 40 Btrfs — repeat §4.1–4.10",
    async ({ page: _page, tauri: _tauri }) => {
      // Btrfs-specific (CoW + reflink) — needs a Fedora host. The
      // engine-side reflink coverage runs on every CI ubuntu host
      // already; the UI half is documented as Computer Use only.
    },
  );

  test.fixme(
    "iOS Safari — PWA install + pair flow",
    async ({ page: _page, tauri: _tauri }) => {
      // Real iPhone with Safari required. PWA-side flow.
    },
  );

  test.fixme(
    "iOS Chrome (Safari WebKit underneath)",
    async ({ page: _page, tauri: _tauri }) => {
      // Same — real iPhone.
    },
  );

  test.fixme(
    "Android Chrome — PWA install + pair flow",
    async ({ page: _page, tauri: _tauri }) => {
      // Real Android device required.
    },
  );

  test.fixme(
    "Android Firefox",
    async ({ page: _page, tauri: _tauri }) => {
      // Real Android device required.
    },
  );
});
