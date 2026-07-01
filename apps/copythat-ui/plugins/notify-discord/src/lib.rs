//! `notify-discord` — Phase 46.5 sample plugin.
//!
//! Registers `HookKind::AfterJob`. When a copy job finishes, this
//! plugin formats a Discord-webhook-shaped JSON payload summarising
//! the job (files copied, bytes, elapsed wall time) and returns it
//! to the engine inside a `HookOutcome::Notify { message }` whose
//! body is itself JSON. The engine — which holds the user's
//! `network` capability grant — performs the actual outbound HTTP
//! POST to the configured webhook URL.
//!
//! # Capability declaration
//!
//! `plugin.toml` requests the `network` capability. In the 46.5
//! runtime the engine is the one that talks to the network; the
//! plugin declares the capability so the user is asked to approve it
//! at install time and so the future WASI `sock_connect` import
//! (landing alongside the Settings UI in 46.6/46.7) will be wired
//! against the same approval.
//!
//! # Expected `HookCtx::data` schema (JSON, host-supplied)
//!
//! ```json
//! {
//!   "job_id": "5f3c…",
//!   "files_total": 1234,
//!   "files_failed": 0,
//!   "bytes_total": 9876543210,
//!   "elapsed_ms": 42_000,
//!   "webhook_url": "https://discord.com/api/webhooks/…"
//! }
//! ```
//!
//! # Outcome
//!
//! * `webhook_url` missing or empty → `HookOutcome::Continue` (no
//!   notification surface configured).
//! * Otherwise → `HookOutcome::Notify { message }` where `message`
//!   is JSON of the form
//!   `{"target":"discord","url":"<webhook_url>","payload":{…}}`.
//!   The engine decodes it, recognises the `discord` target, and
//!   POSTs `payload` to `url`.

use serde::{Deserialize, Serialize};

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
    webhook_url: Option<String>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum HookOutcome {
    Continue,
    Notify { message: String },
}

#[derive(Serialize)]
struct WebhookEnvelope<'a> {
    target: &'static str,
    url: &'a str,
    payload: DiscordPayload<'a>,
}

#[derive(Serialize)]
struct DiscordPayload<'a> {
    /// Plain-text fallback so clients that don't render embeds still
    /// see a useful summary line.
    content: String,
    embeds: [DiscordEmbed<'a>; 1],
}

#[derive(Serialize)]
struct DiscordEmbed<'a> {
    title: &'static str,
    description: String,
    /// Discord uses 24-bit integer colours. `0x57F287` is the
    /// "success green" Discord's own palette uses.
    color: u32,
    fields: Vec<DiscordField>,
    footer: DiscordFooter<'a>,
}

#[derive(Serialize)]
struct DiscordField {
    name: &'static str,
    value: String,
    inline: bool,
}

#[derive(Serialize)]
struct DiscordFooter<'a> {
    text: &'a str,
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
    let webhook_url = match ctx.data.webhook_url.as_deref() {
        Some(u) if !u.trim().is_empty() => u,
        _ => return HookOutcome::Continue,
    };
    let files_total = ctx.data.files_total.unwrap_or(0);
    let files_failed = ctx.data.files_failed.unwrap_or(0);
    let bytes_total = ctx.data.bytes_total.unwrap_or(0);
    let elapsed_ms = ctx.data.elapsed_ms.unwrap_or(0);
    let job_id = ctx.data.job_id.as_deref().unwrap_or("(unset)");

    let summary = format!(
        "Copied {} file{} ({}) in {}",
        files_total,
        if files_total == 1 { "" } else { "s" },
        format_bytes(bytes_total),
        format_duration_ms(elapsed_ms)
    );
    let envelope = WebhookEnvelope {
        target: "discord",
        url: webhook_url,
        payload: DiscordPayload {
            content: summary.clone(),
            embeds: [DiscordEmbed {
                title: "Freally File Manager — job complete",
                description: summary,
                color: if files_failed == 0 { 0x57F287 } else { 0xED4245 },
                fields: vec![
                    DiscordField {
                        name: "Files",
                        value: files_total.to_string(),
                        inline: true,
                    },
                    DiscordField {
                        name: "Failed",
                        value: files_failed.to_string(),
                        inline: true,
                    },
                    DiscordField {
                        name: "Size",
                        value: format_bytes(bytes_total),
                        inline: true,
                    },
                ],
                footer: DiscordFooter { text: job_id },
            }],
        },
    };
    let message = match serde_json::to_string(&envelope) {
        Ok(s) => s,
        Err(_) => return HookOutcome::Continue,
    };
    HookOutcome::Notify { message }
}

/// Render a byte count with binary (KiB / MiB / GiB) units. Plugin
/// defines its own formatter rather than pulling a humansize dep so
/// the WASM stays under 250 KiB.
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
