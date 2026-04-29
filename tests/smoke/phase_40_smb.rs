//! Phase 40 smoke — SMB compression negotiation + Fluent locale parity.
//!
//! Five cases:
//!
//! 1. `negotiate_smb_compression(local_path)` returns `unsupported`.
//! 2. `negotiate_smb_compression(long_path_prefixed_local)` returns
//!    `unsupported` — the `\\?\` long-path prefix on a local volume
//!    must NOT be misinterpreted as a UNC.
//! 3. (Windows-only) `negotiate_smb_compression(unc_path)` returns
//!    `supported = true`, `algorithm = None` (today's user-mode
//!    surface — see the [`copythat_platform::smb`] type-level note).
//! 4. `SmbCompressionAlgo::wire()` round-trips through serde for all
//!    three variants — the wire string is what the IPC layer carries
//!    inside `CopyEvent::SmbCompressionActive { algo }`.
//! 5. All 6 Phase 40 SMB Fluent keys + all 6 cloud-offload keys
//!    appear in every one of the 18 locale files.

use std::path::Path;

use copythat_platform::smb::{SmbCompressionAlgo, SmbCompressionState, negotiate_smb_compression};

const PHASE_40_KEYS: &[&str] = &[
    // SMB block
    "smb-compress-badge",
    "smb-compress-badge-tooltip",
    "smb-compress-toast-saved",
    "smb-compress-algo-unknown",
    "settings-smb-compress-heading",
    "settings-smb-compress-hint",
    // Cloud-offload block
    "cloud-offload-heading",
    "cloud-offload-hint",
    "cloud-offload-render-button",
    "cloud-offload-copy-clipboard",
    "cloud-offload-template-format",
    "cloud-offload-self-destruct-warning",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case01_local_path_is_unsupported() {
    let probe = negotiate_smb_compression(Path::new(r"C:\Users\me\dst.bin"));
    assert!(
        !probe.supported,
        "local path must not negotiate SMB compression"
    );
    assert_eq!(probe.algorithm, None);
    assert_eq!(probe, SmbCompressionState::unsupported());
}

#[test]
fn case02_long_path_prefixed_local_is_unsupported() {
    let probe = negotiate_smb_compression(Path::new(r"\\?\C:\Users\me\dst.bin"));
    assert!(
        !probe.supported,
        "`\\\\?\\` long-path prefix on a local volume is NOT a UNC"
    );
}

#[cfg(windows)]
#[test]
fn case03_unc_path_is_supported_on_windows() {
    let probe = negotiate_smb_compression(Path::new(r"\\fileserver\share\dst.bin"));
    assert!(
        probe.supported,
        "UNC dest on Windows must report supported = true"
    );
    // No public user-mode surface for the negotiated algorithm yet.
    assert_eq!(probe.algorithm, None);
}

#[cfg(not(windows))]
#[test]
fn case03_unc_path_is_unsupported_off_windows() {
    let probe = negotiate_smb_compression(Path::new(r"\\fileserver\share\dst.bin"));
    assert!(
        !probe.supported,
        "off-Windows builds must report SMB compression as unsupported"
    );
}

#[test]
fn case04_algo_wire_strings_round_trip_through_serde() {
    for &algo in &[
        SmbCompressionAlgo::XpressLz77,
        SmbCompressionAlgo::XpressHuffman,
        SmbCompressionAlgo::Lznt1,
    ] {
        let json = serde_json::to_string(&algo).expect("serialize");
        assert_eq!(json, format!("\"{}\"", algo.wire()));
        let back: SmbCompressionAlgo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, algo);
    }
}

#[test]
fn case05_all_phase_40_keys_present_in_every_locale() {
    // Each locale file must contain a `<key> =` line for every Phase 40 key.
    // We don't check that the values are translated (i18n-lint covers parity);
    // we just confirm the literal `<key> =` substring is there. That's enough
    // to catch the "forgot to add to locale X" mistake.
    let workspace_root = workspace_root();
    let mut missing: Vec<String> = Vec::new();
    for locale in LOCALES {
        let path = workspace_root
            .join("locales")
            .join(locale)
            .join("copythat.ftl");
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("could not read {}", path.display()));
        for key in PHASE_40_KEYS {
            let needle = format!("{key} =");
            if !body.contains(&needle) {
                missing.push(format!("{locale}/{key}"));
            }
        }
    }
    assert!(missing.is_empty(), "missing Phase 40 keys: {missing:?}");
}

fn workspace_root() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR points at the crate that registered the test
    // (copythat-platform); the workspace root is two levels up.
    let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace has crates/<name>/Cargo.toml layout")
        .to_path_buf()
}
