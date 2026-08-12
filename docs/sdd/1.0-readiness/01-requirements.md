# SDD 01 — 1.0 acceptance criteria

**Depends on:** `00-baseline.md`  
**Crate target band:** do **not** ship `1.0.0` until this document’s gates pass; prefer `0.8` / `0.9` / `1.0.0-rc.N`.

## Hierarchy of priorities

1. Financial correctness  
2. Safety  
3. Explicitness  
4. Contract correctness  
5. API completeness  
6. Predictable failure behavior  
7. Operational visibility  
8. Rust ergonomics  
9. Performance  
10. Convenience  

## Measurable gates

### Contract

- [ ] Local pin operation count equals upstream pin (today **124**)
- [ ] Generation reproducible (`check_generation_reproducibility.py`)
- [ ] `OperationSafetyProfile` (or evolved `RouteCapability`) covers 100% of ops
- [ ] Dangerous drift CI **blocking**: removed op, auth change, method/path change, financial-write/retry class change, webhook signature shape change
- [ ] Additive upstream ops visible (soft or hard gate — documented)

### Transport safety kernel

- [ ] INV-WRITE-01/02 proven (no multi-attempt financial write without sticky key; attempts≤1 without key)
- [ ] INV-DEADLINE-01 proven (no attempt begins after budget)
- [ ] INV-DELIV-01: NotSent / Rejected / Succeeded / Unknown classified; Unknown + financial + no sticky → no auto-retry
- [ ] INV-HOST-01: cross-origin redirect does not forward Authorization (test)
- [ ] INV-PAGE-01: provider `next` origin constrained
- [ ] INV-CANCEL-01 documented for post-transmit drop
- [ ] INV-PROFILE-01: transport reads only profile SSOT
- [ ] Retry state-machine / property tests for connect/timeout/429/503/success/deadline sequences

### Connect

- [ ] INV-CONN-01 concurrent scoped clients cannot cross-wire credentials
- [ ] Precedence: per-op > scoped client > default > omit
- [ ] testmode/profile only on capable routes

### High-risk Tier-S (justified only)

For each: safety added over Tier G documented.

- [ ] OAuth token lifecycle facade
- [ ] Payouts list/get/create/cancel
- [ ] Transfers (+ BA reads as needed)
- [ ] Verify payee / unmatched CT as justified
- [ ] Primary metric: high-risk ops with **enforced** invariants / total high-risk ≥ agreed threshold (track in release-readiness; target ≥ 0.95 for money-moving writes)

### Webhooks

- [ ] Raw-body verify; constant-time; rotation; fail closed
- [ ] Guides emphasize authoritative refetch
- [ ] Adversarial + fuzz coverage on parsers

### API stability & release

- [ ] `docs/API-STABILITY.md` (Tier S/G, generated types, features, MSRV, semver)
- [ ] `docs/release-readiness.md` filled
- [ ] Default `cargo test` cannot create live financial activity
- [ ] MSRV CI green; deny/audit policy green
- [ ] Hostile review checklist (§ master prompt 39) answered

### Documentation

- [ ] Task guides for payments, retries (incl. Unknown), idempotency, connect, webhooks, payouts/transfers, production levels
- [ ] Examples compile in CI; no real credentials

## Non-goals for 1.0

- Full application ledger / reconciliation product
- Generated-first recommended write API
- Speakeasy-style default write retries
- Optimizing Tier-S method count as vanity metric

## 1.0 readiness bands

| Band | When |
| --- | --- |
| NOT READY | Kernel P0 open or high-risk writes lack enforced invariants |
| NEAR READY | Kernel frozen; money facades in progress; docs incomplete |
| RC READY | Gates green; residual P2 only; rc cut |
| 1.0 READY | All gates above + freeze period without P0 regressions |

## Facade justification template

For every proposed Tier-S API:

1. Operation ids covered  
2. Invariants enforced beyond Tier G  
3. Failure modes  
4. Idempotency / retry class (from profile)  
5. Tests (unit + mock)  
6. If answer to (2) is “none” → **do not** add facade  
