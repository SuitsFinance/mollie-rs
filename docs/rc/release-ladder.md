# Release ladder (`0.8.x` → `1.0.0-rc.1`)

Do **not** tag `1.0.0-rc.1` until every **required** row in
[`1.0-acceptance-matrix.md`](1.0-acceptance-matrix.md) is `pass`.

## Current crate line

| Tag / version | Intent |
| --- | --- |
| `0.8.0` (this branch) | Hardening: builder inject removed, body limits, Tier-S gates, pagination streams, drift telemetry, workflow matrix |
| `0.8.x` patches | Fix residuals that do not claim RC |
| `1.0.0-rc.1` | First honest RC after live Tier 1 + Tier 2 paste + green Actions |

## Gates before `1.0.0-rc.1`

1. GitHub Actions green on the release branch (contracts + quality + semver).
2. `docs/rc/open-residuals.md` has no **P0** open items.
3. Required acceptance-matrix rows all `pass` (today: only credentialed live rows remain `not_run`).
4. Operator pastes Tier 1 + Tier 2 into [`live-assurance-evidence.md`](live-assurance-evidence.md) (no secrets in git).
5. Human release owner sign-off on [`hostile-security-review.md`](hostile-security-review.md).
6. `cargo package` / publish dry-run clean; CHANGELOG section frozen for the tag.

## Explicit non-goals at RC

- Marketplace or Mollie product certification
- Expanding operation count as a success metric
- Redesign of Tier-G / `OperationSafetyProfile` architecture

## After RC

- `1.0.0-rc.N` for blocking fixes only
- `1.0.0` when RC soak is accepted and no P0/P1 blockers remain in `open-residuals.md`
