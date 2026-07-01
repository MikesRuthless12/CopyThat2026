"""cdr-py checks the shared CDR-0 conformance corpus (docs/spec/conformance/)
— the same fixtures the Rust `phase_50f_conformance` smoke uses. Run with
`pytest` from the repo root (needs `pip install cbor2`)."""

import glob
import os

import pytest

from cdr import CdrError, parse_manifest

CORPUS = os.path.join(os.path.dirname(__file__), "..", "docs", "spec", "conformance")


@pytest.mark.parametrize("path", sorted(glob.glob(os.path.join(CORPUS, "*.cbor"))))
def test_corpus_accept_reject(path):
    name = os.path.basename(path)
    with open(path, "rb") as f:
        data = f.read()
    if name.startswith("valid"):
        parse_manifest(data)  # must not raise
    else:
        with pytest.raises(CdrError):
            parse_manifest(data)
