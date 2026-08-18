#!/usr/bin/env python3
"""Validate Tier-S request contract allowlist registry structure (INV-TIER-02 seed)."""

from __future__ import annotations

import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
REG = ROOT / "docs" / "registries" / "tier-s-request-contracts.yaml"
WRITE_REQUESTS = ROOT / "src" / "write_requests.rs"
CREATE_PAYMENT = ROOT / "src" / "create_payment.rs"

REQUIRED_OPS = {
    "create_payment",
    "create_refund",
    "create_capture",
    "create_payout",
    "create_transfer",
    "create_connect_balance_transfer",
}


def main() -> int:
    data = yaml.safe_load(REG.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        print("registry must be a mapping", file=sys.stderr)
        return 1
    errors: list[str] = []
    ops = {k: v for k, v in data.items() if not str(k).startswith("_")}
    missing = sorted(REQUIRED_OPS - set(ops))
    if missing:
        errors.append(f"missing required ops: {missing}")
    sources = WRITE_REQUESTS.read_text(encoding="utf-8") + CREATE_PAYMENT.read_text(encoding="utf-8")
    for oid, body in ops.items():
        if not isinstance(body, dict):
            errors.append(f"{oid}: entry must be mapping")
            continue
        rust_type = body.get("rust_type")
        fields = body.get("fields")
        if not rust_type or not isinstance(rust_type, str):
            errors.append(f"{oid}: rust_type required")
        elif rust_type not in sources:
            errors.append(f"{oid}: rust_type {rust_type} not found in builders")
        if not isinstance(fields, list) or not fields:
            errors.append(f"{oid}: fields allowlist required")
    if errors:
        print("check_tier_s_request_contracts FAILED:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    print(f"tier-s request contracts OK ({len(ops)} ops)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
