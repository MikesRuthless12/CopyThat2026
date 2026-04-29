//! Route handlers.
//!
//! Each handler returns `Result<T, RouteError>` so a History / chunk-
//! store failure renders a stable HTML error page rather than tower's
//! default text "Internal Server Error". JSON-ish endpoints
//! (`POST /restore`) return a JSON object on success and a JSON
//! object on failure for machine consumption.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use askama::Template;
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::{Form, Json};
use copythat_history::{HistoryFilter, ItemRow, JobRowId, JobSummary};
use serde::{Deserialize, Serialize};

use crate::error::RouteError;
use crate::server::ServerState;
use crate::templates::{
    ErrorTemplate, ItemView, JobDetailTemplate, JobView, JobsTemplate, LandingTemplate,
    RestoreTemplate, human_bytes, iso_from_ms,
};

const RECENT_JOBS_LIMIT: u32 = 25;

/// `GET /` — landing page. Renders the most recent
/// [`RECENT_JOBS_LIMIT`] jobs.
pub(crate) async fn landing(State(state): State<ServerState>) -> Result<Html<String>, RouteError> {
    let jobs = state
        .db
        .search(HistoryFilter {
            limit: Some(RECENT_JOBS_LIMIT),
            ..HistoryFilter::default()
        })
        .await?;
    let recent = jobs.into_iter().map(job_view).collect();
    Ok(render(&LandingTemplate { recent })?)
}

/// `GET /jobs` — paginated job list with kind/status/text filters.
pub(crate) async fn jobs_list(
    State(state): State<ServerState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, RouteError> {
    let kind = params.get("kind").cloned().filter(|s| !s.is_empty());
    let status = params.get("status").cloned().filter(|s| !s.is_empty());
    let text = params.get("text").cloned().filter(|s| !s.is_empty());

    let jobs = state
        .db
        .search(HistoryFilter {
            kind: kind.clone(),
            status: status.clone(),
            text: text.clone(),
            ..HistoryFilter::default()
        })
        .await?;
    let view = jobs.into_iter().map(job_view).collect();
    Ok(render(&JobsTemplate {
        jobs: view,
        kind_filter: kind.unwrap_or_default(),
        status_filter: status.unwrap_or_default(),
        text_filter: text.unwrap_or_default(),
    })?)
}

/// `GET /jobs/:id` — per-job detail page.
pub(crate) async fn job_detail(
    State(state): State<ServerState>,
    AxumPath(id): AxumPath<i64>,
) -> Result<Html<String>, RouteError> {
    let row_id = JobRowId(id);
    let job = state.db.get(row_id).await?.ok_or(RouteError::NotFound)?;
    let items = state.db.items_for(row_id).await?;
    let job_view = job_view(job);
    let item_views = items.into_iter().map(item_view).collect();
    Ok(render(&JobDetailTemplate {
        job: job_view,
        items: item_views,
    })?)
}

/// `GET /jobs/:id/files/*path` — download a single file as it was at
/// the time the original job ran. Pulls the manifest out of the
/// chunk store and reassembles the bytes; absent manifest → 404.
///
/// The manifest key search order is:
///
/// 1. The full destination path the engine recorded
///    (`<dst_root>/<rel>`) — what `ingest_file` writes when the
///    Phase 27 chunk-store sink is enabled.
/// 2. `"<job_id>/<rel>"` — fallback used by some early Phase 27
///    smoke tests; cheap to probe, harmless if absent.
pub(crate) async fn job_file_download(
    State(state): State<ServerState>,
    AxumPath((id, rel)): AxumPath<(i64, String)>,
) -> Result<Response, RouteError> {
    let row_id = JobRowId(id);
    let job = state.db.get(row_id).await?.ok_or(RouteError::NotFound)?;
    let decoded = percent_encoding::percent_decode_str(&rel)
        .decode_utf8_lossy()
        .into_owned();

    let chunk = state.chunk.clone();
    let dst_root = job.dst_root.clone();
    let bytes = tokio::task::spawn_blocking(move || -> Result<Option<Vec<u8>>, RouteError> {
        let candidate1 = candidate_key(&dst_root, &decoded);
        let candidate2 = format!("{}/{}", id, decoded);
        let manifest = match chunk.get_manifest(&candidate1)? {
            Some(m) => m,
            None => match chunk.get_manifest(&candidate2)? {
                Some(m) => m,
                None => return Ok(None),
            },
        };
        let mut buf = Vec::with_capacity(manifest.size as usize);
        for c in &manifest.chunks {
            let bytes = chunk.get(&c.hash)?.ok_or_else(|| {
                RouteError::Chunk(copythat_chunk::ChunkStoreError::MissingChunk {
                    hash: copythat_chunk::hex_of(&c.hash),
                })
            })?;
            buf.extend_from_slice(&bytes);
        }
        Ok(Some(buf))
    })
    .await
    .map_err(|_| RouteError::BadRequest("worker panicked".into()))??;

    match bytes {
        Some(bytes) => {
            let suggested_name = std::path::Path::new(&rel)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("download.bin")
                .to_string();
            Ok((
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/octet-stream".to_string()),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{suggested_name}\""),
                    ),
                ],
                bytes,
            )
                .into_response())
        }
        None => Err(RouteError::NotFound),
    }
}

fn candidate_key(dst_root: &std::path::Path, rel: &str) -> String {
    let p: PathBuf = dst_root.join(rel);
    p.to_string_lossy().into_owned()
}

/// `GET /restore` — restore-modal landing. Renders the form that
/// `POST /restore` consumes.
pub(crate) async fn restore_form(
    State(state): State<ServerState>,
) -> Result<Html<String>, RouteError> {
    let jobs = state
        .db
        .search(HistoryFilter {
            limit: Some(RECENT_JOBS_LIMIT),
            ..HistoryFilter::default()
        })
        .await?;
    let view = jobs.into_iter().map(job_view).collect();
    Ok(render(&RestoreTemplate { jobs: view })?)
}

#[derive(Debug, Deserialize)]
pub(crate) struct RestoreRequest {
    /// Job rowid the restore is sourced from. Stringly typed so the
    /// HTML form's `<input type="number">` and a JSON client both
    /// land on the same field.
    pub job_id: i64,
    pub path: String,
    pub timestamp_ms: i64,
}

#[derive(Debug, Serialize)]
pub(crate) struct RestoreResponse {
    /// Newly minted restore-job rowid.
    pub job_id: i64,
}

/// `POST /restore` — initiate a restore. Records a fresh `restore`
/// row in History and hands the rowid back so the caller can poll
/// `/jobs/<id>` for progress. Accepts either an
/// `application/x-www-form-urlencoded` body (HTML form) or a JSON
/// body (programmatic clients).
pub(crate) async fn restore_submit(
    State(state): State<ServerState>,
    request_body: ContentNegotiated<RestoreRequest>,
) -> Result<(StatusCode, Json<RestoreResponse>), RouteError> {
    let req = request_body.0;
    if req.path.is_empty() {
        return Err(RouteError::BadRequest("path is empty".into()));
    }
    if req.timestamp_ms < 0 {
        return Err(RouteError::BadRequest("timestamp_ms must be ≥ 0".into()));
    }
    let source = state
        .db
        .get(JobRowId(req.job_id))
        .await?
        .ok_or(RouteError::NotFound)?;

    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    let summary = JobSummary {
        row_id: 0,
        kind: "restore".into(),
        status: "running".into(),
        started_at_ms: now_ms,
        finished_at_ms: None,
        src_root: source.dst_root.clone(),
        dst_root: source.dst_root.clone(),
        total_bytes: 0,
        files_ok: 0,
        files_failed: 0,
        verify_algo: None,
        options_json: Some(
            serde_json::json!({
                "source_job_id": req.job_id,
                "path": req.path,
                "timestamp_ms": req.timestamp_ms,
            })
            .to_string(),
        ),
    };
    let new_id = state.db.record_start(&summary).await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(RestoreResponse { job_id: new_id.0 }),
    ))
}

/// `GET /sessions` — mobile-pairing dashboard. Wired by Phase 37 in
/// a follow-up; until then we surface 501 with a Link header so a
/// programmatic poller can detect the deferral.
pub(crate) async fn sessions_stub() -> Result<Response, RouteError> {
    Err(RouteError::NotImplemented)
}

/// `GET /metrics` — Prometheus exposition. Deferred to Phase 48.
pub(crate) async fn metrics_stub() -> Result<Response, RouteError> {
    Err(RouteError::NotImplemented)
}

// -------------------------------------------------------------------------
// Helpers + view-model conversion
// -------------------------------------------------------------------------

fn render(t: &impl Template) -> Result<Html<String>, askama::Error> {
    Ok(Html(t.render()?))
}

pub(crate) fn job_view(job: JobSummary) -> JobView {
    JobView {
        row_id: job.row_id,
        kind: job.kind,
        status: job.status,
        started_iso: iso_from_ms(job.started_at_ms),
        src_root: job.src_root.to_string_lossy().into_owned(),
        dst_root: job.dst_root.to_string_lossy().into_owned(),
        total_bytes_human: human_bytes(job.total_bytes),
        files_ok: job.files_ok,
        files_failed: job.files_failed,
    }
}

fn item_view(item: ItemRow) -> ItemView {
    ItemView {
        src: item.src.to_string_lossy().into_owned(),
        dst: item.dst.to_string_lossy().into_owned(),
        size_human: human_bytes(item.size),
        status: item.status,
        error_msg: item.error_msg,
    }
}

// -------------------------------------------------------------------------
// `RouteError` → `Response` glue
// -------------------------------------------------------------------------

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            RouteError::NotFound => (StatusCode::NOT_FOUND, "Not found.".to_string()),
            RouteError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            RouteError::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "This route is reserved for a future phase.".to_string(),
            ),
            RouteError::History(e) => {
                tracing::error!(error = ?e, "history error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "History store unavailable.".to_string(),
                )
            }
            RouteError::Chunk(e) => {
                tracing::error!(error = ?e, "chunk store error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Chunk store unavailable.".to_string(),
                )
            }
            RouteError::Render(e) => {
                tracing::error!(error = ?e, "template render error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Template render failure.".to_string(),
                )
            }
        };
        let body = ErrorTemplate {
            status_code: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or("error").to_string(),
            message,
        };
        match body.render() {
            Ok(html) => (status, Html(html)).into_response(),
            // Last-ditch fallback if even the error template fails.
            Err(_) => (status, "error").into_response(),
        }
    }
}

// -------------------------------------------------------------------------
// Content-type negotiation extractor for `POST /restore`.
//
// Accepts either `application/json` or
// `application/x-www-form-urlencoded`. axum 0.8 doesn't ship a
// per-content-type fan-out extractor; this small wrapper picks the
// right one off `Content-Type`.
// -------------------------------------------------------------------------

pub(crate) struct ContentNegotiated<T>(pub T);

impl<S, T> axum::extract::FromRequest<S> for ContentNegotiated<T>
where
    S: Send + Sync,
    T: for<'de> Deserialize<'de> + Send + 'static,
{
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        if content_type.starts_with("application/json") {
            let Json(value) = Json::<T>::from_request(req, state).await.map_err(|e| {
                RouteError::BadRequest(format!("malformed JSON body: {e}")).into_response()
            })?;
            Ok(ContentNegotiated(value))
        } else {
            // Form URL-encoded fallback. axum's `Form` extractor also
            // accepts requests with no Content-Type, which keeps cURL
            // one-liners working.
            let Form(value) = Form::<T>::from_request(req, state).await.map_err(|e| {
                RouteError::BadRequest(format!("malformed form body: {e}")).into_response()
            })?;
            Ok(ContentNegotiated(value))
        }
    }
}

// -------------------------------------------------------------------------
// Type-tag for `Arc` clones when handlers move state across tasks.
// -------------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) type SharedDb = Arc<copythat_history::History>;
