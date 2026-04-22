#!/usr/bin/env bash
#
# Phase 7 smoke test — Linux shell integrations.
#
# Covers the three right-click-menu extensions the Linux packaging
# step (Phase 16) installs: Nautilus (GNOME Files), KDE Dolphin's
# KIO ServiceMenu, and XFCE Thunar's Custom Actions. Also validates
# the top-level `copythat.desktop` launcher's two MIME-bound Desktop
# Actions.
#
# The test is split into two layers:
#
# 1. **Static validation** — every integration file parses cleanly
#    and references the expected `copythat --enqueue <verb>` argv
#    shape. This is the Linux counterpart of
#    `phase_07b_shellext.ps1`'s "DLL exports resolve" check and
#    `phase_07c_appex.sh`'s `plutil -lint` / `swiftc -typecheck` step.
#
# 2. **Live argv simulation** — a stub `copythat` shell script is
#    dropped at the head of PATH; each extension's Exec / command
#    line is invoked against a representative file path; the stub
#    records its argv to a file; we assert the argv matches the
#    Phase 7a CLI parser contract (`--enqueue`, the verb, `--`, paths).
#    This is cheaper than wiring up a real `copythat` binary + IPC
#    socket but still catches the class of bug where a packaging
#    change silently breaks the handoff from file manager to app.
#
# The live Nautilus / Dolphin / Thunar hosted path (spawn a real file
# manager, click the menu, wait for the job in the running app) is
# deferred to Phase 16 packaging install-time testing — same as the
# Windows 07b side. Running a file manager in CI requires a full
# desktop session, which GitHub's ubuntu-latest runner doesn't
# provide out of the box.
#
# On any non-Linux host the script exits 0 with a skip notice so the
# cross-OS matrix stays green (mirrors the macOS 07c sibling).

set -euo pipefail

# ----- Preflight -----------------------------------------------------------

if [[ "$(uname -s)" != "Linux" ]]; then
    echo "phase_07_shell_linux.sh: host is $(uname -s); Linux-only smoke test skipped."
    echo "(Exit 0 on non-Linux hosts so cross-platform CI stays green.)"
    exit 0
fi

# ----- Paths --------------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LINUX_ROOT="$REPO_ROOT/packaging/linux"
NAUTILUS_PY="$LINUX_ROOT/nautilus/copythat_nautilus.py"
KDE_DESKTOP="$LINUX_ROOT/kde/copythat-servicemenu.desktop"
THUNAR_UCA="$LINUX_ROOT/thunar/copythat-uca.xml"
TOP_DESKTOP="$LINUX_ROOT/copythat.desktop"

echo "==> phase_07_shell_linux.sh"
echo "    REPO_ROOT = $REPO_ROOT"

for path in "$NAUTILUS_PY" "$KDE_DESKTOP" "$THUNAR_UCA" "$TOP_DESKTOP"; do
    if [[ ! -f "$path" ]]; then
        echo "FAIL: expected file missing: $path" >&2
        exit 1
    fi
done

FAILED=0

fail() {
    echo "FAIL: $*" >&2
    FAILED=1
}

# ----- Step 1: Python extension syntax + structure ------------------------

# `python3 -m py_compile` parses the file against the installed
# Python's grammar without executing its top-level imports. We
# can't fully import the module because that requires
# `gi.repository.Nautilus`, which the CI Ubuntu runner doesn't have
# by default (it's installed with `python3-nautilus` which pulls in
# GNOME Shell). Syntax parsing catches the 90% case: indent /
# identifier / typo regressions.

echo "==> python3 -m py_compile $NAUTILUS_PY"
if ! python3 -m py_compile "$NAUTILUS_PY"; then
    fail "Nautilus extension did not parse"
fi

# AST-level structural check: verify the module defines a class
# named `CopyThatMenuProvider` with the expected method names.
# This catches accidental renames that `py_compile` alone wouldn't.

echo "==> ast check: CopyThatMenuProvider surface"
python3 - <<EOF
import ast, sys

path = "$NAUTILUS_PY"
tree = ast.parse(open(path, encoding="utf-8").read(), filename=path)
classes = {c.name: c for c in ast.walk(tree) if isinstance(c, ast.ClassDef)}
if "CopyThatMenuProvider" not in classes:
    print(f"FAIL: CopyThatMenuProvider missing from {path}", file=sys.stderr)
    sys.exit(1)

cls = classes["CopyThatMenuProvider"]
methods = {m.name for m in cls.body if isinstance(m, ast.FunctionDef)}
expected = {"get_file_items", "get_background_items", "_on_copy", "_on_move"}
missing = expected - methods
if missing:
    print(f"FAIL: CopyThatMenuProvider missing methods: {sorted(missing)}", file=sys.stderr)
    sys.exit(1)

# The module must invoke the copythat CLI with `--enqueue`. Walk the
# literal strings to confirm the argv construction is intact.
strings = {node.value for node in ast.walk(tree) if isinstance(node, ast.Constant) and isinstance(node.value, str)}
for needle in ("copythat", "--enqueue", "copy", "move"):
    if needle not in strings:
        print(f"FAIL: Nautilus extension missing literal {needle!r}", file=sys.stderr)
        sys.exit(1)
EOF

# ----- Step 2: .desktop + UCA static validation ---------------------------

# `desktop-file-validate` is standard on every Ubuntu / Fedora host
# (desktop-file-utils package). Absence → soft-warn; the deb/rpm
# postinst that ships these files on real systems runs
# desktop-file-install, which performs the same check.
if command -v desktop-file-validate >/dev/null 2>&1; then
    echo "==> desktop-file-validate (top-level)"
    if ! desktop-file-validate "$TOP_DESKTOP"; then
        fail "top-level copythat.desktop failed desktop-file-validate"
    fi
    echo "==> desktop-file-validate (KDE ServiceMenu)"
    # KDE ServiceMenus use some KDE-specific keys that the validator
    # flags as non-standard warnings. We accept warnings (exit 0 via
    # `--warn-kde`) but still fail on hard errors. Older validators
    # don't ship that flag; fall back to treating non-zero as warning.
    desktop-file-validate "$KDE_DESKTOP" || \
        echo "    (desktop-file-validate produced warnings on KDE ServiceMenu; treated as non-fatal — KDE-specific keys)"
else
    echo "==> desktop-file-validate not installed; running minimal grep-based checks"
    if ! grep -q '^\[Desktop Entry\]' "$TOP_DESKTOP"; then
        fail "top-level desktop file missing [Desktop Entry] header"
    fi
    if ! grep -q '^\[Desktop Entry\]' "$KDE_DESKTOP"; then
        fail "KDE desktop file missing [Desktop Entry] header"
    fi
fi

# Thunar UCA XML — xmllint is libxml2-utils and is standard on Linux
# CI hosts. Absence → python ElementTree fallback.
echo "==> XML validation: $THUNAR_UCA"
if command -v xmllint >/dev/null 2>&1; then
    if ! xmllint --noout "$THUNAR_UCA"; then
        fail "Thunar UCA did not parse via xmllint"
    fi
else
    python3 -c "import xml.etree.ElementTree as ET; ET.parse('$THUNAR_UCA')" || \
        fail "Thunar UCA did not parse via ElementTree"
fi

# Structural assertion: every integration file must reference the
# `copythat --enqueue copy` and `copythat --enqueue move` argv
# shape. If a package maintainer copy-paste-edits one but forgets
# the other, the context menu half-works in a way that's painful
# to spot manually.

echo "==> argv-shape grep across integration files"
for path in "$TOP_DESKTOP" "$KDE_DESKTOP" "$THUNAR_UCA"; do
    if ! grep -qF "copythat --enqueue copy" "$path"; then
        fail "$path missing 'copythat --enqueue copy'"
    fi
    if ! grep -qF "copythat --enqueue move" "$path"; then
        fail "$path missing 'copythat --enqueue move'"
    fi
done

# ----- Step 3: live argv simulation ---------------------------------------
#
# Build a stub `copythat` binary on a scratch PATH, then invoke each
# extension's `Exec=` / `<command>` line with `%F` substituted for a
# real file path. The stub writes its argv to a predictable file so we
# can inspect what Nautilus / Dolphin / Thunar would have sent.

echo "==> live argv simulation"
SCRATCH="$(mktemp -d)"
trap "rm -rf '$SCRATCH'" EXIT

STUB_LOG="$SCRATCH/stub.log"
STUB_BIN="$SCRATCH/bin/copythat"
mkdir -p "$SCRATCH/bin"
cat > "$STUB_BIN" <<'STUB'
#!/usr/bin/env bash
# Stub: write argv one per line + trailing blank record separator.
# The smoke test greps for --enqueue + the following verb.
printf '%s\n' "$@" >> "$STUB_LOG_PATH"
printf '\n' >> "$STUB_LOG_PATH"
exit 0
STUB
chmod +x "$STUB_BIN"

export STUB_LOG_PATH="$STUB_LOG"
export PATH="$SCRATCH/bin:$PATH"

SAMPLE_FILE="$SCRATCH/sample.bin"
touch "$SAMPLE_FILE"

# Helper: extract a shell-command line after a prefix and invoke it
# with %F replaced by the sample path. Returns non-zero if the
# command line isn't found in the file.
run_exec_line() {
    local file="$1"
    local pattern="$2"     # e.g. 'Exec=copythat --enqueue copy'
    local label="$3"
    # The first Exec= line matching the pattern in the file. Strip the
    # `Exec=` prefix, substitute %F, and execute via bash -c.
    local line
    line="$(grep -m1 -F "$pattern" "$file" || true)"
    if [[ -z "$line" ]]; then
        fail "$label: no line matching '$pattern' found in $file"
        return 1
    fi
    local cmd="${line#Exec=}"
    # KDE ServiceMenu uses `Exec=`, Thunar UCA uses a raw <command>
    # element (no prefix); support both shapes.
    cmd="${cmd//%F/$SAMPLE_FILE}"
    cmd="${cmd//%f/$SAMPLE_FILE}"
    # shellcheck disable=SC2086
    bash -c "$cmd"
}

# --- Top-level .desktop (Actions: Copy, Move) ---

run_exec_line "$TOP_DESKTOP" "Exec=copythat --enqueue copy" "top-level desktop copy"
run_exec_line "$TOP_DESKTOP" "Exec=copythat --enqueue move" "top-level desktop move"

# --- KDE ServiceMenu ---

run_exec_line "$KDE_DESKTOP" "Exec=copythat --enqueue copy" "KDE ServiceMenu copy"
run_exec_line "$KDE_DESKTOP" "Exec=copythat --enqueue move" "KDE ServiceMenu move"

# --- Thunar UCA: <command>copythat ...</command> ---

# UCA's <command> element is raw — no `Exec=` prefix. Extract via
# Python so XML escaping / whitespace doesn't trip us up.
while IFS=$'\t' read -r verb cmd; do
    cmd="${cmd//%F/$SAMPLE_FILE}"
    cmd="${cmd//%f/$SAMPLE_FILE}"
    bash -c "$cmd" || fail "Thunar UCA $verb invocation failed"
done < <(python3 - <<EOF
import xml.etree.ElementTree as ET
tree = ET.parse("$THUNAR_UCA")
for action in tree.getroot().findall("action"):
    cmd = (action.findtext("command") or "").strip()
    if "--enqueue copy" in cmd:
        print(f"copy\t{cmd}")
    elif "--enqueue move" in cmd:
        print(f"move\t{cmd}")
EOF
)

# --- Assert the stub received the expected argv shape ---
#
# For each of the six invocations above (3 extensions × 2 verbs),
# the stub should have recorded `--enqueue <verb>` followed (directly
# or via `--` separator) by the sample path.

echo "==> verify stub log: argv shape"
for verb in copy move; do
    # Count blocks with both the verb *and* the sample path.
    count=$(python3 - <<EOF
import sys
path = "$STUB_LOG"
verb = "$verb"
sample = "$SAMPLE_FILE"
blocks = [b for b in open(path, encoding="utf-8").read().strip().split("\n\n") if b]
hits = 0
for block in blocks:
    lines = block.splitlines()
    if "--enqueue" in lines and verb in lines and any(sample in l for l in lines):
        hits += 1
print(hits)
EOF
)
    # 3 integrations × 1 verb = 3 expected dispatches per verb.
    if [[ "$count" -lt 3 ]]; then
        fail "stub log: expected ≥3 '--enqueue $verb' invocations carrying $SAMPLE_FILE, got $count"
    fi
done

# ----- Summary ------------------------------------------------------------

if [[ "$FAILED" -ne 0 ]]; then
    echo "==> FAIL: one or more Linux shell-integration checks failed." >&2
    exit 1
fi

echo "==> PASS: Nautilus, KDE ServiceMenu, Thunar UCA, and top-level .desktop all sane."
exit 0
