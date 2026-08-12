# SDD 05 — OAuth token lifecycle

## Context
`POST/DELETE /oauth2/tokens` outside `/v2`; Basic client credentials; high secret sensitivity.

## Existing
Tier G `oauth_generate_tokens` / `oauth_revoke_tokens`; path via `Client::endpoint`; NonRetryableWrite.

## Desired
Tier-S helpers with `OAuthClientCredentials` / typed bodies; never Debug secrets; retry Never; delivery classification still applies.
**After kernel** (classification already NonRetryable).

## Acceptance
- [ ] Redaction tests
- [ ] Mock token exchange
- [ ] Cannot pass ApiKey where Basic client secret required (type or runtime check)
