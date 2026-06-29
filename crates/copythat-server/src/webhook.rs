//! Phase 48 — webhook delivery (Slack / Discord / ntfy.sh / Pushover).
//!
//! [`crate::format_webhook_payload`] builds each service's JSON body; this
//! module POSTs it. A [`WebhookSink`] is one configured destination, and
//! [`WebhookSink::deliver`] formats + sends a [`JobNotification`] to it.

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{JobNotification, ServerError, WebhookTarget, format_webhook_payload};

/// Pushover requires an application token + user key alongside the message
/// (the other services authenticate via the secret in the webhook URL).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushoverCreds {
    pub token: String,
    pub user: String,
}

/// One configured webhook destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebhookSink {
    /// Which service's payload shape to use.
    pub target: WebhookTarget,
    /// Full webhook URL (Slack / Discord incoming-webhook URL, an ntfy
    /// topic URL, or the Pushover messages endpoint).
    pub url: String,
    /// Pushover app token + user key; ignored for the other targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushover: Option<PushoverCreds>,
}

impl WebhookSink {
    /// Format `event` for this sink's target and POST it. Returns
    /// [`ServerError::Webhook`] on a transport error or a non-2xx response.
    pub async fn deliver(
        &self,
        client: &Client,
        event: &JobNotification,
    ) -> Result<(), ServerError> {
        let mut payload = format_webhook_payload(self.target, event);
        // Pushover's token/user come from config, not the event body.
        if self.target == WebhookTarget::Pushover {
            if let (Some(creds), Some(obj)) = (&self.pushover, payload.as_object_mut()) {
                obj.insert("token".into(), creds.token.clone().into());
                obj.insert("user".into(), creds.user.clone().into());
            }
        }
        send_webhook(client, &self.url, &payload).await
    }
}

/// POST a pre-formatted webhook `payload` to `url`. Public so a caller that
/// already built the body (via [`crate::format_webhook_payload`]) can
/// deliver it directly.
pub async fn send_webhook(
    client: &Client,
    url: &str,
    payload: &serde_json::Value,
) -> Result<(), ServerError> {
    let resp = client.post(url).json(payload).send().await.map_err(|e| {
        // Never surface `e.to_string()`: reqwest embeds the full URL, and
        // the webhook URL *is* the secret (the token lives in its path).
        // Report only the host + error category.
        let kind = if e.is_timeout() {
            "timed out"
        } else if e.is_connect() {
            "connection failed"
        } else {
            "request failed"
        };
        ServerError::Webhook(format!("{} {kind}", url_host(url)))
    })?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(ServerError::Webhook(format!(
            "{} returned HTTP {}",
            url_host(url),
            resp.status()
        )))
    }
}

/// Best-effort host for an error message, without pulling in a URL parser.
fn url_host(url: &str) -> &str {
    url.split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use axum::Router;
    use axum::body::Bytes;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::routing::post;

    use super::*;

    type Store = Arc<Mutex<Option<serde_json::Value>>>;

    async fn capture(State(store): State<Store>, body: Bytes) -> &'static str {
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        *store.lock().unwrap() = Some(v);
        "ok"
    }

    /// Spin a loopback receiver, deliver a Slack notification, and confirm
    /// the captured JSON carries the formatted text.
    #[tokio::test]
    async fn delivers_and_captures_payload() {
        let store: Store = Arc::new(Mutex::new(None));
        let app = Router::new()
            .route("/hook", post(capture))
            .with_state(store.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let sink = WebhookSink {
            target: WebhookTarget::Slack,
            url: format!("http://{addr}/hook"),
            pushover: None,
        };
        let event = JobNotification {
            kind: "job_completed".into(),
            title: "Done".into(),
            body: "5 files".into(),
            ok: true,
        };
        sink.deliver(&Client::new(), &event).await.expect("deliver");

        let got = store.lock().unwrap().clone().expect("captured a payload");
        let text = got.get("text").and_then(|v| v.as_str()).unwrap();
        assert!(
            text.contains("Done") && text.contains("5 files"),
            "text: {text}"
        );
        server.abort();
    }

    /// Pushover credentials from the sink config land in the body.
    #[tokio::test]
    async fn pushover_creds_are_injected() {
        let store: Store = Arc::new(Mutex::new(None));
        let app = Router::new()
            .route("/hook", post(capture))
            .with_state(store.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let sink = WebhookSink {
            target: WebhookTarget::Pushover,
            url: format!("http://{addr}/hook"),
            pushover: Some(PushoverCreds {
                token: "app-tok".into(),
                user: "user-key".into(),
            }),
        };
        let event = JobNotification {
            kind: "job_failed".into(),
            title: "Oops".into(),
            body: "disk full".into(),
            ok: false,
        };
        sink.deliver(&Client::new(), &event).await.expect("deliver");

        let got = store.lock().unwrap().clone().unwrap();
        assert_eq!(got.get("token").and_then(|v| v.as_str()), Some("app-tok"));
        assert_eq!(got.get("user").and_then(|v| v.as_str()), Some("user-key"));
        server.abort();
    }

    /// A non-2xx response surfaces as a `Webhook` error.
    #[tokio::test]
    async fn non_2xx_is_an_error() {
        let app = Router::new().route(
            "/hook",
            post(|| async { (StatusCode::INTERNAL_SERVER_ERROR, "boom") }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let err = send_webhook(
            &Client::new(),
            &format!("http://{addr}/hook"),
            &serde_json::json!({ "text": "x" }),
        )
        .await
        .unwrap_err();
        assert!(matches!(err, ServerError::Webhook(_)), "got {err:?}");
        server.abort();
    }
}
