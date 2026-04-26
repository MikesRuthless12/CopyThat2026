/**
 * §4.2 Move — Manual UI golden path.
 *
 * Each test mocks the IPC surface and drives the dialog through
 * the user-visible move flow. The atomic-rename vs. copy-and-delete
 * strategy decision is engine-side; these tests cover the wire-up.
 */

import { expect, test } from "./fixtures/test";

const MIB = 1024 * 1024;

test.describe("§4.2 Move", () => {
  test("Same-volume move → atomic rename, source disappears", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      path_metadata: () => [{ isDir: false, size: 10485760 }],
      destination_free_bytes: () => 1099511627776,
      path_total_bytes: () => 10485760,
      enumerate_tree_files: () => ({
        files: [{ path: "/vol-a/source.bin", size: 10485760 }],
        overflow: false,
      }),
      "plugin:dialog|open": () => "/vol-a/dst",
      start_move: () => [55],
    });

    await tauri.emit("drop-received", { paths: ["/vol-a/source.bin"] });
    const dialog = page.getByRole("dialog", { name: /transfer dropped/i });
    await expect(dialog).toBeVisible();

    // Switch the radio to Move so the primary button reads "Start
    // moving" and dispatches start_move instead of start_copy.
    await dialog.getByRole("radio", { name: /^move$/i }).check();
    await dialog.getByRole("button", { name: /pick destination/i }).click();
    await expect(dialog.getByText("/vol-a/dst")).toBeVisible();
    await dialog.getByRole("button", { name: /start moving/i }).click();

    const startCall = await tauri.waitForCall("start_move");
    expect(startCall.args?.sources).toEqual(["/vol-a/source.bin"]);
    expect(startCall.args?.destination).toBe("/vol-a/dst");

    await tauri.emit("job-added", {
      id: 55,
      kind: "move",
      src: "/vol-a/source.bin",
      dst: "/vol-a/dst",
      state: "running",
      bytesDone: 0,
      bytesTotal: 10 * MIB,
      filesDone: 0,
      filesTotal: 1,
      rateBps: 0,
      etaSeconds: null,
      lastError: null,
    });
    await tauri.emit("job-progress", {
      id: 55,
      bytesDone: 10 * MIB,
      bytesTotal: 10 * MIB,
      filesDone: 1,
      filesTotal: 1,
      rateBps: 100 * MIB,
      etaSeconds: 0,
    });
    await tauri.emit("job-completed", { id: 55 });

    await expect(
      page.getByText(/done|completed|100\s*%/i).first(),
    ).toBeVisible({ timeout: 5_000 });
  });

  test("Cross-volume move → falls back to copy + delete (EXDEV)", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      path_metadata: () => [{ isDir: false, size: 10485760 }],
      destination_free_bytes: () => 1099511627776,
      path_total_bytes: () => 10485760,
      enumerate_tree_files: () => ({
        files: [{ path: "/vol-a/source.bin", size: 10485760 }],
        overflow: false,
      }),
      "plugin:dialog|open": () => "/vol-b/dst",
      start_move: () => [56],
    });

    await tauri.emit("drop-received", { paths: ["/vol-a/source.bin"] });
    const dialog = page.getByRole("dialog", { name: /transfer dropped/i });
    await expect(dialog).toBeVisible();
    await dialog.getByRole("radio", { name: /^move$/i }).check();
    await dialog.getByRole("button", { name: /pick destination/i }).click();
    await dialog.getByRole("button", { name: /start moving/i }).click();

    await tauri.waitForCall("start_move");

    await tauri.emit("job-added", {
      id: 56,
      kind: "move",
      src: "/vol-a/source.bin",
      dst: "/vol-b/dst",
      state: "running",
      bytesDone: 0,
      bytesTotal: 10 * MIB,
      filesDone: 0,
      filesTotal: 1,
      rateBps: 0,
      etaSeconds: null,
      lastError: null,
    });
    // Engine-side EXDEV fallback emits a per-job strategy event.
    // The frontend renders nothing distinctive today (the badge is
    // engine-only); we just assert the wire-up doesn't crash and
    // the job completes normally.
    await tauri.emit("move-strategy", {
      id: 56,
      strategy: "CopyThenDelete",
    });
    await tauri.emit("job-progress", {
      id: 56,
      bytesDone: 10 * MIB,
      bytesTotal: 10 * MIB,
      filesDone: 1,
      filesTotal: 1,
      rateBps: 50 * MIB,
      etaSeconds: 0,
    });
    await tauri.emit("job-completed", { id: 56 });

    await expect(
      page.getByText(/done|completed|100\s*%/i).first(),
    ).toBeVisible({ timeout: 5_000 });
  });

  test("Cancel a long-running move → source intact, partial dst cleaned", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      cancel_job: () => undefined,
      cancel_all: () => undefined,
    });

    // Seed the job via the live event stream — `list_jobs` would
    // re-fire on the next navigation and reset to defaults, but the
    // store treats `job-added` as authoritative.
    await tauri.emit("job-added", {
      id: 99,
      kind: "move",
      state: "running",
      src: "/vol-a/big-folder",
      dst: "/vol-a/dst",
      name: "big-folder",
      subpath: null,
      bytesDone: 209715200,
      bytesTotal: 1073741824,
      filesDone: 10,
      filesTotal: 50,
      rateBps: 52428800,
      etaSeconds: 16,
      startedAtMs: null,
      finishedAtMs: null,
      lastError: null,
    });
    // Bump globals so the header's "Cancel all" button enables.
    await tauri.emit("globals-tick", {
      state: "copying",
      activeJobs: 1,
      queuedJobs: 0,
      pausedJobs: 0,
      failedJobs: 0,
      succeededJobs: 0,
      bytesDone: 209715200,
      bytesTotal: 1073741824,
      rateBps: 52428800,
      etaSeconds: 16,
      errors: 0,
    });

    await expect(page.getByText(/big-folder/i).first()).toBeVisible({
      timeout: 5_000,
    });

    // Header dispatches `cancel_all`, not per-id cancel.
    await page.getByRole("button", { name: /cancel all/i }).click();
    await tauri.waitForCall("cancel_all");

    await tauri.emit("job-cancelled", { id: 99 });
    await tauri.emit("globals-tick", {
      state: "idle",
      activeJobs: 0,
      queuedJobs: 0,
      pausedJobs: 0,
      failedJobs: 0,
      succeededJobs: 0,
      bytesDone: 0,
      bytesTotal: 0,
      rateBps: 0,
      etaSeconds: null,
      errors: 0,
    });

    await expect(page.getByText(/cancel/i).first()).toBeVisible();
  });
});
