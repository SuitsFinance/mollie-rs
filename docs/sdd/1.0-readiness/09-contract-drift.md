# SDD 09 — Contract drift

## Existing

Local drift report; registry export; generation reproducibility; upstream pin digest blocking; cargo-semver-checks job.

## Desired

Manifest from OperationSafetyProfile + OpenAPI; fail CI on dangerous semantic drift; soft visibility for additive ops.

## SSOT path (single export)

```text
src/route_capabilities.rs  (= OperationSafetyProfile table)
        │
        ├─► scripts/export_operation_registry.py
        │         → docs/registries/operation-registry.yaml
        ├─► scripts/check_dangerous_profile_drift.py   (blocking)
        ├─► scripts/check_generation_reproducibility.py (blocking)
        └─► scripts/report_api_drift.py                 (inventory artifact)
```

Upstream pin: `specs/upstream-pin.toml` + `fetch_upstream_openapi.py` / `compare_upstream_openapi.py`.

## CI gate matrix

| Gate | Script / job | Blocking? | Failure meaning |
| ---- | ------------ | --------- | --------------- |
| Capability ↔ OpenAPI pin | `check_generation_reproducibility.py` | **Yes** | Op missing/extra vs `specs-3.0.yaml` |
| High-risk profile invariants | `check_dangerous_profile_drift.py` | **Yes** | Financial write misclassified / lost facade / SafeRead on write |
| Registry committed | `export_operation_registry.py` + `git diff` | **Yes** | Stale `operation-registry.yaml` |
| Local drift report | `report_api_drift.py --write` | Report artifact | Inventory snapshot |
| Upstream digest | `fetch_upstream_openapi.py` | **Yes** on digest mismatch (exit 3) | Pin outdated |
| Upstream op parity | `compare_upstream_openapi.py` | Advisory missing/extra (exit 2); block local broken (1/3) | Provider drift visibility |
| Public API | `cargo-semver-checks` job | Job fails on breakage vs crates.io | Accidental public break |
| Human contract | `docs/API-STABILITY.md` | Review | Release posture |

## Dangerous invariants (profile)

1. High-risk writes ∈ {IdempotentWrite, NonRetryableWrite, FinancialWrite}
2. High-risk writes `access = ValidatedFacade`
3. IdempotentWrite ⇒ `supports_idempotency`
4. Write classes ⇏ `safe_to_retry`
5. GET ⇒ SafeRead

High-risk set is listed in `scripts/check_dangerous_profile_drift.py` (keep in sync with SDD high-risk inventory).

## Acceptance

- [x] Single export path (capabilities → registry)
- [x] Documented gate matrix in CI + this file
- [x] Dangerous profile drift blocking in generation job
- [x] `docs/API-STABILITY.md` published
