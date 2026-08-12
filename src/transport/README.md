# Transport Policy

Retries, rate limits, deadlines, and delivery outcomes. This is the layer that
decides **whether a request may be sent again** — the single most dangerous
decision in a payments client.

## The core rule

> A payment write is never retried automatically unless the operation is
> classified safe **and** a sticky idempotency key is in play.

Both conditions, not either. A safe classification without a stable idempotency
key means a retry could create a second payment; an idempotency key on an
operation whose safety is unknown means we are guessing. Retries here are
opt-in and conservative by design: the failure mode of retrying too little is a
visible error, and the failure mode of retrying too much is duplicate money
movement.

## Delivery outcomes

`delivery.rs` refuses to collapse "we know nothing" into "it failed":

| Outcome | Meaning | Safe to retry? |
| --- | --- | --- |
| `Succeeded` | The provider processed the request | n/a |
| `NotSent` | The request never left the client | Yes — nothing happened |
| `Unknown` | The request may or may not have been processed | Only with idempotency |

`NotSent` and `Unknown` are deliberately distinct. Connection-refused is
`NotSent`. A timeout after the bytes went out is `Unknown` — the payment may
well exist. Treating `Unknown` as failure is how duplicate charges happen.

## Modules

| Module | Responsibility |
| --- | --- |
| `mod.rs` | Module docs and the public transport surface |
| `classification.rs` | Which operations and which failures are retry-eligible |
| `delivery.rs` | `DeliveryOutcome` and the outcome decision |
| `policy.rs` | `RetryPolicy` configuration and its safe default |
| `rate_limit.rs` | Rate-limit state tracking |
| `retry.rs` | Backoff computation, including `Retry-After` handling |

Operation-level safety is *not* defined here — it lives in
`../operation_safety.rs`, the single source of truth shared by transport and the
domain facades. This layer consumes that profile; it does not extend it.

## Changing this code

Treat every change as security-sensitive:

- Broadening retry eligibility requires justification against the rule above.
- `compute_backoff` and the `Retry-After` path are fuzzed — see
  [`../../fuzz/fuzz_targets/`](../../fuzz/fuzz_targets). Provider-supplied
  values must never panic or produce pathological sleeps.
- Never map `Unknown` to a retryable-failure branch without idempotency.

Background reading:
[`../../docs/guides/safe-payment-retry.md`](../../docs/guides/safe-payment-retry.md)
and
[`../../docs/rc/hostile-transport-evidence.md`](../../docs/rc/hostile-transport-evidence.md).
