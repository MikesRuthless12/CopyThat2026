# Packaging manifests

Per-channel manifests for Copy That 2026's free-first distribution path.
Every channel here costs $0 to publish.

| Folder                           | Channel                   | Target                       |
| -------------------------------- | ------------------------- | ---------------------------- |
| `windows/winget/`                | winget-pkgs (primary)     | Windows MSI                  |
| `windows/chocolatey/`            | Chocolatey community repo | Windows MSI (secondary)      |
| `macos/homebrew-cask/`           | Homebrew cask             | macOS `.app` + `.dmg`        |
| `macos/finder-sync-extension/`   | Finder Sync `.appex`      | Phase 7 shell integration    |
| `macos/services/`                | `NSServicesMenu` plist    | Phase 7 shell integration    |
| `macos/scripts/`                 | Build helpers             | Phase 7 / 16                 |
| `linux/flatpak/`                 | Flathub                   | AppImage / sandboxed Linux   |
| `linux/aur/`                     | Arch User Repository      | `.deb` → Arch repackage      |
| `linux/copythat.desktop`         | Freedesktop               | Phase 7 shell integration    |
| `linux/kde/`, `nautilus/`, `thunar/` | Per-file-manager hooks | Phase 7 shell integration    |

See [`docs/SIGNING_UPGRADE.md`](../docs/SIGNING_UPGRADE.md) for the paid
signing upgrade paths (Azure Trusted Signing on Windows, Apple Notary
on macOS) — keeping those as opt-in keeps v1.0 at $0 to ship.
