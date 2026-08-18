# API stability (mollie-rs 0.7.x â†’ 1.0)

This document is the public contract posture for the crate. It is **not** a
semver promise of 1.0 readiness.

## Tier model

| Tier | Surface | Stability intent |
| ---- | ------- | ---------------- |
| **S** | `MollieClient::{payments,refunds,â€¦}()` facades, validated builders, webhooks | Prefer for application code. Additive growth is normal pre-1.0; removals require changelog. |
| **G** | Generated `Client` route methods + `types::*` | Tracks the pinned OpenAPI (`specs-3.0.yaml`). Field/enum churn follows provider pin. |
| **Kernel** | Transport retry, delivery outcomes, redirects, pagination host policy, `OperationSafetyProfile` | Behavioral safety contracts. Tightening is allowed; loosening financial fail-closed rules is **not**. |

## Provider API maturity (Mollie lifecycle)

Separate from Tier S/G: Mollie marks some route groups as beta or private beta
in the pinned OpenAPI descriptions. `mollie-rs` tracks that lifecycle in
`docs/registries/provider-maturity.yaml` and projects it onto every operation in
`docs/registries/operation-registry.yaml` as `provider_maturity`.

| Value | Meaning |
| ----- | ------- |
| `ga` | Generally available at Mollie; no beta banner in the pinned OpenAPI operation descriptions |
| `beta` | Public beta banner in the pinned OpenAPI descriptions |
| `private_beta` | Private beta banner in the pinned OpenAPI descriptions |

Generated route rustdocs mirror OpenAPI descriptions. When Mollie removes a beta
banner (as with **Sales Invoices**, now `ga` in official SDKs), the local pin
and docs must reflect GA â€” not leave stale ðŸš§ warnings or beta-only registry
labels.

**Sales Invoices (`sales_invoices_api`):** provider maturity is **`ga`**. Tier
coverage remains **Generated only** (Tier G routes; no Tier-S facade).

Route groups still marked beta/private beta in the current pin include Sessions,
Transfers, Capabilities, Verify payee, and Unmatched credit transfers. Next-gen
Webhooks are documented separately in `docs/contracts/webhooks.md`.

Review `provider-maturity.yaml` when repinning OpenAPI or when official SDK
release notes report a maturity change.

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
5. Local OpenAPI pin vs capability inventory mismatch (check_generation_reproducibility.py)
6. High-risk coverage below 100% fully protected (report_high_risk_coverage.py --require-full)
Additive new operations are allowed; they must appear in the capability table
and registry export in the same change.

## cargo-semver-checks

CI runs `cargo-semver-checks` against the last crates.io release. On **0.x**:

- Additive public APIs are expected
- Breaking changes require explicit review and changelog notes
- Do not â€œfixâ€ failures by weakening MSRV or hiding types without product intent

## Idempotency and cancellation

- Financial writes without a caller-owned sticky key: **â‰¤ 1** HTTP attempt
- Ambiguous delivery (`Unknown`) is fail-closed without a sticky key
- Dropping an in-flight write future after transmit is **Unknown** â€” document sticky keys for any write the app may cancel/retry

## Release bands (honest)

| Band | Meaning |
| ---- | ------- |
| NOT READY | Kernel or high-risk profile gaps open |
| NEAR READY | Kernel frozen; money facades landed; docs/assurance incomplete |
| RC READY | Assurance pyramid + drift gates green; residual P1 tracked |
| 1.0 READY | Only if release checklist + hostile review pass |

See `docs/sdd/1.0-readiness/` for the program plan.

## Tier-S request allowlists (drift program)

Machine-readable allowlists live in docs/registries/tier-s-request-contracts.yaml and are validated by scripts/check_tier_s_request_contracts.py.

## cargo-semver-checks posture

The CI semver job remains **advisory** on 0.x until a crates.io baseline + Tier-G noise strategy is finalized (continue-on-error: true). Blocking Tier-S snapshot via cargo public-api is a follow-up before 1.0 RC.

