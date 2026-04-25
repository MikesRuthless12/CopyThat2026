//! Phase 16 release orchestrator.
//!
//! `xtask release` drives the free-first packaging path locally —
//! the same one `.github/workflows/release.yml` runs on each hosted
//! runner. It deliberately avoids any paid signing service: macOS
//! gets an ad-hoc codesign (`APPLE_SIGNING_IDENTITY=-`), Windows
//! ships unsigned, and Linux relies on the optional self-generated
//! GPG key if one is in the caller's keyring.
//!
//! Subcommand shape:
//!     xtask release                  # build for current host only
//!     xtask release --bundles msi,nsis
//!     xtask release --target aarch64-apple-darwin
//!     xtask release --dry-run        # print what we would do, do nothing
//!
//! The tauri bundler is invoked through pnpm to match the CI
//! workflow exactly; if pnpm is missing we print a useful error
//! rather than silently falling back to npm/yarn.

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::repo_root;

#[derive(Debug, Default)]
struct ReleaseArgs {
    bundles: Option<String>,
    target: Option<String>,
    dry_run: bool,
}

/// Parse `args` (already advanced past the `release` token) into a
/// typed record. Unknown flags fail the subcommand instead of being
/// ignored — packaging is the wrong place to silently drop options.
fn parse_args<I, S>(args: I) -> Result<ReleaseArgs, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut out = ReleaseArgs::default();
    let mut iter = args.into_iter();
    while let Some(raw) = iter.next() {
        let s = raw.as_ref().to_string_lossy().into_owned();
        match s.as_str() {
            "--bundles" => {
                out.bundles = iter
                    .next()
                    .map(|v| v.as_ref().to_string_lossy().into_owned());
                if out.bundles.is_none() {
                    return Err("--bundles requires a value (e.g. msi,nsis)".into());
                }
            }
            "--target" => {
                out.target = iter
                    .next()
                    .map(|v| v.as_ref().to_string_lossy().into_owned());
                if out.target.is_none() {
                    return Err("--target requires a value (e.g. aarch64-apple-darwin)".into());
                }
            }
            "--dry-run" => out.dry_run = true,
            other => return Err(format!("unknown flag: `{other}`")),
        }
    }
    Ok(out)
}

/// Pick a reasonable default bundle list for the host OS. Matches
/// the matrix in `.github/workflows/release.yml`.
fn default_bundles() -> &'static str {
    if cfg!(target_os = "windows") {
        "msi,nsis"
    } else if cfg!(target_os = "macos") {
        "app,dmg"
    } else {
        "deb,rpm,appimage"
    }
}

pub(crate) fn run<I, S>(args: I) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args = parse_args(args)?;
    let root = repo_root().ok_or("could not locate repo root (Cargo.toml + locales/)")?;
    let ui_dir: PathBuf = root.join("apps").join("copythat-ui");
    if !ui_dir.is_dir() {
        return Err(format!(
            "expected apps/copythat-ui at {}, but it does not exist",
            ui_dir.display()
        ));
    }

    // `unwrap_or` instead of `unwrap_or_else(default_bundles)` — the
    // function-pointer form locks the return lifetime to `'static`,
    // which then forces `args.bundles` to outlive the function. Eager
    // evaluation is fine: `default_bundles()` is a const-fn-equivalent
    // branch over `cfg!` flags.
    let fallback_bundles = default_bundles();
    let bundles = args.bundles.as_deref().unwrap_or(fallback_bundles);

    // pnpm is our contracted package manager (see apps/copythat-ui
    // `pnpm-lock.yaml`). If the dev forgot to install it, surfacing
    // "pnpm: command not found" one layer deep is noise — catch it
    // at the door with a specific message.
    if which("pnpm").is_none() {
        return Err(
            "pnpm is required for `xtask release` (install via https://pnpm.io/installation, then retry)"
                .into(),
        );
    }

    let mut cmd = Command::new("pnpm");
    cmd.current_dir(&ui_dir);
    cmd.arg("tauri").arg("build");
    if let Some(tgt) = args.target.as_deref() {
        cmd.arg("--target").arg(tgt);
    }
    cmd.arg("--bundles").arg(bundles);

    // Ad-hoc codesign on macOS — the dash identity keeps Gatekeeper
    // satisfied without requiring a paid Apple Developer cert. Only
    // set on macOS hosts (or when the target explicitly is darwin);
    // setting it unconditionally on Linux/Windows risks tripping
    // future Tauri logic that reads the var even when not signing.
    let target = args.target.as_deref().unwrap_or("");
    let needs_apple_signing = cfg!(target_os = "macos") || target.contains("darwin");
    if needs_apple_signing {
        cmd.env("APPLE_SIGNING_IDENTITY", "-");
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    eprintln!(
        "[xtask release] running: cd {} && pnpm tauri build {}{}--bundles {}{}",
        ui_dir.display(),
        if args.target.is_some() {
            "--target "
        } else {
            ""
        },
        args.target.as_deref().unwrap_or(""),
        bundles,
        if args.dry_run { " (DRY RUN)" } else { "" },
    );
    if args.dry_run {
        return Ok(());
    }

    let status = cmd
        .status()
        .map_err(|e| format!("failed to spawn pnpm tauri build: {e}"))?;
    if !status.success() {
        return Err(format!("pnpm tauri build failed with status {status}"));
    }
    eprintln!("[xtask release] done");
    Ok(())
}

/// Minimal `which` lookup: returns `Some(PathBuf)` if `name` resolves
/// on the process PATH, `None` otherwise. Avoids pulling in the
/// `which` crate just for one probe.
fn which(name: &str) -> Option<PathBuf> {
    let exts: Vec<String> = if cfg!(windows) {
        env::var("PATHEXT")
            .unwrap_or_else(|_| ".EXE;.CMD;.BAT".into())
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![String::new()]
    };
    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        for ext in &exts {
            let candidate = dir.join(format!("{name}{ext}"));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_args() {
        let args = parse_args(Vec::<&str>::new()).unwrap();
        assert!(args.bundles.is_none());
        assert!(args.target.is_none());
        assert!(!args.dry_run);
    }

    #[test]
    fn parses_bundles_and_target() {
        let args = parse_args([
            "--bundles",
            "msi,nsis",
            "--target",
            "x86_64-pc-windows-msvc",
        ])
        .unwrap();
        assert_eq!(args.bundles.as_deref(), Some("msi,nsis"));
        assert_eq!(args.target.as_deref(), Some("x86_64-pc-windows-msvc"));
        assert!(!args.dry_run);
    }

    #[test]
    fn parses_dry_run() {
        let args = parse_args(["--dry-run"]).unwrap();
        assert!(args.dry_run);
    }

    #[test]
    fn rejects_unknown_flag() {
        let err = parse_args(["--no-such-flag"]).unwrap_err();
        assert!(err.contains("unknown flag"), "got {err:?}");
    }

    #[test]
    fn rejects_bundles_without_value() {
        let err = parse_args(["--bundles"]).unwrap_err();
        assert!(err.contains("requires a value"), "got {err:?}");
    }

    #[test]
    fn default_bundles_is_non_empty() {
        assert!(!default_bundles().is_empty());
    }
}
