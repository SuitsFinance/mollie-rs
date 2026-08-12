# Implementation baseline freeze — mollie-rs 0.6.1

**Frozen at:** `10ec88e` (docs deep audit) + follow-on Phase 0/1 hardening on top  
**Package version:** `0.6.1`  
**MSRV:** 1.88  
**Baseline date:** 2026-08-04  

## Verified claims

| Claim | Verified |
| --- | --- |
| Generated OpenAPI client | Yes — `src/routes/*`, types |
| Local operations | **100** (`specs-3.0.yaml`, `route_capabilities`) |
| Upstream official operations | **124** (`mollie/openapi` pin sha256 `475da734…`) |
| Missing vs upstream | **24** (registry + live compare) |
| Domain facades | payments, refunds, captures, mandates, subscriptions, payment_links, webhooks |
| Typed credentials / money / IDs | Yes |
| Idempotency + opt-in retries | Yes; sticky-key writes only |
| Route-aware retry class | Yes (`Client::send` → `route_capability`) |
| Sticky profile / with_credential / hooks | Yes |
| Retry budget without leftover send | Yes |
| Webhook HMAC + classic parse | Yes |
| Pagination guards | Yes |
| WireMock / unit tests | **161** lib tests pass (`cargo test --lib --all-features`) |
| CI: fmt, clippy, tests, doctests, MSRV, deny, generation | Present |
| Operation registry | Present + export script |

## Command results (Phase 0)

| Command | Result |
| --- | --- |
| `python scripts/check_generation_reproducibility.py` | OK — 100/100 |
| `python scripts/export_operation_registry.py` | 100 local + 24 gaps |
| `python scripts/fetch_upstream_openapi.py` | OK — digest match, 124 ops |
| `python scripts/compare_upstream_openapi.py --require-upstream` | Exit 2 — 24 missing (expected) |
| `cargo test --lib --all-features` | **161 passed** |

## Incomplete areas (not Phase 0 bugs)

1. Full OpenAPI re-pin / Tier G regeneration for 24 ops (**0.7.0**).
2. OAuth/payouts/BA/transfers/sessions/UCT/verify-payee/pairing facades.
3. Universal `stream_*` on all list facades.
4. Property/fuzz suites.
5. Public API freeze for 1.0.

## Blockers for 0.7.0

| Blocker | Status |
| --- | --- |
| Authoritative upstream pin + digest CI | Addressed in Phase 1 |
| Local path model vs upstream `/v2` prefix | Must adapt during re-pin |
| `/oauth2/tokens` outside `/v2` base URL | Needs dual-base or absolute paths |
| Generator toolchain vs MSRV | Document separately |

## Assumptions

* Upstream pin URL `mollie/openapi` main remains the authoritative GA contract.
* Local `specs-3.0.yaml` remains the generation input until explicit re-pin PR.
* Exit code 2 (missing ops) stays advisory in PR CI until re-pin lands.

## Public API relative to 0.6.0/0.6.1 publish

0.6.1 already published. Additive surfaces since mid-0.6.x (still 0.6.1 line unless bumped):

* `RequestHook` / integration traits
* `profile_id`, `with_credential`, user-agent suffix
* transport timeout retention for credential rebuilds (Phase 1)

## Phase 0 decision

**Baseline is frozen and usable.** Proceed with contract pipeline + hardening; do not treat the 24-op gap as a broken baseline.
