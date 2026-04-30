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

// -------------------------------------------------------------------
// Phase 29 — destination picker + drag-out staging.
// -------------------------------------------------------------------

import type { DirChildDto, DragOutStagedDto } from "./types";

/** List the immediate subdirectories of `path`. */
export async function listDirectory(path: string): Promise<DirChildDto[]> {
  return invoke<DirChildDto[]>("list_directory", { path });
}

/** List the filesystem roots (drive letters on Windows; `/` on Unix). */
export async function listRoots(): Promise<DirChildDto[]> {
  return invoke<DirChildDto[]>("list_roots");
}

/** Drag-out staging stub — validates paths still exist on disk so the
 *  frontend can decide whether to emit an OS-native drag or fall back
 *  to the in-app drop target. */
export async function dragOutStage(
  paths: string[],
): Promise<DragOutStagedDto> {
  return invoke<DragOutStagedDto>("drag_out_stage", { paths });
}

// ---------------------------------------------------------------------
// Phase 32 — cloud backend matrix.
// ---------------------------------------------------------------------

/** Wire-form for a configured remote backend. Mirrors the Rust
 *  `cloud_commands::BackendDto`. The `config` sub-object carries
 *  kind-specific fields (bucket, region, host, etc.) as plain
 *  strings so the wizard can keep one shared form shape. */
export type BackendDto = {
  name: string;
  kind: string;
  config: {
    root: string;
    bucket: string;
    region: string;
    endpoint: string;
    container: string;
    accountName: string;
    serviceAccount: string;
    clientId: string;
    host: string;
    username: string;
    port: number;
  };
  enabledInBuild: boolean;
};

/** Empty backend config — used as the initial-state for the add-
 *  backend wizard so every field is defined before submit. */
export function emptyBackendConfig(): BackendDto["config"] {
  return {
    root: "",
    bucket: "",
    region: "",
    endpoint: "",
    container: "",
    accountName: "",
    serviceAccount: "",
    clientId: "",
    host: "",
    username: "",
    port: 0,
  };
}

/** Response shape for `test_backend_connection`. `reason` maps to a
 *  `cloud-error-<reason>` Fluent key. */
export type TestConnectionResult = {
  ok: boolean;
  reason: string | null;
  detail: string | null;
};

/** Enumerate every backend persisted in `settings.remotes.backends`. */
export async function listBackends(): Promise<BackendDto[]> {
  return invoke<BackendDto[]>("list_backends");
}

/** Insert a new backend + (optionally) save its secret to the OS keychain. */
export async function addBackend(
  dto: BackendDto,
  secret: string | null,
): Promise<BackendDto> {
  return invoke<BackendDto>("add_backend", { dto, secret });
}

/** Overwrite an existing backend entry. Pass `null` for `secret` to
 *  leave the stored credential untouched. */
export async function updateBackend(
  dto: BackendDto,
  secret: string | null,
): Promise<BackendDto> {
  return invoke<BackendDto>("update_backend", { dto, secret });
}

/** Remove a backend entry + its keychain secret. */
export async function removeBackend(name: string): Promise<void> {
  await invoke("remove_backend", { name });
}

/** Test a live connection to `name`. Errors are reported in-band via
 *  the returned DTO. */
export async function testBackendConnection(
  name: string,
): Promise<TestConnectionResult> {
  return invoke<TestConnectionResult>("test_backend_connection", { name });
}

/** Phase 32c — push a local file to the configured backend at `dstKey`.
 *  Returns the number of bytes written. */
export async function copyLocalToBackend(
  backendName: string,
  srcPath: string,
  dstKey: string,
): Promise<number> {
  return invoke<number>("copy_local_to_backend", {
    backendName,
    srcPath,
    dstKey,
  });
}

/** Phase 32c — pull an object from the backend to the local filesystem.
 *  Returns the number of bytes written. */
export async function copyBackendToLocal(
  backendName: string,
  srcKey: string,
  dstPath: string,
): Promise<number> {
  return invoke<number>("copy_backend_to_local", {
    backendName,
    srcKey,
    dstPath,
  });
}

// ---------------------------------------------------------------------
// Phase 40 — SMB compression probe + cloud-VM offload-template wizard.
// ---------------------------------------------------------------------

/** Wire shape for `copythat_platform::smb::SmbCompressionState`.
 *  `algorithm` is `null` on every host today; the field is plumbed
 *  for forward compatibility once the kernel surfaces the negotiated
 *  algorithm to user mode. */
export type SmbCompressionStateDto = {
  supported: boolean;
  algorithm: string | null;
};

/** Probe a destination path. UNC dests on Windows return
 *  `{ supported: true, algorithm: null }`; everything else returns
 *  `{ supported: false, algorithm: null }`. Pure path-prefix check —
 *  no syscalls. */
export async function smbCompressionState(
  dstPath: string,
): Promise<SmbCompressionStateDto> {
  return invoke<SmbCompressionStateDto>("smb_compression_state", { dstPath });
}

/** Knob set the wizard hands to `render_offload_template`.
 *  Mirrors `copythat_cloud::offload::OffloadOpts`. */
export type OffloadOptsDto = {
  jobName: string;
  copythatRelease: string;
  instanceSize: string;
  region: string;
  iamRole: string;
  selfDestructMinutes: number;
};

/** Default knob set so the wizard form is fully populated on first
 *  render. Keeps the front-end source-of-truth in sync with the
 *  Rust side's `OffloadOpts::default`. */
export function defaultOffloadOpts(): OffloadOptsDto {
  return {
    jobName: "copythat-offload",
    copythatRelease: "v1.0.0",
    instanceSize: "t3.small",
    region: "us-east-1",
    iamRole: "copythat-offload-role",
    selfDestructMinutes: 60,
  };
}

/** Stable wire string for the four output formats. Matches the
 *  Rust `OffloadTemplateFormat` enum's serde rename. */
export type OffloadTemplateFormat =
  | "cloud-init"
  | "aws-terraform"
  | "az-arm"
  | "gcp-deployment";

/** Render a single deployment template body. Pure function —
 *  no side effects, no network. The returned string is what the
 *  user pastes into their cloud's console / `terraform apply`. */
export async function renderOffloadTemplate(
  format: OffloadTemplateFormat,
  src: BackendDto,
  dst: BackendDto,
  opts: OffloadOptsDto,
): Promise<string> {
  // Backend wire form is `Backend { name, kind, config }`. The
  // `enabledInBuild` flag is UI-only and is dropped before invoke.
  const stripFlag = (b: BackendDto) => ({
    name: b.name,
    kind: b.kind,
    config: b.config,
  });
  return invoke<string>("render_offload_template", {
    format,
    src: stripFlag(src),
    dst: stripFlag(dst),
    opts: {
      job_name: opts.jobName,
      copythat_release: opts.copythatRelease,
      instance_size: opts.instanceSize,
      region: opts.region,
      iam_role: opts.iamRole,
      self_destruct_minutes: opts.selfDestructMinutes,
    },
  });
}

// ---------------------------------------------------------------------
// Phase 41 — pre-execution tree-diff preview.
// ---------------------------------------------------------------------

/** Mirrors `copythat_core::dryrun::DryRunOptions`. The Tauri command
 *  accepts camelCase (the `DryRunOptionsDto` derives serde with
 *  `rename_all = "camelCase"`). */
export type DryRunOptionsDto = {
  forceOverwrite: boolean;
  trustSizeMtime: boolean;
};

export type ReplacementRow = {
  relPath: string;
  /** Stable wire string: `"source-newer"` / `"force-overwrite-older"` /
   *  `"content-different"`. */
  reason: string;
};

export type SkipRow = {
  relPath: string;
  /** Stable wire string: `"identical-content"` / `"destination-newer"` /
   *  `"filtered-out"` / `"unsupported-source-kind"`. */
  reason: string;
};

export type ConflictRow = {
  relPath: string;
  /** Stable wire string: `"both-modified-since-common-ancestor"` /
   *  `"kind-mismatch"` / `"ambiguous"`. */
  kind: string;
};

/** Wire shape for `copythat_core::dryrun::TreeDiff`. Vectors are
 *  grouped per category so the preview modal can render each colour
 *  band without re-bucketing. */
export type TreeDiffDto = {
  additions: string[];
  replacements: ReplacementRow[];
  skips: SkipRow[];
  conflicts: ConflictRow[];
  unchanged: string[];
  bytesToTransfer: number;
  bytesTotal: number;
  totalFiles: number;
  hasBlockingConflicts: boolean;
};

/** Render a dry-run plan for `src` → `dst`. Single-threaded; the
 *  Settings → "Show preview before running copies" toggle gates this
 *  call on a sane size threshold so multi-million-file jobs don't
 *  stall the UI. */
export async function computeTreeDiff(
  src: string,
  dst: string,
  opts: DryRunOptionsDto,
): Promise<TreeDiffDto> {
  return invoke<TreeDiffDto>("compute_tree_diff", { src, dst, opts });
}

// ---------------------------------------------------------------------
// Phase 33 — mount-as-filesystem.
// ---------------------------------------------------------------------

/** Wire-form for one live mount. `jobRowId` matches the history
 *  `jobs.row_id` primary key. */
export type MountDto = {
  jobRowId: number;
  mountpoint: string;
};

/** Enumerate currently-active mounts. */
export async function listMounts(): Promise<MountDto[]> {
  return invoke<MountDto[]>("list_mounts");
}

/** Mount the snapshot for a history row at `mountpoint`. */
export async function mountSnapshot(
  jobRowId: number,
  mountpoint: string,
): Promise<MountDto> {
  return invoke<MountDto>("mount_snapshot", { jobRowId, mountpoint });
}

/** Unmount the snapshot for `jobRowId`. */
export async function unmountSnapshot(jobRowId: number): Promise<void> {
  await invoke("unmount_snapshot", { jobRowId });
}

/** Phase 33c — report which mount backend this build selected:
 *  `"fuse"` (Linux/macOS with `--features fuse`), `"winfsp"`
 *  (Windows with `--features winfsp`), or `"noop"` (default build;
 *  real kernel callbacks ship in Phase 33d). */
export async function mountBackendName(): Promise<string> {
  return invoke<string>("mount_backend_name");
}

// ---------------------------------------------------------------------
// Phase 45.3 — named-queue tab strip.
// ---------------------------------------------------------------------

import type { QueueSnapshotDto } from "./types";

/** Snapshot the registry — one entry per named queue, with the
 *  badge count the tab strip uses. The legacy default queue is NOT
 *  included; the frontend synthesises its tab from the visibleJobs
 *  store so Phase 45.4+ runner reconciliation can move jobs into
 *  registry queues without UI changes. */
export async function queueList(): Promise<QueueSnapshotDto[]> {
  return invoke<QueueSnapshotDto[]>("queue_list");
}

/** Phase 45.4 — collapse `srcId` into `dstId`. The Rust side moves
 *  every job, removes the source queue, and emits `queue-merged` +
 *  `queue-removed` events; the frontend's event listeners refresh
 *  the tab strip on receipt. No-op when ids are equal; returns a
 *  rejected promise when either id is unknown. */
export async function queueMerge(srcId: number, dstId: number): Promise<void> {
  await invoke("queue_merge", { srcId, dstId });
}
