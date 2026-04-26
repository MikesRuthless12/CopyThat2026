/**
 * §4.4 Secure delete — Manual UI golden path.
 *
 * Note: the per-row "Right-click → Secure Delete" context-menu entry
 * isn't wired in the frontend yet (App.svelte's contextItemsFor
 * builds a static menu of pause/resume/cancel/remove/reveal). The
 * Settings → Secure delete tab does exist and is the only frontend
 * surface today; the engine-side shred logic is covered by
 * `cargo test -p copythat-secure-delete`.
 */

import { expect, test } from "./fixtures/test";
import { fullSettings } from "./fixtures/settings";

test.describe("§4.4 Secure delete", () => {
  test("Settings → Secure delete tab → method dropdown round-trips DoD-3", async ({
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
    const settingsModal = page.getByRole("dialog").filter({ hasText: /settings/i });
    await expect(settingsModal).toBeVisible({ timeout: 5_000 });

    await settingsModal.getByRole("tab", { name: /secure delete/i }).click();

    // The tab body has a Method dropdown (zero / random / dod-3-pass / etc).
    const methodSelect = settingsModal.locator("select").filter({
      hasText: /DoD 5220|3 passes/i,
    });
    await methodSelect.selectOption("dod-3-pass");

    const updateCall = await tauri.waitForCall("update_settings");
    const dto = updateCall.args?.dto as { secureDelete?: { method?: string } } | undefined;
    expect(dto?.secureDelete?.method).toBe("dod-3-pass");
  });

  test.fixme(
    "Right-click → Secure Delete (DoD 3-pass) → confirmation → file gone",
    async ({ page: _page, tauri: _tauri }) => {
      // UI defect: App.svelte's contextItemsFor does not include a
      // "Secure Delete" entry — the menu is just pause/resume/
      // cancel/remove/reveal. When the per-row secure-delete UI
      // lands, this stub becomes:
      //   1. Seed a completed JobRow via list_jobs / job-added.
      //   2. Right-click the row → ContextMenu opens with
      //      "Secure Delete" → click it.
      //   3. Confirmation modal renders with the configured shred
      //      method.
      //   4. Confirm → assert `start_secure_delete` invoked.
      // Until then, the user-action surface lives at the CLI
      // (`copythat shred`) only.
    },
  );

  test.fixme(
    "On a CoW filesystem → SSD-aware refusal explanation",
    async ({ page: _page, tauri: _tauri }) => {
      // Deferred per the QualityAssuranceChecklist appendix —
      // requires a real Btrfs / APFS / ReFS filesystem to exercise
      // the engine's CoW detection. The frontend's role is just
      // surfacing the `err-shred-cow-refusal` ErrorPrompt; that
      // surface is otherwise identical to the §4.3 verify-failed
      // ErrorModal already covered.
    },
  );
});
