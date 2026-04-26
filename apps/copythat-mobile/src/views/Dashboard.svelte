<!--
  Phone-side dashboard. Live globals + per-job control + history +
  live "files being processed" stream + keep-desktop-awake toggle.
  All buttons disable while a `JobLoading` event is in flight so
  the user can't fire conflicting commands during the desktop's
  scan/enumeration phase.
-->
<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  import { applyDesktopLocale } from "../i18n";
  import { getOrMintPhoneIdentity } from "../identity";
  import type { PeerLink } from "../peer";
  import type {
    HistoryRow,
    JobSummary,
    RemoteResponse,
  } from "../protocol";

  let { link }: { link: PeerLink } = $props();

  let jobs = $state<JobSummary[]>([]);
  let history = $state<HistoryRow[]>([]);
  let globals = $state<{
    bytesDone: number;
    bytesTotal: number;
    filesDone: number;
    filesTotal: number;
    rateBps: number;
    copyFiles: number;
    moveFiles: number;
    secureDeleteFiles: number;
  }>({
    bytesDone: 0,
    bytesTotal: 0,
    filesDone: 0,
    filesTotal: 0,
    rateBps: 0,
    copyFiles: 0,
    moveFiles: 0,
    secureDeleteFiles: 0,
  });
  /// Most recent ~50 per-file events streamed from the desktop.
  let liveFiles = $state<
    Array<{
      jobId: string;
      action: string;
      src: string;
      dst: string;
      bytesDone: number;
      bytesTotal: number;
      seenAt: number;
    }>
  >([]);
  /// Active "loading" jobs (keyed by job_id). While non-empty, all
  /// dashboard buttons disable so a Re-run / Start Copy / etc.
  /// can't fire while the desktop is still enumerating files.
  let loadingJobs = $state<Map<string, string>>(new Map());
  let busy = $state(false);
  let error = $state<string | null>(null);
  /// "Keep desktop awake" toggle — fires `set_keep_awake` on the
  /// data channel.
  let keepAwake = $state(false);
  let pollHandle: ReturnType<typeof setInterval> | null = null;

  let buttonsLocked = $derived(busy || loadingJobs.size > 0);

  onMount(() => {
    void hello();
    void refresh();
    pollHandle = setInterval(() => {
      void refresh();
    }, 5000);
  });

  onDestroy(() => {
    if (pollHandle) clearInterval(pollHandle);
  });

  $effect(() => {
    const off = link.onEvent(handleStreamingEvent);
    return () => {
      off();
    };
  });

  async function hello() {
    try {
      // Use this phone's persistent identity (32 bytes minted on
      // first launch + stored in localStorage) so the desktop can
      // match against `MobileSettings::pairings`. The previous
      // `"00".repeat(32)` shape collided every device on the same
      // identity, defeating the pairing record entirely.
      let phonePubkeyHex: string;
      try {
        phonePubkeyHex = getOrMintPhoneIdentity();
      } catch {
        // Storage / crypto unavailable — fall back to a per-session
        // value the desktop will reject as not-paired. Better than
        // sending the all-zero key the previous shape did.
        phonePubkeyHex = "00".repeat(32);
      }
      await link.send({
        kind: "hello",
        phone_pubkey_hex: phonePubkeyHex,
        device_label: "phone",
      });
      // Phase 38 — pull the desktop's selected locale so the PWA
      // mirrors it. Fire-and-forget; on error we keep the
      // browser-detected locale.
      try {
        const locResp = await link.send({ kind: "get_locale" });
        if (locResp.kind === "locale") {
          applyDesktopLocale(locResp.bcp47);
        }
      } catch (e) {
        console.warn("get_locale", e);
      }
    } catch (e) {
      console.error("hello", e);
    }
  }

  async function refresh() {
    try {
      const jobsResp = await link.send({ kind: "list_jobs" });
      if (jobsResp.kind === "jobs") {
        jobs = jobsResp.jobs;
      }
      const histResp = await link.send({
        kind: "recent_history",
        limit: 10,
      });
      if (histResp.kind === "history") {
        history = histResp.rows;
      }
      const globalsResp = await link.send({ kind: "globals" });
      if (globalsResp.kind === "globals") {
        globals = {
          ...globals,
          bytesDone: globalsResp.bytes_done,
          bytesTotal: globalsResp.bytes_total,
          filesDone: globalsResp.files_done,
          filesTotal: globalsResp.files_total,
          rateBps: globalsResp.rate_bps,
        };
      }
    } catch (e) {
      error = `${e}`;
    }
  }

  function handleStreamingEvent(evt: RemoteResponse) {
    if (evt.kind === "globals_tick") {
      globals = {
        bytesDone: evt.bytes_done,
        bytesTotal: evt.bytes_total,
        filesDone: evt.files_done,
        filesTotal: evt.files_total,
        rateBps: evt.rate_bps,
        copyFiles: evt.copy_files,
        moveFiles: evt.move_files,
        secureDeleteFiles: evt.secure_delete_files,
      };
    } else if (evt.kind === "job_progress") {
      jobs = jobs.map((j) =>
        j.jobId === evt.job_id
          ? {
              ...j,
              bytesDone: evt.bytes_done,
              bytesTotal: evt.bytes_total,
              rateBps: evt.rate_bps,
            }
          : j,
      );
    } else if (evt.kind === "job_completed") {
      jobs = jobs.filter((j) => j.jobId !== evt.job_id);
    } else if (evt.kind === "job_failed") {
      jobs = jobs.map((j) =>
        j.jobId === evt.job_id ? { ...j, state: "failed" } : j,
      );
    } else if (evt.kind === "file_tick") {
      const next = liveFiles.slice();
      next.unshift({
        jobId: evt.job_id,
        action: evt.action,
        src: evt.src,
        dst: evt.dst,
        bytesDone: evt.bytes_done,
        bytesTotal: evt.bytes_total,
        seenAt: Date.now(),
      });
      // Cap at 50 so a long-running tree doesn't unbounded-grow
      // the DOM.
      liveFiles = next.slice(0, 50);
    } else if (evt.kind === "job_loading") {
      const next = new Map(loadingJobs);
      next.set(evt.job_id, evt.message);
      loadingJobs = next;
    } else if (evt.kind === "job_ready") {
      const next = new Map(loadingJobs);
      next.delete(evt.job_id);
      loadingJobs = next;
    } else if (evt.kind === "job_state_changed") {
      // Mirror pause/cancel that originated on the desktop side
      // so the PWA's job list matches without a poll.
      jobs = jobs.map((j) =>
        j.jobId === evt.job_id ? { ...j, state: evt.state } : j,
      );
      if (evt.state === "completed" || evt.state === "cancelled") {
        // Drop the row — `JobCompleted` will follow but the PWA
        // already reflects the right state.
        jobs = jobs.filter((j) => j.jobId !== evt.job_id);
      }
    }
  }

  async function pause(jobId: string) {
    if (buttonsLocked) return;
    busy = true;
    try {
      await link.send({ kind: "pause_job", job_id: jobId });
      void refresh();
    } finally {
      busy = false;
    }
  }

  async function resume(jobId: string) {
    if (buttonsLocked) return;
    busy = true;
    try {
      await link.send({ kind: "resume_job", job_id: jobId });
      void refresh();
    } finally {
      busy = false;
    }
  }

  async function cancel(jobId: string) {
    if (buttonsLocked) return;
    busy = true;
    try {
      await link.send({ kind: "cancel_job", job_id: jobId });
      void refresh();
    } finally {
      busy = false;
    }
  }

  async function rerun(rowId: number) {
    if (buttonsLocked) return;
    busy = true;
    try {
      await link.send({ kind: "rerun_history", row_id: rowId });
      // The desktop will emit a `JobLoading` event right after this
      // returns; the per-event handler flips the button-lock flag.
      void refresh();
    } finally {
      busy = false;
    }
  }

  async function toggleKeepAwake(value: boolean) {
    keepAwake = value;
    try {
      await link.send({ kind: "set_keep_awake", enabled: value });
    } catch (e) {
      error = `keep-awake: ${e}`;
      keepAwake = !value;
    }
  }

  function fmtBytes(n: number): string {
    const units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let i = 0;
    let v = n;
    while (v >= 1024 && i < units.length - 1) {
      v /= 1024;
      i++;
    }
    return `${v.toFixed(v < 10 ? 2 : 1)} ${units[i]}`;
  }

  function fmtRate(n: number): string {
    return `${fmtBytes(n)}/s`;
  }

  function pct(done: number, total: number): number {
    if (total === 0) return 0;
    return Math.min(100, (done / total) * 100);
  }

  function fmtPct(done: number, total: number): string {
    return `${pct(done, total).toFixed(2)}%`;
  }

  function shortPath(p: string): string {
    const parts = p.split(/[\\/]/);
    if (parts.length <= 3) return p;
    return `…/${parts.slice(-2).join("/")}`;
  }
</script>

{#if loadingJobs.size > 0}
  <div class="loading-banner">
    <span class="spinner"></span>
    <span>
      Desktop is loading files —
      {[...loadingJobs.values()][0] || "scanning…"}
    </span>
  </div>
{/if}

<div class="panel">
  <h2>Live progress</h2>
  <div class="big">{fmtPct(globals.bytesDone, globals.bytesTotal)}</div>
  <div class="bar">
    <div class="fill" style:width={`${pct(globals.bytesDone, globals.bytesTotal)}%`}></div>
  </div>
  <div class="row stats">
    <div class="stat">
      <div class="muted">Bytes</div>
      <div>{fmtBytes(globals.bytesDone)} / {fmtBytes(globals.bytesTotal)}</div>
    </div>
    <div class="stat">
      <div class="muted">Files</div>
      <div>{globals.filesDone} / {globals.filesTotal}</div>
    </div>
    <div class="stat">
      <div class="muted">Rate</div>
      <div>{fmtRate(globals.rateBps)}</div>
    </div>
  </div>
  <div class="row stats">
    <div class="stat">
      <div class="muted">Copied</div>
      <div>{globals.copyFiles}</div>
    </div>
    <div class="stat">
      <div class="muted">Moved</div>
      <div>{globals.moveFiles}</div>
    </div>
    <div class="stat">
      <div class="muted">Securely deleted</div>
      <div>{globals.secureDeleteFiles}</div>
    </div>
  </div>
</div>

<div class="panel">
  <h2>Settings</h2>
  <label class="row toggle">
    <input
      type="checkbox"
      checked={keepAwake}
      onchange={(e) => toggleKeepAwake((e.currentTarget as HTMLInputElement).checked)}
    />
    <span>Keep desktop awake while paired</span>
  </label>
  <p class="muted small">
    Inhibits screensaver / sleep on the desktop so a long copy
    isn't interrupted. Releases the lock when you Exit.
  </p>
</div>

<div class="panel">
  <h2>Active jobs ({jobs.length})</h2>
  {#if jobs.length === 0}
    <p class="muted">Nothing running right now.</p>
  {:else}
    <ul class="jobs">
      {#each jobs as job (job.jobId)}
        <li>
          <div class="row title">
            <span class="kind">{job.kind}</span>
            <span class="muted state">{job.state}</span>
            <span class="job-pct">{fmtPct(job.bytesDone, job.bytesTotal)}</span>
          </div>
          <div class="muted mono">{shortPath(job.src)} → {shortPath(job.dst)}</div>
          <div class="bar small">
            <div class="fill" style:width={`${pct(job.bytesDone, job.bytesTotal)}%`}></div>
          </div>
          <div class="row controls">
            <button
              type="button"
              class="secondary"
              onclick={() => pause(job.jobId)}
              disabled={buttonsLocked || job.state !== "running"}
            >
              Pause
            </button>
            <button
              type="button"
              class="secondary"
              onclick={() => resume(job.jobId)}
              disabled={buttonsLocked || job.state !== "paused"}
            >
              Resume
            </button>
            <button
              type="button"
              class="destructive"
              onclick={() => cancel(job.jobId)}
              disabled={buttonsLocked}
            >
              Cancel
            </button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<div class="panel">
  <h2>Files being processed ({liveFiles.length})</h2>
  {#if liveFiles.length === 0}
    <p class="muted">No file events yet.</p>
  {:else}
    <ul class="live-files">
      {#each liveFiles as f, idx (f.seenAt + idx + f.src)}
        <li>
          <div class="row">
            <span class="action {f.action}">{f.action}</span>
            <span class="muted mono small flex">{shortPath(f.src)}</span>
            <span class="job-pct small">{fmtPct(f.bytesDone, f.bytesTotal)}</span>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<div class="panel">
  <h2>Recent history</h2>
  {#if history.length === 0}
    <p class="muted">No recent jobs.</p>
  {:else}
    <ul class="history">
      {#each history as row (row.rowId)}
        <li>
          <div class="row">
            <span class="kind">{row.kind}</span>
            <span class="muted">{row.status}</span>
            <span class="muted mono small-fp">{fmtBytes(row.totalBytes)}</span>
          </div>
          <div class="muted mono small">{row.srcRoot} → {row.dstRoot}</div>
          <button
            type="button"
            class="secondary"
            onclick={() => rerun(row.rowId)}
            disabled={buttonsLocked}
          >
            Re-run
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

{#if error}
  <p class="error">{error}</p>
{/if}

<style>
  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .big {
    font-size: 2.5rem;
    font-weight: 700;
    text-align: center;
    margin: 0.5rem 0;
    font-feature-settings: "tnum";
  }
  .bar {
    height: 8px;
    background: var(--border);
    border-radius: 4px;
    overflow: hidden;
    margin: 0.5rem 0;
  }
  .bar.small {
    height: 4px;
  }
  .fill {
    height: 100%;
    background: var(--accent);
    transition: width 0.25s;
  }
  .stats {
    margin-top: 0.5rem;
    gap: 1rem;
  }
  .stat {
    flex: 1;
  }
  .jobs,
  .history,
  .live-files {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .jobs li,
  .history li {
    background: var(--bg);
    padding: 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border);
  }
  .live-files li {
    background: var(--bg);
    padding: 0.4rem 0.6rem;
    border-radius: 0.4rem;
    border: 1px solid var(--border);
    font-size: 0.85rem;
  }
  .controls {
    margin-top: 0.5rem;
    gap: 0.5rem;
  }
  .controls button {
    flex: 1;
    font-size: 0.85rem;
    padding: 0.4rem 0.5rem;
  }
  .kind {
    text-transform: uppercase;
    font-size: 0.75rem;
    background: var(--accent);
    color: white;
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
  }
  .action {
    text-transform: uppercase;
    font-size: 0.7rem;
    padding: 0.1rem 0.35rem;
    border-radius: 3px;
    background: var(--border);
    color: var(--fg);
  }
  .action.copying {
    background: var(--accent);
    color: white;
  }
  .action.completed {
    background: var(--success);
    color: white;
  }
  .action.failed {
    background: var(--error);
    color: white;
  }
  .action.scanning {
    background: var(--muted);
    color: var(--bg);
  }
  .state {
    font-size: 0.85rem;
  }
  .job-pct {
    margin-left: auto;
    font-feature-settings: "tnum";
    font-weight: 600;
    color: var(--accent);
  }
  .small-fp {
    margin-left: auto;
  }
  .small {
    font-size: 0.75rem;
  }
  .flex {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .toggle {
    gap: 0.5rem;
    font-size: 0.95rem;
  }
  .loading-banner {
    background: var(--accent);
    color: white;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
