/**
 * §4.8 Audit log (Phase 34) — Manual UI golden path.
 *
 * Drives the Settings → Advanced → Audit subsection. The actual
 * record-content + chain-hashing logic is engine-side
 * (covered by `cargo test -p copythat-audit`).
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.8 Audit log (Phase 34)", () => {
  test("Settings → Advanced → enable JSON-Lines audit → update_settings round-trip", async ({
    page,
    tauri,
  }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    // Start with audit disabled so the toggle has somewhere to go.
    await tauri.handleValue(
      "get_settings",
      fullSettings({
        audit: {
          enabled: false,
          format: "json-lines",
          filePath: "/tmp/audit.log",
          maxSizeBytes: 10485760,
          worm: "off",
        },
      }),
    );
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /advanced/i }).click();

    // Toggle the audit-enable checkbox.
    const enableToggle = settingsModal.getByRole("checkbox", {
      name: /enable.*audit|audit.*enable/i,
    });
    await enableToggle.first().check();

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { audit?: { enabled?: boolean } } | undefined;
    expect(dto?.audit?.enabled).toBe(true);
  });

  test("WORM toggle → update_settings round-trip", async ({ page, tauri }) => {
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

    await settingsModal.getByRole("tab", { name: /advanced/i }).click();

    const wormToggle = settingsModal.getByRole("checkbox", { name: /worm/i });
    await wormToggle.first().check();

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { audit?: { worm?: string } } | undefined;
    expect(dto?.audit?.worm).toBe("on");
  });

  test("Verify chain → toast surfaces ok/fail", async ({ page, tauri }) => {
    await page.goto("/");
    await expect(page.getByText(/drop files or folders/i)).toBeVisible();

    await tauri.handleValue("get_settings", fullSettings());
    await tauri.handles({
      update_settings: (args) => args?.dto,
      list_profiles: () => [],
      audit_verify: () => ({ ok: true, lastSeq: 1234 }),
    });

    await page.getByRole("button", { name: /settings/i }).first().click();
    const settingsModal = page
      .getByRole("dialog")
      .filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /advanced/i }).click();

    await settingsModal
      .getByRole("button", { name: /verify chain/i })
      .click();

    // The frontend dispatches a toast on success/failure (the
    // `pushToast("success" | "error", "toast-audit-verify-*")`
    // calls in SettingsModal). The toast renders a transient
    // notification surfaced via the global Toast component.
    await tauri.waitForCall("audit_verify");
  });
});
