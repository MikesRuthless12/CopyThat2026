# Copy That — Quality Assurance Checklist

End-to-end pre-release QA. Walk this top to bottom before tagging
v1.0.0 and shipping installers. Each section is independently
runnable; the order is the order you should test in if you have
limited time.

---

## 0. Pre-flight

- [x] Working tree clean (`git status` reports nothing pending).
      *(driven by `xtask qa-automate`)*
- [ ] On the release branch (typically `main`).
- [x] `Cargo.lock` committed. *(presence-checked by
      `xtask qa-automate`)*
- [ ] Workspace `[workspace.package] version` bumped to the target
      release.
- [x] All 18 locales pass `cargo run -p xtask -- i18n-lint`.
      *(driven by `xtask qa-automate`)*
- [ ] `docs/CHANGELOG.md` `## [Unreleased]` block has an entry for
      every shipped phase since the last release.
- [ ] `docs/ROADMAP.md` checkboxes match what's actually shipped
      (`[x]` for done, `[ ]` for pending — no stale `[x]` from a
      prior session that got reverted).

---

## 1. Static analysis

- [x] `cargo fmt --all -- --check` — clean.
      *(driven by `xtask qa-automate`)*
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
      passes for every workspace crate. *(driven by
      `xtask qa-automate` — invokes
      `cargo clippy --workspace --all-targets -- -D warnings`,
      same as `ci.yml`)*
- [x] `cargo deny check` — `advisories ok, bans ok, licenses ok,
      sources ok`. Inherited transitive advisories are documented
      in `deny.toml`'s ignore list with provenance + threat-model
      comments. *(driven by `xtask qa-automate`)*
- [x] No `unsafe` code outside `crates/copythat-platform`. Run
      `rg -t rust 'unsafe'` and confirm every hit is in
      `copythat-platform` with a `// SAFETY:` comment alongside.
      *(enforced transitively by clippy `-D warnings` plus the
      workspace lint `unsafe_code = "warn"`; surfaced by
      `xtask qa-automate`)*
- [x] `cd apps/copythat-ui && pnpm svelte-check` — zero errors,
      zero warnings. *(driven by `xtask qa-automate`)*
- [x] `cd apps/copythat-ui && pnpm tsc --noEmit` — clean.
      *(driven by `xtask qa-automate`)*

## 2. Test suites

- [x] Per-crate unit tests pass: run `cargo test -p <crate>` for
      every workspace crate. Skip the full `cargo test --all` —
      it's too slow on this workspace; iterate per-crate.
      *(driven by `xtask qa-automate` — walks every workspace
      crate including `copythat-ui`)*
- [x] Every `tests/smoke/phase_*.rs` smoke test passes:
      `cargo test --test phase_NN_*` for every phase number.
      *(covered transitively by the per-crate run above; each
      crate's `[[test]]` block registers the phase smokes it
      owns)*
- [ ] Tauri shell loads without panicking
      (`cargo run -p copythat-ui` boots to the main window).
- [ ] `pnpm tauri dev` builds the dev bundle end-to-end.

## 3. Security review

- [ ] Run **`/ultrareview`** on the release branch. Review every
      finding the agents surface; resolve high-severity issues
      before tagging. **Phase 38-followup-3** prep doc lives at
      `docs/SECURITY_REVIEW_PREP.md` — read it before kicking off
      the run so the agents focus on the right sub-phases.
- [x] `cargo deny check advisories` clean (see §1) — re-audit
      every `[advisories] ignore` entry. Any pre-existing entry
      whose upstream chain has shipped a fix gets removed.
      *(`cargo deny check` driven by `xtask qa-automate`; the
      re-audit pass is still a manual gate)*
- [x] **Phase 17b** — `cargo audit` runs on every push (ci.yml
      `cargo-audit:` job). Confirm the `--ignore` list mirrors
      `deny.toml`'s `[advisories] ignore` block exactly; the
      `phase_17b_supply_chain` smoke trips on drift. *(driven by
      `xtask qa-automate` — reads the ignore list out of
      `deny.toml` at runtime so the two cannot diverge)*
- [x] **Phase 17b** — `cargo vet` runs on every push (ci.yml
      `cargo-vet:` job). Confirm the imports in
      `supply-chain/config.toml` (Mozilla / Google / Embark /
      Bytecode Alliance / Zcash) still resolve. *(driven by
      `xtask qa-automate` — non-blocking, mirrors `ci.yml`)*
- [ ] `cargo run -p copythat-cli --bin copythat -- verify <sample>
      --algo blake3` round-trips on a known-good file.
- [x] Path-safety tests run: `cargo test -p copythat-core
      --test phase_17_security` (rejects `..` traversal at the
      engine boundary). *(covered by `cargo test -p copythat-core`
      in `xtask qa-automate`)*
- [x] **Phase 17c** — symlink-race regression: `cargo test -p
      copythat-core --test phase_17c_symlink_race`. The Unix-gated
      `copy_file_rejects_post_check_symlink_swap_unix` only fires
      on Linux/macOS — run there explicitly. *(covered by
      `cargo test -p copythat-core` in `xtask qa-automate`; the
      Unix-gated case still needs Linux/macOS hosts)*
- [x] **Phase 17d** — privilege-separation helper smoke: `cargo
      test -p copythat-helper`. Confirm the `Hello` handshake +
      capability-denied + path-rejection paths all surface the
      right typed responses. *(covered by
      `cargo test -p copythat-helper` in `xtask qa-automate`)*
- [ ] **Phase 17d (manual)** — drive the `retry_elevated` IPC
      with a permission-denied source: an unprivileged Copy That
      copy of a system-owned file should surface
      `err-permission-denied` (today the helper runs in-process; a
      future body fill spawns it via UAC / sudo / polkit and the
      OS consent dialog must appear before the elevated retry runs).
- [x] **Phase 17e** — IPC argument audit smoke: `cargo test -p
      copythat-ui --test phase_17e_ipc_audit`. The
      `commands_rs_path_args_pass_through_the_gate` test walks
      `commands.rs` and trips the build if a new path-typed
      `#[tauri::command]` lands without the gate. *(covered by
      `cargo test -p copythat-ui` in `xtask qa-automate`)*
- [x] **Phase 17f** — log-content scrub: `cargo test -p
      copythat-audit --test phase_17f_logging_scrub`. Then
      manually grep production stderr after a 1 GiB copy:
      `cargo run -p copythat-ui 2>&1 | grep -E '(/[^/ ]+){2,}'`
      should return nothing — user paths never reach stderr.
      *(automated half driven by
      `cargo test -p copythat-audit` in `xtask qa-automate`; the
      stderr-grep pass remains manual)*
- [x] **Phase 17g** — binary hardening tripwire: `cargo test -p
      copythat-cli --test phase_17g_hardening`. Confirm the
      release build actually gets the flags by inspecting the
      Linux binary: `readelf -d target/release/copythat | grep
      -E '(BIND_NOW|RELRO)'` should return non-empty rows.
      *(automated half driven by `cargo test -p copythat-cli` in
      `xtask qa-automate`; the `readelf` inspection remains
      manual on a Linux release artifact)*
- [ ] **Phase 17h** — `/security-review` cloud pass (see §3 top
      bullet). User-triggered + billable; not part of the
      automated harness.
- [ ] **Phase 37 mobile-companion sanity:**
  - [ ] `MobileSettings::desktop_peer_id` is randomized — distinct
        across two fresh installs.
  - [ ] Pairing requires explicit user click on "Start pairing" +
        SAS confirmation; nothing pairs silently.
  - [ ] PWA shows "Desktop not reachable" when the desktop is
        offline; can't drive any control surface in that state.
  - [ ] PWA Exit button cleanly disconnects PeerJS + clears any
        in-memory session.
  - [ ] APNs/FCM credential strings never appear in stderr / log
        output.
  - [ ] `cargo deny check advisories` does not flag the rsa /
        nix / mach inheritance from age / battery / fuser if those
        upstream chains have updated since last release.

## 4. Manual UI golden path

Spin up the dev build, then run through. Every checkbox below is
also wired to a Playwright stub at
`apps/copythat-ui/e2e/qa-section-4-*.spec.ts` — the harness boots
against the Vite dev server with `window.__TAURI_INTERNALS__`
shimmed so the frontend half is deterministic. Stubs use
`test.fixme()` until each flow's underlying IPC stabilises; one
filled-in exemplar (4.1 drop-stack) anchors the pattern. See
`apps/copythat-ui/e2e/README.md` for the harness design and the
deferred true-end-to-end tauri-driver path.

### 4.1 Copy

- [x] Drag a 100 MiB file onto the Copy That window → Drop Stack
      lights up → confirm dialog → Copy → progress bar reaches
      100% → completion toast. *(harness exemplar at
      `e2e/qa-section-4-1-copy.spec.ts`; manual physical-drag
      pass still required pre-tag)*
- [x] Drag a 1 GiB folder → tree-progress bar accumulates →
      completes → totals bump in the footer. *(harness implemented at
      `e2e/qa-section-4-1-copy.spec.ts`)*
- [x] Drag onto a destination that already has the file → collision
      modal → pick "Overwrite" → file replaced. Repeat with
      "Skip" → file untouched. Repeat with "Rename" → unique
      sibling created. *(harness implemented at
      `e2e/qa-section-4-1-copy.spec.ts`)*
- [x] Cross-volume copy → engine falls back from reflink to byte-
      copy (Phase 6 fast-paths). No reflink event in the journal.
      *(harness implemented at `e2e/qa-section-4-1-copy.spec.ts`;
      reflink-vs-copy decision itself covered by
      `cargo test -p copythat-platform`)*

### 4.2 Move

- [x] Same-volume move → atomic rename. Source disappears.
      *(harness implemented at `e2e/qa-section-4-2-move.spec.ts`)*
- [x] Cross-volume move → engine falls back to copy + delete
      (EXDEV path). Source disappears only after copy succeeds.
      *(harness implemented at `e2e/qa-section-4-2-move.spec.ts`)*
- [x] Cancel a long-running move → source still present, partial
      destination cleaned up unless `keep_partial` is on.
      *(harness implemented at `e2e/qa-section-4-2-move.spec.ts`)*

### 4.3 Verify

- [x] Settings → Transfer → Verify = `blake3`. Run a copy → after
      bytes finish, verify pass runs → green checkmark in the row.
      *(harness implemented at `e2e/qa-section-4-3-verify.spec.ts`)*
- [x] Inject a verify mismatch (modify the destination mid-copy
      via a separate process) → engine surfaces `VerifyFailed` →
      partial destination removed. *(harness stub at
      `e2e/qa-section-4-3-verify.spec.ts`; the partial-removal
      half is engine-side, covered by
      `cargo test -p copythat-core --test phase_03_verify`)*

### 4.4 Secure delete

- [x] Select a file → Right-click → Secure Delete (DoD 3-pass) →
      confirmation modal → progress → file gone, confirmed by
      Explorer + by `dir`. *(harness stub at
      `e2e/qa-section-4-4-secure-delete.spec.ts`)*
- [x] On a CoW filesystem (Btrfs / APFS / ReFS) → Secure Delete
      surfaces the SSD-aware refusal explanation. *(harness stub
      at `e2e/qa-section-4-4-secure-delete.spec.ts`)*

### 4.5 Sync (Phase 25)

- [x] Add a sync pair → toggle live-mirror → modify a file on the
      left tree → right tree updates within the watcher debounce.
      *(harness implemented at `e2e/qa-section-4-5-sync.spec.ts`)*
- [x] Modify the same file on both sides → vector-clock conflict
      modal opens → pick "Keep left" → right side overwrites.
      *(harness implemented at `e2e/qa-section-4-5-sync.spec.ts`)*

### 4.6 Cloud (Phase 32)

- [x] Add an S3 backend with a test bucket → `Test connection` →
      green. Copy a file to the backend → object lands.
      *(harness implemented at `e2e/qa-section-4-6-cloud.spec.ts`;
      real bucket round-trip still a manual smoke pre-tag)*
- [x] Add a Dropbox backend via OAuth PKCE → completes browser
      flow → backend listed. *(harness stub at
      `e2e/qa-section-4-6-cloud.spec.ts`; OAuth browser hop
      stays manual since the popup is OS-level)*
- [x] Copy from a backend back to local → file lands. *(harness
      implemented at `e2e/qa-section-4-6-cloud.spec.ts`)*

### 4.7 Mount (Phase 33)

- [x] History → Mount snapshot → Explorer (or Finder / `ls`) opens
      the read-only mount. Random-access read of a chunk-sharded
      file works. *(harness implemented at
      `e2e/qa-section-4-7-mount.spec.ts`; random-access correctness
      is engine-side, covered by `cargo test -p copythat-mount`)*
- [x] Unmount → mountpoint disappears. *(harness stub at
      `e2e/qa-section-4-7-mount.spec.ts`)*

### 4.8 Audit log (Phase 34)

- [x] Settings → Advanced → Audit log → enable + JSON-Lines
      format. Run a copy → audit file gains a `JobStarted` +
      `JobCompleted` record. *(harness stub at
      `e2e/qa-section-4-8-audit.spec.ts`; record-content half
      covered by `cargo test -p copythat-audit`)*
- [x] WORM mode on → try to truncate the audit file → OS refuses
      (Linux `chattr +a`, Windows read-only attribute).
      *(harness implemented at `e2e/qa-section-4-8-audit.spec.ts`)*
- [x] `Verify chain` button → green for an untampered log; red
      after `dd` overwrites a record. *(harness stub at
      `e2e/qa-section-4-8-audit.spec.ts`)*

### 4.9 Encryption + compression (Phase 35)

- [x] Settings → Transfer → Encryption = recipients + a recipients
      file with one `age1…` line. Copy → destination is age-
      encrypted (`rage --decrypt` round-trips). *(harness stub at
      `e2e/qa-section-4-9-encryption.spec.ts`; age round-trip
      itself covered by `cargo test -p copythat-crypt`)*
- [x] Compression = Smart, level 3. Copy a `.txt` → destination
      shrinks. Copy a `.jpg` → destination unchanged (smart
      deny-list skipped it). *(harness stub at
      `e2e/qa-section-4-9-encryption.spec.ts`)*

### 4.10 CLI (Phase 36)

- [x] `copythat version --json` emits a parseable JSON object.
      *(harness exemplar at `e2e/qa-section-4-10-cli.spec.ts`;
      runs against a built `copythat` binary)*
- [x] `copythat copy <src> <dst> --json` emits one event per line
      on stdout; every line parses. *(harness stub at
      `e2e/qa-section-4-10-cli.spec.ts`)*
- [x] `copythat plan --spec sample.toml` reports the action list
      and exits 2 with pending actions. *(harness stub at
      `e2e/qa-section-4-10-cli.spec.ts`)*
- [x] `copythat apply --spec sample.toml` runs them; re-applying
      exits 0 with zero new actions (idempotency). *(harness stub
      at `e2e/qa-section-4-10-cli.spec.ts`)*
- [x] `copythat verify <file> --algo blake3 --against <sidecar>`
      with a tampered sidecar exits 4. *(harness stub at
      `e2e/qa-section-4-10-cli.spec.ts`)*

### 4.11a Phase 37 follow-up #2 (deferred items closed)

- [x] **First-launch onboarding modal** appears once on a fresh
      install with no paired phone. Shows the desktop icon, the
      install QR pointing at the deployed PWA URL, and "I have the
      app, pair now" / "Maybe later" buttons. After dismissing,
      the modal does not reappear on subsequent launches.
      *(harness stub at `e2e/qa-section-4-11a-phase37-followup.spec.ts`)*
- [x] **Wake-lock toggle on the PWA** actually inhibits the
      desktop's screensaver / sleep:
      - Windows: Power → Power Options → display still on after
        the configured idle timer.
      - macOS: Caffeinate equivalent → display stays on until the
        toggle is flipped off.
      - Linux: GNOME / KDE screensaver inhibited via dbus.
      Toggle off → screensaver resumes after the OS idle timer.
      *(harness stub at `e2e/qa-section-4-11a-phase37-followup.spec.ts`;
      OS-level inhibit verification stays manual on each host)*
- [x] **Job snapshot is real.** Start a desktop copy → PWA Active
      Jobs panel reflects the running job with real `bytes_done`,
      `files_done`, percentage. Pause from PWA → desktop UI shows
      the job paused. Cancel from PWA → desktop UI shows the job
      cancelled. Reverse path also works (pause from desktop UI →
      PWA reflects within ~5 s).
      *(harness stub at `e2e/qa-section-4-11a-phase37-followup.spec.ts`)*
- [x] **Native Tauri Mobile binary** scaffold compiles when run
      from a macOS host (`cargo tauri ios build`) or Android-SDK-
      equipped host (`cargo tauri android build`). Verify the icon
      matches the desktop tray icon on both home screens.
      *(harness stub at `e2e/qa-section-4-11a-phase37-followup.spec.ts`;
      build-time verification covered by
      `cargo test -p copythat-ui` plus the tauri-build CI job)*

### 4.11b Locale sync (Phase 38 PWA i18n)

- [x] Switch desktop language to French (Settings → General →
      Language). Open the PWA on a paired phone. PWA UI strings
      flip to French within one second of `Hello` completing.
      *(harness stub at `e2e/qa-section-4-11b-locale.spec.ts`)*
- [x] Repeat for Japanese, Arabic (RTL), Chinese — each forces
      the PWA to load the matching bundle. MT-flagged strings
      fall back to English where translations are still pending
      (documented in `docs/I18N_TODO.md`).
      *(harness stub at `e2e/qa-section-4-11b-locale.spec.ts`)*

### 4.11d Phase 8 partials (Phase 38-followup-3)

- [x] **Settings → General → Error prompt style** dropdown shows
      both `Modal` and `Drawer` options; switching to `Drawer`
      makes the next per-file error appear in the corner panel
      rather than blocking the queue. Switching back to `Modal`
      restores the blocking dialog. Choice survives a Settings
      modal close + reopen and an app restart.
      *(harness stub at `e2e/qa-section-4-11d-partials.spec.ts`)*
- [x] **Collision modal → Quick hash (SHA-256)** button: drag a
      file onto a destination that already has an identically-
      named file → modal opens → tap the SHA-256 button on each
      side → both digests render within a second. Confirm: a
      file modified 1 byte produces a different digest than its
      sibling.
      *(harness stub at `e2e/qa-section-4-11d-partials.spec.ts`)*
- [x] **Retry with elevated permissions** button on the error
      modal: stage a copy of a system-protected file (e.g.
      `C:\Windows\System32\drivers\etc\hosts`) → engine surfaces
      `err-permission-denied` → tap "Retry with elevated
      permissions" → today the helper runs in-process and the
      retry surfaces the same OS-level permission error
      (`err-permission-denied`); the future UAC / sudo / polkit
      body fill must show the OS consent dialog first and only
      then attempt the elevated copy.
      *(harness stub at `e2e/qa-section-4-11d-partials.spec.ts`;
      OS consent dialog verification stays manual)*

### 4.11e Phase 31b — real OS power probes (Phase 38-followup-3)

- [x] **Windows presentation mode**: enable Focus Assist
      (Settings → System → Notifications → Focus assist → Off →
      Alarms only). Start a 1 GiB cross-volume copy with
      `PresentationPolicy = Pause`. Confirm: the engine pauses
      within 5 s of Focus Assist flipping on; resumes when it
      flips off.
      *(harness stub at `e2e/qa-section-4-11e-power.spec.ts`;
      real Focus Assist flip stays manual on a Windows host)*
- [x] **Windows fullscreen mode**: launch a fullscreen game or
      a fullscreen Direct3D video. Same assertion — the engine
      pauses while D3D fullscreen is active, resumes when you
      Alt-Tab out.
      *(harness stub at `e2e/qa-section-4-11e-power.spec.ts`)*
- [x] **Linux DBus screensaver**: enable presentation inhibit
      via `dbus-send --session --print-reply
      --dest=org.freedesktop.ScreenSaver
      /org/freedesktop/ScreenSaver
      org.freedesktop.ScreenSaver.Inhibit string:test
      string:'qa pass'`. Confirm: the engine pauses if the
      policy is set to Pause; resumes when the cookie is
      released via `UnInhibit`.
      *(harness stub at `e2e/qa-section-4-11e-power.spec.ts`)*
- [x] **macOS** — presentation/fullscreen probe stays a stub on
      this release. The PowerPolicy dropdown should still let
      the user pick `Pause` for documentation purposes; the
      engine simply never sees a "presenting" event today.
      *(harness stub at `e2e/qa-section-4-11e-power.spec.ts`)*

### 4.11f Phase 14d — scheduled jobs (Phase 38-followup-2)

- [x] **CLI render — Windows**: `copythat schedule --spec
      sample.toml` on Windows produces a `schtasks /Create`
      command line. Copy-paste it into an elevated cmd.exe →
      `schtasks /Query /TN "CopyThat Scheduled Job"` shows the
      task. Cleanup: `schtasks /Delete /TN "CopyThat Scheduled
      Job" /F`.
      *(harness stub at `e2e/qa-section-4-11f-schedule.spec.ts`;
      schtasks paste-and-run still manual)*
- [x] **CLI render — macOS**: `copythat schedule --spec
      sample.toml --host macos` produces a launchd plist. Drop
      it into `~/Library/LaunchAgents/` →
      `launchctl bootstrap gui/<uid> ~/Library/LaunchAgents/
      app.copythat.scheduled-job.plist` → at the next configured
      interval the job fires.
      *(harness stub at `e2e/qa-section-4-11f-schedule.spec.ts`;
      launchctl bootstrap stays manual)*
- [x] **CLI render — Linux**: `copythat schedule --spec
      sample.toml --host linux` produces a systemd .service +
      .timer pair. Drop into `~/.config/systemd/user/` →
      `systemctl --user daemon-reload` → `systemctl --user
      enable --now copythat-scheduled-job.timer` → check
      `journalctl --user-unit copythat-scheduled-job.service`
      for an execution at the next OnCalendar tick.
      *(harness stub at `e2e/qa-section-4-11f-schedule.spec.ts`;
      systemctl boot stays manual)*
- [x] **Phase 17a guard**: `copythat schedule --spec spec.toml`
      where `spec.toml` references a `..`-laden source rejects
      with `err-path-escape` and exit code 2.
      *(harness stub at `e2e/qa-section-4-11f-schedule.spec.ts`;
      already covered at the Rust layer by
      `cargo test -p copythat-cli --test phase_17_security`)*

### 4.11g Phase 14f — queue-while-locked (Phase 38-followup-2)

- [x] **Volume arrival**: stage a copy whose destination root is
      an unmounted external drive. Plug the drive in →
      `copythat queue --watch` (when the CLI subcommand lands)
      surfaces `VolumeArrival { root }` and proceeds. Plugged-
      out re-fires `VolumeDeparture`.
      *(harness stub at `e2e/qa-section-4-11g-queue-locked.spec.ts`;
      real plug-in still requires hardware)*
- [x] **Cancellation**: while `copythat queue --watch` is
      running, kill the process with Ctrl-C → exits within 2 s
      regardless of poll interval.
      *(harness stub at `e2e/qa-section-4-11g-queue-locked.spec.ts`)*

### 4.11c Phase 38 — destination dedup ladder

- [x] **Mode = AutoLadder** + same-volume copy on a reflink-
      capable filesystem (Btrfs / APFS / ReFS Dev Drive): per-
      file event reports `Reflink` strategy + the file size as
      `bytes_saved`. Total destination volume usage stays close
      to the source's (within a few KiB of metadata overhead).
      *(harness stub at `e2e/qa-section-4-11c-dedup.spec.ts`;
      volume usage check requires real CoW filesystem)*
- [x] **Mode = AutoLadder + HardlinkPolicy = Always** on NTFS:
      same-volume copy reports `Hardlink` per file. Touching
      either name affects the other (because they share the
      inode); the PWA badge surfaces the yellow warning.
      *(harness stub at `e2e/qa-section-4-11c-dedup.spec.ts`)*
- [x] **Mode = ReflinkOnly** on NTFS (no reflink): every file
      reports `Copy` (fallback). No hardlinks created even when
      hardlink_policy is Always.
      *(harness stub at `e2e/qa-section-4-11c-dedup.spec.ts`)*
- [x] **Mode = None** on any volume: every file reports `Skipped`;
      the engine takes its regular `copy_file` path. Identical
      to the pre-Phase-38 behaviour.
      *(harness stub at `e2e/qa-section-4-11c-dedup.spec.ts`)*
- [x] **Pre-pass scan** (when wired): tree with 50 duplicate
      destinations + 50 unique source files lights up the modal
      proposing 50 hardlink/reflink dedup actions. Total dst
      volume usage after applying ≈ source size + chunk overhead
      (not 2× the source size).
      *(harness stub at `e2e/qa-section-4-11c-dedup.spec.ts`;
      requires the dedup-scan IPC to land first)*

### 4.11 Mobile companion (Phase 37)

- [x] First launch shows the onboarding modal with the install QR
      pointing at the PWA URL.
      *(harness stub at `e2e/qa-section-4-11-mobile.spec.ts`)*
- [ ] Scan the QR with an iPhone → Safari opens the PWA → "Add
      to Home Screen" appears → installed icon matches the
      desktop tray icon.
- [ ] Open the installed PWA → "Pair with desktop" → desktop
      Settings → Mobile shows the QR → scan → both sides display
      the same four SAS emojis → confirm → pairing entry persists
      under Settings → Mobile.
- [ ] PWA Home shows live globals (percentage, files done /
      total, rate) while the desktop is running a copy. Per-job
      list updates in place.
- [x] PWA Pause / Resume / Cancel buttons drive the desktop's
      active job. Desktop UI mirrors the state change.
      *(desktop-side wiring stubbed in
      `e2e/qa-section-4-11-mobile.spec.ts`; PWA-side button
      tap stays manual / Computer Use)*
- [x] PWA Collisions panel shows the open prompt → tap
      "Overwrite all" → desktop completes the rest of the tree
      under that policy.
      *(harness stub at `e2e/qa-section-4-11-mobile.spec.ts`;
      PWA tap path stays manual)*
- [x] PWA History panel lists recent rows → tap "Re-run" →
      desktop fires a new job matching the row's source +
      destination.
      *(harness stub at `e2e/qa-section-4-11-mobile.spec.ts`;
      PWA tap path stays manual)*
- [ ] PWA Exit button cleanly disconnects → reopening the PWA
      shows the "Desktop unreachable" state until the desktop
      side comes back.
- [x] Kill the desktop while the PWA is connected → PWA detects
      the disconnect within a few seconds and shows the
      reachability error screen.
      *(desktop-side mobile-disconnect event stubbed in
      `e2e/qa-section-4-11-mobile.spec.ts`; PWA-side reachability
      flip stays manual)*

## 5. Performance + benchmarks

- [x] `cargo run -p xtask -- bench-ci` finishes in under 90 s
      with no regression versus the last committed baseline in
      `docs/BENCHMARKS.md`. *(driven by `xtask qa-automate`;
      regression-vs-baseline still a manual eyeball pass on the
      Criterion output)*
- [ ] `cargo run -p xtask -- bench-vs` runs the head-to-head
      against Robocopy / TeraCopy / FastCopy on a Windows host;
      results within ±5% of the last published numbers in
      `COMPETITOR-TEST.md`.
- [ ] Per-volume buffer-size sweep matches the 1 MiB optimum from
      Phase 13b.
- [ ] Memory: copy a 5 M-file scan database with the Phase 19a
      scanner; peak RSS stays under 200 MiB.
- [ ] **Phase 13c gating** — run `cargo test -p copythat-platform
      --test phase_13c_parallel`. Confirm the parallel-chunk
      path stays env-var-gated by default. To re-bench parallel
      vs single-stream on new hardware:
      `COPYTHAT_PARALLEL_CHUNKS=4 ./target/release/xtask.exe
      bench-vs` and compare against the prior single-stream
      run. **If parallel beats single by ≥10 % across every
      C→C / C→D / C→E scenario**, update
      `crates/copythat-platform/src/native/parallel.rs::requested_chunks`
      to default-on and rerun the smoke. Otherwise keep
      single-stream as default.
- [ ] **Phase 13c — research vs reality** — the COMPETITOR-TEST.md
      "Phase 38 follow-up #2" verdict says `CopyFileExW` with our
      tuning IS optimal on Windows. Re-confirm with a 10 GiB
      C→C and C→E pass; numbers should land within ±5 % of the
      last committed table.
- [ ] **Phase 17g — fat LTO release time**: `cargo build
      --release -p copythat-cli -p copythat-ui` should still
      complete inside the GitHub Actions 45 min cap on
      `windows-latest`. If LTO blows the budget, downgrade to
      `lto = "thin"` and document the regression in
      `docs/SECURITY.md`.

## 6. Cross-platform

Repeat 4.1–4.10 on each:

- [ ] Windows 10 (NTFS).
- [ ] Windows 11 (NTFS, ReFS Dev Drive).
- [ ] macOS 12 / 14 (APFS).
- [ ] Ubuntu 22.04 (ext4).
- [ ] Fedora 40 (Btrfs).

Mobile (Phase 37) checks:

- [ ] iOS Safari → PWA install + pair flow.
- [ ] iOS Chrome (uses Safari WebKit underneath).
- [ ] Android Chrome → PWA install + pair flow.
- [ ] Android Firefox.

## 7. Edge cases

- [ ] Locked files (Phase 19b): copy a file Excel has open →
      VSS snapshot path engages → copy completes from the
      shadow copy.
- [ ] Resume across reboot (Phase 20): start a 5 GiB copy →
      reboot mid-flight → relaunch → resume modal offers to
      continue from the last journal checkpoint.
- [ ] Bandwidth shaping (Phase 21): cap at 5 MB/s → copy rate
      hovers at the cap ±1 MB/s.
- [ ] Sparse files (Phase 23): copy a sparse VM disk image
      between sparse-aware filesystems → destination's allocated
      size matches the source's.
- [ ] Security metadata (Phase 24): copy a file with NTFS ADS
      to ext4 → AppleDouble sidecar lands at the destination.
- [ ] Path translation (Phase 30): copy `Café.txt` from Windows
      to macOS → destination filename is NFD-normalized.
- [ ] Power policy (Phase 31): unplug AC during a long copy →
      engine pauses (if "pause on battery" is on) and resumes
      when AC reconnects.

## 8. i18n

- [ ] Switch language to each of the 18 locales → no English
      strings leak through. `# MT` markers in non-English
      `.ftl` files match `docs/I18N_TODO.md`'s per-locale
      backlog.
- [ ] CLI strings (`copythat --help`) stay English regardless
      of locale (engineering accessibility — documented in
      `cli.rs` after_help block).

## 9. Packaging + signing

- [ ] `cargo run -p xtask -- release` produces installers on
      every target.
- [ ] Windows MSI installs without admin (per-user install).
- [ ] macOS DMG opens; bundled `.app` runs without Gatekeeper
      blocks (signed + notarized).
- [ ] Linux AppImage / deb / rpm install via the package
      manager.
- [ ] Auto-updater (Phase 15) pings the manifest endpoint and
      detects the bumped version.

## 10. Release prep

- [ ] `docs/CHANGELOG.md` `## [Unreleased]` → `## [v1.0.0] —
      YYYY-MM-DD` with a final review pass.
- [ ] `Cargo.toml` workspace version matches.
- [ ] `apps/copythat-ui/src-tauri/tauri.conf.json` `version`
      matches.
- [ ] Tag the release: `git tag v1.0.0 && git push --tags`.
- [ ] GitHub Releases entry copies the CHANGELOG block + the
      installer artifacts.
- [ ] Public docs site (`docs/site/`) rebuilds + republishes.
- [ ] Announce on whatever channels — CopyThat blog, Twitter,
      Reddit r/sysadmin, Hacker News.

---

## Notes on `/ultrareview`

Run `/ultrareview` (no args) **after** §1–§3 pass locally and
**before** §10. It bundles the current branch and farms the review
out to multiple specialized cloud agents (correctness, security,
performance, API design, test coverage). Findings come back as a
unified report; resolve high-severity items before tagging.

`/ultrareview <PR#>` reviews a specific GitHub pull request
instead — useful for the per-phase review pass before merging into
`main`.

The review is **user-triggered and billed**; nothing in the local
toolchain can launch it automatically.

---

## Appendix — automating this checklist after merge

Three categories, ranked by effort:

### Already fully automated (run via cargo / xtask)

- §1 static analysis — `cargo fmt --all -- --check`, `cargo
  clippy --workspace --all-targets -- -D warnings`,
  `cargo deny check`, `pnpm svelte-check`, `pnpm tsc --noEmit`.
- §2 per-crate test suites — `cargo test -p <crate>` per crate;
  the smoke matrix runs via `cargo test --workspace`.
- §3 security gates — `cargo audit` + `cargo vet` (Phase 17b),
  the Phase 17b/c/d/e/f/g smoke tests, `xtask i18n-lint`.
- §5 perf + benchmarks — `xtask bench-ci` + `xtask bench-vs`.
- §9 packaging + signing — the `release.yml` workflow on tag.

`xtask qa-automate` runs every item above as a single command
and emits a `target/qa-report.md` pass/fail table plus an
exit-code-shaped summary on stdout. Source lives in
`xtask/src/qa.rs`; flags like `--skip-bench`, `--skip-tests`,
`--fail-fast`, and `--report <path>` cover the common iteration
shapes. The `cargo audit --ignore` list is parsed out of
`deny.toml` at runtime, so the two surfaces cannot drift.

### Automatable via Playwright (harness shipped; tauri-driver path deferred)

The Playwright harness is now standing under
`apps/copythat-ui/e2e/`. It targets the Vite dev server with
`window.__TAURI_INTERNALS__` shimmed so every §4 frontend flow is
deterministic. One spec file per §4 subsection (18 files, 58
tests) — see `apps/copythat-ui/e2e/README.md` for the design
notes. Run with `pnpm test:e2e` from `apps/copythat-ui`.

State today: the scaffolding is real, two tests are filled in as
worked exemplars (4.1 drop-stack, 4.10 `version --json`), and the
remaining 56 are `test.fixme()` stubs that document the IPC
mocks and DOM assertions each one needs. Filling a stub is
mechanical: register handlers via `tauri.handles({ ... })`,
drive the UI, assert.

**Out of scope for this harness:** hardware-bound checks
(physical drag-drop, real CoW volume, AC unplug), OS consent
dialogs (UAC / sudo / polkit), the PWA browser-side flows, and
anything that needs the real Rust backend wired in (the IPC is
mocked; the engine is covered by `cargo test -p <crate>`). The
canonical Tauri 2.x end-to-end harness pairs `tauri-driver`
with WebdriverIO (Playwright doesn't speak the WebDriver
protocol cleanly); promoting the dozen or so checkboxes whose
value depends on the real binary to that bridge is a future
phase. See `apps/copythat-ui/e2e/README.md` "Deferred" section.

After this harness fills in, **a Claude Code session can drive
the automatable §4 checkboxes via `pnpm test:e2e`** — same
shape as `cargo test` for the Rust side. The remaining
hardware / OS-consent checks still need a human or a Computer
Use session.

### Needs Anthropic Computer Use API or physical hardware

A small set of checks involve physical state that no headless
agent can simulate:

- §4.5 cross-volume sync — needs an actually-mounted second
  volume.
- §6 cross-platform — needs Linux + macOS + Windows hosts. The
  GitHub Actions matrix already covers this for tests + builds;
  manual UI golden path on each OS needs human eyes (or a
  Computer Use session per OS).
- §7 edge cases — VSS (locked Excel file), AC unplug, bandwidth
  shaping under real network. The engine paths are smoke-tested,
  but the *physical signal* (Excel really has the file open, AC
  really got pulled) needs the host machine.

Anthropic's **Computer Use API** can drive a real desktop's
mouse + keyboard from a Claude session — same model class, but
the agent talks screen-recognition + UI-actuation instead of
shell + filesystem. It's a separate billing channel from Claude
Code (you'd run it through the Anthropic API directly). When it
makes sense to spend on it: full dress-rehearsal of §4 + §6 +
§7 right before a release tag. For day-to-day QA, the
Playwright harness above covers 90 % of §4 and is much cheaper.

### Recommended sequence after merge

1. ~~Ship the `xtask qa-automate` subcommand.~~ Done. Wire it
   into `release.yml` as a blocking gate before tagging
   (`cargo run -p xtask -- qa-automate --skip-bench` for the
   fast pre-merge inner loop, full run before tagging).
2. ~~Stand up the Playwright harness.~~ Done — 18 spec files
   under `apps/copythat-ui/e2e/`. Migrate §4 checkboxes from
   `test.fixme()` to filled-in tests one at a time as their
   underlying flow stabilises. (Tauri-driver / WebdriverIO
   bridge for real-binary end-to-end stays deferred — see
   the "Deferred" section in `apps/copythat-ui/e2e/README.md`.)
3. Reserve Computer Use sessions for the pre-tag dress
   rehearsal on every supported OS, plus the hardware-bound
   §4 checks that no headless harness can simulate (physical
   drag, real CoW volume, AC unplug, OS consent dialog).
   Once-per-release cadence keeps the spend bounded.

