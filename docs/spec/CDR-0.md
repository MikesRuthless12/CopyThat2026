<!--
SPDX-License-Identifier: CC0-1.0
This specification is dedicated to the public domain under Creative
Commons CC0 1.0 Universal. No rights reserved. Anyone may implement,
modify, or redistribute it for any purpose.
-->

# Common Dedup Repository — Specification, Version 0 (CDR-0)

**Status:** Draft · **Spec version:** `0` · **License:** [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/)

**Reference implementation:** [`copythat-chunk`](../../crates/copythat-chunk) in this
repository — module [`cdr`](../../crates/copythat-chunk/src/cdr.rs) (manifest layer,
v0) and [`store`](../../crates/copythat-chunk/src/store.rs) (chunk + pack layer).

---

## 0. Why this exists

Every content-defined backup/sync tool — restic, Borg, Kopia, CopyThat — stores
the same thing: variable-sized **chunks** keyed by a cryptographic hash, packed
into files, indexed by per-file **manifests**, grouped into **snapshots**. They
all reinvent an incompatible on-disk layout, so moving between them means a
full re-ingest of terabytes.

CDR-0 defines a single, public-domain layout for that shared structure. A
compliant tool can read any compliant repository; migrating **in** from another
compliant tool becomes a near-instant manifest translation that *reuses the
existing chunk bytes* (via reflink/hardlink) instead of re-chunking and
re-uploading.

The terms **MUST**, **MUST NOT**, **SHOULD**, and **MAY** are used per
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

---

## 1. Repository on-disk layout

```text
<repo>/
  cdr.toml                 # repository descriptor (§9): spec_version, algo
  index.redb               # chunk index: hash -> (pack_id, offset, len)
  repository.redb          # snapshot catalog + chunks_refcount (§6, §7)
  packs/
    pack-<uuid>.pack       # concatenated chunk bytes (§3)
```

- A reader **MUST** read `cdr.toml` first and apply the §8 version gate before
  touching any other file.
- The chunk index and the snapshot catalog **MAY** live in any embedded
  key-value store; the reference implementation uses [`redb`](https://crates.io/crates/redb).
  Their *logical* schemas (§5, §6) are normative; the physical KV encoding is not.
- The `packs/` directory is the only normative *byte* container and is fully
  specified in §3.

---

## 2. Chunk algorithm (content addressing)

- Chunking **MUST** be **FastCDC (2020 variant)** with these parameters:

  | Parameter | Value      |
  | --------- | ---------- |
  | minimum   | `524288`  (512 KiB) |
  | average   | `1048576` (1 MiB)   |
  | maximum   | `4194304` (4 MiB)   |

- Each chunk is keyed by the **BLAKE3-256** digest of its bytes (32 bytes).
- The canonical algorithm tag string is:

  ```text
  fastcdc-2020;min=524288;avg=1048576;max=4194304;hash=blake3-256
  ```

  This exact string **MUST** appear in every manifest (`algo`, §5) and in the
  repository descriptor (§9). A reader that does not implement this algorithm
  **MUST** refuse the repository rather than mis-address chunks.

Because the parameters and hash are fixed, two compliant tools chunking the same
bytes produce identical chunk boundaries and identical keys — which is what makes
chunk reuse across tools sound.

---

## 3. Pack file format

Chunk bytes are appended into rolling `pack-<uuid>.pack` files. A pack **MUST**
be laid out as:

```text
+------------------+  offset 0
| MAGIC  "CDR0"    |  4 bytes
| VERSION u8 = 0   |  1 byte
| RESERVED [3]     |  3 bytes (MUST be zero)
+------------------+  offset 8  = header end
| chunk bytes ...  |  concatenated raw chunk bytes, in append order
|        ...       |
+------------------+
| INDEX (CBOR)     |  array of [hash:bstr(32), offset:u64, len:u32]
+------------------+
| FOOTER           |
|   index_len  u64 (LE)   # byte length of the INDEX section          |
|   index_hash bstr(32)   # BLAKE3-256 of the INDEX section bytes     |
|   MAGIC  "CDR0"  4 bytes # trailing magic for reverse scan          |
+------------------+  EOF
```

- `offset` in the INDEX is relative to the start of the chunk-bytes region
  (i.e. absolute file offset `8 + offset`).
- A reader recovers a pack's index by reading the trailing magic, then
  `index_hash` + `index_len`, then the INDEX section, and **MUST** verify the
  INDEX bytes BLAKE3 to `index_hash` before trusting it.
- A chunk read **MUST** verify the bytes read back BLAKE3 to the requested key;
  a mismatch is on-disk corruption and **MUST** be surfaced as an error, never
  returned as data.
- Packs are immutable once an index/footer is written. Reclaiming space is the
  GC's job (§7): a pack none of whose chunks remain reachable is deleted whole.

> **Reference-implementation note (v0).** The current `copythat-chunk` store
> writes raw concatenated chunk bytes + a `redb` index and tracks the active
> pack in `index.redb` rather than emitting the in-pack INDEX/FOOTER above. The
> framed pack format in this section is the **normative CDR-0 container**; a
> one-time, on-upgrade migration writes the frame. Both forms address chunks
> identically (BLAKE3 key → bytes), so manifests are already portable today.

---

## 4. Digests and byte strings

- All digests are raw 32-byte BLAKE3-256 values.
- In CBOR, every digest **MUST** be encoded as a definite-length **byte string**
  (major type 2), never as an array of integers. (The reference implementation
  uses `serde_bytes` to enforce this.)
- In JSON representations (tooling, debugging), digests are lowercase hex.

---

## 5. Manifest format (normative, implemented in v0)

A **manifest** describes how to reconstruct one file from chunks. It is encoded
as **deterministic CBOR** (§7). Logical schema:

```jsonc
CdrManifest = {
  "spec_version": uint,        // CDR spec version this manifest was written under
  "algo":         tstr,        // §2 algorithm tag; MUST equal the canonical string
  "file_hash":    bstr .size 32, // BLAKE3-256 of the whole file
  "size":         uint,        // total file size in bytes
  "chunks": [ * CdrChunkRef ]  // ordered; concatenation reconstructs the file
}

CdrChunkRef = {
  "hash":   bstr .size 32,     // BLAKE3-256 of the chunk
  "offset": uint,              // byte offset of this chunk in the file
  "len":    uint               // chunk byte length (<= 4 MiB per §2)
}
```

A reader **MUST** reject a manifest unless **all** of the following hold
(see `CdrManifest::validate` in the reference implementation):

1. `spec_version` passes the §8 version gate.
2. `algo` equals the canonical §2 tag.
3. `file_hash` is exactly 32 bytes and every `chunks[i].hash` is exactly 32 bytes.
4. Chunks tile the file contiguously from offset 0: `chunks[0].offset == 0` and
   `chunks[i].offset == chunks[i-1].offset + chunks[i-1].len`.
5. `sum(chunks[i].len) == size`.

A machine-readable JSON-Schema mirror of this schema is published alongside this
document as `CDR-0.manifest.schema.json` (planned; the CDDL above is normative in
the interim).

---

## 6. Snapshot metadata schema (normative; reference impl: catalog implemented, rich metadata pending)

A **snapshot** is a named point-in-time set of file manifests. Logical schema:

```jsonc
CdrSnapshot = {
  "spec_version": uint,
  "id":           uint,                 // monotonic within a repository
  "kind":         tstr,                 // "copy" | "sync" | "version" | "backup"
  "created_at":   tstr,                 // RFC 3339 / ISO 8601 timestamp (UTC)
  "label":        tstr,
  "files": [ * CdrFileEntry ]
}

CdrFileEntry = {
  "path":     tstr,                     // NFC-normalized, '/'-separated logical path
  "manifest": CdrManifest,              // §5
  "meta":     ? CdrFileMeta             // optional; absent = unknown
}

CdrFileMeta = {
  "mode":   ? uint,                     // POSIX mode bits
  "uid":    ? uint, "gid": ? uint,
  "owner":  ? tstr, "group": ? tstr,    // resolved names where available
  "acl":    ? bstr,                     // Windows ACL (SDDL) serialization
  "mtime":  ? tstr,                     // RFC 3339
  "xattrs": ? { * tstr => bstr }        // extended attributes
}
```

Rules:
- Timestamps **MUST** be RFC 3339 in UTC.
- Paths **MUST** be Unicode NFC and use `/` as the separator regardless of host OS.
- Metadata fields are individually optional; a reader **MUST** treat an absent
  field as "unknown" and **MUST NOT** fail solely because metadata is missing.

> **Reference-implementation note (v0).** `copythat-chunk`'s `Repository` (Phase
> 49) implements the snapshot **catalog** — id, kind, label, timestamp, and the
> per-file manifests — and stores the timestamp as integer epoch-milliseconds
> internally; the RFC 3339 surface and the optional `CdrFileMeta` (mode/ACL/
> xattrs) are emitted at the CDR export boundary. Populating the full
> `CdrFileMeta` is tracked with the migration tooling (§11).

---

## 7. Canonical (deterministic) encoding

- All CDR-0 CBOR **MUST** use **definite-length** arrays and maps and the
  shortest-form integer encodings (the default for compliant CBOR encoders).
- Struct/map fields are emitted in the field order given by the schemas above.
  Re-encoding an identical logical value therefore yields **byte-identical**
  output — the property two tools rely on to agree on a manifest's identity.
- **v1 hardening (non-normative for v0):** strict RFC 8949 §4.2.1
  core-deterministic *map-key ordering* (keys sorted by encoded bytes). v0
  readers **MUST NOT** assume key sorting; they **MUST** parse by key name.

---

## 8. Versioning and the forward-compatibility gate

- The repository descriptor (§9) and every manifest/snapshot carry a
  `spec_version` (an unsigned integer; this document defines version `0`).
- A reader **MUST refuse to read or write** a repository whose `spec_version`
  is **greater than** the highest version the reader implements, returning a
  typed "unsupported version" error rather than attempting a best-effort parse.
  (Reference: `cdr::ensure_readable`.)
- A reader **MAY** read a repository with a `spec_version` **lower** than its
  own (older repositories remain readable). Writers **SHOULD** write the lowest
  `spec_version` that can represent the data.

---

## 9. Repository descriptor — `cdr.toml`

```toml
spec_version = 0
algo = "fastcdc-2020;min=524288;avg=1048576;max=4194304;hash=blake3-256"
# optional, advisory:
created_by = "copythat-chunk 0.19"
```

A reader **MUST** apply the §8 gate to `spec_version` and verify `algo` matches
§2 before reading chunks.

---

## 10. Encryption envelope (normative spec; reference impl pending)

When a repository is encrypted, chunk bytes in packs (§3) are sealed and the
manifest/snapshot CBOR **MAY** be sealed:

- **Cipher:** AES-256-GCM with a 96-bit (12-byte) IV. Each sealed unit stores
  `iv || ciphertext || tag`.
- **IV uniqueness:** an IV **MUST NOT** be reused under the same key.
- **Key derivation (passphrase):** scrypt with `N = 2^17`, `r = 8`, `p = 1`,
  32-byte output. The salt **MUST** be stored with the repository.
- **Key derivation (raw):** alternatively a directly supplied 32-byte key.

### 10.1 Recipient envelope (forward reference — Phase 51)

For multi-recipient sharing, a per-file random data key is wrapped once per
recipient using an `age` recipient (X25519). The manifest **MAY** carry an
optional `envelope` listing wrapped data keys keyed by recipient fingerprint;
readers that do not understand `envelope` **MUST** ignore it (it is additive and
backward compatible).

---

## 11. Migration & compliance tooling

CopyThat ships the migration **framework + repository detector**; full importers
for the other tools are gated on the constraints below.

**Implemented** (`copythat-chunk::migrate`, `copythat migrate` / `export` CLI):
- `RepoFormat::detect()` recognises restic / Borg / Kopia / CDR-0 repositories
  from their on-disk markers — no decryption, no new dependencies.
- `copythat migrate cdr <src> <cdr-repo>` — copy / re-home a CDR-0 repository
  (the reference path; exercises the whole pipeline end to end).
- `copythat migrate <restic|borg|kopia> …` — returns a typed, actionable error
  naming exactly what a full importer needs. It does **not** silently emit a
  wrong migration.
- `copythat export <cdr-repo> <tool> <dst>` — the inverse entry point; writing a
  foreign tool's on-disk format is not yet implemented.

**Why the foreign importers are blocked.** Enumerating even `path → (chunkID,
length)` from another tool's repository is not a plaintext parse:

| Tool | Passphrase required to enumerate? | Crypto / codec needed |
| ---- | -------------------------------- | --------------------- |
| restic | **Yes, always** (index, snapshots, trees all encrypted) | AES-256-CTR + Poly1305-AES + scrypt |
| Borg `repokey*` / `keyfile*` (default) + all 2.0 | **Yes** | AES-256-CTR + HMAC + PBKDF2 (1.x) / AEAD + Argon2 (2.0); **MessagePack** |
| Borg `none` / `authenticated` (non-default) | No | still needs a **MessagePack** parser |
| Kopia | **Yes, always** (even content IDs are keyed hashes) | AES-256-GCM + HKDF + scrypt + keyed-BLAKE2b |

None of those crypto primitives (nor a MessagePack codec) are in this
workspace's dependency tree, and Phase 50's rule is **no new crates** — so a
correct importer cannot land without (a) relaxing that rule to add the codec +
crypto crates, and (b) real source repositories to validate against (shipping
untested decryption would risk silently corrupting a migration). Chunk identity
is in any case **not portable across tools**: each uses a per-repo random CDC
parameter and most key the chunk-ID hash with a per-repo secret, so migration
translates *manifests*, never chunk IDs.

**Planned ecosystem items:**
- `cdr-py` — a reference parser on PyPI, published from a sibling repository
  with a "spec compliance" README.
- Upstream engagement: issues / PRs proposing CDR-0 adoption to restic, Borg,
  and Kopia.

Status tracked in [`docs/ROADMAP.md`](../ROADMAP.md).

---

## 12. WORM marker (forward reference — Phase 34)

A repository participating in tamper-evident audit (Phase 34) carries a WORM
marker file whose format chains to the audit log's rolling BLAKE3 hash. The
marker pins the repository's audit-chain head so external verification can
detect rollback. Full format specified with the audit-chain work.

---

## 13. Conformance summary

A **CDR-0 reader** is conformant if it:
1. reads `cdr.toml` and applies the §8 version gate (`spec_version <= 0`);
2. verifies the §2 algorithm tag;
3. parses manifests per §5 and enforces all five validation rules;
4. verifies chunk bytes against their BLAKE3 keys on read (§3);
5. ignores unknown optional fields (§6, §10.1) rather than failing.

A **CDR-0 writer** is conformant if it additionally:
6. emits deterministic CBOR per §7;
7. writes the §3 pack frame and §9 descriptor;
8. refuses to write a `spec_version` higher than it implements (§8).

The reference implementation's manifest layer (§5, §7, §8) is covered by
`tests/smoke/phase_50_portability.rs`.

---

*CDR-0 is published into the public domain under CC0-1.0. Contributions and
independent implementations are welcome.*
