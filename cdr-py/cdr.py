"""Pure-Python CDR-0 reference reader (docs/spec/CDR-0.md; §13 compliance).

Decodes + validates CDR-0 file manifests (CBOR), parses the repository
descriptor (cdr.toml) with the §8 version gate, and verifies chunk BLAKE3 on
§3 pack read. This is the reference the language-neutral conformance corpus
(docs/spec/conformance/) checks — the same fixtures the Rust `CdrManifest`
smoke uses, so the two implementations agree by construction.

Dependencies: ``cbor2`` (manifest decode); ``blake3`` (only for pack read).
"""

from __future__ import annotations

CDR_SPEC_VERSION = 0
CDR_ALGO = "fastcdc-2020;min=524288;avg=1048576;max=4194304;hash=blake3-256"
HASH_LEN = 32


class CdrError(Exception):
    """A CDR-0 manifest or descriptor violated a spec invariant."""


def ensure_readable(spec_version: int) -> None:
    """§8 version gate: refuse anything newer than this reader implements."""
    if spec_version > CDR_SPEC_VERSION:
        raise CdrError(
            f"spec_version {spec_version} is newer than this reader "
            f"({CDR_SPEC_VERSION})"
        )


def parse_manifest(data: bytes) -> dict:
    """Decode a manifest CBOR and enforce every §5 invariant. Returns the
    decoded manifest ``dict`` or raises :class:`CdrError`."""
    import cbor2

    m = cbor2.loads(data)
    if not isinstance(m, dict):
        raise CdrError("manifest is not a CBOR map")
    ensure_readable(int(m.get("spec_version", 0)))
    if m.get("algo") != CDR_ALGO:
        raise CdrError(f"unsupported chunk algorithm {m.get('algo')!r}")
    file_hash = m.get("file_hash")
    if not isinstance(file_hash, (bytes, bytearray)) or len(file_hash) != HASH_LEN:
        raise CdrError("file_hash must be 32 bytes")
    size = int(m.get("size", 0))
    cursor = 0
    total = 0
    for i, c in enumerate(m.get("chunks", [])):
        h = c.get("hash")
        if not isinstance(h, (bytes, bytearray)) or len(h) != HASH_LEN:
            raise CdrError(f"chunk {i} hash must be 32 bytes")
        if int(c.get("offset", -1)) != cursor:
            raise CdrError(
                f"chunk {i} is not contiguous (offset {c.get('offset')} != {cursor})"
            )
        ln = int(c.get("len", 0))
        cursor += ln
        total += ln
    if total != size:
        raise CdrError(f"declared size {size} != sum of chunk lengths {total}")
    return m


def parse_descriptor(toml_text: str) -> dict:
    """Parse a ``cdr.toml`` repository descriptor and gate its spec_version."""
    try:
        import tomllib  # Python 3.11+
    except ModuleNotFoundError:  # pragma: no cover
        import tomli as tomllib

    d = tomllib.loads(toml_text)
    ensure_readable(int(d.get("spec_version", 0)))
    return d


def verify_chunk(chunk_plaintext: bytes, expected_hash: bytes) -> None:
    """§3 pack read: the chunk's BLAKE3-256 must equal its manifest hash."""
    import blake3

    if blake3.blake3(chunk_plaintext).digest() != bytes(expected_hash):
        raise CdrError("chunk failed its BLAKE3 integrity check")
