 Mollie API Rust SDK

[![CI](https://github.com/SuitsFinance/mollie-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SuitsFinance/mollie-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/mollie-rs.svg)](https://crates.io/crates/mollie-rs)
[![docs.rs](https://docs.rs/mollie-rs/badge.svg)](https://docs.rs/mollie-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-orange.svg)](docs/compatibility.md)

**Version:** `0.7.0` · **MSRV:** `1.88`

Typed Rust SDK for the Mollie API. Built and maintained by **Suits Finance B.V.**

> **Unofficial community SDK.** This project is owned and maintained by Suits Finance B.V. It is not affiliated with, endorsed by, or supported by Mollie B.V. “Mollie” is a trademark of Mollie B.V., used here only to describe API compatibility. For official product documentation see [docs.mollie.com](https://docs.mollie.com/).

`MollieClient` is the recommended entry point for application code. It builds a reusable HTTPS-capable `reqwest` client, configures typed bearer authentication, and still exposes the full typed route surface from `Client`.

## Status

**Early production - use with caveats.** Core Payments, Refunds, Captures, Subscriptions, Mandates, Payment Links, payouts/transfers, OAuth/Connect, and signed webhooks are covered by the pinned contract and exercised in CI. High-risk writes are fail-closed under a frozen **23/23** safety coverage metric. The crate is `0.x`, so minor releases may break the public API.

Your application still owns idempotency keys, webhook dedupe, and authoritative refetch after classic webhooks — see [`docs/guides/safe-payment-retry.md`](docs/guides/safe-payment-retry.md) and [`SECURITY.md`](SECURITY.md).

Read [`docs/release-readiness.md`](docs/release-readiness.md) and [`docs/audits/`](docs/audits/) before adopting this for production payment infrastructure; they document known gaps against the official SDKs honestly.

**Compatibility:** see [`docs/compatibility.md`](docs/compatibility.md) (facade vs generated tiers, MSRV **1.88**, feature flags).  
**API stability:** [`docs/API-STABILITY.md`](docs/API-STABILITY.md) · **Release readiness:** [`docs/release-readiness.md`](docs/release-readiness.md).  
**Safe retries / Unknown outcomes:** [`docs/guides/safe-payment-retry.md`](docs/guides/safe-payment-retry.md).
**Guides index:** [`docs/guides/README.md`](docs/guides/README.md).

## Install

```toml
[dependencies]
mollie-rs = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Default features include `app-helpers` (`.env` loading via `dotenvy` and `init_tracing` via `tracing-subscriber`). Library embeddings that must not load `.env` or install a global subscriber:

```toml
mollie-rs = { version = "0.7", default-features = false }
```

### Production features (0.7)

- **Retries:** disabled by default; enable with `RetryPolicy::default_safe()` (reads + sticky-key writes only).
- **Idempotency:** prefer `IdempotencyKey` on facade `create` methods; avoid long-lived sticky keys.
- **Webhooks:** classic parse + Next-gen HMAC via `WebhookVerifier` / `client.webhooks()`; refetch for classic; event fetch for authenticity. See `SECURITY.md`.
- **Metadata:** `response.metadata()` / `error.metadata()`.
- **Domain facades:** payments, refunds, captures, subscriptions, mandates, payment links, webhooks, payouts, transfers, Connect balance transfers, payee verifications, unmatched CT, sessions, terminals, OAuth — validated builders where available.
- **Optional `zeroize`:** zero credential secret material on drop (`features = ["zeroize"]`).
- **EmptyResponse:** typed empty cancel/revoke/delete bodies on facades.

```rust
// Request-scoped idempotent refund create (validated builder)
use mollie_rs::{CreateRefundRequired, IdempotencyKey, Money, PaymentId};

// async fn demo(client: mollie_rs::MollieClient) -> Result<(), mollie_rs::MollieError> {
let payment = PaymentId::parse("tr_WDqYK6vllg")?;
let required = CreateRefundRequired::new(Money::new("EUR", "1.00")?, "Partial refund")?;
let key = IdempotencyKey::generate();
let _refund = client
    .refunds()
    .create(&payment, required, Some(key))
    .await?;
// Ok(())
// }
```

## Quick Start

```rust
use mollie_rs::MollieClient;

fn create_client() -> Result<MollieClient, mollie_rs::MollieError> {
    MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
}
```

### From `.env` / process environment

Copy `.env.example` to `.env`, set `MOLLIE_API_KEY` (or `MOLLIE_OAUTH_ACCESS_TOKEN`), then:

```rust
use mollie_rs::{init_tracing, MollieClient};

fn create_client_from_env() -> Result<MollieClient, mollie_rs::MollieError> {
    init_tracing()?; // honors RUST_LOG; default filter is `info`
    MollieClient::from_env() // loads `.env` when present
}
```

`from_env` loads `.env` automatically (missing file is ignored) and does not overwrite variables already set in the process. Optional `MOLLIE_BASE_URL` overrides the default API base URL. Use `try_init_tracing()` when a second init should be ignored instead of returning an error.

Helpers: `mollie_rs::init_tracing`, re-exports of `tracing` / `tracing_subscriber`, plus the same names on `mollie_rs::prelude::*`.

## Typed Credentials

Use `ApiKey` or `OAuthAccessToken` when credentials come from env vars or user-managed settings and you want validation before the first request.

```rust
use mollie_rs::{ApiKey, Credential, MollieClient};

fn create_configured_client() -> Result<MollieClient, mollie_rs::MollieError> {
    let api_key = ApiKey::new("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    MollieClient::builder()
        .credential(Credential::from(api_key))
        .build()
}
```

Or load a validated key from the environment (`.env` is loaded inside `from_env`):

```rust
use mollie_rs::{ApiKey, Credential, MollieClient};

fn create_configured_client_from_env() -> Result<MollieClient, mollie_rs::MollieError> {
    let api_key = ApiKey::from_env()?;

    MollieClient::builder()
        .credential(Credential::from(api_key))
        .build()
}
```

## Payment methods

Use `PaymentMethod` to validate method identifiers before sending requests. The set matches Mollie request `method` values (`ideal`, `creditcard`, …) and rejects invalid values such as `googlepay` locally.

```rust
use mollie_rs::{types::CreatePaymentRequest, Money, PaymentMethod};

fn payment_with_method() -> Result<CreatePaymentRequest, mollie_rs::MollieError> {
    Ok(CreatePaymentRequest {
        amount: Some(Money::new("EUR", "10.00")?.into()),
        description: Some("Order #12345".parse().expect("static description is valid")),
        redirect_url: Some("https://example.com/return".to_string()),
        // Constant, or `PaymentMethod::parse("ideal")?.into()`
        method: PaymentMethod::IDEAL.into(),
        ..Default::default()
    })
}

fn payment_link_allowed_methods() -> Result<mollie_rs::types::PaymentLinkMethods, mollie_rs::MollieError> {
    // For CreatePaymentLinkBody.allowed_methods
    PaymentMethod::payment_link_methods([PaymentMethod::IDEAL, PaymentMethod::BANCONTACT])
}
```

## Locales

Use `Locale` for hosted payment-page languages. Hosted values (`nl_NL`, `en_US`, …) are named enum variants; any other ISO 15897 `xx_XX` form is accepted as `Locale::Other`. Convert with `into_generated()` when assigning to `CreatePaymentRequest.locale` (requires a locale present on the generated OpenAPI enum).

```rust
use mollie_rs::{types::CreatePaymentRequest, Locale, Money};

fn payment_with_locale() -> Result<CreatePaymentRequest, mollie_rs::MollieError> {
    Ok(CreatePaymentRequest {
        amount: Some(Money::new("EUR", "10.00")?.into()),
        description: Some("Order #12345".parse().expect("static description is valid")),
        redirect_url: Some("https://example.com/return".to_string()),
        locale: Some(Locale::NL_NL.into_generated()?),
        // or: Some(Locale::parse("en_US")?.into_generated()?)
        ..Default::default()
    })
}
```

## Country codes (ISO 3166-1 alpha-2)

Use `CountryCode` for billing/address country fields. See `docs/iso/` for ISO 3166-1, 4217, 8601, and 15897 notes.

```rust
use mollie_rs::CountryCode;

fn billing_country() -> Result<&'static str, mollie_rs::MollieError> {
    Ok(CountryCode::parse("NL")?.as_str()) // "NL"
}
```

## Phone numbers (E.164)

All Mollie phone fields must use E.164 strings. See `docs/e.164.md`.

```rust
use mollie_rs::PhoneNumber;

fn customer_phone() -> Result<String, mollie_rs::MollieError> {
    Ok(PhoneNumber::parse("+31208202070")?.into())
}
```

## Datetimes (ISO 8601)

Use `DateTime` for offset-aware timestamps and `Date` for `YYYY-MM-DD` calendar fields. See `docs/iso/iso-8601.md`.

```rust
use mollie_rs::{Date, DateTime};

fn expires_at() -> Result<String, mollie_rs::MollieError> {
    Ok(DateTime::parse("2026-07-13T12:00:00+00:00")?.to_rfc3339())
}

fn due_date() -> Result<chrono::NaiveDate, mollie_rs::MollieError> {
    Ok(Date::parse("2026-07-13")?.as_naive())
}
```

## Resource ids

Validate id **prefixes** before calling routes so a profile id is never sent as a payment id (and vice versa).

| Prefix | Resource | Type |
| --- | --- | --- |
| `tr_` | payment | `PaymentId` |
| `pfl_` | profile | `ProfileId` |

```rust
use mollie_rs::{PaymentId, ProfileId};

fn ids() -> Result<(), mollie_rs::MollieError> {
    let payment = PaymentId::parse("tr_WDqYK6vllg")?;
    let profile = ProfileId::parse("pfl_QkEhN94Ba")?;
    assert!(PaymentId::parse(profile.as_str()).is_err());
    assert!(ProfileId::parse(payment.as_str()).is_err());
    Ok(())
}
```

## Create a Payment

Use `CreatePaymentRequired` for the three required body fields (`description`, `amount`, `redirectUrl`) so invalid values fail locally.

```rust
use mollie_rs::{
    CreatePaymentRequired, IntoMollieFuture, Locale, MollieClient, Money, PaymentMethod,
};

async fn create_payment(client: &MollieClient) -> Result<(), mollie_rs::MollieError> {
    let mut payment_request = CreatePaymentRequired::new(
        "Order #12345",
        Money::new("EUR", "10.00")?,
        "https://example.com/return",
    )?
    .into_payment_request();
    payment_request.webhook_url = Some("https://example.com/webhook".to_string());
    payment_request.method = PaymentMethod::IDEAL.into();
    payment_request.locale = Some(Locale::NL_NL.into_generated()?);

    // Sticky key for retries of this logical create (optional; omit for auto UUID).
    let client = client.with_idempotency_key_ref("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");

    let response = client
        .create_payment(None, &payment_request)
        .into_mollie_result()
        .await?;

    let payment = response.into_inner();
    println!("Payment created: {:?}", payment);

    Ok(())
}
```

`MollieClient` dereferences to `Client`, so every typed route remains callable. Use `IntoMollieFuture` on route futures to get `MollieResponse<T>` or `MollieResult<T>` directly when awaited.

Client-owned request policy (not per-route arguments):

- **Idempotency:** `with_idempotency_key` / auto UUID v4
- **Test mode:** `with_testmode(true)` (or builder `.testmode(true)`) for OAuth org tokens that need test entities. Support is operation-specific: the sticky query is sent only by routes that declare it in the OpenAPI contract. Request-body `testmode` fields are separate. See [`docs/contracts/test-mode.md`](docs/contracts/test-mode.md).

## SDK Types

- `ApiKey` and `OAuthAccessToken` validate bearer secrets and redact debug output.
- `Currency`, `AmountValue`, and `Money` validate supported currency/value pairs before converting into `types::Amount` (payments, refunds, captures, balances, settlements). `ApplicationFee` validates Mollie Connect fee amount + description for payment / payment-link / subscription bodies.
- `MollieEnvelope<T>` and `MollieResponse<T>` are shared aliases for response envelopes.
- `GeneratedMollieResult<T>` is the shared route result shape for operations with Mollie's documented error body.
- **Typed errors:** `MollieError` with factories (`rate_limit_exceeded`, `validation_error`, `entity_not_found`, `invalid_cursor`, …), match helpers (`is_rate_limited`, `is_not_found`), and `to_envelope()` → JSON with `ok: false`, `code`, `key`, `message_key` (see `docs/contracts/mollieError.md`).
- **Typed success:** `ResponseEnvelope<T>` remains the primary result; `to_success_envelope()` yields parallel JSON with `ok: true` and status keys (`OK` / `CREATED` / …). Shared constructors live in `mollie_rs::factory`.
- Non-success HTTP responses always decode Mollie’s HAL error body into `MollieError::Api` (including global 429) instead of a bare unexpected status.
- `mollie_rs::types::*` request and response structs are the typed API payloads.

## Docs

| Doc | Purpose |
| --- | --- |
| [`docs/compatibility.md`](docs/compatibility.md) | Facade vs generated tiers, features, MSRV |
| [`docs/API-STABILITY.md`](docs/API-STABILITY.md) | Public API stability posture |
| [`docs/release-readiness.md`](docs/release-readiness.md) | Production readiness band |
| [`docs/production-checklist.md`](docs/production-checklist.md) | Operator checklist for integrators |
| [`SECURITY.md`](SECURITY.md) | Vulnerability reporting + webhook guidance |
| [`NOTICE`](NOTICE) | Third-party attribution + spec licensing |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Dev setup, architecture rules |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history |
| [`docs/route-coverage.md`](docs/route-coverage.md) | Generated route matrix |
| [`docs/route-examples.md`](docs/route-examples.md) | Call-shape examples per route |
| [`docs/contracts/`](docs/contracts/) | Facade contracts |

`examples/<method>.rs` compile-check one binary per route method. Examples call `try_init_tracing` + `MollieClient::from_env` (dotenv is loaded inside `from_env`). Set `MOLLIE_API_KEY` (or a `.env` file) before running against Mollie; missing credentials are treated as a local skip.

Each example appends to `logs/<example>.log` and refreshes `docs/example-support-matrix.md`. Offline rebuild: `python scripts/rebuild_example_support_matrix.py`.

## License

MIT — see [`LICENSE`](LICENSE). Copyright © 2026 Suits Finance B.V.

**Third-party material:** the vendored Mollie OpenAPI documents (`specs.yaml`, `specs-3.0.yaml`) are copyright Mollie B.V. and licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/), **not** MIT. They are excluded from the published crate. mollie-rs is a non-commercial, open-source project, which is compatible with that licence; see [`NOTICE`](NOTICE) for full attribution and the constraints that carry over to anyone building on this repository.

## Security & conduct

- Report vulnerabilities privately via [`SECURITY.md`](SECURITY.md).
- Community standards: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## Regenerating the Client

`specs-3.0.yaml` and `specs.yaml` are the pinned Mollie OpenAPI documents (third-party, CC BY-NC-SA 4.0 — see [`NOTICE`](NOTICE)). The generation and contract-gate scripts need PyYAML:

```sh
python -m pip install -r scripts/requirements.txt
```

After updating the specs, regenerate the checked-in typed client and examples:

```sh
sh scripts/generate_openapi_client.sh
sh scripts/check_route_examples.sh
cargo fmt --all -- --check
```

On Windows, use:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/generate_openapi_client.ps1
powershell -ExecutionPolicy Bypass -File scripts/check_route_examples.ps1
cargo fmt --all -- --check
```
