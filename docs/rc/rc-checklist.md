# RC checklist — mollie-rs

Track toward **RC READY**. Evidence links preferred over checkmarks alone.

Baseline: [`baseline.md`](baseline.md) · SDD-15.

## Freeze / static

- [x] HEAD recorded (`df6a9d434b56f5663aac1003f94db0e0e028b587`)
- [x] OpenAPI 124/124
- [x] high-risk profile coverage 100% (16/16)
- [x] cargo fmt green
- [x] clippy green (all-features + no-default lib/tests)
- [x] default tests green
- [x] no-default tests green
- [ ] MSRV green (CI job exists; local rustup 1.88 re-run optional)
- [x] docs tests green (`cargo test --doc`)
- [x] cargo deny green (local `cargo deny check`)
- [ ] cargo audit reviewed
- [ ] semver diff reviewed (`docs/rc/public-api-diff.md`)
- [x] examples compile gate in CI (`cargo check --examples --all-features` in `check` job)
- [x] fuzz targets listed / CI fuzz-build present
- [ ] hostile transport tests green
- [x] live readonly suite **implemented** (13 ops; needs credentialed run for green evidence)
- [x] sandbox payment smoke **implemented** (multi-gate; needs credentialed run)
- [ ] sandbox refund smoke green or provider limitation documented
- [ ] payout/transfer sandbox state documented
- [ ] webhook framework examples verified
- [x] package dry-run green (`cargo package --allow-dirty --no-verify`)
- [ ] packed crate downstream compilation green
- [ ] hostile security review signed (`docs/rc/hostile-security-review.md`)
- [ ] P0 = 0
- [ ] P1 = 0 or accepted explicitly
- [ ] release-readiness updated for RC claim

## Documents still required

- [x] `docs/rc/live-test-matrix.md`
- [ ] `docs/rc/hostile-security-review.md`
- [ ] `docs/rc/performance.md`
- [ ] `docs/rc/package-audit.md`
- [ ] `docs/rc/public-api-diff.md`
- [x] `docs/rc/baseline.md`
- [x] `docs/rc/rc-checklist.md`
