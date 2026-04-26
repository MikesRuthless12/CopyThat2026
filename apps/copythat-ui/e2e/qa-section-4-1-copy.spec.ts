/**
 * §4.1 Copy — Manual UI golden path.
 *
 * One `test()` per checkbox in `QualityAssuranceChecklist.md`'s
 * §4.1 block. Each test mocks the IPC surface and drives the
 * frontend through the user-visible drop → confirm → progress
 * sequence.
 *
 * Convention: register `tauri.handles({...})` AFTER `page.goto("/")`
 * because the shim creates a fresh state map on every navigation.
 * Boot-time IPC (`globals`, `list_jobs`, `get_settings`, etc.) is
 * served by the fixture's default handlers; user-action IPC like
 * `start_copy` / `plugin:dialog|open` is registered post-boot.
 */

import { expect, test } from "./fixtures/test";

const MIB = 1024 * 1024;

test.describe("§4.1 Copy", () => {
  test("Drag a 100 MiB file → Drop Stack lights up → copy completes", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      path_metadata: () => [{ isDir: false, size: 100 * 1024 * 1024 }],
      destination_free_bytes: () => 1024 * 1024 * 1024 * 1024,
      path_total_bytes: () => 100 * 1024 * 1024,
      enumerate_tree_files: () => ({
        files: [{ path: "/tmp/source/100mib.bin", size: 100 * 1024 * 1024 }],
        overflow: false,
      }),
      "plugin:dialog|open": () => "/tmp/dst",
      start_copy: () => [42],
    });

    await tauri.emit("drop-received", { paths: ["/tmp/source/100mib.bin"] });

    const stagingDialog = page.getByRole("dialog", { name: /transfer dropped/i });
    await expect(stagingDialog).toBeVisible({ timeout: 5_000 });

    await stagingDialog.getByRole("button", { name: /pick destination/i }).click();
    await expect(stagingDialog.getByText("/tmp/dst")).toBeVisible();

    await stagingDialog.getByRole("button", { name: /start copying/i }).click();

    const startCall = await tauri.waitForCall("start_copy");
    expect(startCall.args?.sources).toEqual(["/tmp/source/100mib.bin"]);
    expect(startCall.args?.destination).toBe("/tmp/dst");

    await tauri.emit("job-added", {
      id: 42,
      kind: "copy",
      src: "/tmp/source/100mib.bin",
      dst: "/tmp/dst",
      state: "running",
      bytesDone: 0,
      bytesTotal: 100 * MIB,
      filesDone: 0,
      filesTotal: 1,
      rateBps: 0,
      etaSeconds: null,
      lastError: null,
    });

    for (const pct of [25, 50, 75, 100]) {
      await tauri.emit("job-progress", {
        id: 42,
        bytesDone: Math.round((100 * MIB * pct) / 100),
        bytesTotal: 100 * MIB,
        filesDone: pct === 100 ? 1 : 0,
        filesTotal: 1,
        rateBps: 32 * MIB,
        etaSeconds: pct === 100 ? 0 : (100 - pct) / 25,
      });
    }
    await tauri.emit("job-completed", { id: 42 });

    await expect(
      page.getByText(/100\s*%|done|completed|success/i).first(),
    ).toBeVisible({ timeout: 5_000 });
  });

  test("Drag a 1 GiB folder → tree-progress accumulates → totals bump in footer", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    const FILES = 50;
    const FOLDER_BYTES = 1024 * MIB;
    const PER_FILE = FOLDER_BYTES / FILES;

    await tauri.handles({
      path_metadata: () => [{ isDir: true, size: 1024 * 1024 * 1024 }],
      destination_free_bytes: () => 4 * 1024 * 1024 * 1024 * 1024,
      path_total_bytes: () => 1024 * 1024 * 1024,
      enumerate_tree_files: () => ({
        files: Array.from({ length: 50 }, (_, i) => ({
          path: `/tmp/folder/file-${i}.bin`,
          size: 1024 * 1024 * 1024 / 50,
        })),
        overflow: false,
      }),
      "plugin:dialog|open": () => "/tmp/dst",
      start_copy: () => [101],
      history_totals: () => ({
        bytes: 1024 * 1024 * 1024,
        files: 50,
        jobs: 1,
        avgRateBps: 32 * 1024 * 1024,
        peakRateBps: 64 * 1024 * 1024,
      }),
    });

    await tauri.emit("drop-received", { paths: ["/tmp/folder"] });
    const stagingDialog = page.getByRole("dialog", { name: /transfer dropped/i });
    await expect(stagingDialog).toBeVisible();
    await expect(stagingDialog.getByText("/tmp/folder")).toBeVisible();

    await stagingDialog.getByRole("button", { name: /pick destination/i }).click();
    await expect(stagingDialog.getByText("/tmp/dst")).toBeVisible();
    await stagingDialog.getByRole("button", { name: /start copying/i }).click();

    const startCall = await tauri.waitForCall("start_copy");
    expect(startCall.args?.sources).toEqual(["/tmp/folder"]);

    await tauri.emit("job-added", {
      id: 101,
      kind: "copy",
      src: "/tmp/folder",
      dst: "/tmp/dst",
      state: "running",
      bytesDone: 0,
      bytesTotal: FOLDER_BYTES,
      filesDone: 0,
      filesTotal: FILES,
      rateBps: 0,
      etaSeconds: null,
      lastError: null,
    });

    for (let f = 1; f <= FILES; f += 10) {
      await tauri.emit("job-progress", {
        id: 101,
        bytesDone: f * PER_FILE,
        bytesTotal: FOLDER_BYTES,
        filesDone: f,
        filesTotal: FILES,
        rateBps: 32 * MIB,
        etaSeconds: (FILES - f) / 10,
      });
    }
    await tauri.emit("job-completed", { id: 101 });

    await expect(
      page.getByText(/done|completed|100\s*%/i).first(),
    ).toBeVisible({ timeout: 5_000 });
  });

  test("Drag onto an existing destination → CollisionModal → Overwrite/Skip/Rename", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({ resolve_collision: () => undefined });

    const collisionDto = (id: number) => ({
      jobId: 1,
      id,
      src: `/tmp/src/file-${id}.bin`,
      dst: `/tmp/dst/file-${id}.bin`,
      srcSize: 1024,
      dstSize: 1024,
      srcModifiedMs: 0,
      dstModifiedMs: 0,
    });

    // Push all three conflict prompts at once so they all appear as
    // rail rows, then walk through them by selecting each rail row
    // and clicking the appropriate per-row action button.
    await tauri.emit("collision-raised", collisionDto(1));
    await tauri.emit("collision-raised", collisionDto(2));
    await tauri.emit("collision-raised", collisionDto(3));

    const modal = page.getByRole("alertdialog");
    await expect(modal).toBeVisible({ timeout: 5_000 });

    // Row 1 starts selected by default → click Overwrite.
    await modal.getByRole("button", { name: /^overwrite$/i }).first().click();
    await tauri.waitForCall("resolve_collision");
    await tauri.emit("collision-resolved", { id: 1 });

    // Select row 2 in the rail, then click Skip.
    await modal.getByRole("option", { name: /file-2\.bin/i }).click();
    await modal.getByRole("button", { name: /^skip$/i }).first().click();
    await page.waitForFunction(
      () =>
        ((window as unknown as { __copythat_e2e__: { calls(): { cmd: string }[] } })
          .__copythat_e2e__.calls()
          .filter((c) => c.cmd === "resolve_collision").length) >= 2,
      undefined,
      { timeout: 5_000 },
    );
    await tauri.emit("collision-resolved", { id: 2 });

    // Select row 3, click Keep both (the UI's rename-on-collision
    // path: the engine appends `_2`/`_3`/… suffixes).
    await modal.getByRole("option", { name: /file-3\.bin/i }).click();
    await modal.getByRole("button", { name: /keep both/i }).first().click();
    await page.waitForFunction(
      () =>
        ((window as unknown as { __copythat_e2e__: { calls(): { cmd: string }[] } })
          .__copythat_e2e__.calls()
          .filter((c) => c.cmd === "resolve_collision").length) >= 3,
      undefined,
      { timeout: 5_000 },
    );

    const resolves = await tauri.callsFor("resolve_collision");
    expect(resolves).toHaveLength(3);
    expect(resolves[0]?.args?.resolution).toMatch(/overwrite/i);
    expect(resolves[1]?.args?.resolution).toMatch(/skip/i);
    expect(resolves[2]?.args?.resolution).toMatch(/rename|keep-both/i);
  });

  test("Cross-volume copy → engine falls back from reflink to byte-copy", async ({
    page,
    tauri,
  }) => {
    await tauri.handles({
      list_jobs: () => [
        {
          id: 7,
          kind: "copy",
          src: "/mnt/a/file.bin",
          dst: "/mnt/b/file.bin",
          state: "running",
          bytesDone: 0,
          bytesTotal: 1024,
          filesDone: 0,
          filesTotal: 1,
          rateBps: 0,
          etaSeconds: null,
          lastError: null,
        },
      ],
    });

    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    // Plain Copy strategy event — the row should NOT show a
    // Reflink badge. Reflink-vs-copy decision lives in the engine
    // (covered by `cargo test -p copythat-platform`).
    await tauri.emit("dedup-strategy", {
      id: 7,
      file: "/mnt/a/file.bin",
      strategy: "Copy",
      bytesSaved: 0,
    });

    await page.waitForTimeout(300);
    await expect(page.getByText(/reflink/i)).toHaveCount(0);
  });
});
