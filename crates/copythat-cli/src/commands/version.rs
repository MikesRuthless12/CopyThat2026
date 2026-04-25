//! `copythat version [--json]`. Prints the workspace version + crate
//! identity. The JSON event round-trips cleanly through `serde_json`
//! so the smoke test can assert structural integrity.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::GlobalArgs;
use crate::output::{JsonEventKind, OutputWriter};

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) async fn run(_global: &GlobalArgs, writer: Arc<OutputWriter>) -> ExitCode {
    let _ = writer.emit(JsonEventKind::Version {
        version: CRATE_VERSION.into(),
        crate_name: CRATE_NAME.into(),
        rustc_known_at_compile: true,
    });
    let _ = writer.human(&format!("{CRATE_NAME} {CRATE_VERSION}"));
    ExitCode::Success
}
