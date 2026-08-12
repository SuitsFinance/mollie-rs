# SDD 11 — Security (host, redirect, limits)

## Context
Auth headers on default client; provider-controlled pagination URLs; redirects.

## Problem
Cross-origin 302 can leak Authorization if client follows redirects with default headers. Malicious `next` href could point off-origin.

## Existing
- HTTPS base URL required (loopback HTTP ok)
- TLS 1.2 min
- URL query redaction in hooks
- Pagination cycle/budgets — **no origin check**
- reqwest default redirect behavior; Authorization via default_headers

## Desired
1. **Redirect policy:** `redirect::Policy::none()` on default builder **or** custom policy stripping Authorization on scheme/host/port change. Prefer fail-closed (no auto redirect) for API clients + document. Regression: mock 302 to evil host → no Auth header on second hop (or no follow).
2. **Pagination origin:** `PageCursor::from_list_link` requires same origin as client base URL (or extract `from` only after allowlist).
3. Body size limits where practical without breaking legitimate payloads.
4. cargo audit/deny in release path.

## Invariants
INV-HOST-01, INV-PAGE-01, INV-SEC-01.

## Acceptance
- [ ] Redirect regression test
- [ ] Off-origin next link rejected
- [ ] SECURITY.md / production checklist updated
