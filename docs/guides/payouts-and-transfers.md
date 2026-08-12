# High-risk money: payouts & transfers

Use **Tier-S** facades (`client.payouts()`, `client.transfers()`) — not ad-hoc JSON — for money movement. Writes inherit the transport safety kernel.

## Payouts

`create_payout` / `cancel_payout` are **IdempotentWrite**. Bind a caller-owned sticky key before the first attempt.

```rust,no_run
use mollie_rs::{CreatePayoutRequired, IdempotencyKey, MollieClient, Money};

# async fn ex(client: MollieClient) -> Result<(), mollie_rs::MollieError> {
let key = IdempotencyKey::new("ledger-payout-991")?;
let required = CreatePayoutRequired::full_balance_str("bal_gVMhHKqSSRYJyPsuoPNFH")?
    .with_amount(Money::new("EUR", "10.00")?); // or omit amount for full balance
let payout = client.payouts().create(required, Some(key)).await?.into_inner();
let _ = payout;
# Ok(())
# }
```

On `error.is_outcome_unknown()`, reconcile with `payouts().get(...)` or retry **only** with the **same** sticky key.

## Transfers

`create_transfer` is **IdempotentWrite** and requires non-empty:

- sticky `IdempotencyKey`
- `X-Client-Signature` / `X-Client-Signed-At` via `TransferClientSignature`

Empty signature or key is refused fail-closed (no auto-generation).

```rust,no_run
use mollie_rs::types::TransferSchemeType;
use mollie_rs::{
    CreateTransferRequired, IdempotencyKey, MollieClient, Money, TransferClientSignature,
};

# async fn ex(client: MollieClient) -> Result<(), mollie_rs::MollieError> {
let key = IdempotencyKey::new("ba-transfer-42")?;
let required = CreateTransferRequired::new(
    Money::new("EUR", "5.00")?,
    "NL91ABNA0417164300",
    "Creditor Name",
    "NL39RABO0300065264",
    TransferSchemeType::SepaCredit,
)?;
let sig = TransferClientSignature {
    signature: "base64-signature-from-your-hsm",
    signed_at: "2026-03-22T12:00:00Z",
};
let _ = client.transfers().create(required, &key, sig).await?;
# Ok(())
# }
```

## Verify payee / OAuth

Classified **NonRetryableWrite** — transport never auto-retries. Treat timeouts as **Unknown**; do not invent a second distinct request body without operator policy.

## See also

- [`safe-payment-retry.md`](safe-payment-retry.md) — DeliveryOutcome, cancellation
- [`../API-STABILITY.md`](../API-STABILITY.md) — public surface
- [`../release-readiness.md`](../release-readiness.md) — 1.0 band

