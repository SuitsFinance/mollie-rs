# Webhooks guide

Tier-S: `client.webhooks()` + `WebhookVerifier`.

- **Verify before decode** (HMAC).
- Constant-time compare.
- Body size limits apply.
- **Replay ownership is application-side** — store event ids; SDK cannot provide distributed replay storage.
- Support current + previous secrets where configured.
