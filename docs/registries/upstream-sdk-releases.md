# Upstream SDK release canary

Official Mollie language SDKs are **informational canaries only**. They are **not** OpenAPI/codegen authority for `mollie-rs`.

| Artifact | Role |
| --- | --- |
| `docs/registries/upstream-sdk-releases.json` | Committed **last-reviewed** tags (baseline) |
| `.github/workflows/upstream-canary.yml` | Weekly probe + issue queue |
| `scripts/upstream_sdk_canary.py` | Fetch, diff, warn, optional issue create |
| `docs/generated/upstream-canary.json` | Ephemeral run snapshot (gitignored; CI artifact) |

## Behavior

1. Fetch latest release tags for the configured SDK repos.
2. Diff against the committed baseline.
3. No changes -> exit cleanly.
4. Tag change -> GitHub Actions warning + **one issue per SDK** (`upstream-drift`), deduped by `repo` + `new_tag` in title (any issue state).
5. **Never** auto-commit baseline bumps. After human review, update the baseline in a PR.

## Manual run

```bash
python scripts/upstream_sdk_canary.py
# CI only:
python scripts/upstream_sdk_canary.py --create-issues
```
