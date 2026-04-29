//! Phase 42 Part B — Tauri IPC commands for the per-file rolling-
//! versions panel.
//!
//! Three read-only / advisory commands:
//!
//! - `list_versions(dst_path)` — fetch every recorded version for a
//!   destination path. Newest-first.
//! - `select_versions_to_prune(dst_path, policy)` — render the
//!   retention math against the current set without committing —
//!   the Settings panel uses this to preview "applying this policy
//!   would drop N versions" before the user confirms.
//! - `prune_versions(dst_path, policy)` — actually delete the rows
//!   the math selects. Returns the number deleted.
//!
//! The actual snapshot-on-overwrite hook (which inserts new rows in
//! the first place) lands when a downstream `VersioningSink`
//! implementation wires `copythat-history` + `copythat-chunk` —
//! tracked as a Part C follow-up. These IPC commands are read-only
//! today and become useful the moment a real sink starts populating
//! the table.

use std::path::PathBuf;

use copythat_core::versioning::{GfsPolicy, RetentionPolicy, VersionEntry, select_for_pruning};
use copythat_history::VersionRowId;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

/// Wire shape for `copythat_history::VersionRecord`. `manifestBlake3`
/// is hex-encoded for a JSON-friendly surface.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionRecordDto {
    pub row_id: i64,
    pub dst_path: String,
    pub ts_ms: i64,
    pub manifest_blake3_hex: String,
    pub size: u64,
    pub retained_until_ms: Option<i64>,
    pub triggered_by_job_id: Option<i64>,
}

/// Wire shape for `copythat_core::versioning::RetentionPolicy`.
/// Tagged-enum serde so the Svelte side can emit the variant by
/// name + payload.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RetentionPolicyDto {
    None,
    LastN { n: u32 },
    OlderThanDays { days: u32 },
    Gfs(GfsPolicyDto),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GfsPolicyDto {
    #[serde(default)]
    pub keep_hourly: u32,
    #[serde(default)]
    pub keep_daily: u32,
    #[serde(default)]
    pub keep_weekly: u32,
    #[serde(default)]
    pub keep_monthly: u32,
}

impl From<RetentionPolicyDto> for RetentionPolicy {
    fn from(dto: RetentionPolicyDto) -> Self {
        match dto {
            RetentionPolicyDto::None => RetentionPolicy::None,
            RetentionPolicyDto::LastN { n } => RetentionPolicy::LastN(n),
            RetentionPolicyDto::OlderThanDays { days } => RetentionPolicy::OlderThanDays(days),
            RetentionPolicyDto::Gfs(g) => RetentionPolicy::Gfs(GfsPolicy {
                keep_hourly: g.keep_hourly,
                keep_daily: g.keep_daily,
                keep_weekly: g.keep_weekly,
                keep_monthly: g.keep_monthly,
            }),
        }
    }
}

fn hex_of(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// `list_versions(dst_path)` — pull every recorded version for the
/// destination, newest-first. Returns an empty list when the History
/// database is unavailable (the runtime can be launched with
/// `--no-history`); the Settings panel renders the empty-state in
/// that case.
#[tauri::command]
pub async fn list_versions(
    state: tauri::State<'_, AppState>,
    dst_path: String,
) -> Result<Vec<VersionRecordDto>, String> {
    let Some(history) = state.history.as_ref() else {
        return Ok(Vec::new());
    };
    let rows = history
        .versions_for_path(PathBuf::from(&dst_path))
        .await
        .map_err(|e| format!("versions_for_path: {e}"))?;
    Ok(rows
        .into_iter()
        .map(|r| VersionRecordDto {
            row_id: r.row_id,
            dst_path: r.dst_path.to_string_lossy().into_owned(),
            ts_ms: r.ts_ms,
            manifest_blake3_hex: hex_of(&r.manifest_blake3),
            size: r.size,
            retained_until_ms: r.retained_until_ms,
            triggered_by_job_id: r.triggered_by_job_id,
        })
        .collect())
}

/// `select_versions_to_prune(dst_path, policy)` — preview the
/// retention pass without committing. Returns the row ids the
/// matching `prune_versions` call would delete, plus a count for
/// the UI summary line.
#[tauri::command]
pub async fn select_versions_to_prune(
    state: tauri::State<'_, AppState>,
    dst_path: String,
    policy: RetentionPolicyDto,
    now_ms: i64,
) -> Result<Vec<i64>, String> {
    let Some(history) = state.history.as_ref() else {
        return Ok(Vec::new());
    };
    let rows = history
        .versions_for_path(PathBuf::from(&dst_path))
        .await
        .map_err(|e| format!("versions_for_path: {e}"))?;
    let entries: Vec<VersionEntry> = rows
        .iter()
        .map(|r| VersionEntry {
            row_id: r.row_id,
            ts_ms: r.ts_ms,
        })
        .collect();
    let policy: RetentionPolicy = policy.into();
    Ok(select_for_pruning(&entries, &policy, now_ms))
}

/// `prune_versions(dst_path, policy)` — apply the retention math +
/// delete the selected rows. Returns the number of rows actually
/// removed (the UI surfaces this so the operator can sanity-check
/// the policy).
#[tauri::command]
pub async fn prune_versions(
    state: tauri::State<'_, AppState>,
    dst_path: String,
    policy: RetentionPolicyDto,
    now_ms: i64,
) -> Result<u64, String> {
    let Some(history) = state.history.as_ref() else {
        return Ok(0);
    };
    let rows = history
        .versions_for_path(PathBuf::from(&dst_path))
        .await
        .map_err(|e| format!("versions_for_path: {e}"))?;
    let entries: Vec<VersionEntry> = rows
        .iter()
        .map(|r| VersionEntry {
            row_id: r.row_id,
            ts_ms: r.ts_ms,
        })
        .collect();
    let policy: RetentionPolicy = policy.into();
    let drop_ids: Vec<VersionRowId> = select_for_pruning(&entries, &policy, now_ms)
        .into_iter()
        .map(VersionRowId)
        .collect();
    history
        .delete_versions(drop_ids)
        .await
        .map_err(|e| format!("delete_versions: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_of_round_trips_zero_and_max_bytes() {
        assert_eq!(hex_of(&[0u8; 32]), "0".repeat(64));
        let mut all_ff = [0u8; 32];
        for b in all_ff.iter_mut() {
            *b = 0xFF;
        }
        assert_eq!(hex_of(&all_ff), "f".repeat(64));
    }

    #[test]
    fn retention_policy_dto_round_trip_through_serde() {
        let cases = [
            (r#"{"kind":"none"}"#, RetentionPolicy::None),
            (r#"{"kind":"last-n","n":5}"#, RetentionPolicy::LastN(5)),
            (
                r#"{"kind":"older-than-days","days":30}"#,
                RetentionPolicy::OlderThanDays(30),
            ),
        ];
        for (json, expected) in cases {
            let dto: RetentionPolicyDto = serde_json::from_str(json).expect("ser");
            let p: RetentionPolicy = dto.into();
            assert_eq!(p, expected);
        }
        // GFS variant separately because it nests.
        let gfs_json =
            r#"{"kind":"gfs","keepHourly":24,"keepDaily":7,"keepWeekly":4,"keepMonthly":12}"#;
        let dto: RetentionPolicyDto = serde_json::from_str(gfs_json).expect("ser");
        let p: RetentionPolicy = dto.into();
        match p {
            RetentionPolicy::Gfs(g) => {
                assert_eq!(g.keep_hourly, 24);
                assert_eq!(g.keep_daily, 7);
                assert_eq!(g.keep_weekly, 4);
                assert_eq!(g.keep_monthly, 12);
            }
            _ => panic!("expected Gfs"),
        }
    }
}
