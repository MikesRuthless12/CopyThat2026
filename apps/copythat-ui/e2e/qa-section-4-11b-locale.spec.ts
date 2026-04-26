/**
 * §4.11b Locale sync (Phase 38 PWA i18n).
 *
 * Frontend coverage: language picker → setLocale → update_settings.
 * The PWA-side bundle reload is mediated by the Rust runtime
 * (`mobile_locale_push` doesn't exist in the frontend today;
 * pairing pushes the locale through the same WebRTC channel as
 * other settings).
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.11b Locale sync (Phase 38 PWA i18n)", () => {
  test("Switch desktop to French → update_settings carries general.language = fr", async ({
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

    // The Language picker is on the General tab (the default).
    const langSelect = settingsModal.locator("select").first();
    await langSelect.selectOption("fr");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { general?: { language?: string } } | undefined;
    expect(dto?.general?.language).toBe("fr");
  });

  test("Arabic locale flips html dir to rtl", async ({ page, tauri }) => {
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

    const langSelect = settingsModal.locator("select").first();
    await langSelect.selectOption("ar");

    // setLocale("ar") sets html dir="rtl" via theme.ts.
    await page.waitForFunction(
      () => document.documentElement.getAttribute("dir") === "rtl",
      undefined,
      { timeout: 5_000 },
    );
    expect(await page.evaluate(() => document.documentElement.getAttribute("dir"))).toBe("rtl");
  });
});
