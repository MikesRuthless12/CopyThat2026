//! `copythat-secure-delete` — multi-pass shredding.
//!
//! # What's here (Phase 4)
//!
//! - [`ShredMethod`] — nine named overwrite strategies: `Zero`,
//!   `Random`, `DoD3Pass`, `DoD7Pass`, `Gutmann35`, `Schneier7`,
//!   `Vsitr7`, `Nist80088Clear`, `Nist80088Purge`. Each one expands to
//!   a fixed sequence of [`PassPattern`]s.
//! - [`shred_file`] and [`shred_tree`] — async APIs mirroring the
//!   shape of `copythat_core::copy_file` / `copy_tree`: caller passes
//!   a `copythat_core::CopyControl` for pause / resume / cancel and an mpsc
//!   `Sender<ShredEvent>` for progress events.
//! - [`is_ssd`] — best-effort probe of the underlying block device. On
//!   SSDs the public API emits [`ShredEvent::SsdAdvisory`] before the
//!   first pass (localized message key) — the operation continues
//!   regardless; the event is advisory only.
//! - NIST 800-88 Purge is handled specially: it prefers a hardware
//!   secure-erase (ATA SECURE ERASE, NVMe Format w/ Secure Erase) and,
//!   when the hardware path isn't available, the API refuses with
//!   [`ShredErrorKind::PurgeNotSupported`] and recommends Clear + FDE
//!   key rotation.
//!
//! # Why this shape
//!
//! The shredder shares the `CopyControl` steering primitive with the
//! copy engine (Phase 1) and the hashing pipeline (Phase 3): a UI that
//! already drives copy / verify jobs can drive shred jobs with the
//! same pause / resume / cancel handle.
//!
//! # Caveats
//!
//! Multi-pass overwrite patterns are a **magnetic-media** tradition.
//! On SSDs and modern NVMe devices wear-leveling, over-provisioning,
//! and FTL remapping mean a logical pass over a file does not
//! guarantee the underlying NAND cells are overwritten. For SSDs the
//! only reliable sanitization is ATA SECURE ERASE / NVMe Format or
//! full-disk encryption with a discarded key; the [`ShredEvent::SsdAdvisory`]
//! event surfaces this to the UI before the first pass.
//!
//! # Example
//!
//! ```no_run
//! use copythat_core::CopyControl;
//! use copythat_secure_delete::{shred_file, ShredEvent, ShredMethod};
//! use tokio::sync::mpsc;
//!
//! # async fn demo() -> Result<(), copythat_secure_delete::ShredError> {
//! let (tx, mut rx) = mpsc::channel::<ShredEvent>(64);
//! let ctrl = CopyControl::new();
//! let task = tokio::spawn(async move {
//!     shred_file(
//!         std::path::Path::new("secret.pdf"),
//!         ShredMethod::DoD3Pass,
//!         ctrl,
//!         tx,
//!     )
//!     .await
//! });
//! while let Some(evt) = rx.recv().await {
//!     if let ShredEvent::PassProgress { pass_index, bytes, total, .. } = evt {
//!         eprintln!("pass {pass_index}: {bytes}/{total}");
//!     }
//! }
//! let _ = task.await.unwrap()?;
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

mod engine;
mod error;
mod event;
#[cfg(target_os = "linux")]
mod linux_helper;
#[cfg(target_os = "macos")]
mod macos_helper;
mod method;
#[cfg(feature = "experimental-tcg-opal")]
pub mod opal;
mod pattern;
mod purge;
mod sanitize;
mod ssd;
mod tree;
#[cfg(target_os = "windows")]
mod windows_helper;

pub use engine::shred_file;
pub use error::{ShredError, ShredErrorKind};
pub use event::{ShredEvent, ShredReport};
#[cfg(target_os = "linux")]
pub use linux_helper::LinuxSanitizeHelper;
#[cfg(target_os = "macos")]
pub use macos_helper::MacosSanitizeHelper;
pub use method::{CHUNK_SIZE, ShredMethod};
pub use pattern::PassPattern;
pub use sanitize::{
    CowProbe, FreeSpaceTrimReport, NoopSanitizeHelper, SanitizeCapabilities, SanitizeHelper,
    SanitizeReport, SsdSanitizeMode, free_space_trim, is_cow_filesystem, opal_psid_revert,
    refuse_shred_on_cow, sanitize_capabilities, set_cow_probe, whole_drive_sanitize,
};
pub use ssd::is_ssd;
pub use tree::shred_tree;
#[cfg(target_os = "windows")]
pub use windows_helper::WindowsSanitizeHelper;
