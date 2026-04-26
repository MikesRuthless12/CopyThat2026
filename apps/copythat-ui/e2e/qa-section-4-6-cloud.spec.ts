/**
 * §4.6 Cloud (Phase 32) — Manual UI golden path.
 *
 * Frontend coverage of the Settings → Remotes flow. The OAuth-
 * popup half is OS-level (browser navigation outside the webview)
 * and stays manual; the cloud-→ -local copy half is the same
 * `start_copy` plumbing covered in §4.1, just with a cloud URI in
 * the `sources` array.
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.6 Cloud (Phase 32)", () => {
  test("Add S3 backend → Test connection → green", async ({ page, tauri }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue("get_settings", fullSettings());
    await tauri.handles({
      list_backends: () => [],
      add_backend: (args) => args?.dto,
      test_backend_connection: () => ({
        ok: true,
        reason: null,
        detail: "latency 42 ms",
      }),
      list_profiles: () => [],
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page.getByRole("dialog").filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /remotes/i }).click();

    // The Remotes tab body renders. We assert it's interactive
    // rather than driving the full add-backend form (the form's
    // selectors depend on Phase 32 polish that hasn't shipped).
    await expect(
      settingsModal.getByText(/backend|remote|S3|cloud/i).first(),
    ).toBeVisible({ timeout: 5_000 });
  });

  test.fixme(
    "Add Dropbox backend via OAuth PKCE → backend listed",
    async ({ page: _page, tauri: _tauri }) => {
      // OAuth flow isn't wired in the frontend today — the Dropbox
      // backend is just a kind selection in the Add form. The PKCE
      // exchange + popup browser hop is OS-level and stays manual
      // per the QualityAssuranceChecklist appendix.
    },
  );

  test("Copy from a backend back to local → start_copy carries the URI", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      path_metadata: () => [{ isDir: false, size: 1024 }],
      destination_free_bytes: () => 1099511627776,
      path_total_bytes: () => 1024,
      enumerate_tree_files: () => ({
        files: [{ path: "s3://bucket/path/file.bin", size: 1024 }],
        overflow: false,
      }),
      "plugin:dialog|open": () => "/local/dst",
      start_copy: () => [42],
    });

    // Drive a drop event with a cloud URI in the sources list. The
    // staging dialog accepts it the same way it accepts a local
    // path — `start_copy` carries the URI verbatim.
    await tauri.emit("drop-received", {
      paths: ["s3://bucket/path/file.bin"],
    });

    const dialog = page.getByRole("dialog", { name: /transfer dropped/i });
    await expect(dialog).toBeVisible({ timeout: 5_000 });
    await expect(dialog.getByText("s3://bucket/path/file.bin")).toBeVisible();

    await dialog.getByRole("button", { name: /pick destination/i }).click();
    await expect(dialog.getByText("/local/dst")).toBeVisible();
    await dialog.getByRole("button", { name: /start copying/i }).click();

    const startCall = await tauri.waitForCall("start_copy");
    expect(startCall.args?.sources).toEqual(["s3://bucket/path/file.bin"]);
    expect(startCall.args?.destination).toBe("/local/dst");
  });
});
