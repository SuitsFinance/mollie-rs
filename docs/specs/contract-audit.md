# Spec-driven contract audit (mollie-rs)

**Baseline HEAD at audit start:** `6a10172` (+ local hardening).  
**Verified:** 2026-08-03 against implementation, not docs alone.

## A. Provider contract (Mollie)

| Invariant | Status | Evidence | Risk if wrong |
| --- | --- | --- | --- |
| Auth: Bearer API key or OAuth access token | complete | `auth.rs`, wiremock auth tests | Unauthorized calls |
| Idempotency-Key on writes | partial | always sent by `Client::request`; Mollie documents write use | Double charge if misused |
| Rate limit 429 + Retry-After | partial | classified + optional retry policy | Thundering herd |
| Classic webhook = resource id only | complete | `webhook.rs` | Trusting callback as state |
| Next-gen HMAC-SHA256 raw body hex (`X-Mollie-Signature`) | complete | `webhook_verify.rs` + Mollie docs | Forged events |
| Pagination via opaque `from` + limit ≤250 | partial | primitives + payments facade; not all list routes | Infinite loops |
| HAL error envelope | complete | fixtures + catalog | Bad ops triage |

## B. Generated SDK contract

| Invariant | Status | Location |
| --- | --- | --- |
| 100 operations from `specs-3.0.yaml` | complete | `route_capabilities`, generation check |
| Spec-coupled types in `types.rs` | complete | generation pipeline |
| Regeneration changes Tier G freely | documented | `docs/compatibility.md` |
| Upstream live parity | missing | only local pin + drift script inventory |

## C. Handwritten facade contract

| Invariant | Status | Test |
| --- | --- | --- |
| Money/currency validation | complete | money unit tests |
| Resource ID prefixes | complete | ids unit tests |
| Error taxonomy + catalog keys | complete | factory + postman fixtures |
| Retries default **off** | complete | http_contract disabled 503 |
| Safe reads retry with `default_safe` | complete | 503→200 list_payments |
| Writes retry only with **sticky** key | complete | write sticky vs non-sticky tests |
| Webhook HMAC constant-time, no reserialize | complete | webhook_verify tests |
| Missing vs malformed signature | complete | verify_header tests |
| MSRV 1.88 | complete | rust-version + CI |
| cargo-deny blocking | complete (CI) | workflow without continue-on-error |

## Known ambiguities (do not encode unsafely)

1. Mollie does **not** require a signed timestamp on Next-gen HMAC; `with_max_skew` is application-only.
2. Auto UUID keys enable provider-level idempotency of a single HTTP attempt series only when preserved on the same request object; they do **not** enable policy-level multi-attempt write retries without sticky binding.
3. Live OpenAPI drift vs Mollie-hosted specs is not automatically enforced without an upstream snapshot URL policy.
