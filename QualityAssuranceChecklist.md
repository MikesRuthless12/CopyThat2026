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
      before tagging.
- [ ] `cargo deny check advisories` clean (see §1) — re-audit
      every `[advisories] ignore` entry. Any pre-existing entry
      whose upstream chain has shipped a fix gets removed.
- [ ] `cargo run -p copythat-cli --bin copythat -- verify <sample>
      --algo blake3` round-trips on a known-good file.
- [ ] Path-safety tests run: `cargo test -p copythat-core
      --test phase_17_security` (rejects `..` traversal at the
      engine boundary).
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
