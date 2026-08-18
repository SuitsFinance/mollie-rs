#!/usr/bin/env python3
"""Snapshot / verify Tier-S public facade surface (blocking drift gate).

Captures:
  - domain module re-exports (*Api types)
  - public methods on *Api impl blocks in src/domain/
  - curated crate-root re-exports for Tier-S builders / safety types

Usage:
  python scripts/check_tier_s_public_api.py           # verify
  python scripts/check_tier_s_public_api.py --write    # refresh snapshot
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOMAIN = ROOT / "src" / "domain"
LIB = ROOT / "src" / "lib.rs"
SNAP = ROOT / "docs" / "registries" / "tier-s-public-api.snapshot"
CREATE_PAYMENT = ROOT / "src" / "create_payment.rs"
WRITE_REQUESTS = ROOT / "src" / "write_requests.rs"

RE_PUB_USE = re.compile(r"^pub use (\w+)::(\w+);", re.M)
# Matches: impl Foo {, impl<'a> Foo {, impl Foo<'_> {, impl<'a> Foo<'a> {
RE_IMPL = re.compile(r"^impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{", re.M)
RE_PUB_FN = re.compile(r"^\s+pub (?:async )?fn ([A-Za-z0-9_]+)\s*[(<]", re.M)
RE_CLIENT_FACADE = re.compile(
    r"^\s+pub fn ([A-Za-z0-9_]+)\(&self\)\s*->\s*(\w+)(?:<[^>]+>)?", re.M
)
RE_PUB_STRUCT = re.compile(r"^pub struct (\w+)", re.M)
RE_PUB_ENUM = re.compile(r"^pub enum (\w+)", re.M)
RE_LIB_USE_ITEM = re.compile(
    r"pub use (?:create_payment|write_requests|open_enum|nullable_field|webhook_verify|operation_safety|money|idempotency)::\{([^}]+)\}",
    re.S,
)

CURATED_PREFIXES = (
    "facade:",
    "method:",
    "builder:",
    "safety:",
    "export:",
)


def domain_exports() -> list[str]:
    text = (DOMAIN / "mod.rs").read_text(encoding="utf-8")
    out = []
    for m in RE_PUB_USE.finditer(text):
        out.append(f"facade:{m.group(2)}")
    return sorted(set(out))


def domain_methods() -> list[str]:
    out: list[str] = []
    for path in sorted(DOMAIN.glob("*.rs")):
        if path.name in {"mod.rs", "common.rs", "README.md"}:
            continue
        text = path.read_text(encoding="utf-8")
        # MollieClient facade entry points: pub fn payments(&self) -> PaymentsApi
        for m in RE_CLIENT_FACADE.finditer(text):
            fn, ret = m.group(1), m.group(2)
            if ret.endswith("Api"):
                out.append(f"method:MollieClient.{fn}->{ret}")
        # Find impl blocks for *Api types (and TransferClientSignature)
        positions = [(m.start(), m.group(1)) for m in RE_IMPL.finditer(text)]
        for i, (start, ty) in enumerate(positions):
            if not (ty.endswith("Api") or ty == "TransferClientSignature"):
                continue
            end = positions[i + 1][0] if i + 1 < len(positions) else len(text)
            block = text[start:end]
            for fm in RE_PUB_FN.finditer(block):
                name = fm.group(1)
                if name in {"new", "fmt", "clone", "default"}:
                    continue
                out.append(f"method:{ty}.{name}")
    return sorted(set(out))


def builder_types() -> list[str]:
    out: list[str] = []
    for path in (CREATE_PAYMENT, WRITE_REQUESTS):
        text = path.read_text(encoding="utf-8")
        for m in RE_PUB_STRUCT.finditer(text):
            out.append(f"builder:{m.group(1)}")
        for m in RE_PUB_ENUM.finditer(text):
            out.append(f"builder:{m.group(1)}")
    return sorted(set(out))


def safety_exports() -> list[str]:
    text = LIB.read_text(encoding="utf-8")
    out: list[str] = []
    # Fixed critical exports that must remain public
    required = [
        "export:OpenEnum",
        "export:NullableField",
        "export:VerifiedWebhook",
        "export:OperationRisk",
        "export:OperationExposure",
        "export:OperationSafetyProfile",
        "export:HIGH_RISK_WRITE_OPERATION_IDS",
        "export:PAYMENT_CAPABILITY_MUTATION_OPERATION_IDS",
        "export:CreatePaymentRequired",
        "export:CreateRefundRequired",
        "export:CreateCaptureRequired",
        "export:CreatePayoutRequired",
        "export:CreateTransferRequired",
        "export:CreateConnectBalanceTransferRequired",
        "export:IdempotencyKey",
        "export:Money",
    ]
    for item in required:
        name = item.split(":", 1)[1]
        if re.search(rf"\b{re.escape(name)}\b", text):
            out.append(item)
        else:
            out.append(item)  # still list; verify will fail if missing from lib later
    # Verify presence
    missing = [i for i in out if not re.search(rf"\b{i.split(':',1)[1]}\b", text)]
    if missing:
        raise SystemExit(f"required exports missing from lib.rs: {missing}")
    return sorted(set(out))


def collect() -> list[str]:
    lines = domain_exports() + domain_methods() + builder_types() + safety_exports()
    return sorted(set(lines))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true", help="rewrite snapshot")
    args = ap.parse_args()
    current = collect()
    body = "# Tier-S public API snapshot — do not hand-edit; regenerate with --write\n"
    body += "\n".join(current) + "\n"
    if args.write:
        SNAP.parent.mkdir(parents=True, exist_ok=True)
        SNAP.write_text(body, encoding="utf-8", newline="\n")
        print(f"wrote {SNAP.relative_to(ROOT)} ({len(current)} symbols)")
        return 0
    if not SNAP.is_file():
        print(f"missing snapshot {SNAP}; run with --write", file=sys.stderr)
        return 1
    expected = [
        ln.strip()
        for ln in SNAP.read_text(encoding="utf-8").splitlines()
        if ln.strip() and not ln.strip().startswith("#")
    ]
    exp_set, cur_set = set(expected), set(current)
    added = sorted(cur_set - exp_set)
    removed = sorted(exp_set - cur_set)
    if added or removed:
        print("Tier-S public API snapshot DRIFT:", file=sys.stderr)
        for r in removed:
            print(f"  - removed: {r}", file=sys.stderr)
        for a in added:
            print(f"  + added:   {a}", file=sys.stderr)
        print(
            "If intentional, refresh: python scripts/check_tier_s_public_api.py --write",
            file=sys.stderr,
        )
        return 1
    print(f"tier-s public API snapshot OK ({len(current)} symbols)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
