# mollieErrorEnvelope

## Summary
`MollieErrorEnvelope` is the consistent JSON-serializable shape for every `MollieError`. Always includes `ok: false` so it pairs with `MollieSuccessEnvelope`.

## Symbol
- Name: `MollieErrorEnvelope`
- Kind: `struct`
- Owner: `mollie_rs::error_catalog`

## Location
- `src/error_catalog.rs`

## Fields
| Field | Type | Notes |
| --- | --- | --- |
| `ok` | `bool` | Always `false` |
| `status` | `Option<u16>` | HTTP status when present |
| `code` | `u32` | Catalog code |
| `key` | `MollieErrorKey` | UPPER_SNAKE via serde |
| `message_key` | `&'static str` | i18n key |
| `title` | `Option<String>` | Mollie/SDK title |
| `detail` | `String` | Human detail |
| `field` | `Option<String>` | Request field when set |
| `documentation` | `Option<String>` | Docs URL |

## Example (429)
```json
{
  "ok": false,
  "status": 429,
  "code": 42901,
  "key": "RATE_LIMIT_EXCEEDED",
  "message_key": "errors.too_many_requests.rate_limit_exceeded",
  "title": "Too Many Requests",
  "detail": "You have exceeded the rate limit. Please slow down your requests.",
  "documentation": "https://docs.mollie.com/overview/handling-errors"
}
```

## Source of Truth
- `src/error_catalog.rs` via `MollieError::to_envelope`
