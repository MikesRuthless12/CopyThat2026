// Thin wrappers over Tauri's `invoke` / `listen` so the rest of the
// frontend imports typed functions instead of stringly-typed channels.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type {
  CollisionAction,
  CopyOptionsDto,
  ErrorAction,
  FileIconDto,
  GlobalsDto,
  JobDto,
  LoggedErrorDto,
} from "./types";

export async function startCopy(
  sources: string[],
  destination: string,
  options?: CopyOptionsDto,
): Promise<number[]> {
  return invoke<number[]>("start_copy", { sources, destination, options });
}

export async function startMove(
  sources: string[],
  destination: string,
  options?: CopyOptionsDto,
): Promise<number[]> {
  return invoke<number[]>("start_move", { sources, destination, options });
}

export async function pauseJob(id: number): Promise<void> {
  await invoke("pause_job", { id });
}

export async function resumeJob(id: number): Promise<void> {
  await invoke("resume_job", { id });
}

export async function cancelJob(id: number): Promise<void> {
  await invoke("cancel_job", { id });
}

export async function removeJob(id: number): Promise<void> {
  await invoke("remove_job", { id });
}

export async function pauseAll(): Promise<void> {
  await invoke("pause_all");
}

export async function resumeAll(): Promise<void> {
  await invoke("resume_all");
}

export async function cancelAll(): Promise<void> {
  await invoke("cancel_all");
}

export async function listJobs(): Promise<JobDto[]> {
  return invoke<JobDto[]>("list_jobs");
}

export async function globals(): Promise<GlobalsDto> {
  return invoke<GlobalsDto>("globals");
}

export async function fileIcon(path: string): Promise<FileIconDto> {
  return invoke<FileIconDto>("file_icon", { path });
}

export async function revealInFolder(path: string): Promise<void> {
  await invoke("reveal_in_folder", { path });
}

export async function translations(
  locale: string,
): Promise<Record<string, string>> {
  return invoke<Record<string, string>>("translations", { locale });
}

export async function availableLocales(): Promise<string[]> {
  return invoke<string[]>("available_locales");
}

export async function systemLocale(): Promise<string> {
  return invoke<string>("system_locale");
}

export async function onEvent<T>(
  name: string,
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(name, (event) => handler(event.payload));
}

// ---------- Phase 8 error / collision / log ----------

export async function resolveError(
  id: number,
  action: ErrorAction,
  applyToAll: boolean = false,
): Promise<void> {
  await invoke("resolve_error", { id, action, applyToAll });
}

export async function resolveCollision(
  id: number,
  resolution: CollisionAction,
  renameTo: string | null = null,
  applyToAll: boolean = false,
): Promise<void> {
  await invoke("resolve_collision", { id, resolution, renameTo, applyToAll });
}

export async function errorLog(): Promise<LoggedErrorDto[]> {
  return invoke<LoggedErrorDto[]>("error_log");
}

export async function clearErrorLog(): Promise<void> {
  await invoke("clear_error_log");
}

export async function errorLogExport(
  format: "csv" | "txt",
  path: string,
): Promise<number> {
  return invoke<number>("error_log_export", { format, path });
}

export async function retryElevated(id: number): Promise<void> {
  await invoke("retry_elevated", { id });
}

// ---------- Phase 9 history ----------

import type {
  DayTotalDto,
  HistoryFilterDto,
  HistoryItemDto,
  HistoryJobDto,
  TotalsDto,
} from "./types";

export async function historySearch(
  filter?: HistoryFilterDto,
): Promise<HistoryJobDto[]> {
  return invoke<HistoryJobDto[]>("history_search", { filter });
}

export async function historyItems(rowId: number): Promise<HistoryItemDto[]> {
  return invoke<HistoryItemDto[]>("history_items", { rowId });
}

export async function historyPurge(days: number): Promise<number> {
  return invoke<number>("history_purge", { days });
}

export async function historyExportCsv(
  path: string,
  filter?: HistoryFilterDto,
): Promise<number> {
  return invoke<number>("history_export_csv", { path, filter });
}

export async function historyRerun(rowId: number): Promise<number[]> {
  return invoke<number[]>("history_rerun", { rowId });
}

// ---------- Phase 10 totals ----------

export async function historyTotals(sinceMs?: number): Promise<TotalsDto> {
  return invoke<TotalsDto>("history_totals", { sinceMs });
}

export async function historyDaily(sinceMs: number): Promise<DayTotalDto[]> {
  return invoke<DayTotalDto[]>("history_daily", { sinceMs });
}

export async function historyClearAll(): Promise<number> {
  return invoke<number>("history_clear_all");
}
