# Pagination

- Mollie uses opaque `from` cursors + `limit` (max 250).
- Always pass `PaginationGuard` budgets for `list_all` / streams.
- Next-link **origin allowlist** blocks foreign hosts.
- Cycle detection rejects repeating cursors.
- Prefer `list_page` for UIs; streams for bounded exports.
