// Copy That 2026 — Finder Sync Extension.
//
// Registers "Copy with Copy That" and "Move with Copy That" in the
// Finder context menu (right-click on a selection) and in the Finder
// toolbar. Every action spawns the app binary with the same argv the
// Phase 7a CLI parser expects:
//
//     copythat --enqueue <copy|move> -- <paths…>
//
// The extension lives in a .appex bundle embedded inside the main
// `Copy That 2026.app` (see packaging/macos/scripts/bundle-appex.sh).
// macOS loads the bundle in a sandboxed XPC process; the spawned
// `copythat` child runs in the user's session, and the Phase 7a
// single-instance plumbing hands its argv to the live app instance.

import Cocoa
import FinderSync

// MARK: - Constants

/// Where the Finder Sync extension looks for the app binary.
/// Packaging symlinks `/usr/local/bin/copythat` → the app's
/// Contents/MacOS/copythat binary on install (also via Homebrew cask).
/// Falls back to the PATH resolution inside Process via /usr/bin/env.
private let copyThatBinaryCandidates: [String] = [
    "/usr/local/bin/copythat",
    "/opt/homebrew/bin/copythat",
    "/Applications/Copy That 2026.app/Contents/MacOS/copythat",
]

/// Root the sync watches. `/` covers every local volume; macOS scopes
/// the extension to the user's own session regardless.
private let syncRootURL = URL(fileURLWithPath: "/")

// MARK: - Principal class

@objc(CopyThatFinderSync)
final class CopyThatFinderSync: FIFinderSync {
    override init() {
        super.init()

        // Tell Finder we want to hear about every local directory —
        // otherwise our menu builder is never called for items
        // outside a registered root. The badge registration is
        // intentionally empty; we add no badges in 0.x.
        FIFinderSyncController.default().directoryURLs = [syncRootURL]
    }

    // MARK: - Menu builder

    override func menu(for menuKind: FIMenuKind) -> NSMenu {
        let menu = NSMenu(title: "Copy That")
        switch menuKind {
        case .contextualMenuForItems, .contextualMenuForContainer, .toolbarItemMenu:
            let copyItem = NSMenuItem(
                title: "Copy with Copy That",
                action: #selector(copyAction(_:)),
                keyEquivalent: ""
            )
            copyItem.image = NSImage(
                systemSymbolName: "doc.on.doc",
                accessibilityDescription: "Copy"
            )

            let moveItem = NSMenuItem(
                title: "Move with Copy That",
                action: #selector(moveAction(_:)),
                keyEquivalent: ""
            )
            moveItem.image = NSImage(
                systemSymbolName: "arrow.right.doc.on.clipboard",
                accessibilityDescription: "Move"
            )

            menu.addItem(copyItem)
            menu.addItem(moveItem)

        case .contextualMenuForSidebar:
            // Sidebar right-click doesn't surface our entries today —
            // users treat sidebar items as mount-points, not transfer
            // sources. Return an empty menu so macOS's default menu
            // still shows.
            break
        @unknown default:
            break
        }
        return menu
    }

    // MARK: - Actions

    @objc func copyAction(_ sender: AnyObject?) {
        spawn(verb: "copy")
    }

    @objc func moveAction(_ sender: AnyObject?) {
        spawn(verb: "move")
    }

    // MARK: - Private

    /// Resolve the Copy That binary and launch it detached with the
    /// current selection.
    private func spawn(verb: String) {
        let controller = FIFinderSyncController.default()
        guard let urls = controller.selectedItemURLs(), !urls.isEmpty else {
            NSLog("copythat-finder-sync: no selection; ignoring \(verb)")
            return
        }
        guard let binary = resolveBinary() else {
            NSLog("copythat-finder-sync: no copythat binary on PATH or in known locations")
            return
        }

        let process = Process()
        process.executableURL = URL(fileURLWithPath: binary)
        var args = ["--enqueue", verb, "--"]
        args.append(contentsOf: urls.map { $0.path })
        process.arguments = args
        process.standardInput = FileHandle.nullDevice
        process.standardOutput = FileHandle.nullDevice
        process.standardError = FileHandle.nullDevice

        do {
            try process.run()
        } catch {
            NSLog("copythat-finder-sync: failed to spawn \(binary): \(error)")
        }
    }

    /// Pick the first existing binary from the candidate list.
    /// Returns nil if none are found; callers log and abort.
    private func resolveBinary() -> String? {
        let fm = FileManager.default
        for candidate in copyThatBinaryCandidates {
            if fm.isExecutableFile(atPath: candidate) {
                return candidate
            }
        }
        return nil
    }
}
