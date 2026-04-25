// Phase 37 — wire vocabulary the data channel carries. Mirror of
// `copythat_mobile::server::{RemoteCommand, RemoteResponse}` in
// Rust. Keep this file in lockstep with `crates/copythat-mobile/src/server.rs`.

export type CollisionAction =
  | "overwrite"
  | "overwrite_all"
  | "skip"
  | "skip_all"
  | "rename"
  | "keep_both";

export type RemoteCommand =
  | {
      kind: "hello";
      phone_pubkey_hex: string;
      device_label: string;
    }
  | { kind: "list_jobs" }
  | { kind: "pause_job"; job_id: string }
  | { kind: "resume_job"; job_id: string }
  | { kind: "cancel_job"; job_id: string }
  | {
      kind: "resolve_collision";
      prompt_id: string;
      action: CollisionAction;
    }
  | { kind: "globals" }
  | { kind: "recent_history"; limit: number }
  | { kind: "rerun_history"; row_id: number }
  | { kind: "secure_delete"; paths: string[]; method: string }
  | {
      kind: "start_copy";
      sources: string[];
      destination: string;
      verify: string | null;
    }
  | { kind: "goodbye" }
  | { kind: "set_keep_awake"; enabled: boolean };

export interface JobSummary {
  jobId: string;
  kind: string;
  state: string;
  src: string;
  dst: string;
  bytesDone: number;
  bytesTotal: number;
  rateBps: number;
}

export interface HistoryRow {
  rowId: number;
  kind: string;
  status: string;
  startedAtMs: number;
  finishedAtMs: number | null;
  srcRoot: string;
  dstRoot: string;
  totalBytes: number;
  filesOk: number;
  filesFailed: number;
}

export type RemoteResponse =
  | { kind: "hello_ack"; paired: boolean }
  | { kind: "jobs"; jobs: JobSummary[] }
  | {
      kind: "globals";
      bytes_done: number;
      bytes_total: number;
      files_done: number;
      files_total: number;
      rate_bps: number;
    }
  | { kind: "history"; rows: HistoryRow[] }
  | { kind: "ok" }
  | { kind: "error"; message: string }
  | {
      kind: "job_progress";
      job_id: string;
      bytes_done: number;
      bytes_total: number;
      rate_bps: number;
    }
  | { kind: "job_completed"; job_id: string; bytes: number }
  | { kind: "job_failed"; job_id: string; reason: string }
  | {
      kind: "globals_tick";
      bytes_done: number;
      bytes_total: number;
      files_done: number;
      files_total: number;
      rate_bps: number;
      copy_files: number;
      move_files: number;
      secure_delete_files: number;
    }
  | {
      kind: "file_tick";
      job_id: string;
      action: string;
      src: string;
      dst: string;
      bytes_done: number;
      bytes_total: number;
    }
  | { kind: "job_loading"; job_id: string; message: string }
  | { kind: "job_ready"; job_id: string }
  | { kind: "job_state_changed"; job_id: string; state: string }
  | { kind: "server_shutting_down"; reason: string };
