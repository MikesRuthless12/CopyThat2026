//! Constants mirroring `copythat-core`'s engine bounds.
//!
//! The settings crate intentionally doesn't depend on `copythat-core`
//! so it stays a lean preference + IO layer. Instead we mirror the
//! buffer-size window here. If the engine ever relaxes its bounds,
//! update both in one commit — the `copythat-core` change + this
//! constant + any `effective_buffer_size` callers.

/// Mirror of `copythat_core::options::DEFAULT_BUFFER_SIZE` (1 MiB).
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024;

/// Mirror of `copythat_core::options::MIN_BUFFER_SIZE` (64 KiB).
pub const MIN_BUFFER_SIZE: usize = 64 * 1024;

/// Mirror of `copythat_core::options::MAX_BUFFER_SIZE` (16 MiB).
pub const MAX_BUFFER_SIZE: usize = 16 * 1024 * 1024;

/// Default system-wide hotkey for "paste files via Copy That".
/// Tauri's `global-shortcut` plugin resolves `CmdOrCtrl` to Cmd on
/// macOS and Ctrl on Windows / Linux, so one spelling covers all three
/// hosts. `Shift` avoids colliding with the platform-native paste
/// (Cmd+V / Ctrl+V) so users keep normal text-paste semantics.
pub const DEFAULT_PASTE_SHORTCUT: &str = "CmdOrCtrl+Shift+V";
