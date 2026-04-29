//! Phase 40 — SMB 3.1.1 traffic compression negotiation probe.
//!
//! Surfaces whether a destination path is reachable via the Windows SMB
//! redirector and is therefore a candidate for the kernel-side
//! `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` flag (Win10 1909+; always
//! satisfied on the Win11+ support matrix). The actual chained-
//! compression algorithm is negotiated by the SMB 3.1.1 client / server
//! handshake on first connect — Windows does not surface the negotiated
//! algorithm through any public user-mode API, so [`SmbCompressionState`]
//! reports `algorithm = None` even when `supported = true`. The engine
//! still emits a [`copythat_core::CopyEvent::SmbCompressionActive`]
//! event when the flag was passed, so the UI can render the badge with
//! the unknown-algorithm string.
//!
//! Off-Windows builds report `supported = false`; SMB compression is a
//! Windows-redirector feature.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Phase 40 — best-effort negotiation outcome for an SMB UNC
/// destination.
///
/// `supported` reports whether the kernel SMB redirector is willing to
/// pass the destination through the SMB 3.1.1 stack at all (i.e., the
/// path looks like a UNC path on Win11+). It does **not** confirm that
/// the remote server agreed to compress traffic — that decision happens
/// inside the kernel handshake and is not surfaced through any public
/// user-mode API. When the server refuses, `CopyFileExW` simply skips
/// the flag and falls back to plain SMB, so the optimistic flag-pass
/// is always safe.
///
/// `algorithm` is `None` on every host today; it is plumbed through the
/// IPC surface so a future kernel-mode probe (or telemetry hop) can
/// fill it in without breaking the wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmbCompressionState {
    /// `true` when the destination is a UNC path on Windows and the
    /// SMB redirector is in scope. Always `false` off-Windows or for
    /// local-volume destinations.
    pub supported: bool,
    /// Negotiated chained-compression algorithm if the OS exposes it.
    /// `None` on every host today — see the [`SmbCompressionState`]
    /// type-level note.
    pub algorithm: Option<SmbCompressionAlgo>,
}

impl SmbCompressionState {
    /// Sentinel returned when the destination is not an SMB candidate
    /// (local path, non-Windows host, or empty path).
    pub const fn unsupported() -> Self {
        Self {
            supported: false,
            algorithm: None,
        }
    }
}

/// Phase 40 — chained compression algorithms SMB 3.1.1 negotiates over.
///
/// The algorithm is server-decided at session establishment; this enum
/// is the wire-stable surface the IPC + UI layers use when the kernel
/// (eventually) lets us read it back. See [`SmbCompressionState`] for
/// today's `None`-on-every-host disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SmbCompressionAlgo {
    /// `XPRESS_LZ77` — the SMB 3.1.1 default on Windows clients.
    XpressLz77,
    /// `XPRESS_HUFFMAN` — selected on links where the server signals
    /// preference for higher-ratio compression.
    XpressHuffman,
    /// `LZNT1` — legacy NTFS compression algorithm; rare in modern
    /// SMB 3.1.1 negotiation but accepted by older Windows servers.
    Lznt1,
}

impl SmbCompressionAlgo {
    /// Stable wire string used by IPC + the Phase 40 Fluent
    /// `smb-compress-*` keys. Mirrors the `serde(rename_all =
    /// "kebab-case")` derive.
    pub const fn wire(&self) -> &'static str {
        match self {
            Self::XpressLz77 => "xpress-lz77",
            Self::XpressHuffman => "xpress-huffman",
            Self::Lznt1 => "lznt1",
        }
    }

    /// Capitalised display string matching the algorithm's canonical
    /// name in Microsoft's protocol docs (`XPRESS_LZ77`, etc.). The
    /// header-badge string template in `smb-compress-badge` substitutes
    /// this directly.
    pub const fn display(&self) -> &'static str {
        match self {
            Self::XpressLz77 => "XPRESS_LZ77",
            Self::XpressHuffman => "XPRESS_HUFFMAN",
            Self::Lznt1 => "LZNT1",
        }
    }
}

/// Best-effort: probe whether `dst` is an SMB UNC destination eligible
/// for kernel-side traffic compression.
///
/// The Phase 42 `CopyFileExW` fast path already passes
/// `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` unconditionally on UNC dests
/// — this function lets the engine label a started-copy with the
/// matching `SmbCompressionActive` event so the UI can render the
/// header badge. Returns `unsupported()` on non-Windows hosts, on
/// local-volume paths, and on long-path-prefixed local paths
/// (`\\?\C:\...`).
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use copythat_platform::smb::negotiate_smb_compression;
///
/// // Local destinations are never SMB candidates.
/// assert!(!negotiate_smb_compression(Path::new(r"C:\Users\me\dst.bin")).supported);
/// ```
pub fn negotiate_smb_compression(dst: &Path) -> SmbCompressionState {
    if !cfg!(windows) {
        return SmbCompressionState::unsupported();
    }
    if !crate::topology::is_unc_path(dst) {
        return SmbCompressionState::unsupported();
    }
    // The Win11+ baseline (build 22000+) always ships SMB 3.1.1 with
    // chained compression support on the client side; the server-side
    // half is decided per share in the SMB 3.1.1 negotiate exchange.
    // We can't read the negotiated algorithm from user mode without a
    // kernel-mode hook, so we mark `supported = true, algorithm =
    // None`. The engine still passes the flag — if the server refuses
    // it, the kernel transparently falls back to plain SMB.
    SmbCompressionState {
        supported: true,
        algorithm: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_paths_are_unsupported() {
        let probe = negotiate_smb_compression(Path::new(r"C:\Users\me\dst.bin"));
        assert!(!probe.supported);
        assert_eq!(probe.algorithm, None);
    }

    #[test]
    fn long_path_prefixed_local_is_unsupported() {
        let probe = negotiate_smb_compression(Path::new(r"\\?\C:\dst.bin"));
        assert!(!probe.supported);
    }

    #[test]
    fn empty_path_is_unsupported() {
        assert!(!negotiate_smb_compression(Path::new("")).supported);
    }

    #[cfg(windows)]
    #[test]
    fn unc_path_is_supported_on_windows() {
        let probe = negotiate_smb_compression(Path::new(r"\\fileserver\share\dst.bin"));
        assert!(probe.supported);
        // Algorithm intentionally None until we have a kernel probe.
        assert_eq!(probe.algorithm, None);
    }

    #[cfg(windows)]
    #[test]
    fn unc_long_path_prefix_is_supported() {
        let probe = negotiate_smb_compression(Path::new(r"\\?\UNC\fileserver\share\dst.bin"));
        assert!(probe.supported);
    }

    #[cfg(not(windows))]
    #[test]
    fn unc_path_is_unsupported_off_windows() {
        let probe = negotiate_smb_compression(Path::new(r"\\fileserver\share\dst.bin"));
        assert!(!probe.supported);
    }

    #[test]
    fn algo_wire_string_is_kebab_case() {
        assert_eq!(SmbCompressionAlgo::XpressLz77.wire(), "xpress-lz77");
        assert_eq!(SmbCompressionAlgo::XpressHuffman.wire(), "xpress-huffman");
        assert_eq!(SmbCompressionAlgo::Lznt1.wire(), "lznt1");
    }

    #[test]
    fn algo_display_is_canonical_microsoft_spelling() {
        assert_eq!(SmbCompressionAlgo::XpressLz77.display(), "XPRESS_LZ77");
        assert_eq!(
            SmbCompressionAlgo::XpressHuffman.display(),
            "XPRESS_HUFFMAN"
        );
        assert_eq!(SmbCompressionAlgo::Lznt1.display(), "LZNT1");
    }

    #[test]
    fn algo_serde_round_trip_uses_wire_string() {
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
    fn unsupported_constructor_round_trips() {
        let s = SmbCompressionState::unsupported();
        let json = serde_json::to_string(&s).expect("serialize");
        let back: SmbCompressionState = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
        assert!(!back.supported);
        assert_eq!(back.algorithm, None);
    }
}
