# SDD 04 — Business accounts & transfers

## Context
BA accounts/transactions (read) and transfers (write + signing headers already partially wired).

## Desired
Read facades for accounts/txns; transfer create with validated amount/payee fields where contract allows; sticky idempotency; kernel delivery.
**After kernel + preferably after payouts pattern.**

## Invariants
INV-WRITE-*, INV-MONEY-01, INV-HOST-01 (no open redirects).

## Acceptance
- [ ] Justified Tier-S or explicit “Tier G sufficient for reads”
- [ ] Transfer write mock tests + signing headers regression
