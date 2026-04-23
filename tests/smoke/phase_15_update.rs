//! Phase 15 smoke test — auto-update manifest pre-fetch.
//!
//! Spins up a minimal HTTP server on a loopback port, serves a fake
//! `99.0.0` manifest, and asserts the updater module:
//!
//! 1. Resolves + connects within the 10 s bound mandated by the
//!    phase spec.
//! 2. Parses the canonical manifest JSON (the same schema Tauri's
//!    updater plugin consumes).
//! 3. Reports the advertised version as strictly newer than the
//!    running binary (`CARGO_PKG_VERSION`, currently `0.1.0`).
//! 4. Applies the 24 h throttle on `UpdaterSettings::due_for_check`.
//!
//! We deliberately don't boot Tauri here — the real plugin covers
//! signature verification + artifact install; this smoke test covers
//! the pre-fetch + comparison path the UI uses to surface the
//! "Update available" banner, which is the user-observable claim of
//! the phase.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use copythat_settings::UpdaterSettings;
use copythat_ui_lib::updater::{
    UpdateManifest, fetch_manifest_http, format_endpoint, is_strictly_newer,
};

const MANIFEST_BODY: &str = r#"{
  "version": "99.0.0",
  "notes": "Smoke-test fixture. Everything works.",
  "pub_date": "2026-04-22T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "stub-signature-not-verified-by-smoke",
      "url": "http://127.0.0.1:1/dummy.zip"
    },
    "darwin-aarch64": {
      "signature": "stub-signature-not-verified-by-smoke",
      "url": "http://127.0.0.1:1/dummy.tar.gz"
    },
    "linux-x86_64": {
      "signature": "stub-signature-not-verified-by-smoke",
      "url": "http://127.0.0.1:1/dummy.AppImage"
    }
  }
}"#;

/// Spawn an HTTP listener that replies to `GET /manifest.json` (or
/// any path) with [`MANIFEST_BODY`]. Returns the bound URL as
/// `http://127.0.0.1:<port>/manifest.json` and a `JoinHandle` the
/// test can drop to tear the thread down.
fn spawn_manifest_server() -> (String, mpsc::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().unwrap().port();
    listener
        .set_nonblocking(true)
        .expect("set_nonblocking on listener");
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
    let url = format!("http://127.0.0.1:{port}/manifest.json");

    thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if shutdown_rx.try_recv().is_ok() || Instant::now() > deadline {
                return;
            }
            match listener.accept() {
                Ok((stream, _)) => {
                    let _ = handle_one(stream);
                    // One request is enough for the smoke test; keep the
                    // loop alive to accept any late retries but don't
                    // block the shutdown channel.
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(_) => return,
            }
        }
    });

    (url, shutdown_tx)
}

fn handle_one(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    // Drain the request line + headers. We don't actually care what
    // path the client asks for — every path answers with the fixture.
    let mut reader = BufReader::new(stream.try_clone()?);
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }

    let body = MANIFEST_BODY.as_bytes();
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len(),
    );
    stream.write_all(headers.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

#[test]
fn fetches_and_parses_manifest_within_time_bound() {
    let (url, _shutdown) = spawn_manifest_server();

    let start = Instant::now();
    let manifest: UpdateManifest = fetch_manifest_http(&url, Duration::from_secs(10))
        .expect("fetch_manifest_http should succeed against local fixture");
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(10),
        "manifest fetch should resolve well under 10 s; got {elapsed:?}"
    );

    assert_eq!(manifest.version, "99.0.0");
    assert!(
        manifest.notes.contains("Smoke-test"),
        "notes should round-trip; got {:?}",
        manifest.notes
    );
    assert_eq!(
        manifest.platforms.len(),
        3,
        "fixture declares three platforms"
    );
    for key in ["windows-x86_64", "darwin-aarch64", "linux-x86_64"] {
        assert!(
            manifest.platforms.contains_key(key),
            "platform {key} missing"
        );
    }
}

#[test]
fn manifest_version_is_strictly_newer_than_running_binary() {
    let (url, _shutdown) = spawn_manifest_server();
    let manifest = fetch_manifest_http(&url, Duration::from_secs(10))
        .expect("fetch_manifest_http should succeed");
    let current = env!("CARGO_PKG_VERSION");
    assert!(
        is_strictly_newer(&manifest.version, current),
        "99.0.0 should beat running version {current:?}"
    );
}

#[test]
fn format_endpoint_threads_channel_target_arch_version() {
    // Mirrors the exact substitution the production path performs so
    // a refactor of `format_endpoint` that drops a placeholder is
    // caught here rather than surfacing only in a live release build.
    let template =
        "https://releases.copythat.app/{{channel}}/{{target}}-{{arch}}/{{current_version}}.json";
    let url = format_endpoint(template, "beta", "windows", "x86_64", "0.1.0");
    assert_eq!(
        url,
        "https://releases.copythat.app/beta/windows-x86_64/0.1.0.json"
    );
}

#[test]
fn due_for_check_respects_24h_throttle() {
    // Default — never checked → due.
    let u = UpdaterSettings::default();
    assert!(u.due_for_check(1_748_736_000));

    // Just checked — not due for roughly 24 h.
    let mut checked = u.clone();
    checked.last_check_unix_secs = 1_748_736_000;
    assert!(!checked.due_for_check(1_748_736_000));
    assert!(!checked.due_for_check(1_748_736_000 + 86_399));
    assert!(checked.due_for_check(1_748_736_000 + 86_400));
}
