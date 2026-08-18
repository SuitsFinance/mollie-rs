# API stability (mollie-rs 0.8.x - 1.0)

This document is the public contract posture for the crate. It is **not** by
itself a claim that `1.0.0` is ready - see `docs/rc/1.0-acceptance-matrix.md`.

## Tier model (SSOT)

| Tier | Surface | Stability intent | Blocking gates |
| ---- | ------- | ---------------- | -------------- |
| **S** | `MollieClient::{payments,refunds,...}()` facades, validated builders, webhooks, curated crate-root safety exports | Prefer for application code. Removals/renames of Tier-S surface require changelog + snapshot refresh. | `python scripts/check_tier_s_public_api.py` (CI `contract`); `python scripts/check_tier_s_request_contracts.py` |
| **G** | Generated `Client` route methods + `types::*` | Tracks the pinned OpenAPI (`specs-3.0.yaml`). Field/enum/route churn follows the provider pin. Not a frozen application API. | Generation reproducibility + OpenAPI pin digest + drift fixtures |
| **Kernel** | Transport retry, delivery outcomes, redirects, body limits, pagination host policy, `OperationSafetyProfile` | Behavioral safety contracts. Tightening is allowed; loosening financial fail-closed rules is **not**. | unit/integration tests + dangerous-profile drift |

### Application guidance

- **Write app code against Tier-S** (`MollieClient` facades + validated builders).
- Use Tier-G (`Client` / `types::*`) only when a facade does not yet exist, or for
  advanced/generated coverage - expect churn on OpenAPI repins.
- Do **not** treat generated types as a long-term semver surface for 1.0 apps.

### Low-level HTTP escape

- Safe path: `MollieClientBuilder` (redirect-none, TLS 1.2+, auth headers,
  timeouts, `ResponseLimits` - see `docs/rc/transport-security-policy.md`).
- Unrestricted path: `MollieClient::from_generated` / `Client::new_with_client`
  only. There is no builder `http_client` inject.

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
and docs must reflect GA - not leave stale beta-only registry labels.

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
5. Local OpenAPI pin vs capability inventory mismatch (`check_generation_reproducibility.py`)
6. High-risk coverage below 100% fully protected (`report_high_risk_coverage.py --require-full`)

Additive new operations are allowed; they must appear in the capability table
and registry export in the same change.

## cargo-semver-checks (fail-closed)

CI job `semver` runs `cargo semver-checks check-release` against the last
**crates.io** release and is **blocking** (no `continue-on-error`, no swallowed
exit code).

On **0.x**:

- A **minor** bump (e.g. `0.7.1` -> `0.8.0`) may include intentional breaking
  changes; cargo-semver-checks treats that as the 0.x contract.
- A **patch** bump must remain additive for the published public API.
- Intentional breaks require: version bump, `CHANGELOG.md` entry, and (for
  Tier-S surface) snapshot refresh via
  `python scripts/check_tier_s_public_api.py --write`.

Tier-S snapshot and cargo-semver-checks are **complementary**:

| Gate | Scope | Role |
| ---- | ----- | ---- |
| `check_tier_s_public_api.py` | Curated facade/builder/safety symbols | Application-facing rename/removal tripwire |
| `cargo-semver-checks` | Full crate public API vs crates.io | Release-grade rustc API lint (includes Tier-G) |

Do not "fix" failures by weakening MSRV or hiding types without product intent.

## Tier-S public API snapshot (blocking)

Machine-checked facade surface:

- Registry: `docs/registries/tier-s-public-api.snapshot`
- Gate: `python scripts/check_tier_s_public_api.py` (CI `contract` job)
- Refresh (intentional changes only): `python scripts/check_tier_s_public_api.py --write`

This gate catches renames/removals of Tier-S facades, builder types, and critical
safety exports.

## Tier-S request allowlists (drift program)

Machine-readable allowlists live in
`docs/registries/tier-s-request-contracts.yaml` and are validated by
`scripts/check_tier_s_request_contracts.py`.

## Idempotency and cancellation

- Financial writes without a caller-owned sticky key: **<= 1** HTTP attempt
- Ambiguous delivery (`Unknown`) is fail-closed without a sticky key
- Dropping an in-flight write future after transmit is **Unknown** - document sticky keys for any write the app may cancel/retry

## Release bands (honest)

| Band | Meaning |
| ---- | ------- |
| NOT READY | Kernel or high-risk profile gaps open |
| NEAR READY | Kernel frozen; money facades landed; docs/assurance incomplete |
| RC READY | Assurance pyramid + drift gates green; residual P1 tracked |
| 1.0 READY | Only if release checklist + hostile review pass |

See `docs/rc/` and `docs/sdd/1.0-readiness/` for the program plan.