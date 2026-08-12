# Business-account transfers guide

Tier-S: `client.transfers()`.

- Requires sticky `IdempotencyKey` **and** `TransferClientSignature` (never empty).
- SDK does not invent signatures or keys.
- Signature material must never appear in logs/hooks.
