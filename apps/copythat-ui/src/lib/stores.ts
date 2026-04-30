// Svelte stores that mirror the Rust-side queue.
//
// The data flow is one-way: Rust is the source of truth, the UI
// receives events and rebuilds state. Commands that mutate state
// (pause/resume/cancel) fire over IPC and wait for the matching
// event to come back before updating; this keeps the UI and the
// engine in sync even when the engine rejects or ignores a request.

import { derived, writable, type Readable } from "svelte/store";

import {
  enumerateTreeFiles,
  getSettings,
  listJobs,
  globals as fetchGlobals,
  onEvent,
  queueList,
} from "./ipc";
import {
  EVENTS,
  type ClipboardFilesDetectedDto,
  type CollisionAutoResolvedDto,
  type CollisionPromptDto,
  type CollisionResolvedDto,
  type DropReceivedDto,
  type ErrorDisplayModeWire,
  type ErrorPromptDto,
  type ErrorResolvedDto,
  type FileActivityDto,
  type GlobalsDto,
  type JobDto,
  type JobFailedDto,
  type JobIdDto,
  type JobProgressDto,
  type QueueSnapshotDto,
  type ToastKind,
  type ToastMessage,
} from "./types";

const jobsStore = writable<JobDto[]>([]);
const globalsStore = writable<GlobalsDto>({
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
const droppedStore = writable<string[]>([]);
const toastStore = writable<ToastMessage[]>([]);
// Phase 8: FIFO of pending prompts. Frontend renders the head;
// `resolve_error` / `resolve_collision` pops on response.
const errorQueueStore = writable<ErrorPromptDto[]>([]);
const collisionQueueStore = writable<CollisionPromptDto[]>([]);

// ---- Phase 22 aggregate conflict dialog state ----
//
// The aggregate dialog stays open for the lifetime of a job and
// shows every collision that occurs — pending, interactively
// resolved, and rule-auto-resolved. Each row carries enough state
// for the dialog to render its two-column view without another
// IPC round-trip:
//
// - `pending`: the oneshot is parked in the Rust registry; the UI
//   renders "action buttons" to let the user resolve.
// - `resolved`: the user picked an action OR a rule auto-resolved.
//   The row shows a checkmark + the chosen action.
//
// A single `batch` store is simpler than two queues because every
// list view the dialog needs (pending, resolved, by-extension)
// derives from the same row set. Cleared on job removal.
export type ConflictBatchRowState =
  | { phase: "pending" }
  | {
      phase: "resolved";
      action: string;
      matchedRulePattern: string | null;
    };

export interface ConflictBatchRow {
  jobId: number;
  /// Mirrors CollisionPromptDto.id. `null` when the row is a
  /// rule-auto-resolved event (no interactive prompt was
  /// registered, so there's no id to address later).
  id: number | null;
  src: string;
  dst: string;
  srcSize: number | null;
  srcModifiedMs: number | null;
  dstSize: number | null;
  dstModifiedMs: number | null;
  state: ConflictBatchRowState;
  createdAtMs: number;
}
const conflictBatchStore = writable<ConflictBatchRow[]>([]);
const errorLogDrawerOpenStore = writable<boolean>(false);
// Phase 9: History drawer open/closed flag + selected detail row.
// The drawer fetches on open; no in-store cache of rows (Rust is
// source of truth).
const historyDrawerOpenStore = writable<boolean>(false);
const historyDetailRowStore = writable<number | null>(null);
// Phase 10: Totals drawer open flag. The drawer fetches fresh
// aggregates on open; no store cache.
const totalsDrawerOpenStore = writable<boolean>(false);
// Phase 11b: Settings modal open flag. Minimal today — General
// tab with Language only — Phase 12 extends to Transfer, Shell,
// Secure-delete, Advanced tabs.
const settingsOpenStore = writable<boolean>(false);
// Phase 25: Sync drawer open flag.
const syncDrawerOpenStore = writable<boolean>(false);
// How error prompts render — mirrors
// `general.error_display_mode` from the settings crate. Seeded by
// `initStores` via `getSettings()`; re-written whenever the user
// flips the toggle in SettingsModal. App.svelte picks between
// ErrorModal and ErrorPromptDrawer based on this value.
const errorDisplayModeStore = writable<ErrorDisplayModeWire>("modal");

// Phase 13d / 16 — the Activity panel doubles as the full file list
// for each queued job. When a new job lands, we recursively enumerate
// its source paths and pre-seed one row per file in a `pending` phase,
// so the user can see the total count + sort by name / size before
// the first byte moves. As the engine fires per-file Started /
// Progress / Completed events, matching rows flip through `start`,
// `progress`, `done` in place. Errors flip to `error`; rows that
// were never pre-seeded (e.g. huge trees past the enumerator's
// overflow cap) still append lazily via the old path.
//
// Cap picked to land under ~50 MB of reactive store memory on
// modest hardware (≈200 B per row × 250 k rows). Enough that most
// users' typical trees sit entirely inside the list; beyond that,
// older completed rows age out so the pending / in-flight rows
// always stay visible at the bottom.
const ACTIVITY_LIMIT = 250_000;
export type ActivityPhase =
  | "pending"
  | "start"
  | "progress"
  | "done"
  | "error"
  | "dir";
interface FileActivityRow {
  key: string;
  jobId: number;
  seq: number;
  src: string;
  dst: string;
  phase: ActivityPhase;
  bytesDone: number;
  bytesTotal: number;
  isDir: boolean;
  message: string | null;
  addedAtMs: number;
}
const fileActivityStore = writable<FileActivityRow[]>([]);
const activityOpenStore = writable<boolean>(
  typeof localStorage !== "undefined"
    ? localStorage.getItem("copythat-activity-open") !== "false"
    : true,
);

export type ActivitySortMode =
  | "insertion"
  | "name-asc"
  | "name-desc"
  | "size-asc"
  | "size-desc";
const activitySortStore = writable<ActivitySortMode>(
  (typeof localStorage !== "undefined"
    ? (localStorage.getItem("copythat-activity-sort") as ActivitySortMode | null)
    : null) ?? "insertion",
);

export const jobs: Readable<JobDto[]> = { subscribe: jobsStore.subscribe };
export const globals: Readable<GlobalsDto> = {
  subscribe: globalsStore.subscribe,
};
export const dropped: Readable<string[]> = {
  subscribe: droppedStore.subscribe,
};
export const toasts: Readable<ToastMessage[]> = {
  subscribe: toastStore.subscribe,
};
export const errorQueue: Readable<ErrorPromptDto[]> = {
  subscribe: errorQueueStore.subscribe,
};
export const collisionQueue: Readable<CollisionPromptDto[]> = {
  subscribe: collisionQueueStore.subscribe,
};

/// Phase 22 — the full per-job conflict feed. Each incoming
/// `collision-raised` appends a `pending` row; each
/// `collision-resolved` flips the matching row to `resolved`;
/// each `collision-auto-resolved` appends a brand-new `resolved`
/// row (no prior prompt existed). The aggregate modal reads this;
/// the single-file `CollisionModal` still reads `collisionQueue`
/// so both pathways coexist until the aggregate modal is the
/// default everywhere.
export const conflictBatch: Readable<ConflictBatchRow[]> = {
  subscribe: conflictBatchStore.subscribe,
};

/// Reset the aggregate feed — called by the modal's "Close"
/// button once every row is resolved, and by job removal.
export function clearConflictBatch(jobId?: number): void {
  if (jobId === undefined) {
    conflictBatchStore.set([]);
    return;
  }
  conflictBatchStore.update(($rows) => $rows.filter((r) => r.jobId !== jobId));
}
export const errorLogDrawerOpen: Readable<boolean> = {
  subscribe: errorLogDrawerOpenStore.subscribe,
};
export function openErrorLogDrawer(): void {
  errorLogDrawerOpenStore.set(true);
}
export function closeErrorLogDrawer(): void {
  errorLogDrawerOpenStore.set(false);
}

// Phase 9 history drawer readables + mutators.
export const historyDrawerOpen: Readable<boolean> = {
  subscribe: historyDrawerOpenStore.subscribe,
};
export const historyDetailRow: Readable<number | null> = {
  subscribe: historyDetailRowStore.subscribe,
};
export function openHistoryDrawer(): void {
  historyDrawerOpenStore.set(true);
}
export function closeHistoryDrawer(): void {
  historyDrawerOpenStore.set(false);
  historyDetailRowStore.set(null);
}
export function openHistoryDetail(rowId: number): void {
  historyDetailRowStore.set(rowId);
}
export function closeHistoryDetail(): void {
  historyDetailRowStore.set(null);
}

// Phase 10 Totals drawer.
export const totalsDrawerOpen: Readable<boolean> = {
  subscribe: totalsDrawerOpenStore.subscribe,
};
export function openTotalsDrawer(): void {
  totalsDrawerOpenStore.set(true);
}
export function closeTotalsDrawer(): void {
  totalsDrawerOpenStore.set(false);
}

// Phase 11b Settings modal.
export const settingsOpen: Readable<boolean> = {
  subscribe: settingsOpenStore.subscribe,
};
export function openSettings(): void {
  settingsOpenStore.set(true);
}
export function closeSettings(): void {
  settingsOpenStore.set(false);
}

// Phase 25 — Sync drawer.
export const syncDrawerOpen: Readable<boolean> = {
  subscribe: syncDrawerOpenStore.subscribe,
};
export function openSyncDrawer(): void {
  syncDrawerOpenStore.set(true);
}
export function closeSyncDrawer(): void {
  syncDrawerOpenStore.set(false);
}

// Error display mode — readable + write-through setter. SettingsModal
// calls the setter after a successful `updateSettings` round-trip so
// the rest of the UI picks up the change without a reload.
export const errorDisplayMode: Readable<ErrorDisplayModeWire> = {
  subscribe: errorDisplayModeStore.subscribe,
};
export function setErrorDisplayMode(mode: ErrorDisplayModeWire): void {
  errorDisplayModeStore.set(mode);
}

// Phase 13d — file-activity feed exports.
export const fileActivity: Readable<FileActivityRow[]> = {
  subscribe: fileActivityStore.subscribe,
};
export const activityOpen: Readable<boolean> = {
  subscribe: activityOpenStore.subscribe,
};
export const activitySort: Readable<ActivitySortMode> = {
  subscribe: activitySortStore.subscribe,
};
export function setActivitySort(mode: ActivitySortMode): void {
  activitySortStore.set(mode);
  try {
    localStorage.setItem("copythat-activity-sort", mode);
  } catch {
    // Best-effort — non-fatal if storage rejects.
  }
}
// Sorted view of the activity list. Cheap: the store shape is one
// row per file, capped at ACTIVITY_LIMIT, and the sort runs via
// derived on each update. Insertion mode is the identity.
function basenameOf(p: string): string {
  const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
  return i >= 0 ? p.slice(i + 1) : p;
}

function parentOf(p: string): string {
  if (!p) return "";
  const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
  if (i <= 0) return p;
  return p.slice(0, i);
}

export interface CurrentCopyDirs {
  src: string;
  dst: string;
}

/// Per-job "currently copying from / to" parent-folder paths,
/// derived from the most recent in-flight FileActivity row. The
/// JobRow's second line uses these instead of the static job-root
/// src/dst while a copy is actively running, so the user sees the
/// real subfolder the engine is walking through right now.
export const currentCopyDirs: Readable<Map<number, CurrentCopyDirs>> = derived(
  fileActivityStore,
  ($rows) => {
    const out = new Map<number, CurrentCopyDirs>();
    // Walk newest-first: first in-flight (`start`/`progress`) row
    // we see for a given job wins. Done/error rows are ignored so
    // a finished file doesn't pin the label after the engine has
    // already moved on to the next folder.
    for (let i = $rows.length - 1; i >= 0; i--) {
      const r = $rows[i];
      if (r.phase !== "start" && r.phase !== "progress" && r.phase !== "dir") {
        continue;
      }
      if (out.has(r.jobId)) continue;
      out.set(r.jobId, {
        src: parentOf(r.src),
        dst: parentOf(r.dst),
      });
    }
    return out;
  },
);
export const sortedFileActivity: Readable<FileActivityRow[]> = derived(
  [fileActivityStore, activitySortStore],
  ([$rows, $mode]) => {
    if ($mode === "insertion") return $rows;
    const copy = $rows.slice();
    copy.sort((a, b) => {
      switch ($mode) {
        case "name-asc":
          return basenameOf(a.src).localeCompare(basenameOf(b.src));
        case "name-desc":
          return basenameOf(b.src).localeCompare(basenameOf(a.src));
        case "size-asc":
          return (a.bytesTotal || 0) - (b.bytesTotal || 0);
        case "size-desc":
          return (b.bytesTotal || 0) - (a.bytesTotal || 0);
        default:
          return 0;
      }
    });
    return copy;
  },
);

export function toggleActivity(): void {
  activityOpenStore.update((v) => {
    const next = !v;
    try {
      localStorage.setItem("copythat-activity-open", next ? "true" : "false");
    } catch {
      // Best-effort — non-fatal if storage rejects.
    }
    return next;
  });
}
export function clearActivity(): void {
  fileActivityStore.set([]);
}

/// Pre-seed the Activity store with every file inside this job's
/// source tree. The enumerator is recursive on the Rust side +
/// capped at 500 000 entries; past that the flag flips to
/// `overflow: true` and we silently fall back to lazy-append mode
/// (the `file-activity` event handler takes over as engine fires
/// per-file Started events).
async function preseedJobFiles(jobId: number, src: string): Promise<void> {
  try {
    const res = await enumerateTreeFiles([src]);
    if (res.files.length === 0) return;
    const now = Date.now();
    fileActivityStore.update(($rows) => {
      const existingKeys = new Set($rows.map((r) => r.key));
      const toAdd: FileActivityRow[] = [];
      for (const f of res.files) {
        const key = `${jobId}:${f.path}`;
        if (existingKeys.has(key)) continue;
        toAdd.push({
          key,
          jobId,
          seq: 0,
          src: f.path,
          dst: "",
          phase: "pending",
          bytesDone: 0,
          bytesTotal: f.size,
          isDir: false,
          message: null,
          addedAtMs: now,
        });
      }
      const appended = [...$rows, ...toAdd];
      if (appended.length > ACTIVITY_LIMIT) {
        return appended.slice(appended.length - ACTIVITY_LIMIT);
      }
      return appended;
    });
  } catch {
    // Non-fatal — lazy-append via the file-activity event stream
    // will still populate the list as the engine starts files.
  }
}

/// Eager pre-seed used by DropStagingDialog *before* calling
/// `startCopy`. We don't know the job id yet (startCopy returns it
/// asynchronously), so rows land under a temporary key
/// `pending-<src>:<path>` and get promoted to the real
/// `<jobId>:<path>` by `adoptPreseededForJob` when the `job-added`
/// event fires. That two-step dance guarantees the full file list
/// is visible the moment the engine starts — no race between
/// per-file Started events and the enumerator.
const pendingPreseedPaths = new Set<string>();

export async function preseedBeforeEnqueue(
  sources: string[],
  opts: { timeoutMs?: number } = {},
): Promise<{ completed: boolean; fileCount: number }> {
  // Hard wall-clock budget — if the walk is still running after
  // this, skip the pre-seed and let the lazy-append path handle
  // it. Protects the "whole drive" case where the recursive walk
  // would block the Start Copy button for minutes.
  const timeoutMs = opts.timeoutMs ?? 8_000;
  const enumPromise = enumerateTreeFiles(sources);
  const timeoutPromise = new Promise<null>((resolve) =>
    setTimeout(() => resolve(null), timeoutMs),
  );
  let res: Awaited<ReturnType<typeof enumerateTreeFiles>> | null = null;
  try {
    const outcome = await Promise.race([enumPromise, timeoutPromise]);
    if (outcome === null) {
      // Timed out — walk keeps running in the background but the
      // UI has already bailed. The engine's own pre-walk is the
      // fallback source of truth for TreeStarted totals.
      for (const s of sources) pendingPreseedPaths.add(s);
      return { completed: false, fileCount: 0 };
    }
    res = outcome;
  } catch {
    return { completed: false, fileCount: 0 };
  }
  try {
    if (res.files.length === 0) return { completed: true, fileCount: 0 };
    // Defensive cap — if the enumerator hit its HARD_LIMIT the
    // earlier D:\ attempt showed the webview renderer can't
    // stomach that many reactive rows in one atomic append
    // (crashed `Chrome_WidgetWin_0`). Skip pre-seed entirely on
    // overflow; lazy append via the file-activity event stream
    // keeps the app alive.
    if (res.overflow) {
      for (const s of sources) pendingPreseedPaths.add(s);
      return { completed: false, fileCount: 0 };
    }
    const now = Date.now();
    const files = res.files;
    // Chunk the append so a 2 000-row drop doesn't land in a
    // single store update — that forces Svelte to create and
    // track 2 000 reactive rows atomically, which on a loaded
    // webview renderer is the kind of spike that triggers the
    // OOM we already saw. 256 rows per batch with a rAF yield
    // between batches keeps the UI thread responsive.
    const CHUNK = 256;
    for (let off = 0; off < files.length; off += CHUNK) {
      const slice = files.slice(off, off + CHUNK);
      fileActivityStore.update(($rows) => {
        const existingKeys = new Set($rows.map((r) => r.key));
        const toAdd: FileActivityRow[] = [];
        for (const f of slice) {
          let rootSrc = "";
          for (const s of sources) {
            if (f.path === s || f.path.startsWith(s)) {
              rootSrc = s;
              break;
            }
          }
          const key = `pending-${rootSrc}:${f.path}`;
          if (existingKeys.has(key)) continue;
          toAdd.push({
            key,
            jobId: -1,
            seq: 0,
            src: f.path,
            dst: "",
            phase: "pending",
            bytesDone: 0,
            bytesTotal: f.size,
            isDir: false,
            message: null,
            addedAtMs: now,
          });
        }
        return [...$rows, ...toAdd];
      });
      // Yield to the renderer between batches so it can paint
      // the partial list and avoid a blocking mega-update.
      if (off + CHUNK < files.length) {
        await new Promise<void>((r) => {
          if (typeof requestAnimationFrame === "function") {
            requestAnimationFrame(() => r());
          } else {
            setTimeout(r, 0);
          }
        });
      }
    }
    for (const s of sources) pendingPreseedPaths.add(s);
    return { completed: true, fileCount: files.length };
  } catch {
    // Silent fallback — lazy append will still work.
    return { completed: false, fileCount: 0 };
  }
}

/// Batched activity-event handler. High-concurrency tree copies
/// fire hundreds of `file-activity` events per second; doing one
/// Svelte store update per event would force a full list-diff +
/// derived-store re-sort on every tick, which is what crashed the
/// webview on the whole-drive enqueue. We queue events and flush
/// once per 100 ms — one store update, one diff, one sort per
/// batch regardless of how many events landed in between.
let activityQueue: FileActivityDto[] = [];
let activityFlushScheduled = false;
const ACTIVITY_FLUSH_MS = 100;

function queueActivityEvent(p: FileActivityDto): void {
  activityQueue.push(p);
  if (!activityFlushScheduled) {
    activityFlushScheduled = true;
    setTimeout(flushActivityQueue, ACTIVITY_FLUSH_MS);
  }
}

function flushActivityQueue(): void {
  activityFlushScheduled = false;
  const batch = activityQueue;
  if (batch.length === 0) return;
  activityQueue = [];
  fileActivityStore.update(($rows) => {
    // Build an index once per flush so each event in the batch
    // gets an O(1) lookup instead of $rows.findIndex (which is
    // O(n) per event and quadratic over a batch).
    const byKey = new Map<string, number>();
    for (let i = 0; i < $rows.length; i++) byKey.set($rows[i].key, i);
    let next = $rows;
    let mutated = false;
    const cloneIfNeeded = () => {
      if (!mutated) {
        next = $rows.slice();
        mutated = true;
      }
    };
    for (const p of batch) {
      const key = `${p.jobId}:${p.src}`;
      const idx = byKey.get(key);
      if (idx !== undefined) {
        const existing = next[idx];
        // Keep the pre-seeded bytesTotal (engine's Progress
        // events only ship bytesDone) so the size column stays
        // stable across the row's whole lifetime.
        const bytesTotal =
          p.bytesTotal > 0 ? p.bytesTotal : existing.bytesTotal;
        const dst = p.dst || existing.dst;
        // Monotonic: a stale Progress arriving after Done/Error
        // must not re-open the row.
        if (
          (existing.phase === "done" || existing.phase === "error") &&
          p.phase === "progress"
        ) {
          continue;
        }
        cloneIfNeeded();
        next[idx] = {
          ...existing,
          seq: p.seq,
          dst,
          phase: p.phase,
          bytesDone: p.bytesDone,
          bytesTotal,
          isDir: p.isDir || existing.isDir,
          message: p.message ?? existing.message,
        };
      } else {
        cloneIfNeeded();
        const row: FileActivityRow = {
          key,
          jobId: p.jobId,
          seq: p.seq,
          src: p.src,
          dst: p.dst,
          phase: p.phase,
          bytesDone: p.bytesDone,
          bytesTotal: p.bytesTotal,
          isDir: p.isDir,
          message: p.message,
          addedAtMs: Date.now(),
        };
        byKey.set(key, next.length);
        next.push(row);
      }
    }
    if (!mutated) return $rows;
    if (next.length > ACTIVITY_LIMIT) {
      next = next.slice(next.length - ACTIVITY_LIMIT);
    }
    return next;
  });
  // If more events arrived during the flush, schedule another.
  if (activityQueue.length > 0 && !activityFlushScheduled) {
    activityFlushScheduled = true;
    setTimeout(flushActivityQueue, ACTIVITY_FLUSH_MS);
  }
}

function adoptPreseededForJob(jobId: number, src: string): void {
  const prefix = `pending-${src}:`;
  fileActivityStore.update(($rows) => {
    let changed = false;
    const next = $rows.map((r) => {
      if (r.key.startsWith(prefix)) {
        changed = true;
        const filePath = r.key.slice(prefix.length);
        return { ...r, jobId, key: `${jobId}:${filePath}` };
      }
      return r;
    });
    return changed ? next : $rows;
  });
  pendingPreseedPaths.delete(src);
}

/// The sum of live per-job rates. `GlobalsDto.rateBps` from Rust is
/// intentionally 0 — derivating it in the UI keeps the numbers tied
/// to the store we already trust.
export const liveRateBps: Readable<number> = derived(jobsStore, ($jobs) =>
  $jobs
    .filter((j) => j.state === "running")
    .reduce((acc, j) => acc + (j.rateBps || 0), 0),
);

/// Aggregate bytes across all live (running / paused / pending)
/// jobs. Derived from the jobsStore rather than `GlobalsDto` so the
/// bottom ProgressBar can't drift out of sync with each row's ring
/// — they both read from the same source-of-truth. Completed and
/// failed jobs are excluded from both numerator and denominator so
/// the "current batch" bar reflects what's still in flight, not
/// lifetime history.
export const liveBytes: Readable<{ done: number; total: number }> = derived(
  jobsStore,
  ($jobs) =>
    $jobs
      .filter(
        (j) =>
          j.state === "running" || j.state === "paused" || j.state === "pending",
      )
      .reduce(
        (acc, j) => ({
          done: acc.done + (j.bytesDone || 0),
          total: acc.total + (j.bytesTotal || 0),
        }),
        { done: 0, total: 0 },
      ),
);

// ----------------------------------------------------------------------
// Phase 45.3 — named-queue tab strip
//
// `queuesStore` mirrors the Rust-side `QueueRegistry` snapshot via
// `queue_list()`; it's reseeded on `queue-added` / `queue-removed` /
// `queue-merged` events. The legacy single `state.queue` is NOT in
// this list — its jobs carry `queueId === 0` and surface under the
// synthesised "default" tab the JobListTabs component renders on top.
//
// `selectedQueueIdStore` defaults to 0 (default tab) so first-launch
// users see the same JobList they always have. Switching tabs filters
// `visibleJobs` to that queue.
// ----------------------------------------------------------------------

const queuesStore = writable<QueueSnapshotDto[]>([]);
const selectedQueueIdStore = writable<number>(0);

export const queues: Readable<QueueSnapshotDto[]> = {
  subscribe: queuesStore.subscribe,
};
export const selectedQueueId: Readable<number> = {
  subscribe: selectedQueueIdStore.subscribe,
};

export function setSelectedQueue(id: number): void {
  selectedQueueIdStore.set(id);
}

/// Re-fetch the registry snapshot. Cheap (Rust-side iteration of a
/// `Vec<Queue>`); called after every queue-* event so the badge
/// counts stay current without polling.
export async function refreshQueues(): Promise<void> {
  try {
    const list = await queueList();
    queuesStore.set(list);
    // If the previously-selected queue disappeared (merged/removed),
    // fall back to the default tab so the JobList isn't pinned to a
    // dead id.
    const sel = currentSelectedQueueId();
    if (sel !== 0 && !list.some((q) => q.id === sel)) {
      selectedQueueIdStore.set(0);
    }
  } catch {
    // Tauri may not be ready in test harnesses; keep the current
    // snapshot rather than blanking the tab strip.
  }
}

function currentSelectedQueueId(): number {
  let value = 0;
  selectedQueueIdStore.subscribe((v) => {
    value = v;
  })();
  return value;
}

/// Convenience: jobs visible in the list, filtered to the active
/// queue tab and sorted by id. The default tab (id=0) shows every
/// job whose `queueId` is 0 — i.e. the legacy single-queue surface
/// that today produces 100% of running work. Registry-routed jobs
/// (Phase 45.4+) appear under their respective tabs.
export const visibleJobs: Readable<JobDto[]> = derived(
  [jobsStore, selectedQueueIdStore],
  ([$jobs, $selected]) =>
    $jobs
      .filter((j) => (j.queueId ?? 0) === $selected)
      .sort((a, b) => a.id - b.id),
);

/// Bootstrap: hydrate stores from a cold `list_jobs` call, then
/// subscribe to the live event stream. Returns a teardown that the
/// root component calls from `onDestroy`.
export async function initStores(): Promise<() => void> {
  const [initialJobs, initialGlobals] = await Promise.all([
    listJobs(),
    fetchGlobals(),
  ]);
  jobsStore.set(initialJobs);
  globalsStore.set(initialGlobals);

  // Pull the persisted error-display-mode preference once at startup
  // so the conditional render in App.svelte mounts the right
  // component from the first frame. Failures fall back to the
  // default ("modal") — a fresh install with no settings.toml is the
  // common case and that's exactly what Settings::default() returns.
  try {
    const s = await getSettings();
    errorDisplayModeStore.set(s.general.errorDisplayMode);
  } catch {
    // Non-fatal: Tauri may not be ready yet in test harnesses.
  }

  // Phase 45.3 — seed the queue tab strip from the registry. Empty
  // on a cold launch (registry queues are spawned on first
  // `queue_route_job` call); the synthesised default tab covers
  // legacy-queue jobs in the meantime.
  await refreshQueues();

  const unlisten = await Promise.all([
    onEvent<JobDto>(EVENTS.jobAdded, (job) => {
      jobsStore.update(($jobs) => {
        if ($jobs.some((j) => j.id === job.id)) return $jobs;
        return [...$jobs, job];
      });
      // Phase 16 — pre-seed the Activity list for jobs that bypassed
      // the DropStagingDialog flow (shell enqueue, CLI --enqueue,
      // history re-run). DropStagingDialog enqueues pre-seed its
      // own rows *before* calling startCopy, so this path is only a
      // fallback for jobs whose IDs we don't know in advance.
      const alreadySeeded = pendingPreseedPaths.has(job.src);
      if (!alreadySeeded) {
        void preseedJobFiles(job.id, job.src);
      } else {
        // Promote the pre-seeded rows from `pending-<src>` to the
        // real `jobId:<path>` keys so incoming file-activity events
        // match them in place.
        adoptPreseededForJob(job.id, job.src);
      }
    }),
    onEvent<JobIdDto>(EVENTS.jobStarted, (payload) => {
      updateJob(payload.id, (job) => ({ ...job, state: "running" }));
    }),
    onEvent<JobProgressDto>(EVENTS.jobProgress, (p) => {
      updateJob(p.id, (job) => ({
        ...job,
        bytesDone: p.bytesDone,
        bytesTotal: p.bytesTotal,
        filesDone: p.filesDone,
        filesTotal: p.filesTotal,
        rateBps: p.rateBps,
        etaSeconds: p.etaSeconds,
        state: job.state === "paused" ? "paused" : "running",
      }));
    }),
    onEvent<JobIdDto>(EVENTS.jobPaused, (p) =>
      updateJob(p.id, (job) => ({ ...job, state: "paused" })),
    ),
    onEvent<JobIdDto>(EVENTS.jobResumed, (p) =>
      updateJob(p.id, (job) => ({ ...job, state: "running" })),
    ),
    onEvent<JobIdDto>(EVENTS.jobCancelled, (p) =>
      updateJob(p.id, (job) => ({ ...job, state: "cancelled", rateBps: 0 })),
    ),
    onEvent<JobIdDto>(EVENTS.jobCompleted, (p) => {
      updateJob(p.id, (job) => ({
        ...job,
        state: "succeeded",
        bytesDone: job.bytesTotal || job.bytesDone,
        filesDone: job.filesTotal || job.filesDone,
        rateBps: 0,
        etaSeconds: 0,
      }));
      pushToast("success", "toast-job-done");
    }),
    onEvent<JobFailedDto>(EVENTS.jobFailed, (p) => {
      updateJob(p.id, (job) => ({
        ...job,
        state: "failed",
        lastError: p.message,
        rateBps: 0,
      }));
      pushToast("error", p.message);
    }),
    onEvent<JobIdDto>(EVENTS.jobRemoved, (p) => {
      jobsStore.update(($jobs) => $jobs.filter((j) => j.id !== p.id));
    }),
    onEvent<GlobalsDto>(EVENTS.globalsTick, (g) => {
      globalsStore.set(g);
    }),
    onEvent<DropReceivedDto>(EVENTS.dropReceived, (p) => {
      droppedStore.set(p.paths ?? []);
    }),
    // Phase 8: error / collision prompts + resolution echoes.
    onEvent<ErrorPromptDto>(EVENTS.errorRaised, (p) => {
      errorQueueStore.update(($q) => [...$q, p]);
    }),
    onEvent<ErrorResolvedDto>(EVENTS.errorResolved, (p) => {
      errorQueueStore.update(($q) => $q.filter((e) => e.id !== p.id));
      pushToast("info", "toast-error-resolved");
    }),
    onEvent<CollisionPromptDto>(EVENTS.collisionRaised, (p) => {
      collisionQueueStore.update(($q) => [...$q, p]);
      conflictBatchStore.update(($rows) => [
        ...$rows,
        {
          jobId: p.jobId,
          id: p.id,
          src: p.src,
          dst: p.dst,
          srcSize: p.srcSize,
          srcModifiedMs: p.srcModifiedMs,
          dstSize: p.dstSize,
          dstModifiedMs: p.dstModifiedMs,
          state: { phase: "pending" } satisfies ConflictBatchRowState,
          createdAtMs: Date.now(),
        },
      ]);
    }),
    onEvent<CollisionResolvedDto>(EVENTS.collisionResolved, (p) => {
      collisionQueueStore.update(($q) => $q.filter((c) => c.id !== p.id));
      conflictBatchStore.update(($rows) =>
        $rows.map((r) =>
          r.id === p.id
            ? {
                ...r,
                state: {
                  phase: "resolved",
                  action: p.resolution,
                  matchedRulePattern: null,
                } satisfies ConflictBatchRowState,
              }
            : r,
        ),
      );
      pushToast("info", "toast-collision-resolved");
    }),
    // Phase 22 — the runner auto-resolved a collision by matching
    // the job's active ConflictProfile. No prompt was registered,
    // so this appends a fresh `resolved` row rather than flipping
    // an existing one.
    onEvent<CollisionAutoResolvedDto>(EVENTS.collisionAutoResolved, (p) => {
      conflictBatchStore.update(($rows) => [
        ...$rows,
        {
          jobId: p.jobId,
          id: null,
          src: p.src,
          dst: p.dst,
          srcSize: null,
          srcModifiedMs: null,
          dstSize: null,
          dstModifiedMs: null,
          state: {
            phase: "resolved",
            action: p.resolution,
            matchedRulePattern: p.matchedRulePattern,
          } satisfies ConflictBatchRowState,
          createdAtMs: Date.now(),
        },
      ]);
    }),
    // Phase 13d — per-file live activity. `start`/`dir` append a new
    // row; `progress` patches the existing in-flight row; `done`/
    // `error` flip the phase + bytes. Rows are keyed by
    // `jobId + src` so rapid repeated events collapse onto one row.
    onEvent<FileActivityDto>(EVENTS.fileActivity, (p) => {
      queueActivityEvent(p);
    }),
    // Post-Phase-12 paste-hotkey + clipboard-watcher. `count > 0`
    // with `paths` is a live detection ("files appeared on the
    // clipboard"); `count = 0` is the "hotkey fired but nothing to
    // paste" ping. Both surface as toasts — the hotkey's empty case
    // tells the user why nothing happened; the non-empty case
    // hints at the combo.
    onEvent<ClipboardFilesDetectedDto>(
      EVENTS.clipboardFilesDetected,
      (p) => {
        if (p.count === 0) {
          pushToast("info", "toast-clipboard-no-files");
          return;
        }
        // Store last-seen combo so the toast key can interpolate
        // the user-configured shortcut. For now the toast message
        // is a fixed key; the count and combo flow via a future
        // Fluent argument, keeping parity rules simple.
        pushToast("info", "toast-clipboard-files-detected");
      },
    ),
    // Phase 45.3 — registry events. `queue-added` / `queue-removed`
    // / `queue-merged` mutate the tab strip; `queue-job-routed` only
    // updates badge counts (the snapshot already captures who owns
    // which job). Cheap to re-fetch — the registry lives in-process
    // and the snapshot iterates a `Vec<Queue>`.
    onEvent<unknown>(EVENTS.queueAdded, () => {
      void refreshQueues();
    }),
    onEvent<unknown>(EVENTS.queueRemoved, () => {
      void refreshQueues();
    }),
    onEvent<unknown>(EVENTS.queueMerged, () => {
      void refreshQueues();
    }),
    onEvent<unknown>(EVENTS.queueJobRouted, () => {
      void refreshQueues();
    }),
  ]);

  return () => {
    for (const un of unlisten) un();
  };
}

function updateJob(id: number, patch: (job: JobDto) => JobDto) {
  jobsStore.update(($jobs) => {
    let changed = false;
    const next = $jobs.map((job) => {
      if (job.id !== id) return job;
      changed = true;
      return patch(job);
    });
    return changed ? next : $jobs;
  });
}

export function clearDropped(): void {
  droppedStore.set([]);
}

/// Programmatic push into the staging queue. Used by the Header's
/// file/folder picker buttons — the DropStagingDialog subscribes to
/// `dropped`, so seeding it from anywhere in the UI makes the same
/// "pick destination → start copy" flow kick in.
export function setDropped(paths: string[]): void {
  droppedStore.set(paths);
}

let toastCounter = 0;

/// Schedule a toast. `message` may be either a Fluent key (short,
/// kebab-case) or an already-localised string — the view layer
/// decides which lookup to run.
export function pushToast(
  kind: ToastKind,
  message: string,
  timeoutMs = 4000,
): number {
  const id = ++toastCounter;
  const toast: ToastMessage = { id, kind, message, timeoutMs };
  toastStore.update(($t) => [...$t, toast]);
  if (timeoutMs > 0) {
    setTimeout(() => dismissToast(id), timeoutMs);
  }
  return id;
}

export function dismissToast(id: number): void {
  toastStore.update(($t) => $t.filter((t) => t.id !== id));
}
