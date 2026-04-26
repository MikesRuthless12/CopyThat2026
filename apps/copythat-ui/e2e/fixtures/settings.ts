/**
 * Default `SettingsDto` shape for tests that open the SettingsModal.
 *
 * The fixture's built-in `get_settings` handler returns a minimal
 * shape that misses required fields like `bufferSizeBytes`,
 * `verify`, `concurrency`, etc. — sufficient for boot but not for
 * the SettingsModal's bind:value usage.
 *
 * Tests that exercise Settings should `tauri.handles({
 * get_settings: () => fullSettings(...) })` so every dropdown has a
 * defined value to bind to.
 *
 * Mirrors the field set in `apps/copythat-ui/src/lib/types.ts`'s
 * `SettingsDto`. Optional fields (Phase 33+ scan/dnd/mount/audit/
 * crypt/mobile) are populated where the modal touches them.
 */

export function fullSettings(overrides: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    general: {
      language: "en",
      theme: "auto",
      startWithOs: false,
      singleInstance: true,
      minimizeToTray: false,
      errorDisplayMode: "modal",
      pasteShortcutEnabled: false,
      pasteShortcut: "CmdOrCtrl+Shift+V",
      clipboardWatcherEnabled: false,
      autoResumeInterrupted: false,
      mobileOnboardingDismissed: true,
    },
    transfer: {
      bufferSizeBytes: 1048576,
      verify: "off",
      concurrency: "auto",
      reflink: "prefer",
      fsyncOnClose: false,
      preserveTimestamps: true,
      preservePermissions: true,
      preserveAcls: false,
      onLocked: "ask",
      preserveSparseness: true,
      preserveSecurityMetadata: false,
      preserveMotw: true,
      preservePosixAcls: false,
      preserveSelinuxContexts: false,
      preserveResourceForks: true,
      appledoubleFallback: true,
      dedupMode: "auto-ladder",
      dedupHardlinkPolicy: "off",
      dedupPrescan: false,
    },
    shell: {
      contextMenuEnabled: false,
      interceptDefaultCopy: false,
      notifyOnCompletion: true,
    },
    secureDelete: { method: "dod-3-pass", confirmTwice: true },
    advanced: {
      logLevel: "info",
      telemetry: false,
      errorPolicy: { kind: "ask" },
      historyRetentionDays: 90,
      databasePath: null,
    },
    filters: {
      enabled: false,
      includeGlobs: [],
      excludeGlobs: [],
      minSizeBytes: null,
      maxSizeBytes: null,
      minMtimeUnixSecs: null,
      maxMtimeUnixSecs: null,
      skipHidden: false,
      skipSystem: false,
      skipReadonly: false,
    },
    updater: {
      autoCheck: false,
      channel: "stable",
      lastCheckUnixSecs: 0,
      dismissedVersion: "",
      checkIntervalSecs: 86400,
    },
    network: {
      mode: "off",
      fixedBytesPerSecond: 0,
      scheduleSpec: "",
      autoOnMetered: { mode: "off", fixedBytesPerSecond: 0 },
      autoOnBattery: { mode: "off", fixedBytesPerSecond: 0 },
      autoOnCellular: { mode: "off", fixedBytesPerSecond: 0 },
    },
    audit: {
      enabled: true,
      format: "json-lines",
      filePath: "/tmp/audit.log",
      maxSizeBytes: 10485760,
      worm: "off",
    },
    crypt: {
      encryptionMode: "off",
      recipientsFile: "",
      compressionMode: "off",
      compressionLevel: 3,
    },
    mobile: {
      pairEnabled: true,
      autoConnect: false,
      peerjsBroker: "",
      desktopPeerId: "test-peer",
      pairings: [],
      apnsP8Pem: "",
      apnsTeamId: "",
      apnsKeyId: "",
      fcmServiceAccountJson: "",
    },
    ...overrides,
  };
}
