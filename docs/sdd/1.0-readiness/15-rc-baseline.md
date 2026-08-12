# SDD 15 — RC baseline freeze (`mollie-rs` Final RC Hardening)

**Status:** Phase A freeze (evidence recorded)  
**Crate version:** `0.7.0` (`Cargo.toml`)  
**HEAD (this freeze):** `df6a9d434b56f5663aac1003f94db0e0e028b587`  
**Branch:** `floris-xlx-scaling-journey`  
**MSRV:** `1.88` (`rust-version` + CI job)  
**Freeze date:** 2026-08-10  
**Honest band entering this program:** **NEAR READY**  
**Supersedes for RC entry claims:** narrative in `docs/sdd/1.0-readiness/00-baseline.md` §facade inventory (stale HEAD `55187ee` / 7 Tier-S modules)

Companion checklist: [`docs/rc/rc-checklist.md`](../../rc/rc-checklist.md)  
Companion copy: [`docs/rc/baseline.md`](../../rc/baseline.md)

---

## 1. Mission lock

This program does **not** redesign:

- `OperationSafetyProfile` / `RouteCapability` SSOT
- `DeliveryOutcome` / sticky-key retry gates / deadline protections
- 124-operation Tier-G parity
- Tier-S payouts / transfers / OAuth / money facades already present
- dangerous drift detection / cargo-semver CI / fuzz **build** CI

It **does** prove those surfaces with live/sandbox matrix, hostile transport, package soak, API freeze review, and release evidence.

---

## 2. Pins and inventory (measured)

| Item | Value | Evidence |
| --- | --- | --- |
| HEAD | `df6a9d434b56f5663aac1003f94db0e0e028b587` | `git rev-parse HEAD` |
| Crate | `mollie-rs` `0.7.0` | `Cargo.toml` |
| MSRV | `1.88` | `Cargo.toml` `rust-version`, CI `msrv` |
| Upstream OpenAPI pin SHA-256 | `8b6839e22c14dc341ec76d3f71b3292fd03502db204387918fbd607f90591010` | `specs/upstream-pin.toml` |
| Upstream pin date | 2026-08-09 | `specs/upstream-pin.toml` |
| Local pin ops | **124** | `specs-3.0.yaml` `operationId` count |
| Tier-G `pub async fn` routes | **124** | `rg "pub async fn" src/routes` |
| Registry ops | **124** | `docs/registries/operation-registry.yaml` meta |
| Capabilities rows | **124** | `check_generation_reproducibility.py` |
| High-risk writes (denominator) | **16/16** profile-checked | `check_dangerous_profile_drift.py` |
| ValidatedFacade ops | **18** | registry `access=ValidatedFacade` |
| Facade-flagged ops | **18** | registry `facade: true` |
| Retry classes | SafeRead **75** · IdempotentWrite **35** · NonRetryableWrite **14** | registry |
| Tier-S domain modules | **15** public modules (14 facades + `common`) | `src/domain/mod.rs` |
| Examples (`examples/*.rs`) | **126** | directory count |
| Fuzz targets | `webhook_signature`, `webhook_form`, `money_amount`, `payment_id`, `page_cursor`, `retry_after_header` | `fuzz/fuzz_targets` |
| Live smoke (pre-RC) | **2** ignored tests: `list_methods`, `payments().list_page` | `tests/live_smoke.rs` |
| Live write suite | **absent** | only `MOLLIE_LIVE_SMOKE` read path |
| Open GitHub issues (API) | none listed via `gh issue list` at freeze | may be incomplete vs Linear |

### 2.1 High-risk write set (SSOT denominator)

From `scripts/check_dangerous_profile_drift.py` `HIGH_RISK_WRITES`:

```text
create_payment, create_refund, create_capture, create_subscription,
create_payout, cancel_payout, create_transfer, verify_payee,
oauth_generate_tokens, oauth_revoke_tokens, payment_create_route,
create_session, terminals_request_pairing_code, terminals_revoke_pairing_code,
match_unmatched_credit_transfer, return_unmatched_credit_transfer
```

### 2.2 Tier-S facade inventory (public)

| Module | Public API type |
| --- | --- |
| `payments` | `PaymentsApi` |
| `refunds` | `RefundsApi` |
| `captures` | `CapturesApi` |
| `mandates` | `MandatesApi` |
| `payment_links` | `PaymentLinksApi` |
| `subscriptions` | `SubscriptionsApi` |
| `webhooks` | `WebhooksApi` |
| `payouts` | `PayoutsApi` |
| `transfers` | `TransfersApi` (+ `TransferClientSignature`) |
| `oauth` | `OAuthApi` |
| `sessions` | `SessionsApi` |
| `terminals` | `TerminalsApi` |
| `verify_payee` | `VerifyPayeeApi` |
| `unmatched_credit_transfers` | `UnmatchedCreditTransfersApi` |

Kernel exports (not domain): `OperationSafetyProfile`, `DeliveryOutcome`, `RetryPolicy`, credentials, `IdempotencyKey`, etc. via crate root.

---

## 3. CI jobs (current)

Source: `.github/workflows/ci.yml`

| Job | Role |
| --- | --- |
| `check` | fmt, clippy all-features, clippy no-default lib+tests, test all-targets, test no-default, doc tests, `cargo package --allow-dirty --no-verify` |
| `msrv` | `cargo check --all-targets --all-features` on 1.88.0 |
| `generation` | capabilities sync, dangerous profile drift, registry export+diff, local drift report |
| `upstream-openapi` | pin digest fetch (blocking mismatch), compare inventory (advisory missing ops) |
| `deny` | `cargo deny check` (action) |
| `public-api` | `cargo-semver-checks` vs crates.io (stable toolchain) |
| `fuzz-build` | nightly build of 6 fuzz targets (no long run) |

**Not present yet (RC gaps):** dedicated examples compile job, hostile-transport job name, scheduled live readonly, workflow_dispatch sandbox write, package clean-room matrix.

---

## 4. Test pyramid counts (this freeze)

| Layer | Count / result | Command evidence |
| --- | --- | --- |
| Lib unit/property | **224** passed | `cargo test --all-targets` lib suite |
| Integration `http_contract` | **23** passed | same |
| Integration `postman_all_responses` | **3** passed | same |
| Live smoke | **2 ignored** | `tests/live_smoke.rs` |
| Doc tests | **146** passed, 1 ignored | `cargo test --doc` |
| No-default lib+tests | **221** + **23** + **3** + 2 ignored | `cargo test --lib --tests --no-default-features` |
| `#[test]`/`#[tokio::test]` markers (src+tests search) | **253** match lines / 52 files | `rg` stats (includes non-executed attrs) |
| Listed tests (approx) | lib 224 · tests 252 · doc 147 | `cargo test --* -- --list` |

---

## 5. Gate run log (Phase A)

Session log: session-state `files/phase-a-baseline-gates.txt` (+ deny/package notes).

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | **PASS** |
| `cargo clippy --all-targets --all-features -- -D warnings` | **PASS** |
| `cargo clippy --lib --tests --no-default-features -- -D warnings` | **PASS** |
| `cargo test --all-targets` | **PASS** (224 + 23 + 3; 2 live ignored) |
| `cargo test --lib --tests --no-default-features` | **PASS** |
| `cargo test --doc` | **PASS** (146) |
| `cargo check --all-targets --all-features` | **PASS** |
| `python scripts/check_generation_reproducibility.py` | **PASS** 124=124 |
| `python scripts/check_dangerous_profile_drift.py` | **PASS** 16 high-risk |
| `python scripts/export_operation_registry.py` | **PASS** |
| `python scripts/report_api_drift.py` | **PASS** local 124 |
| `cargo deny check` | **PASS** (warnings: unused license allows; duplicate getrandom/hashbrown/windows-sys) |
| `cargo package --allow-dirty --no-verify` | **PASS** 379 files, ~18.4MiB / 2.0MiB compressed |
| `cargo semver-checks` | **NOT RE-RUN** this freeze (CI job exists; local optional) — mark **UNVERIFIED locally** |

### 5.1 Package residual notes

Package includes `.env.default` and `.env.example` (fixture IDs only; no live secrets observed in sample heads). Full package audit is Phase later (`docs/rc/package-audit.md`).

---

## 6. Assumptions verified vs master RC plan

| Plan assumption | Status @ `df6a9d4` |
| --- | --- |
| 124/124 Tier-G | **True** |
| `OperationSafetyProfile` exists | **True** (`type` alias + classes in `operation_safety.rs`) |
| Write-attempt / sticky proofs | **True** (property + contract tests present) |
| Drift gates + deny + semver + fuzz-build CI | **True** |
| Tier-S money/OAuth surfaces | **True** (payouts/transfers/oauth/… modules) |
| Live smoke only two readonly paths | **True** — primary RC weakness |
| Default CI cannot live-write money | **True** structurally today (no write live tests); multi-gate still required before adding sandbox writes |

---

## 7. RC READY definition (acceptance target)

Copied for freeze (must all be true to leave NEAR READY):

```text
P0 safety findings                    = 0
P1 unreviewed safety findings         = 0
124/124 OpenAPI operations            = verified
high-risk safety profile coverage     = 100%
default CI live-money capability      = impossible
live readonly smoke                   = meaningful
sandbox/testmode write smoke          = meaningful
new facade examples                   = compile-gated
security hostile review               = signed off
cargo-deny                            = green
cargo-semver-checks                   = reviewed green
MSRV                                  = verified
release package                       = inspected
pre-release soak                      = complete
release-readiness document            = current
```

---

## 8. Execution order (locked)

```text
1. Baseline freeze                         ← THIS DOCUMENT
2. Examples compile gate
3. Feature/MSRV matrix (already mostly CI)
4. Hostile local transport integration tests
5. Webhook framework integration polish
6. Resource/TLS/proxy review
7. Live readonly expansion
8. Sandbox payment/refund smoke (+ multi-gate)
9. Optional payout/transfer/OAuth sandbox
10. Connect concurrency soak
11. Package audit
12. 0.8.0 release ladder step
13. Clean-room downstream testing
14. Short soak / feedback
15. Public API freeze
16. Hostile security sign-off
17. RC readiness report
18. 1.0.0-rc.1
```

---

## 9. Must-nots

Do not redesign Tier-S, replace transport/reqwest, add ledger/reconciliation, enable write retries by default, put live writes in ordinary `cargo test`, run destructive tests on PRs, waive safety gates with `continue-on-error`, or jump 0.7 → 1.0 without 0.8 soak.

---

## 10. Verdict after Phase A only

```text
NEAR READY
```

Architecture + static/kernel assurance are strong. Live/sandbox matrix, hostile transport expansion, package soak, and signed hostile review remain before **RC READY**.
