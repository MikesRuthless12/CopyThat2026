//! Phase 40 smoke — cloud-VM offload helper template renderers.
//!
//! Six cases, one per renderer plus two integration checks:
//!
//! 1. `render_cloudinit_template` for an S3 → S3 pair contains the
//!    canonical `aws s3 cp` line, both bucket URIs, and the
//!    self-destruct watchdog.
//! 2. `render_aws_terraform` produces a valid HCL snippet — provider
//!    block, `aws_instance` resource, region substitution.
//! 3. `render_az_arm` produces JSON whose top level is a single
//!    object containing the canonical `Microsoft.Compute/virtualMachines`
//!    resource type.
//! 4. `render_gcp_deployment` produces YAML containing
//!    `compute.v1.instance` and a zone derived from the region.
//! 5. Cross-cloud (S3 → GCS) defers to the `copythat copy` CLI when
//!    no provider-native idiom applies.
//! 6. The Terraform heredoc + ARM `customData` placeholder do not
//!    contain unescaped `EOT` / `}` sequences that would break the
//!    enclosing template.

use copythat_cloud::backend::{Backend, BackendConfig, BackendKind, GcsConfig, S3Config};
use copythat_cloud::offload::{
    OffloadOpts, render_aws_terraform, render_az_arm, render_cloudinit_template,
    render_gcp_deployment,
};

fn s3_backend(name: &str, bucket: &str) -> Backend {
    Backend {
        name: name.to_string(),
        kind: BackendKind::S3,
        config: BackendConfig::S3(S3Config {
            bucket: bucket.to_string(),
            region: "us-east-1".to_string(),
            endpoint: String::new(),
            root: String::new(),
        }),
    }
}

fn gcs_backend(name: &str, bucket: &str) -> Backend {
    Backend {
        name: name.to_string(),
        kind: BackendKind::Gcs,
        config: BackendConfig::Gcs(GcsConfig {
            bucket: bucket.to_string(),
            service_account: String::new(),
            root: String::new(),
        }),
    }
}

#[test]
fn case01_cloudinit_s3_to_s3_uses_aws_s3_cp() {
    let src = s3_backend("src", "primary-bucket");
    let dst = s3_backend("dst", "secondary-bucket");
    let out = render_cloudinit_template(&src, &dst, &OffloadOpts::default());
    assert!(out.contains("aws s3 cp"), "missing aws s3 cp:\n{out}");
    assert!(out.contains("s3://primary-bucket/"));
    assert!(out.contains("s3://secondary-bucket/"));
    assert!(out.contains("shutdown -h"), "watchdog missing");
}

#[test]
fn case02_aws_terraform_has_provider_block_and_region() {
    let src = s3_backend("src", "primary-bucket");
    let dst = s3_backend("dst", "secondary-bucket");
    let out = render_aws_terraform(&src, &dst, &OffloadOpts::default());
    assert!(out.contains("provider \"aws\""));
    assert!(out.contains("aws_instance"));
    assert!(out.contains("us-east-1"));
    // The release tag the cloud-init pulls must thread through.
    assert!(out.contains(OffloadOpts::default_release()));
}

#[test]
fn case03_az_arm_is_a_single_top_level_json_object() {
    let src = s3_backend("src", "primary-bucket");
    let dst = s3_backend("dst", "secondary-bucket");
    let out = render_az_arm(&src, &dst, &OffloadOpts::default());
    assert!(out.starts_with('{'), "must start with '{{'");
    assert!(
        out.contains("Microsoft.Compute/virtualMachines"),
        "ARM template missing VM resource type"
    );
    assert!(
        out.contains("\"$schema\""),
        "ARM template missing schema declaration"
    );
}

#[test]
fn case04_gcp_deployment_has_compute_instance_and_derived_zone() {
    let src = s3_backend("src", "primary-bucket");
    let dst = gcs_backend("dst", "secondary-bucket");
    let opts = OffloadOpts {
        region: "us-central1".to_string(),
        ..OffloadOpts::default()
    };
    let out = render_gcp_deployment(&src, &dst, &opts);
    assert!(out.contains("compute.v1.instance"));
    assert!(
        out.contains("zone: us-central1-a"),
        "zone derivation broke:\n{out}"
    );
}

#[test]
fn case05_cross_cloud_pair_defers_to_copythat_cli() {
    let src = s3_backend("src", "primary-bucket");
    let dst = gcs_backend("dst", "secondary-bucket");
    let out = render_cloudinit_template(&src, &dst, &OffloadOpts::default());
    assert!(
        out.contains("copythat copy"),
        "cross-cloud must use the copythat CLI when no provider-native idiom applies"
    );
    assert!(out.contains("s3://primary-bucket/"));
    assert!(out.contains("gs://secondary-bucket/"));
}

#[test]
fn case06_template_escapes_keep_renderers_well_formed() {
    let src = s3_backend("src", "primary-bucket");
    let dst = s3_backend("dst", "secondary-bucket");

    // Terraform: the heredoc body is indented; the closing `EOT`
    // sentinel must sit on its own line at column 0 of the heredoc
    // (i.e., the rendered string must contain a line that is
    // exactly `EOT`).
    let tf = render_aws_terraform(&src, &dst, &OffloadOpts::default());
    let has_terminator = tf.lines().any(|l| l.trim() == "EOT");
    assert!(has_terminator, "Terraform heredoc terminator missing");

    // ARM: the JSON must close — every `{` matched by a `}`.
    let arm = render_az_arm(&src, &dst, &OffloadOpts::default());
    let opens = arm.matches('{').count();
    let closes = arm.matches('}').count();
    assert_eq!(
        opens, closes,
        "ARM JSON braces unbalanced: {opens} vs {closes}"
    );
}
