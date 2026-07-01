#!/usr/bin/env python3
"""Phase 7 follow-up — REAL-runtime load check for the Nautilus extension.

Unlike `tests/smoke/phase_07_shell_linux.sh` (py_compile + AST only), this
imports `packaging/linux/nautilus/copythat_nautilus.py` against the REAL
`gi.repository.Nautilus` typelib, instantiates the provider, and exercises
`get_file_items` / `get_background_items` — the runtime discoverability the
static check cannot perform.

Requires the Nautilus 3.0 GIR typelib (python3-gi + gir1.2-nautilus-3.0),
which is why CI runs it inside an ubuntu:20.04 container (modern distros
ship Nautilus 4.x). See tests/smoke/nautilus/Dockerfile + run.sh for local
reproduction and the `nautilus-runtime` job in .github/workflows/ci.yml.

Exits non-zero on any failure.
"""
import os
import sys

EXT_DIR = os.environ.get(
    "COPYTHAT_NAUTILUS_DIR",
    os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "..", "..", "..", "packaging", "linux", "nautilus",
    ),
)
sys.path.insert(0, os.path.abspath(EXT_DIR))

# Importing the module runs gi.require_version("Nautilus", "3.0") and
# `from gi.repository import Nautilus` — the exact runtime resolution the
# static AST check cannot perform. An ImportError/ValueError here means the
# extension would not load in a real GNOME Files session.
import copythat_nautilus as ext  # noqa: E402
from gi.repository import Nautilus  # noqa: E402


class _FakeFile:
    """Minimal stand-in for Nautilus.FileInfo — the provider only calls
    get_uri() on each selected file."""

    def __init__(self, uri):
        self._uri = uri

    def get_uri(self):
        return self._uri


def main():
    failures = []

    def check(cond, msg):
        if not cond:
            failures.append(msg)

    # 1. Provider instantiates and IS a real Nautilus.MenuProvider.
    prov = ext.CopyThatMenuProvider()
    check(
        isinstance(prov, Nautilus.MenuProvider),
        "CopyThatMenuProvider is not a Nautilus.MenuProvider",
    )

    # 2. get_file_items returns the two real Nautilus.MenuItem entries.
    files = [
        _FakeFile("file:///tmp/alpha.txt"),
        _FakeFile("file:///tmp/beta%20gamma.bin"),
    ]
    items = prov.get_file_items(None, files)
    check(len(items) == 2, "expected 2 menu items, got {}".format(len(items)))
    if len(items) == 2:
        check(
            all(isinstance(i, Nautilus.MenuItem) for i in items),
            "menu entries are not real Nautilus.MenuItem objects",
        )
        names = sorted(i.get_property("name") for i in items)
        check(
            names == ["CopyThatMenuProvider::Copy", "CopyThatMenuProvider::Move"],
            "unexpected menu item names: {}".format(names),
        )
        labels = sorted(i.get_property("label") for i in items)
        check(
            labels == ["Copy with Freally File Manager", "Move with Freally File Manager"],
            "unexpected menu item labels: {}".format(labels),
        )

    # 3. Empty selection -> no items.
    check(prov.get_file_items(None, []) == [], "empty selection should yield no items")

    # 4. Background (empty-space) right-click contributes nothing.
    check(
        prov.get_background_items(None, None) == [],
        "get_background_items should be empty",
    )

    # 5. URI->path decoding (including percent-decoding) is correct.
    check(
        ext._uri_to_path("file:///tmp/beta%20gamma.bin") == "/tmp/beta gamma.bin",
        "percent-decoding of file:// URI failed",
    )
    check(
        ext._uri_to_path("trash:///") == "",
        "non-file scheme should map to empty path",
    )

    if failures:
        print("NAUTILUS RUNTIME CHECK: FAIL")
        for f in failures:
            print("  - " + f)
        return 1
    print("NAUTILUS RUNTIME CHECK: OK — provider loaded against the real "
          "Nautilus typelib; 2 menu items + background + URI decode verified")
    return 0


if __name__ == "__main__":
    sys.exit(main())
