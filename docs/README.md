# Documentation

Reference material for `mollie-rs`. Start with the
[crate README](../README.md) for installation and a first payment; come here for
depth, contracts, and evidence.

Some files in this tree are **generated** and are rewritten by scripts or by
running the examples — those are marked below. Do not hand-edit them.

## Start here

| Document | What it answers |
| --- | --- |
| [`api-overview.md`](api-overview.md) | What the client surface looks like end to end |
| [`architecture.png`](architecture.png) | How the layers fit together |
| [`compatibility.md`](compatibility.md) | Supported Rust versions and platforms |
| [`API-STABILITY.md`](API-STABILITY.md) | What is stable, what is beta, what may change |
| [`release-readiness.md`](release-readiness.md) | Honest 0.7 → 1.0 readiness band |
| [`production-checklist.md`](production-checklist.md) | Before you go live |
| [`guides/README.md`](guides/README.md) | Task-oriented production guides |

## Guides

Task-oriented walkthroughs in [`guides/`](guides):

- [`safe-payment-retry.md`](guides/safe-payment-retry.md) — sticky idempotency, retries, `DeliveryOutcome`
- [`payments.md`](guides/payments.md) — create, cancel, customer payments, streams
- [`refunds.md`](guides/refunds.md) — refund create/cancel/list
- [`handle-signed-webhook.md`](guides/handle-signed-webhook.md) — Next-gen webhook signatures
- [`payouts-and-transfers.md`](guides/payouts-and-transfers.md) — payouts and business-account transfers
- [`oauth-connect.md`](guides/oauth-connect.md) — OAuth tokens and Connect balance transfers
- [`pagination.md`](guides/pagination.md) — cursors, budgets, streams
- [`multi-merchant.md`](guides/multi-merchant.md) — credential and profile scoping
- [`error-handling.md`](guides/error-handling.md) — structured errors and safe logging
- [`testing.md`](guides/testing.md) — unit, contract, live, and fuzz layers

## Contracts and specifications

| Directory | Contents |
| --- | --- |
| [`contracts/`](contracts) | Per-type field contracts |
| [`specs/`](specs) | Design specifications and audits |
| [`iso/`](iso) | ISO standards enforced by the crate |
| [`sdd/`](sdd) | Spec-driven development / 1.0 readiness program |

## Coverage and drift

Generated — rebuilt by tooling, not by hand (except where noted):

| Document | Rebuilt by |
| --- | --- |
| [`route-coverage.md`](route-coverage.md) | Client generation |
| [`route-examples.md`](route-examples.md) | `scripts/check_route_examples.sh` |
| [`example-support-matrix.md`](example-support-matrix.md) | Examples / `scripts/rebuild_example_support_matrix.py` |
| [`rc/workflow-example-matrix.md`](rc/workflow-example-matrix.md) | Tier-S workflows → examples; `scripts/check_workflow_examples.py` |
| [`registries/tier-s-workflow-examples.yaml`](registries/tier-s-workflow-examples.yaml) | Machine source for EX-001 CI gate |
| [`postman-response-matrix.md`](postman-response-matrix.md) | `scripts/generate_postman_matrix.py` |
| [`api-drift-report.md`](api-drift-report.md), [`api-drift-report-upstream.md`](api-drift-report-upstream.md) | `scripts/report_api_drift.py` |
| [`registries/operation-registry.yaml`](registries/operation-registry.yaml) | `scripts/export_operation_registry.py` |
| [`registries/provider-maturity.yaml`](registries/provider-maturity.yaml) | Hand-maintained; consumed by `export_operation_registry.py` |
| [`registries/high-risk-coverage.md`](registries/high-risk-coverage.md) | `scripts/report_high_risk_coverage.py` |

The operation registry is a CI-enforced source of truth: it must agree with
`src/route_capabilities.rs` and `specs/upstream-pin.toml`. High-risk coverage
must stay at 100% fully protected under the frozen denominator.

## Release and audit evidence

| Directory | Contents |
| --- | --- |
| [`rc/`](rc) | Release-candidate evidence: checklist, scorecard, live matrix, SBOM notes |
| [`audits/`](audits) | Point-in-time audits and official-SDK parity assessments |

These are historical records tied to specific versions. Check the version each
one refers to before relying on it.

## Generation and tooling

- [`openapi-generation.md`](openapi-generation.md) — how the client is generated
- [`example-runtime-config.md`](example-runtime-config.md) — env and CLI options shared by examples
- [`dependency-graph.png`](dependency-graph.png) — module dependency direction

See also [`/specs/README.md`](../specs/README.md) for the upstream spec pin and drift policy.

## Licensing note

Documentation derived from Mollie's OpenAPI specification inherits that
document's **CC BY-NC-SA 4.0** terms, which differ from this crate's code
licence. See [`../NOTICE`](../NOTICE).
