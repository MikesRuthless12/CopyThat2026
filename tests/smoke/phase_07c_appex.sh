#!/usr/bin/env bash
#
# Phase 7c smoke test — macOS Finder Sync Extension + Services.
#
# On macOS:
#   1. Lint Info.plist / Services.plist with `plutil -lint`.
#   2. Type-check every Swift source with `swiftc -typecheck` (does
#      not produce a binary; confirms the source is syntactically
#      valid and references resolvable APIs).
#   3. Run `packaging/macos/scripts/bundle-appex.sh` to produce the
#      universal .appex bundle; verify the layout.
#   4. Verify the bundle passes `codesign --verify`.
#
# On any other host the script exits 0 with an explanation; macOS-
# only artefacts cannot be smoke-tested from Linux or Windows. The
# GitHub Actions macos-latest runner is where this test really runs.
#
# Exit 0 on success, non-zero otherwise.

set -euo pipefail

# ----- Preflight ------------------------------------------------------------

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "phase_07c_appex.sh: host is $(uname -s); macOS-only smoke test skipped."
    echo "(This script exits 0 on non-macOS hosts so cross-platform CI stays green.)"
    exit 0
fi

# ----- Paths ---------------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MACOS_ROOT="$REPO_ROOT/packaging/macos"

echo "==> phase_07c_appex.sh"
echo "    REPO_ROOT  = $REPO_ROOT"

# ----- Step 1: plist lint --------------------------------------------------

echo "==> plutil -lint"
plutil -lint "$MACOS_ROOT/finder-sync-extension/Info.plist"
plutil -lint "$MACOS_ROOT/services/Services.plist"

# ----- Step 2: Swift type-check ------------------------------------------

# -typecheck does not emit code — it asserts the source resolves
# against the standard frameworks. We pass the same frameworks the
# .appex uses so the Finder Sync references compile.

echo "==> swiftc -typecheck CopyThatFinderSync.swift"
swiftc \
    -typecheck \
    -target "arm64-apple-macos12.0" \
    -framework Cocoa \
    -framework FinderSync \
    "$MACOS_ROOT/finder-sync-extension/CopyThatFinderSync.swift"

echo "==> swiftc -typecheck CopyThatServiceHandler.swift"
swiftc \
    -typecheck \
    -target "arm64-apple-macos12.0" \
    -framework Cocoa \
    "$MACOS_ROOT/services/CopyThatServiceHandler.swift"

# ----- Step 3: bundle --------------------------------------------------

echo "==> running bundle-appex.sh"
bash "$MACOS_ROOT/scripts/bundle-appex.sh"

APPEX="$MACOS_ROOT/build/CopyThatFinderSync.appex"

if [[ ! -d "$APPEX" ]]; then
    echo "FAIL: expected .appex directory not found at $APPEX" >&2
    exit 1
fi

# Required files
for path in \
    "$APPEX/Contents/Info.plist" \
    "$APPEX/Contents/MacOS/CopyThatFinderSync"; do
    if [[ ! -e "$path" ]]; then
        echo "FAIL: missing $path" >&2
        exit 1
    fi
done

# NSExtension principal class check
principal="$(plutil -extract NSExtension.NSExtensionPrincipalClass raw -o - \
    "$APPEX/Contents/Info.plist")"
if [[ "$principal" != "CopyThatFinderSync" ]]; then
    echo "FAIL: principal class in Info.plist is '$principal', expected 'CopyThatFinderSync'" >&2
    exit 1
fi

# Universal binary check (arm64 + x86_64 slices present)
lipo_info="$(lipo -info "$APPEX/Contents/MacOS/CopyThatFinderSync")"
echo "    $lipo_info"
if ! grep -q "arm64" <<<"$lipo_info"; then
    echo "FAIL: .appex executable missing arm64 slice" >&2
    exit 1
fi
if ! grep -q "x86_64" <<<"$lipo_info"; then
    echo "FAIL: .appex executable missing x86_64 slice" >&2
    exit 1
fi

# ----- Step 4: codesign verify --------------------------------------------

echo "==> codesign --verify"
codesign --verify --verbose=2 "$APPEX" || {
    echo "FAIL: codesign verification did not pass" >&2
    exit 1
}

echo "==> PASS: phase 07c .appex built, signed, and verified."
