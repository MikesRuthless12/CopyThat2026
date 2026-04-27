#!/usr/bin/env bash
#
# Build the Copy That v1.0.0 Finder Sync Extension (.appex) bundle.
#
# Produces `packaging/macos/build/CopyThatFinderSync.appex/` with the
# layout macOS expects:
#
#   CopyThatFinderSync.appex/
#     Contents/
#       Info.plist                    (copied from finder-sync-extension/)
#       MacOS/
#         CopyThatFinderSync          (Mach-O bundle exec, `swiftc` output)
#
# The bundle is ad-hoc signed (`codesign -s -`) so macOS Gatekeeper
# lets the user load it after a right-click → Open on first launch.
# Notarisation requires a paid Apple Developer account and is out of
# scope for 0.x (see Phase 16 free-packaging policy).
#
# Run from the repo root on macOS:
#     bash packaging/macos/scripts/bundle-appex.sh
#
# Exit codes:
#   0 — success
#   1 — wrong host OS
#   2 — swiftc / codesign missing
#   3 — Swift compile failed
#   4 — codesign failed
#   5 — output verification failed

set -euo pipefail

# ----- Preflight ------------------------------------------------------------

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "bundle-appex.sh: host is $(uname -s); this script only runs on macOS." >&2
    exit 1
fi

for tool in swiftc codesign plutil; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "bundle-appex.sh: required tool '$tool' not on PATH." >&2
        exit 2
    fi
done

# ----- Paths ---------------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MACOS_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$MACOS_ROOT/../.." && pwd)"

SRC="$MACOS_ROOT/finder-sync-extension/CopyThatFinderSync.swift"
INFO_PLIST="$MACOS_ROOT/finder-sync-extension/Info.plist"
BUILD_DIR="$MACOS_ROOT/build"
APPEX="$BUILD_DIR/CopyThatFinderSync.appex"
APPEX_CONTENTS="$APPEX/Contents"
APPEX_MACOS="$APPEX_CONTENTS/MacOS"
APPEX_BIN="$APPEX_MACOS/CopyThatFinderSync"

# Minimum supported OS. Matches LSMinimumSystemVersion in Info.plist and
# the project's README Targets table.
MIN_MACOS="12.0"

# Detect arch — build universal by default so both Apple Silicon and
# Intel users get a working extension.
TARGETS=("arm64-apple-macos${MIN_MACOS}" "x86_64-apple-macos${MIN_MACOS}")

echo "==> bundle-appex.sh"
echo "    SRC          = $SRC"
echo "    INFO_PLIST   = $INFO_PLIST"
echo "    APPEX        = $APPEX"
echo "    MIN_MACOS    = $MIN_MACOS"

# ----- Clean + prepare the bundle skeleton ---------------------------------

rm -rf "$APPEX"
mkdir -p "$APPEX_MACOS"

# ----- Compile Swift for each target, then lipo ----------------------------

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

THIN_BINS=()
for triple in "${TARGETS[@]}"; do
    out="$TMP_DIR/CopyThatFinderSync-${triple}"
    echo "==> Compiling ($triple)"
    swiftc \
        -target "$triple" \
        -module-name CopyThatFinderSync \
        -parse-as-library \
        -emit-executable \
        -framework Cocoa \
        -framework FinderSync \
        -O \
        -o "$out" \
        "$SRC" \
        || { echo "swiftc failed for $triple" >&2; exit 3; }
    THIN_BINS+=("$out")
done

echo "==> Joining thin binaries with lipo"
lipo -create "${THIN_BINS[@]}" -output "$APPEX_BIN"

# ----- Copy Info.plist + validate -----------------------------------------

cp "$INFO_PLIST" "$APPEX_CONTENTS/Info.plist"
plutil -lint "$APPEX_CONTENTS/Info.plist" || {
    echo "Info.plist failed plutil lint" >&2
    exit 5
}

# ----- Ad-hoc sign ---------------------------------------------------------
#
# `-s -` is the ad-hoc (no identity) signature. Gatekeeper treats this
# the same as an unsigned bundle for third-party-distribution purposes
# but still satisfies the macOS codesign-required-for-extension-load
# policy. Upgrade-path: swap `-` for a Developer ID identity when the
# project acquires one (Phase 16 free-packaging policy).

echo "==> Ad-hoc signing"
codesign --force --deep --sign - --timestamp=none "$APPEX" \
    || { echo "codesign failed" >&2; exit 4; }

# ----- Verify bundle layout ------------------------------------------------

required=(
    "$APPEX_CONTENTS/Info.plist"
    "$APPEX_BIN"
)
for path in "${required[@]}"; do
    if [[ ! -e "$path" ]]; then
        echo "bundle verification: missing $path" >&2
        exit 5
    fi
done

# Check the Mach-O includes both architectures.
if ! lipo -info "$APPEX_BIN" | grep -q "arm64"; then
    echo "bundle verification: arm64 slice missing from $APPEX_BIN" >&2
    exit 5
fi
if ! lipo -info "$APPEX_BIN" | grep -q "x86_64"; then
    echo "bundle verification: x86_64 slice missing from $APPEX_BIN" >&2
    exit 5
fi

echo "==> PASS"
echo "    Built: $APPEX"
lipo -info "$APPEX_BIN"
echo "    Size:  $(du -sh "$APPEX" | cut -f1)"
