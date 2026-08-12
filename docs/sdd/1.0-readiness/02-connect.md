# SDD 02 — Connect / multi-merchant safety

## Context
Connect platforms hold many organization tokens. Concurrent requests must never cross-wire credentials.

## Problem
Mutable shared client credential state can send merchant A traffic under merchant B.

## Existing behavior
- `MollieClient::with_credential` rebuilds HTTP client with new Authorization; preserves timeouts, UA, testmode, profile, sticky idempotency, retry, hooks (`src/client.rs`).
- Client is `Clone` (reqwest pool shared).
- Sticky `testmode` / `profile_id` on `Client`.
- Route capabilities: `supports_testmode`, `requires_profile_scope`.

## Desired behavior
- Immutable scoped clones only (no interior mutability of credential).
- Precedence: per-operation override > scoped client context > builder default > omit.
- Never attach testmode/profileId when profile says unsupported.
- Concurrent stress test: N tasks with distinct credentials; each request sees only its Authorization.

## Non-goals
Global process-wide Mollie credential; automatic token refresh daemon inside SDK.

## Invariants
INV-CONN-01, INV-PROFILE-01 (attachment rules), INV-SEC-01.

## API design
Keep `with_credential`. Optional thin `ClientContext` only if it reduces dual paths—default is scoped `MollieClient` clones.

## Failure modes
Blank credential → InvalidConfiguration. Unsupported testmode on live-only route → omit or error per capability (document).

## Security
Secrets never in Debug; hooks get redacted URLs only.

## Testing
Unit rebuild test (exists) + concurrent mock server asserting Authorization header per task.

## Acceptance
- [ ] Concurrency regression green
- [ ] Precedence documented + tested
- [ ] Production checklist Connect section accurate
