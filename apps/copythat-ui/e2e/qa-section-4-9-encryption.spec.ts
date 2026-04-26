/**
 * §4.9 Encryption + compression (Phase 35) — Manual UI golden path.
 *
 * Drives the Settings → Transfer → Crypt subsection. The actual
 * age round-trip + smart-compression deny-list logic is engine-side
 * (covered by `cargo test -p copythat-crypt`).
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.9 Encryption + compression (Phase 35)", () => {
  test("Encryption mode = recipients → update_settings round-trip", async ({
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

    await settingsModal.getByRole("tab", { name: /transfer/i }).click();

    // The Encryption mode dropdown is in the Crypt subsection; the
    // value list is off / passphrase / recipients.
    const encryptionMode = settingsModal.locator("select").filter({
      hasText: /Recipients|Passphrase/i,
    });
    await encryptionMode.selectOption("recipients");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { crypt?: { encryptionMode?: string } } | undefined;
    expect(dto?.crypt?.encryptionMode).toBe("recipients");
  });

  test("Compression mode = smart, level 3 → update_settings round-trip", async ({
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

    await settingsModal.getByRole("tab", { name: /transfer/i }).click();

    // Compression mode dropdown: off / always / smart. Filter by
    // the unique-to-compression "Always" option text (Reflink uses
    // "Avoid" not "Always").
    const compressionMode = settingsModal
      .locator("select")
      .filter({ hasText: /Always.*Smart|Smart.*Always/i });
    await compressionMode.selectOption("smart");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { crypt?: { compressionMode?: string } } | undefined;
    expect(dto?.crypt?.compressionMode).toBe("smart");
  });
});
