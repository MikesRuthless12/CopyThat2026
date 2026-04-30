//! Phase 45.2 — `VolumeProbe` adapter for `copythat-core::QueueRegistry`.
//!
//! [`PlatformVolumeProbe`] is the production [`copythat_core::VolumeProbe`]
//! impl: `volume_id` delegates to the existing
//! [`crate::helpers::volume_id`] helper (Windows VolumeSerialNumber /
//! Unix `st_dev`); `drive_label` returns a short, user-friendly name
//! (`"C:"`, `"D:"`) on Windows and `None` elsewhere — the registry
//! falls back to a generic `"queue N"` name when the probe yields no
//! label.
//!
//! Stateless + `Clone + Copy`; one instance suffices for the lifetime
//! of the process.

use std::path::Path;

use copythat_core::VolumeProbe;

/// Production [`VolumeProbe`] backed by [`crate::helpers::volume_id`].
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformVolumeProbe;

impl VolumeProbe for PlatformVolumeProbe {
    fn volume_id(&self, path: &Path) -> Option<u64> {
        crate::helpers::volume_id(path)
    }

    fn drive_label(&self, path: &Path) -> Option<String> {
        drive_label_impl(path)
    }
}

#[cfg(target_os = "windows")]
fn drive_label_impl(path: &Path) -> Option<String> {
    // Walk components looking for the first `<letter>:` prefix. Works
    // for both `C:\foo\bar` and `\\?\C:\foo\bar` (the long-path form).
    // Returns `"C:"` (no trailing slash) — matches what the user sees
    // in the address bar and keeps tab labels short.
    use std::path::Component;
    for comp in path.components() {
        if let Component::Prefix(prefix) = comp {
            let raw = prefix.as_os_str().to_string_lossy();
            // Prefix kinds we care about: `Disk("C:")` and
            // `VerbatimDisk("C:")`. Both surface their letter as the
            // first ASCII alphabetic char in the prefix string.
            for (i, ch) in raw.char_indices() {
                if ch.is_ascii_alphabetic() {
                    let after = &raw[i..];
                    let mut iter = after.chars();
                    if let (Some(letter), Some(colon)) = (iter.next(), iter.next())
                        && letter.is_ascii_alphabetic()
                        && colon == ':'
                    {
                        let mut s = String::with_capacity(2);
                        s.push(letter.to_ascii_uppercase());
                        s.push(':');
                        return Some(s);
                    }
                }
            }
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn drive_label_impl(_path: &Path) -> Option<String> {
    // Phase 45.2 — no portable mount-point label on Unix without
    // pulling in mount-table parsing. Returning `None` lets the
    // registry fall back to `"queue N"`; a future phase can mine
    // `/proc/self/mountinfo` or `getmntinfo` if better labels become
    // important on Linux/macOS.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_is_clone_and_copy() {
        let p = PlatformVolumeProbe;
        let _q = p;
        let _r = p;
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_drive_label_extracts_letter() {
        use std::path::PathBuf;
        let probe = PlatformVolumeProbe;
        assert_eq!(
            probe.drive_label(&PathBuf::from(r"C:\Users\me")),
            Some("C:".to_string()),
        );
        assert_eq!(
            probe.drive_label(&PathBuf::from(r"\\?\D:\backup")),
            Some("D:".to_string()),
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_drive_label_is_none() {
        use std::path::PathBuf;
        let probe = PlatformVolumeProbe;
        assert!(probe.drive_label(&PathBuf::from("/home/me")).is_none());
        assert!(probe.drive_label(&PathBuf::from("/Volumes/Backup")).is_none());
    }
}
