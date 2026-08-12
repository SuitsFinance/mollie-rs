#!/usr/bin/env python3
"""Generate high-risk operation coverage report (INV-TIER-01).

Reads `src/route_capabilities.rs` and the frozen HIGH_RISK_WRITES set from
`check_dangerous_profile_drift.py` (imported via exec of the constant).

Outputs:
  docs/registries/high-risk-coverage.md
  docs/registries/high-risk-coverage.json

Exit codes:
  0 — report written; if --require-full and not 100% ValidatedFacade → 1
  1 — parse failure or incomplete under --require-full
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAPS = ROOT / "src" / "route_capabilities.rs"
DOMAIN = ROOT / "src" / "domain"
OUT_MD = ROOT / "docs" / "registries" / "high-risk-coverage.md"
OUT_JSON = ROOT / "docs" / "registries" / "high-risk-coverage.json"

# Keep in lockstep with check_dangerous_profile_drift.HIGH_RISK_WRITES
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

# operation_id → domain module hint for Tier-S presence
TIER_S_HINTS = {
    "create_payment": "payments",
    "cancel_payment": "payments",
    "create_refund": "refunds",
    "cancel_refund": "refunds",
    "create_capture": "captures",
    "create_subscription": "subscriptions",
    "cancel_subscription": "subscriptions",
    "create_mandate": "mandates",
    "create_payment_link": "payment_links",
    "create_customer_payment": "payments",
    "create_payout": "payouts",
    "cancel_payout": "payouts",
    "create_transfer": "transfers",
    "create_connect_balance_transfer": "connect_balance_transfers",
    "verify_payee": "verify_payee",
    "oauth_generate_tokens": "oauth",
    "oauth_revoke_tokens": "oauth",
    "payment_create_route": "payments",
    "create_session": "sessions",
    "terminals_request_pairing_code": "terminals",
    "terminals_revoke_pairing_code": "terminals",
    "match_unmatched_credit_transfer": "unmatched_credit_transfers",
    "return_unmatched_credit_transfer": "unmatched_credit_transfers",
}


def parse_capabilities(text: str) -> dict[str, dict]:
    ops: dict[str, dict] = {}
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
        ops[op_id] = {
            "operation_id": op_id,
            "http_method": field("http_method"),
            "supports_idempotency": field("supports_idempotency"),
            "safe_to_retry": field("safe_to_retry"),
            "retry_class": field("retry_class"),
            "access": field("access"),
            "path": field("path"),
        }
    return ops


def tier_s_present(module: str | None) -> bool:
    if not module:
        return False
    return (DOMAIN / f"{module}.rs").is_file()


def row_status(op: dict, has_tier_s: bool) -> str:
    if op.get("access") != "ValidatedFacade":
        return "partial"
    if not has_tier_s:
        return "partial"
    if op.get("retry_class") not in (
        "IdempotentWrite",
        "NonRetryableWrite",
        "FinancialWrite",
    ):
        return "partial"
    return "full"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--require-full",
        action="store_true",
        help="exit 1 unless every high-risk op is fully protected",
    )
    parser.add_argument("--write", action="store_true", default=True)
    args = parser.parse_args()

    if not CAPS.is_file():
        print(f"error: missing {CAPS}", file=sys.stderr)
        return 1

    by_id = parse_capabilities(CAPS.read_text(encoding="utf-8"))
    rows = []
    full = 0
    for oid in sorted(HIGH_RISK_WRITES):
        op = by_id.get(oid)
        module = TIER_S_HINTS.get(oid)
        has_ts = tier_s_present(module)
        if not op:
            rows.append(
                {
                    "operation_id": oid,
                    "status": "missing",
                    "access": None,
                    "retry_class": None,
                    "tier_s_module": module,
                    "tier_s": False,
                }
            )
            continue
        st = row_status(op, has_ts)
        if st == "full":
            full += 1
        rows.append(
            {
                "operation_id": oid,
                "status": st,
                "access": op["access"],
                "retry_class": op["retry_class"],
                "supports_idempotency": op["supports_idempotency"],
                "http_method": op["http_method"],
                "path": op["path"],
                "tier_s_module": module,
                "tier_s": has_ts,
            }
        )

    total = len(HIGH_RISK_WRITES)
    payload = {
        "high_risk_total": total,
        "fully_protected": full,
        "ratio": f"{full}/{total}",
        "operations": rows,
    }

    OUT_MD.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "# High-risk operation coverage",
        "",
        f"**Fully protected:** {full} / {total}",
        "",
        "| operation_id | access | retry | Tier-S | status |",
        "| --- | --- | --- | --- | --- |",
    ]
    for r in rows:
        lines.append(
            f"| `{r['operation_id']}` | {r.get('access')} | {r.get('retry_class')} | "
            f"{'yes' if r.get('tier_s') else 'no'} ({r.get('tier_s_module')}) | **{r['status']}** |"
        )
    lines.append("")
    lines.append(
        "Fully protected = `ValidatedFacade` + Tier-S module present + write retry class."
    )
    lines.append("")
    OUT_MD.write_text("\n".join(lines) + "\n", encoding="utf-8")
    OUT_JSON.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    print(f"high-risk fully protected: {full}/{total}")
    print(f"wrote {OUT_MD.relative_to(ROOT)}")
    print(f"wrote {OUT_JSON.relative_to(ROOT)}")

    if args.require_full and full != total:
        print("FAIL: high-risk coverage not 100%", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
