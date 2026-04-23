// Thin wrappers over Tauri's `invoke` / `listen` so the rest of the
// frontend imports typed functions instead of stringly-typed channels.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open as openDialog } from "@tauri-apps/plugin-dialog";

/// Open the native file picker (multi-select). Returns [] if the
/// user cancels. The Header "Add files" button pipes the result
/// into the staging queue, where the DropStagingDialog handles
/// destination + copy / move.
export async function pickFiles(): Promise<string[]> {
  const picked = await openDialog({ multiple: true, directory: false });
  if (picked === null || picked === undefined) return [];
  return Array.isArray(picked) ? picked : [picked];
}

/// Open the native folder picker (multi-select). Same flow as
/// `pickFiles` but `directory: true`.
export async function pickFolders(): Promise<string[]> {
  const picked = await openDialog({ multiple: true, directory: true });
  if (picked === null || picked === undefined) return [];
  return Array.isArray(picked) ? picked : [picked];
}

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

// ---------- Phase 12 settings + profiles ----------

import type { ProfileInfoDto, SettingsDto } from "./types";

export async function getSettings(): Promise<SettingsDto> {
  return invoke<SettingsDto>("get_settings");
}

export async function updateSettings(dto: SettingsDto): Promise<SettingsDto> {
  return invoke<SettingsDto>("update_settings", { dto });
}

export async function resetSettings(): Promise<SettingsDto> {
  return invoke<SettingsDto>("reset_settings");
}

/// Debug hook used by the Phase 12 smoke test; returns the clamped
/// buffer size the engine would use given current settings.
export async function effectiveBufferSize(): Promise<number> {
  return invoke<number>("effective_buffer_size");
}

export async function listProfiles(): Promise<ProfileInfoDto[]> {
  return invoke<ProfileInfoDto[]>("list_profiles");
}

export async function saveProfile(name: string): Promise<ProfileInfoDto> {
  return invoke<ProfileInfoDto>("save_profile", { name });
}

export async function loadProfile(name: string): Promise<SettingsDto> {
  return invoke<SettingsDto>("load_profile", { name });
}

export async function deleteProfile(name: string): Promise<void> {
  await invoke("delete_profile", { name });
}

export async function exportProfile(
  name: string,
  dest: string,
): Promise<void> {
  await invoke("export_profile", { name, dest });
}

export async function importProfile(
  name: string,
  src: string,
): Promise<ProfileInfoDto> {
  return invoke<ProfileInfoDto>("import_profile", { name, src });
}

export type PostCompletionAction =
  | "keep-open"
  | "exit"
  | "shutdown"
  | "logoff"
  | "sleep";

export async function postCompletionAction(
  action: PostCompletionAction,
): Promise<void> {
  await invoke("post_completion_action", { action });
}

// Phase 14 — preflight space checks.
export async function destinationFreeBytes(path: string): Promise<number> {
  return invoke<number>("destination_free_bytes", { path });
}

export async function pathTotalBytes(paths: string[]): Promise<number> {
  return invoke<number>("path_total_bytes", { paths });
}

export async function pathSizesIndividual(paths: string[]): Promise<number[]> {
  return invoke<number[]>("path_sizes_individual", { paths });
}

export interface PathMetaDto {
  isDir: boolean;
  size: number;
}

export async function pathMetadata(paths: string[]): Promise<PathMetaDto[]> {
  return invoke<PathMetaDto[]>("path_metadata", { paths });
}

export interface TreeFileDto {
  path: string;
  size: number;
}

export interface TreeEnumerationDto {
  files: TreeFileDto[];
  overflow: boolean;
}

export async function enumerateTreeFiles(
  paths: string[],
): Promise<TreeEnumerationDto> {
  return invoke<TreeEnumerationDto>("enumerate_tree_files", { paths });
}

// ---------- Phase 15 updater ----------

import type { UpdateCheckDto } from "./types";

/** Ask the backend to hit the configured updater endpoint for the
 *  current channel. When `force` is false, the 24 h throttle gates
 *  the network call and the reply carries `skippedByThrottle = true`
 *  alongside the stored `lastCheckUnixSecs`. `endpointOverride` is
 *  for tests — production callers pass `null`. */
export async function updaterCheckNow(
  force: boolean,
  endpointOverride: string | null = null,
): Promise<UpdateCheckDto> {
  return invoke<UpdateCheckDto>("updater_check_now", {
    force,
    endpointOverride,
  });
}

// ---------- Phase 21 bandwidth shape ----------

import type { ShapeRateDto } from "./types";

/** Snapshot the bandwidth-shaping rate for the header badge.
 *  `bytesPerSecond` is `null` when the shape is unlimited. */
export async function currentShapeRate(): Promise<ShapeRateDto> {
  return invoke<ShapeRateDto>("current_shape_rate");
}

/** Validate an rclone-style schedule spec without persisting it.
 *  Returns the rule count on success, throws with the parser error
 *  message on failure. The Settings textarea uses this for inline
 *  feedback as the user types. */
export async function validateScheduleSpec(spec: string): Promise<number> {
  return invoke<number>("validate_schedule_spec", { spec });
}

/** Persist that the user dismissed the named version. Empty string
 *  clears the dismissal. */
export async function updaterDismissVersion(version: string): Promise<void> {
  await invoke("updater_dismiss_version", { version });
}

// ---------- Phase 22 aggregate conflict dialog ----------

import type {
  ConflictProfileDto,
  ConflictRuleResolution,
  ThumbnailDto,
} from "./types";

/** Fetch a thumbnail for `path`. Images return a `data:image/png;base64,…`
 *  URL the caller drops into `<img>`; other types return a file-kind
 *  icon descriptor. Result is cached on disk by `(path, mtime, size,
 *  maxDim)`. */
export async function thumbnailFor(
  path: string,
  maxDim: number = 240,
): Promise<ThumbnailDto> {
  return invoke<ThumbnailDto>("thumbnail_for", { path, maxDim });
}

/** Append one pattern → resolution rule to a running job's live
 *  rule set. Subsequent collisions matching `pattern` auto-resolve
 *  without surfacing another prompt. Returns the new rule count. */
export async function addConflictRule(
  jobId: number,
  pattern: string,
  resolution: ConflictRuleResolution,
): Promise<number> {
  return invoke<number>("add_conflict_rule", { jobId, pattern, resolution });
}

/** Snapshot the live rule list for a running job — populates the
 *  "Save these rules as profile…" dialog. */
export async function currentConflictRules(
  jobId: number,
): Promise<ConflictProfileDto> {
  return invoke<ConflictProfileDto>("current_conflict_rules", { jobId });
}

/** List saved conflict-profile names (alphabetical). */
export async function listConflictProfiles(): Promise<string[]> {
  return invoke<string[]>("list_conflict_profiles");
}

/** Save `profile` under `name`, replacing any existing entry. */
export async function saveConflictProfile(
  name: string,
  profile: ConflictProfileDto,
): Promise<string[]> {
  return invoke<string[]>("save_conflict_profile", { name, profile });
}

/** Delete a saved conflict profile. */
export async function deleteConflictProfile(name: string): Promise<string[]> {
  return invoke<string[]>("delete_conflict_profile", { name });
}

/** Activate a saved profile (empty string clears). */
export async function setActiveConflictProfile(
  name: string,
): Promise<string | null> {
  return invoke<string | null>("set_active_conflict_profile", { name });
}
