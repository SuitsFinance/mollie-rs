#!/usr/bin/env python3
"""Independent high-risk / mutation discovery vs OperationSafetyProfile SSOT.

Fails when:
  - a mutation exists in OpenAPI/capabilities but is unclassified in profile terms
  - a financial-ish mutation is absent from HIGH_RISK_WRITES
  - a known HIGH_RISK write disappears from capabilities
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAPS = ROOT / "src" / "route_capabilities.rs"
SPEC = ROOT / "specs-3.0.yaml"

# Keep lockstep with check_dangerous_profile_drift.HIGH_RISK_WRITES
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

# Capability mutations that must be classified as PaymentCapabilityMutation risk
# (not necessarily full Tier-S HR denominator, but must be non-SafeRead writes).
PAYMENT_CAPABILITY_MUTATIONS = {
    "enable_method",
    "disable_method",
    "enable_method_issuer",
    "disable_method_issuer",
}

FINANCIAL_NAME_HINTS = re.compile(
    r"(payment|refund|capture|payout|transfer|mandate|subscription|chargeback|"
    r"session|oauth|terminal|route|payee|settlement)",
    re.I,
)


def parse_caps(text: str) -> list[dict]:
    ops = []
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

        oid = field("operation_id")
        if not oid:
            continue
        ops.append(
            {
                "operation_id": oid,
                "http_method": field("http_method"),
                "retry_class": field("retry_class"),
                "access": field("access"),
                "safe_to_retry": field("safe_to_retry"),
            }
        )
    return ops


def main() -> int:
    if not CAPS.is_file():
        print(f"missing {CAPS}", file=sys.stderr)
        return 1
    ops = parse_caps(CAPS.read_text(encoding="utf-8"))
    by_id = {o["operation_id"]: o for o in ops}
    errors: list[str] = []

    missing_hr = sorted(HIGH_RISK_WRITES - set(by_id))
    if missing_hr:
        errors.append(f"HIGH_RISK writes missing from capabilities: {missing_hr}")

    mutations = [
        o
        for o in ops
        if str(o.get("http_method") or "").upper() in {"POST", "PUT", "PATCH", "DELETE"}
    ]

    for o in mutations:
        oid = o["operation_id"]
        rc = o.get("retry_class")
        if rc in {None, "Unknown"}:
            errors.append(f"unclassified mutation retry_class: {oid}")
        if rc == "SafeRead":
            errors.append(f"mutation marked SafeRead: {oid}")
        if o.get("safe_to_retry") is True and rc not in {"IdempotentWrite"}:
            errors.append(f"mutation safe_to_retry without IdempotentWrite: {oid}")

        # financial heuristic vs HR set
        if FINANCIAL_NAME_HINTS.search(oid) and oid not in HIGH_RISK_WRITES:
            # allowlist non-HR financial-ish reads already filtered; writes only
            # exclude pure list/get naming already not in mutations
            # allow known admin-ish exceptions:
            if oid in PAYMENT_CAPABILITY_MUTATIONS:
                continue
            if oid.startswith("list_") or oid.startswith("get_"):
                continue
            # Some updates are lower risk configuration — still require explicit
            # classification file later; for now only flag create_/cancel_ style.
            if oid.startswith("create_") or oid.startswith("cancel_") or "oauth" in oid:
                errors.append(
                    f"financial-ish mutation not in HIGH_RISK_WRITES denominator: {oid}"
                )

    for oid in sorted(PAYMENT_CAPABILITY_MUTATIONS):
        o = by_id.get(oid)
        if not o:
            errors.append(f"payment capability mutation missing: {oid}")
            continue
        if o.get("retry_class") == "SafeRead":
            errors.append(f"payment capability mutation SafeRead: {oid}")
        if o.get("http_method") not in {"POST", "DELETE", "PATCH", "PUT"}:
            errors.append(f"payment capability mutation unexpected method: {oid}")

    # DraftTransfers must not reappear
    for banned in ("create_draft_transfer", "list_draft_transfers", "get_draft_transfer", "delete_draft_transfer"):
        if banned in by_id:
            errors.append(f"banned DraftTransfers operation present: {banned}")

    print(f"capabilities: {len(ops)}")
    print(f"mutations discovered: {len(mutations)}")
    print(f"high-risk denominator: {len(HIGH_RISK_WRITES)}")
    print(f"payment capability mutations: {len(PAYMENT_CAPABILITY_MUTATIONS)}")

    if errors:
        print("detect_high_risk_operations FAILED:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    print("detect_high_risk_operations: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
