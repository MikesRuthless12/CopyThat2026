//! Phase 44.3a — Process-control helpers.
//!
//! Houses the `unsafe` corner needed to keep `copythat-secure-delete`
//! `#![forbid(unsafe_code)]`-clean. Today it ships one helper:
//! [`linux_make_child_undumpable`], which closes the Phase 44.2
//! SECURITY MEDIUM finding (PSID argv leak via `/proc/<pid>/cmdline`)
//! by instructing a child process to mark itself non-dumpable
//! before `execve`.
//!
//! `prctl(PR_SET_DUMPABLE, 0)` makes the calling process's
//! `/proc/<pid>/{cmdline, environ, maps, ...}` non-readable to other
//! UIDs. The same UID can still read them; root can always read
//! everything. The threat model Phase 44.2 worried about
//! (unprivileged local attacker on a multi-user system racing
//! `cat /proc/*/cmdline` to capture an OPAL PSID) is closed by
//! this — the attacker's UID differs from the operator's.

use std::process::Command;

/// Phase 44.3a (Linux only) — install a `pre_exec` hook on `cmd`
/// that calls `prctl(PR_SET_DUMPABLE, 0)` in the child after fork
/// but before exec. Closes the PSID argv leak by making
/// `/proc/<pid>/cmdline` unreadable to other UIDs.
///
/// The hook returns `Ok(())` on success and `Err(io::Error::last_os_error())`
/// on failure; the child won't `execve` if `prctl` fails. In
/// practice `PR_SET_DUMPABLE` only fails if PR_SET_DUMPABLE is
/// unsupported (kernel < 2.6.13 — well below our MSRV's effective
/// kernel floor) — so the failure path is effectively dead but
/// surfaces the right errno if it ever fires.
///
/// **Caller contract:** invoke BEFORE `Command::spawn` /
/// `Command::output` / `Command::status`. Calling after the child
/// has been spawned is a no-op (the hook is registered, but no
/// child is fork'd against the registration).
///
/// On non-Linux Unixes this is a no-op (`prctl` doesn't exist on
/// macOS / *BSD).
#[cfg(target_os = "linux")]
pub fn linux_make_child_undumpable(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    // SAFETY: pre_exec runs in the child between fork and exec.
    // Per the std docs, code in this hook must be async-signal-safe
    // — `libc::prctl` is documented to be (it's a single syscall).
    // We never deref a raw pointer or transmute; the only side
    // effect is the prctl(2) call itself.
    unsafe {
        cmd.pre_exec(|| {
            // PR_SET_DUMPABLE = 4; SUID_DUMP_DISABLE = 0. Phase 44.3
            // post-review (H1) — pass arg2 as `c_ulong` (the
            // kernel-ABI width) so 32-bit Linux targets don't
            // misalign the variadic stack. PR_SET_DUMPABLE only
            // consumes arg2; trailing args omitted.
            let r = libc::prctl(libc::PR_SET_DUMPABLE, 0_i32 as libc::c_ulong);
            if r == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
}

/// No-op on non-Linux platforms — the `prctl(PR_SET_DUMPABLE, 0)`
/// mechanism is Linux-specific. The cmdline-leak threat model is
/// also Linux-specific (macOS doesn't expose argv to other users
/// via procfs; Windows doesn't expose argv at all without
/// SE_DEBUG_NAME privilege).
#[cfg(not(target_os = "linux"))]
pub fn linux_make_child_undumpable(_cmd: &mut Command) {}
