# Copy That v1.0.0 — Nautilus extension.
#
# Adds "Copy with Copy That" and "Move with Copy That" entries to the
# right-click menu on selected files and folders in GNOME Files
# (Nautilus 3.30+). Phase 7a ships this as a skeleton resource; the
# packaging step (Phase 16, deb/rpm/AppImage postinst) drops it into
# /usr/share/nautilus-python/extensions/ (system-wide) or
# ~/.local/share/nautilus-python/extensions/ (per-user).
#
# Runtime requirements on the target system:
#   - python3-nautilus (>= 3.30, provides the `Nautilus` typelib)
#   - python3-gi (GObject bindings)
#   - `copythat` binary in PATH (resolved lazily at click time)
#
# Licence: proprietary (same as the rest of this repo), source-visible.

import gi
gi.require_version("Nautilus", "3.0")
from gi.repository import GObject, Nautilus  # noqa: E402
import shlex
import subprocess
from urllib.parse import unquote, urlparse


def _uri_to_path(uri: str) -> str:
    """Map a Nautilus `file:///…` URI to a POSIX path."""
    parsed = urlparse(uri)
    if parsed.scheme != "file":
        return ""
    return unquote(parsed.path)


def _spawn_copythat(verb: str, paths: list) -> None:
    """Invoke the `copythat` CLI with the chosen verb + selection.

    Detached so Nautilus doesn't wait on the GUI: stdin/stdout/stderr
    go to /dev/null, close_fds=True keeps Nautilus's fds private, and
    start_new_session puts Copy That in its own process group so a
    later Nautilus restart does not take it down.
    """
    argv = ["copythat", "--enqueue", verb, "--", *paths]
    try:
        subprocess.Popen(
            argv,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            close_fds=True,
            start_new_session=True,
        )
    except FileNotFoundError:
        # `copythat` not on PATH — silently fail: the menu item should
        # never have been installed without the app, but we would
        # rather no-op than raise into the Nautilus main loop.
        print("copythat: binary not found on PATH; menu click ignored")
    except OSError as exc:
        print("copythat: spawn failed: {}".format(shlex.quote(str(exc))))


class CopyThatMenuProvider(GObject.GObject, Nautilus.MenuProvider):
    """Menu provider contributing "Copy with Copy That" and
    "Move with Copy That" to file/folder selections."""

    def get_file_items(self, window, files):
        paths = [_uri_to_path(f.get_uri()) for f in files]
        paths = [p for p in paths if p]
        if not paths:
            return []

        copy_item = Nautilus.MenuItem(
            name="CopyThatMenuProvider::Copy",
            label="Copy with Copy That",
            tip="Queue a copy job with Copy That v1.0.0",
        )
        copy_item.connect("activate", self._on_copy, paths)

        move_item = Nautilus.MenuItem(
            name="CopyThatMenuProvider::Move",
            label="Move with Copy That",
            tip="Queue a move job with Copy That v1.0.0",
        )
        move_item.connect("activate", self._on_move, paths)

        return [copy_item, move_item]

    def get_background_items(self, window, folder):
        # Right-click on empty space inside a folder: we don't contribute
        # a menu entry there because there's no selection to act on.
        return []

    def _on_copy(self, _menu, paths):
        _spawn_copythat("copy", paths)

    def _on_move(self, _menu, paths):
        _spawn_copythat("move", paths)
