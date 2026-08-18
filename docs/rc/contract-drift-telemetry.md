# Contract drift telemetry (TEL-001 / SUI-2366)

## Goal

Give applications an **opt-in**, **redacted**, **non-panicking** signal when the
SDK observes soft provider-contract surprises at runtime — without changing
decode success semantics.

## API

| Surface | Role |
| --- | --- |
| `ContractDriftKind` | `UnknownEnumValue`, `OffOriginPaginationLink` |
| `ContractDriftSignal` | kind + optional operation + field_path + truncated detail |
| `ContractDriftObserver` | `on_drift(&ContractDriftSignal)` |
| `MollieClientBuilder::contract_drift_observer` | preferred multi-tenant attach |
| `MollieClient::with_contract_drift_observer` | attach on existing client |
| `set_global_contract_drift_observer` | process fallback (tests / single-tenant) |
| `emit_contract_drift` | manual emit (advanced) |

## Emission points

1. **`OpenEnum::parse_str`** — when the wire string does not map to a known
   variant (still decodes successfully; raw preserved).
2. **`PageCursor::from_list_link_for_base`** — when a HAL `next` href is rejected
   for origin policy (cursor becomes `None`; walk ends safely).

During each HTTP attempt the transport installs a
`ContractDriftScopeGuard` so signals carry the active operation id and prefer
the client-scoped observer over the global fallback.

## Safety

- Observer panics are caught (`catch_unwind`); the SDK path continues.
- Detail strings are length-capped (`CONTRACT_DRIFT_DETAIL_MAX_LEN`) and
  secret-looking substrings (`Bearer `, `live_`, `test_`, `access_token`) become
  `<redacted>`.
- No Authorization headers, bodies, or webhook secrets are passed to observers.

## Tests

See `src/contract_drift.rs` unit tests: truncation, redaction, global emit,
panic isolation, request-scope preference.
