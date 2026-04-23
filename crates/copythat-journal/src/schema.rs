//! redb table definitions and on-disk codec.
//!
//! Three tables:
//!
//! - `jobs`: `u64 (JobRowId) -> JobRecord` (JSON)
//! - `files`: `(u64, u64) -> FileCheckpoint` (JSON)
//! - `seq`: `&str -> u64` — a one-row store for the row-id allocator.
//!
//! redb requires every key + value type to implement `redb::Value` /
//! `redb::Key`. We use the built-in primitive impls (u64, &str,
//! tuple) and a JSON-string wrapper for the structured payload — JSON
//! costs a small parse on read but lets the schema evolve without a
//! manual migration table.

use redb::TableDefinition;

pub(crate) const JOBS: TableDefinition<u64, &str> = TableDefinition::new("jobs");
pub(crate) const FILES: TableDefinition<(u64, u64), &str> = TableDefinition::new("files");
pub(crate) const SEQ: TableDefinition<&str, u64> = TableDefinition::new("seq");

pub(crate) const SEQ_KEY_NEXT_ROW_ID: &str = "next-row-id";
