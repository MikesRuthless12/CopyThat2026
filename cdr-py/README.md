# cdr-py

A pure-Python reference reader for **CDR-0** — the Copy-That Deduplicated
Repository format (see `docs/spec/CDR-0.md`). It decodes + validates file
manifests (CBOR), parses the repository descriptor (`cdr.toml`) with the §8
version gate, and verifies chunk BLAKE3 on §3 pack read.

`cdr-py` is the **§13 reference implementation**: it self-tests against the
language-neutral conformance corpus in `docs/spec/conformance/` — the exact
fixtures the Rust `CdrManifest` smoke (`tests/smoke/phase_50f_conformance.rs`)
uses, so the two implementations agree by construction.

## Use

```python
from cdr import parse_manifest, CdrError
manifest = parse_manifest(open("some.cbor", "rb").read())   # raises CdrError if invalid
```

## Test

```sh
pip install cbor2 pytest        # blake3 only needed for pack verification
pytest cdr-py/
```

The corpus is regenerated from the Rust reference with
`cargo run -p xtask -- gen-conformance`, so a schema/impl change never drifts
from the fixtures.

> Publishing to PyPI (as the sibling `cdr-py` package) + upstreaming a
> JSON-Schema PR are follow-ups; the source of truth lives here.
