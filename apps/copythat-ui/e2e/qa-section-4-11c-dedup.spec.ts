/**
 * §4.11c Phase 38 — destination dedup ladder.
 */

import { test } from "./fixtures/test";

test.describe("§4.11c Phase 38 destination dedup ladder", () => {
  test.fixme(
    "Mode = AutoLadder + reflink-capable FS → Reflink strategy + bytes_saved",
    async ({ page: _page, tauri: _tauri }) => {
      // Settings → Transfer → Dedup mode = AutoLadder. Save.
      // Drive a copy that emits `dedup-strategy { id, file,
      // strategy: "Reflink", bytesSaved: <size> }` per file.
      // Assert the per-file row renders the green "Reflink"
      // badge and the Footer's saved-bytes counter increments.
    },
  );

  test.fixme(
    "AutoLadder + HardlinkPolicy = Always on NTFS → Hardlink + yellow warning",
    async ({ page: _page, tauri: _tauri }) => {
      // Settings → Transfer → HardlinkPolicy = "always". Save.
      // Drive a copy emitting `dedup-strategy { strategy:
      // "Hardlink" }`. Assert the row's badge is yellow and the
      // PWA-mirror panel's hardlink-warning chip is visible
      // (covers the "touching either name affects the other"
      // warning from the checklist).
    },
  );

  test.fixme(
    "Mode = ReflinkOnly on NTFS → every file reports Copy",
    async ({ page: _page, tauri: _tauri }) => {
      // Same as AutoLadder test but mock `dedup-strategy` to
      // emit `strategy: "Copy"` for every file. No hardlink
      // chip; no yellow warning. Footer's saved-bytes stays at
      // 0. Confirms the mode select properly gates fallback.
    },
  );

  test.fixme(
    "Mode = None on any volume → every file reports Skipped",
    async ({ page: _page, tauri: _tauri }) => {
      // Strategy events arrive with `strategy: "Skipped"`. Per-
      // file rows have no dedup badge at all. Identical visual
      // shape to pre-Phase-38 builds.
    },
  );

  test.fixme(
    "Pre-pass scan (when wired) — modal proposes 50 hardlink/reflink actions",
    async ({ page: _page, tauri: _tauri }) => {
      // The pre-pass scan modal is a Phase 38 deferred item.
      // When the IPC `start_dedup_scan` lands and emits
      // `dedup-scan-finished { proposed: [...] }`, this test
      // mocks that response and asserts the modal renders the
      // 50-row table with "Apply" / "Cancel" buttons. Today
      // the IPC isn't wired yet — leave fixme.
    },
  );
});
