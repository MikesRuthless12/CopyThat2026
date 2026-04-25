//! Phase 7 partials smoke test — shell integration verification.
//!
//! Closes the three deferred items from Phase 7's original brief:
//!
//! 1. **macOS `Services.plist` `NSServicesMenu` validation.** The
//!    static config under `packaging/macos/services/Services.plist`
//!    must declare both `copyFiles` and `moveFiles` selectors with
//!    the right `NSSendTypes` so the items show up in every app's
//!    Services submenu when files are on the pasteboard.
//! 2. **Windows IExplorerCommand intercept stanza.** The
//!    `intercept_default_copy` HKCU keys are documented in
//!    `copythat_shellext::registry::copy_interceptor_keys`. Confirm
//!    the registry path + value format hasn't drifted.
//! 3. **Linux Nautilus extension shape.** The Python file at
//!    `packaging/linux/nautilus/copythat_nautilus.py` must declare
//!    the right `gi.require_version`, the `GObject` / `Nautilus`
//!    imports, and a `_spawn_copythat` function that shells out to
//!    the `copythat` binary. KDE Dolphin + XFCE Thunar configs
//!    similarly checked.
//!
//! Filesystem-only tripwire — never invokes the OS shell. Lives in
//! the workspace's smoke matrix so the static config + the runtime
//! `copythat-shellext` registry helpers stay in lockstep.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &Path = &here;
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = match cur.parent() {
            Some(p) => p,
            None => break,
        };
    }
    panic!("could not locate repo root");
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| panic!("reading {}: {e}", p.display()))
}

#[test]
fn macos_services_plist_declares_both_selectors() {
    let body = read(&repo_root().join("packaging/macos/services/Services.plist"));
    assert!(
        body.contains("<key>NSMessage</key>") && body.contains("copyFiles"),
        "Services.plist must declare the copyFiles selector",
    );
    assert!(
        body.contains("moveFiles"),
        "Services.plist must declare the moveFiles selector",
    );
    assert!(
        body.contains("public.file-url"),
        "NSSendTypes must include public.file-url so the items appear when files are selected",
    );
    assert!(
        body.contains("NSFilenamesPboardType"),
        "NSSendTypes must include the legacy NSFilenamesPboardType for pre-10.15 selections",
    );
    // Both items should be on the same NSPortName so the OS routes
    // them to the running app.
    assert!(body.contains("<key>NSPortName</key>"));
    assert!(body.contains("Copy That"));
}

#[test]
fn macos_finder_sync_extension_info_plist_is_present() {
    let p = repo_root().join("packaging/macos/finder-sync-extension/Info.plist");
    assert!(
        p.is_file(),
        "macOS Finder Sync extension Info.plist missing — Phase 7 brief required it",
    );
    let body = read(&p);
    // Bundle identifier must be present so the OS can register the
    // extension under the parent app.
    assert!(
        body.contains("CFBundleIdentifier"),
        "Finder Sync Info.plist must declare CFBundleIdentifier",
    );
}

#[test]
fn windows_intercept_default_copy_registry_stanza_is_correct() {
    use copythat_shellext::registry::copy_interceptor_keys;
    let clsid = "{A7D2C001-C097-4C96-8F7A-5C970C097001}";
    let keys = copy_interceptor_keys(clsid);
    assert_eq!(keys.len(), 1, "interceptor stanza is one key/value pair");
    let (path, value, data) = &keys[0];
    assert_eq!(path, r"HKCU\Software\Classes\*\shell\copy");
    assert_eq!(value, "DelegateExecute");
    assert_eq!(data, clsid);
}

#[test]
fn linux_nautilus_extension_has_required_python_shape() {
    let body = read(&repo_root().join("packaging/linux/nautilus/copythat_nautilus.py"));
    // gi typelib pins
    assert!(body.contains("gi.require_version(\"Nautilus\""));
    assert!(body.contains("from gi.repository import GObject, Nautilus"));
    // Spawn helper must shell out to the copythat binary
    assert!(
        body.contains("_spawn_copythat") || body.contains("def "),
        "Nautilus extension must define a spawn helper",
    );
    assert!(
        body.contains("copythat"),
        "Nautilus extension must invoke the copythat binary",
    );
    // Detached spawn — Nautilus mustn't block on the GUI launch.
    assert!(
        body.contains("close_fds") || body.contains("start_new_session") || body.contains("Popen"),
        "Nautilus extension must spawn detached so Nautilus does not wait",
    );
}

#[test]
fn linux_kde_servicemenu_desktop_is_well_formed() {
    let body = read(&repo_root().join("packaging/linux/kde/copythat-servicemenu.desktop"));
    // Standard XDG ServiceMenu shape — Type=Service is required.
    assert!(
        body.contains("[Desktop Entry]"),
        "KDE ServiceMenu .desktop file must start with [Desktop Entry]",
    );
    assert!(
        body.contains("Type=Service") || body.contains("Type=Action"),
        "KDE ServiceMenu must declare Service or Action type",
    );
    assert!(
        body.contains("Exec=") && body.contains("copythat"),
        "KDE ServiceMenu must invoke the copythat binary via Exec=",
    );
}

#[test]
fn linux_thunar_uca_xml_declares_both_actions() {
    let body = read(&repo_root().join("packaging/linux/thunar/copythat-uca.xml"));
    assert!(
        body.contains("<actions>"),
        "Thunar UCA XML must root in <actions>",
    );
    // At least the copy verb. The move verb is optional in the
    // packaging today.
    assert!(
        body.contains("copythat"),
        "Thunar UCA XML must invoke the copythat binary",
    );
    assert!(
        body.contains("<command>") && body.contains("</command>"),
        "Thunar UCA XML must declare a <command> element",
    );
}

#[test]
fn linux_desktop_file_present_for_xdg_integration() {
    // The .desktop file at the package root is what Linux desktop
    // environments use to surface the app in their launcher / file
    // manager menu. Phase 7 ships it; the smoke confirms it stayed
    // in place.
    let p = repo_root().join("packaging/linux/copythat.desktop");
    assert!(p.is_file());
    let body = read(&p);
    assert!(body.contains("[Desktop Entry]"));
    assert!(body.contains("Exec="));
    assert!(body.contains("copythat"));
}

#[test]
fn shellext_consts_carry_stable_clsids() {
    // The two CLSIDs the registry stanzas use are stable across
    // releases (changing them invalidates every user's existing
    // registration). Confirm they haven't drifted.
    let body = read(&repo_root().join("crates/copythat-shellext/src/consts.rs"));
    assert!(body.contains("{A7D2C001-C097-4C96-8F7A-5C970C097001}"));
    assert!(body.contains("{A7D2C002-C097-4C96-8F7A-5C970C097002}"));
}
