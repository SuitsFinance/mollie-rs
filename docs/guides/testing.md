# Testing with mollie-rs

- Unit: builders, profiles, `simulate_retry_loop`, secret-leak suite.
- Integration: WireMock HTTP contract tests.
- Live: ignored smoke tests behind env flags.
- Fuzz: webhook, money, page cursor, retry-after targets under `fuzz/`.
- Prefer Tier-S in app tests; use Tier-G only for contract edge cases.
