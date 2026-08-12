# OpenAPI re-pin report (Phase 2 → path to 0.7.0)

**Base HEAD:** `576922f`  
**Work:** adapt upstream Mollie OpenAPI → local `specs-3.0.yaml`, regenerate Tier G client, production nullability + test fixes  
**Crate version:** `0.7.0` development cut (not yet published to crates.io)

## 1. Goal

Close the **24-operation gap** between the local generation pin and the
authoritative upstream Mollie OpenAPI (`mollie/openapi`, 124 ops) while keeping
payment-safe transport behavior and live-payload compatibility.

## 2. Pipeline

| Step | Artifact / command | Result |
| --- | --- | --- |
| Upstream pin | `specs/upstream-pin.toml` | sha256 pinned, **124** ops |
| Adapt | `scripts/adapt_upstream_openapi.py` | 3.1 HAL → 3.0.3 `application/json`, `/v2` strip, `/oauth2` absolute, deepObject→form, enum dedupe, nullable arrays |
| Generate | `scripts/generate_openapi_client.py` | `src/types.rs`, `src/routes/*`, `route_capabilities` |
| OAuth path | `Client::endpoint` | `/oauth2/*` resets URL path on host root (not under `/v2`) |
| Registry | `python scripts/export_operation_registry.py` | **124 local + 0 gaps** |
| Compare | `python scripts/compare_upstream_openapi.py` | **0 missing / 0 extra** |

## 3. Operation counts

| Metric | Before (0.6.1 baseline) | After re-pin |
| --- | --- | --- |
| Local ops | 100 | **124** |
| Upstream ops | 124 | **124** |
| Gaps | 24 | **0** |
| Lib tests | 163 | **163 passed** |

## 4. New generated route modules (examples)

* `accounts`, `oauth`, `payouts`, `sessions`, `transfers`
* `unmatched_credit_transfers`, `verify_payee`
* Terminal pairing codes, payment get-route, expanded existing groups

## 5. Handwritten / post-gen production fixes

| Area | Fix |
| --- | --- |
| Refund domain list | `list_all` returns `Vec<ListEntityRefund>` (matches list embed type) |
| Method issuers | `MethodIdWithIssuer` path encoding via `Display` |
| Transfers create | Wire `Idempotency-Key`, `X-Client-Signature`, `X-Client-Signed-At` |
| Chargeback/refund links | `documentation` optional (live list embeds omit it) |
| Refund metadata | `Option<Metadata>` (`null` on live refunds) |
| EntityRefund dual-use | Server fields optional so create body works (`CreateRefundRequest` alias) |
| PaymentMethod parse | Fix `parse` ↔ `FromStr` infinite recursion; error includes raw method id |
| Locale / methods tests | Align with expanded OpenAPI enums (35 methods, `cs_CZ` on LocaleInner) |
| Capabilities fixture | Non-optional `count` / `_embedded` / status shapes |

## 6. Facades

| Domain | Status |
| --- | --- |
| Existing 7 facades (payments, refunds, …) | Compile green against new types |
| OAuth / payouts / BA thin facades | **Not yet** (Phase 3) — Tier G routes available |

## 7. Verification

```text
cargo check --lib          # OK
cargo test --lib           # 163 passed
python scripts/export_operation_registry.py
python scripts/compare_upstream_openapi.py   # exit 0, 0 gaps
```

## 8. Release recommendation

* **Do not ship crates.io 0.7.0 yet** without:
  * examples / route_examples fixture pass
  * optional high-value facades (OAuth token exchange, payouts)
  * CHANGELOG + MSRV/docs sweep
  * CI green on full matrix
* Safe interim: keep the `0.7.0` development cut unpublished until the remaining checklist items are complete.

## 9. Follow-ups (Phase 3+)

1. Thin Tier S facades for OAuth, payouts, transfers (signing helper)
2. Request-level RetryConfig + universal streams
3. Route examples / Postman fixture refresh for new ops
4. Harden generator post-pass for known nullability (so re-gen does not re-break refunds/links)
5. Version bump + release notes when checklist is green
