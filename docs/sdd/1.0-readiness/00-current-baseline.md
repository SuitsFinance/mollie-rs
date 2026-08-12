# SDD 00 — Current baseline freeze (`mollie-rs` exceptional 1.0 program)

**Status:** Phase 0 freeze (this session)  
**Crate version:** `0.7.0` (`Cargo.toml`)  
**HEAD:** `e3358d2e49cb065d690deea8b43cdf2c9ed93a8a`  
**Branch:** `floris-xlx-cuddly-umbrella`  
**MSRV:** `1.88`  
**Freeze date:** 2026-08-12  
**Honest band:** **NEAR READY → RC path** (kernel + high-risk facades landed; RC evidence + guide matrix incomplete)  
**Profile:** `payment-sdk`  
**Supersedes for inventory claims:**  
- `docs/sdd/1.0-readiness/00-baseline.md` (HEAD `55187ee`, 7 Tier-S modules) — **STALE facade inventory**  
- `docs/sdd/1.0-readiness/15-rc-baseline.md` (HEAD `df6a9d4`) — still useful for RC checklist shape; **re-pin SHA here**  
- Mission paste “implement payouts/transfers/oauth from scratch” — **STALE** relative to HEAD (facades present)

Companion elite pack (corpus): `docs/sdd/xylex/mollie-rs/` under `XYLEX_SDD_ROOT`  
Companion program: [`16-exceptional-1.0-program.md`](16-exceptional-1.0-program.md)

---

## 1. Mission lock

Primary release metric is **not** OpenAPI operation count (already 124/124).

```text
high_risk_operations_with_enforced_safety_invariants
---------------------------------------------------
total_high_risk_operations
                                          → 100% for 1.0.0
```

An operation counts as **fully protected** only when profile + transport enforcement + delivery semantics + secret-leak coverage + negative tests + justified Tier-S (when material) all hold. See `01-high-risk-operation-inventory.md`.

---

## 2. Pins and measured inventory

| Item | Value | Evidence | Confidence |
| --- | --- | --- | --- |
| HEAD | `e3358d2e49cb065d690deea8b43cdf2c9ed93a8a` | `git rev-parse HEAD` | Verified-static |
| Crate | `mollie-rs` `0.7.0` | `Cargo.toml` | Verified-static |
| MSRV | `1.88` | `Cargo.toml` `rust-version` | Verified-static |
| Local OpenAPI ops | **124** | `specs-3.0.yaml` `operationId` count | Verified-static |
| Tier-G `pub async fn` | **124** | `rg "pub async fn" src/routes` | Verified-static |
| Capabilities / profiles | **124** unique | `src/route_capabilities.rs` parse + `check_generation_reproducibility.py` | Verified-build |
| High-risk writes (CI denominator) | **16/16** | `scripts/check_dangerous_profile_drift.py` `HIGH_RISK_WRITES` | Verified-build |
| ValidatedFacade ops | **18** | capability `access: ValidatedFacade` | Verified-static |
| Tier-S domain modules | **14 facades + `common`** | `src/domain/mod.rs` | Verified-static |
| `OperationSafetyProfile` | type alias of `RouteCapability` + derived classes | `src/operation_safety.rs` | Verified-static |
| `DeliveryOutcome` | present | `src/transport/delivery.rs`, `MollieError::delivery_outcome` | Verified-static |
| Examples | **126** `examples/*.rs` | directory count | Verified-static |
| Production guides (mission list) | **3 / 12** | `docs/guides/*` | Verified-static |
| `API-STABILITY.md` | present | `docs/API-STABILITY.md` | Verified-static |
| Fuzz targets | 6 | `fuzz/fuzz_targets` | Documented + prior freeze |
| SBOM / build provenance | **absent** | search | Verified-static |
| Mutation testing | **absent** | search | Verified-static |
| `stream_pages` / `stream_items` | **absent** | `rg fn stream_` | Verified-static |
| Connect balance Tier-S facade | **absent** (Tier-G only) | `src/routes/connect.rs`; no `src/domain/connect*` | Verified-static |
| Retry-After HTTP-date | **ignored** (numeric only) | `src/metadata.rs` `parse_retry_after` doc + test | Verified-static |

### 2.1 Architecture (preserve)

```text
Application
    │
    ▼
Tier S — MollieClient domain facades, validated requests, typed IDs, webhooks
    │
    ▼
Tier G — generated OpenAPI routes (124) + types
    │
    ▼
Transport safety kernel — reqwest, OperationSafetyProfile/RouteCapability,
  RetryPolicy (off by default), sticky idempotency, DeliveryOutcome,
  hooks, timeouts, redirect none, pagination origin allowlist
    │
    ▼
Mollie API
```

**Singular policy SSOT:** `RouteCapability` / `OperationSafetyProfile` in `src/route_capabilities.rs`. No second registry.

---

## 3. Gate run log (this freeze)

| Gate | Result | Confidence |
| --- | --- | --- |
| `python scripts/check_generation_reproducibility.py` | **PASS** 124=124 | Verified-build |
| `python scripts/check_dangerous_profile_drift.py` | **PASS** 16 high-risk | Verified-build |
| `python scripts/report_api_drift.py` | **PASS** local 124 (no upstream snapshot this run) | Verified-build |
| `cargo check --all-targets --all-features` | **PASS** | Verified-build |
| `cargo fmt --check` | **NOT RE-RUN** this session | Unverified |
| `cargo clippy … -D warnings` | **NOT RE-RUN** this session | Unverified |
| `cargo test --all-features` | **NOT RE-RUN** this session | Unverified (prior RC freeze PASS) |
| `cargo deny check` | **NOT RE-RUN** this session | Unverified (prior PASS) |
| `cargo audit` | **NOT RUN** | Unverified |
| `cargo semver-checks` | **NOT RUN** locally | Documented CI job |

---

## 4. Mission phase truth table (summary)

| Mission band | Status at `e3358d2` | Notes |
| --- | --- | --- |
| P0 baseline freeze | **Refresh now** | This document |
| P1 profile SSOT | **Closed v1** | Alias + derived classes; expand fields only if dual-core avoided |
| P2 high-risk freeze | **Partial** | CI set=16; mission seed≈21 — denominator incomplete |
| P3 payouts Tier-S | **Closed** | `domain/payouts.rs` |
| P4 transfers Tier-S | **Closed** | `domain/transfers.rs` + signature type |
| P5 OAuth Tier-S | **Closed** | `domain/oauth.rs` + secret types |
| P6 Connect balance Tier-S | **Open** | Tier-G only |
| P7 DeliveryOutcome | **Closed** | Kernel + error helpers |
| P8 retry model tests | **Closed (unit sim)** | `simulate_retry_loop`; proptest optional |
| P9 Retry-After HTTP-date | **Open** | Explicitly ignored today |
| P10 domain types | **Partial** | Strong IDs/money; audit loose strings remains |
| P11 forward enums | **Unverified** | Needs enum audit evidence |
| P12 pagination streams | **Open** | `list_page`/`list_all` only; no stream_* |
| P13 webhook adversarial | **Partial** | Fuzz + verify present; expand adversarial matrix |
| P14 secret leak suite | **Closed (base)** | `src/secret_leak_tests.rs` |
| P15 semantic OpenAPI drift | **Partial** | Dangerous profile drift blocking; full semantic upstream classifier incomplete |
| P16 gen reproducibility | **Closed** | CI generation job |
| P17 public API grammar | **Partial** | Facades exist; stream grammar + connect missing |
| P18 Tier-G lower-level | **Partial** | Docs/API-STABILITY; no `raw()` rename required pre-1.0 |
| P19 error taxonomy | **Strong / polish** | Many helpers; normalize remaining gaps |
| P20 observability allowlist | **Partial** | Hooks/redaction; formal allowlist tests |
| P21 failure injection | **Partial** | WireMock + property; expand hostile matrix |
| P22 concurrency | **Partial** | Credential isolation test closed; broaden |
| P23 mutation testing | **Open** | Not present |
| P24 fuzzing | **Partial** | 6 targets + CI build; smoke duration limited |
| P25 differential contract | **Partial** | Postman/http_contract; expand |
| P26 documentation guides | **Open** | 3/12 mission guides |
| P27 examples | **Strong** | 126 examples; keep Tier-S preferred |
| P28 API stability policy | **Closed** | `docs/API-STABILITY.md` |
| P29 semver enforcement | **Closed (CI)** | `cargo-semver-checks` job |
| P30 supply chain | **Partial** | deny present; audit triage open |
| P31 SBOM/provenance | **Open** | |
| P32 CI release gates | **Strong / gaps** | RC checklist open rows |
| P33 invariant suite | **Partial** | property + drift IDs; expand INV-* release suite docs |
| P34 readiness scorecard | **Open as generator** | Manual RC checklist only |

---

## 5. Finding seed (catalog-mapped)

| Local ID | Catalog | Sev | Title | Status |
| --- | --- | --- | --- | --- |
| HR-001 | PAY-008 / CON-003 | P1 | High-risk denominator incomplete vs mission financial set | Open |
| HR-002 | CON-003 | P1 | `create_connect_balance_transfer` Tier-G only | Open |
| KER-RA-01 | PAY-004 | P2 | Retry-After HTTP-date ignored | Open |
| PAG-001 | CON-003 | P2 | No `stream_pages`/`stream_items` grammar | Open |
| DOC-GUIDE-01 | CON-004 | P1 | Mission production guides 3/12 | Open |
| REL-001 | CI-001 | P1 | RC hostile review / package audit / live green missing | Open |
| REL-002 | — | P2 | SBOM + provenance absent | Open |
| TEST-MUT-01 | — | P3 | Mutation testing absent | Open |
| KER-001…007 | PAY/AUT/WHK | P0 | Delivery, redirect, origin, profile, retry sim, cancel docs, concurrency | **Closed** (prior) |
| FAC-001 | CON-003 | P1 | Payouts/transfers/oauth facades | **Closed** |

---

## 6. Must not regress

Do not destroy: 124/124 ops, capability parity, sticky-key write retries, retries-off-default, DeliveryOutcome, pagination budgets/cycle/origin, redirect none, scoped credentials, hooks, redaction/zeroize feature, webhook HMAC + constant-time compare, generation reproducibility, MSRV, cargo-deny, WireMock contracts, typed money/IDs, structured errors.

Architecture non-goals (mission): no generic PSP layer, no ledger embedding, no reqwest replacement, no global mutable config, no default financial write retries, no dual profile registries.

---

## 7. Exit criteria for Phase 0

- [x] HEAD/version/MSRV recorded  
- [x] 124/124 parity re-measured  
- [x] High-risk CI set re-measured (16)  
- [x] Tier-S inventory re-measured (14)  
- [x] Mission phases mapped Closed/Open/Partial  
- [x] Stale audit SHAs marked  
- [ ] Full gate matrix green this SHA (fmt/clippy/test/deny) — complete in Phase 0 close-out or first implement slice  
- [x] Follow-on inventories linked (`01`, `02`, program `16`)
