// Raw Win32 / NSPasteboard FFI requires `unsafe`; same relaxation
// copythat-shellext makes for the same reason. Scoped to this module
// so the rest of the UI crate keeps the workspace default.
#![allow(unsafe_code)]

//! Cross-platform OS clipboard reader — file URLs only.
//!
//! The post-Phase-12 paste-hotkey + clipboard-watcher features both
//! need to answer the same question: "are there files on the OS
//! clipboard right now, and what are their paths?" Each OS exposes
//! this differently; this module hides the difference behind two
//! functions:
//!
//! - [`read_file_paths`] returns every absolute path currently on the
//!   clipboard, or an empty `Vec` if the clipboard holds something
//!   other than files (text, image, nothing at all).
//! - [`sequence_token`] returns a monotonically-increasing counter
//!   that the poll-based watcher uses to detect "clipboard changed
//!   since last tick" without re-reading the actual contents on every
//!   tick. Not every OS has a stable counter (Linux/Wayland doesn't
//!   expose one); on those targets we synthesise one from the text
//!   contents' hash so the watcher at least sees *content* changes.
//!
//! The module is intentionally free of Tauri imports so the file-URL
//! parser can be host-tested. Callers (`global_paste.rs`,
//! `clipboard_watcher.rs`) do the event-emitting.

use std::path::PathBuf;

/// Errors a clipboard read can surface. Keep the variants low-churn —
/// callers typically just log and fall through to "no files".
#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("clipboard backend unavailable: {0}")]
    Unavailable(String),
    #[error("clipboard read failed: {0}")]
    ReadFailed(String),
}

/// Minimal percent-decoder good enough for `file://` URIs, which only
/// need to round-trip the reserved set (`%20`, `%2F`, …). Keeps the
/// dep list short — a full URL crate isn't worth adding for this one
/// path. Shared between the Linux and macOS clipboard back-ends.
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &s[i + 1..i + 3];
            if let Ok(byte) = u8::from_str_radix(hex, 16) {
                out.push(byte as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// Read every absolute file path currently on the OS clipboard.
///
/// Returns an empty `Vec` (not an error) when the clipboard holds
/// non-file content — text, image, nothing at all. That way the paste
/// hotkey callback can just check `is_empty()` to decide whether to
/// open the staging dialog or surface a "no files on clipboard" toast.
pub fn read_file_paths() -> Result<Vec<PathBuf>, ClipboardError> {
    #[cfg(windows)]
    return windows_impl::read_file_paths();
    #[cfg(target_os = "macos")]
    return macos_impl::read_file_paths();
    #[cfg(target_os = "linux")]
    return linux_impl::read_file_paths();
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    return Ok(Vec::new());
}

/// Monotonically-increasing token that changes whenever the
/// clipboard contents change. The watcher compares successive calls;
/// equality means "nothing happened, skip the work".
pub fn sequence_token() -> u64 {
    #[cfg(windows)]
    return windows_impl::sequence_token();
    #[cfg(target_os = "macos")]
    return macos_impl::sequence_token();
    #[cfg(target_os = "linux")]
    return linux_impl::sequence_token();
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    return 0;
}

// ---------------------------------------------------------------------
// Windows: CF_HDROP + GetClipboardSequenceNumber
// ---------------------------------------------------------------------

#[cfg(windows)]
mod windows_impl {
    use super::{ClipboardError, PathBuf};
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    use windows::Win32::Foundation::{HANDLE, HWND};
    use windows::Win32::System::DataExchange::{
        CloseClipboard, GetClipboardData, GetClipboardSequenceNumber, IsClipboardFormatAvailable,
        OpenClipboard,
    };
    use windows::Win32::System::Ole::CF_HDROP;
    use windows::Win32::UI::Shell::{DragQueryFileW, HDROP};

    pub fn sequence_token() -> u64 {
        // Session-scoped, wraps at 2^32; we upcast to u64 so the
        // watcher's saturating-compare logic stays trivial.
        unsafe { GetClipboardSequenceNumber() as u64 }
    }

    pub fn read_file_paths() -> Result<Vec<PathBuf>, ClipboardError> {
        // Fast-exit when the clipboard has no file list at all — no
        // need to OpenClipboard if CF_HDROP isn't registered.
        // SAFETY: IsClipboardFormatAvailable is a read-only probe;
        // windows 0.61 surfaces the absent-format case as Err.
        if unsafe { IsClipboardFormatAvailable(CF_HDROP.0.into()) }.is_err() {
            return Ok(Vec::new());
        }

        // Open/close must be paired via a guard; any early return
        // without CloseClipboard would starve other apps' reads.
        struct ClipGuard;
        impl Drop for ClipGuard {
            fn drop(&mut self) {
                // SAFETY: paired with the OpenClipboard below.
                unsafe {
                    let _ = CloseClipboard();
                }
            }
        }

        // SAFETY: HWND::default() == NULL means "current task's
        // clipboard" per MSDN. Returns 0 on failure (another app owns
        // the clipboard, brief race window).
        unsafe {
            if OpenClipboard(Some(HWND::default())).is_err() {
                return Err(ClipboardError::Unavailable(
                    "OpenClipboard failed — another app may hold the clipboard".into(),
                ));
            }
        }
        let _guard = ClipGuard;

        // SAFETY: we just asserted IsClipboardFormatAvailable above.
        let handle = unsafe { GetClipboardData(CF_HDROP.0.into()) };
        let handle: HANDLE = match handle {
            Ok(h) => h,
            Err(e) => {
                return Err(ClipboardError::ReadFailed(format!(
                    "GetClipboardData(CF_HDROP): {e}"
                )));
            }
        };
        let hdrop = HDROP(handle.0 as *mut _);

        // First call with index 0xFFFFFFFF asks DragQueryFileW for the
        // file count; subsequent calls pull each path's wide-char
        // buffer by index.
        // SAFETY: HDROP came straight from GetClipboardData; MSDN
        // guarantees it's valid until CloseClipboard below.
        let count = unsafe { DragQueryFileW(hdrop, u32::MAX, None) };
        if count == 0 {
            return Ok(Vec::new());
        }

        let mut out = Vec::with_capacity(count as usize);
        // Windows paths cap at 32 767 wide chars with the long-path
        // opt-in; allocate per-index, sized to what DragQueryFileW
        // reports (it returns the length needed sans NUL when `buf`
        // is None / 0-length).
        for i in 0..count {
            // SAFETY: `hdrop` valid; `None` buf asks for the length.
            let len_no_nul = unsafe { DragQueryFileW(hdrop, i, None) };
            if len_no_nul == 0 {
                continue;
            }
            let mut buf = vec![0u16; (len_no_nul as usize) + 1];
            // SAFETY: `buf` is sized len+1 to hold the trailing NUL.
            let copied = unsafe { DragQueryFileW(hdrop, i, Some(&mut buf[..])) };
            if copied == 0 {
                continue;
            }
            let os: OsString = OsStringExt::from_wide(&buf[..copied as usize]);
            let path = PathBuf::from(os);
            if !path.as_os_str().is_empty() {
                out.push(path);
            }
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------
// macOS: NSPasteboard + changeCount
// ---------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos_impl {
    use super::{ClipboardError, PathBuf, percent_decode};

    use objc2_app_kit::{NSPasteboard, NSPasteboardTypeFileURL};

    pub fn sequence_token() -> u64 {
        // `changeCount` is an NSInteger that bumps every time any app
        // writes to the general pasteboard. Session-scoped; stable
        // across our polling interval.
        // SAFETY: `generalPasteboard` returns a non-null singleton for
        // the lifetime of the app; `changeCount` is a simple getter.
        // `allow(unused_unsafe)`: newer `objc2-app-kit` marks these
        // calls safe; older versions required `unsafe`. The block
        // stays for doc-comment locality even when clippy says the
        // wrapper is redundant.
        #[allow(unused_unsafe)]
        unsafe {
            let pb = NSPasteboard::generalPasteboard();
            pb.changeCount() as u64
        }
    }

    pub fn read_file_paths() -> Result<Vec<PathBuf>, ClipboardError> {
        // SAFETY: every Obj-C call here is on the main pasteboard
        // singleton with reference-type arguments; the returned
        // NSPasteboardItem's `stringForType:` yields a retained
        // NSString we convert to `String` before drop.
        #[allow(unused_unsafe)]
        unsafe {
            let pb = NSPasteboard::generalPasteboard();
            let Some(items) = pb.pasteboardItems() else {
                return Ok(Vec::new());
            };
            let mut out = Vec::with_capacity(items.count());
            for i in 0..items.count() {
                let item = items.objectAtIndex(i);
                // `public.file-url` is the UTI the Finder copy hands
                // over. A regular text copy won't have it, so the
                // loop naturally shorts when the clipboard is text.
                let Some(ns) = item.stringForType(NSPasteboardTypeFileURL) else {
                    continue;
                };
                let s = ns.to_string();
                if let Some(p) = file_uri_to_path(&s) {
                    out.push(p);
                }
            }
            Ok(out)
        }
    }

    /// Decode a `file://localhost/…` or `file:///…` URI to a local
    /// path. macOS typically emits the 3-slash variant; the 2-slash
    /// form is legal per RFC 8089 and shows up when apps round-trip
    /// through the web share-sheet.
    fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
        let rest = uri.strip_prefix("file://")?;
        // Drop an optional `localhost` authority, keeping the leading
        // slash on the path.
        let after_auth = rest.strip_prefix("localhost").unwrap_or(rest);
        let path = if after_auth.starts_with('/') {
            after_auth.to_string()
        } else {
            format!("/{after_auth}")
        };
        let decoded = percent_decode(&path);
        if decoded.is_empty() {
            None
        } else {
            Some(PathBuf::from(decoded))
        }
    }
}

// ---------------------------------------------------------------------
// Linux: arboard text read + text/uri-list / GNOME x-special parse
// ---------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::{ClipboardError, PathBuf, percent_decode};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// `arboard` doesn't expose a portable "file list" reader, but
    /// every GTK/KDE/Qt file manager writes the selection as
    /// `text/uri-list` on the primary clipboard when the user hits
    /// Ctrl+C. GNOME additionally writes an `x-special/gnome-copied-files`
    /// blob that starts with the verb (`copy\n` / `cut\n`) followed
    /// by URIs. Both variants appear in the plain-text read; we
    /// tolerate both shapes.
    pub fn read_file_paths() -> Result<Vec<PathBuf>, ClipboardError> {
        let mut board =
            arboard::Clipboard::new().map_err(|e| ClipboardError::Unavailable(e.to_string()))?;
        let text = match board.get_text() {
            Ok(t) => t,
            Err(arboard::Error::ContentNotAvailable) => return Ok(Vec::new()),
            Err(e) => return Err(ClipboardError::ReadFailed(e.to_string())),
        };
        Ok(parse_uri_list(&text))
    }

    pub fn sequence_token() -> u64 {
        // No portable kernel-level sequence counter on Linux; the
        // watcher asks for the text and hashes it so at least
        // content changes register. Cost: ~one string alloc per
        // poll, which at 500 ms intervals is negligible.
        let mut h = DefaultHasher::new();
        let Ok(mut board) = arboard::Clipboard::new() else {
            return 0;
        };
        if let Ok(text) = board.get_text() {
            text.hash(&mut h);
            h.finish()
        } else {
            0
        }
    }

    /// Public for unit tests. Accepts either a bare `file://…` list
    /// (one per line), a GNOME `copy\nfile://…` block, or a plain
    /// absolute-path list (some apps skip the URI encoding entirely).
    /// Silently drops non-`file://` URIs so a browser-copied link
    /// doesn't spawn a paste dialog.
    pub fn parse_uri_list(text: &str) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for raw in text.lines() {
            let line = raw.trim();
            // Skip GNOME's verb prefix, blank lines, and comments.
            if line.is_empty() || line == "copy" || line == "cut" || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("file://") {
                // Percent-decode plus drop a potential authority
                // component (`file:///absolute` → `/absolute`).
                let path_part = rest.strip_prefix('/').map(|s| format!("/{s}"));
                let decoded = percent_decode(path_part.as_deref().unwrap_or(rest));
                out.push(PathBuf::from(decoded));
            } else if line.starts_with('/') {
                // Some apps (older GTK) write raw absolute paths when
                // nothing else is available. Accept them.
                out.push(PathBuf::from(line));
            }
        }
        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parses_bare_uri_list() {
            let txt = "file:///home/mike/a.bin\nfile:///home/mike/b.bin\n";
            let paths = parse_uri_list(txt);
            assert_eq!(paths.len(), 2);
            assert_eq!(paths[0], PathBuf::from("/home/mike/a.bin"));
            assert_eq!(paths[1], PathBuf::from("/home/mike/b.bin"));
        }

        #[test]
        fn parses_gnome_x_special_block() {
            let txt = "copy\nfile:///tmp/one\nfile:///tmp/two\n";
            let paths = parse_uri_list(txt);
            assert_eq!(paths.len(), 2);
            assert!(paths.iter().all(|p| p.starts_with("/tmp")));
        }

        #[test]
        fn percent_decodes_spaces() {
            let txt = "file:///home/mike/with%20space.bin\n";
            let paths = parse_uri_list(txt);
            assert_eq!(paths[0], PathBuf::from("/home/mike/with space.bin"));
        }

        #[test]
        fn drops_non_file_uris() {
            let txt = "http://example.com/a\nfile:///ok\nftp://server/b";
            let paths = parse_uri_list(txt);
            assert_eq!(paths.len(), 1);
            assert_eq!(paths[0], PathBuf::from("/ok"));
        }

        #[test]
        fn accepts_bare_absolute_paths() {
            let txt = "/opt/raw/path.bin\n/other/file.txt\n";
            let paths = parse_uri_list(txt);
            assert_eq!(paths.len(), 2);
        }

        #[test]
        fn ignores_comments_and_blank_lines() {
            let txt = "\n# a comment\ncopy\nfile:///x\n\n";
            let paths = parse_uri_list(txt);
            assert_eq!(paths.len(), 1);
        }
    }
}
