# Upstream Spec Pin

This directory holds the **upstream drift baseline** — a snapshot of Mollie's
own OpenAPI document plus the digest we compare against on every CI run.

| File | Role |
| --- | --- |
| `upstream-pin.toml` | The pin: upstream URL, ref, SHA-256 digest, operation count, and the policy exit codes. |
| `upstream-openapi.yaml` | The fetched snapshot the digest refers to. |

## This is not the generation input

Two different specs live in this repository and they are easy to confuse:

| Path | What it is |
| --- | --- |
| `specs/upstream-openapi.yaml` | Mollie's upstream OpenAPI **3.1** document. Read-only reference for drift detection. |
| `/specs.yaml`, `/specs-3.0.yaml` (repo root) | The adapted **3.0.3** spec that actually generates `src/types.rs`, `src/routes/`, and `src/route_capabilities.rs`. |

The root specs are derived from the upstream snapshot by
`scripts/adapt_upstream_openapi.py`. Never hand-edit either one; regenerate.

> Both spec documents are vendored from Mollie's OpenAPI repository, which is
> licensed **CC BY-NC-SA 4.0** — a different licence from this crate's code. See
> [`../NOTICE`](../NOTICE) before redistributing them.

## Drift pipeline

```sh
python scripts/fetch_upstream_openapi.py     # refresh snapshot + digest
python scripts/compare_upstream_openapi.py   # compare upstream vs local
python scripts/report_api_drift.py           # human-readable drift report
```

`compare_upstream_openapi.py` exit codes (declared in `[policy]`):

| Code | Meaning | CI effect |
| --- | --- | --- |
| `0` | Inventory OK, upstream matches the pin | pass |
| `1` | Local pin, capabilities, or registry are inconsistent with each other | fail |
| `2` | Upstream has changed relative to local | advisory in PR CI; blocks release |
| `3` | Upstream digest does not match this pin | always blocking |

Exit `1` means *we* are internally inconsistent — the pin, `src/route_capabilities.rs`,
and `docs/registries/operation-registry.yaml` disagree on the operation set. Exit
`2` or `3` means *Mollie* moved.

## Re-pinning

Bumping the pin is a deliberate, reviewable act, not a routine refresh:

1. `python scripts/fetch_upstream_openapi.py` — updates the snapshot and digest.
2. `python scripts/adapt_upstream_openapi.py` — regenerates the root 3.0.3 spec.
3. `sh scripts/generate_openapi_client.sh` — regenerates client, capabilities, examples.
4. Update `operation_count`, `pinned_date`, and `notes` in `upstream-pin.toml`.
5. Review the resulting diff for removed or renamed operations — those are
   breaking changes for downstream users.

Python dependencies for these scripts: `pip install -r scripts/requirements.txt`.
