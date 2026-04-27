#!/usr/bin/env bash
# Phase 42 — i18n drop for the gap-list rollout.
#
# Adds 6 keys covering: paranoid-verify mode, configurable
# sharing-violation retries, OneDrive cloud placeholder warning,
# and the Defender exclusion hint banner.
#
# Non-English locales receive English placeholders for first-pass
# parity (so `xtask i18n-lint` passes and the UI doesn't crash on
# missing keys); proper translations are a follow-up by the
# translation team.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append_en() {
  local file="$root/locales/en/copythat.ftl"
  cat >> "$file" <<'EOF'
phase42-paranoid-verify-label = Paranoid verify
phase42-paranoid-verify-hint = Drops the destination's cached pages and re-reads from disk to catch write-cache lies and silent corruption. About 50% slower than the default verify; off by default.
phase42-sharing-violation-retries-label = Retry attempts on locked source files
phase42-sharing-violation-retries-hint = How many times to retry when another process is holding the source file open with an exclusive lock. Backoff doubles each attempt (50 ms / 100 ms / 200 ms by default). Default 3, matching Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } is a cloud-only OneDrive file. Copying it will trigger a download — up to { $size } over your network connection.
phase42-defender-exclusion-hint = For maximum copy throughput, add the destination folder to Microsoft Defender exclusions before bulk transfers. See docs/PERFORMANCE_TUNING.md.
EOF
}

append_other() {
  local locale="$1"
  local file="$root/locales/$locale/copythat.ftl"
  # English placeholders — translation pending. Each locale gets
  # the same fallback strings so `xtask i18n-lint` reports parity.
  cat >> "$file" <<'EOF'
phase42-paranoid-verify-label = Paranoid verify
phase42-paranoid-verify-hint = Drops the destination's cached pages and re-reads from disk to catch write-cache lies and silent corruption. About 50% slower than the default verify; off by default.
phase42-sharing-violation-retries-label = Retry attempts on locked source files
phase42-sharing-violation-retries-hint = How many times to retry when another process is holding the source file open with an exclusive lock. Backoff doubles each attempt (50 ms / 100 ms / 200 ms by default). Default 3, matching Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } is a cloud-only OneDrive file. Copying it will trigger a download — up to { $size } over your network connection.
phase42-defender-exclusion-hint = For maximum copy throughput, add the destination folder to Microsoft Defender exclusions before bulk transfers. See docs/PERFORMANCE_TUNING.md.
EOF
}

append_en
for code in ar de es fr hi id it ja ko nl pl pt-BR ru tr uk vi zh-CN; do
  append_other "$code"
done

echo "Phase 42 i18n keys appended to all 18 locales."
echo "Note: non-English locales received English placeholders;"
echo "proper translations are a follow-up by the translation team."
