<script lang="ts">
  // Phase 49k — Repository setup/connect wizard. Create a new repository or
  // connect to an existing one, switch the active repo, disconnect (list-only),
  // and rotate the access passphrase. The passphrase is an ACCESS gate, not
  // at-rest encryption (that arrives in a later release).
  import { open as openDialog } from "@tauri-apps/plugin-dialog";

  import { i18nVersion, t } from "../i18n";
  import {
    repositoryChangePassword,
    repositoryConnect,
    repositoryCreate,
    repositoryDisconnect,
    repositorySetActive,
  } from "../ipc";
  import {
    closeRepositoryWizard,
    pushToast,
    refreshRepos,
    repos,
    repositoryWizardOpen,
  } from "../stores";

  type Tab = "create" | "connect";
  let tab = $state<Tab>("create");
  let name = $state("");
  let path = $state("");
  let password = $state("");
  let busy = $state(false);

  let showChangePass = $state(false);
  let oldPass = $state("");
  let newPass = $state("");

  function friendly(e: unknown): string {
    const msg = e instanceof Error ? e.message : String(e);
    if (msg.includes("already initialised")) return t("repo-error-exists");
    if (msg.includes("no repository found")) return t("repo-error-not-found");
    if (msg.includes("passphrase") || msg.includes("locked")) return t("repo-error-bad-pass");
    return msg;
  }

  async function browse() {
    const picked = await openDialog({ directory: true, multiple: false });
    if (typeof picked === "string") path = picked;
  }

  async function submit() {
    if (!name.trim() || !path.trim()) return;
    busy = true;
    try {
      const pw = password.length > 0 ? password : null;
      if (tab === "create") {
        await repositoryCreate(name.trim(), path.trim(), pw);
        pushToast("success", t("repo-toast-created", { name: name.trim() }));
      } else {
        await repositoryConnect(name.trim(), path.trim(), pw);
        pushToast("success", t("repo-toast-connected", { name: name.trim() }));
      }
      name = "";
      path = "";
      password = "";
      await refreshRepos();
    } catch (e) {
      pushToast("error", friendly(e));
    } finally {
      busy = false;
    }
  }

  // Reuses the form's passphrase field, so a gated repo can be unlocked by
  // typing its passphrase above before clicking the repo to switch.
  async function switchTo(id: string) {
    try {
      const pw = password.length > 0 ? password : null;
      await repositorySetActive(id, pw);
      password = "";
      await refreshRepos();
    } catch (e) {
      pushToast("error", friendly(e));
    }
  }

  async function disconnect(id: string) {
    try {
      await repositoryDisconnect(id);
      await refreshRepos();
    } catch (e) {
      pushToast("error", friendly(e));
    }
  }

  async function changePass() {
    try {
      await repositoryChangePassword(oldPass.length > 0 ? oldPass : null, newPass);
      oldPass = "";
      newPass = "";
      showChangePass = false;
      pushToast("success", t("repo-toast-pass-changed"));
    } catch (e) {
      pushToast("error", friendly(e));
    }
  }
</script>

{#if $repositoryWizardOpen}
  <div class="scrim" role="presentation" onclick={closeRepositoryWizard}></div>
  <div class="modal" role="dialog" aria-modal="true" aria-label={t("repo-wizard-title")}>
    {#key $i18nVersion}
      <header>
        <h2>{t("repo-wizard-title")}</h2>
        <button type="button" class="close" onclick={closeRepositoryWizard}>×</button>
      </header>

      <div class="tabs" role="tablist">
        <button type="button" class:active={tab === "create"} onclick={() => (tab = "create")}>
          {t("repo-wizard-create-tab")}
        </button>
        <button type="button" class:active={tab === "connect"} onclick={() => (tab = "connect")}>
          {t("repo-wizard-connect-tab")}
        </button>
      </div>

      <div class="form">
        <label>
          {t("repo-field-name")}
          <input type="text" bind:value={name} />
        </label>
        <label>
          {t("repo-field-path")}
          <span class="path-row">
            <input type="text" bind:value={path} />
            <button type="button" onclick={browse}>{t("repo-action-browse")}</button>
          </span>
        </label>
        <label>
          {t("repo-field-password")}
          <input type="password" bind:value={password} />
        </label>
        <p class="note">{t("repo-note-no-encryption")}</p>
        <button type="button" class="primary" disabled={busy} onclick={submit}>
          {tab === "create" ? t("repo-action-create") : t("repo-action-connect")}
        </button>
      </div>

      {#if $repos.length > 0}
        <ul class="repo-list">
          {#each $repos as r (r.id)}
            <li class:active={r.active}>
              <button
                type="button"
                class="repo-name"
                disabled={r.active}
                title={t("repo-switcher-label")}
                onclick={() => switchTo(r.id)}
              >
                {r.name}{#if r.active}&nbsp;✓{/if}
              </button>
              <button type="button" class="forget" onclick={() => disconnect(r.id)}>
                {t("repo-action-forget")}
              </button>
            </li>
          {/each}
        </ul>
      {/if}

      <div class="change-pass">
        <button type="button" class="link" onclick={() => (showChangePass = !showChangePass)}>
          {t("repo-action-change-pass")}
        </button>
        {#if showChangePass}
          <label>
            {t("repo-password-old")}
            <input type="password" bind:value={oldPass} />
          </label>
          <label>
            {t("repo-password-new")}
            <input type="password" bind:value={newPass} />
          </label>
          <button type="button" onclick={changePass}>{t("repo-action-change-pass")}</button>
        {/if}
      </div>
    {/key}
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    z-index: 50;
    border: 0;
  }
  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: min(460px, 94vw);
    max-height: 88vh;
    overflow-y: auto;
    background: var(--surface, #1e1e1e);
    color: var(--text, #eee);
    border-radius: 8px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    z-index: 51;
    padding: 16px;
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }
  header h2 {
    margin: 0;
    font-size: 1.05rem;
  }
  .close {
    background: none;
    border: 0;
    color: inherit;
    font-size: 1.4rem;
    line-height: 1;
    cursor: pointer;
  }
  .tabs {
    display: flex;
    gap: 4px;
    margin-bottom: 12px;
  }
  .tabs button {
    flex: 1;
    padding: 6px;
    background: var(--border, #2a2a2a);
    border: 0;
    color: inherit;
    border-radius: 4px;
    cursor: pointer;
  }
  .tabs button.active {
    background: var(--accent, #4c8bf5);
    color: #fff;
  }
  .form label,
  .change-pass label {
    display: block;
    font-size: 0.85rem;
    margin-bottom: 8px;
  }
  .form input,
  .change-pass input {
    display: block;
    width: 100%;
    box-sizing: border-box;
    margin-top: 3px;
    padding: 5px 7px;
    background: var(--bg, #141414);
    color: inherit;
    border: 1px solid var(--border, #333);
    border-radius: 4px;
  }
  .path-row {
    display: flex;
    gap: 6px;
  }
  .path-row input {
    flex: 1;
  }
  .path-row button,
  .change-pass button {
    background: var(--border, #2a2a2a);
    border: 0;
    color: inherit;
    border-radius: 4px;
    padding: 5px 10px;
    cursor: pointer;
  }
  .note {
    font-size: 0.75rem;
    opacity: 0.6;
    margin: 4px 0 10px;
  }
  .primary {
    width: 100%;
    padding: 8px;
    background: var(--accent, #4c8bf5);
    color: #fff;
    border: 0;
    border-radius: 4px;
    cursor: pointer;
  }
  .primary:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .repo-list {
    list-style: none;
    margin: 14px 0;
    padding: 0;
    border-top: 1px solid var(--border, #333);
  }
  .repo-list li {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid var(--border, #2a2a2a);
  }
  .repo-name {
    background: none;
    border: 0;
    color: inherit;
    cursor: pointer;
    font-size: 0.9rem;
    text-align: left;
    flex: 1;
  }
  .repo-name:disabled {
    cursor: default;
    opacity: 0.85;
  }
  .repo-list li.active {
    font-weight: 600;
  }
  .forget {
    background: none;
    border: 1px solid var(--border, #444);
    color: inherit;
    border-radius: 4px;
    font-size: 0.75rem;
    padding: 2px 8px;
    cursor: pointer;
  }
  .change-pass {
    margin-top: 10px;
  }
  .link {
    background: none;
    border: 0;
    color: var(--accent, #4c8bf5);
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0;
  }
</style>
