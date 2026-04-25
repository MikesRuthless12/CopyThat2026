# Copy That — Quality Assurance Checklist

End-to-end pre-release QA. Walk this top to bottom before tagging
v1.0.0 and shipping installers. Each section is independently
runnable; the order is the order you should test in if you have
limited time.

---

## 0. Pre-flight

- [ ] Working tree clean (`git status` reports nothing pending).
- [ ] On the release branch (typically `main`).
- [ ] `Cargo.lock` committed.
- [ ] Workspace `[workspace.package] version` bumped to the target
      release.
- [ ] All 18 locales pass `cargo run -p xtask -- i18n-lint`.
- [ ] `docs/CHANGELOG.md` `## [Unreleased]` block has an entry for
      every shipped phase since the last release.
- [ ] `docs/ROADMAP.md` checkboxes match what's actually shipped
      (`[x]` for done, `[ ]` for pending — no stale `[x]` from a
      prior session that got reverted).

---

## 1. Static analysis

- [ ] `cargo fmt --all -- --check` — clean.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
      passes for every workspace crate.
- [ ] `cargo deny check` — `advisories ok, bans ok, licenses ok,
      sources ok`. Inherited transitive advisories are documented
      in `deny.toml`'s ignore list with provenance + threat-model
      comments.
- [ ] No `unsafe` code outside `crates/copythat-platform`. Run
      `rg -t rust 'unsafe'` and confirm every hit is in
      `copythat-platform` with a `// SAFETY:` comment alongside.
- [ ] `cd apps/copythat-ui && pnpm svelte-check` — zero errors,
      zero warnings.
- [ ] `cd apps/copythat-ui && pnpm tsc --noEmit` — clean.

## 2. Test suites

- [ ] Per-crate unit tests pass: run `cargo test -p <crate>` for
      every workspace crate. Skip the full `cargo test --all` —
      it's too slow on this workspace; iterate per-crate.
- [ ] Every `tests/smoke/phase_*.rs` smoke test passes:
      `cargo test --test phase_NN_*` for every phase number.
- [ ] Tauri shell loads without panicking
      (`cargo run -p copythat-ui` boots to the main window).
- [ ] `pnpm tauri dev` builds the dev bundle end-to-end.

## 3. Security review

- [ ] Run **`/ultrareview`** on the release branch. Review every
      finding the agents surface; resolve high-severity issues
      before tagging. **Phase 38-followup-3** prep doc lives at
      `docs/SECURITY_REVIEW_PREP.md` — read it before kicking off
      the run so the agents focus on the right sub-phases.
- [ ] `cargo deny check advisories` clean (see §1) — re-audit
      every `[advisories] ignore` entry. Any pre-existing entry
      whose upstream chain has shipped a fix gets removed.
- [ ] **Phase 17b** — `cargo audit` runs on every push (ci.yml
      `cargo-audit:` job). Confirm the `--ignore` list mirrors
      `deny.toml`'s `[advisories] ignore` block exactly; the
      `phase_17b_supply_chain` smoke trips on drift.
- [ ] **Phase 17b** — `cargo vet` runs on every push (ci.yml
      `cargo-vet:` job). Confirm the imports in
      `supply-chain/config.toml` (Mozilla / Google / Embark /
      Bytecode Alliance / Zcash) still resolve.
- [ ] `cargo run -p copythat-cli --bin copythat -- verify <sample>
      --algo blake3` round-trips on a known-good file.
- [ ] Path-safety tests run: `cargo test -p copythat-core
      --test phase_17_security` (rejects `..` traversal at the
      engine boundary).
- [ ] **Phase 17c** — symlink-race regression: `cargo test -p
      copythat-core --test phase_17c_symlink_race`. The Unix-gated
      `copy_file_rejects_post_check_symlink_swap_unix` only fires
      on Linux/macOS — run there explicitly.
- [ ] **Phase 17d** — privilege-separation helper smoke: `cargo
      test -p copythat-helper`. Confirm the `Hello` handshake +
      capability-denied + path-rejection paths all surface the
      right typed responses.
- [ ] **Phase 17d (manual)** — drive the `retry_elevated` IPC
      with a permission-denied source: an unprivileged Copy That
      copy of a system-owned file should surface
      `err-permission-denied` (today the helper runs in-process; a
      future body fill spawns it via UAC / sudo / polkit and the
      OS consent dialog must appear before the elevated retry runs).
- [ ] **Phase 17e** — IPC argument audit smoke: `cargo test -p
      copythat-ui --test phase_17e_ipc_audit`. The
      `commands_rs_path_args_pass_through_the_gate` test walks
      `commands.rs` and trips the build if a new path-typed
      `#[tauri::command]` lands without the gate.
- [ ] **Phase 17f** — log-content scrub: `cargo test -p
      copythat-audit --test phase_17f_logging_scrub`. Then
      manually grep production stderr after a 1 GiB copy:
      `cargo run -p copythat-ui 2>&1 | grep -E '(/[^/ ]+){2,}'`
      should return nothing — user paths never reach stderr.
- [ ] **Phase 17g** — binary hardening tripwire: `cargo test -p
      copythat-cli --test phase_17g_hardening`. Confirm the
      release build actually gets the flags by inspecting the
      Linux binary: `readelf -d target/release/copythat | grep
      -E '(BIND_NOW|RELRO)'` should return non-empty rows.
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

Spin up the dev build, then run through:

### 4.1 Copy

- [ ] Drag a 100 MiB file onto the Copy That window → Drop Stack
      lights up → confirm dialog → Copy → progress bar reaches
      100% → completion toast.
- [ ] Drag a 1 GiB folder → tree-progress bar accumulates →
      completes → totals bump in the footer.
- [ ] Drag onto a destination that already has the file → collision
      modal → pick "Overwrite" → file replaced. Repeat with
      "Skip" → file untouched. Repeat with "Rename" → unique
      sibling created.
- [ ] Cross-volume copy → engine falls back from reflink to byte-
      copy (Phase 6 fast-paths). No reflink event in the journal.

### 4.2 Move

- [ ] Same-volume move → atomic rename. Source disappears.
- [ ] Cross-volume move → engine falls back to copy + delete
      (EXDEV path). Source disappears only after copy succeeds.
- [ ] Cancel a long-running move → source still present, partial
      destination cleaned up unless `keep_partial` is on.

### 4.3 Verify

- [ ] Settings → Transfer → Verify = `blake3`. Run a copy → after
      bytes finish, verify pass runs → green checkmark in the row.
- [ ] Inject a verify mismatch (modify the destination mid-copy
      via a separate process) → engine surfaces `VerifyFailed` →
      partial destination removed.

### 4.4 Secure delete

- [ ] Select a file → Right-click → Secure Delete (DoD 3-pass) →
      confirmation modal → progress → file gone, confirmed by
      Explorer + by `dir`.
- [ ] On a CoW filesystem (Btrfs / APFS / ReFS) → Secure Delete
      surfaces the SSD-aware refusal explanation.

### 4.5 Sync (Phase 25)

- [ ] Add a sync pair → toggle live-mirror → modify a file on the
      left tree → right tree updates within the watcher debounce.
- [ ] Modify the same file on both sides → vector-clock conflict
      modal opens → pick "Keep left" → right side overwrites.

### 4.6 Cloud (Phase 32)

- [ ] Add an S3 backend with a test bucket → `Test connection` →
      green. Copy a file to the backend → object lands.
- [ ] Add a Dropbox backend via OAuth PKCE → completes browser
      flow → backend listed.
- [ ] Copy from a backend back to local → file lands.

### 4.7 Mount (Phase 33)

- [ ] History → Mount snapshot → Explorer (or Finder / `ls`) opens
      the read-only mount. Random-access read of a chunk-sharded
      file works.
- [ ] Unmount → mountpoint disappears.

### 4.8 Audit log (Phase 34)

- [ ] Settings → Advanced → Audit log → enable + JSON-Lines
      format. Run a copy → audit file gains a `JobStarted` +
      `JobCompleted` record.
- [ ] WORM mode on → try to truncate the audit file → OS refuses
      (Linux `chattr +a`, Windows read-only attribute).
- [ ] `Verify chain` button → green for an untampered log; red
      after `dd` overwrites a record.

### 4.9 Encryption + compression (Phase 35)

- [ ] Settings → Transfer → Encryption = recipients + a recipients
      file with one `age1…` line. Copy → destination is age-
      encrypted (`rage --decrypt` round-trips).
- [ ] Compression = Smart, level 3. Copy a `.txt` → destination
      shrinks. Copy a `.jpg` → destination unchanged (smart
      deny-list skipped it).

### 4.10 CLI (Phase 36)

- [ ] `copythat version --json` emits a parseable JSON object.
- [ ] `copythat copy <src> <dst> --json` emits one event per line
      on stdout; every line parses.
- [ ] `copythat plan --spec sample.toml` reports the action list
      and exits 2 with pending actions.
- [ ] `copythat apply --spec sample.toml` runs them; re-applying
      exits 0 with zero new actions (idempotency).
- [ ] `copythat verify <file> --algo blake3 --against <sidecar>`
      with a tampered sidecar exits 4.

### 4.11a Phase 37 follow-up #2 (deferred items closed)

- [ ] **First-launch onboarding modal** appears once on a fresh
      install with no paired phone. Shows the desktop icon, the
      install QR pointing at the deployed PWA URL, and "I have the
      app, pair now" / "Maybe later" buttons. After dismissing,
      the modal does not reappear on subsequent launches.
- [ ] **Wake-lock toggle on the PWA** actually inhibits the
      desktop's screensaver / sleep:
      - Windows: Power → Power Options → display still on after
        the configured idle timer.
      - macOS: Caffeinate equivalent → display stays on until the
        toggle is flipped off.
      - Linux: GNOME / KDE screensaver inhibited via dbus.
      Toggle off → screensaver resumes after the OS idle timer.
- [ ] **Job snapshot is real.** Start a desktop copy → PWA Active
      Jobs panel reflects the running job with real `bytes_done`,
      `files_done`, percentage. Pause from PWA → desktop UI shows
      the job paused. Cancel from PWA → desktop UI shows the job
      cancelled. Reverse path also works (pause from desktop UI →
      PWA reflects within ~5 s).
- [ ] **Native Tauri Mobile binary** scaffold compiles when run
      from a macOS host (`cargo tauri ios build`) or Android-SDK-
      equipped host (`cargo tauri android build`). Verify the icon
      matches the desktop tray icon on both home screens.

### 4.11b Locale sync (Phase 38 PWA i18n)

- [ ] Switch desktop language to French (Settings → General →
      Language). Open the PWA on a paired phone. PWA UI strings
      flip to French within one second of `Hello` completing.
- [ ] Repeat for Japanese, Arabic (RTL), Chinese — each forces
      the PWA to load the matching bundle. MT-flagged strings
      fall back to English where translations are still pending
      (documented in `docs/I18N_TODO.md`).

### 4.11d Phase 8 partials (Phase 38-followup-3)

- [ ] **Settings → General → Error prompt style** dropdown shows
      both `Modal` and `Drawer` options; switching to `Drawer`
      makes the next per-file error appear in the corner panel
      rather than blocking the queue. Switching back to `Modal`
      restores the blocking dialog. Choice survives a Settings
      modal close + reopen and an app restart.
- [ ] **Collision modal → Quick hash (SHA-256)** button: drag a
      file onto a destination that already has an identically-
      named file → modal opens → tap the SHA-256 button on each
      side → both digests render within a second. Confirm: a
      file modified 1 byte produces a different digest than its
      sibling.
- [ ] **Retry with elevated permissions** button on the error
      modal: stage a copy of a system-protected file (e.g.
      `C:\Windows\System32\drivers\etc\hosts`) → engine surfaces
      `err-permission-denied` → tap "Retry with elevated
      permissions" → today the helper runs in-process and the
      retry surfaces the same OS-level permission error
      (`err-permission-denied`); the future UAC / sudo / polkit
      body fill must show the OS consent dialog first and only
      then attempt the elevated copy.

### 4.11e Phase 31b — real OS power probes (Phase 38-followup-3)

- [ ] **Windows presentation mode**: enable Focus Assist
      (Settings → System → Notifications → Focus assist → Off →
      Alarms only). Start a 1 GiB cross-volume copy with
      `PresentationPolicy = Pause`. Confirm: the engine pauses
      within 5 s of Focus Assist flipping on; resumes when it
      flips off.
- [ ] **Windows fullscreen mode**: launch a fullscreen game or
      a fullscreen Direct3D video. Same assertion — the engine
      pauses while D3D fullscreen is active, resumes when you
      Alt-Tab out.
- [ ] **Linux DBus screensaver**: enable presentation inhibit
      via `dbus-send --session --print-reply
      --dest=org.freedesktop.ScreenSaver
      /org/freedesktop/ScreenSaver
      org.freedesktop.ScreenSaver.Inhibit string:test
      string:'qa pass'`. Confirm: the engine pauses if the
      policy is set to Pause; resumes when the cookie is
      released via `UnInhibit`.
- [ ] **macOS** — presentation/fullscreen probe stays a stub on
      this release. The PowerPolicy dropdown should still let
      the user pick `Pause` for documentation purposes; the
      engine simply never sees a "presenting" event today.

### 4.11f Phase 14d — scheduled jobs (Phase 38-followup-2)

- [ ] **CLI render — Windows**: `copythat schedule --spec
      sample.toml` on Windows produces a `schtasks /Create`
      command line. Copy-paste it into an elevated cmd.exe →
      `schtasks /Query /TN "CopyThat Scheduled Job"` shows the
      task. Cleanup: `schtasks /Delete /TN "CopyThat Scheduled
      Job" /F`.
- [ ] **CLI render — macOS**: `copythat schedule --spec
      sample.toml --host macos` produces a launchd plist. Drop
      it into `~/Library/LaunchAgents/` →
      `launchctl bootstrap gui/<uid> ~/Library/LaunchAgents/
      app.copythat.scheduled-job.plist` → at the next configured
      interval the job fires.
- [ ] **CLI render — Linux**: `copythat schedule --spec
      sample.toml --host linux` produces a systemd .service +
      .timer pair. Drop into `~/.config/systemd/user/` →
      `systemctl --user daemon-reload` → `systemctl --user
      enable --now copythat-scheduled-job.timer` → check
      `journalctl --user-unit copythat-scheduled-job.service`
      for an execution at the next OnCalendar tick.
- [ ] **Phase 17a guard**: `copythat schedule --spec spec.toml`
      where `spec.toml` references a `..`-laden source rejects
      with `err-path-escape` and exit code 2.

### 4.11g Phase 14f — queue-while-locked (Phase 38-followup-2)

- [ ] **Volume arrival**: stage a copy whose destination root is
      an unmounted external drive. Plug the drive in →
      `copythat queue --watch` (when the CLI subcommand lands)
      surfaces `VolumeArrival { root }` and proceeds. Plugged-
      out re-fires `VolumeDeparture`.
- [ ] **Cancellation**: while `copythat queue --watch` is
      running, kill the process with Ctrl-C → exits within 2 s
      regardless of poll interval.

### 4.11c Phase 38 — destination dedup ladder

- [ ] **Mode = AutoLadder** + same-volume copy on a reflink-
      capable filesystem (Btrfs / APFS / ReFS Dev Drive): per-
      file event reports `Reflink` strategy + the file size as
      `bytes_saved`. Total destination volume usage stays close
      to the source's (within a few KiB of metadata overhead).
- [ ] **Mode = AutoLadder + HardlinkPolicy = Always** on NTFS:
      same-volume copy reports `Hardlink` per file. Touching
      either name affects the other (because they share the
      inode); the PWA badge surfaces the yellow warning.
- [ ] **Mode = ReflinkOnly** on NTFS (no reflink): every file
      reports `Copy` (fallback). No hardlinks created even when
      hardlink_policy is Always.
- [ ] **Mode = None** on any volume: every file reports `Skipped`;
      the engine takes its regular `copy_file` path. Identical
      to the pre-Phase-38 behaviour.
- [ ] **Pre-pass scan** (when wired): tree with 50 duplicate
      destinations + 50 unique source files lights up the modal
      proposing 50 hardlink/reflink dedup actions. Total dst
      volume usage after applying ≈ source size + chunk overhead
      (not 2× the source size).

### 4.11 Mobile companion (Phase 37)

- [ ] First launch shows the onboarding modal with the install QR
      pointing at the PWA URL.
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
- [ ] PWA Pause / Resume / Cancel buttons drive the desktop's
      active job. Desktop UI mirrors the state change.
- [ ] PWA Collisions panel shows the open prompt → tap
      "Overwrite all" → desktop completes the rest of the tree
      under that policy.
- [ ] PWA History panel lists recent rows → tap "Re-run" →
      desktop fires a new job matching the row's source +
      destination.
- [ ] PWA Exit button cleanly disconnects → reopening the PWA
      shows the "Desktop unreachable" state until the desktop
      side comes back.
- [ ] Kill the desktop while the PWA is connected → PWA detects
      the disconnect within a few seconds and shows the
      reachability error screen.

## 5. Performance + benchmarks

- [ ] `cargo run -p xtask -- bench-ci` finishes in under 90 s
      with no regression versus the last committed baseline in
      `docs/BENCHMARKS.md`.
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

A future `xtask qa-automate` subcommand can run all of the above
and emit a single pass/fail report. ~1 day of work; most of the
plumbing already exists in `xtask/src/{bench.rs,main.rs}`.

### Automatable via Playwright + tauri-driver (deferred since Phase 29)

Every checkbox in §4 (`Manual UI golden path`) that involves
clicking a button, dragging a file, or watching a modal can be
driven by a Playwright test once the tauri-driver harness is
standing. The harness setup is the work; once it's there, each
§4 checkbox becomes one Playwright test file. ~3 days of setup;
adds ~4 minutes to the CI matrix.

After the harness exists, **a Claude Code session can drive
every §4 checkbox from the workspace** without the user having
to click anything — same as the Rust smoke tests today.

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

1. Ship the `xtask qa-automate` subcommand. One command runs
   §1 + §2 + §3 + §5 + §9 — call it from `release.yml` as a
   blocking gate before tagging.
2. Stand up the Playwright + tauri-driver harness (Phase 29
   deferred item). Migrate §4 checkboxes one at a time as
   their underlying flow stabilises.
3. Reserve Computer Use sessions for the pre-tag dress
   rehearsal on every supported OS. Once-per-release cadence
   keeps the spend bounded.

