//! `copythat-helper` — privilege-separated elevated worker.
//!
//! Phase 17d's load-bearing piece. The main `copythat-ui` process
//! NEVER runs elevated; when it needs an operation that requires
//! Administrator / root privileges it spawns this helper through the
//! OS-native consent flow (UAC on Windows, `sudo` / `polkit` on
//! Unix) and speaks JSON-RPC to it over a per-launch random-named
//! pipe (Windows) or Unix socket. The helper exits as soon as the
//! caller closes the pipe or sends `Request::Shutdown`.
//!
//! The Phase 19b VSS helper (`copythat-helper-vss`) is the narrow
//! precedent. This Phase 17d helper is the *general* one — it
//! covers every workstream that needs elevation across the
//! workspace except VSS itself (which has its own helper because
//! Windows VSS pulls heavyweight COM dependencies the rest of the
//! helper has no need for).
//!
//! ## Capabilities the helper exposes
//!
//! - **`ElevatedRetry { src, dst, kind }`** — retry a per-file copy
//!   that the unprivileged engine surface failed on with
//!   `PermissionDenied`. Closes the gap Phase 8's `retry_elevated`
//!   stub left open. Path arguments pass through Phase 17a's
//!   lexical safety bar before any FS call.
//! - **`InstallShellExtension { kind }`** — write to system
//!   directories that need elevation: HKLM ProgID keys on Windows,
//!   `/Library/PreferencePanes/` on macOS, `/usr/share/nautilus-
//!   python/extensions/` on Linux. Caller-supplied policy decides
//!   per-OS scope.
//! - **`UninstallShellExtension { kind }`** — symmetric undo.
//! - **`HardwareErase { device }`** — NVMe Sanitize / OPAL Crypto
//!   Erase / ATA Secure Erase. Phase 4's `Nist80088Purge` stub
//!   surfaced "Phase 17 will wire this"; this is the surface. The
//!   actual ioctls land in Phase 44 — the helper here returns
//!   `Response::HardwareEraseUnavailable` so the main UI can offer
//!   the user a clear path forward without the helper silently
//!   pretending to succeed.
//!
//! ## Threat model
//!
//! - **The pipe / socket name is per-launch random.** 256 bits
//!   from `getrandom`; reading it requires racing the kernel before
//!   the helper opens it (the main process opens the listener
//!   *first*, then spawns the helper with the name on the command
//!   line). A malicious local user could only attack the surface
//!   between consent and connection — and the consent flow itself
//!   is the moat.
//! - **Every IPC argument passes through Phase 17a.** Path-typed
//!   fields run through `validate_path_no_traversal`. A traversal-
//!   laden request rejects with `Response::PathRejected` carrying
//!   the same `err-path-escape` Fluent key the engine uses.
//! - **Capability check before action.** Before each request, the
//!   helper checks `Capability::is_allowed_for(request)` — a
//!   coarse allowlist that ensures e.g. a `HardwareErase` request
//!   never accidentally accepts a target outside the user's
//!   chosen drive.
//! - **The helper runs only as long as it has work.** EOF on the
//!   request pipe terminates the loop; an explicit `Shutdown`
//!   request is accepted but optional. Shadow shell-extension
//!   installs (think: registry entries the helper added but the
//!   main process never confirmed) get a best-effort cleanup
//!   pass on EOF.
//!
//! ## What this crate does NOT carry
//!
//! - The actual platform-specific elevation flow (`runas` /
//!   `pkexec` / `osascript -e "do shell script ... with
//!   administrator privileges"`). That's the **caller's**
//!   responsibility — `copythat-ui` builds the spawn command
//!   appropriate to its platform, opens the pipe, and execs the
//!   helper. Phase 17d ships the helper + protocol; the caller
//!   ships the spawner. (Phase 19b's `copythat-snapshot::vss`
//!   spawn helper is the existing pattern.)
//! - VSS — `copythat-helper-vss` already handles that.
//! - Hardware secure-erase ioctls — Phase 44.
//!
//! Both deliberate scope cuts are documented inline at every place
//! the helper would otherwise need them.

#![forbid(unsafe_code)]

pub mod capability;
pub mod handler;
pub mod rpc;
pub mod transport;

pub use capability::{Capability, CapabilityError};
pub use handler::handle_request;
pub use rpc::{
    HelperError, Request, Response, ShellExtensionKind, generate_pipe_name, parse_pipe_name,
};
pub use transport::TransportError;
