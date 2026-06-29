//! Phase 48 smoke — server config/protocol serde, webhook payload shapes,
//! Prometheus exposition, and a real WebDAV PUT/GET round-trip that bumps
//! the `/metrics` counters.

use copythat_server::{
    AuthMode, JobNotification, Metrics, OtelConfig, Protocol, ServerConfig, ServerError,
    WebhookTarget, format_webhook_payload, serve,
};

#[test]
fn config_and_protocol_round_trip() {
    let cfg = ServerConfig {
        bind_addr: "0.0.0.0:9000".into(),
        protocols: vec![Protocol::WebDav, Protocol::Http],
        auth: AuthMode::Bearer {
            token: "secret".into(),
        },
        root: "/srv/data".into(),
        readonly: true,
    };
    let json = serde_json::to_string(&cfg).unwrap();
    let back: ServerConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(cfg, back);
    // snake_case wire form for the protocol enum + tagged auth mode.
    assert!(
        json.contains("web_dav"),
        "expected snake_case protocol: {json}"
    );
    assert!(json.contains("\"mode\":\"bearer\""), "tagged auth: {json}");

    let otel: OtelConfig =
        serde_json::from_str(r#"{"endpoint":"http://localhost:4317","enabled":true}"#).unwrap();
    assert!(otel.enabled && otel.endpoint.contains("4317"));
}

#[test]
fn protocol_display_labels() {
    assert_eq!(Protocol::WebDav.to_string(), "WebDAV");
    assert_eq!(Protocol::Sftp.to_string(), "SFTP");
    assert_eq!(Protocol::S3.to_string(), "S3");
}

#[test]
fn webhook_payloads_carry_the_right_keys() {
    let ev = JobNotification {
        kind: "job_completed".into(),
        title: "Copy done".into(),
        body: "100 files".into(),
        ok: true,
    };
    let slack = format_webhook_payload(WebhookTarget::Slack, &ev);
    assert!(slack.get("text").and_then(|v| v.as_str()).is_some());

    let discord = format_webhook_payload(WebhookTarget::Discord, &ev);
    assert!(discord.get("content").is_some());

    let ntfy = format_webhook_payload(WebhookTarget::Ntfy, &ev);
    assert_eq!(
        ntfy.get("topic").and_then(|v| v.as_str()),
        Some("job_completed")
    );
    assert!(ntfy.get("message").is_some());

    let push = format_webhook_payload(WebhookTarget::Pushover, &ev);
    assert!(
        push.get("token").is_some() && push.get("user").is_some() && push.get("message").is_some()
    );
}

#[test]
fn prometheus_exposition_is_well_formed() {
    let m = Metrics {
        jobs_total: 3,
        files_copied_total: 100,
        bytes_copied_total: 4096,
        errors_total: 1,
        active_jobs: 2,
    };
    let s = m.render_prometheus();
    assert!(s.contains("# TYPE copythat_jobs_total counter"));
    assert!(s.contains("copythat_jobs_total 3"));
    assert!(s.contains("# TYPE copythat_active_jobs gauge"));
    assert!(s.contains("copythat_active_jobs 2"));
    for name in ["copythat_jobs_total", "copythat_active_jobs"] {
        let help = s.find(&format!("# HELP {name} ")).unwrap();
        let typ = s.find(&format!("# TYPE {name} ")).unwrap();
        let sample = s.find(&format!("\n{name} ")).unwrap();
        assert!(
            help < typ && typ < sample,
            "HELP/TYPE must precede the sample for {name}"
        );
    }
}

/// SFTP (own SSH transport) and S3 (a distinct REST/XML API) aren't served
/// yet, so `serve` reports them as not-yet-implemented rather than silently
/// downgrading an advertised protocol to the WebDAV subset.
#[tokio::test]
async fn unsupported_protocols_are_deferred() {
    for proto in [Protocol::Sftp, Protocol::S3] {
        let cfg = ServerConfig {
            bind_addr: "127.0.0.1:0".into(),
            protocols: vec![proto],
            ..Default::default()
        };
        match serve(cfg).await {
            Err(ServerError::NotImplemented { protocol }) => assert_eq!(protocol, proto),
            other => panic!("expected NotImplemented for {proto}, got {other:?}"),
        }
    }
    // A mix advertising an unsupported protocol is rejected, not silently
    // downgraded to the served subset.
    let mixed = ServerConfig {
        bind_addr: "127.0.0.1:0".into(),
        protocols: vec![Protocol::WebDav, Protocol::S3],
        ..Default::default()
    };
    match serve(mixed).await {
        Err(ServerError::NotImplemented {
            protocol: Protocol::S3,
        }) => {}
        other => panic!("expected S3 NotImplemented for mixed config, got {other:?}"),
    }
}

/// The spec's acceptance test: PUT a 1 MiB file over WebDAV, GET it back
/// byte-equal, then confirm `/metrics` counted the write.
#[tokio::test]
async fn webdav_put_get_roundtrip_and_metrics() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = ServerConfig {
        bind_addr: "127.0.0.1:0".into(),
        protocols: vec![Protocol::WebDav],
        auth: AuthMode::None,
        root: dir.path().to_path_buf(),
        readonly: false,
    };
    let handle = serve(cfg).await.expect("serve should bind");
    let base = format!("http://{}", handle.local_addr());
    let client = reqwest::Client::new();

    // Deterministic 1 MiB payload.
    let payload: Vec<u8> = (0..1024usize * 1024).map(|i| (i % 251) as u8).collect();

    let put = client
        .put(format!("{base}/file.bin"))
        .body(payload.clone())
        .send()
        .await
        .unwrap();
    assert!(put.status().is_success(), "PUT status {}", put.status());

    let got = client.get(format!("{base}/file.bin")).send().await.unwrap();
    assert!(got.status().is_success(), "GET status {}", got.status());
    let body = got.bytes().await.unwrap();
    assert_eq!(
        body.as_ref(),
        payload.as_slice(),
        "GET body must byte-match the PUT payload"
    );

    // The file really landed under the served root.
    assert!(dir.path().join("file.bin").is_file());

    // `/metrics` counted the write.
    let metrics = client
        .get(format!("{base}/metrics"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(metrics.contains("# TYPE copythat_jobs_total counter"));
    let jobs_line = metrics
        .lines()
        .find(|l| l.starts_with("copythat_jobs_total "))
        .expect("jobs_total sample present");
    let n: u64 = jobs_line.rsplit(' ').next().unwrap().parse().unwrap();
    assert!(n >= 1, "expected >=1 job after PUT, got {n}");
    let bytes_line = metrics
        .lines()
        .find(|l| l.starts_with("copythat_bytes_copied_total "))
        .unwrap();
    let bytes: u64 = bytes_line.rsplit(' ').next().unwrap().parse().unwrap();
    assert_eq!(bytes, payload.len() as u64, "bytes_copied_total");

    handle.shutdown().await;
}

/// A read-only server rejects writes with 403 before they touch disk.
#[tokio::test]
async fn readonly_rejects_writes() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = ServerConfig {
        bind_addr: "127.0.0.1:0".into(),
        protocols: vec![Protocol::WebDav],
        readonly: true,
        root: dir.path().to_path_buf(),
        ..Default::default()
    };
    let handle = serve(cfg).await.expect("serve");
    let base = format!("http://{}", handle.local_addr());
    let client = reqwest::Client::new();

    let put = client
        .put(format!("{base}/nope.bin"))
        .body(vec![0u8; 16])
        .send()
        .await
        .unwrap();
    assert_eq!(put.status().as_u16(), 403, "read-only must reject PUT");
    assert!(!dir.path().join("nope.bin").exists());

    handle.shutdown().await;
}

/// Bearer auth: a missing/wrong token is 401, the right one passes.
#[tokio::test]
async fn bearer_auth_is_enforced() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = ServerConfig {
        bind_addr: "127.0.0.1:0".into(),
        protocols: vec![Protocol::WebDav],
        auth: AuthMode::Bearer {
            token: "s3cr3t".into(),
        },
        root: dir.path().to_path_buf(),
        ..Default::default()
    };
    let handle = serve(cfg).await.expect("serve");
    let base = format!("http://{}", handle.local_addr());
    let client = reqwest::Client::new();

    // No credential → 401.
    let anon = client.get(format!("{base}/x")).send().await.unwrap();
    assert_eq!(anon.status().as_u16(), 401);

    // Correct bearer → reaches the filesystem (404 for a missing file is
    // a "passed auth" outcome).
    let authed = client
        .get(format!("{base}/x"))
        .bearer_auth("s3cr3t")
        .send()
        .await
        .unwrap();
    assert_ne!(authed.status().as_u16(), 401, "valid token must pass auth");

    // `/metrics` stays open for scrapers even with auth on.
    let metrics = client.get(format!("{base}/metrics")).send().await.unwrap();
    assert!(metrics.status().is_success(), "metrics open for scraping");

    handle.shutdown().await;
}
