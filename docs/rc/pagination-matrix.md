# Pagination matrix (Phase 4 / PAG-001)

Freeze intent: every domain list that walks Mollie HAL `next` links exposes the
same safe surface — `list_page` + guarded multi-page (`list_all` and/or
`stream_pages` / `stream_items`) — and never follows off-origin `next` hrefs.

Kernel (shared):

| Primitive | Location | Invariant |
| --- | --- | --- |
| `PageCursor::from_list_link*` | `src/pagination.rs` | **INV-PAGE-01** origin allowlist (api.mollie.com/nl, matching client base, or loopback when no base) |
| `PaginationGuard` | `src/pagination.rs` | page/item budgets + **cursor cycle** detection (2- and 3-page cycles covered in unit tests) |
| `AsyncPaginator` / `ItemStream` | `src/pagination.rs` | one network call per `next_page` / page fill |
| `validate_page_limit` | `src/domain/common.rs` | fail-closed `limit` 1..=250 |
| `next_cursor_from_links` | `src/domain/common.rs` | extracts `from` only via origin-safe parse |
| `stream_pages` / `stream_items` helpers | `src/domain/common.rs` | facades compose the same guarded streams |

## Domain list surface

| Facade | `list_page` | `list_all` | `stream_pages` | `stream_items` | Notes |
| --- | --- | --- | --- | --- | --- |
| `payments` | yes | yes | yes | yes | top-level payments |
| `refunds` | yes | yes | yes | yes | payment-scoped |
| `captures` | yes | yes | yes | yes | payment-scoped |
| `payouts` | yes | yes | yes | yes | |
| `payment_links` | yes | yes | yes | yes | |
| `mandates` | yes | yes | yes | yes | customer-scoped |
| `subscriptions` | yes | yes | yes | yes | customer-scoped |
| `terminals` (terminals list) | yes | yes | yes | yes | |
| `connect_balance_transfers` | yes | yes | yes | yes | |
| `unmatched_credit_transfers` | yes | yes | yes | yes | |
| `terminals` pairing-codes list | n/a | n/a | n/a | n/a | **Intentional residual:** `list_pairing_codes` returns a generated envelope (not HAL `Page` through the kernel). Pairing lifecycle is create/get/revoke, not long walks. |
| `webhooks` | n/a | n/a | n/a | n/a | **No list API** on facade (verify/parse + `get_event` only) |
| `transfers` | n/a | n/a | n/a | n/a | **No list API** (create/get SEPA transfer) |
| `sessions` | n/a | n/a | n/a | n/a | **No list API** (create/get session) |
| `oauth` / `verify_payee` | n/a | n/a | n/a | n/a | non-list domains |

## Tests (evidence)

| Case | Where |
| --- | --- |
| Off-origin `next` rejected | `pagination::tests::from_list_link_rejects_off_origin_host` |
| Official host + matching base | `from_list_link_parses_from_query`, `from_list_link_allows_matching_client_base` |
| Repeated / 2-page / 3-page cycles | `guard_detects_*` |
| Stream walk + item flatten | `async_paginator_walks_two_pages`, `item_stream_flattens_pages` |
| Domain `next` cursor mapping | per-facade `maps_*_list_next_cursor` unit tests |
| Invalid limit | `domain::common::tests::rejects_zero_and_oversized_limits` |

## Exit for PAG-001

- [x] Kernel origin + cycle + stream primitives tested
- [x] All HAL list facades expose `list_page` + guarded streams/`list_all`
- [x] Intentional non-stream residuals documented (pairing codes, non-list domains)
- [ ] Optional follow-up: thread client `baseurl()` into every `next_cursor_from_links` call so custom non-loopback mock bases do not rely on the no-base loopback exception (production path already allows official Mollie hosts)
