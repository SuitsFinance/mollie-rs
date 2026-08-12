# Compatibility policy

`mollie-rs` exposes both a **stable application facade** and a large **generated** OpenAPI surface. Breaking changes mean different things in each tier. This document is the source of truth for consumers and for release decisions.

## Versioning

| Field | Current |
| ----- | ------- |
| Crate version | `0.7.0` |
| Install example | `mollie-rs = "0.7"` |
| MSRV (`package.rust-version`) | **1.88** |
| Default toolchain pin | `rust-toolchain.toml` → `1.88.0` |

Pre-1.0: minor bumps (`0.x` → `0.y`) **may** include breaking changes. We still document them in `CHANGELOG.md` and prefer deprecation first when practical.

## Stability tiers

### Tier S — Stable facade (preferred for applications)

Examples: `MollieClient`, `MollieClientBuilder`, `Credential` / `ApiKey` / `OAuthAccessToken`, `Money` and related validators, resource ID newtypes, `MollieError` / catalog keys, `ResponseEnvelope` / `IntoMollieResult`, classic `WebhookNotification` / `WebhookUrl`, env constant names.

**Promise (0.x):** avoid silent breaks; deprecate before remove when possible; document migrations.

### Tier G — Generated API

Examples: `Client` route methods, `types::*` request/response models, `routes::*` modules produced from `specs-3.0.yaml`.

**Promise:** faithful to the **checked-in** OpenAPI contract. Regenerating from a newer Mollie specification **may** rename types, add required fields, or change enums even when Mollie’s live API behavior is unchanged. Treat Tier G as **spec-coupled**, not application-stable.

### Tier E — Experimental / policy APIs

Examples: `RetryPolicy`, paginators, Next-gen webhook verifiers, route capability fields, response metadata, `RequestHook`, client `profile_id` defaults, `with_credential`, integration traits (`WebhookEventStore`, …).

**Promise:** may change shape between minors until marked Tier S (target 1.0).

## What kind of change is “breaking”?

| Change class | Typical tier | Release guidance |
| ------------ | ------------ | ---------------- |
| Provider OpenAPI drift (new ops, field renames) | G | Document in changelog; may land in minor pre-1.0 |
| Facade method removal / signature change | S | Deprecate first; prefer next minor with notes |
| New `MollieError` variant | S | Additive in 0.x when variants stay matchable; prefer non-exhaustive long-term |
| Default retry-on behavior | E → S | **Never** enable unsafe write retries by default |
| Feature default flip (`app-helpers`) | S | Announce one minor ahead |
| MSRV increase | all | Allowed in minor; documented |

## Feature flags

| Feature | Default | Purpose |
| ------- | ------- | ------- |
| `app-helpers` | **on** | `dotenvy` `.env` loading + `tracing-subscriber` init helpers |
| `zeroize` | **off** | Zeroize `ApiKey` / `OAuthAccessToken` / `BasicAuth` secret buffers on drop |

Library embeddings that must not load `.env` or install a global subscriber:

```toml
mollie-rs = { version = "0.7", default-features = false }
```

`tracing` spans/events remain available; only the **subscriber installer** is feature-gated.

Enable credential zeroization when secrets must not linger in process memory:

```toml
mollie-rs = { version = "0.7", features = ["zeroize"] }
```

## Generated vs provider specification

| Artifact | Role |
| -------- | ---- |
| `specs-3.0.yaml` | Authoritative input for **this crate’s** generated code |
| Upstream Mollie OpenAPI / docs | Compatibility **reference** for drift reports |
| `docs/route-coverage.md` | Human route matrix |
| `src/route_capabilities.rs` | Machine-readable operation metadata |

CI runs:

1. **Generation reproducibility** — regenerating capabilities/docs from the pinned spec must not produce an unexpected dirty tree (see scripts).  
2. **API drift report** — compares local operation inventory to an optional upstream snapshot; does **not** auto-publish regenerations.

Do **not** auto-merge upstream OpenAPI changes without human review of Tier G fallout.

## MSRV policy

- Declared in `Cargo.toml` as `rust-version`.  
- CI verifies the MSRV toolchain and the pinned `rust-toolchain.toml` channel.  
- MSRV may increase in a minor release when required by security or dependency policy; the bump is listed in `CHANGELOG.md`.

## Related docs

- [Production assessment](assessment-production-sdk.md)  
- [OpenAPI generation](openapi-generation.md)  
- [Route coverage](route-coverage.md)  
- [Security policy](../SECURITY.md)  
