// Copy That v1.0.0 — macOS Services menu handler.
//
// Registers two entries in every app's `<AppName> ▸ Services` submenu:
// "Copy with Copy That" and "Move with Copy That". They show up
// whenever a selection on the pasteboard is a file path
// (NSFilenamesPboardType or public.file-url).
//
// Unlike the Finder Sync Extension (Phase 7c-a), Services lives in
// the *main app bundle* — Phase 16 packaging merges the NSServices
// array from `Services.plist` into the app's Info.plist, and this
// Swift source compiles into the app's Contents/MacOS binary
// alongside the Rust Tauri core via a tiny Objective-C runtime
// shim at app launch.
//
// Phase 7c scope: ship the Swift source + plist fragment, smoke-
// test that both parse cleanly. Full wiring into the Tauri
// `copythat-ui` bundle is a Phase 16 packaging task (Swift + Rust
// co-residency needs the `lipo`/ld rigging that the packaging
// step owns).

import Cocoa

/// NSObject subclass that macOS invokes for each Services menu entry.
/// The two selectors `copyFiles:userData:error:` and
/// `moveFiles:userData:error:` are the NSMessage keys declared in
/// `Services.plist`.
@objc(CopyThatServiceHandler)
public final class CopyThatServiceHandler: NSObject {

    /// Known locations of the app binary, in priority order. Matches
    /// `CopyThatFinderSync.swift` — the two handlers should resolve
    /// to the same installed binary so argv flows are identical.
    private static let binaryCandidates: [String] = [
        "/usr/local/bin/copythat",
        "/opt/homebrew/bin/copythat",
        "/Applications/Copy That v1.0.0.app/Contents/MacOS/copythat",
    ]

    /// Register this handler with NSApplication so the selectors are
    /// visible to the Services system. Call this from the Rust side
    /// during app startup.
    @objc public static func registerWithApplication() {
        NSApp.servicesProvider = CopyThatServiceHandler()
    }

    // MARK: - Service entry points

    /// `NSMessage = copyFiles`
    @objc public func copyFiles(
        _ pboard: NSPasteboard,
        userData: String,
        error: AutoreleasingUnsafeMutablePointer<NSString>
    ) {
        enqueue(verb: "copy", pboard: pboard, error: error)
    }

    /// `NSMessage = moveFiles`
    @objc public func moveFiles(
        _ pboard: NSPasteboard,
        userData: String,
        error: AutoreleasingUnsafeMutablePointer<NSString>
    ) {
        enqueue(verb: "move", pboard: pboard, error: error)
    }

    // MARK: - Private

    private func enqueue(
        verb: String,
        pboard: NSPasteboard,
        error: AutoreleasingUnsafeMutablePointer<NSString>
    ) {
        let paths = collectPaths(from: pboard)
        guard !paths.isEmpty else {
            error.pointee = "Copy That services handler: no file paths on the pasteboard" as NSString
            return
        }
        guard let binary = resolveBinary() else {
            error.pointee = "Copy That services handler: copythat binary not found" as NSString
            return
        }

        let process = Process()
        process.executableURL = URL(fileURLWithPath: binary)
        var args = ["--enqueue", verb, "--"]
        args.append(contentsOf: paths)
        process.arguments = args
        process.standardInput = FileHandle.nullDevice
        process.standardOutput = FileHandle.nullDevice
        process.standardError = FileHandle.nullDevice

        do {
            try process.run()
        } catch let spawnError {
            error.pointee = "Copy That services handler: spawn failed: \(spawnError)" as NSString
        }
    }

    /// Read file paths from the pasteboard, preferring the modern
    /// `public.file-url` type. Falls back to the deprecated-but-still-
    /// alive NSFilenamesPboardType when a caller passes paths that
    /// way (older apps often do).
    private func collectPaths(from pboard: NSPasteboard) -> [String] {
        if let urls = pboard.readObjects(forClasses: [NSURL.self], options: nil) as? [URL],
           !urls.isEmpty
        {
            return urls.map { $0.path }
        }
        if let filenames = pboard.propertyList(forType: .fileURL) as? [String] {
            return filenames
        }
        if let fallbackFilenames = pboard.propertyList(
            forType: NSPasteboard.PasteboardType(rawValue: "NSFilenamesPboardType")
        ) as? [String] {
            return fallbackFilenames
        }
        return []
    }

    private func resolveBinary() -> String? {
        let fm = FileManager.default
        for candidate in Self.binaryCandidates {
            if fm.isExecutableFile(atPath: candidate) {
                return candidate
            }
        }
        return nil
    }
}
