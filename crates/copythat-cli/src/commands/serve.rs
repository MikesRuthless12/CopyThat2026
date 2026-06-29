//! `copythat serve` — Phase 48 headless server mode.
//!
//! Translates the CLI flags into a [`copythat_server::ServerConfig`],
//! starts the server, prints the bound address + metrics URL, then blocks
//! until Ctrl-C and shuts down gracefully.

use std::sync::Arc;

use copythat_server::{AuthMode, Protocol, ServerConfig};

use crate::ExitCode;
use crate::cli::{GlobalArgs, ServeArgs};
use crate::output::OutputWriter;

/// Build a [`ServerConfig`] from parsed `serve` flags. Pure + testable.
fn build_config(args: ServeArgs) -> ServerConfig {
    let mut protocols = Vec::new();
    if args.webdav {
        protocols.push(Protocol::WebDav);
    }
    if args.http {
        protocols.push(Protocol::Http);
    }
    if args.s3 {
        protocols.push(Protocol::S3);
    }
    if args.sftp {
        protocols.push(Protocol::Sftp);
    }
    // Default to WebDAV when no protocol flag was given.
    if protocols.is_empty() {
        protocols.push(Protocol::WebDav);
    }
    // clap guarantees token is exclusive with user/password, and that
    // user + password arrive together.
    let auth = match (args.token, args.user, args.password) {
        (Some(token), _, _) => AuthMode::Bearer { token },
        (None, Some(user), Some(password)) => AuthMode::Basic { user, password },
        _ => AuthMode::None,
    };
    ServerConfig {
        bind_addr: args.bind,
        protocols,
        auth,
        root: args.root,
        readonly: args.readonly,
    }
}

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: ServeArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    // Pulled out before `build_config` consumes `args`.
    let otel_endpoint = args.otel_endpoint.clone();
    let config = build_config(args);
    let protocols: Vec<&str> = config.protocols.iter().map(|p| p.label()).collect();
    let readonly = config.readonly;
    // SFTP speaks SSH (no HTTP `/metrics`); everything else is HTTP-served.
    let is_sftp = config.protocols.contains(&Protocol::Sftp);

    // Wire OpenTelemetry trace export before serving, and hold the guard for
    // the whole server lifetime so buffered spans flush on Ctrl-C shutdown.
    // Export is best-effort: a failed pipeline build is logged, not fatal.
    let _otel_guard = match otel_endpoint {
        Some(endpoint) => {
            let cfg = copythat_server::OtelConfig {
                endpoint,
                enabled: true,
            };
            match copythat_server::install_otel(&cfg) {
                Ok(guard) => Some(guard),
                Err(e) => {
                    eprintln!("copythat: OpenTelemetry trace export disabled: {e}");
                    None
                }
            }
        }
        None => None,
    };

    match copythat_server::serve(config).await {
        Ok(handle) => {
            let addr = handle.local_addr();
            let ro = if readonly { " (read-only)" } else { "" };
            if is_sftp {
                let _ = writer.human(&format!(
                    "CopyThat serving [{}]{ro} on sftp://{addr}",
                    protocols.join(", ")
                ));
            } else {
                let _ = writer.human(&format!(
                    "CopyThat serving [{}]{ro} on http://{addr}  (metrics: http://{addr}/metrics)",
                    protocols.join(", ")
                ));
            }
            // Loud warning if exposed to the network with no auth.
            if copythat_server::exposes_unauthenticated(&addr, &handle.config().auth) {
                let access = if readonly { "read-only" } else { "read/write" };
                eprintln!(
                    "copythat: WARNING — serving on {addr} with NO authentication; any host \
                     that can reach this address has {access} access to the served directory. \
                     Pass --token or --user/--password to require auth."
                );
            }
            let _ = writer.human("Press Ctrl-C to stop.");
            if tokio::signal::ctrl_c().await.is_err() {
                eprintln!("copythat: failed to install Ctrl-C handler; shutting down");
            }
            let _ = writer.human("Shutting down…");
            handle.shutdown().await;
            ExitCode::Success
        }
        Err(e) => {
            eprintln!("copythat: serve failed: {e}");
            ExitCode::GenericError
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_args() -> ServeArgs {
        ServeArgs {
            webdav: false,
            http: false,
            s3: false,
            sftp: false,
            bind: "127.0.0.1:0".into(),
            root: ".".into(),
            readonly: false,
            token: None,
            user: None,
            password: None,
            otel_endpoint: None,
        }
    }

    #[test]
    fn defaults_to_webdav_no_auth() {
        let cfg = build_config(base_args());
        assert_eq!(cfg.protocols, vec![Protocol::WebDav]);
        assert_eq!(cfg.auth, AuthMode::None);
        assert!(!cfg.readonly);
    }

    #[test]
    fn bearer_token_maps_to_bearer_auth() {
        let cfg = build_config(ServeArgs {
            token: Some("tok".into()),
            readonly: true,
            ..base_args()
        });
        assert_eq!(
            cfg.auth,
            AuthMode::Bearer {
                token: "tok".into()
            }
        );
        assert!(cfg.readonly);
    }

    #[test]
    fn user_password_maps_to_basic_and_collects_protocols() {
        let cfg = build_config(ServeArgs {
            user: Some("u".into()),
            password: Some("p".into()),
            webdav: true,
            http: true,
            ..base_args()
        });
        assert_eq!(
            cfg.auth,
            AuthMode::Basic {
                user: "u".into(),
                password: "p".into()
            }
        );
        assert_eq!(cfg.protocols, vec![Protocol::WebDav, Protocol::Http]);
    }
}
