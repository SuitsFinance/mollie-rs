# SDD 07 — Webhooks

## Existing
HMAC next-gen verify raw body; classic form parser; rotation secrets; guide refetch.

## Desired
Adversarial suite; fuzz parsers; Axum/Actix raw-body examples; never recommend parse-before-verify.

## Invariants
INV-WH-01, INV-WH-02, INV-SEC-01.

## Acceptance
- [ ] Fuzz or property on signature header parsing
- [ ] Framework examples
