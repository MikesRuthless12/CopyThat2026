//! RFC 3161 timestamping — schema-only stub.
//!
//! The Phase 43 spec listed an `rfc3161-client` crate as the
//! upstream dependency, but no such crate exists on crates.io as of
//! Phase 43's ship date (2026-04-29). Rather than introduce a
//! half-working bespoke ASN.1 encoder, this module ships the
//! [`crate::manifest::Rfc3161Token`] schema and leaves the request
//! path classified as
//! [`crate::error::ProvenanceErrorKind::TsaFeatureDisabled`] for
//! both the `tsa`-disabled (default) and `tsa`-enabled builds. A
//! follow-up phase will wire the actual TSA HTTP client (likely a
//! minimal in-tree TimeStampReq encoder over `pkcs8` + `reqwest`).
//!
//! Until then, callers requesting a TSA timestamp via
//! [`crate::SinkConfig::tsa_url`] receive a classified error so
//! they can fail-fast or drop the request and proceed with an
//! unsigned-but-otherwise-valid manifest.

use crate::error::{ProvenanceError, ProvenanceErrorKind};
use crate::manifest::{ProvenanceManifest, Rfc3161Token};

#[allow(dead_code)]
pub(crate) fn request(
    _tsa_url: &str,
    _manifest: &ProvenanceManifest,
) -> Result<Rfc3161Token, ProvenanceError> {
    Err(ProvenanceError::classify(
        ProvenanceErrorKind::TsaFeatureDisabled,
        "RFC 3161 TSA client is deferred to a follow-up phase; rebuild with the wired feature when it lands",
    ))
}

#[allow(dead_code)]
pub(crate) fn verify(
    _token: &Rfc3161Token,
    _manifest: &ProvenanceManifest,
) -> Result<(), ProvenanceError> {
    Err(ProvenanceError::classify(
        ProvenanceErrorKind::TsaFeatureDisabled,
        "RFC 3161 verify is deferred to a follow-up phase",
    ))
}
