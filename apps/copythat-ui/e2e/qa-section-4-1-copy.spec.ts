/**
 * §4.1 Copy — Manual UI golden path.
 *
 * One `test()` per checkbox in `QualityAssuranceChecklist.md`'s
 * §4.1 block. The first test is filled in as a worked example; the
 * rest are `test.fixme()` stubs that document the IPC mocks and
 * DOM assertions the implementation should land.
 */

import { expect, test } from "./fixtures/test";

test.describe("§4.1 Copy", () => {
  test("Drag a 100 MiB file → Drop Stack lights up → copy completes", async ({
    page,
    tauri,
  }) => {
    // Worked-example test. The frontend half of the §4.1 first
    // checkbox: a `drop-received` event surfaces the staging
    // dialog, the user confirms, `start_copy` fires with the
    // expected sources/destination, and a synthetic
    // `job-progress` stream pushes the bar to 100 %.
    interface StartCopyRecord {
      sources: string[];
      destination: string;
    }
    let recordedStartCopy: StartCopyRecord | null = null;

    await tauri.handles({
      start_copy: (args) => {
        recordedStartCopy = {
          sources: (args?.sources as string[]) ?? [],
          destination: (args?.destination as string) ?? "",
        } satisfies StartCopyRecord;
        return [42];
      },
      pick_destination: () => "/tmp/dst",
    });

    await page.goto("/");
    // Boot is async — wait for the empty-state to render so we
    // know the stores have hydrated past `globals`/`list_jobs`.
    await expect(
      page.locator("body").filter({ hasText: /copy that/i }),
    ).toBeVisible();

    // The Tauri webview swallows raw HTML drops and re-emits a
    // typed `drop-received` event back into the frontend. Mirror
    // that contract.
    await tauri.emit("drop-received", {
      paths: ["/tmp/source/100mib.bin"],
    });

    // The staging dialog should open with the dropped path
    // listed. We assert by role rather than by class so the test
    // survives a CSS rename.
    const stagingDialog = page.getByRole("dialog");
    await expect(stagingDialog).toBeVisible({ timeout: 5_000 });

    // Confirm the dialog. The exact button label is i18n'd; the
    // default 'en' bundle uses "Copy". Once a test wants to
    // exercise this against a non-en locale, override the
    // `translations` handler in `tauri.handles`.
    await stagingDialog.getByRole("button", { name: /^copy$/i }).click();

    // The frontend should have invoked `start_copy` with the
    // exact list it surfaced.
    await tauri.waitForCall("start_copy");
    expect(recordedStartCopy).not.toBeNull();
    const recorded = recordedStartCopy as StartCopyRecord | null;
    expect(recorded?.sources).toEqual(["/tmp/source/100mib.bin"]);

    // Walk a synthetic progress timeline. The Rust side emits
    // `job-progress` every 50 ms; we fast-forward to 100 %.
    for (const pct of [0, 25, 50, 75, 100]) {
      const bytesDone = Math.round((100 * 1024 * 1024 * pct) / 100);
      await tauri.emit("job-progress", {
        id: 42,
        bytesDone,
        bytesTotal: 100 * 1024 * 1024,
        filesDone: pct === 100 ? 1 : 0,
        filesTotal: 1,
        rateBps: 32 * 1024 * 1024,
        etaSeconds: pct === 100 ? 0 : (100 - pct) / 25,
      });
    }
    await tauri.emit("job-finished", { id: 42, ok: true });

    // Completion toast or status badge surfaces somewhere in the
    // tree. The job-row for id=42 should now read "succeeded".
    // (If the actual selector lands on something else after a
    // future refactor, update here — the assertion shape is
    // "user sees positive completion feedback".)
    await expect(
      page.getByText(/100\s*%|completed|success/i).first(),
    ).toBeVisible({ timeout: 3_000 });
  });

  test.fixme(
    "Drag a 1 GiB folder → tree-progress bar accumulates → totals bump in footer",
    async ({ page: _page, tauri: _tauri }) => {
      // Same shape as the 100 MiB exemplar above, but the
      // dropped path is a directory and the progress timeline
      // walks `filesDone` from 1 → N. Assert:
      // - DropStagingDialog renders the directory entry.
      // - `start_copy` fires with the directory path.
      // - JobRow's nested progress shows file count incrementing.
      // - Footer totals (`historyTotals` derived store) bumps the
      //   bytes/files counter once the job-finished event lands.
    },
  );

  test.fixme(
    "Drag onto an existing destination → CollisionModal → Overwrite/Skip/Rename",
    async ({ page: _page, tauri: _tauri }) => {
      // Drive each of the three resolution paths in sequence:
      // 1. Emit `collision-prompt` { id, src, dst, sizes } → assert
      //    CollisionModal is visible with both file blocks.
      // 2. Click "Overwrite" → assert
      //    `resolve_collision` invoked with `resolution = "overwrite"`.
      // 3. Repeat for "Skip" → resolution = "skip".
      // 4. Repeat for "Rename" → modal must surface a rename input;
      //    `resolve_collision` carries the entered name.
    },
  );

  test.fixme(
    "Cross-volume copy → engine falls back from reflink to byte-copy",
    async ({ page: _page, tauri: _tauri }) => {
      // Frontend assertion only: a per-file `dedup-strategy` event
      // with `strategy = "Copy"` is rendered as a neutral badge,
      // and no `Reflink` chip is shown. The actual reflink-vs-copy
      // decision lives in the engine and is covered by
      // `cargo test -p copythat-platform`.
    },
  );
});
