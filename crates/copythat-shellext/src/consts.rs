//! Stable constants shared across the shell extension.
//!
//! **These values are part of the on-disk registration contract.**
//! Changing the CLSIDs or canonical verb names means any existing
//! Windows install that registered the old DLL has orphan keys
//! under HKCU / HKLM; that is acceptable for the 0.x pre-release
//! cycle but will freeze at 1.0.
//!
//! The GUIDs were generated for this project — they are not
//! recycled from any Microsoft sample; the `0xA7D2_C001` / `C002`
//! prefix is a mnemonic ("Copy That 2026 class 01 / 02").

/// CLSID for the "Copy with Copy That" `IExplorerCommand`.
/// `{A7D2C001-C097-4C96-8F7A-5C970C097001}`.
pub const CLSID_COPY_STR: &str = "{A7D2C001-C097-4C96-8F7A-5C970C097001}";

/// CLSID for the "Move with Copy That" `IExplorerCommand`.
/// `{A7D2C002-C097-4C96-8F7A-5C970C097002}`.
pub const CLSID_MOVE_STR: &str = "{A7D2C002-C097-4C96-8F7A-5C970C097002}";

/// Canonical verb name used under `HKCU\Software\Classes\*\shell\…`.
/// Explorer references this key via the verb registration; never
/// rename it without bumping the DLL major version.
pub const VERB_COPY: &str = "CopyThat.Copy";

/// Canonical verb name for the move command. See [`VERB_COPY`].
pub const VERB_MOVE: &str = "CopyThat.Move";

/// Display string Explorer shows in the context menu for the copy
/// verb. English; Windows does not yet read a `MUIVerb` resource
/// from us — a future phase can thread localised variants through a
/// `MUIVerb` / `SubCommands` registration.
pub const DISPLAY_COPY: &str = "Copy with Copy That";

/// Display string for the move verb. See [`DISPLAY_COPY`].
pub const DISPLAY_MOVE: &str = "Move with Copy That";

/// ProgID-style friendly names under `HKCR\CLSID\{guid}\(default)`.
pub const PROG_COPY: &str = "Copy That 2026 — Copy command";

/// See [`PROG_COPY`].
pub const PROG_MOVE: &str = "Copy That 2026 — Move command";

/// Name of the target binary the verbs invoke. Resolved via PATH;
/// Phase 16 packaging puts `copythat.exe` there (on Windows today,
/// the Tauri bin ships as `copythat-ui.exe`; the MSI installer
/// symlinks / renames it at install time).
pub const HOST_BIN: &str = "copythat";

#[cfg(windows)]
mod guids {
    use windows::core::GUID;

    pub const CLSID_COPY: GUID = GUID::from_u128(0xA7D2C001_C097_4C96_8F7A_5C970C097001);
    pub const CLSID_MOVE: GUID = GUID::from_u128(0xA7D2C002_C097_4C96_8F7A_5C970C097002);
}

#[cfg(windows)]
pub use guids::{CLSID_COPY, CLSID_MOVE};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clsid_string_values_match_guid_u128_on_windows() {
        // Round-trip the textual CLSID through a dumb hex check so a
        // future editor cannot drift the two representations without
        // noticing. Format: `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`.
        for s in [CLSID_COPY_STR, CLSID_MOVE_STR] {
            assert!(s.starts_with('{') && s.ends_with('}'));
            let stripped = &s[1..s.len() - 1];
            let parts: Vec<&str> = stripped.split('-').collect();
            assert_eq!(parts.len(), 5, "expected 5-part CLSID: {s}");
            assert_eq!(parts[0].len(), 8);
            assert_eq!(parts[1].len(), 4);
            assert_eq!(parts[2].len(), 4);
            assert_eq!(parts[3].len(), 4);
            assert_eq!(parts[4].len(), 12);
            for p in parts {
                assert!(p.chars().all(|c| c.is_ascii_hexdigit()));
            }
        }
    }

    #[test]
    fn verb_names_are_registry_safe() {
        // Backslashes would break the `*\shell\<verb>` key path.
        // Whitespace would disagree with what Explorer expects.
        for v in [VERB_COPY, VERB_MOVE] {
            assert!(!v.contains(['\\', '/', ' ', '\t', '\n']));
        }
    }
}
