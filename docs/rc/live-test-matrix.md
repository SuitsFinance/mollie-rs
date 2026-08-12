# RC live test matrix

**Baseline HEAD:** see [`baseline.md`](baseline.md)  
**Harness:** `tests/live_smoke.rs`

## Tiers

| Tier | Env gates | Mutates Mollie? | CI default |
| --- | --- | --- | --- |
| 1 Readonly | `MOLLIE_LIVE_READONLY=1` **or** legacy `MOLLIE_LIVE_SMOKE=1` + credentials | No | Off (manual / scheduled) |
| 2 Testmode write | `MOLLIE_TESTMODE_WRITE=1` **and** `MOLLIE_ALLOW_MUTATION=I_UNDERSTAND_THIS_MUTATES_MOLLIE` **and** `test_` API key (or `MOLLIE_TESTMODE_WRITE_ALLOW_OAUTH=1`) | Yes (test entities) | Off — never PR-automatic |
| 3 Destructive | Tier 2 **and** `MOLLIE_DESTRUCTIVE_SMOKE=1` | Cancel/delete/revoke | Manual only |

`live_` API keys are **refused** for write suites.

## Readonly operations exercised

| Test | Operation / path | Auth/serialization intent |
| --- | --- | --- |
| `live_methods_readonly` | `list_methods` | Methods + Tier-G decode |
| `live_payments_readonly` | `payments().list_page` | Tier-S pagination |
| `live_profiles_readonly` | `list_profiles` | Profiles route group |
| `live_current_profile_readonly` | `get_current_profile` | API-key profile scope |
| `live_balances_readonly` | `list_balances` | Balances / Connect-ish |
| `live_settlements_readonly` | `list_settlements` | Settlements |
| `live_organizations_readonly` | `get_current_organization` | Org “me” |
| `live_permissions_readonly` | `list_permissions` | OAuth permissions |
| `live_refunds_readonly` | `list_all_refunds` | Global refunds list |
| `live_payouts_readonly` | `payouts().list_page` | Tier-S payouts read |
| `live_business_accounts_readonly` | `list_business_accounts` | BA entitlement |
| `live_terminals_readonly` | `terminals().list_page` | Terminals |
| `live_webhooks_readonly` | `list_webhooks` | Webhooks API |

## Outcome classes

| Class | Meaning | Test result |
| --- | --- | --- |
| Succeeded | 2xx path | Pass |
| PermissionDenied | 403 | Pass (account limit) |
| UnsupportedByAccount | 404/410 | Pass (account limit) |
| ProviderRejected | 422 | Pass (env/shape limit) |
| AuthenticationFailed | 401 | **Fail** |
| SdkOrTransportFailure | 5xx / transport / other | **Fail** |

Empty resource sets are success.

## Write smoke (implemented)

| Test | Flow | Notes |
| --- | --- | --- |
| `sandbox_payment_create` | create → get (optional cancel if Tier 3) | Uses `Money`, `CreatePaymentRequired`, sticky `IdempotencyKey` |
| `sandbox_payment_idempotency` | create ×2 same key | Asserts same provider payment id |

## Not yet automated live

| Area | Status |
| --- | --- |
| Refund create (needs paid payment state) | Pending / provider-blocked risk |
| Payout create/cancel | Pending |
| Transfer create | Pending |
| OAuth generate/revoke disposable | Pending |
| Connect dual-merchant live soak | Pending (mock exists) |

## Commands

```bash
# Readonly matrix
MOLLIE_LIVE_READONLY=1 MOLLIE_API_KEY=test_... \
  cargo test --test live_smoke -- --ignored --nocapture

# Payment write smoke
MOLLIE_TESTMODE_WRITE=1 \
MOLLIE_ALLOW_MUTATION=I_UNDERSTAND_THIS_MUTATES_MOLLIE \
MOLLIE_API_KEY=test_... \
  cargo test --test live_smoke sandbox_payment -- --ignored --nocapture
```

## Gate unit tests (always on)

- `write_gate_rejects_live_api_keys`
- `classify_auth_and_permission_errors`
