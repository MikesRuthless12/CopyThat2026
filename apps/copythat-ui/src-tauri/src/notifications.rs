//! Phase 49q — job-lifecycle notifications.
//!
//! Fans a [`JobNotification`] out to the configured server webhooks
//! (Phase 48 [`WebhookSink`]) when a backup/job completes or fails, gated
//! by the user's [`NotificationSettings`](copythat_settings::NotificationSettings)
//! on-success / on-failure toggles. Best-effort: each sink delivers in its
//! own task and one failure never blocks the job or the other sinks.
//!
//! Destinations reuse the server-mode webhook list ([`ServerSettings::webhooks`])
//! so the user configures webhooks in one place; these notifications just
//! add the engine-job lifecycle as a trigger.

use copythat_server::{JobNotification, PushoverCreds, WebhookSink, WebhookTarget};
use copythat_settings::WebhookSettings;

use crate::state::AppState;

/// Map a persisted [`WebhookSettings`] to a [`WebhookSink`], or `None` if
/// the target string is unrecognised.
fn webhook_sink(s: &WebhookSettings) -> Option<WebhookSink> {
    let target = match s.target.as_str() {
        "slack" => WebhookTarget::Slack,
        "discord" => WebhookTarget::Discord,
        "ntfy" => WebhookTarget::Ntfy,
        "pushover" => WebhookTarget::Pushover,
        _ => return None,
    };
    let pushover = (target == WebhookTarget::Pushover).then(|| PushoverCreds {
        token: s.pushover_token.clone(),
        user: s.pushover_user.clone(),
    });
    Some(WebhookSink {
        target,
        url: s.url.clone(),
        pushover,
    })
}

/// Deliver `event` to every sink concurrently, best-effort. A sink failure
/// is logged (host + category only, never the secret URL), never propagated.
pub(crate) async fn deliver_all(sinks: Vec<WebhookSink>, event: JobNotification) {
    if sinks.is_empty() {
        return;
    }
    let client = reqwest::Client::new();
    let mut tasks = Vec::with_capacity(sinks.len());
    for sink in sinks {
        let client = client.clone();
        let event = event.clone();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = sink.deliver(&client, &event).await {
                tracing::warn!(error = %e, "notification webhook delivery failed");
            }
        }));
    }
    for t in tasks {
        let _ = t.await;
    }
}

/// Fire `event` to the configured server webhooks IF the matching
/// on-success / on-failure toggle is set. Reads the live settings.
pub(crate) async fn dispatch(state: &AppState, event: JobNotification) {
    let settings = state.settings_snapshot();
    let gated = if event.ok {
        settings.notifications.on_success
    } else {
        settings.notifications.on_failure
    };
    if !gated {
        return;
    }
    let sinks: Vec<WebhookSink> = settings
        .server
        .webhooks
        .iter()
        .filter_map(webhook_sink)
        .collect();
    deliver_all(sinks, event).await;
}

/// `notifications_test` — send a test notification to the configured
/// webhooks now, ignoring the on-success/on-failure gates, so the user can
/// verify their destinations from Settings. Returns the number of sinks
/// attempted.
#[tauri::command]
pub async fn notifications_test(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    let sinks: Vec<WebhookSink> = state
        .settings_snapshot()
        .server
        .webhooks
        .iter()
        .filter_map(webhook_sink)
        .collect();
    let n = sinks.len();
    deliver_all(
        sinks,
        JobNotification {
            kind: "test".into(),
            title: "Copy That".into(),
            body: "Test notification".into(),
            ok: true,
        },
    )
    .await;
    Ok(n)
}

/// Wire shape for [`copythat_settings::NotificationSettings`].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettingsDto {
    pub on_success: bool,
    pub on_failure: bool,
}

/// Read the notification toggles.
#[tauri::command]
pub fn notifications_get(state: tauri::State<'_, AppState>) -> NotificationSettingsDto {
    let n = state.settings_snapshot().notifications;
    NotificationSettingsDto {
        on_success: n.on_success,
        on_failure: n.on_failure,
    }
}

/// Set + persist the notification toggles.
#[tauri::command]
pub fn notifications_set(
    on_success: bool,
    on_failure: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    guard.notifications.on_success = on_success;
    guard.notifications.on_failure = on_failure;
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        return Ok(()); // tests use an empty path; skip persistence
    }
    guard
        .save_to(path)
        .map_err(|e| format!("save settings: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(target: &str) -> WebhookSettings {
        WebhookSettings {
            target: target.into(),
            url: "https://hooks.example/x".into(),
            pushover_token: "tok".into(),
            pushover_user: "usr".into(),
        }
    }

    #[test]
    fn webhook_sink_maps_known_targets() {
        assert_eq!(
            webhook_sink(&settings("slack")).unwrap().target,
            WebhookTarget::Slack
        );
        assert_eq!(
            webhook_sink(&settings("discord")).unwrap().target,
            WebhookTarget::Discord
        );
        assert_eq!(
            webhook_sink(&settings("ntfy")).unwrap().target,
            WebhookTarget::Ntfy
        );
    }

    #[test]
    fn pushover_carries_creds_others_dont() {
        let p = webhook_sink(&settings("pushover")).unwrap();
        assert_eq!(p.target, WebhookTarget::Pushover);
        let creds = p.pushover.unwrap();
        assert_eq!(creds.token, "tok");
        assert_eq!(creds.user, "usr");
        assert!(webhook_sink(&settings("slack")).unwrap().pushover.is_none());
    }

    #[test]
    fn unknown_target_is_rejected() {
        assert!(webhook_sink(&settings("carrier-pigeon")).is_none());
    }
}
