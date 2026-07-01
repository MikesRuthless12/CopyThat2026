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

// Phase 31b — count of in-flight cloud transfers (S3 / SFTP / WebDAV).
// The header disables the Pause-all button while this is > 0, since a
// cloud transfer can't pause (only cancel). Pairs with the
// `cloud-transfers-changed` event for live updates.
export async function activeCloudTransferCount(): Promise<number> {
  return invoke<number>("active_cloud_transfer_count");
}

// Phase 31b — cancel one in-flight cloud transfer by its registry id.
export async function cancelCloudTransfer(id: number): Promise<boolean> {
  return invoke<boolean>("cancel_cloud_transfer", { id });
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

/** Phase 8 follow-up — compute the SHA-256 of one side of a collision
 *  so the user can decide overwrite-vs-skip on content, not just
 *  metadata. Returns the lowercase hex digest. */
export async function quickHashForCollision(path: string): Promise<string> {
  return invoke<string>("quick_hash_for_collision", { path });
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

import type { PinnedDestinationDto, QueueSnapshotDto } from "./types";

/** Snapshot the registry — one entry per named queue, with the
 *  badge count the tab strip uses. The legacy default queue is NOT
 *  included; the frontend synthesises its tab from the visibleJobs
 *  store so Phase 45.4+ runner reconciliation can move jobs into
 *  registry queues without UI changes. */
export async function queueList(): Promise<QueueSnapshotDto[]> {
  return invoke<QueueSnapshotDto[]>("queue_list");
}

/** Phase 45.6 — return the persisted pinned-destination list. The
 *  Settings panel and active-target store seed from this on mount. */
export async function queueGetPinned(): Promise<PinnedDestinationDto[]> {
  return invoke<PinnedDestinationDto[]>("queue_get_pinned");
}

/** Phase 45.6 — append a pinned destination. Returns the post-add
 *  list; duplicate `(label, path)` pairs are deduped server-side.
 *  Empty label or path rejects with a typed error. The Rust side
 *  also rebuilds the OS tray menu on success. */
export async function queuePinDestination(
  label: string,
  path: string,
): Promise<PinnedDestinationDto[]> {
  return invoke<PinnedDestinationDto[]>("queue_pin_destination", {
    label,
    path,
  });
}

/** Phase 45.6 — remove a pinned destination. Returns the post-
 *  remove list; idempotent (removing a row that isn't pinned is a
 *  no-op). Whitespace in the inputs is trimmed before the
 *  comparison. Rebuilds the OS tray menu on success. */
export async function queueUnpinDestination(
  label: string,
  path: string,
): Promise<PinnedDestinationDto[]> {
  return invoke<PinnedDestinationDto[]>("queue_unpin_destination", {
    label,
    path,
  });
}

/** Phase 45.4 — collapse `srcId` into `dstId`. The Rust side moves
 *  every job, removes the source queue, and emits `queue-merged` +
 *  `queue-removed` events; the frontend's event listeners refresh
 *  the tab strip on receipt. No-op when ids are equal; returns a
 *  rejected promise when either id is unknown. */
export async function queueMerge(srcId: number, dstId: number): Promise<void> {
  await invoke("queue_merge", { srcId, dstId });
}

/** Phase 45.5 — flip the registry's `auto_enqueue_next` atomic. When
 *  enabled, every subsequent `queue_route_job` lands in whichever
 *  queue currently owns a running job rather than spawning a new
 *  parallel queue. Transient state — never persisted. */
export async function queueSetF2Mode(enabled: boolean): Promise<void> {
  await invoke("queue_set_f2_mode", { enabled });
}

// ---------------------------------------------------------------------
// Phase 48 — server mode + observability (Settings → Server).
// ---------------------------------------------------------------------

/** Status snapshot for the Settings → Server panel. `metricsUrl` is set
 *  only while running with an HTTP-family protocol (WebDAV / HTTP). */
export interface ServerStatusDto {
  running: boolean;
  boundAddr: string | null;
  metricsUrl: string | null;
}

/** Build a server from the saved `ServerSettings` and start it. Throws
 *  with an `err-server-*` Fluent key on failure (no protocols, bind
 *  error, …). The panel persists the form via `update_settings` before
 *  calling this. */
export async function serverStart(): Promise<ServerStatusDto> {
  return invoke<ServerStatusDto>("server_start");
}

/** Stop the running server (idempotent). */
export async function serverStop(): Promise<ServerStatusDto> {
  return invoke<ServerStatusDto>("server_stop");
}

/** Read-only snapshot — running? bound address? metrics URL? */
export async function serverStatus(): Promise<ServerStatusDto> {
  return invoke<ServerStatusDto>("server_status");
}

// ---------------------------------------------------------------------
// Phase 46.6 — Settings → Plugins UI.
// ---------------------------------------------------------------------

/** One installed plugin as enumerated by `plugin_list`. Mirrors the
 *  Rust `PluginEntryDto`. */
export type PluginEntryDto = {
  name: string;
  version: string;
  hooks: string[];
  manifestCapabilities: string[];
  grantedCapabilities: string[];
  enabled: boolean;
  directory: string;
};

/** Result of a `plugin_install_from_url` call. `installed = false`
 *  when the call was a preview (no `expectedHash`); `installed = true`
 *  when the second confirm-phase call wrote into the plugin store. */
export type PluginInstallPreviewDto = {
  name: string;
  version: string;
  hash: string;
  hooks: string[];
  capabilities: string[];
  installed: boolean;
};

/** Enumerate every plugin under `<config_dir>/plugins/`. */
export async function pluginList(): Promise<PluginEntryDto[]> {
  return invoke<PluginEntryDto[]>("plugin_list");
}

/** Flip a plugin's `enabled` bit on. Returns the refreshed entry. */
export async function pluginEnable(name: string): Promise<PluginEntryDto> {
  return invoke<PluginEntryDto>("plugin_enable", { name });
}

/** Flip a plugin's `enabled` bit off. Returns the refreshed entry. */
export async function pluginDisable(name: string): Promise<PluginEntryDto> {
  return invoke<PluginEntryDto>("plugin_disable", { name });
}

/** Add a capability to the per-plugin grant. Refuses capabilities
 *  the manifest doesn't declare. Idempotent. */
export async function pluginGrantCapability(
  name: string,
  capability: string,
): Promise<PluginEntryDto> {
  return invoke<PluginEntryDto>("plugin_grant_capability", {
    name,
    capability,
  });
}

/** Remove a capability from the per-plugin grant. Idempotent. */
export async function pluginRevokeCapability(
  name: string,
  capability: string,
): Promise<PluginEntryDto> {
  return invoke<PluginEntryDto>("plugin_revoke_capability", {
    name,
    capability,
  });
}

/** Copy a wasm + manifest pair into the plugin store. `manifestPath`
 *  is optional — when omitted the host reads `plugin.toml` from the
 *  directory next to the wasm. */
export async function pluginInstallFromFile(args: {
  wasmPath: string;
  manifestPath?: string;
}): Promise<PluginEntryDto> {
  return invoke<PluginEntryDto>("plugin_install_from_file", { args });
}

/** Two-phase URL install. First call without `expectedHash` to get
 *  the BLAKE3 + manifest preview; second call with `expectedHash`
 *  set to the preview hash to commit. */
export async function pluginInstallFromUrl(args: {
  wasmUrl: string;
  manifestUrl: string;
  expectedHash?: string | null;
}): Promise<PluginInstallPreviewDto> {
  return invoke<PluginInstallPreviewDto>("plugin_install_from_url", { args });
}

// ---------- Phase 49 — content-addressed Library / Repository ----------

/** Dedup stats for the Library "hero" readout. Mirrors `RepoStatsDto`. */
export type RepositoryStatsDto = {
  storedBytes: number;
  uniqueBytes: number;
  effectiveBytes: number;
  snapshotCount: number;
  chunkCount: number;
  savedRatio: number;
  physicalUniqueBytes: number;
  compressionRatio: number;
};

/** At-rest compression policy (Phase 49h). Mirrors `RepoCompressionSettings`. */
export type RepoCompressionSettings =
  | { mode: "off" }
  | { mode: "auto"; level: number }
  | { mode: "always"; level: number };

export async function repositoryCompressionGet(): Promise<RepoCompressionSettings> {
  return invoke<RepoCompressionSettings>("repository_compression_get");
}

export async function repositoryCompressionSet(
  compression: RepoCompressionSettings,
): Promise<void> {
  return invoke<void>("repository_compression_set", { compression });
}

/** One task in the Tasks & progress center (Phase 49j). Mirrors `TaskDto`. */
export type TaskDto = {
  id: number;
  kind: string;
  label: string;
  state: "running" | "completed" | "failed" | "cancelled";
  progress: number;
  detail: string;
  startedAtMs: number;
  finishedAtMs: number | null;
  error: string | null;
};

export async function tasksList(): Promise<TaskDto[]> {
  return invoke<TaskDto[]>("tasks_list");
}

export async function taskGet(id: number): Promise<TaskDto | null> {
  return invoke<TaskDto | null>("task_get", { id });
}

export async function taskCancel(id: number): Promise<boolean> {
  return invoke<boolean>("task_cancel", { id });
}

/** Phase 49i — start a full compaction; returns the task id to watch in the center. */
export async function repositoryCompact(): Promise<number> {
  return invoke<number>("repository_compact");
}

/** One registered repository (Phase 49k). Mirrors `RepoEntryDto`. */
export type RepoEntryDto = {
  id: string;
  name: string;
  path: string;
  createdAtMs: number;
  active: boolean;
  requiresPassphrase: boolean;
};

export async function repositoryList(): Promise<RepoEntryDto[]> {
  return invoke<RepoEntryDto[]>("repository_list");
}

export async function repositoryActive(): Promise<RepoEntryDto | null> {
  return invoke<RepoEntryDto | null>("repository_active");
}

export async function repositoryCreate(
  name: string,
  path: string,
  password: string | null,
): Promise<RepoEntryDto> {
  return invoke<RepoEntryDto>("repository_create", { name, path, password });
}

export async function repositoryConnect(
  name: string,
  path: string,
  password: string | null,
): Promise<RepoEntryDto> {
  return invoke<RepoEntryDto>("repository_connect", { name, path, password });
}

export async function repositorySetActive(
  id: string,
  password: string | null,
): Promise<RepoEntryDto | null> {
  return invoke<RepoEntryDto | null>("repository_set_active", { id, password });
}

export async function repositoryDisconnect(id: string): Promise<void> {
  return invoke<void>("repository_disconnect", { id });
}

export async function repositoryChangePassword(
  oldPass: string | null,
  newPass: string,
): Promise<void> {
  return invoke<void>("repository_change_password", { old: oldPass, new: newPass });
}

/** One source-dashboard row (Phase 49l). Mirrors `RepoSourceDto`. */
export type RepoSourceDto = {
  source: string;
  snapshotCount: number;
  latestMs: number;
  latestKind: string;
  latestSize: number;
  totalFiles: number;
};

export async function repositorySources(): Promise<RepoSourceDto[]> {
  return invoke<RepoSourceDto[]>("repository_sources");
}

/** A verify pass result (Phase 49n). Mirrors `VerifyReportDto`. */
export type VerifyReportDto = {
  snapshotsChecked: number;
  filesChecked: number;
  chunksChecked: number;
  isClean: boolean;
  missing: number;
  corrupt: number;
  damage: { snapshotId: number; path: string; chunkHashHex: string; kind: string }[];
};

/** A repair (quarantine) result (Phase 49n). Mirrors `RepairReportDto`. */
export type RepairReportDto = {
  removedIds: number[];
  bytesReclaimed: number;
  applied: boolean;
};

export async function repositoryVerify(
  snapshotId: number | null,
  deep: boolean,
): Promise<VerifyReportDto> {
  return invoke<VerifyReportDto>("repository_verify", { snapshotId, deep });
}

export async function repositoryRepair(deep: boolean, apply: boolean): Promise<RepairReportDto> {
  return invoke<RepairReportDto>("repository_repair", { deep, apply });
}

/** One row of the unified snapshot timeline. Mirrors `RepoSnapshotDto`. */
export type RepositorySnapshotDto = {
  id: number;
  kind: string;
  createdAtMs: number;
  label: string;
  fileCount: number;
  totalSize: number;
  pinned: boolean;
  description: string;
};

/** One Phase 42 per-file version. Mirrors `version_commands::VersionRecordDto`. */
export type VersionRecordDto = {
  rowId: number;
  dstPath: string;
  tsMs: number;
  manifestBlake3Hex: string;
  size: number;
  retainedUntilMs: number | null;
  triggeredByJobId: number | null;
};

/** Repository dedup stats for the Library "hero" readout. */
export async function repositoryStats(): Promise<RepositoryStatsDto> {
  return invoke<RepositoryStatsDto>("repository_stats");
}

/** The unified snapshot timeline (oldest first). */
export async function repositorySnapshots(): Promise<RepositorySnapshotDto[]> {
  return invoke<RepositorySnapshotDto[]>("repository_snapshots");
}

/** One file's diff between two snapshots (Phase 49o). */
export type FileDiffDto = {
  path: string;
  change: "added" | "removed" | "modified" | "unchanged";
  oldSize: number | null;
  newSize: number | null;
  chunksShared: number;
  chunksChanged: number;
  bytesAdded: number;
};

/** Diff between two snapshots (Phase 49o). Mirrors `repository_commands::SnapshotDiffDto`. */
export type SnapshotDiffDto = {
  fromId: number;
  toId: number;
  files: FileDiffDto[];
  added: number;
  removed: number;
  modified: number;
  unchanged: number;
  bytesAdded: number;
};

/** Compare two snapshots: per-file changes + incremental chunk cost. */
export async function repositoryDiff(
  fromId: number,
  toId: number,
): Promise<SnapshotDiffDto> {
  return invoke<SnapshotDiffDto>("repository_diff", { fromId, toId });
}

/** Per-kind breakdown row (Phase 49r). */
export type KindBreakdownDto = { kind: string; count: number; effectiveBytes: number };
/** Storage-growth point (Phase 49r). */
export type GrowthPointDto = {
  tsMs: number;
  cumulativeUniqueBytes: number;
  snapshotCount: number;
};
/** Top-file row (Phase 49r). */
export type TopFileDto = { path: string; versions: number; maxSize: number };

/** Repository report (Phase 49r). Mirrors `repository_commands::RepoReportDto`. */
export type RepoReportDto = {
  stats: RepositoryStatsDto;
  byKind: KindBreakdownDto[];
  growth: GrowthPointDto[];
  topFiles: TopFileDto[];
  dedupRatio: number;
};

/** Compute the repository analytics report (top `topN` files). */
export async function repositoryReport(topN: number): Promise<RepoReportDto> {
  return invoke<RepoReportDto>("repository_report", { topN });
}

/** Export the report to `path` as `"md"` or `"json"`. */
export async function repositoryExportReport(
  path: string,
  topN: number,
  format: "md" | "json",
): Promise<void> {
  return invoke<void>("repository_export_report", { path, topN, format });
}

/** Pin/unpin a snapshot (pinned survives prune). Phase 49p. */
export async function repositorySetPinned(snapshotId: number, pinned: boolean): Promise<boolean> {
  return invoke<boolean>("repository_set_pinned", { snapshotId, pinned });
}

/** Rename a snapshot. Phase 49p. */
export async function repositorySetLabel(snapshotId: number, label: string): Promise<boolean> {
  return invoke<boolean>("repository_set_label", { snapshotId, label });
}

/** Set a snapshot's description. Phase 49p. */
export async function repositorySetDescription(
  snapshotId: number,
  description: string,
): Promise<boolean> {
  return invoke<boolean>("repository_set_description", { snapshotId, description });
}

/** Set a snapshot's tags. Phase 49p. */
export async function repositorySetTags(snapshotId: number, tags: string[]): Promise<boolean> {
  return invoke<boolean>("repository_set_tags", { snapshotId, tags });
}

/** Global keep-last / keep-within prune (pinned protected); returns removed ids. */
export async function repositoryPrunePolicy(
  keepLast: number | null,
  keepWithinMs: number | null,
  nowMs: number,
): Promise<number[]> {
  return invoke<number[]>("repository_prune_policy", { keepLast, keepWithinMs, nowMs });
}

/** Per-file rolling versions (Phase 42) for a destination path. */
export async function listVersions(
  dstPath: string,
): Promise<VersionRecordDto[]> {
  return invoke<VersionRecordDto[]>("list_versions", { dstPath });
}

/**
 * A backup source's retention policy (Phase 49e). Mirrors
 * `copythat_settings::RetentionSettings` — a serde tagged enum keyed on
 * `kind`.
 */
export type RetentionSettings =
  | { kind: "keep-all" }
  | { kind: "last-n"; n: number }
  | { kind: "older-than-days"; days: number }
  | { kind: "gfs"; hourly: number; daily: number; weekly: number; monthly: number };

/** One configured backup source. Mirrors `backup_commands::SourceConfigDto`. */
export type SourceConfigDto = {
  id: string;
  label: string;
  path: string;
  excludeGlobs: string[];
  lastRunAt: string;
  lastSnapshotId: number | null;
  retention: RetentionSettings;
  schedule: string;
  enabled: boolean;
};

/** Per-source schedule status (Phase 49f). Mirrors `backup_commands::BackupSourceStatusDto`. */
export type BackupSourceStatusDto = {
  id: string;
  label: string;
  enabled: boolean;
  schedule: string;
  lastRunMs: number;
  nextRunMs: number | null;
  due: boolean;
};

/** Outcome of a retention prune (Phase 49e). Mirrors `backup_commands::PruneReportDto`. */
export type PruneReportDto = {
  snapshotsRemoved: number;
  chunksSwept: number;
  bytesReclaimed: number;
};

/** List the configured backup sources (Phase 49c). */
export async function sourcesList(): Promise<SourceConfigDto[]> {
  return invoke<SourceConfigDto[]>("sources_list");
}

/** Add a backup source (Phase 49g: include/exclude globs + skip-hidden). */
export async function sourcesAdd(
  label: string,
  path: string,
  excludeGlobs: string[],
  includeGlobs: string[],
  skipHidden: boolean,
): Promise<SourceConfigDto> {
  return invoke<SourceConfigDto>("sources_add", {
    label,
    path,
    excludeGlobs,
    includeGlobs,
    skipHidden,
  });
}

/** Edit an existing backup source. */
export async function sourcesUpdate(
  id: string,
  label: string,
  path: string,
  excludeGlobs: string[],
): Promise<SourceConfigDto> {
  return invoke<SourceConfigDto>("sources_update", {
    id,
    label,
    path,
    excludeGlobs,
  });
}

/** Remove a backup source by id. */
export async function sourcesRemove(id: string): Promise<void> {
  return invoke<void>("sources_remove", { id });
}

/** Snapshot a source into the repository as a Backup; returns the new id. */
export async function backupNow(id: string): Promise<number> {
  return invoke<number>("backup_now", { id });
}

/** Set a source's retention policy (Phase 49e). */
export async function sourcesSetRetention(
  id: string,
  retention: RetentionSettings,
): Promise<SourceConfigDto> {
  return invoke<SourceConfigDto>("sources_set_retention", { id, retention });
}

/** Apply a source's retention policy now (forget + gc); returns the report. */
export async function repositoryPrune(id: string, nowMs: number): Promise<PruneReportDto> {
  return invoke<PruneReportDto>("repository_prune", { id, nowMs });
}

/** Preview which snapshot ids a source's retention policy would prune. */
export async function repositoryPrunePreview(id: string, nowMs: number): Promise<number[]> {
  return invoke<number[]>("repository_prune_preview", { id, nowMs });
}

/** Set a source's auto-run schedule + enabled flag (Phase 49f). */
export async function sourcesSetSchedule(
  id: string,
  schedule: string,
  enabled: boolean,
): Promise<SourceConfigDto> {
  return invoke<SourceConfigDto>("sources_set_schedule", { id, schedule, enabled });
}

/** Per-source schedule status (last/next run + due flag). */
export async function backupSourcesStatus(): Promise<BackupSourceStatusDto[]> {
  return invoke<BackupSourceStatusDto[]>("backup_sources_status");
}

/** Notification toggles (Phase 49q). Mirrors `notifications::NotificationSettingsDto`. */
export type NotificationSettingsDto = { onSuccess: boolean; onFailure: boolean };

export async function notificationsGet(): Promise<NotificationSettingsDto> {
  return invoke<NotificationSettingsDto>("notifications_get");
}

export async function notificationsSet(onSuccess: boolean, onFailure: boolean): Promise<void> {
  return invoke<void>("notifications_set", { onSuccess, onFailure });
}

/** Send a test notification to the configured webhooks; returns the count attempted. */
export async function notificationsTest(): Promise<number> {
  return invoke<number>("notifications_test");
}

/** One file in a snapshot's flat tree. Mirrors `SnapshotFileDto`. */
export type SnapshotFileDto = { path: string; size: number };
/** Tally of a restore run. Mirrors `RestoreReportDto`. */
export type RestoreReportDto = {
  restored: number;
  skipped: number;
  failed: number;
};
/** Per-file existence check for the restore conflict step. */
export type RestoreConflictDto = { path: string; exists: boolean };
/** Outcome of a repository GC pass. Mirrors `GcReportDto`. */
export type GcReportDto = {
  chunksSwept: number;
  packsRemoved: number;
  bytesReclaimed: number;
};

/** Flat file listing of a snapshot (Phase 49d restore browser). */
export async function snapshotTree(
  snapshotId: number,
): Promise<SnapshotFileDto[]> {
  return invoke<SnapshotFileDto[]>("snapshot_tree", { snapshotId });
}
/** Dry-run: which selected paths already exist under the destination. */
export async function restorePreview(
  snapshotId: number,
  paths: string[],
  dstRoot: string,
): Promise<RestoreConflictDto[]> {
  return invoke<RestoreConflictDto[]>("restore_preview", {
    snapshotId,
    paths,
    dstRoot,
  });
}
/** Restore selected files under `dstRoot` with the given conflict policy. */
export async function restorePaths(
  snapshotId: number,
  paths: string[],
  dstRoot: string,
  conflict: "overwrite" | "skip" | "keep-both",
): Promise<RestoreReportDto> {
  return invoke<RestoreReportDto>("restore_paths", {
    snapshotId,
    paths,
    dstRoot,
    conflict,
  });
}
/** Drop a snapshot from the catalog (run GC to reclaim its chunks). */
export async function repositoryForget(snapshotId: number): Promise<boolean> {
  return invoke<boolean>("repository_forget", { snapshotId });
}
/** Mark-and-sweep unreferenced chunks; reclaim space. */
export async function repositoryGc(): Promise<GcReportDto> {
  return invoke<GcReportDto>("repository_gc");
}
