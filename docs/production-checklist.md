# Production checklist (mollie-rs)

**Crate version:** 0.7.0
Use this checklist when shipping a Mollie integration that depends on `mollie-rs`.

## Three readiness levels

| Level | Meaning |
| --- | --- |
| **SDK production readiness** | The crate is safe to depend on: redacted secrets, payment-safe retries, verified contracts for **pinned** operations, CI gates. |
| **Payment-integration production readiness** | Your app correctly creates payments, verifies webhooks, refetches authoritative state, handles duplicates, and uses idempotency keys. |
| **Full payment-infrastructure readiness** | Ledgers, reconciliation jobs, payout/settlement ops, Connect multi-merchant, monitoring, incident runbooks. **Not** provided by this crate alone. |

Passing unit tests does **not** imply levels 2 or 3.

## SDK dependency

- [ ] Pin a minor (`mollie-rs = "0.7"`) and read `CHANGELOG.md` on upgrades.
- [ ] Disable `app-helpers` in libraries that must not load `.env` or install a global subscriber.
- [ ] Enable `zeroize` when process memory retention of API keys/tokens is a threat model concern.
- [ ] Use `MollieClient` / domain facades for writes; treat `types::*` as Tier G.

## Credentials

- [ ] Never log `Authorization`, API keys, OAuth tokens, or webhook secrets.
- [ ] Prefer env-based loading in production (`MOLLIE_API_KEY` / `MOLLIE_OAUTH_ACCESS_TOKEN`).
- [ ] For Connect multi-merchant, use short-lived scoped clients (`with_credential`) per organization token.
- [ ] Set sticky `testmode` only with organization OAuth tokens that require it; never send testmode to live-only business routes.

## Payments and idempotency

- [ ] Generate a **caller-owned** idempotency key per logical write; store it with your order id.
- [ ] Do not enable write retries without sticky keys (`RetryPolicy::default_safe` only retries writes when a sticky key is bound).
- [ ] Treat auto-generated UUID keys as single-attempt markers, not cross-retry identifiers.
- [ ] Validate amounts with `Money` before HTTP.

## Webhooks

- [ ] Verify Next-gen signatures on the **raw** body before JSON parse.
- [ ] Distinguish missing vs malformed signatures (fail closed).
- [ ] Support current + previous secrets during rotation.
- [ ] Acknowledge quickly; process asynchronously.
- [ ] Deduplicate by event id / delivery id (SDK does not store events).
- [ ] Always **refetch** the payment (or resource) from the API before mutating your ledger.
- [ ] Assume HMAC does **not** prevent replay.

## Retries and rate limits

- [ ] Default retries remain off unless you opt in.
- [ ] Honor `Retry-After` / rate-limit metadata on errors.
- [ ] Bound pagination with `PaginationGuard` (never unbounded `list_all` in request paths).

## Observability

- [ ] Prefer structured fields: operation, status, request_id, attempt — never full payment payloads.
- [ ] Use request hooks for metrics; keep secrets redacted.
- [ ] Alert on elevated 401/403, 429, and 5xx rates.

## Contract drift

- [ ] Review `docs/api-drift-report.md` / CI upstream drift artifacts before regenerating OpenAPI.
- [ ] Consult `docs/registries/operation-registry.yaml` for local vs missing upstream ops.
- [ ] Block releases on removed operations, auth changes, or webhook signature field changes.

## Known gaps (honest)

As of 0.7.0:

- Local OpenAPI pin and the official contract both contain **124** operations.
- Domain facades cover payments, refunds, captures, subscriptions, mandates, payment links, webhooks, payouts, transfers, OAuth, sessions, terminals, verify-payee, and unmatched credit transfers. Business-account and remaining Connect BA convenience surfaces remain generated (Tier G) without dedicated facades.
- Live Mollie e2e is opt-in only (not default CI).

Track progress in [`docs/release-readiness.md`](release-readiness.md) and `docs/audits/official-sdk-parity-assessment.md`.
