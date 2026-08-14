# Official SDK feature matrix (`mollie-rs` 0.6.1)

Cross-SDK comparison. Status values: **Y** full, **P** partial, **G** generated-only, **N** no, **n/a** not applicable, **!** should not copy, **?** needs contract verify.

Provider lifecycle maturity (`ga` / `beta` / `private_beta`) is tracked separately in `docs/registries/provider-maturity.yaml` (see SUI-2369 for Sales Invoices GA).

Evidence bases: local `main` 0.6.1; PHP/TS/Go/Java/C# default branches as of 2026-08-04; official openapi 124 ops.

| Feature | PHP | TS | Go | Java | C# | mollie-rs | Status label |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OpenAPI-aligned ops (~124) | P | Y | Y | Y | Y | P (100) | Partially implemented |
| Payments CRUD | Y | Y | Y | Y | Y | Y | Implemented |
| Refunds / captures / chargebacks | Y | Y | Y | Y | Y | Y | Implemented |
| Customers / mandates / subscriptions | Y | Y | Y | Y | Y | Y | Implemented |
| Payment links | Y | Y | Y | Y | Y | Y | Implemented |
| Profiles / methods / onboarding | Y | Y | Y | Y | Y | Y | Implemented |
| Organizations / permissions / clients | Y | Y | Y | Y | Y | Y | Generated only |
| Connect balance transfers | Y | Y | Y | Y | Y | Y | Generated only |
| Balances / settlements / invoices | Y | Y | Y | Y | Y | Y | Generated only |
| Sales invoices | Y | Y | Y | Y | Y | Y | Generated only (provider GA) |
| Terminals list/get | Y | Y | Y | Y | Y | Y | Generated only |
| Terminal pairing codes | Y | Y | Y | Y | Y | N | Requires provider-contract verification |
| Webhooks CRUD + events | Y | Y | Y | Y | Y | Y | Partially (facade for CRUD) |
| Business accounts | N/P | Y | Y | Y | Y | N | Requires provider-contract verification |
| BA transfers | N/P | Y | Y | Y | Y | N | Requires provider-contract verification |
| Payouts | Y | Y | Y | Y | Y | N | Requires provider-contract verification |
| Sessions | Y | Y | Y | Y | Y | N | Requires provider-contract verification |
| Unmatched credit transfers | N/P | Y | Y | Y | Y | N | Requires provider-contract verification |
| Verify payee | N/P | Y | Y | Y | Y | N | Requires provider-contract verification |
| Delayed routing get route | P | Y | Y | Y | Y | P (list/create) | Partially implemented |
| OAuth token generate/revoke | P | Y | Y | Y | Y | N | Requires provider-contract verification |
| API key auth | Y | Y | Y | Y | Y | Y | Implemented |
| OAuth access token auth | Y | Y | Y | Y | Y | Y | Implemented |
| Basic auth client credentials | Y | Y | Y | Y | Y | Y | Implemented |
| Sticky / global testmode | Y | Y | Y | Y | Y | Y | Implemented |
| Sticky / global profileId | P | Y | Y | Y | Y | N | Facade missing |
| Operation-level profile override | Y | Y | Y | Y | Y | Y | Implemented (params) |
| Custom user-agent | Y | Y | Y | Y | Y | Y | Implemented |
| User-agent suffix | P | Y | Y | Y | Y | N | Facade missing |
| Idempotency keys | Y | Y | Y | Y | Y | Y | Implemented |
| Safe default: no unsafe write retry | P | P | P | P | P | Y | Implemented (stronger) |
| Opt-in retries | Y | Y | Y | Y | Y | Y | Implemented |
| Route-aware retry class | P | P | P | P | P | P | Partially implemented |
| 429 Retry-After | Y | Y | Y | Y | Y | P | Partially implemented |
| Total deadline / budget | P | P | P | P | P | P | Partially implemented |
| Cursor pagination | Y | Y | Y | Y | Y | Y | Implemented |
| Bounded list_all | P | P | P | P | P | Y | Implemented (stronger) |
| Stream pages/items | Y (lazy) | P | P | P | P | P | Partially implemented |
| Custom HTTP client | Y | Y | Y | Y | Y | P | Partially implemented |
| Request lifecycle hooks | Y | Y | Y | Y | Y | N | Facade missing |
| HTTP adapters abstraction | Y | ! | ! | ! | ! | ! | Should not be copied (prefer reqwest inject) |
| Webhook HMAC verify | Y | P | P | P | P | Y | Implemented |
| Classic webhook form parse | Y | P | P | P | P | Y | Implemented |
| Webhook event type catalog | Y | G | G | G | G | N | Partially / Generated only elsewhere |
| Validated money types | N | N | N | N | N | Y | Implemented (Rust-native strength) |
| Typed resource IDs | N | P | P | P | P | Y | Implemented |
| Structured error catalog | P | P | P | P | P | Y | Implemented |
| Response metadata | P | P | P | P | P | Y | Implemented |
| Redacted Debug for secrets | P | P | P | P | P | Y | Implemented |
| Optional zeroize | N | N | N | N | N | Y | Implemented |
| Task recipes | Y | P | P | P | P | N | Facade missing / docs gap |
| Migration guides | Y | Y | Y | Y | Y | P | Partially implemented |
| WireMock / mock tests | Y | P | P | P | P | Y | Implemented |
| Generation reproducibility CI | n/a | Y | Y | Y | Y | Y | Implemented |
| Blocking dependency audit | P | P | P | P | P | Y | Implemented |
| Upstream OpenAPI blocking drift | N | n/a | n/a | n/a | n/a | P | Partially (advisory) |
| Streaming downloads | P | P | P | P | P | N | Not applicable / low priority |
| File upload helpers | P | P | P | P | P | N | Requires provider-contract verification if API adds files |

## Domain facade depth (mollie-rs only)

| Domain | Generated routes | Tier S facade | Validated write builder | Pagination helpers |
| --- | --- | --- | --- | --- |
| Payments | Y | Y | Y | list_page, list_all |
| Refunds | Y | Y | Y | list_page, list_all |
| Captures | Y | Y | P | list_page, list_all |
| Subscriptions | Y | Y | Y | list_page |
| Mandates | Y | Y | P | list_page, list_all |
| Payment links | Y | Y | P | list_page, list_all |
| Webhooks | Y | Y | N | list via generated |
| Customers | Y | N | N | generated |
| Settlements | Y | N | n/a | generated |
| Organizations | Y | N | n/a | generated |
| Connect transfers | Y | N | N | generated |
| Payouts | N | N | N | — |
| Accounts | N | N | N | — |
| OAuth tokens | N | N | N | — |

## Gap → proposed Rust API (selected)

| Gap | Proposed API | Belongs in crate? | Priority |
| --- | --- | --- | --- |
| Sticky profile | `MollieClientBuilder::profile_id`, `with_profile_id` | Yes | P1 |
| Scoped credentials | `client.with_credential(Credential::…)?` | Yes | P1 |
| Hooks | `RequestHook` trait on builder | Yes (narrow) | P2 |
| OAuth | `client.oauth().generate/revoke` | Yes after contract | P0/P1 |
| Payouts | `client.payouts().create/list/get/cancel` | Yes after contract | P1 |
| Deadline fix | rename/clarify `retry_budget` + no leftover send | Yes | P0 |
| Operation registry CI | compare YAML registry to upstream | Yes | P0 |
| Axum webhook recipe | `docs/recipes/axum-payment-webhook.rs` | Docs only | P2 |
| Webhook store trait | `WebhookEventStore` in docs/contracts | Boundary only | P2 |
| Generic PSP trait | — | **No** | — |

## Notes on Speakeasy SDKs

TS/Go/Java/C# share nearly the same generated surface (accounts, payouts, sessions, transfers, unmatched CT, verify payee, oauth, pairing codes). PHP is hand-maintained and lagging some BA surfaces but leads on recipes, adapters, middleware, and webhook event mapping.

`mollie-rs` should **import contract coverage** from OpenAPI/Speakeasy peers while **keeping stronger payment-safety defaults** than those generated SDKs.
