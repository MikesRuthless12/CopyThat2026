// Svelte stores that mirror the Rust-side queue.
//
// The data flow is one-way: Rust is the source of truth, the UI
// receives events and rebuilds state. Commands that mutate state
// (pause/resume/cancel) fire over IPC and wait for the matching
// event to come back before updating; this keeps the UI and the
// engine in sync even when the engine rejects or ignores a request.

import { derived, writable, type Readable } from "svelte/store";

import { listJobs, globals as fetchGlobals, onEvent } from "./ipc";
import {
  EVENTS,
  type DropReceivedDto,
  type GlobalsDto,
  type JobDto,
  type JobFailedDto,
  type JobIdDto,
  type JobProgressDto,
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

/// The sum of live per-job rates. `GlobalsDto.rateBps` from Rust is
/// intentionally 0 — derivating it in the UI keeps the numbers tied
/// to the store we already trust.
export const liveRateBps: Readable<number> = derived(jobsStore, ($jobs) =>
  $jobs
    .filter((j) => j.state === "running")
    .reduce((acc, j) => acc + (j.rateBps || 0), 0),
);

/// Convenience: jobs visible in the list, in stable id order.
export const visibleJobs: Readable<JobDto[]> = derived(jobsStore, ($jobs) =>
  [...$jobs].sort((a, b) => a.id - b.id),
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

  const unlisten = await Promise.all([
    onEvent<JobDto>(EVENTS.jobAdded, (job) => {
      jobsStore.update(($jobs) => {
        if ($jobs.some((j) => j.id === job.id)) return $jobs;
        return [...$jobs, job];
      });
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
