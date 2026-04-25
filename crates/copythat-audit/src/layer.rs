//! Phase 34 — `tracing::Layer` that fans events into the audit sink.
//!
//! The runner sends structured events directly (`sink.record(&evt)`)
//! because the conversion from queue/engine state to [`AuditEvent`]
//! needs typed fields we don't want to re-parse from
//! `tracing::Event` metadata. The layer is a second path for ad-hoc
//! `tracing::warn!(target: "copythat::audit", ...)` calls inside the
//! rest of the workspace — auditors occasionally want those in the
//! log too. Events without the `copythat::audit` target are ignored
//! so the layer is cheap on non-matching traces.
//!
//! The layer translates every matching event into an
//! [`AuditEvent::UnauthorizedAccess`] fallback form with the message
//! payload in `attempted_action` and the level in `reason`; richer
//! events should be emitted by the runner directly.

use std::sync::Arc;

use chrono::Utc;
use tracing::{Event, Level, Subscriber, field::Visit};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

use crate::{AuditEvent, AuditSink};

/// Tracing layer that forwards events whose `target` starts with
/// `copythat::audit` into the owned [`AuditSink`].
pub struct AuditLayer {
    sink: Arc<AuditSink>,
}

impl AuditLayer {
    pub fn new(sink: Arc<AuditSink>) -> Self {
        Self { sink }
    }
}

struct MessageVisitor {
    message: String,
}

/// Phase 17f — content-scrubbing field allowlist. Any tracing field
/// whose name is on this list is dropped before it lands in the audit
/// log. The list is conservative: keep it broad enough that a future
/// developer who forgets to consider scrubbing can't accidentally
/// route file contents through `tracing::warn!(body = …)`.
///
/// `body` / `bytes` / `chunk` cover raw file content the engine may
/// be tempted to log for debugging; `password` / `passphrase` /
/// `secret` / `token` / `api_key` cover credential surfaces that
/// pass through the audit layer's reach.
fn is_sensitive_field(name: &str) -> bool {
    matches!(
        name,
        "body"
            | "bytes"
            | "chunk"
            | "password"
            | "passphrase"
            | "secret"
            | "token"
            | "api_key"
            | "api-key"
    )
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if is_sensitive_field(field.name()) {
            // Phase 17f — drop the value entirely. Logging a redacted
            // marker (`field=<redacted>`) is itself information leakage
            // about which fields the caller chose to attach, so we
            // simply omit the field.
            return;
        }
        if field.name() == "message" {
            self.message.push_str(value);
        } else {
            if !self.message.is_empty() {
                self.message.push(' ');
            }
            self.message.push_str(field.name());
            self.message.push('=');
            self.message.push_str(value);
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if is_sensitive_field(field.name()) {
            return;
        }
        if field.name() == "message" {
            use std::fmt::Write;
            let _ = write!(&mut self.message, "{value:?}");
        } else {
            use std::fmt::Write;
            if !self.message.is_empty() {
                self.message.push(' ');
            }
            let _ = write!(&mut self.message, "{}={value:?}", field.name());
        }
    }
}

impl<S> Layer<S> for AuditLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        if !metadata.target().starts_with("copythat::audit") {
            return;
        }

        let mut visitor = MessageVisitor {
            message: String::new(),
        };
        event.record(&mut visitor);

        let reason = match *metadata.level() {
            Level::ERROR => "error",
            Level::WARN => "warn",
            Level::INFO => "info",
            Level::DEBUG => "debug",
            Level::TRACE => "trace",
        }
        .to_string();

        let evt = AuditEvent::UnauthorizedAccess {
            user: String::new(),
            host: gethostname::gethostname().to_string_lossy().into_owned(),
            attempted_action: visitor.message,
            reason,
            ts: Utc::now(),
        };
        let _ = self.sink.record(&evt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuditFormat, WormMode};
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    fn tracing_event_reaches_the_sink() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("audit.log");
        let sink = Arc::new(AuditSink::open(&path, AuditFormat::JsonLines, WormMode::Off).unwrap());
        let layer = AuditLayer::new(sink.clone());

        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::with_default(subscriber, || {
            tracing::warn!(target: "copythat::audit", "unauthorized-attempt");
        });

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("unauthorized-access"));
        assert!(contents.contains("unauthorized-attempt"));
    }

    #[test]
    fn phase_17f_sensitive_fields_are_scrubbed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("audit.log");
        let sink = Arc::new(AuditSink::open(&path, AuditFormat::JsonLines, WormMode::Off).unwrap());
        let layer = AuditLayer::new(sink.clone());

        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::with_default(subscriber, || {
            // Every name on the scrub allowlist must drop. Use literals
            // — the macro arm distinguishes string vs Debug and we
            // need to cover both visit paths.
            tracing::warn!(
                target: "copythat::audit",
                body = "secret-bytes",
                bytes = "more-secret",
                chunk = "even-more",
                password = "hunter2",
                passphrase = "longer-hunter2",
                secret = "shhh",
                token = "abc.def.ghi",
                api_key = "sk-test-...",
                "tagged-event"
            );
        });

        let contents = std::fs::read_to_string(&path).unwrap();
        // The tag itself flows through (it's the message body).
        assert!(contents.contains("tagged-event"));
        // Each sensitive field name + value must be absent.
        for forbidden in [
            "secret-bytes",
            "more-secret",
            "even-more",
            "hunter2",
            "longer-hunter2",
            "shhh",
            "abc.def.ghi",
            "sk-test-",
            "body=",
            "bytes=",
            "chunk=",
            "password=",
            "passphrase=",
            "secret=",
            "token=",
            "api_key=",
        ] {
            assert!(
                !contents.contains(forbidden),
                "audit log contains scrubbed field/value `{forbidden}`:\n{contents}",
            );
        }
    }
}
