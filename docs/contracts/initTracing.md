# initTracing

## Summary
`init_tracing` installs a global `tracing-subscriber` fmt layer so SDK and application `tracing` events are emitted. Log level is controlled by `RUST_LOG`.

## Symbol
- Name: `init_tracing`
- Kind: `function`
- Owner: `mollie_rs::tracing_config`

## Signature
```rust
pub fn init_tracing() -> MollieResult<()>
pub fn init_tracing_with_filter(filter: impl AsRef<str>) -> MollieResult<()>
pub fn try_init_tracing() -> bool
pub fn try_init_tracing_with_filter(filter: impl AsRef<str>) -> bool
```

## Location
- `src/tracing_config.rs`

## Inputs
- `init_tracing` uses `RUST_LOG` via `EnvFilter::try_from_default_env`, defaulting to `info` when unset.
- `init_tracing_with_filter` takes an explicit filter directive string.

## Returns
- `Ok(())` when the global subscriber is installed.
- `try_init_*` returns `true` when this call installed the subscriber, `false` otherwise (already initialized or install failure).

## Errors
- Returns `MollieError::InvalidConfiguration` when a global subscriber is already installed or the filter cannot be parsed.

## Side Effects
- Installs a process-global tracing subscriber (once).

## Guarantees
- Uses `try_init` so double installation does not panic.
- Does not log credentials.

## Source of Truth
- Implementation: `src/tracing_config.rs`
- Public exports: `src/lib.rs`
