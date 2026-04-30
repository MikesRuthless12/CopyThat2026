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
  /// Phase 45.3 — id of the queue this job belongs to. The legacy
  /// single-queue surface emits 0 (the default queue); registry-
  /// routed jobs carry the assigned id from `queue_route_job`.
  queueId: number;
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
  // Phase 22 — aggregate conflict dialog: auto-resolved by rule.
  collisionAutoResolved: "collision-auto-resolved",
  // Phase 13d
  fileActivity: "file-activity",
  // Post-Phase-12 paste hotkey + clipboard watcher
  clipboardFilesDetected: "clipboard-files-detected",
  // Phase 45 — named-queue registry events.
  queueAdded: "queue-added",
  queueRemoved: "queue-removed",
  queueMerged: "queue-merged",
  queueJobRouted: "queue-job-routed",
} as const;

/// Phase 45.3 — wire shape of `queue_list()`.
export interface QueueSnapshotDto {
  /// Stable u64 id (`QueueId::as_u64()`). 0 is reserved for the
  /// legacy default queue; registry queues start at 1.
  id: number;
  /// Tab label — drive-derived (`"D: queue"`) or `"queue N"` when
  /// no probe label is available.
  name: string;
  /// Pending + Running jobs in this queue.
  badgeCount: number;
  /// `true` when at least one job in the queue is `Running`.
  /// Phase 45.5 uses this to render the F2-mode pulse.
  running: boolean;
}

/// Phase 45.3 — `queue-added` event payload.
export interface QueueAddedEvent {
  id: number;
  name: string;
}

/// Phase 45.3 — `queue-removed` event payload.
export interface QueueIdEvent {
  id: number;
}

/// Phase 45.3 — `queue-merged` event payload.
export interface QueueMergedEvent {
  src: number;
  dst: number;
}

/// Phase 45.3 — `queue-job-routed` event payload.
export interface QueueJobRoutedEvent {
  queueId: number;
  jobId: number;
}

export type FileActivityPhase =
  | "start"
  | "progress"
  | "done"
  | "error"
  | "dir";

export interface FileActivityDto {
  jobId: number;
  seq: number;
  phase: FileActivityPhase;
  src: string;
  dst: string;
  bytesDone: number;
  bytesTotal: number;
  isDir: boolean;
  message: string | null;
}

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

// ---- Phase 22 — aggregate conflict dialog v2 ----

/// Wire-level rule resolutions — a superset of `CollisionAction`.
/// The runner translates `overwrite-if-newer` / `overwrite-if-larger`
/// / `keep-both` into one of the engine's four variants by peeking
/// source + destination metadata before calling the engine's
/// oneshot.
export type ConflictRuleResolution =
  | "skip"
  | "overwrite"
  | "overwrite-if-newer"
  | "overwrite-if-larger"
  | "keep-both";

export interface ConflictRuleDto {
  pattern: string;
  resolution: ConflictRuleResolution;
}

export interface ConflictProfileDto {
  rules: ConflictRuleDto[];
  fallback: ConflictRuleResolution | null;
}

/// `collision-auto-resolved` payload — the runner silently
/// resolved a collision against the job's rule set and the UI
/// should render a "✓ via rule '*.docx → newer'" row in the
/// aggregate dialog without having surfaced a prompt.
export interface CollisionAutoResolvedDto {
  jobId: number;
  src: string;
  dst: string;
  /// `"skip"` / `"overwrite"` / `"rename"` — what the engine did.
  resolution: CollisionAction;
  /// `"skip"` / `"overwrite"` / `"overwrite-if-newer"` /
  /// `"overwrite-if-larger"` / `"keep-both"` — the user's intent.
  ruleResolution: ConflictRuleResolution;
  matchedRulePattern: string;
}

/// `thumbnail_for` return shape — image data URL OR a file-kind
/// icon descriptor. Frontend renders `<img>` for `kind="image"`
/// and hands to the existing FileKindIcon component otherwise.
export interface ThumbnailDto {
  kind: "image" | "icon";
  dataUrl: string | null;
  iconKind: string | null;
  extension: string | null;
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

// Phase 12 — Settings DTOs. Mirror the Rust `SettingsDto` at
// apps/copythat-ui/src-tauri/src/ipc.rs. Enum-shaped fields are
// kebab-case lowercase strings on the wire; the UI formats them.

export type ErrorDisplayModeWire = "modal" | "drawer";

export interface GeneralSettingsDto {
  language: string;
  theme: "auto" | "light" | "dark";
  startWithOs: boolean;
  singleInstance: boolean;
  minimizeToTray: boolean;
  errorDisplayMode: ErrorDisplayModeWire;
  pasteShortcutEnabled: boolean;
  /** Tauri global-shortcut combo (e.g. "CmdOrCtrl+Shift+V"). */
  pasteShortcut: string;
  clipboardWatcherEnabled: boolean;
  /** Phase 20 — silent re-enqueue of unfinished jobs at startup. */
  autoResumeInterrupted: boolean;
  mobileOnboardingDismissed?: boolean;
}

/** Phase-post-12 — fired when the clipboard watcher sees new files land
 *  on the OS clipboard. The UI surfaces a toast hint; the user can then
 *  press the configured paste shortcut to open the staging dialog. */
export interface ClipboardFilesDetectedDto {
  paths: string[];
  count: number;
  shortcut: string;
}

export type VerifyChoiceWire =
  | "off"
  | "crc32"
  | "md5"
  | "sha1"
  | "sha256"
  | "sha512"
  | "xxhash3-64"
  | "xxhash3-128"
  | "blake3";

export type ReflinkWire = "prefer" | "avoid" | "disabled";

/**
 * Phase 19b — wire value for the "When a file is locked" setting.
 * Mirrors `copythat_core::LockedFilePolicy`.
 */
export type LockedFilePolicyWire = "ask" | "retry" | "skip" | "snapshot";

/**
 * Phase 20 — one row of the resume modal. Mirrors
 * `crate::ipc::PendingResumeDto`.
 */
export interface PendingResumeDto {
  rowId: number;
  /** "copy" | "move" | "delete" | "secure-delete" | "verify" */
  kind: string;
  srcRoot: string;
  dstRoot: string | null;
  /** "running" | "paused" — terminal statuses are filtered out */
  status: string;
  startedAtMs: number;
  bytesDone: number;
  bytesTotal: number;
  filesDone: number;
  filesTotal: number;
  lastCheckpointAtMs: number;
}

export interface TransferSettingsDto {
  bufferSizeBytes: number;
  verify: VerifyChoiceWire;
  /// "auto" or "manual-N" for 1..=16
  concurrency: string;
  reflink: ReflinkWire;
  fsyncOnClose: boolean;
  preserveTimestamps: boolean;
  preservePermissions: boolean;
  preserveAcls: boolean;
  /// Phase 19b — filesystem-snapshot fallback for locked sources.
  onLocked: LockedFilePolicyWire;
  /// Phase 23 — preserve source sparseness on the destination.
  preserveSparseness: boolean;
  /// Phase 24 — master toggle for security-metadata preservation.
  preserveSecurityMetadata: boolean;
  /// Phase 24 — preserve Mark-of-the-Web (Zone.Identifier ADS).
  preserveMotw: boolean;
  /// Phase 24 — preserve POSIX ACLs and the broader xattr surface.
  preservePosixAcls: boolean;
  /// Phase 24 — preserve `security.selinux` MAC labels.
  preserveSelinuxContexts: boolean;
  /// Phase 24 — preserve macOS resource forks + Finder info.
  preserveResourceForks: boolean;
  /** Phase 38 — destination dedup ladder mode. */
  dedupMode?: string;
  dedupHardlinkPolicy?: string;
  dedupPrescan?: boolean;
  /// Phase 24 — fall back to AppleDouble sidecar on incompatible
  /// destination filesystems.
  appledoubleFallback: boolean;
}

export interface ShellSettingsDto {
  contextMenuEnabled: boolean;
  interceptDefaultCopy: boolean;
  notifyOnCompletion: boolean;
}

export type ShredMethodWire =
  | "zero"
  | "random"
  | "dod-3-pass"
  | "dod-7-pass"
  | "gutmann"
  | "nist-800-88";

export interface SecureDeleteSettingsDto {
  method: ShredMethodWire;
  confirmTwice: boolean;
}

export type LogLevelWire =
  | "off"
  | "trace"
  | "debug"
  | "info"
  | "warn"
  | "error";

export type ErrorPolicyDtoV2 =
  | { kind: "ask" }
  | { kind: "skip" }
  | { kind: "abort" }
  | { kind: "retryN"; maxAttempts: number; backoffMs: number };

export interface AdvancedSettingsDto {
  logLevel: LogLevelWire;
  telemetry: boolean;
  errorPolicy: ErrorPolicyDtoV2;
  historyRetentionDays: number;
  databasePath: string | null;
}

/** Phase 14a — enumeration-time filters. Sizes are byte counts;
 *  dates are signed Unix epoch seconds so pre-1970 mtimes don't wrap.
 *  `null` on a bound means "unbounded on that end". */
export interface FiltersDto {
  enabled: boolean;
  includeGlobs: string[];
  excludeGlobs: string[];
  minSizeBytes: number | null;
  maxSizeBytes: number | null;
  minMtimeUnixSecs: number | null;
  maxMtimeUnixSecs: number | null;
  skipHidden: boolean;
  skipSystem: boolean;
  skipReadonly: boolean;
}

/** Phase 15 — auto-update channel + throttle state. */
export type UpdateChannelWire = "stable" | "beta";

export interface UpdaterSettingsDto {
  autoCheck: boolean;
  channel: UpdateChannelWire;
  lastCheckUnixSecs: number;
  dismissedVersion: string;
  checkIntervalSecs: number;
}

export interface SettingsDto {
  general: GeneralSettingsDto;
  transfer: TransferSettingsDto;
  shell: ShellSettingsDto;
  secureDelete: SecureDeleteSettingsDto;
  advanced: AdvancedSettingsDto;
  filters: FiltersDto;
  updater: UpdaterSettingsDto;
  /// Phase 19a — disk-backed scan database. The settings field is
  /// always present in the wire payload; the modal exposes the knobs
  /// inside the Advanced tab.
  scan?: ScanSettingsDto;
  /// Phase 21 — bandwidth shaping (global cap + schedule + auto-throttle).
  network: NetworkSettingsDto;
  /// Phase 29 — drag-and-drop polish (spring-load, drag thumbnails,
  /// invalid-target highlight). Optional on the wire so older
  /// frontends opening a newer settings.toml don't explode.
  dnd?: DndSettingsDto;
  /// Phase 33 — mount-as-filesystem "mount latest snapshot on launch"
  /// toggle. Optional on the wire so older backends without the
  /// field don't break this frontend.
  mount?: MountSettingsDto;
  /// Phase 34 — audit log export + WORM mode. Optional on the wire
  /// so older backends without the field don't break this frontend.
  audit?: AuditSettingsDto;
  /// Phase 35 — encryption + compression policy. Optional on the
  /// wire so older backends without the field don't break this
  /// frontend.
  crypt?: CryptSettingsDto;
  /// Phase 37 — mobile companion (pairing + push). Optional on the
  /// wire so older backends without the field don't break this
  /// frontend.
  mobile?: MobileSettingsDto;
}

/** Phase 37 — wire form of `copythat_settings::MobileSettings`. */
export interface MobileSettingsDto {
  pairEnabled: boolean;
  autoConnect: boolean;
  peerjsBroker: string;
  desktopPeerId: string;
  pairings: MobilePairingEntryDto[];
  apnsP8Pem: string;
  apnsTeamId: string;
  apnsKeyId: string;
  fcmServiceAccountJson: string;
}

export interface MobilePairingEntryDto {
  label: string;
  phonePublicKeyHex: string;
  pairedAt: number;
  pushTarget?: MobilePushTargetDto;
}

export type MobilePushTargetDto =
  | { kind: "apns"; token: string }
  | { kind: "fcm"; token: string }
  | { kind: "stub_endpoint"; url: string };

/** Phase 35 — wire form of `copythat_settings::CryptSettings`. */
export type EncryptionModeWire = "off" | "passphrase" | "recipients";
export type CompressionModeWire = "off" | "always" | "smart";

export interface CryptSettingsDto {
  encryptionMode: EncryptionModeWire;
  recipientsFile: string;
  compressionMode: CompressionModeWire;
  /** zstd level 1–22. Slider in Settings clamps into that range. */
  compressionLevel: number;
  /** Extras appended to the Smart policy's built-in deny list. */
  compressionExtraDeny: string[];
}

/** Phase 33 — wire form of `copythat_settings::MountSettings`. */
export interface MountSettingsDto {
  mountOnLaunch: boolean;
  mountOnLaunchPath: string;
}

/** Phase 34 — wire form of `copythat_settings::AuditSettings`. */
export type AuditFormatWire =
  | "csv"
  | "json-lines"
  | "syslog"
  | "cef"
  | "leef";

export interface AuditSettingsDto {
  enabled: boolean;
  format: AuditFormatWire;
  filePath: string;
  /** `"off" | "on"` — persist as string so forward-compat works. */
  worm: string;
  maxSizeBytes: number;
  syslogDestination: string;
}

/** Phase 19a — disk-backed scan database settings. Optional in TS
 *  because the Svelte UI mostly ignores this struct (the Advanced
 *  tab edits a few fields directly through `pushSettings`). */
export interface ScanSettingsDto {
  hashDuringScan: boolean;
  databasePath: string | null;
  autoDeleteAfterDays: number;
  maxScansToKeep: number;
}

/** Phase 21 — wire form of `copythat_settings::NetworkSettings`. */
export type BandwidthModeWire = "off" | "fixed" | "schedule";

/** Phase 21 — `unchanged` keeps the global mode authoritative;
 *  `pause` blocks the copy entirely; `cap` overrides at N bytes/s. */
export type AutoThrottleRuleDto =
  | { kind: "unchanged" }
  | { kind: "pause" }
  | { kind: "cap"; value: number };

export interface NetworkSettingsDto {
  mode: BandwidthModeWire;
  fixedBytesPerSecond: number;
  scheduleSpec: string;
  autoOnMetered: AutoThrottleRuleDto;
  autoOnBattery: AutoThrottleRuleDto;
  autoOnCellular: AutoThrottleRuleDto;
}

/** Phase 21 — payload of `shape-rate-changed`. `bytesPerSecond` is
 *  null when the shape is unlimited, 0 when paused, otherwise the
 *  cap. `source` drives the badge's secondary label. */
export interface ShapeRateDto {
  bytesPerSecond: number | null;
  source:
    | "settings"
    | "schedule"
    | "auto-metered"
    | "auto-battery"
    | "auto-cellular"
    | "off";
}

/** Phase 15 — return shape of `updater_check_now`. */
export interface UpdateCheckDto {
  availableVersion: string;
  notes: string;
  pubDate: string;
  isNewer: boolean;
  checkedAtUnixSecs: number;
  skippedByThrottle: boolean;
}

export interface ProfileInfoDto {
  name: string;
  path: string;
}

// ---------------------------------------------------------------------
// Phase 25 — two-way sync DTOs (wire payloads from the Tauri runtime).
// ---------------------------------------------------------------------

export type SyncModeWire =
  | "two-way"
  | "mirror-left-to-right"
  | "mirror-right-to-left"
  | "contribute-left-to-right";

export type SyncSideWire = "left-to-right" | "right-to-left";

export type SyncConflictKindWire =
  | "concurrent-write"
  | "delete-edit"
  | "add-add"
  | "corrupt-equal";

export interface SyncPairDto {
  id: string;
  label: string;
  left: string;
  right: string;
  mode: SyncModeWire;
  lastRunAt: string;
  lastRunSummary: string;
  running: boolean;
  /** Phase 26 — true when the pair is in live-mirror mode (a
   *  watcher on the left side triggers a re-sync on each debounced
   *  change). The UI renders a green pulsing dot + "Watching"
   *  label when this flag is set. */
  liveMirror: boolean;
}

export interface SyncStartedDto {
  pairId: string;
  label: string;
  left: string;
  right: string;
  mode: SyncModeWire;
}

export interface SyncConflictDto {
  pairId: string;
  relpath: string;
  kind: SyncConflictKindWire;
  winnerSide: SyncSideWire;
  loserSide: SyncSideWire;
  loserPreservationPath: string;
}

export interface SyncCompletedDto {
  pairId: string;
  appliedLeft: number;
  appliedRight: number;
  deletedLeft: number;
  deletedRight: number;
  conflicts: number;
  cancelled: boolean;
  durationMs: number;
}

export interface SyncFailedDto {
  pairId: string;
  message: string;
}

// ---------------------------------------------------------------------
// Phase 28 — tray-resident Drop Stack
// ---------------------------------------------------------------------

export interface DropStackEntryDto {
  path: string;
  addedAtMs: number;
}

export interface DropStackPathMissingDto {
  path: string;
}

// ---------------------------------------------------------------------
// Phase 29 — destination picker + drag-out staging DTOs.
// ---------------------------------------------------------------------

export interface DirChildDto {
  name: string;
  path: string;
  writable: boolean;
}

export interface DragOutStagedDto {
  paths: string[];
  count: number;
}

/// Wire form of `copythat_settings::DndSettings`. Mirrored in
/// `apps/copythat-ui/src-tauri/src/ipc.rs`. Governs the Phase 29
/// spring-load + drag-thumbnail UX.
export interface DndSettingsDto {
  springLoadEnabled: boolean;
  springLoadDelayMs: number;
  showDragThumbnails: boolean;
  highlightInvalidTargets: boolean;
}
