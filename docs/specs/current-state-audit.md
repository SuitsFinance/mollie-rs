# Current state audit (commit 6167338 + 0.6.0 tranche)

## Classification legend

| Status | Meaning |
| --- | --- |
| **IT** | Implemented and tested |
| **IP** | Implemented, partial tests |
| **NI** | Not implemented |
| **DOC** | Documented, not implemented |
| **STALE** | Docs were wrong (corrected in 0.6) |

## Feature matrix

| Feature | Status | Notes |
| --- | --- | --- |
| Generated 100 ops | IT | generation check |
| Money / IDs / locale validation | IT | unit tests |
| ResponseMetadata | IT | unit + envelope |
| Error catalog / factories | IT | fixtures |
| IdempotencyKey | IT | unit |
| Sticky-key write retries only | IT | wiremock |
| Safe read retries + 429/502/503/504 | IT | wiremock |
| Entropy backoff jitter | IT | unit |
| Webhook HMAC + missing/malformed | IT | unit |
| AsyncPaginator + arbitrary cycle set | IT | unit |
| PaymentsApi (validated create) | IT | unit + contract |
| RefundsApi (validated create) | IT | unit + contract |
| CapturesApi | IP | list mapping tests; no create builder yet |
| SubscriptionsApi (validated create) | IT | unit |
| MandatesApi | IT | list mapping + API surface |
| WebhooksApi (classic + next-gen + get_event) | IT | unit |
| Upstream live OpenAPI parity CI | NI | local pin only |
| Live Mollie e2e in default CI | NI | intentionally blocked |
| 1.0 stability freeze | NI | — |

## Ownership

| Layer | Path |
| --- | --- |
| Tier S facade | `src/domain/*`, `MollieClient`, validators, errors, pagination, webhooks |
| Tier G generated | `src/types.rs`, `src/routes/*`, raw `Client` methods |
| Tier E | experimental policy knobs still evolving |

## Version truth

| Artifact | Value |
| --- | --- |
| Cargo.toml | **0.6.0** |
| README install | **0.6** |
| MSRV | 1.88 |
