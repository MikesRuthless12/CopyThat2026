//! `notify-ntfy` — Phase 46.5 sample plugin.
//!
//! Registers `HookKind::AfterJob`. Same shape as `notify-discord`
//! but produces an ntfy.sh-style envelope: ntfy reads the message
//! body verbatim from the POST body and routes it by URL path
//! (`https://ntfy.sh/<topic>`), so the JSON envelope is simpler than
//! the Discord one.
//!
//! # Capability declaration
//!
//! `plugin.toml` requests the `network` capability — see
//! `notify-discord/src/lib.rs` for the rationale (engine performs
//! the POST in the 46.5 runtime; in-plugin `sock_connect` lands
//! once WASI wiring is complete in 46.6/46.7).
//!
//! # Expected `HookCtx::data` schema (JSON, host-supplied)
//!
//! ```json
//! {
//!   "job_id": "5f3c…",
//!   "files_total": 1234,
//!   "bytes_total": 9876543210,
//!   "elapsed_ms": 42_000,
//!   "ntfy_topic_url": "https://ntfy.sh/copythat-mike",
//!   "priority": "default" | "high" | "low"
//! }
//! ```
//!
//! # Outcome
//!
//! * `ntfy_topic_url` missing or empty → `HookOutcome::Continue`.
//! * Otherwise → `HookOutcome::Notify { message }` where `message`
//!   is JSON of the form
//!   `{"target":"ntfy","url":"<topic_url>","headers":{…},"body":"…"}`.
//!   The engine sees `target == "ntfy"`, sets the headers (`Title`,
//!   `Priority`, `Tags`) per ntfy's HTTP API, and POSTs `body`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod abi;

#[derive(Deserialize)]
struct HookCtx {
    #[serde(default)]
    data: CtxData,
}

#[derive(Deserialize, Default)]
struct CtxData {
    #[serde(default)]
    job_id: Option<String>,
    #[serde(default)]
    files_total: Option<u64>,
    #[serde(default)]
    files_failed: Option<u64>,
    #[serde(default)]
    bytes_total: Option<u64>,
    #[serde(default)]
    elapsed_ms: Option<u64>,
    #[serde(default)]
    ntfy_topic_url: Option<String>,
    #[serde(default)]
    priority: Option<String>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum HookOutcome {
    Continue,
    Notify { message: String },
}

#[derive(Serialize)]
struct NtfyEnvelope<'a> {
    target: &'static str,
    url: &'a str,
    headers: BTreeMap<&'static str, String>,
    body: String,
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: i32) -> i32 {
    abi::alloc(size)
}

#[unsafe(no_mangle)]
pub extern "C" fn hook(ctx_ptr: i32, ctx_len: i32) -> i64 {
    let outcome = abi::with_ctx(ctx_ptr, ctx_len, compute);
    abi::pack_outcome(&outcome)
}

fn compute(ctx_bytes: &[u8]) -> HookOutcome {
    let ctx: HookCtx = match serde_json::from_slice(ctx_bytes) {
        Ok(c) => c,
        Err(_) => return HookOutcome::Continue,
    };
    let url = match ctx.data.ntfy_topic_url.as_deref() {
        Some(u) if !u.trim().is_empty() => u,
        _ => return HookOutcome::Continue,
    };
    let files_total = ctx.data.files_total.unwrap_or(0);
    let files_failed = ctx.data.files_failed.unwrap_or(0);
    let bytes_total = ctx.data.bytes_total.unwrap_or(0);
    let elapsed_ms = ctx.data.elapsed_ms.unwrap_or(0);
    let job_id = ctx.data.job_id.as_deref().unwrap_or("(unset)");
    let priority = sanitise_priority(ctx.data.priority.as_deref());

    let body = format!(
        "Copied {} file{} ({}) in {} — job {}",
        files_total,
        if files_total == 1 { "" } else { "s" },
        format_bytes(bytes_total),
        format_duration_ms(elapsed_ms),
        job_id
    );
    let mut headers: BTreeMap<&'static str, String> = BTreeMap::new();
    headers.insert("Title", "Freally File Manager — job complete".to_string());
    headers.insert("Priority", priority.to_string());
    headers.insert(
        "Tags",
        if files_failed == 0 {
            "white_check_mark,copythat".to_string()
        } else {
            "warning,copythat".to_string()
        },
    );
    let envelope = NtfyEnvelope {
        target: "ntfy",
        url,
        headers,
        body,
    };
    let message = match serde_json::to_string(&envelope) {
        Ok(s) => s,
        Err(_) => return HookOutcome::Continue,
    };
    HookOutcome::Notify { message }
}

/// Restrict the user-supplied `priority` to ntfy's documented set
/// (`min` / `low` / `default` / `high` / `urgent`). Any unknown value
/// (including a typo or a hostile input from the host) collapses to
/// `default` so the eventual HTTP POST cannot smuggle a header value
/// the user didn't intend.
fn sanitise_priority(raw: Option<&str>) -> &'static str {
    match raw {
        Some("min") => "min",
        Some("low") => "low",
        Some("high") => "high",
        Some("urgent") => "urgent",
        _ => "default",
    }
}

fn format_bytes(n: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    const TIB: u64 = GIB * 1024;
    if n >= TIB {
        format!("{:.2} TiB", n as f64 / TIB as f64)
    } else if n >= GIB {
        format!("{:.2} GiB", n as f64 / GIB as f64)
    } else if n >= MIB {
        format!("{:.2} MiB", n as f64 / MIB as f64)
    } else if n >= KIB {
        format!("{:.2} KiB", n as f64 / KIB as f64)
    } else {
        format!("{n} B")
    }
}

fn format_duration_ms(ms: u64) -> String {
    if ms < 1000 {
        return format!("{ms} ms");
    }
    let secs = ms / 1000;
    let rem_ms = ms % 1000;
    if secs < 60 {
        return format!("{secs}.{:03} s", rem_ms);
    }
    let mins = secs / 60;
    let rem_s = secs % 60;
    if mins < 60 {
        return format!("{mins}m {rem_s}s");
    }
    let hours = mins / 60;
    let rem_m = mins % 60;
    format!("{hours}h {rem_m}m")
}
