//! NIST 800-88 Purge — hardware secure-erase support.
//!
//! NIST SP 800-88 rev. 1 defines three sanitization tiers for storage
//! media:
//!   - **Clear** — logical overwrite through normal read/write APIs
//!     (modelled by [`ShredMethod::Nist80088Clear`](crate::ShredMethod::Nist80088Clear)).
//!   - **Purge** — hardware or cryptographic sanitization that defeats
//!     a laboratory-level attacker (ATA SECURE ERASE, NVMe Format w/
//!     Secure Erase, or discarding an FDE key).
//!   - **Destroy** — physical destruction, out of scope for software.
//!
//! A per-file API can't honestly offer Purge: SECURE ERASE and NVMe
//! Format operate at the **block device** level. They wipe the whole
//! drive, not a single file. Advertising Purge on a per-file shred
//! would violate the contract the name implies.
//!
//! So this module exposes a single capability probe
//! [`hardware_purge_available`]. It intentionally returns `false` on
//! every platform: the Phase 4 library is the unprivileged layer.
//! Phase 17 will add a privilege-separated `copythat-helper` binary
//! that can invoke `hdparm --security-erase`, `nvme format --ses=1`,
//! or the Windows / macOS equivalents. Until then, `Nist80088Purge`
//! fails at the API boundary with [`ShredErrorKind::PurgeNotSupported`]
//! (and a user-facing message pointing at Clear + FDE rotation).
//!
//! [`ShredErrorKind::PurgeNotSupported`]: crate::error::ShredErrorKind::PurgeNotSupported

/// Is a hardware secure-erase path available to the current process
/// for the device underlying `path`?
///
/// Phase 4 unconditionally reports `false`. See module-level docs.
pub(crate) fn hardware_purge_available(_path: &std::path::Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardware_purge_is_unsupported_in_phase_4() {
        // When this flips to `true` a maintainer must also add a
        // privileged helper path to the engine. Guard with a test so
        // the flip is deliberate, not accidental.
        assert!(!hardware_purge_available(std::path::Path::new(".")));
    }
}
