# Phase 0–1 implementation report (continuation from 0.6.1)

**HEAD base:** `10ec88e`  
**Work:** baseline freeze + contract pipeline + transport credential fix  

## 1. Baseline state

* Version `0.6.1`, 100 local ops, 124 upstream ops, 24 gaps  
* 161 lib tests → **163** after this tranche  
* Generation reproducibility OK  

## 2. Phases completed

| Phase | Status |
| --- | --- |
| 0 Baseline freeze | **Done** — `docs/audits/implementation-baseline-0.6.1.md` |
| 1 Contract pipeline | **Done** — pin, fetch, compare, CI, docs |
| 1b High-risk fixes | **Partial** — credential timeouts preserved; money scale table explicit |
| 2 OpenAPI re-pin / regen | **Not done** (separate reviewable 0.7 PR) |
| 3 Facades for missing ops | **Not done** (depends on Phase 2) |

## 3. Phases not completed

* Full Tier G regeneration for 24 ops  
* OAuth/payouts/BA facades  
* Request-level RetryConfig  
* Universal streams  
* 0.7.0 release cut  

## 4. Files changed (this tranche)

* `specs/upstream-pin.toml`  
* `scripts/fetch_upstream_openapi.py`  
* `scripts/compare_upstream_openapi.py`  
* `.github/workflows/ci.yml`  
* `.gitignore`  
* `src/lib.rs`, `src/client.rs`, `src/money.rs`  
* `docs/audits/implementation-baseline-0.6.1.md`  
* `docs/audits/phase-0-1-implementation-report.md`  
* `docs/contracts/openapi-pipeline.md`  
* `docs/contracts/operation-coverage.md`  
* `docs/contracts/route-capabilities.md`  
* `docs/registries/operation-registry.yaml` (export refresh)  

## 5. Public API changes

* Additive: `Client::timeout`, `connect_timeout`, `user_agent`  
* `with_credential` now preserves timeouts and user-agent (behavior fix)  
* `Currency::minor_units` is an explicit match table (still 2 for all pinned currencies)  

## 6–9. Counts

| Metric | Before | After |
| --- | --- | --- |
| Local ops | 100 | 100 |
| Upstream ops (pinned) | ~124 claimed | **124 verified + sha256** |
| Facades | 7 | 7 |
| Lib tests | 161 | **163** |

## 10–11. CI / commands

| Command | Result |
| --- | --- |
| `check_generation_reproducibility.py` | OK |
| `fetch_upstream_openapi.py` | OK digest |
| `compare_upstream_openapi.py` | Exit 2 (24 missing — expected) |
| `cargo test --lib --all-features` | **163 passed** |

## 12–15. Findings

* Security: no new secret exposure; UA stored as non-secret string  
* Transport: credential rebuild no longer drops timeouts  
* Parity: still 24 ops short; pipeline now blocks wrong upstream digest  
* Migration: none required for consumers  

## 16. Release recommendation

* **Do not ship 0.7.0 yet.**  
* Ship this as **0.6.2** (docs + pipeline + small additive API) or fold into next minor with re-pin.  

## 17–18. Blockers / next slices

1. Adapt upstream OpenAPI paths and re-pin `specs-3.0.yaml`  
2. Regenerate Tier G + capabilities + examples  
3. OAuth + payouts + BA facades + wiremock  

## 19. Production-readiness score

Still **~72/100** (pipeline improved; contract surface unchanged).

## 20. Is 0.7.0 ready?

| Area | Status | Evidence | Blocking? |
| --- | --- | --- | --- |
| Contract parity | Incomplete | 100/124 | **Yes for 0.7 claim** |
| Generated client | Current pin OK | 100 ops | No for 0.6.x |
| Domain facades | Core only | 7 modules | Yes for full platform |
| Authentication | Strong + credential rebuild fix | tests | No |
| Idempotency / retries | Strong | existing | No |
| Webhooks | Strong verify | existing | No |
| CI/release | Upstream pin fixed | ci.yml | Improved |
| **0.7.0 ready?** | **No** | re-pin not done | — |
