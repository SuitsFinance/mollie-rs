# SDD 13 — Documentation

Guides under `docs/guides/`:

| Guide | Covers |
| ----- | ------ |
| `safe-payment-retry.md` | Sticky keys, RetryClass, DeliveryOutcome, cancellation → Unknown |
| `handle-signed-webhook.md` | HMAC verify, raw body, non-authoritative webhooks |

Program / release:

| Doc | Role |
| --- | ---- |
| `docs/release-readiness.md` | Honest 1.0 band + primary metric |
| `docs/API-STABILITY.md` | Public surface posture |
| `docs/sdd/1.0-readiness/*` | Spec pack |

Every financial-write facade path documents sticky keys (see payouts/transfers domain rustdoc + safe-payment-retry).

## Acceptance

- [x] Unknown/cancel documented in safe-payment-retry
- [x] Release readiness + API stability published
- [x] README links stability / readiness / retry guide
