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

export interface CopyOptionsDto {
  verify?: string;
  preserveTimes?: boolean;
  preservePermissions?: boolean;
  fsyncOnClose?: boolean;
  followSymlinks?: boolean;
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
} as const;

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
