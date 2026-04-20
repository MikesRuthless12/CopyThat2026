// TypeScript mirrors of the Rust IPC DTOs in `src-tauri/src/ipc.rs`.
//
// We don't auto-generate these from Rust (no ts-rs dep yet — Phase 7
// can revisit). Keep them in sync by hand: any change to `ipc.rs`
// must land here in the same commit.

export type JobKind = "copy" | "move" | "delete" | "secure-delete" | "verify";
export type JobState =
  | "pending"
  | "running"
  | "paused"
  | "cancelled"
  | "succeeded"
  | "failed";

export type GlobalState = "idle" | "copying" | "paused" | "verifying" | "error";

export interface JobDto {
  id: number;
  kind: JobKind;
  state: JobState;
  src: string;
  dst: string | null;
  name: string;
  subpath: string | null;
  bytesDone: number;
  bytesTotal: number;
  filesDone: number;
  filesTotal: number;
  rateBps: number;
  etaSeconds: number | null;
  startedAtMs: number | null;
  finishedAtMs: number | null;
  lastError: string | null;
}

export interface JobProgressDto {
  id: number;
  bytesDone: number;
  bytesTotal: number;
  filesDone: number;
  filesTotal: number;
  rateBps: number;
  etaSeconds: number | null;
}

export interface JobIdDto {
  id: number;
}

export interface JobFailedDto {
  id: number;
  message: string;
}

export interface GlobalsDto {
  state: GlobalState;
  activeJobs: number;
  queuedJobs: number;
  pausedJobs: number;
  failedJobs: number;
  succeededJobs: number;
  bytesDone: number;
  bytesTotal: number;
  rateBps: number;
  etaSeconds: number | null;
  errors: number;
}

export interface DropReceivedDto {
  paths: string[];
}

export interface FileIconDto {
  kind:
    | "folder"
    | "symlink"
    | "file"
    | "image"
    | "audio"
    | "video"
    | "archive"
    | "text"
    | "code"
    | "pdf"
    | "binary";
  extension: string | null;
}

export type CollisionPolicyWire =
  | "skip"
  | "overwrite"
  | "overwrite-if-newer"
  | "keep-both"
  | "prompt";

export type ErrorPolicyDto =
  | { kind: "ask" }
  | { kind: "skip" }
  | { kind: "abort" }
  | { kind: "retryN"; maxAttempts: number; backoffMs: number };

export interface CopyOptionsDto {
  verify?: string;
  preserveTimes?: boolean;
  preservePermissions?: boolean;
  fsyncOnClose?: boolean;
  followSymlinks?: boolean;
  // Phase 8
  onError?: ErrorPolicyDto;
  collision?: CollisionPolicyWire;
}

export const EVENTS = {
  jobAdded: "job-added",
  jobStarted: "job-started",
  jobProgress: "job-progress",
  jobPaused: "job-paused",
  jobResumed: "job-resumed",
  jobCancelled: "job-cancelled",
  jobCompleted: "job-completed",
  jobFailed: "job-failed",
  jobRemoved: "job-removed",
  globalsTick: "globals-tick",
  dropReceived: "drop-received",
  shellEnqueue: "shell-enqueue",
  // Phase 8
  errorRaised: "error-raised",
  errorResolved: "error-resolved",
  collisionRaised: "collision-raised",
  collisionResolved: "collision-resolved",
} as const;

// Phase 8 DTOs
export type ErrorKind =
  | "not-found"
  | "permission-denied"
  | "disk-full"
  | "interrupted"
  | "verify-failed"
  | "io-other";

export type ErrorAction = "retry" | "skip" | "abort";

export type CollisionAction = "skip" | "overwrite" | "rename" | "abort";

export interface ErrorPromptDto {
  id: number;
  jobId: number;
  src: string;
  dst: string;
  kind: ErrorKind;
  localizedKey: string;
  message: string;
  rawOsError: number | null;
  createdAtMs: number;
}

export interface CollisionPromptDto {
  id: number;
  jobId: number;
  src: string;
  dst: string;
  srcSize: number | null;
  srcModifiedMs: number | null;
  dstSize: number | null;
  dstModifiedMs: number | null;
}

export interface ErrorResolvedDto {
  id: number;
  jobId: number;
  action: ErrorAction;
}

export interface CollisionResolvedDto {
  id: number;
  jobId: number;
  resolution: CollisionAction;
}

export interface LoggedErrorDto {
  id: number;
  jobId: number;
  timestampMs: number;
  src: string;
  dst: string;
  kind: ErrorKind;
  localizedKey: string;
  message: string;
  rawOsError: number | null;
  resolution: "retry" | "skip" | "abort" | "auto-skip" | null;
}

export type ToastKind = "info" | "success" | "error";

export interface ToastMessage {
  id: number;
  kind: ToastKind;
  message: string;
  timeoutMs: number;
}

export interface ContextMenuItem {
  id: string;
  label: string;
  icon?:
    | "pause"
    | "play"
    | "x"
    | "trash"
    | "refresh"
    | "external-link"
    | "info";
  tone?: "default" | "danger";
  disabled?: boolean;
  onClick: () => void;
}

// Phase 9 — History DTOs (mirror the Rust `HistoryJobDto`,
// `HistoryItemDto`, `HistoryFilterDto` in `src-tauri/src/ipc.rs`).

export interface HistoryJobDto {
  rowId: number;
  kind: string;
  status: "running" | "succeeded" | "failed" | "cancelled" | string;
  startedAtMs: number;
  finishedAtMs: number | null;
  srcRoot: string;
  dstRoot: string;
  totalBytes: number;
  filesOk: number;
  filesFailed: number;
  verifyAlgo: string | null;
  optionsJson: string | null;
}

export interface HistoryItemDto {
  jobRowId: number;
  src: string;
  dst: string;
  size: number;
  status: "ok" | "failed" | "skipped" | "cancelled" | string;
  hashHex: string | null;
  errorCode: string | null;
  errorMsg: string | null;
  timestampMs: number;
}

export interface HistoryFilterDto {
  startedSinceMs?: number;
  startedUntilMs?: number;
  kind?: string;
  status?: string;
  text?: string;
  limit?: number;
}

// Phase 10 — lifetime aggregates (mirror the Rust `TotalsDto`,
// `KindBreakdownDto`, `DayTotalDto`).

export interface KindBreakdownDto {
  kind: string;
  bytes: number;
  files: number;
  jobs: number;
}

export interface TotalsDto {
  bytes: number;
  files: number;
  jobs: number;
  errors: number;
  durationMs: number;
  byKind: KindBreakdownDto[];
}

export interface DayTotalDto {
  dateMs: number;
  bytes: number;
  files: number;
  jobs: number;
}
