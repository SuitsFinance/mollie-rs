# Deep production-foundation audit — mollie-rs 0.6.1

**Audit date:** 2026-08-04  
**HEAD:** `bb5d460` — `feat: official SDK parity audit and client context foundations`  
**Package:** `mollie-rs` 0.6.1  
**Repository:** https://github.com/suitsfinance/mollie-rs (not Mollie-owned)  
**Upstream remote also present:** `brainpodnl/mollie-api-rust`  

This document is the authoritative deep audit for using mollie-rs as a foundation for production payment infrastructure. It supersedes optimistic phrasing in older assessments where evidence conflicts.

---

## 1. Executive verdict

**Use with restrictions** for **core Mollie Payments, Refunds, Captures, Subscriptions, Mandates, Payment Links, and signed webhooks** on the **checked-in 100-operation pin**, provided the application owns idempotency keys, webhook dedupe, and authoritative refetch.

**Do not yet treat mollie-rs as a complete Mollie platform SDK** or as a drop-in peer to official Speakeasy SDKs: **24 operations** present in current official OpenAPI / TS/Go are missing from the local pin (business accounts, payouts, sessions, BA transfers, unmatched credit transfers, verify-payee, terminal pairing codes, OAuth token endpoints, payment route get).

**This is not an official Mollie product.** It is a competent third-party Rust SDK with unusually strong payment-safety instincts relative to many generated SDKs.

## 2. Production-readiness classification

| Classification | Applies? |
| --- | --- |
| Experimental | No for core payment create/get/list |
| **Early production / production with caveats** | **Yes — primary classification** |
| Mature production SDK | No |
| Full payment-infrastructure foundation | No |

**Nuanced:**

| Use case | Verdict |
| --- | --- |
| Hosted checkout payments + classic/HMAC webhooks + refunds | **Production with caveats** |
| Connect multi-merchant / OAuth platform | **Not ready** (no OAuth token ops; thin Connect) |
| Business accounts / payouts / BA transfers | **Not ready** (missing contract) |
| In-person terminals pairing | **Not ready** |
| Settlement/balance reporting (generated reads) | **Usable generated-only** with app wrappers |

## 3. Overall score: **72 / 100**

Weighted methodology in §4. Confidence in overall score: **high** for architecture/safety; **high** for parity gap size; **medium** for untested live provider edge cases.

## 4. Weighted score methodology

| Weight | Axis family | Why heavy |
| --- | --- | --- |
| 25% | Payment safety (retries, idempotency, money, webhooks) | Financial double-charge risk |
| 20% | Official contract parity | Incomplete surface is operational risk |
| 15% | Correctness of transport (deadlines, errors, rate limits) | Reliability under load |
| 12% | Security (secrets, HMAC, logging) | Credential/webhook compromise |
| 10% | Public API / facades / maintainability | Long-term cost |
| 8% | Testing & CI | Regression resistance |
| 5% | Documentation & recipes | Operational adoption |
| 5% | Release / governance / version truth | Consumer trust |

Raw category scores (0–10) × weights → scaled to 100 ≈ **72**.

## 5. Massive rating matrix (0–10)

| Axis | Score | Conf. | Evidence | Strength | Weakness | Improvement |
| --- | --- | --- | --- | --- | --- | --- |
| API coverage | 6.5 | H | 100/124 ops | Core payments covered | BA/payouts/OAuth missing | Re-pin openapi |
| Official-contract parity | 6.0 | H | mollie/openapi 124 vs pin 100 | Registry tracks gaps | Upstream CI URL 404 | Fix URL; block dangerous drift |
| Type safety | 8.5 | H | IDs, Money, builders | Newtypes | Tier G models still leak | Expand facades |
| Money safety | 8.0 | H | string AmountValue, no f64 | No floats | `minor_units` hardcodes 2 (HUF) | Per-currency scale |
| Identifier safety | 8.5 | H | prefix validation | Good | Not every resource id typed | Add BA/payout ids post-contract |
| Authentication | 8.5 | H | ApiKey/OAuth/Basic, redacted Debug | Env + zeroize | Injected http_client skips auth merge | Document/merge headers |
| OAuth | 4.0 | H | Tokens as Bearer only | Basic auth helper | No generate/revoke routes | Pin `/oauth2/tokens` |
| Profile support | 7.5 | H | sticky profile_id + params | Client defaults | Facades don't auto-inject | Wire facades to context |
| Test mode | 8.5 | H | sticky + live-only reject | Correct | Body vs query confusion risk | Docs per-op matrix |
| Idempotency | 9.0 | H | sticky-key write rule | Best-in-class default | Auto UUID can mislead | Docs + metrics key hash |
| Retry safety | 8.5 | H | off by default; sticky writes | Route-aware class | Retry-After seconds only | HTTP-date Retry-After |
| Timeout correctness | 8.0 | M | budget no leftover send | Fixed leftover bug | Named total_deadline ambiguous | Rename retry_budget |
| Error modeling | 8.0 | H | catalog + metadata | Rich | Incomplete operation fill | Always set operation_id |
| Request metadata | 7.5 | M | ResponseMetadata | Good skeleton | Not always populated | Fill on all paths |
| Rate-limit handling | 7.0 | M | headers parsed | Present | No client-side throttle | Optional RateLimitState use |
| Pagination | 8.0 | H | guard + cycle set | Bounded | No stream on all facades | Universal stream_* |
| Webhook verification | 9.0 | H | HMAC raw, CT eq, rotation | Strong | Replay app-owned | Recipes Axum/Actix |
| Webhook processing ergonomics | 6.5 | H | traits only | Correct boundary | No recipes binaries | docs/recipes/* |
| Transport customization | 7.0 | H | http_client inject | Possible | Auth header dual-path | Layered client |
| Observability | 6.5 | M | tracing + hooks | Hooks added | Field schema incomplete | Structured mollie.* fields |
| Logging safety | 8.0 | M | redacted Debug | Good | Need leak tests expansion | Assert no secrets in traces |
| Security posture | 8.0 | H | SECURITY.md solid | Honest webhook guidance | Private vuln path informal | SECURITY contact email |
| Secret handling | 8.5 | H | redaction + zeroize opt | Solid | Webhook secret zeroize partial | Feature parity |
| Zeroization | 7.5 | H | feature flag | Opt-in | Not default | Document when required |
| Generated maintainability | 7.5 | H | progenitor scripts | Reproducible check | Manual regenerations heavy | Automate pin bump |
| Handwritten abstraction quality | 8.5 | H | two-tier clear | Coherent | Facade coverage uneven | Facade gap registry CI |
| Public API consistency | 7.0 | H | mixed Option/&Id patterns | Improving | Domain variance | Canonical list API |
| Documentation | 7.5 | H | contracts + audits | Better than most | PHP recipes stronger | guides/* complete |
| Examples | 8.5 | H | ~100 examples | Route matrix | Not task-oriented | Recipe examples |
| Testing | 7.5 | H | 161 lib tests + wiremock | Good unit | Thin multi-page HTTP | Expand contract matrix |
| CI | 8.0 | H | fmt/clippy/deny/gen | Strong | Upstream advisory + bad URL | mollie/openapi + dangerous fail |
| Release engineering | 7.0 | M | tags, xbp, changelog | Present | Tag naming mixed | Semver release artifact |
| Semver discipline | 7.0 | M | tiers documented | Honest pre-1.0 | Tier G can break minors | Public API diff CI |
| Dependency maintenance | 8.0 | H | cargo-deny | Blocking | — | Keep advisories green |
| MSRV discipline | 8.0 | H | 1.88 pinned | Clear | Aggressive MSRV | Justify 1.88 |
| Onboarding | 7.0 | M | README + examples | OK | No quickstart payments recipe | create-payment guide |
| Low-level escape hatches | 9.0 | H | Client + types | Full routes | Generated noise | Keep |
| High-level facades | 7.0 | H | 7 domains | Core strong | No settlements facade | Add high-value facades |
| Performance | 7.5 | L | reqwest clone Arc | Fine | No benchmarks | Optional benches |
| Memory behavior | 7.5 | M | body bounds | Bounded error bodies | — | Keep limits |
| Cancellation | 6.5 | M | tokio sleep cancelable | Partial | No explicit cancel errors | Propagate cancel |
| Forward compatibility | 7.0 | M | unknown enums? | Spec-coupled | Tier G churn | Facade insulation |
| Operational maturity | 6.5 | H | private third-party | Safety-first | Incomplete platform surface | 0.7 contract |
| **Production readiness** | **7.0** | H | caveats documented | Safe core path | Missing ops + app duties | Roadmap |

## 6. Strengths

1. **Payment-safe defaults:** retries off; writes retry only with sticky caller-owned idempotency keys.
2. **Clear two-tier API:** `MollieClient`/domain facades vs generated `Client`.
3. **Validated money/IDs/credentials** without floating point.
4. **Webhook HMAC** on raw bytes, constant-time compare, rotation, body bounds, classic vs next-gen separation.
5. **Bounded pagination** with cycle detection — better than lazy unbounded iterators.
6. **Route capability registry** + generation reproducibility + cargo-deny CI.
7. **Honest SECURITY.md** (refetch classic webhooks; HMAC ≠ replay protection).
8. **Dense examples** (~100) for generated routes.
9. **Recent foundations:** profile context, hooks, `with_credential`, route-aware retry, deadline leftover fix, operation registry.

## 7. Weaknesses

1. **Incomplete provider contract** (24 missing ops vs official openapi/Speakeasy).
2. **Not official**; private repo; governance is single-maintainer scale.
3. **Facades thin** outside payments stack; most of 100 ops are generated-only.
4. **OAuth incomplete** (no token generate/revoke).
5. **Upstream CI URL** points at non-existent OpenAPI-Specification path.
6. **Registry meta drift** (comments claim 101 ops; actual 100).
7. **Parity assessment doc** partially stale (claims missing profile/hooks already fixed in same release tranche).
8. **Currency minor units** hardcoded to 2 for all listed currencies including HUF.
9. **Task recipes** still thin vs PHP.
10. **No public API semver freeze** (pre-1.0).

## 8. Critical findings

| ID | Finding | Impact |
| --- | --- | --- |
| C1 | Local OpenAPI pin missing 24 live official operations | Platforms needing BA/payouts/OAuth cannot use crate alone |
| C2 | OAuth token endpoints absent (`/oauth2/tokens`) | Cannot complete Connect token lifecycle in-SDK |
| C3 | Application must own idempotency storage + webhook dedupe + refetch | Using SDK without these controls is unsafe regardless of score |

## 9. High-risk findings

| ID | Finding |
| --- | --- |
| H1 | Upstream drift job uses dead URL (`mollie/OpenAPI-Specification/.../openapi-v2.yaml` → historically 404); real pin source is `mollie/openapi` |
| H2 | Dangerous provider changes only advisory in CI |
| H3 | Sticky auto-UUID idempotency can be misread as safe multi-retry identity |
| H4 | Injected `http_client` path does not merge builder Authorization headers |
| H5 | Money `minor_units() == 2` for all currencies — wrong for zero-decimal currencies (e.g. HUF) if Mollie expects `10` not `10.00` |

## 10. Medium-risk findings

| ID | Finding |
| --- | --- |
| M1 | Facades inconsistent: some `list_all`, few `stream_*` |
| M2 | Only 3 ops marked ValidatedFacade in capabilities (builders exist more broadly) |
| M3 | Retry-After parses integer seconds only |
| M4 | Operation/request_id not always on errors |
| M5 | Version/tag naming mixed (`mollie-api-rust-0.6.1` vs package `mollie-rs`) |
| M6 | Registry export meta counts can desync (101 vs 100) |
| M7 | Profile sticky default not auto-applied by all facades |
| M8 | `docs/specs/current-state-audit.md` still cites 0.6.0 Cargo truth |

## 11. Low-risk findings

| ID | Finding |
| --- | --- |
| L1 | Aggressive MSRV 1.88 |
| L2 | Encoding glitches in some markdown (smart quotes / mojibake) |
| L3 | No property/fuzz suites |
| L4 | No criterion benches |
| L5 | License field empty on GitHub API despite MIT in Cargo.toml |

## 12. API parity matrix (condensed)

Status: **C** complete (facade+tests), **G** generated-only, **P** partial, **M** missing pin, **N/A**, **!** do not copy.

| Area | API | Official SDKs | mollie-rs | Notes |
| --- | --- | --- | --- | --- |
| Payments CRUD/update/cancel/release | Y | Y | **C/G** | Facade create/get/list; update/cancel generated |
| Refunds | Y | Y | **C** | Validated create |
| Captures | Y | Y | **P** | Facade list; builder exists |
| Chargebacks | Y | Y | **G** | Read-only generated |
| Customers + payments | Y | Y | **G** | No domain facade |
| Mandates | Y | Y | **P** | SEPA builder |
| Subscriptions | Y | Y | **C** | Validated create |
| Payment links | Y | Y | **P** | Builder + list |
| Profiles/methods/onboarding | Y | Y | **G** | |
| Balances/settlements/invoices | Y | Y | **G** | Live-only testmode reject |
| Connect balance transfers | Y | Y | **G** | |
| Delayed routing | Y | Y | **P** | list/create; **get route missing** |
| Business accounts + txns | Y | Y | **M** | Critical gap |
| BA transfers | Y | Y | **M** | |
| Payouts | Y | Y | **M** | |
| Sessions | Y | Y | **M** | |
| Unmatched CT | Y | Y | **M** | |
| Verify payee | Y | Y | **M** | |
| Terminals list/get | Y | Y | **G** | |
| Terminal pairing | Y | Y | **M** | |
| OAuth generate/revoke | Y | Y | **M** | Outside /v2 |
| Webhooks CRUD + events | Y | Y | **P/C** | Facade + HMAC |
| Classic webhooks | Y | Y | **C** | Form parse |
| Wallets Apple Pay | Y | Y | **G** | |
| Pagination | Y | Y | **P** | Strong primitives, uneven facade |
| Idempotency | Y | Y | **C** | Safer defaults |
| Retries | Y | Y | **C** | Safer defaults |
| Custom HTTP | Y | Y | **P** | reqwest inject |
| Hooks | Y | Speakeasy | **P** | Narrow RequestHook |
| File upload/stream | rare | varies | **N/A/M** | Verify if API adds |

**Operational importance of missing groups:** OAuth/BA/payouts/transfers block **Connect platforms and treasury automation**. Sessions/pairing block **in-person**. UCT/verify-payee block **reconciliation of bank credit**.

## 13. Official SDK comparison

| | PHP | TS/Go/Java/C# | mollie-rs |
| --- | --- | --- | --- |
| Ownership | Mollie | Mollie (Speakeasy) | floris-xlx (third-party) |
| Surface | Hand endpoints | ~120–124 ops | 100 pin |
| Recipes | Best | Generated docs | Thin guides |
| Payment safety defaults | Medium | Medium (generated retries) | **High** |
| Money types | Weak | Weak | **Strong** |
| Webhooks | Strong + event map | Models | Strong verify, thin process |
| HTTP adapters | Strong | httpClient | reqwest-centric |

**Should not copy:** Speakeasy unbounded lazy lists; write retries without sticky keys; PHP adapter zoo without Rust benefit; app ledger inside SDK.

## 14. Architectural assessment

```
App
 └─ Tier S: MollieClient, domain/*, Money/IDs, WebhookVerifier, RetryPolicy, hooks
      └─ Tier G: Client + routes/* + types (progenitor from specs-3.0.yaml)
           └─ Transport: reqwest, send() retry loop
```

**Healthy:** generation scripts, capabilities, facades don't reimplement HTTP.  
**Risk:** public re-export of large `types` couples apps to OpenAPI churn.  
**Predictable add-path:** update specs → generate → capabilities → optional facade.  
**Over-abstracted?** No. **Under-abstracted?** Yes on non-payment domains.

Route-aware retry: **implemented in `Client::send`** via `route_capability(operation.id())` (method fallback only).

## 15. Transport / retry assessment

| Property | Status |
| --- | --- |
| Retries default | Disabled |
| Safe reads | Opt-in |
| Idempotent writes | Sticky key required |
| Route class | Capability-driven |
| Retry-After | Integer seconds |
| total_deadline | Retry budget; **no leftover send** |
| Body non-cloneable | Single attempt |
| Connect/timeout | Builder + injected client |

**Better model (incremental):** keep current policy; add `RequestRetryConfig` overrides; rename field to `retry_budget`; parse HTTP-date Retry-After; populate attempt on all error metadata.

## 16. Security assessment

- Secrets: redacted Debug; optional zeroize; env load under app-helpers.
- Webhooks: raw HMAC; CT compare; rotation; bounds; honest replay stance.
- Gaps: formal security contact; expanded secret-leak tests; webhook secret zeroize; http_client auth merge.

## 17. Financial correctness assessment

- **No f32/f64** in amount path (good).
- `AmountValue` validates digit/scale pattern as string.
- **Issue:** `Currency::minor_units` always returns 2 — HUF (and any zero-decimal) risk.
- Application fees use validated description length.
- Generated amount types still available — apps must prefer facade Money.

## 18. Webhook assessment

| Concern | Status |
| --- | --- |
| Verify before parse | Yes |
| Classic refetch guidance | Yes (docs) |
| Durable dedupe | App-owned (traits only) |
| Event type catalog | Not PHP-level |
| Framework recipes | Starter docs only |

Boundary is **correct** for a client SDK.

## 19. Testing and CI assessment

- ~161 lib unit tests; wiremock contract tests; doctests; no-default-features; MSRV; deny; generation reproducibility; registry dirty check.
- Missing: property tests for amounts/cursors; fuzz on signatures; full facade HTTP matrix; blocking upstream dangerous-diff.
- Live smoke opt-in (correct).

## 20. Documentation assessment

Strong: contracts/, route-coverage, SECURITY, compatibility tiers, audits.  
Weak: PHP-scale recipes, complete guides list, stale secondary docs (current-state-audit 0.6.0).

## 21. Engineering-maturity assessment

| Inference | Confidence |
| --- | --- |
| Experienced Rust engineer / strong solo maintainer | **Strong** |
| Payments-domain awareness (idempotency, webhook honesty) | **Strong** |
| Multi-person platform team + official Mollie process | **Weak / absent** |
| Iterative hardening (0.5→0.6 transport/facades) | **Strong** |
| Some AI-assisted volume in docs/examples | **Moderate** (not a quality dismissal) |
| Official-contract generation lag | **Strong** (100 vs 124) |

**Staff-level first changes:** re-pin OpenAPI; fix upstream CI URL; HUF scale; facade injection of profile; dangerous-drift gate; OAuth endpoints.

## 22. Safe to use today

- Create/get/list payments via `PaymentsApi` + sticky idempotency keys you store.
- Refunds/captures/subscriptions/mandates/payment links with same discipline.
- Webhook HMAC verify + classic parse + **your** dedupe + **refetch**.
- Generated GETs for methods/profiles/etc. when you accept Tier G churn.
- Opt-in `RetryPolicy::default_safe` for reads and sticky-key writes.

## 23. Not trustworthy without app controls / incomplete SDK

- Double-submit protection without your idempotency store.
- Webhook-driven ledger updates without refetch/dedupe.
- Connect OAuth full lifecycle.
- BA / payouts / transfers / UCT / verify-payee / terminal pairing / sessions.
- Blind reliance on generated write types without validation.
- Multi-merchant without `with_credential` + token isolation.

## 24. Immediate fixes (P0, ≤2 weeks)

1. Point CI upstream fetch at `https://raw.githubusercontent.com/mollie/openapi/main/specs.yaml`.
2. Re-pin `specs-3.0.yaml` to 124 ops; regenerate; update examples/capabilities.
3. Fix registry meta count (100 not 101).
4. Fix `Currency::minor_units` for zero-decimal currencies.
5. Refresh stale audit claims (profile/hooks already present).
6. Document `http_client` + Authorization interaction.
7. Wire `PaymentsApi` profile default from client context.

## 25. Short-term roadmap (0.7.x)

- OAuthApi generate/revoke.
- Payouts + BA + transfers facades.
- Dangerous drift hard-fail for removed ops / auth / webhook signature changes.
- Expand ValidatedFacade registry to all money writes.
- Guides: create-payment, refund, capture, oauth-connect.

## 26. Medium-term (0.8.x)

- Sessions, pairing codes, UCT, verify-payee.
- Universal stream_pages/items.
- RequestRetryConfig per call.
- Property + fuzz suites.
- Public API diff CI.

## 27. Long-term (1.0)

- Tier S freeze; facade coverage policy; release artifact digests; supported N-1 security; recipe completeness.

## 28. Target architecture

```
Application ledger / queue / secrets manager
        │
        ▼
Tier S (stable): MollieClient, domain APIs, Money, IDs, Credential,
                 WebhookVerifier, RetryPolicy, ClientContext, RequestHook
        │
        ▼
Tier G (spec-coupled): generated routes + types from pinned OpenAPI
        │
        ▼
Transport: reqwest (+ inject), route RetryClass, retry_budget, metadata
```

## 29. Proposed module layout

```
src/
  client.rs, auth.rs, hooks.rs, integration.rs
  transport/{policy,retry,classification,rate_limit}
  domain/{payments,refunds,captures,subscriptions,mandates,
          payment_links,webhooks,oauth,payouts,accounts,transfers,...}
  money.rs, ids.rs, pagination.rs, webhook*.rs
  routes/*  # generated
  types.rs  # generated or module tree
  route_capabilities.rs  # generated
docs/registries/operation-registry.yaml
specs-3.0.yaml
```

## 30. Proposed public API examples

```rust
// Preferred write
client.with_idempotency(key).payments().create(required, None).await?;

// Multi-merchant
client.with_credential(Credential::oauth_access_token(token)?)?
      .payments().get(&payment_id).await?;

// Webhook
verifier.verify(raw, sig)?;
store.already_processed(event_id).await?;
dispatcher.enqueue_verified(event_id, raw).await?;
// worker: refetcher.refetch_payment(&id).await?;
```

## 31. Definition of production-grade (this audit)

An SDK is production-grade for a use case when:

1. Contract covers that use case's operations.
2. Writes are typed/validated or explicitly advanced.
3. Retries cannot double-charge by default.
4. Secrets and webhooks are safe by construction.
5. Errors are operable (status, request id, retryability).
6. CI prevents silent contract/security regressions.
7. Docs state app-owned duties honestly.

## 32. Definition of done (roadmap)

- 124/124 pin parity (or explicit waivers).
- OAuth + payouts + BA at least generated + tested.
- Money scale correct per currency.
- Upstream dangerous drift blocking.
- Core guides + 3 webhook recipes compile-checked.
- Tier S listed and freeze policy for 1.0-rc.
- No stale version claims across docs.

## 33. Unknowns and assumptions

- Live Mollie edge cases for each of 24 missing ops not re-tested live here.
- Whether HUF amounts in Mollie's current contract are zero-decimal string forms (verify against live docs before coding).
- Private repo visibility may change distribution/security process.
- Speakeasy SDKs may include ops slightly ahead/behind openapi main on a given day.

## 34. Final recommendation

### **Use with restrictions**

| Allowed now | Deferred |
| --- | --- |
| Core payments/refunds/captures/subscriptions/webhooks with app controls | BA, payouts, OAuth tokens, sessions, pairing, UCT, verify-payee |
| Generated reads for balances/settlements with acceptance of Tier G | Claiming full Mollie platform parity |

## Top 10 actions before public production recommendation

1. **Re-pin OpenAPI to official 124 operations and regenerate** (closes C1).
2. **Implement OAuth token generate/revoke** (closes C2).
3. **Fix upstream CI URL + dangerous-drift blocking** (H1/H2).
4. **Per-currency minor units** (H5).
5. **Application runbook:** idempotency store + webhook dedupe + refetch (C3).
6. **Facade profile injection + money write ValidatedFacade completeness**.
7. **Payouts + BA + transfers** at least generated + smoke-tested.
8. **http_client auth header merge semantics** (H4).
9. **Expand HTTP contract tests** for retries/deadlines/webhooks/profile.
10. **Version-truth + release artifact** (registry digests, no stale 0.6.0 claims).

---

*End of deep audit. Related: `docs/audits/official-sdk-parity-assessment.md`, `docs/registries/operation-registry.yaml`, `docs/production-checklist.md`.*
