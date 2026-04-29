<script lang="ts">
  // Phase 32b — Settings → Remotes tab.
  //
  // Lists configured cloud backends, exposes Add-backend / Remove /
  // Test-connection flows, and surfaces per-kind config forms. Secrets
  // are collected once at save time and flow straight through to the
  // OS keychain via `add_backend` / `update_backend`; they never
  // round-trip back on `list_backends`.
  import { onMount } from "svelte";
  import { t } from "../i18n";
  import {
    addBackend,
    defaultOffloadOpts,
    emptyBackendConfig,
    listBackends,
    removeBackend,
    renderOffloadTemplate,
    testBackendConnection,
    updateBackend,
    type BackendDto,
    type OffloadOptsDto,
    type OffloadTemplateFormat,
    type TestConnectionResult,
  } from "../ipc";

  // Phase 40 Part B — cloud-VM offload-template wizard. Lets the user
  // pair two configured backends + a target format and render a
  // deployment template they paste into their cloud's console. The
  // wizard never invokes cloud APIs itself; the rendered string is
  // the only side effect.
  let offloadShow = $state(false);
  let offloadFormat = $state<OffloadTemplateFormat>("cloud-init");
  let offloadSrcName = $state("");
  let offloadDstName = $state("");
  let offloadOpts = $state<OffloadOptsDto>(defaultOffloadOpts());
  let offloadOutput = $state("");
  let offloadBusy = $state(false);
  let offloadCopied = $state(false);

  function offloadOpen() {
    offloadShow = true;
    offloadFormat = "cloud-init";
    offloadOutput = "";
    offloadCopied = false;
    offloadOpts = defaultOffloadOpts();
    if (backends.length >= 2) {
      offloadSrcName = backends[0].name;
      offloadDstName = backends[1].name;
    } else if (backends.length === 1) {
      offloadSrcName = backends[0].name;
      offloadDstName = backends[0].name;
    }
  }

  async function offloadRender() {
    const src = backends.find((b) => b.name === offloadSrcName);
    const dst = backends.find((b) => b.name === offloadDstName);
    if (!src || !dst) {
      offloadOutput = "";
      return;
    }
    offloadBusy = true;
    offloadCopied = false;
    try {
      offloadOutput = await renderOffloadTemplate(
        offloadFormat,
        src,
        dst,
        offloadOpts,
      );
    } catch (e) {
      offloadOutput = String(e);
    } finally {
      offloadBusy = false;
    }
  }

  async function offloadCopyToClipboard() {
    if (!offloadOutput) return;
    try {
      await navigator.clipboard.writeText(offloadOutput);
      offloadCopied = true;
      setTimeout(() => {
        offloadCopied = false;
      }, 2000);
    } catch {
      // Clipboard write can fail on locked-down platforms; swallow
      // and let the user copy manually from the textarea.
    }
  }

  // Twelve kinds in display order. The `is_enabled` flag on each
  // returned BackendDto tells us whether the build lights the OpenDAL
  // driver — SFTP is config-only on Windows today (openssh upstream).
  const ALL_KINDS: { wire: string; label: string }[] = [
    { wire: "s3", label: "backend-s3" },
    { wire: "r2", label: "backend-r2" },
    { wire: "b2", label: "backend-b2" },
    { wire: "azure-blob", label: "backend-azure-blob" },
    { wire: "gcs", label: "backend-gcs" },
    { wire: "onedrive", label: "backend-onedrive" },
    { wire: "google-drive", label: "backend-google-drive" },
    { wire: "dropbox", label: "backend-dropbox" },
    { wire: "webdav", label: "backend-webdav" },
    { wire: "sftp", label: "backend-sftp" },
    { wire: "ftp", label: "backend-ftp" },
    { wire: "local-fs", label: "backend-local-fs" },
  ];

  let backends = $state<BackendDto[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Form state. `editingName` tracks whether the form is editing an
  // existing entry (name is read-only) or adding a new one.
  let showForm = $state(false);
  let editingName = $state<string | null>(null);
  let formName = $state("");
  let formKind = $state("local-fs");
  let formConfig = $state(emptyBackendConfig());
  let formSecret = $state("");
  let formBusy = $state(false);

  // Test-connection state keyed by backend name.
  let testResults = $state<Record<string, TestConnectionResult>>({});
  let testing = $state<Record<string, boolean>>({});

  async function refresh() {
    loading = true;
    error = null;
    try {
      backends = await listBackends();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(refresh);

  function resetForm() {
    showForm = false;
    editingName = null;
    formName = "";
    formKind = "local-fs";
    formConfig = emptyBackendConfig();
    formSecret = "";
  }

  function openAdd() {
    resetForm();
    showForm = true;
  }

  function openEdit(b: BackendDto) {
    editingName = b.name;
    formName = b.name;
    formKind = b.kind;
    formConfig = { ...b.config };
    formSecret = "";
    showForm = true;
  }

  async function onSave() {
    formBusy = true;
    try {
      const dto: BackendDto = {
        name: formName.trim(),
        kind: formKind,
        config: { ...formConfig },
        enabledInBuild: true,
      };
      const secret = formSecret.trim() ? formSecret : null;
      if (editingName) {
        await updateBackend(dto, secret);
      } else {
        await addBackend(dto, secret);
      }
      resetForm();
      await refresh();
    } catch (e) {
      error = String(e);
    } finally {
      formBusy = false;
    }
  }

  async function onRemove(name: string) {
    try {
      await removeBackend(name);
      delete testResults[name];
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  async function onTest(name: string) {
    testing[name] = true;
    try {
      testResults[name] = await testBackendConnection(name);
    } catch (e) {
      testResults[name] = { ok: false, reason: "network", detail: String(e) };
    } finally {
      testing[name] = false;
    }
  }

  // Per-kind form-field gating. Keeps the form from asking for an S3
  // bucket when the user picked SFTP.
  function showsBucket(kind: string): boolean {
    return (
      kind === "s3" || kind === "r2" || kind === "b2" || kind === "gcs"
    );
  }
  function showsRegion(kind: string): boolean {
    return kind === "s3" || kind === "r2" || kind === "b2";
  }
  function showsEndpoint(kind: string): boolean {
    return (
      kind === "s3" ||
      kind === "r2" ||
      kind === "b2" ||
      kind === "webdav" ||
      kind === "azure-blob"
    );
  }
  function showsContainer(kind: string): boolean {
    return kind === "azure-blob";
  }
  function showsAccount(kind: string): boolean {
    return kind === "azure-blob";
  }
  function showsServiceAccount(kind: string): boolean {
    return kind === "gcs";
  }
  function showsClientId(kind: string): boolean {
    return (
      kind === "onedrive" || kind === "google-drive" || kind === "dropbox"
    );
  }
  function showsHost(kind: string): boolean {
    return kind === "sftp" || kind === "ftp";
  }
  function showsPort(kind: string): boolean {
    return kind === "sftp" || kind === "ftp";
  }
  function showsUsername(kind: string): boolean {
    return kind === "sftp" || kind === "ftp" || kind === "webdav";
  }

  function kindLabel(wire: string): string {
    const found = ALL_KINDS.find((k) => k.wire === wire);
    return found ? t(found.label) : wire;
  }
</script>

<div class="remotes-tab">
  <h3>{t("remote-heading")}</h3>

  {#if loading}
    <p>{t("settings-loading")}</p>
  {:else}
    {#if error}
      <p class="error">{error}</p>
    {/if}

    {#if backends.length === 0}
      <p class="empty">{t("remote-list-empty")}</p>
    {:else}
      <ul class="backend-list">
        {#each backends as b (b.name)}
          <li class="backend-row">
            <div class="left">
              <strong>{b.name}</strong>
              <span class="kind">{kindLabel(b.kind)}</span>
              {#if !b.enabledInBuild}
                <span class="build-off">{t("cloud-error-invalid-config")}</span>
              {/if}
            </div>
            <div class="right">
              <button type="button" onclick={() => onTest(b.name)} disabled={testing[b.name]}>
                {t("remote-test")}
              </button>
              <button type="button" onclick={() => openEdit(b)}>
                {t("remote-save")}
              </button>
              <button type="button" class="danger" onclick={() => onRemove(b.name)}>
                {t("remote-remove")}
              </button>
            </div>
            {#if testResults[b.name]}
              {#if testResults[b.name].ok}
                <div class="test-result ok">{t("remote-test-success")}</div>
              {:else}
                <div class="test-result fail">
                  {t("remote-test-failed")}
                  {#if testResults[b.name].reason}
                    — {t(`cloud-error-${testResults[b.name].reason}`) ?? testResults[b.name].reason}
                  {/if}
                </div>
              {/if}
            {/if}
          </li>
        {/each}
      </ul>
    {/if}

    {#if !showForm}
      <button type="button" class="primary" onclick={openAdd}>
        {t("remote-add")}
      </button>
    {:else}
      <form class="add-form" onsubmit={(e) => { e.preventDefault(); onSave(); }}>
        <label>
          <span>{t("remote-name-label")}</span>
          <input
            type="text"
            bind:value={formName}
            required
            disabled={!!editingName}
          />
        </label>
        <label>
          <span>{t("remote-kind-label")}</span>
          <select bind:value={formKind} disabled={!!editingName}>
            {#each ALL_KINDS as k (k.wire)}
              <option value={k.wire}>{t(k.label)}</option>
            {/each}
          </select>
        </label>

        {#if formKind === "local-fs"}
          <label>
            <span>{t("cloud-config-root")}</span>
            <input type="text" bind:value={formConfig.root} />
          </label>
        {/if}
        {#if showsBucket(formKind)}
          <label>
            <span>{t("cloud-config-bucket")}</span>
            <input type="text" bind:value={formConfig.bucket} required />
          </label>
        {/if}
        {#if showsRegion(formKind)}
          <label>
            <span>{t("cloud-config-region")}</span>
            <input type="text" bind:value={formConfig.region} />
          </label>
        {/if}
        {#if showsEndpoint(formKind)}
          <label>
            <span>{t("cloud-config-endpoint")}</span>
            <input type="text" bind:value={formConfig.endpoint} />
          </label>
        {/if}
        {#if showsContainer(formKind)}
          <label>
            <span>Container</span>
            <input type="text" bind:value={formConfig.container} required />
          </label>
        {/if}
        {#if showsAccount(formKind)}
          <label>
            <span>Account name</span>
            <input type="text" bind:value={formConfig.accountName} required />
          </label>
        {/if}
        {#if showsServiceAccount(formKind)}
          <label>
            <span>Service account</span>
            <input type="text" bind:value={formConfig.serviceAccount} />
          </label>
        {/if}
        {#if showsClientId(formKind)}
          <label>
            <span>Client ID</span>
            <input type="text" bind:value={formConfig.clientId} />
          </label>
        {/if}
        {#if showsHost(formKind)}
          <label>
            <span>Host</span>
            <input type="text" bind:value={formConfig.host} required />
          </label>
        {/if}
        {#if showsPort(formKind)}
          <label>
            <span>Port</span>
            <input
              type="number"
              bind:value={formConfig.port}
              min="0"
              max="65535"
            />
          </label>
        {/if}
        {#if showsUsername(formKind)}
          <label>
            <span>Username</span>
            <input type="text" bind:value={formConfig.username} />
          </label>
        {/if}
        {#if formKind !== "local-fs"}
          <label>
            <span>Secret (access key / password / token)</span>
            <input type="password" bind:value={formSecret} autocomplete="off" />
          </label>
          <label>
            <span>{t("cloud-config-root")}</span>
            <input type="text" bind:value={formConfig.root} />
          </label>
        {/if}

        <div class="form-actions">
          <button type="submit" class="primary" disabled={formBusy}>
            {t("remote-save")}
          </button>
          <button type="button" onclick={resetForm} disabled={formBusy}>
            {t("remote-cancel")}
          </button>
        </div>
      </form>
    {/if}
  {/if}

  <!--
    Phase 40 Part B — cloud-VM offload helper section. Sits below the
    backend list; the "Render template" button gates on two configured
    backends so the user has source + destination to pair.
  -->
  <section class="offload-section">
    <h3 class="offload-heading">{t("cloud-offload-heading")}</h3>
    <p class="offload-hint">{t("cloud-offload-hint")}</p>
    {#if !offloadShow}
      <button
        type="button"
        class="primary"
        onclick={offloadOpen}
        disabled={backends.length < 1}
      >
        {t("cloud-offload-render-button")}
      </button>
    {:else}
      <div class="offload-form">
        <label>
          <span>{t("cloud-offload-template-format")}</span>
          <select bind:value={offloadFormat}>
            <option value="cloud-init">cloud-init</option>
            <option value="aws-terraform">AWS Terraform</option>
            <option value="az-arm">Azure ARM</option>
            <option value="gcp-deployment">GCP Deployment Manager</option>
          </select>
        </label>
        <div class="offload-pair">
          <label>
            <span>Source</span>
            <select bind:value={offloadSrcName}>
              {#each backends as b (b.name)}
                <option value={b.name}>{b.name} ({b.kind})</option>
              {/each}
            </select>
          </label>
          <label>
            <span>Destination</span>
            <select bind:value={offloadDstName}>
              {#each backends as b (b.name)}
                <option value={b.name}>{b.name} ({b.kind})</option>
              {/each}
            </select>
          </label>
        </div>
        <div class="offload-pair">
          <label>
            <span>Job name</span>
            <input type="text" bind:value={offloadOpts.jobName} />
          </label>
          <label>
            <span>Region</span>
            <input type="text" bind:value={offloadOpts.region} />
          </label>
        </div>
        <div class="offload-pair">
          <label>
            <span>Instance size</span>
            <input type="text" bind:value={offloadOpts.instanceSize} />
          </label>
          <label>
            <span>IAM role / identity</span>
            <input type="text" bind:value={offloadOpts.iamRole} />
          </label>
        </div>
        <p class="offload-warning">
          {t("cloud-offload-self-destruct-warning", {
            minutes: offloadOpts.selfDestructMinutes,
          })}
        </p>
        <div class="form-actions">
          <button type="button" class="primary" onclick={offloadRender} disabled={offloadBusy}>
            {t("cloud-offload-render-button")}
          </button>
          <button type="button" onclick={() => { offloadShow = false; }} disabled={offloadBusy}>
            {t("remote-cancel")}
          </button>
        </div>
        {#if offloadOutput}
          <textarea readonly class="offload-output" rows="14">{offloadOutput}</textarea>
          <div class="form-actions">
            <button type="button" onclick={offloadCopyToClipboard}>
              {offloadCopied ? "✓" : t("cloud-offload-copy-clipboard")}
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </section>
</div>

<style>
  .remotes-tab {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  /* Phase 40 Part B — offload-helper section. Reuses the form-action
     button styles from the existing add-backend wizard so the visual
     vocabulary stays consistent. */
  .offload-section {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .offload-heading {
    font-size: 13px;
    font-weight: 600;
    margin: 0;
  }
  .offload-hint {
    margin: 0;
    color: var(--fg-dim, #5f5f5f);
    font-size: 12px;
  }
  .offload-warning {
    margin: 4px 0;
    padding: 8px;
    background: rgba(255, 165, 0, 0.08);
    border-left: 3px solid #ffa500;
    color: var(--fg, #1f1f1f);
    font-size: 12px;
    line-height: 1.4;
  }
  .offload-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: var(--surface-2, rgba(128, 128, 128, 0.05));
    border-radius: 6px;
  }
  .offload-form label {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 12px;
  }
  .offload-pair {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }
  .offload-output {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-mono, ui-monospace, monospace);
    font-size: 11px;
    line-height: 1.4;
    padding: 8px;
    background: var(--surface, #fff);
    border: 1px solid var(--border, rgba(128, 128, 128, 0.18));
    border-radius: 4px;
    resize: vertical;
  }
  .backend-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .backend-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 4px 12px;
    padding: 10px 12px;
    border: 1px solid var(--color-border, #d0d0d0);
    border-radius: 6px;
    background: var(--color-surface, #f8f8f8);
  }
  .backend-row .left {
    display: flex;
    gap: 10px;
    align-items: baseline;
  }
  .backend-row .kind {
    color: var(--color-muted, #666);
    font-size: 0.85em;
  }
  .backend-row .right {
    display: flex;
    gap: 6px;
  }
  .build-off {
    color: var(--color-warn, #9a6600);
    font-size: 0.85em;
  }
  .test-result {
    grid-column: 1 / -1;
    font-size: 0.9em;
  }
  .test-result.ok {
    color: var(--color-ok, #1a6b1a);
  }
  .test-result.fail {
    color: var(--color-error, #a00);
  }
  .empty {
    color: var(--color-muted, #666);
    font-style: italic;
  }
  .error {
    color: var(--color-error, #a00);
  }
  .add-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border: 1px solid var(--color-border, #d0d0d0);
    border-radius: 6px;
  }
  .add-form label {
    display: grid;
    grid-template-columns: 180px 1fr;
    gap: 8px;
    align-items: center;
  }
  .form-actions {
    display: flex;
    gap: 8px;
    margin-top: 6px;
  }
  .danger {
    color: var(--color-error, #a00);
  }
  .primary {
    font-weight: 600;
  }
</style>
