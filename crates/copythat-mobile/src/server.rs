//! Remote-control message vocabulary.
//!
//! Phase 37 pivots away from a hand-rolled axum LAN pair-server in
//! favour of a PeerJS WebRTC data channel. The desktop's Tauri
//! webview boots a PeerJS client (registered under
//! `MobileSettings::desktop_peer_id`), accepts incoming connections
//! from paired phones, and dispatches each `RemoteCommand` arriving
//! on the data channel to the appropriate Tauri command. The data-
//! channel transport runs over WebRTC's DTLS, so confidentiality and
//! integrity are handled at the transport layer; this crate only
//! defines the message vocabulary plus a typed `RemoteResponse`
//! envelope.
//!
//! The runtime that actually implements the [`RemoteControl`] trait
//! lives in `apps/copythat-ui/src-tauri/src/mobile_commands.rs`.
//! The phone-side PWA that drives this protocol lives in
//! `apps/copythat-mobile/`.

use serde::{Deserialize, Serialize};

/// Envelope wrapping every request the phone sends. Tagged enum on
/// the wire — `{"kind":"list_jobs"}` /
/// `{"kind":"pause_job","job_id":"…"}` etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RemoteCommand {
    /// `Hello` is the first message a freshly-connected phone sends.
    /// Carries the phone's long-term X25519 public key (hex) + the
    /// human label the user typed during the pairing modal. Desktop
    /// looks the device up in `MobileSettings::pairings`; on a
    /// match the data channel transitions from "pairing" to
    /// "authenticated" and subsequent commands are accepted.
    Hello {
        phone_pubkey_hex: String,
        device_label: String,
    },
    /// List active engine jobs (running / paused / queued / failed).
    ListJobs,
    /// Pause a running job by id.
    PauseJob { job_id: String },
    /// Resume a paused job.
    ResumeJob { job_id: String },
    /// Cancel a job (graceful — engine emits a `Failed` event with
    /// `Cancelled` kind).
    CancelJob { job_id: String },
    /// Resolve an open collision prompt. Mirrors the desktop modal:
    /// `overwrite` / `overwrite_all` / `skip` / `skip_all` /
    /// `rename` / `keep_both`.
    ResolveCollision {
        prompt_id: String,
        action: CollisionAction,
    },
    /// Read the current global counters (running bytes / files /
    /// rate). Bridge target for the home-screen widget.
    Globals,
    /// Read the last `limit` history rows.
    RecentHistory { limit: u32 },
    /// Re-run a history row as a fresh job.
    RerunHistory { row_id: i64 },
    /// Start a secure-delete job against a list of paths.
    SecureDelete { paths: Vec<String>, method: String },
    /// Start a copy job. Mirrors the GUI's `start_copy` IPC.
    StartCopy {
        sources: Vec<String>,
        destination: String,
        verify: Option<String>,
    },
    /// Phone-side Exit button. Tells the desktop to drop this peer
    /// from its active-connection set so the next time the PWA
    /// reopens it has to rehandshake. The desktop replies with `Ok`
    /// and closes the data channel; the PeerJS client on the PWA
    /// side disconnects from the signaling broker.
    Goodbye,
    /// Phone-side toggle that asks the desktop to inhibit screen
    /// sleep / screensaver while the connection is live. When
    /// `enabled = true` the desktop calls the platform wake-lock
    /// API (`SetThreadExecutionState` on Windows,
    /// `IOPMAssertionCreateWithName` on macOS,
    /// `org.freedesktop.ScreenSaver.Inhibit` on Linux). When
    /// `false` it releases the assertion. The OS-level wake-lock
    /// glue lives in `copythat-platform` (Phase 37 follow-up); the
    /// IPC + setting land here.
    SetKeepAwake { enabled: bool },
    /// PWA queries the desktop for its current BCP-47 locale tag
    /// (`"en"`, `"fr"`, `"pt-BR"`, …). The PWA loads the matching
    /// translation bundle so the phone UI stays in sync with the
    /// desktop's selected language. Reply: `RemoteResponse::Locale`.
    GetLocale,
}

/// Collision-resolution action the phone replies with. Mirrors the
/// GUI's modal options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionAction {
    Overwrite,
    OverwriteAll,
    Skip,
    SkipAll,
    Rename,
    KeepBoth,
}

/// What the desktop sends back. Tagged enum so the PWA's switch
/// statement covers every variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RemoteResponse {
    /// Reply to `Hello` — `paired = true` means the device's
    /// public key matched a `MobileSettings::pairings` entry and
    /// subsequent commands will be dispatched. `false` means
    /// re-enrolment is required.
    HelloAck { paired: bool },
    /// Reply to `ListJobs`.
    Jobs { jobs: Vec<JobSummary> },
    /// Reply to `Globals`.
    Globals {
        bytes_done: u64,
        bytes_total: u64,
        files_done: u64,
        files_total: u64,
        rate_bps: u64,
    },
    /// Reply to `RecentHistory`.
    History { rows: Vec<HistoryRow> },
    /// Generic ack — used by pause / resume / cancel / resolve-
    /// collision / shred / rerun / start_copy when the action
    /// committed successfully.
    Ok,
    /// Generic error — surfaces back to the PWA as a toast.
    Error { message: String },
    /// Streaming live progress: the desktop pushes these on the data
    /// channel without a matching request whenever a job ticks. The
    /// PWA's job list updates in place.
    JobProgress {
        job_id: String,
        bytes_done: u64,
        bytes_total: u64,
        rate_bps: u64,
    },
    /// Streaming completion notification.
    JobCompleted { job_id: String, bytes: u64 },
    /// Streaming failure notification.
    JobFailed { job_id: String, reason: String },
    /// Streaming live globals tick. The desktop emits one of these
    /// per ~250 ms while at least one job is running so the PWA's
    /// home screen can render the aggregate percentage + the
    /// per-operation file counters without polling.
    GlobalsTick {
        bytes_done: u64,
        bytes_total: u64,
        files_done: u64,
        files_total: u64,
        rate_bps: u64,
        /// Counters split by job kind. The PWA's stats panel
        /// renders one row per non-zero entry.
        copy_files: u64,
        move_files: u64,
        secure_delete_files: u64,
    },
    /// Streaming per-file event. Emitted on every engine
    /// `Started` / `Progress` / `Completed` / `FileError` so the
    /// PWA's "Live files" panel can show exactly what's mid-flight
    /// — the user sees individual filenames scroll by as the
    /// desktop processes the tree.
    FileTick {
        job_id: String,
        /// `"scanning" | "copying" | "moving" | "verifying" |
        /// "shredding" | "completed" | "failed"` — what the engine
        /// is doing to this file right now.
        action: String,
        src: String,
        dst: String,
        bytes_done: u64,
        bytes_total: u64,
    },
    /// "Job is initialising" notification. Fires on `start_copy`,
    /// `rerun_history`, and any other path where the engine has to
    /// scan + enumerate before per-file work begins. The PWA's
    /// dashboard locks every control button until the matching
    /// `JobReady` event lands so the user can't fire conflicting
    /// commands during the load phase.
    JobLoading { job_id: String, message: String },
    /// Inverse of `JobLoading` — the engine has finished the scan
    /// + enumeration and is ready to accept user input again.
    JobReady { job_id: String },
    /// Streaming pause/cancel-from-desktop notification. When the
    /// user pauses or cancels a job from the desktop UI, the runner
    /// pushes one of these to every connected phone so the PWA's
    /// job list mirrors the state change without polling. The
    /// reverse path (PWA pauses → desktop pauses) flows through
    /// the regular `PauseJob` / `CancelJob` request → `Ok` reply.
    JobStateChanged {
        job_id: String,
        /// Stable wire string: `"running"` / `"paused"` / `"cancelled"`
        /// / `"failed"` / `"completed"`. PWA's switch maps to UI
        /// state.
        state: String,
    },
    /// Desktop is exiting (user closed the window, OS shutdown,
    /// crash recovery). Streamed to every connected phone right
    /// before the data channel closes so the PWA can render an
    /// explicit "Desktop exited — reconnect when Copy That is
    /// running" screen instead of a generic disconnect.
    ServerShuttingDown { reason: String },
    /// Reply to `GetLocale`. Empty string = "auto-detect"; the PWA
    /// falls back to its own browser locale in that case.
    Locale { bcp47: String },
}

/// Per-job summary in `RemoteResponse::Jobs`. Mirrors the shape the
/// GUI renders in `JobRow.svelte` so the PWA can re-use the same
/// styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSummary {
    pub job_id: String,
    pub kind: String,
    pub state: String,
    pub src: String,
    pub dst: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub rate_bps: u64,
}

/// History row summary in `RemoteResponse::History`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRow {
    pub row_id: i64,
    pub kind: String,
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub src_root: String,
    pub dst_root: String,
    pub total_bytes: u64,
    pub files_ok: u64,
    pub files_failed: u64,
}

/// Async-trait the Tauri shell implements. Each method runs against
/// the live `AppState`. The dispatcher requires the caller to
/// present a paired-phone pubkey via [`RemoteCommand::Hello`] before
/// non-lifecycle commands are accepted; see [`SessionAuth`] +
/// [`dispatch_with_auth`] for the per-connection state machine.
#[async_trait::async_trait]
pub trait RemoteControl: Send + Sync {
    async fn list_jobs(&self) -> Result<Vec<JobSummary>, String>;
    async fn pause_job(&self, job_id: &str) -> Result<(), String>;
    async fn resume_job(&self, job_id: &str) -> Result<(), String>;
    async fn cancel_job(&self, job_id: &str) -> Result<(), String>;
    async fn resolve_collision(
        &self,
        prompt_id: &str,
        action: CollisionAction,
    ) -> Result<(), String>;
    async fn globals(&self) -> Result<RemoteResponse, String>;
    async fn recent_history(&self, limit: u32) -> Result<Vec<HistoryRow>, String>;
    async fn rerun_history(&self, row_id: i64) -> Result<(), String>;
    async fn secure_delete(&self, paths: Vec<String>, method: &str) -> Result<(), String>;
    async fn start_copy(
        &self,
        sources: Vec<String>,
        destination: String,
        verify: Option<String>,
    ) -> Result<(), String>;
    /// Phase 37 — toggle the OS wake-lock so the desktop won't
    /// sleep / screensaver while a phone is paired in. The
    /// platform syscall lives in `copythat-platform`; this trait
    /// method is a thin shim the Tauri shell wires up.
    async fn set_keep_awake(&self, enabled: bool) -> Result<(), String>;
    /// Phase 38 — return the desktop's current BCP-47 locale tag
    /// so the PWA can load the matching translation bundle.
    async fn get_locale(&self) -> Result<String, String>;

    /// Look up whether `phone_pubkey_hex` matches a stored
    /// `MobileSettings::pairings` entry. Default impl returns
    /// `false` (refuse all) — implementations that wire pairings
    /// override this. Implementations MUST do a constant-time
    /// comparison over the canonicalised hex form to avoid leaking
    /// match-prefix length via timing.
    async fn is_paired_phone(&self, _phone_pubkey_hex: &str) -> bool {
        false
    }
}

/// Per-connection auth state. The PeerJS adapter in the Tauri
/// webview owns one of these per data-channel and threads it into
/// every [`dispatch_with_auth`] call. The state survives across
/// commands but resets when the channel closes.
#[derive(Debug, Default)]
pub struct SessionAuth {
    paired_pubkey: Option<String>,
}

impl SessionAuth {
    /// Fresh, unauthenticated session.
    pub fn new() -> Self {
        Self::default()
    }
    /// True iff a `Hello` against a known pairing has succeeded
    /// during this session.
    pub fn is_authenticated(&self) -> bool {
        self.paired_pubkey.is_some()
    }
    /// Public key (hex) of the authenticated pairing, if any.
    pub fn paired_pubkey(&self) -> Option<&str> {
        self.paired_pubkey.as_deref()
    }
}

/// Dispatch a single decoded [`RemoteCommand`] through a
/// [`RemoteControl`] implementation, returning the matching
/// [`RemoteResponse`]. The PeerJS adapter in the Tauri webview
/// handles JSON encoding + decoding around this entry point.
///
/// **DEPRECATED, will be removed once every caller migrates to
/// [`dispatch_with_auth`].** This shape always treats the
/// connection as authenticated, which is unsafe — see Vuln 1 of
/// the Phase 38 mobile-pairing security review.
pub async fn dispatch<C: RemoteControl + ?Sized>(cmd: RemoteCommand, ctl: &C) -> RemoteResponse {
    let mut auth = SessionAuth {
        paired_pubkey: Some("legacy-trusted".into()),
    };
    dispatch_with_auth(cmd, ctl, &mut auth).await
}

/// Authenticated dispatcher. Callers thread one `SessionAuth` per
/// data-channel; the dispatcher requires `Hello` to land first and
/// to carry a `phone_pubkey_hex` matching a stored pairing before
/// any non-lifecycle command is accepted.
///
/// Lifecycle (`Hello`, `Goodbye`) are always allowed; everything
/// else returns `RemoteResponse::Error { message: "err-mobile-not-authenticated" }`
/// when the session hasn't yet authenticated.
pub async fn dispatch_with_auth<C: RemoteControl + ?Sized>(
    cmd: RemoteCommand,
    ctl: &C,
    auth: &mut SessionAuth,
) -> RemoteResponse {
    // Hello and Goodbye always pass — Hello to *establish* auth,
    // Goodbye to tear the channel down even if pairing failed.
    if let RemoteCommand::Hello {
        ref phone_pubkey_hex,
        ..
    } = cmd
    {
        let paired = ctl.is_paired_phone(phone_pubkey_hex).await;
        if paired {
            auth.paired_pubkey = Some(phone_pubkey_hex.clone());
        } else {
            auth.paired_pubkey = None;
        }
        return RemoteResponse::HelloAck { paired };
    }
    if matches!(cmd, RemoteCommand::Goodbye) {
        auth.paired_pubkey = None;
        return RemoteResponse::Ok;
    }
    if !auth.is_authenticated() {
        return RemoteResponse::Error {
            message: "err-mobile-not-authenticated".to_string(),
        };
    }
    match cmd {
        RemoteCommand::Hello { .. } | RemoteCommand::Goodbye => unreachable!(),
        RemoteCommand::ListJobs => match ctl.list_jobs().await {
            Ok(jobs) => RemoteResponse::Jobs { jobs },
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::PauseJob { job_id } => map_unit(ctl.pause_job(&job_id).await),
        RemoteCommand::ResumeJob { job_id } => map_unit(ctl.resume_job(&job_id).await),
        RemoteCommand::CancelJob { job_id } => map_unit(ctl.cancel_job(&job_id).await),
        RemoteCommand::ResolveCollision { prompt_id, action } => {
            map_unit(ctl.resolve_collision(&prompt_id, action).await)
        }
        RemoteCommand::Globals => match ctl.globals().await {
            Ok(resp) => resp,
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::RecentHistory { limit } => match ctl.recent_history(limit).await {
            Ok(rows) => RemoteResponse::History { rows },
            Err(message) => RemoteResponse::Error { message },
        },
        RemoteCommand::RerunHistory { row_id } => map_unit(ctl.rerun_history(row_id).await),
        RemoteCommand::SecureDelete { paths, method } => {
            map_unit(ctl.secure_delete(paths, &method).await)
        }
        RemoteCommand::StartCopy {
            sources,
            destination,
            verify,
        } => map_unit(ctl.start_copy(sources, destination, verify).await),
        RemoteCommand::SetKeepAwake { enabled } => map_unit(ctl.set_keep_awake(enabled).await),
        RemoteCommand::GetLocale => match ctl.get_locale().await {
            Ok(bcp47) => RemoteResponse::Locale { bcp47 },
            Err(message) => RemoteResponse::Error { message },
        },
    }
}

fn map_unit(r: Result<(), String>) -> RemoteResponse {
    match r {
        Ok(()) => RemoteResponse::Ok,
        Err(message) => RemoteResponse::Error { message },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_command_round_trips_through_serde() {
        let cmd = RemoteCommand::PauseJob {
            job_id: "abc-123".into(),
        };
        let s = serde_json::to_string(&cmd).unwrap();
        let back: RemoteCommand = serde_json::from_str(&s).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn collision_action_serializes_snake_case() {
        let s = serde_json::to_string(&CollisionAction::OverwriteAll).unwrap();
        assert_eq!(s, "\"overwrite_all\"");
    }

    #[test]
    fn remote_response_jobs_round_trip() {
        let resp = RemoteResponse::Jobs {
            jobs: vec![JobSummary {
                job_id: "j1".into(),
                kind: "copy".into(),
                state: "running".into(),
                src: "C:/src".into(),
                dst: "D:/dst".into(),
                bytes_done: 1024,
                bytes_total: 4096,
                rate_bps: 800_000,
            }],
        };
        let s = serde_json::to_string(&resp).unwrap();
        let back: RemoteResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back);
    }
}
