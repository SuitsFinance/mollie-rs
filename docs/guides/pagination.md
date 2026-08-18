# Pagination

Mollie list endpoints use opaque `from` cursors and a `limit` (1..=250).

## Safe patterns

| API | Use when |
| --- | --- |
| `list_page` | Interactive UI / one screen |
| `list_all` | Bounded export / reconciliation |
| `stream_pages` / `stream_items` | Async processing without loading everything first |

Always pass a `PaginationGuard` for multi-page walks. Guards enforce page and item budgets so walks are never unbounded.

HAL list facades with the full surface (`list_page` / `list_all` / `stream_*`):
payments, refunds, captures, payouts, payment links, mandates, subscriptions,
terminals, Connect balance transfers, unmatched credit transfers. See
`docs/rc/pagination-matrix.md` for intentional non-list residuals.

## Safety kernel

- **Origin allowlist** on `next` links (foreign hosts rejected)
- **Cycle detection** on repeating cursors (including multi-page cycles)
- Invalid `limit` values fail closed before HTTP

```rust,no_run
use mollie_rs::PaginationGuard;

# async fn example(client: mollie_rs::MollieClient) -> Result<(), mollie_rs::MollieError> {
let guard = PaginationGuard::new(20, 500); // max_pages, max_items
let items = client.payments().list_all(Some(50), guard).await?;
let _ = items;
# Ok(())
# }
```

Check `PaginationGuard` constructors in rustdoc for the exact budget API on your version.

Streaming example (same guard rules as `list_all`):

```rust,no_run
use mollie_rs::PaginationGuard;

# async fn example(client: mollie_rs::MollieClient) -> Result<(), mollie_rs::MollieError> {
let mut stream = client
    .payments()
    .stream_items(Some(50), PaginationGuard::default_safe());
while let Some(_payment) = stream.next_item().await? {
    // process one payment
}
# Ok(())
# }
```
