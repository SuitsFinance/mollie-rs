# SDD 14 — Release plan

## Ladder

```text
0.7.x  truth + transport kernel + OperationSafetyProfile + drift CI
  → 0.8.x  Connect + OAuth + money facades (public)
  → 0.9/rc assurance + docs + sandbox soak
  → 1.0    only if 01-requirements gates pass
```

## Checklist (every release candidate)

- [ ] cargo fmt --check
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all-features (default tests; no live money)
- [ ] cargo test --doc / examples build as configured in CI
- [ ] MSRV toolchain job green
- [ ] cargo deny / audit as configured
- [ ] python scripts/check_generation_reproducibility.py
- [ ] python scripts/check_dangerous_profile_drift.py
- [ ] Registry committed (export_operation_registry.py)
- [ ] Secret leak tests green
- [ ] cargo package dry-run (or CI package job)
- [ ] Changelog entry
- [ ] docs/release-readiness.md band still honest

## Primary metric

Report in docs/release-readiness.md:

```text
high_risk_ops_with_enforced_safety_invariants / total_high_risk_ops
```

Current program target after Phase 7: **16/16** profile-enforced high-risk writes (not facade-count vanity).

## 1.0 stop conditions

Do **not** ship 1.0 if any of:

- Dangerous profile drift CI disabled or failing
- Financial write auto-retry without sticky key possible
- Auth forwarded to arbitrary redirect/pagination hosts
- Default test suite can create live payouts/transfers
- Honest band still NOT READY / open P0 in FINDINGS
