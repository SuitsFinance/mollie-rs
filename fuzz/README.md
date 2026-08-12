# Fuzz targets for mollie-rs

High-value parsers and transport helpers (security / reliability).

Requires **nightly** Rust and [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz):

```sh
rustup install nightly
cargo install cargo-fuzz
cd fuzz
cargo +nightly fuzz run webhook_signature -- -runs=10000
```

## Targets

| Target | Surface |
| --- | --- |
| `webhook_signature` | Next-gen HMAC verify + header normalize |
| `webhook_form` | Classic `id=` form-urlencoded body |
| `money_amount` | `Currency` / `Money` / `AmountValue` |
| `payment_id` | `PaymentId::parse` |
| `page_cursor` | `PageCursor` + list-link extraction |
| `retry_after_header` | Backoff / `Retry-After` duration inputs |

CI builds these targets on nightly (short smoke); local runs can use larger `-runs`.
