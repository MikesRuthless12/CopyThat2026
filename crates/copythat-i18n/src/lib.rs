//! `copythat-i18n` — Fluent loader.
//!
//! Phase 11 shipped the full i18n surface: per-locale Fluent bundle
//! loading, translation key lookup with cascading parameter
//! interpolation, the `t!()` macro families consumed across the Tauri
//! bridge and the CLI, and the lint-time enforcement that every key
//! referenced from Rust / Svelte source has a matching entry in the
//! `en-US` baseline (gated by `cargo run -p xtask -- i18n-lint`).
//!
//! This crate is a thin facade over those entrypoints. Concrete
//! exports live in dedicated modules (loader, bundle, macros);
//! downstream callers reach for them directly. Adding a new locale
//! is a content-only change — drop a `<lang>.ftl` under `locales/`
//! and the loader picks it up at runtime; the lint catches missing
//! keys at xtask time.
