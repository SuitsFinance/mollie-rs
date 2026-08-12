# mollieSuccessEnvelope

## Summary
`MollieSuccessEnvelope<T>` is the serializable success counterpart to `MollieErrorEnvelope`. Primary Rust success type remains `ResponseEnvelope<T>` (typed body + headers); call `to_success_envelope()` at app boundaries.

## Symbol
- Name: `MollieSuccessEnvelope`
- Kind: `struct`
- Owner: `mollie_rs::error_catalog`

## Location
- `src/error_catalog.rs`
- Built from `ResponseEnvelope::to_success_envelope` in `src/envelope.rs`
- Factories: `mollie_rs::factory::success_ok` / `success_created` / …

## Fields
| Field | Type | Notes |
| --- | --- | --- |
| `ok` | `bool` | Always `true` |
| `status` | `u16` | HTTP status |
| `code` | `u32` | e.g. 20000, 20100 |
| `key` | `MollieSuccessKey` | `OK`, `CREATED`, `ACCEPTED`, `NO_CONTENT` |
| `message_key` | `&'static str` | e.g. `success.ok` |
| `data` | `T` | Same typed body as the route |

## Status map
| HTTP | code | key |
| --- | --- | --- |
| 200 | 20000 | `OK` |
| 201 | 20100 | `CREATED` |
| 202 | 20200 | `ACCEPTED` |
| 204 | 20400 | `NO_CONTENT` |

## Examples
```rust
use mollie_rs::ResponseEnvelope;

let success = ResponseEnvelope::ok("customer").to_success_envelope();
assert!(success.ok);
assert_eq!(success.code, 20000);
assert_eq!(success.data, "customer");
```

```json
{
  "ok": true,
  "status": 200,
  "code": 20000,
  "key": "OK",
  "message_key": "success.ok",
  "data": { }
}
```

## Example: `list_capabilities` 200
`ResponseEnvelope<ListCapabilitiesResponse>` → `to_success_envelope()` yields `ok: true`, `code: 20000`, `key: OK`, with `data` shaped as:

- `count` — number of capabilities  
- `_embedded.capabilities[]` — each with `name` (`payments` / `settlements`), `status`, optional `statusReason`, and `requirements[]`  

Regression fixture: `src/capabilities_fixture.rs` (unit tests).

## Source of Truth
- `src/error_catalog.rs`, `src/envelope.rs`, `src/factory.rs`
