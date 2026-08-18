# Transport security policy (safe builder path)

Applies to [`MollieClientBuilder`](../../src/client.rs) / [`MollieClient`](../../src/client.rs).  
Low-level escape: [`Client::new_with_client`](../../src/lib.rs) and [`MollieClient::from_generated`](../../src/client.rs) — no automatic redirect/TLS last-apply.

## Invariants (INV-HTTP-01 / INV-HOST-01)

| Control | Safe builder behavior |
| --- | --- |
| Redirects | Always `reqwest::redirect::Policy::none()` **after** `configure_http` |
| TLS | Always `min_tls_version(TLS_1_2)` after `configure_http`; stack is **rustls** (`reqwest` features `rustls-tls`, no default features / no gzip) |
| Auth header | Builder always inserts `Authorization` from configured `Credential` last among default headers |
| Timeouts | Builder `timeout` + `connect_timeout` always applied last |
| Base URL | Non-loopback hosts must be `https://`; loopback/`localhost` may use `http://` for mock servers |
| Response bodies | [`ResponseLimits`](../../src/response_limits.rs) cap success JSON (default 8 MiB) and error bodies (default 64 KiB); webhook verifier default 1 MiB |
| Compression | No `gzip`/`brotli` reqwest features enabled on this crate — provider compression is not auto-decoded by the SDK feature set |

## Explicit non-goals on the safe path

- Callers cannot inject a prebuilt `reqwest::Client` via the builder (removed).
- `configure_http` may set proxies, custom roots, or extra middleware **before** security last-apply; it cannot keep redirects enabled or drop the final TLS floor on the safe path.
- Fully custom transports (including enabling redirects) must use `from_generated` and accept operational ownership.

## Proxy guidance

- Prefer explicit `configure_http(|b| b.proxy(...))` or `configure_http(|b| b.no_proxy())` over ambient `HTTP(S)_PROXY` when credentials or isolation matter.
- Do not put Mollie API keys into proxy URLs; use proxy-specific credentials only.
- `MollieClientBuilder` `Debug` records only that `configure_http` was set — not the closure body or proxy userinfo (tested).
- SDK does not log `Authorization` or proxy passwords (see secret-leak tests).
- Safe path still last-applies redirect-none + TLS 1.2+ after proxy configuration.

## Evidence commands

```text
cargo test --lib --all-features configure_http_cannot_reenable
cargo test --lib --all-features configure_http_proxy_userinfo
cargo test --lib --all-features configure_http_no_proxy
cargo test --lib --all-features build_rejects_remote_http
cargo test --lib --all-features build_allows_loopback_http
cargo test --lib --all-features accepts_body_exactly
cargo test --lib --all-features rejects_body_one_byte
cargo test --lib --all-features rejects_declared_content
cargo test --lib --all-features json_uses_error_body
cargo deny check
cargo tree -e features -i reqwest
```
