/**
 * §4.11d Phase 8 partials (Phase 38-followup-3).
 *
 * Frontend coverage:
 *   - Error prompt style toggle (Modal vs Drawer)
 *   - Retry-with-elevated-permissions button on the ErrorModal
 *   - Quick hash button on the collision dialog (no quick_hash IPC
 *     wired today — defer)
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.11d Phase 8 partials", () => {
  test("Settings → Error prompt style: Modal vs Drawer", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue("get_settings", fullSettings());
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    // The Error display mode dropdown is on the General tab.
    const promptStyle = settingsModal.locator("select").filter({
      hasText: /Drawer|Modal.*Drawer|Drawer.*Modal/i,
    });
    await promptStyle.first().selectOption("drawer");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { general?: { errorDisplayMode?: string } } | undefined;
    expect(dto?.general?.errorDisplayMode).toBe("drawer");
  });

  test("Retry with elevated permissions surfaces err-permission-denied", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handles({
      retry_elevated: () => undefined,
      resolve_error: () => undefined,
    });

    // Emit a permission-denied error prompt — the ErrorModal
    // surfaces with the Retry-with-elevated button.
    await tauri.emit("error-raised", {
      id: 11,
      jobId: 1,
      src: "C:\\Windows\\System32\\drivers\\etc\\hosts",
      dst: "C:\\Users\\me\\hosts.bak",
      kind: "permission-denied",
      localizedKey: "err-permission-denied",
      message: "Access is denied. (os error 5)",
      rawOsError: 5,
      createdAtMs: Date.now(),
    });

    const errorModal = page
      .getByRole("alertdialog")
      .filter({ hasText: /permission|denied/i });
    await expect(errorModal.first()).toBeVisible({ timeout: 5_000 });
  });

  test.fixme(
    "Collision modal → Quick hash (SHA-256) renders both digests",
    async ({ page: _page, tauri: _tauri }) => {
      // The `quick_hash` IPC isn't wired today — the conflict
      // dialog has no per-side hash button. When the IPC and
      // button land, this stub becomes:
      //   1. Emit `collision-raised` for two paths.
      //   2. Click the SHA-256 button on each side.
      //   3. Assert both digest strings render distinctly.
    },
  );
});
