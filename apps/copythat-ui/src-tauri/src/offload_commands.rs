//! Phase 40 Part B — Tauri IPC commands for the SMB compression probe
//! + cloud-VM offload-template wizard.
//!
//! Two commands:
//!
//! - `smb_compression_state(dst_path)` returns the typed
//!   [`SmbCompressionStateDto`] so the Svelte Settings panel can show
//!   "🗜 SMB compression eligible" / "❌ SMB compression unsupported"
//!   prose for an arbitrary destination path the user pastes in.
//! - `render_offload_template(format, src, dst, opts)` returns the
//!   rendered template body the user pastes into their cloud
//!   provider's console / `terraform apply`. Four formats:
//!   `cloud-init` / `aws-terraform` / `az-arm` / `gcp-deployment`.
//!   Templates do not embed credentials — see
//!   [`copythat_cloud::offload`] for the threat model.

use std::path::PathBuf;

use copythat_cloud::backend::Backend;
use copythat_cloud::offload::{
    OffloadOpts, render_aws_terraform, render_az_arm, render_cloudinit_template,
    render_gcp_deployment,
};
use copythat_platform::smb::{SmbCompressionAlgo, SmbCompressionState, negotiate_smb_compression};
use serde::{Deserialize, Serialize};

/// Wire shape for [`SmbCompressionState`] across the IPC boundary.
/// Mirrors the Rust struct field names in camelCase so the Svelte
/// caller can read `state.supported` / `state.algorithm`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmbCompressionStateDto {
    pub supported: bool,
    /// Stable wire string when the kernel surfaces the negotiated
    /// algorithm (`"xpress-lz77"` / `"xpress-huffman"` / `"lznt1"`),
    /// `None` otherwise. Today the value is `None` on every host —
    /// see the [`copythat_platform::smb`] type-level note.
    pub algorithm: Option<String>,
}

impl From<SmbCompressionState> for SmbCompressionStateDto {
    fn from(state: SmbCompressionState) -> Self {
        Self {
            supported: state.supported,
            algorithm: state
                .algorithm
                .as_ref()
                .map(SmbCompressionAlgo::wire)
                .map(str::to_string),
        }
    }
}

/// `smb_compression_state(dst_path)` — probe the destination path the
/// user typed into the Settings panel. Returns
/// `{ supported: false, algorithm: null }` for local volumes, non-
/// Windows hosts, and long-path-prefixed local paths
/// (`\\?\C:\...`); returns `{ supported: true, algorithm: null }` for
/// UNC destinations on Windows. The probe is a path-prefix check —
/// no syscalls, no network round-trips.
#[tauri::command]
pub async fn smb_compression_state(dst_path: String) -> Result<SmbCompressionStateDto, String> {
    let path = PathBuf::from(dst_path);
    Ok(negotiate_smb_compression(&path).into())
}

/// Wire-stable selector for the four output formats. Mirrors the
/// `OffloadTemplateFormat` Svelte enum in `RemotesTab.svelte`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OffloadTemplateFormat {
    /// Generic cloud-init user-data YAML.
    CloudInit,
    /// AWS-flavoured Terraform HCL.
    AwsTerraform,
    /// Azure Resource Manager JSON deployment.
    AzArm,
    /// Google Cloud Deployment Manager YAML.
    GcpDeployment,
}

/// `render_offload_template(format, src, dst, opts)` — render a single
/// deployment template the user pastes into their cloud's console.
/// Pure function (no side effects, no network) — the rendered string
/// is the only return value. Templates never embed credentials; the
/// VM relies on IAM-role / managed-identity / service-account access
/// the user provisions out-of-band.
#[tauri::command]
pub async fn render_offload_template(
    format: OffloadTemplateFormat,
    src: Backend,
    dst: Backend,
    opts: OffloadOpts,
) -> Result<String, String> {
    let body = match format {
        OffloadTemplateFormat::CloudInit => render_cloudinit_template(&src, &dst, &opts),
        OffloadTemplateFormat::AwsTerraform => render_aws_terraform(&src, &dst, &opts),
        OffloadTemplateFormat::AzArm => render_az_arm(&src, &dst, &opts),
        OffloadTemplateFormat::GcpDeployment => render_gcp_deployment(&src, &dst, &opts),
    };
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use copythat_cloud::backend::{BackendConfig, BackendKind, S3Config};

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

    #[tokio::test]
    async fn smb_compression_state_local_path_unsupported() {
        let dto = smb_compression_state(r"C:\Users\me\dst.bin".to_string())
            .await
            .expect("ok");
        assert!(!dto.supported);
        assert_eq!(dto.algorithm, None);
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn smb_compression_state_unc_path_supported() {
        let dto = smb_compression_state(r"\\fileserver\share\dst.bin".to_string())
            .await
            .expect("ok");
        assert!(dto.supported);
        // Algorithm intentionally None until kernel probe lands.
        assert_eq!(dto.algorithm, None);
    }

    #[tokio::test]
    async fn render_offload_template_cloudinit_contains_aws_s3_cp() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let body = render_offload_template(
            OffloadTemplateFormat::CloudInit,
            src,
            dst,
            OffloadOpts::default(),
        )
        .await
        .expect("ok");
        assert!(body.contains("aws s3 cp"));
        assert!(body.contains("s3://bucket-a/"));
        assert!(body.contains("s3://bucket-b/"));
    }

    #[tokio::test]
    async fn render_offload_template_aws_terraform_contains_provider_block() {
        let src = s3_backend("src", "bucket-a");
        let dst = s3_backend("dst", "bucket-b");
        let body = render_offload_template(
            OffloadTemplateFormat::AwsTerraform,
            src,
            dst,
            OffloadOpts::default(),
        )
        .await
        .expect("ok");
        assert!(body.contains("provider \"aws\""));
    }

    #[test]
    fn template_format_serde_round_trip() {
        for &fmt in &[
            OffloadTemplateFormat::CloudInit,
            OffloadTemplateFormat::AwsTerraform,
            OffloadTemplateFormat::AzArm,
            OffloadTemplateFormat::GcpDeployment,
        ] {
            let json = serde_json::to_string(&fmt).expect("ser");
            let back: OffloadTemplateFormat = serde_json::from_str(&json).expect("de");
            assert_eq!(fmt, back);
        }
    }
}
