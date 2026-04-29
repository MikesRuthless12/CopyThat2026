//! Windows COM in-proc shell extension for Copy That v0.19.84.
//!
//! Adds "Copy with Copy That" and "Move with Copy That" entries to
//! the Windows Explorer right-click menu for files and folders. Each
//! verb invokes the `copythat` app binary with
//! `--enqueue <verb> <paths…>`; the Phase 7a single-instance plumbing
//! routes those paths into the running app's queue.
//!
//! Registration is per-user by default (HKCU) so `regsvr32` does not
//! need admin. System-wide (HKLM) install is supported via a flag in
//! the registration helper, for the Phase 16 MSI packaging step.
//!
//! Non-Windows targets compile this crate as an empty library so
//! `cargo build --all` succeeds on CI macOS / Linux runners without
//! workspace-membership gymnastics. All the interesting code lives
//! under `#[cfg(windows)]`.
//!
//! # Layout
//!
//! - [`consts`] — stable CLSID + verb-name constants shared across
//!   the module tree.
//! - [`spawn`] — argv composition for the detached `copythat`
//!   subprocess call. Pure-Rust, testable on every host.
//! - [`registry`] — per-user + system-wide registration helpers,
//!   plus the opt-in default-copy-verb interceptor (TeraCopy mode).
//!   The registry-path builders are pure-Rust and host-tested.
//! - [`com`] — `CopyCommand` / `MoveCommand` `IExplorerCommand`
//!   implementations (Windows only).
//! - [`factory`] — one `ClassFactory` per command (Windows only).
//! - DLL exports (`DllGetClassObject` / `DllCanUnloadNow` /
//!   `DllRegisterServer` / `DllUnregisterServer`) live at the bottom
//!   of this file, also Windows only.

pub mod consts;
pub mod spawn;

// Registry-key composition is host-independent — we test it on every
// platform even though the actual HKCU writes only land on Windows.
pub mod registry;

#[cfg(windows)]
pub mod com;

#[cfg(windows)]
pub mod factory;

#[cfg(windows)]
pub mod dll;
