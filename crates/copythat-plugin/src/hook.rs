use serde::{Deserialize, Serialize};

/// Lifecycle event the plugin is being invoked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookKind {
    BeforeJob,
    BeforeFile,
    AfterFile,
    AfterJob,
    OnError,
}

/// JSON-serializable context handed to a plugin invocation.
///
/// Fields are intentionally minimal at the scaffold layer; sub-phase
/// 46.2 fleshes them out (job id, source/dest path snapshots, byte
/// counts, error variants) once the dispatch plumbing is in place.
/// `data` is held as `serde_json::Value` so 46.2 can grow the schema
/// without breaking the ABI on already-loaded plugins.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HookCtx {
    /// Hook kind being dispatched. Mirrors the [`HookKind`] passed to
    /// [`crate::PluginHandle::call_hook`] so the plugin can branch on
    /// it without a separate argument.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<HookKind>,

    /// Free-form fields the dispatcher fills in per-hook. 46.2 adds
    /// typed accessors layered on top; at 46.1 the raw value lets
    /// tests round-trip arbitrary payloads.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub data: serde_json::Value,
}

/// What the plugin tells the engine to do next.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HookOutcome {
    /// Engine continues normally.
    Continue,
    /// Engine skips the current file (only meaningful for
    /// [`HookKind::BeforeFile`] and [`HookKind::AfterFile`]).
    SkipFile,
    /// Engine aborts the job after the current file finishes.
    AbortJob,
    /// Engine emits a notification with the given text and continues.
    Notify { message: String },
}

impl Default for HookOutcome {
    fn default() -> Self {
        HookOutcome::Continue
    }
}
