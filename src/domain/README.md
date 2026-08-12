# Domain Facades

**Handwritten.** Unlike [`../routes/`](../routes), nothing in this directory is
generated — edit these files directly.

Domain facades sit on top of the generated route surface and add the things a
1:1 OpenAPI translation cannot: input validation, safe defaults, idempotency
scoping, and pagination ergonomics. They do **not** reimplement HTTP; every
facade ultimately calls a generated `Client` method.

## Contract

Each facade exposes a `*Api` type and follows the same rules:

- **Validate before sending.** Malformed money, ids, or country codes are
  rejected locally rather than round-tripped to Mollie for a 422.
- **Scope idempotency to the operation.** Write operations carry a key derived
  for that specific request, so a retry is genuinely the same request.
- **Never widen retry safety.** Facades read the profile in
  `../operation_safety.rs`; they do not decide it.
- **Return the crate error type.** Callers see one error surface regardless of
  which layer failed.

The full contract is written up in
[`../../docs/specs/domain-facade-contract.md`](../../docs/specs/domain-facade-contract.md).

## Facades

| Module | Domain |
| --- | --- |
| `payments.rs` | Create / get / list payments with safe defaults |
| `refunds.rs` | Payment-scoped refunds |
| `captures.rs` | Payment-scoped captures |
| `payment_links.rs` | Payment links |
| `mandates.rs` | Customer-scoped mandates |
| `subscriptions.rs` | Customer-scoped subscriptions |
| `payouts.rs` | Balance → bank settlement |
| `transfers.rs` | Business-account SEPA credit transfers |
| `unmatched_credit_transfers.rs` | UCT match / return |
| `terminals.rs` | Point-of-sale terminals and pairing codes |
| `sessions.rs` | Components checkout sessions (**beta**) |
| `verify_payee.rs` | Verification of Payee (VoP) |
| `oauth.rs` | OAuth token lifecycle (generate / revoke) |
| `webhooks.rs` | Classic + Next-gen webhook workflow, event fetch |
| `common.rs` | Shared helpers used across facades |

## Choosing a layer

Use a facade by default. Drop to a generated route method when you need an
operation or parameter the facade does not expose — and then take on the
validation and idempotency responsibilities yourself.

## Adding a facade

1. Keep it a thin, honest wrapper. If a facade grows provider-specific
   workarounds, that is a signal the spec or generator should change instead.
2. Reuse `common.rs` rather than duplicating pagination or option plumbing.
3. Cover the validation rules with unit tests — these are the guarantees
   downstream users rely on, and they are not enforced by the generated layer.
4. Guard beta surfaces explicitly, as `sessions.rs` does.
