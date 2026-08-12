# Fuzz Targets

libFuzzer targets for the parsers and transport helpers where malformed input is
attacker-controlled or externally supplied. See [`../README.md`](../README.md)
for how to install `cargo-fuzz` and run a target.

## Targets

| Target | Entry points | Why it is fuzzed |
| --- | --- | --- |
| `webhook_signature.rs` | `WebhookVerifier`, `compute_mollie_signature_hex` | Attacker-controlled bodies and signature headers reach HMAC verification. A panic here is a DoS on the webhook endpoint; a wrong `true` is a forged-webhook vulnerability. |
| `webhook_form.rs` | `WebhookNotification::parse_form_urlencoded` | Classic webhooks arrive as raw form-urlencoded bytes straight off the network. |
| `money_amount.rs` | `Currency::parse`, `Money::new`, `AmountValue::parse` | Money parsing guards financial correctness; these values flow into payment requests. |
| `payment_id.rs` | `PaymentId::parse` | Resource ids are frequently built from untrusted routing/user input. |
| `page_cursor.rs` | `PageCursor::new`, `PageCursor::from_list_link` | Cursors are parsed out of provider-supplied `_links` hrefs. |
| `retry_after_header.rs` | `compute_backoff`, `RetryPolicy` | `Retry-After` is provider-controlled; bad values must not panic or produce pathological sleeps. |

## Invariants under test

Every target asserts the same baseline contract: **no panics, no aborts, no
unbounded work** for any byte sequence. Parsers must reject bad input by
returning an error rather than unwrapping, indexing out of bounds, or slicing a
multi-byte UTF-8 character.

`webhook_signature` carries the extra security invariant: verification must not
accept a signature it did not compute.

## Adding a target

1. Add `fuzz_targets/<name>.rs` with `#![no_main]` and a `fuzz_target!` body.
2. Register it as a `[[bin]]` in [`../Cargo.toml`](../Cargo.toml).
3. Document it in the table above and in [`../README.md`](../README.md).

Keep targets fast and allocation-light — the fuzzer's value comes from
executions per second. Derive structured inputs from the raw byte slice (see how
`webhook_signature` splits the buffer into body and signature) rather than
pulling in heavier input-generation machinery.
