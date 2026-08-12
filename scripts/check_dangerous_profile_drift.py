#!/usr/bin/env python3
"""Fail CI on dangerous OperationSafetyProfile / capability drift.

INV-DRIFT-01 / INV-PROFILE-01: high-risk write profiles must stay consistent
with transport expectations. This gate does not fetch upstream OpenAPI; it
validates the checked-in SSOT table in `src/route_capabilities.rs`.

Exit codes:
  0 — invariants hold
  1 — dangerous drift or parse failure
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAPS = ROOT / "src" / "route_capabilities.rs"

# High-risk operations that must remain profiled and write-classified.
# Frozen denominator for INV-TIER-01 / 1.0 high-risk coverage metric.
HIGH_RISK_WRITES = {
    "create_payment",
    "cancel_payment",
    "create_refund",
    "cancel_refund",
    "create_capture",
    "create_subscription",
    "cancel_subscription",
    "create_mandate",
    "create_payment_link",
    "create_customer_payment",
    "create_payout",
    "cancel_payout",
    "create_transfer",
    "create_connect_balance_transfer",
    "verify_payee",
    "oauth_generate_tokens",
    "oauth_revoke_tokens",
    "payment_create_route",
    "create_session",
    "terminals_request_pairing_code",
    "terminals_revoke_pairing_code",
    "match_unmatched_credit_transfer",
    "return_unmatched_credit_transfer",
}


def parse_capabilities(text: str) -> list[dict]:
    ops: list[dict] = []
    for block in text.split("RouteCapability {")[1:]:

        def field(name: str):
            m = re.search(rf'{name}:\s*"([^"]*)"', block)
            if m:
                return m.group(1)
            m = re.search(rf"{name}:\s*(true|false)", block)
            if m:
                return m.group(1) == "true"
            m = re.search(rf"{name}:\s*RetryClass::(\w+)", block)
            if m:
                return m.group(1)
            m = re.search(rf"{name}:\s*RouteAccess::(\w+)", block)
            if m:
                return m.group(1)
            return None

        op_id = field("operation_id")
        if not op_id:
            continue
        ops.append(
            {
                "operation_id": op_id,
                "http_method": field("http_method"),
                "supports_idempotency": field("supports_idempotency"),
                "safe_to_retry": field("safe_to_retry"),
                "retry_class": field("retry_class"),
                "access": field("access"),
            }
        )
    return ops


def main() -> int:
    if not CAPS.is_file():
        print(f"error: missing {CAPS}", file=sys.stderr)
        return 1

    ops = parse_capabilities(CAPS.read_text(encoding="utf-8"))
    by_id = {op["operation_id"]: op for op in ops}
    failures: list[str] = []

    # Completeness of high-risk set
    missing = sorted(HIGH_RISK_WRITES - set(by_id))
    if missing:
        failures.append(f"high-risk ops missing from capabilities: {missing}")

    for op in ops:
        oid = op["operation_id"]
        method = (op["http_method"] or "").upper()
        retry = op["retry_class"]
        idem = op["supports_idempotency"]
        safe = op["safe_to_retry"]

        if method == "GET" and retry not in (None, "SafeRead"):
            failures.append(f"{oid}: GET must be SafeRead, got {retry}")
        if retry == "IdempotentWrite" and not idem:
            failures.append(f"{oid}: IdempotentWrite requires supports_idempotency=true")
        if retry in ("IdempotentWrite", "NonRetryableWrite", "FinancialWrite") and safe:
            failures.append(f"{oid}: write class must not set safe_to_retry=true")
        if retry == "SafeRead" and method not in ("GET", "HEAD", "OPTIONS"):
            # allow unknown methods only if explicitly SafeRead for rare cases
            if method in ("POST", "PUT", "PATCH", "DELETE"):
                failures.append(f"{oid}: SafeRead on mutating method {method}")

    for oid in sorted(HIGH_RISK_WRITES & set(by_id)):
        op = by_id[oid]
        if op["retry_class"] == "SafeRead":
            failures.append(f"{oid}: high-risk write classified SafeRead")
        if op["access"] != "ValidatedFacade":
            failures.append(f"{oid}: high-risk write must be RouteAccess::ValidatedFacade")
        if op["retry_class"] not in ("IdempotentWrite", "NonRetryableWrite", "FinancialWrite"):
            failures.append(
                f"{oid}: high-risk write unexpected retry_class={op['retry_class']}"
            )

    print(f"capabilities: {len(ops)}")
    print(f"high-risk writes checked: {len(HIGH_RISK_WRITES & set(by_id))}")
    if failures:
        print("DANGEROUS PROFILE DRIFT:")
        for line in failures:
            print(f"  - {line}")
        return 1
    print("operation safety invariants: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
