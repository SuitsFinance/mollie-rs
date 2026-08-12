# OpenAPI Generation

The checked-in route and type sources are regenerated from `specs-3.0.yaml`.

Use the repo wrapper instead of calling generator tools directly:

```sh
sh scripts/generate_openapi_client.sh
```

```powershell
powershell -ExecutionPolicy Bypass -File scripts/generate_openapi_client.ps1
```

The wrapper does three things:

- Runs the repo-owned OpenAPI generator source in `scripts/openapi_generator.rs`.
- Normalizes the raw output into `src/types.rs` and grouped `src/routes/*.rs` modules.
- Regenerates `docs/route-examples.md` and `examples/<method>.rs`.

During normalization, each type's rustdoc **JSON schema** block is fully expanded from
`specs-3.0.yaml` `components.schemas` (local `$ref` / `allOf` resolution). Cyclic
references keep a `$ref` edge to avoid infinite expansion.

To re-expand schemas in the current `src/types.rs` without a full regenerate:

```sh
python scripts/generate_openapi_client.py --expand-doc-schemas-only
```

The normalizer keeps repeated generated plumbing out of each route method:

- `Client::endpoint` owns base URL joining.
- `Client::request` owns common headers and idempotency handling. The OpenAPI
  `idempotency-key` header parameter is stripped from method signatures; the key
  is client state (`Client::with_idempotency_key` / default auto UUID v4).
  `Client::request` always sends `Idempotency-Key` and returns the resolved key
  for the response envelope.
- The OpenAPI `testmode` query parameter is also stripped from method signatures;
  sticky mode is client state (`Client::with_testmode`). Generated routes that
  document the param bind `QueryParam::new("testmode", &self.testmode())`.
  Routes without that parameter do not receive the sticky query. The complete
  route-level behavior, including live-only reporting routes and request-body
  `testmode` fields, is documented in [`contracts/test-mode.md`](contracts/test-mode.md).
- `routes::Operation` owns operation ids.
- `routes::response::json` owns documented status handling and attaches the
  resolved idempotency key to the response headers/`ResponseEnvelope`.

Run these checks after regeneration:

```sh
sh scripts/check_route_examples.sh
python scripts/check_generation_reproducibility.py
python scripts/report_api_drift.py --write docs/api-drift-report.md
cargo fmt --all -- --check
cargo test --all-targets
cargo test --doc
cargo clippy --all-targets --all-features -- -D warnings
```

### Reproducibility and drift

| Script | Purpose |
| ------ | ------- |
| `scripts/check_generation_reproducibility.py` | Asserts `src/route_capabilities.rs` matches every `operationId` in `specs-3.0.yaml`. |
| `scripts/report_api_drift.py` | Writes `docs/api-drift-report.md` local inventory; optional `--upstream` comparison. |

Do **not** auto-publish regenerations from upstream drift. Review Tier G fallout per [`compatibility.md`](compatibility.md).

On this Windows workspace, run Cargo test and clippy through WSL when the MSVC toolchain is unavailable for a target.
