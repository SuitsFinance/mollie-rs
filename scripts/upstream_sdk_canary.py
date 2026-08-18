#!/usr/bin/env python3
"""Official Mollie SDK release canary (informational).

Compares latest GitHub release tags against the committed baseline
`docs/registries/upstream-sdk-releases.json`.

- Does NOT treat official SDKs as codegen sources.
- Does NOT auto-update the committed baseline.
- Emits GitHub Actions warnings on tag changes.
- Optionally creates one GitHub issue per changed SDK (deduped by repo + new tag).

Usage:
  python scripts/upstream_sdk_canary.py
  python scripts/upstream_sdk_canary.py --create-issues
  python scripts/upstream_sdk_canary.py --snapshot path.json
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BASELINE = ROOT / "docs" / "registries" / "upstream-sdk-releases.json"

REPOS = [
    "mollie/mollie-api-node",
    "mollie/mollie-api-golang",
    "mollie/mollie-api-python",
    "mollie/mollie-api-java",
    "mollie/mollie-api-csharp",
    "mollie/mollie-api-php",
]


def _headers() -> dict[str, str]:
    headers = {
        "Accept": "application/vnd.github+json",
        "User-Agent": "mollie-rs-upstream-canary",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def fetch_latest(repo: str) -> dict[str, Any]:
    url = f"https://api.github.com/repos/{repo}/releases/latest"
    req = urllib.request.Request(url, headers=_headers())
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = json.load(resp)
        return {
            "tag": data.get("tag_name"),
            "published_at": data.get("published_at"),
            "html_url": data.get("html_url"),
        }
    except urllib.error.HTTPError as exc:
        return {"error": f"HTTP {exc.code}: {exc.reason}"}
    except Exception as exc:  # noqa: BLE001 - canary must not crash the job on one repo
        return {"error": f"{type(exc).__name__}: {exc}"}


def load_baseline() -> dict[str, Any]:
    if not BASELINE.is_file():
        raise SystemExit(f"missing baseline: {BASELINE}")
    data = json.loads(BASELINE.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or "sdks" not in data:
        raise SystemExit("baseline must be an object with top-level 'sdks'")
    return data


def baseline_tag(entry: Any) -> str | None:
    if entry is None:
        return None
    if isinstance(entry, str):
        return entry
    if isinstance(entry, dict):
        tag = entry.get("tag")
        return tag if isinstance(tag, str) else None
    return None


def github_actions() -> bool:
    return os.environ.get("GITHUB_ACTIONS") == "true"


def emit_warning(message: str) -> None:
    if github_actions():
        # Restrict newlines for workflow command safety
        safe = message.replace("\n", "%0A")
        print(f"::warning::{safe}")
    else:
        print(f"WARNING: {message}", file=sys.stderr)


def gh_json(args: list[str]) -> Any:
    proc = subprocess.run(
        ["gh", *args],
        check=False,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or "gh failed")
    return json.loads(proc.stdout) if proc.stdout.strip() else None


def issue_exists(repo_full: str, sdk_repo: str, new_tag: str) -> bool:
    """Deduplicate by upstream repo + new tag appearing in issue title (any state)."""
    # gh search: quoted tokens in title
    query = f'"{sdk_repo}" "{new_tag}" in:title'
    data = gh_json(
        [
            "issue",
            "list",
            "--repo",
            repo_full,
            "--state",
            "all",
            "--search",
            query,
            "--json",
            "number,title",
            "--limit",
            "20",
        ]
    )
    if not data:
        return False
    needle_repo = sdk_repo.lower()
    needle_tag = new_tag.lower()
    for item in data:
        title = (item.get("title") or "").lower()
        if needle_repo in title and needle_tag in title:
            return True
    return False


def create_issue(
    repo_full: str,
    sdk_repo: str,
    old_tag: str | None,
    new_tag: str,
    release_url: str | None,
) -> None:
    old_disp = old_tag or "(none)"
    title = f"Upstream SDK update: {sdk_repo} {old_disp} -> {new_tag}"
    body = f"""An official Mollie SDK has published a new release.

- SDK: `{sdk_repo}`
- Previous (committed baseline): `{old_disp}`
- Current: `{new_tag}`
- Release: {release_url or "(unknown)"}

This SDK is **informational only** and is **not** a code-generation source for mollie-rs.
OpenAPI pin + local generators remain the contract authority.

Review the upstream release for potentially relevant changes to:

- API models and fields
- request/response behavior
- pagination
- idempotency
- error handling
- webhook behavior
- endpoint additions or removals
- serialization edge cases

If no mollie-rs changes are required, close this issue with the findings.

After review, update `docs/registries/upstream-sdk-releases.json` for `{sdk_repo}` to `{new_tag}` in a follow-up PR (the canary workflow never auto-commits baseline bumps).
"""
    cmd = [
        "gh",
        "issue",
        "create",
        "--repo",
        repo_full,
        "--title",
        title,
        "--body",
        body,
        "--label",
        "upstream-drift",
    ]
    proc = subprocess.run(cmd, check=False, capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or "gh issue create failed")
    print(f"created issue: {proc.stdout.strip()}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--create-issues",
        action="store_true",
        help="Create one GitHub issue per newly released SDK (deduped)",
    )
    ap.add_argument(
        "--snapshot",
        default="docs/generated/upstream-canary.json",
        help="Write ephemeral run snapshot JSON (not the committed baseline)",
    )
    ap.add_argument(
        "--repo",
        default=os.environ.get("GITHUB_REPOSITORY", "SuitsFinance/mollie-rs"),
        help="GitHub repo for issue create/list (owner/name)",
    )
    args = ap.parse_args()

    baseline = load_baseline()
    baseline_sdks = baseline.get("sdks") or {}

    current: dict[str, Any] = {}
    changes: list[dict[str, Any]] = []
    errors: list[str] = []

    for repo in REPOS:
        latest = fetch_latest(repo)
        current[repo] = latest
        if "error" in latest:
            errors.append(f"{repo}: {latest['error']}")
            emit_warning(f"upstream canary fetch failed for {repo}: {latest['error']}")
            continue

        new_tag = latest.get("tag")
        old_tag = baseline_tag(baseline_sdks.get(repo))
        if not isinstance(new_tag, str) or not new_tag:
            errors.append(f"{repo}: missing tag in latest release payload")
            continue
        if old_tag != new_tag:
            change = {
                "repo": repo,
                "old_tag": old_tag,
                "new_tag": new_tag,
                "html_url": latest.get("html_url"),
            }
            changes.append(change)
            emit_warning(
                f"upstream SDK release changed: {repo} {old_tag or '(none)'} -> {new_tag}"
            )

    snapshot = {
        "schema_version": 1,
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "informational_only": True,
        "baseline_path": str(BASELINE.relative_to(ROOT)).replace("\\", "/"),
        "baseline_not_auto_updated": True,
        "sdks": current,
        "changes": changes,
        "errors": errors,
    }

    snap_path = Path(args.snapshot)
    if not snap_path.is_absolute():
        snap_path = ROOT / snap_path
    snap_path.parent.mkdir(parents=True, exist_ok=True)
    snap_path.write_text(json.dumps(snapshot, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(snapshot, indent=2, sort_keys=True))

    if args.create_issues:
        for ch in changes:
            sdk = ch["repo"]
            new_tag = ch["new_tag"]
            old_tag = ch.get("old_tag")
            try:
                if issue_exists(args.repo, sdk, new_tag):
                    print(f"skip issue (already exists): {sdk} {new_tag}")
                    continue
                create_issue(
                    args.repo,
                    sdk,
                    old_tag if isinstance(old_tag, str) else None,
                    new_tag,
                    ch.get("html_url"),
                )
            except Exception as exc:  # noqa: BLE001
                emit_warning(f"failed to create/dedupe issue for {sdk}: {exc}")
                errors.append(f"issue:{sdk}: {exc}")

    # Informational canary: never fail the job solely because tags moved.
    # Fail only if every probe errored (total outage / auth).
    if len(errors) >= len(REPOS):
        print("all upstream SDK probes failed", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
