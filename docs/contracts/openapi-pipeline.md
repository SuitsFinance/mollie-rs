# OpenAPI contract pipeline

This document is the operational truth for how `mollie-rs` pins, generates, and
diffs the Mollie provider contract.

## Layers of truth

| Layer | Artifact | Owner |
| --- | --- | --- |
| Provider contract | Upstream Mollie OpenAPI (`mollie/openapi`) | Mollie |
| Upstream pin | `specs/upstream-pin.toml` (URL + sha256 + op count) | SDK maintainers |
| Local generation pin | `specs-3.0.yaml` | SDK maintainers |
| Generated client | `src/routes/*`, `src/types*`, `src/route_capabilities.rs` | Generator + CI |
| Facade truth | `src/domain/*` | Handwritten Tier S |
| Registry | `docs/registries/operation-registry.yaml` | Export script + CI |
| Docs | `docs/route-coverage.md`, parity audits | Maintainers |

Never conflate these layers. A facade can lag generation; generation can lag upstream.

## Commands

```text
# Verify local capabilities match specs-3.0.yaml
python scripts/check_generation_reproducibility.py

# Export operation registry (local + known gaps)
python scripts/export_operation_registry.py

# Fetch upstream OpenAPI and verify pin digest (writes specs/upstream-openapi.yaml)
python scripts/fetch_upstream_openapi.py

# Compare local vs upstream inventory
python scripts/compare_upstream_openapi.py --require-upstream --write docs/api-drift-report-upstream.md

# Full client regeneration (requires generator toolchain; may exceed MSRV)
# powershell -ExecutionPolicy Bypass -File scripts/generate_openapi_client.ps1
# sh scripts/generate_openapi_client.sh
```

## Exit codes (upstream)

| Code | Meaning | CI policy |
| --- | --- | --- |
| 0 | Match / OK | Pass |
| 1 | Local pin/capability inconsistency | **Fail** |
| 2 | Upstream differs (missing/extra ops) | Advisory (expect 0 after Phase 2 re-pin) |
| 3 | Upstream digest ≠ pin | **Fail** (review + update pin) |

## Updating the upstream pin

1. Review Mollie OpenAPI changelog / SDK release notes.
2. Run fetch once against a candidate URL.
3. Update `specs/upstream-pin.toml` `sha256` and `operation_count`.
4. Run `compare_upstream_openapi.py` and refresh the operation registry gaps.
5. Only then plan a Tier G regeneration (`specs-3.0.yaml` re-pin).

## Regenerating the local client (0.7+)

1. Snapshot current public API / route count.
2. Adapt upstream paths to the local base-URL style (`/v2` stem vs absolute).
3. Special-case `/oauth2/tokens` (not under `/v2`).
4. Run generator scripts; regenerate capabilities + registry + examples.
5. Separate commits: pin → generate → facades → docs.

## Exception process

Known intentional drift (local missing ops) must appear in:

* `docs/registries/operation-registry.yaml` `upstream_gaps`
* `docs/audits/official-sdk-parity-assessment.md`
* release notes when shipping 0.7+

Dangerous changes (removed ops, auth changes, webhook signature fields) must
block release even if PR CI remains advisory for “added ops only”.
