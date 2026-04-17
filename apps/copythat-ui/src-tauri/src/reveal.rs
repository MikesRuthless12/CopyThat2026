//! "Show in folder" — open the platform file manager at the
//! containing folder with the item pre-selected where possible.
//!
//! - Windows: `explorer.exe /select,<path>`
//! - macOS:   `open -R <path>` (reveals + selects)
//! - Linux:   `xdg-open <parent>` (best-effort; desktop environments
//!   differ on whether they can select a specific item from a URI —
//!   we open the enclosing folder either way).

use std::path::Path;
use std::process::Command;

pub fn reveal(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("path does not exist: {}", path.display()));
    }
    platform_reveal(path)
}

#[cfg(target_os = "windows")]
fn platform_reveal(path: &Path) -> Result<(), String> {
    use std::os::windows::process::CommandExt;

    // Explorer parses `/select,<path>` as a single argument split on
    // the comma. `raw_arg` avoids Rust's automatic quoting which would
    // break this convention.
    let arg = format!("/select,\"{}\"", path.display());
    Command::new("explorer.exe")
        .raw_arg(&arg)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
fn platform_reveal(path: &Path) -> Result<(), String> {
    Command::new("open")
        .arg("-R")
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_reveal(path: &Path) -> Result<(), String> {
    let target = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    Command::new("xdg-open")
        .arg(target)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}
