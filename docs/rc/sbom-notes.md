# SBOM / provenance notes

## Recommended local SBOM

```bash
cargo install cargo-cyclonedx
cargo cyclonedx -f json -o sbom.cdx.json
```

Or:

```bash
cargo install cargo-auditable
# rebuild with auditable metadata for provenance of binary consumers
```

## Provenance for this crate

- Source: git SHA of release tag
- Lockfile: committed `Cargo.lock` (library publishes with lock for apps; crate is library-first)
- Upstream OpenAPI: pinned `specs-3.0.yaml` + `scripts/compare_upstream_openapi.py`
- High-risk SSOT: `HIGH_RISK_WRITE_OPERATION_IDS` + CI drift + coverage report

Do not claim SLSA levels without a signed release pipeline.
