# mollie-rs — Exceptional 1.0 safety program (elite)

## Goal

Evolve `mollie-rs` `0.7.0` @ `e3358d2` into a **1.0.0** payment SDK judged by **enforced high-risk safety invariants**, not by OpenAPI operation count.

## Repositories

| Role | Repo | Branch / path |
| --- | --- | --- |
| Target | `SuitsFinance/mollie-rs` | `floris-xlx-cuddly-umbrella` @ `e3358d2e49cb065d690deea8b43cdf2c9ed93a8a` |
| Workspace | worktree | `C:\Users\floris\.copilot\copilot-worktrees\mollie-rs\floris-xlx-cuddly-umbrella` |
| Corpus pack | `spec-driven-development` | `docs/sdd/xylex/mollie-rs/` |

## Context

Mission brief (pasted 2026-08-12) describes a 0→34 phase exceptional program. **HEAD already closed** large slices of that mission (kernel DeliveryOutcome, redirect none, pagination origin, OperationSafetyProfile v1, payouts/transfers/oauth/terminals/UCT/sessions Tier-S, secret-leak tests, generation parity, dangerous profile drift CI, API-STABILITY, 126 examples).

This plan is a **retrofit + residual program**: Phase 0 freezes truth; later phases implement only **Open/Partial** gaps. Greenfield “add payouts facade” work must not be re-opened as if missing.

Evidence SSOT freeze: [`00-current-baseline.md`](00-current-baseline.md), [`01-high-risk-operation-inventory.md`](01-high-risk-operation-inventory.md), [`02-public-api-inventory.md`](02-public-api-inventory.md).  
Prior: `15-rc-baseline.md`, `FINDINGS.md`, `docs/rc/rc-checklist.md` (RC path).

**Spec maturity:** Level 3 architecture specification (R13) for Tier S/G/kernel ownership.

## Working assumptions

- Profile: `payment-sdk`
- Primary metric: `fully_protected_high_risk / total_high_risk == 100%` with frozen denominator
- Preserve existing architecture layers; extend `RouteCapability` rather than dual registries
- Pre-1.0: additive > deprecation > breaking
- No product code in assess-only turns; implement via dual-suite RED→GREEN per phase
- Official SDK parity is secondary to financial safety

## Non-goals

- Generic PSP / multi-acquirer abstraction
- Embedding merchant ledger or reconciliation engine
- Replacing reqwest or inventing async trait pyramids
- Hand-rewriting generated route surface
- Global mutable client state
- Default-on financial write retries
- Treating 124/124 as the 1.0 done metric
- Self-scored “9.8/10” without ACT/gates

## Forbidden shortcuts (EP-13)

- Dual profile registries (OWN-001)
- Closing runtime gates with static-only evidence (R12)
- Inventing catalog IDs outside `catalogs/05`
- Claiming RC/1.0 READY while `docs/rc/rc-checklist.md` open P0/P1 rows remain

---

## Why this architecture exists

**Goals:** reduce financial double-charge ambiguity; make delivery uncertainty explicit; keep one policy SSOT; prefer validated Tier-S for money; keep Tier-G complete; make unsafe retries unrepresentable; keep secrets out of diagnostics; pin OpenAPI with reproducible generation.

**Non-goals:** hide Mollie-specific behavior; orchestrate multi-PSP; own webhook replay storage universally.

---

## Architecture invariants (after Phase 1 freeze)

True at every commit after Phase 1 exit. Doctor/CI must fail if false.

| ID | Invariant | Enforcement |
| --- | --- | --- |
| INV-PROF-01 | Exactly one operation policy SSOT (`ROUTE_CAPABILITIES` / `OperationSafetyProfile`) | `check_generation_reproducibility.py` + no second table |
| INV-PARITY-01 | OpenAPI ops == generated routes == capability rows | generation CI |
| INV-WRITE-01 | No automatic financial mutation retry without profile approval | `RetryPolicy::allows` + property_tests |
| INV-WRITE-02 | IdempotentWrite multi-attempt requires sticky caller-owned key | transport + property_tests |
| INV-DELIV-01 | Possible transmit without final state ⇒ `DeliveryOutcome::Unknown`; never silently NotSent | delivery.rs + error mapping |
| INV-DEADLINE-01 | No new attempt starts after retry budget/deadline exhaustion | Client::send + sim |
| INV-REDIRECT-01 | Authenticated client never follows redirects | reqwest Policy::none + http_contract |
| INV-ORIGIN-01 | Pagination next links cannot escape trusted API origin | PageCursor allowlist tests |
| INV-CRED-01 | Scoped credentials do not cross concurrent client contexts | concurrency test |
| INV-SECRET-01 | Secrets absent from Debug/Display/errors/hooks/tracing captures | secret_leak_tests |
| INV-WHK-01 | Verify-before-decode for signed webhooks | webhook_verify + fuzz |
| INV-DRIFT-01 | Dangerous high-risk profile drift fails CI | check_dangerous_profile_drift.py |
| INV-TIER-01 | Frozen high-risk set is 100% fully protected before 1.0 tag | coverage generator + scorecard |

**Edges not shown are forbidden:** Application ↛ generated types bypassing Tier-S for new high-risk app code (docs/lint guidance); domain ↛ second HTTP stack; transport ↛ business ledger; generated ↛ hand policy tables.

### Ownership matrix (M-20)

| Concern | Canonical owner | Must not own |
| --- | --- | --- |
| Operation policy | `route_capabilities` / `operation_safety` | facades inventing retry rules |
| HTTP send/retry/delivery | `lib.rs` Client + `transport/*` | domain reimplementing send |
| Tier-S validation | `domain/*` + write_requests | routes doing money validation |
| Tier-G completeness | `routes/*` + generator | manual route edits |
| Webhook crypto | `webhook_verify` | app frameworks in core crate |
| Registry export | `scripts/export_operation_registry.py` | ad-hoc markdown counts |
| Release gates | `.github/workflows/ci.yml` + rc docs | local-only “trust me” |

### Package must-never

- Never log Authorization / client secrets / webhook secrets  
- Never default RetryPolicy to on for writes  
- Never follow redirects with credentials  
- Never treat Unknown as safe replay without sticky key  
- Never add float money  
- Never dual-own capability tables  

### Surfaces

| Surface | Allowlist concept |
| --- | --- |
| Runtime public | `src/lib.rs` exports + domain accessors |
| Generated | `src/routes/*`, `src/types.rs` (generator-owned) |
| Canonical models long-term | typed domain modules + `ids`/`money` (not a single models blob) |
| N/N-1 | Tier-S additive; Tier-G pin-driven; kernel fail-closed may tighten |

### Performance budgets (initial)

| Budget | Target | Gate |
| --- | --- | --- |
| Default request timeout | existing builder defaults | doc + test |
| Pagination max pages/items | existing PaginationGuard | unit |
| Webhook body | existing max | unit/fuzz |
| CI full test wall | keep under team SLA | workflow |

### Architecture debt (AD-*)

| ID | Title | Removal target | Status |
| --- | --- | --- | --- |
| AD-01 | High-risk denominator split (CI 16 vs mission/ValidatedFacade) | Phase 2 | Open |
| AD-02 | Connect balance Tier-G only | Phase 3 | Open |
| AD-03 | Retry-After HTTP-date ignored | Phase 4 | Open |
| AD-04 | No stream_* pagination grammar | Phase 5 | Open |
| AD-05 | Production guide matrix incomplete | Phase 6 | Open |
| AD-06 | RC hostile/package/live evidence incomplete | Phase 7 | Open |
| AD-07 | SBOM/provenance missing | Phase 7 | Open |
| AD-08 | Mutation testing absent | Phase 8 (optional) | Open |
| AD-09 | OperationSafetyProfile still thin alias | only if fields required | Accepted v1 |

### ACT-* (Architecture Conformance Suite)

| ACT | Proof |
| --- | --- |
| ACT-PARITY | generation reproducibility script |
| ACT-HR | dangerous profile drift + future coverage report |
| ACT-DELIV | property_tests + delivery unit |
| ACT-REDIRECT | http_contract foreign redirect |
| ACT-ORIGIN | pagination off-origin tests |
| ACT-SECRET | secret_leak_tests |
| ACT-WHK | webhook verify/fuzz |
| ACT-SEMVER | cargo-semver-checks CI |
| ACT-DENY | cargo deny |

---

## Findings inventory

| ID | Catalog | Sev | Title | Status | Phase |
| --- | --- | --- | --- | --- | --- |
| HR-001 | PAY-008, CON-003 | P1 | High-risk denominator incomplete | Open | 2 |
| HR-002 | CON-003 | P1 | Connect balance transfer no Tier-S | Open | 3 |
| KER-RA-01 | PAY-004 | P2 | Retry-After HTTP-date ignored | Open | 4 |
| PAG-001 | — | P2 | stream_pages/items missing | Open | 5 |
| DOC-GUIDE-01 | CON-004 | P1 | Guides 3/12 | Open | 6 |
| REL-001 | CI-001 | P1 | RC checklist open (hostile, package, live green) | Open | 7 |
| REL-002 | — | P2 | SBOM/provenance | Open | 7 |
| TEST-MUT-01 | — | P3 | Mutation testing | Open | 8 |
| OBS-001 | AUT-003 | P2 | Observability allowlist formalization | Open | 6 |
| ENUM-001 | CON-006 | P2 | Response enum forward-compat audit | Open | 5 |
| KER-001…007 | PAY/AUT | P0 | Kernel slice | **Closed** | — |
| FAC-payouts/transfers/oauth | CON-003 | P1 | Facades | **Closed** | — |

---

## Spec catalogue

| Path | Purpose |
| --- | --- |
| `00-current-baseline.md` | Phase 0 freeze |
| `01-high-risk-operation-inventory.md` | Denominator + rubric |
| `02-public-api-inventory.md` | Tier-S grammar |
| `16-exceptional-1.0-program.md` | This program SSOT |
| `docs/API-STABILITY.md` | Stability tiers |
| `docs/rc/rc-checklist.md` | RC evidence board |
| Corpus `docs/sdd/xylex/mollie-rs/` | Elite pack mirror |

---

## Phases

### Phase 0 — Baseline freeze

### Objectives

Record HEAD truth; supersede stale facade/kernel claims; seed residual findings.

### In scope / out of scope

In: inventories, gate samples, stale banners. Out: product code.

### Work

- Write `00-current-baseline.md`, `01-…`, `02-…`, this program  
- Mark `00-baseline.md` / partial `15-rc-baseline.md` SHA drift  
- Seed AD-* / findings  

### Catalog IDs

CON-004 (stale counts), baseline for PAY/CON.

### Verification

| Kind | Command / proof |
| --- | --- |
| Static | files exist @ HEAD |
| Build | `check_generation_reproducibility.py`, `check_dangerous_profile_drift.py`, `cargo check --all-targets --all-features` |

### Acceptance

- [x] SHA/version/MSRV pinned  
- [x] 124/124 remeasured  
- [x] Mission map Closed/Open  
- [ ] Full fmt/clippy/test matrix on this SHA (carry to Phase 0 exit PR)

### Residual risks

Unverified full test suite this session; prior RC freeze PASS not re-proven.

### Architecture debt delta

AD-01…09 seeded.

---

### Phase 1 — Invariant + ACT freeze (no dual cores)

### Objectives

Publish invariants/ACT as fail-closed CI contracts; confirm profile SSOT; document doctor commands.

### Work

- Ensure CI jobs map 1:1 to INV-* / ACT-* in `docs/sdd/1.0-readiness/` matrix  
- Optionally extend `operation_safety` docs only (no second registry)  
- Dual-suite characterization: current gates PASS is baseline  

### Catalog IDs

OWN-001, CON-001, PAY-001/002.

### Verification

| Kind | Proof |
| --- | --- |
| CI | generation, dangerous drift, deny, tests |

### Acceptance

- [ ] INV table merged and linked from README/API-STABILITY  
- [ ] ACT commands pasteable in M-02  
- [ ] No new parallel capability store  

### Residual risks

Profile remains alias (AD-09 Accepted).

---

### Phase 2 — Freeze high-risk denominator + coverage generator

### Objectives

Single machine-readable high-risk set; auto report `fully_protected / total`.

### Work

- Expand `HIGH_RISK_WRITES` (or generate from mutation class + explicit list) to include mission gaps: at least `cancel_payment`, `cancel_refund`, `create_customer_payment`, `create_connect_balance_transfer`, align `create_mandate`/`create_payment_link`  
- `scripts/report_high_risk_coverage.py` → md+json  
- Wire CI fail if coverage < 100% **after** Phase 3–4 close (gate starts advisory then blocking)  

### Catalog IDs

PAY-008, CON-003, CI-001.

### Dual-suite

| Suite | On current | On desired |
| --- | --- | --- |
| Baseline CI 16-set | PASS | PASS |
| Expanded denominator report | missing/partial | 100% Full |

### Acceptance

- [x] Denominator frozen in script + doc  
- [x] Report generated in CI  
- [x] HR-001 closed or explicitly Accepted with waiver  

---

### Phase 3 — Connect balance transfers Tier-S

### Objectives

`client.connect_balance_transfers()` (or domain-consistent name) with validation, sticky idempotency, isolation tests.

### Work

- `src/domain/connect_balance_transfers.rs` (name to confirm against domain grammar)  
- Validated create request; get/list if supported  
- Concurrency: credential/profile/idempotency isolation  
- Example already exists: ensure Tier-S path preferred  
- Mark ValidatedFacade + HIGH_RISK  

### Catalog IDs

CON-003, AUT-006/009, PAY-001/002.

### Verification

| Kind | Proof |
| --- | --- |
| Unit | validation reject paths |
| Integration | WireMock create + Unknown timeout |
| Concurrency | multi-credential stress |

### Acceptance

- [x] Tier-S API public  
- [x] Profile + tests  
- [x] HR-002 closed  
- [x] No secret logging of signing/credentials  

---

### Phase 4 — Transport polish (Retry-After HTTP-date + failure injection)

### Objectives

Parse HTTP-date `Retry-After`; never sleep past deadline; expand hostile failure injection for high-risk writes.

### Work

- `parse_retry_after` HTTP-date branch + clock skew tests  
- Hostile cases: reset, slow body, 429/5xx, malformed JSON, foreign redirect (extend existing)  
- Document precedence: Retry-After vs backoff vs deadline  

### Catalog IDs

PAY-004, PAY-005.

### Acceptance

- [x] HTTP-date honored within budget  
- [x] KER-RA-01 closed  
- [ ] create_payout timeout-after-send ⇒ Unknown + no auto duplicate  

---

### Phase 5 — Pagination streams + enum forward-compat

### Objectives

Consistent `stream_pages`/`stream_items` where meaningful; response enum unknown variant strategy where safe.

### Work

- Shared helpers in `domain/common` preserving max pages/items, cycle, origin, credentials, deadline  
- Tests: cyclic next, foreign next, http downgrade, huge pages  
- Enum audit for critical response types; request enums stay closed  

### Catalog IDs

CON-006, PAG-001.

### Acceptance

- [ ] At least payments/refunds/payouts/UCT expose streams or documented N/A  
- [ ] Origin/cycle tests still green  

---

### Phase 6 — Docs, observability, error DX

### Objectives

Mission production guides; observability allowlist tests; error helper consistency.

### Work

- Add guides under `docs/guides/`: payments, refunds, payouts, transfers, oauth-connect, webhooks, retries-and-idempotency, delivery-outcomes, multi-merchant, pagination, error-handling, testing (merge with existing 3)  
- Allowlist safe hook/tracing fields; tests for forbidden  
- rustdoc Tier-S vs Tier-G clarity  

### Catalog IDs

CON-004, AUT-003, WHK-002/006.

### Acceptance

- [ ] Guide matrix ≥ mission list or explicit deferrals  
- [x] DOC-GUIDE-01 closed  
- [ ] Secret leak still PASS  

---

### Phase 7 — RC → 1.0 release assurance

### Objectives

Close `docs/rc/rc-checklist.md`; SBOM/provenance; scorecard green; tag discipline.

### Work

- Hostile security review doc signed  
- Package audit + clean-room compile  
- cargo audit triage process  
- Live readonly green evidence (credentials)  
- Sandbox write matrix documented limitations  
- SBOM (e.g. cargo-cyclonedx / ecosystem standard)  
- Generated 1.0 scorecard from coverage script  
- Release pipeline includes mission gate list  

### Catalog IDs

CI-001, REL-*.

### Acceptance

- [ ] RC checklist P0=0  
- [ ] Scorecard critical fields green  
- [ ] REL-001/002 closed or Accepted with owner  

---

### Phase 8 — Optional hardening (post-RC or parallel)

### Objectives

Mutation testing on handwritten kernel; deeper fuzz; proptest if justified without dep bloat.

### Work

- Targeted mutation on retry/idempotency/origin/webhook/redaction  
- Fuzz smoke in CI bounded  
- Differential contract expansion  

### Acceptance

- [ ] TEST-MUT-01 addressed or deferred Accepted  

---

## Mission phase crosswalk (0–34 → program)

| Mission P | Program |
| --- | --- |
| 0 baseline | Phase 0 |
| 1 profile | Closed v1 + Phase 1 ACT |
| 2 high-risk freeze | Phase 2 |
| 3–5 payouts/transfers/oauth | **Closed** @ HEAD |
| 6 connect | Phase 3 |
| 7–8 delivery/retry model | **Closed** + Phase 4 expand |
| 9 Retry-After date | Phase 4 |
| 10–11 types/enums | Phase 5 (+ ongoing) |
| 12 pagination streams | Phase 5 |
| 13–14 webhook/secrets | Partial → Phase 6/8 |
| 15–16 drift/gen | Partial/Closed → Phase 2/7 |
| 17–20 API/errors/obs | Phase 6 |
| 21–25 testing pyramid | Phase 4/7/8 |
| 26–27 docs/examples | Phase 6 (examples strong) |
| 28–34 stability/release | Phase 7 |

---

## Definition of done (program)

```text
124/124 contract + profiles
Frozen high-risk denominator at 100% fully protected
Connect + other justified high-risk Tier-S present
Financial retries require correct idempotency
Delivery ambiguity explicit
Credential isolation concurrency-tested
Redirect + pagination origin blocked
Secret-leak automated
Webhook verify adversarially covered
Dangerous drift blocking; gen reproducible
Public API grammar consistent (streams where due)
Production guides present
Semver + deny + audit triage
SBOM/provenance for 1.0 tag
RC checklist green / Accepted residuals only
CI green on release commit
```

**Scoring rule:** report gate/ACT status and open AD-*; **do not** self-assign 9.8–10.0 aesthetics.

---

## Handoff brief

### Done this session

- Phase 0 freeze docs at `e3358d2`  
- High-risk + public API inventories  
- Elite residual program with R13 invariants/ACT/AD  
- Mission 0–34 mapped to Closed vs Phases 1–8  

### Next commands

```powershell
cd C:\Users\floris\.copilot\copilot-worktrees\mollie-rs\floris-xlx-cuddly-umbrella
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
python scripts/check_generation_reproducibility.py
python scripts/check_dangerous_profile_drift.py
# implement: Phase 2 denominator script (RED report first)
```

### Must not

- Reimplement closed facades/kernel  
- Dual capability registries  
- Claim 1.0 READY on 16/16 alone while Connect/guides/RC open  

### Open AD / ACT red

AD-01…08 Open; ACT-HR incomplete until generator; full test matrix Unverified this SHA.
