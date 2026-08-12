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
| [`production-checklist.md`](production-checklist.md) | Before you go live |

## Guides

Task-oriented walkthroughs in [`guides/`](guides):

- [`safe-payment-retry.md`](guides/safe-payment-retry.md) — retrying without
  double-charging
- [`handle-signed-webhook.md`](guides/handle-signed-webhook.md) — verifying
  Next-gen webhook signatures
- [`payouts-and-transfers.md`](guides/payouts-and-transfers.md) — balance to bank

## Contracts and specifications

| Directory | Contents |
| --- | --- |
| [`contracts/`](contracts) | Per-type field contracts — one file per data type (`amountValue`, `countryCode`, `apiKey`, …), 35 in total |
| [`specs/`](specs) | Design specifications and audits: domain facade contract, webhook verification, baseline forensics |
| [`iso/`](iso) | The ISO standards this crate enforces (3166-1, 4217, 8601, 15897) and how |
| [`sdd/`](sdd) | Spec-driven development documents, `00`–`16`, tracking requirements per domain |

## Coverage and drift

Generated — rebuilt by tooling, not by hand:

| Document | Rebuilt by |
| --- | --- |
| [`route-coverage.md`](route-coverage.md) | Client generation |
| [`route-examples.md`](route-examples.md) | `scripts/check_route_examples.sh` |
| [`example-support-matrix.md`](example-support-matrix.md) | Running any example (or `scripts/rebuild_example_support_matrix.py`) |
| [`postman-response-matrix.md`](postman-response-matrix.md) | `scripts/generate_postman_matrix.py` |
| [`api-drift-report.md`](api-drift-report.md), [`api-drift-report-upstream.md`](api-drift-report-upstream.md) | `scripts/report_api_drift.py` |
| [`registries/operation-registry.yaml`](registries/operation-registry.yaml) | `scripts/export_operation_registry.py` |

The operation registry is a CI-enforced source of truth: it must agree with
`src/route_capabilities.rs` and `specs/upstream-pin.toml`.

## Release and audit evidence

| Directory | Contents |
| --- | --- |
| [`rc/`](rc) | Release-candidate evidence: checklist, baseline, live test matrix, hostile-transport evidence |
| [`audits/`](audits) | Point-in-time audits and official-SDK parity assessments |

These are historical records tied to specific versions. They are kept for
traceability and are **not** continuously updated — check the version each one
refers to before relying on it.

## Generation and tooling

- [`openapi-generation.md`](openapi-generation.md) — how the client is generated
- [`example-runtime-config.md`](example-runtime-config.md) — env and CLI options
  shared by all examples
- [`dependency-graph.png`](dependency-graph.png) — module dependency direction

See also [`/specs/README.md`](../specs/README.md) for the upstream spec pin and
drift policy.

## Licensing note

Documentation derived from Mollie's OpenAPI specification inherits that
document's **CC BY-NC-SA 4.0** terms, which differ from this crate's code
licence. See [`../NOTICE`](../NOTICE).
