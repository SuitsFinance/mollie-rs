#!/usr/bin/env python3
"""Run miniature openapi-drift fixture pairs against contract_diff."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts.contract_diff.diff import diff_paths  # noqa: E402

FIXTURES = ROOT / "tests" / "fixtures" / "openapi-drift"

# Expected dominant kind per fixture directory name
EXPECT: dict[str, str] = {
    "enum-add": "AdditiveResponseEnum",
    "enum-remove": "EnumValueRemoved",
    "nullable-add": "NullableRelaxation",
    "nullable-remove": "NullableRestriction",
    "required-add": "RequirednessChange",
    "schema-rename": "SchemaReplacement",
    "endpoint-add-get": "OperationAdded",
    "endpoint-add-post": "MutationAdded",
    "endpoint-remove": "OperationRemoved",
    "error-403-add": "ErrorContractChange",
    "testmode-add": "TestmodeChange",
    "auth-change": "AuthChange",
    "idempotency-change": "IdempotencyChange",
    "money-type-change": "MoneyChange",
    "beta-to-ga": "MaturityChange",
    "stable-to-beta": "MaturityChange",
    "draft-transfers-remove": "OperationRemoved",
}


def main() -> int:
    if not FIXTURES.is_dir():
        print(f"missing fixtures dir {FIXTURES}", file=sys.stderr)
        return 1
    failed = 0
    checked = 0
    for child in sorted(FIXTURES.iterdir()):
        if not child.is_dir():
            continue
        before, after = child / "before.yaml", child / "after.yaml"
        if not before.is_file() or not after.is_file():
            print(f"SKIP {child.name}: missing before/after")
            continue
        expect = EXPECT.get(child.name)
        report = diff_paths(str(before), str(after))
        kinds = {c.get("kind") for c in report.get("changes", [])}
        checked += 1
        ok = expect is None or expect in kinds
        status = "OK" if ok else "FAIL"
        print(f"{status} {child.name}: kinds={sorted(kinds)} max_risk={report.get('max_risk')}")
        if not ok:
            failed += 1
            print(json.dumps(report.get("changes"), indent=2)[:2000])
    if checked == 0:
        print("no fixtures checked", file=sys.stderr)
        return 1
    if failed:
        print(f"{failed}/{checked} fixture suites failed", file=sys.stderr)
        return 1
    print(f"all {checked} openapi-drift fixture suites passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
