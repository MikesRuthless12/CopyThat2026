//! `copythat history [--filter X] [--limit N]` — read-only summary
//! of the persistent history database.

use std::sync::Arc;

use copythat_history::{History, HistoryFilter};

use crate::ExitCode;
use crate::cli::{GlobalArgs, HistoryArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: HistoryArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let history = match History::open_default().await {
        Ok(h) => h,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("open history: {e}"),
                code: ExitCode::GenericError.as_u8(),
            });
            return ExitCode::GenericError;
        }
    };

    let filter = HistoryFilter {
        text: args.filter.clone(),
        limit: Some(args.limit),
        ..HistoryFilter::default()
    };

    let rows = match history.search(filter).await {
        Ok(r) => r,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("search history: {e}"),
                code: ExitCode::GenericError.as_u8(),
            });
            return ExitCode::GenericError;
        }
    };

    for row in &rows {
        let value = serde_json::json!({
            "row_id": row.row_id,
            "kind": row.kind,
            "status": row.status,
            "started_at_ms": row.started_at_ms,
            "finished_at_ms": row.finished_at_ms,
            "src_root": row.src_root,
            "dst_root": row.dst_root,
            "total_bytes": row.total_bytes,
            "files_ok": row.files_ok,
            "files_failed": row.files_failed,
            "verify_algo": row.verify_algo,
        });
        let _ = writer.emit(JsonEventKind::ConfigValue {
            key: format!("history.row.{}", row.row_id),
            value,
        });
        let _ = writer.human(&format!(
            "[{}] {} {} bytes={} files={} ({})",
            row.row_id,
            row.kind,
            row.src_root.display(),
            row.total_bytes,
            row.files_ok,
            row.status,
        ));
    }

    let _ = writer.human(&format!("history: {} row(s)", rows.len()));
    ExitCode::Success
}
