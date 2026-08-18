# RC current baseline freeze

**Status:** Phase 0 freeze (hardening program entry)  
**Crate version:** `0.8.0` (`Cargo.toml`; freeze entry was `0.7.1`)  
**HEAD:** `c4e909131a3797c69309c017661628b3a92700d5`  
**Branch:** `floris-xlx-hardening`  
**MSRV:** `1.88`  
**Freeze date:** 2026-08-18  
**Honest band:** **NEAR READY** (kernel + high-risk + drift + Phase 1 HTTP body limits landed; residual Tier-S semver fail-closed + provider-model + live/hostile remain)

Program source: exceptional 1.0 RC hardening plan (release hardening, not redesign).

Architecture preserved:

```text
Tier-S facade
    ↓
Tier-G generated API
    ↓
OperationSafetyProfile
    ↓
Transport / retry / delivery kernel
    ↓
Mollie API
```

---

## Pins and inventory (measured)

| Item | Value | Evidence |
| --- | --- | --- |
| HEAD | `c4e909131a3797c69309c017661628b3a92700d5` | `git rev-parse HEAD` |
| Crate | `mollie-rs` `0.7.1` | `Cargo.toml` |
| MSRV | `1.88` | `Cargo.toml` `rust-version` |
| Upstream OpenAPI pin SHA-256 | `0cbba39eed3c1b5ddd6cb815170a106a0877d45d22403364dfb4d8c18d99e993` | `specs/upstream-pin.toml` |
| Upstream pin date | 2026-08-18 | `specs/upstream-pin.toml` |
| Local / Tier-G ops | **124** | `check_generation_reproducibility.py` |
| High-risk writes | **23/23** fully protected | `check_dangerous_profile_drift.py` + `report_high_risk_coverage.py --require-full` |
| Mutations discovered | **49** (denominator 23) | `detect_high_risk_operations.py` |
| OpenAPI drift fixtures | **17/17** | `run_openapi_drift_fixtures.py` |
| Tier-S request contracts | **6 ops** | `check_tier_s_request_contracts.py` |
| Tier-S public API snapshot | **147 symbols** | `check_tier_s_public_api.py` |
| Contract graph | **124 ops / 15 modules** | `build_contract_graph.py` |
| Tier-S domain facades | **16** public APIs (+ `common`) | `src/domain/mod.rs` |
| Examples | **126** | `examples/*.rs` |
| Features | `default = ["app-helpers"]`, optional `zeroize` | `Cargo.toml` |
| Generator / client stack | progenitor-client `0.11.2`, reqwest `0.12` (json, rustls-tls, stream; no default features) | `Cargo.toml` |

### Tier-S facade inventory

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
| `connect_balance_transfers` | `ConnectBalanceTransfersApi` |

### High-risk write set (SSOT denominator = 23)

See `docs/registries/high-risk-coverage.md` and `src/operation_safety.rs` (`HIGH_RISK_WRITE_OPERATION_IDS`).

---

## CI gates (current)

Source: `.github/workflows/ci.yml`

| Job | Role | Blocking? |
| --- | --- | --- |
| `contract` | generation, high-risk drift/coverage, mutation discovery, OpenAPI fixtures, Tier-S request + public API snapshot, contract graph, registry commit, upstream pin digest | **Yes** |
| `quality` | fmt, clippy all-features + no-default, test all-features + no-default, doc tests, examples compile, rustdoc `-D warnings` | **Yes** |
| `deny` | `cargo deny check --all-features` | **Yes** |
| `msrv` | check + lib tests on 1.88.0 | **Yes** |
| `package` | `cargo package` dry-run + verify | **Yes** |
| `semver` | `cargo-semver-checks` vs crates.io | **Yes** (fail-closed as of 0.8.0) |

---

## Gate run log (this freeze)

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | **PASS** |
| `cargo check --all-targets` | **PASS** |
| `python scripts/check_generation_reproducibility.py` | **PASS** 124=124 |
| `python scripts/check_dangerous_profile_drift.py` | **PASS** 23 high-risk |
| `python scripts/report_high_risk_coverage.py --require-full` | **PASS** 23/23 |
| `python scripts/detect_high_risk_operations.py` | **PASS** |
| `python scripts/run_openapi_drift_fixtures.py` | **PASS** 17/17 |
| `python scripts/check_tier_s_request_contracts.py` | **PASS** |
| `python scripts/check_tier_s_public_api.py` | **PASS** 147 symbols |
| `python scripts/build_contract_graph.py` | **PASS** (wrote registry) |
| `cargo clippy --all-targets --all-features -- -D warnings` | **PASS** (post Phase 1 body-limits) |
| `cargo test --all-features` | **PASS** (lib 245 + integration + 146 doctests) |
| `cargo test --doc` | **PASS** (included in `--all-features` doctest run) |
| `cargo build --examples` / `cargo check --examples` | **PASS** (`cargo check --examples --all-features`) |
| `cargo deny check` | **PASS** (advisories/bans/licenses/sources ok) |
| `cargo audit` | **SKIP** (tool not installed locally; covered by `cargo deny` advisories) |
| `cargo tree` TLS stack | **PASS** (`reqwest` features `rustls-tls`/`json`/`stream`; no `openssl`/`native-tls` package) |
| `cargo-semver-checks` | **FAIL-CLOSED** in CI (local tool optional; job structurally blocking) |

Phase 1 landed response body limits + removed builder `http_client` inject path. See `docs/rc/transport-security-policy.md`.

---

## Feature matrix

| Feature | Default | Purpose |
| --- | --- | --- |
| `app-helpers` | on | `dotenvy` + `tracing-subscriber` for examples/apps |
| `zeroize` | off | optional secret zeroization |

---

## Known residual findings (summary)

Full list: [`open-residuals.md`](open-residuals.md). Highest priority:

| ID | Sev | Title | Status |
| --- | --- | --- | --- |
| HTTP-001 / SUI-2329 | P0 | Deprecated `http_client` still bypasses SDK last-apply | **Open** (partial: `configure_http` exists) |
| TIER-002 / SUI-2361 | P0 | `cargo-semver-checks` advisory; Tier-S snapshot already blocking | **Partial** |
| REL-001 / SUI-2330 | P1 | Live readonly + sandbox write evidence not credentialed | **Open** |
| HOST-001 / SUI-2332 | P1 | Hostile live soak not run | **Partial** (static review PASS) |
| Provider-model coverage | P1 | OpenEnum/NullableField foundation landed; production field migrations incomplete | **Open** (SUI-2337/2339/2348/2350/2351/2353) |
| PAG-001 / SUI-2328 | P1 | Pagination consistency matrix incomplete | **Open** |
| TEL-001 / SUI-2366 | P2 | Runtime drift telemetry | **Open** |
| EX-001 / SUI-2331 | P1 | Example workflow coverage script | **Partial** (examples compile in CI) |

---

## Acceptance for Phase 0

- [x] baseline inventory committed under `docs/rc/`
- [x] no known failure silently ignored (pending cargo quality gates listed explicitly)
- [x] residuals mapped to plan issue IDs / FINDINGS
- [x] no Phase 1+ redesign of architecture layers

Companions:

- [`open-residuals.md`](open-residuals.md)
- [`1.0-acceptance-matrix.md`](1.0-acceptance-matrix.md)
- [`baseline.md`](baseline.md) (historical pointer)
- [`../sdd/1.0-readiness/FINDINGS.md`](../sdd/1.0-readiness/FINDINGS.md)
