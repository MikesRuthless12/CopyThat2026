//! Per-OS snapshot backend discriminator.

/// Which filesystem-snapshot primitive a handle was minted from.
///
/// `#[non_exhaustive]` so a future phase can add APFS-without-tmutil
/// (Finder-backed snapshot API) or ReFS integrity streams without
/// forcing a semver break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SnapshotKind {
    /// Windows Volume Shadow Copy Service. Requires Administrator
    /// privilege — the main process shells to `copythat-helper-vss.exe`
    /// via `ShellExecute("runas", ...)` and talks to it over JSON-RPC.
    Vss,
    /// ZFS. `zfs snapshot <dataset>@copythat-<uuid>`.
    Zfs,
    /// Btrfs. `btrfs subvolume snapshot -r <subvol> <snap-path>`.
    Btrfs,
    /// macOS APFS. `tmutil localsnapshot` + `mount_apfs -o nobrowse`.
    Apfs,
}

impl SnapshotKind {
    /// Stable wire string that round-trips to / from this enum in IPC
    /// payloads. Matches the kebab-case convention the rest of the
    /// workspace uses for string-tagged enums.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vss => "vss",
            Self::Zfs => "zfs",
            Self::Btrfs => "btrfs",
            Self::Apfs => "apfs",
        }
    }

    /// Human-readable short label the UI renders inside the row badge.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Vss => "VSS",
            Self::Zfs => "ZFS",
            Self::Btrfs => "Btrfs",
            Self::Apfs => "APFS",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_strings_are_stable() {
        assert_eq!(SnapshotKind::Vss.as_str(), "vss");
        assert_eq!(SnapshotKind::Zfs.as_str(), "zfs");
        assert_eq!(SnapshotKind::Btrfs.as_str(), "btrfs");
        assert_eq!(SnapshotKind::Apfs.as_str(), "apfs");
    }
}
