# Mollie contract delta vs GitHub `main` (implementation baseline)

**Status:** Phase 0 freeze (implemented)  
**HEAD:** `98025b9`  
**Crate:** `mollie-rs` `0.7.1`  
**MSRV:** `1.88`  
**Upstream pin:** `specs/upstream-pin.toml` sha256 `d2a3bd80c1fa029268521066e1082aa781b2ddeee911c6172be0321f0ba8014e` (124 ops, pinned 2026-08-14)  
**Truth rule:** GitHub `main` only. Linear status ignored.  
**Freeze date:** 2026-08-18  

## Gate samples (this freeze)

| Gate | Result | Notes |
| --- | --- | --- |
| `python scripts/check_generation_reproducibility.py` | **PASS** | 124=124 |
| `python scripts/check_dangerous_profile_drift.py` | **PASS** | 23 high-risk writes |
| `python scripts/report_high_risk_coverage.py --require-full` | **PASS** | 23/23 |
| `python scripts/export_operation_registry.py` | **PASS** | registry export OK |
| `cargo-semver-checks` job | **advisory** | `continue-on-error: true` + `|| echo` (Phase 6 hardens) |

## Architecture (preserve)

```text
Application → Tier-S facade → Tier-G generated → OperationSafetyProfile → Transport kernel → Mollie API
```

Missing layer addressed by this program: **Semantic Drift Analyzer** (`scripts/contract_diff`) with quarantine + explicit Tier-S promotion.

## Delta table (main_status)

| Change | Tier-G | Tier-S | Tests | main_status | Action |
| --- | --- | --- | --- | --- | --- |
| Capture `testmode` | yes | verify | partial | landed | leave + snapshots |
| Locales `en_BE` / `en_NL` | yes | locale | verify | landed | leave |
| `dueDate` tri-state | Option | builders | weak | incomplete | NullableField + registry |
| ShippingAddress | present variant | N/A | weak | inspect | classifier replacement |
| Billink | yes | constant | weak | landed | history fixtures |
| Settlements distinct ops | yes | none | weak | incomplete | model audit |
| Terminal pairing 403 | not in pin OpenAPI | terminals | weak | incomplete | error-contract fixture |
| `requiredCustomerDetails` | yes | sessions | weak | incomplete | private-beta quarantine |
| DraftTransfers removed | absent | absent | missing | missing test | mini-spec regression |
| Balance / security enums | closed gen | N/A | missing | missing | OpenEnum |
| Method/issuer enable/disable | generated | not HR | weak | incomplete | OperationRisk |
| Semantic contract_diff | — | — | — | implementing | Phases 1–2 |
| OpenEnum / NullableField | — | — | — | implementing | Phase 4 |
| OperationRisk / Exposure | — | — | — | implementing | Phase 5 |
| Mutation auto-discovery | — | — | HR manual | implementing | Phase 5 |
| Tier-S allowlists / API snap | partial | partial | partial | incomplete | Phase 6 |
| Blocking semver | — | — | — | incomplete | Phase 6 |
| VerifiedWebhook recover | — | verify_and_decode | base | implementing | Phase 7 |
| `configure_http` | — | http_client footgun | docs | implementing | Phase 7 |
| Provenance / canary / corpus | pin only | — | — | implementing | Phase 8 |
| Live/sandbox/hostile/RC | — | — | incomplete | incomplete | Phase 9 |

## STALE prior SDD claims

| Claim | Correction |
| --- | --- |
| Drift-ready because residual 1.0 closed | **STALE** — HR facades landed; semantic drift stack was missing |
| Baseline SHA `e3358d2` / 0.7.0 | **STALE** — use `98025b9` / 0.7.1 |
| Semver “Closed (CI)” | **STALE** — job non-blocking until Phase 6 |
| stream_pages absent / Connect Tier-S absent / HR=16 | **STALE** — streams, connect facade, HR=23 landed |

## Program DoD invariant

> A Mollie upstream change can never silently change financial behavior exposed through the stable SDK.

See `docs/sdd/1.0-readiness/FINDINGS.md` drift program rows and `scripts/contract_diff`.
