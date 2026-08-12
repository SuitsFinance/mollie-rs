# Operation coverage

| Source | Operations | As of |
| --- | --- | --- |
| Local pin `specs-3.0.yaml` | **124** | Phase 2 re-pin |
| Upstream pin `specs/upstream-pin.toml` | **124** | 2026-08-04 |
| Missing from local | **0** | full Tier G parity |

Machine-readable inventory:

* `docs/registries/operation-registry.yaml`
* `src/route_capabilities.rs`
* Live compare: `python scripts/compare_upstream_openapi.py --require-upstream`

## Formerly missing groups (now generated)

| Group | Ops (approx) | Status |
| --- | --- | --- |
| Business accounts + transactions | 4 | Generated-only |
| BA transfers | 2 | Generated-only (signing headers wired) |
| Payouts | 4 | Generated-only |
| Sessions | 2 | Generated-only |
| Unmatched credit transfers | 4 | Generated-only |
| Verify payee | 1 | Generated-only |
| OAuth tokens (`/oauth2/tokens`) | 2 | Generated-only (path rewrite off `/v2`) |
| Terminal pairing codes | 4 | Generated-only |
| Payment get route | 1 | Generated-only |

## Facade coverage (local)

| Domain | Generated | Tier S facade |
| --- | --- | --- |
| Payments | Y | Y |
| Refunds | Y | Y |
| Captures | Y | Y |
| Subscriptions | Y | Y |
| Mandates | Y | Y |
| Payment links | Y | Y |
| Webhooks | Y | Y |
| All other local groups (incl. new) | Y | Generated-only |

See `docs/contracts/openapi-pipeline.md` for regeneration policy and
`docs/audits/openapi-repin-0.7.0.md` for the Phase 2 re-pin report.
