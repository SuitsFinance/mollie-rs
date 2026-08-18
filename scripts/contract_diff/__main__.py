"""CLI: python -m scripts.contract_diff"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

# Allow `python -m scripts.contract_diff` from repo root
if __name__ == "__main__" and (__package__ is None or __package__ == ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[2]))

from scripts.contract_diff.diff import diff_paths
from scripts.contract_diff.report import to_json, to_markdown


def load_approvals(path: Path | None) -> set[str]:
    if path is None or not path.is_file():
        return set()
    try:
        import yaml  # type: ignore
    except ImportError:
        return set()
    data = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
    approved: set[str] = set()
    if not isinstance(data, dict):
        return approved
    for _digest, entries in data.items():
        if _digest.startswith("_"):
            continue
        if not isinstance(entries, list):
            continue
        for ent in entries:
            if isinstance(ent, dict) and "change" in ent:
                approved.add(str(ent["change"]))
            elif isinstance(ent, str):
                approved.add(ent)
    return approved


def change_key(c: dict) -> str:
    return f"{c.get('kind')}:{c.get('path')}"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Semantic OpenAPI contract diff")
    parser.add_argument("--old", required=True, help="Baseline OpenAPI YAML/JSON")
    parser.add_argument("--new", required=True, help="New OpenAPI YAML/JSON")
    parser.add_argument("--json", dest="json_out", default=None)
    parser.add_argument("--markdown", dest="md_out", default=None)
    parser.add_argument(
        "--approvals",
        default="docs/registries/approved-contract-deltas.yaml",
        help="Approved dangerous deltas registry",
    )
    parser.add_argument(
        "--fail-on-blocking",
        action="store_true",
        help="Exit 2 when unapproved blocking changes exist",
    )
    args = parser.parse_args(argv)

    report = diff_paths(args.old, args.new)
    approvals = load_approvals(Path(args.approvals)) if args.approvals else set()

    unapproved = []
    for c in report.get("blocking", []):
        keys = {change_key(c), str(c.get("kind")), f"{c.get('kind')}.{c.get('path')}"}
        if keys.isdisjoint(approvals):
            unapproved.append(c)
    report["unapproved_blocking"] = unapproved
    report["unapproved_blocking_count"] = len(unapproved)

    if args.json_out:
        to_json(report, args.json_out)
    if args.md_out:
        to_markdown(report, args.md_out)

    print(
        json.dumps(
            {
                "changes": len(report.get("changes", [])),
                "blocking": report.get("blocking_count", 0),
                "unapproved_blocking": len(unapproved),
                "max_risk": report.get("max_risk", 0),
            }
        )
    )

    if args.fail_on_blocking and unapproved:
        print("unapproved blocking contract changes:", file=sys.stderr)
        for c in unapproved:
            print(f"  - {change_key(c)} risk={c.get('risk')}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
