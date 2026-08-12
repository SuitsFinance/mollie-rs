# API stability (mollie-rs 0.7.x → 1.0)

This document is the public contract posture for the crate. It is **not** a
semver promise of 1.0 readiness.

## Tier model

| Tier | Surface | Stability intent |
| ---- | ------- | ---------------- |
| **S** | `MollieClient::{payments,refunds,…}()` facades, validated builders, webhooks | Prefer for application code. Additive growth is normal pre-1.0; removals require changelog. |
| **G** | Generated `Client` route methods + `types::*` | Tracks the pinned OpenAPI (`specs-3.0.yaml`). Field/enum churn follows provider pin. |
| **Kernel** | Transport retry, delivery outcomes, redirects, pagination host policy, `OperationSafetyProfile` | Behavioral safety contracts. Tightening is allowed; loosening financial fail-closed rules is **not**. |

## OperationSafetyProfile SSOT

Per-operation policy lives in `src/route_capabilities.rs` and is exported as
`OperationSafetyProfile`. Consumers:

- transport retry / sticky-key gates
- Tier-S facades
- `docs/registries/operation-registry.yaml`
- CI: `scripts/check_dangerous_profile_drift.py`

Do **not** invent a second parallel registry.

## Dangerous drift (CI blocking)

CI fails when any of the following hold:

1. High-risk write ops lose `ValidatedFacade` access or become `SafeRead`
2. `IdempotentWrite` without `supports_idempotency`
3. Write classes marked `safe_to_retry: true`
4. GET ops not classified `SafeRead`
5. Local OpenAPI pin vs capability inventory mismatch (`check_generation_reproducibility.py`)

Additive new operations are allowed; they must appear in the capability table
and registry export in the same change.

## cargo-semver-checks

CI runs `cargo-semver-checks` against the last crates.io release. On **0.x**:

- Additive public APIs are expected
- Breaking changes require explicit review and changelog notes
- Do not “fix” failures by weakening MSRV or hiding types without product intent

## Idempotency and cancellation

- Financial writes without a caller-owned sticky key: **≤ 1** HTTP attempt
- Ambiguous delivery (`Unknown`) is fail-closed without a sticky key
- Dropping an in-flight write future after transmit is **Unknown** — document sticky keys for any write the app may cancel/retry

## Release bands (honest)

| Band | Meaning |
| ---- | ------- |
| NOT READY | Kernel or high-risk profile gaps open |
| NEAR READY | Kernel frozen; money facades landed; docs/assurance incomplete |
| RC READY | Assurance pyramid + drift gates green; residual P1 tracked |
| 1.0 READY | Only if release checklist + hostile review pass |

See `docs/sdd/1.0-readiness/` for the program plan.
