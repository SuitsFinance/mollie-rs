# Pagination

Mollie list endpoints use opaque `from` cursors and a `limit` (1..=250).

## Safe patterns

| API | Use when |
| --- | --- |
| `list_page` | Interactive UI / one screen |
| `list_all` | Bounded export / reconciliation |
| `stream_pages` / `stream_items` | Async processing without loading everything first |

Always pass a `PaginationGuard` for multi-page walks. Guards enforce page and item budgets so walks are never unbounded.

## Safety kernel

- **Origin allowlist** on `next` links (foreign hosts rejected)
- **Cycle detection** on repeating cursors
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
