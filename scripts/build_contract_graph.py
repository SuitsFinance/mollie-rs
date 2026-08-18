#!/usr/bin/env python3
"""Build OpenAPI operation → capability → Tier-S exposure contract graph."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAPS = ROOT / "src" / "route_capabilities.rs"
OUT = ROOT / "docs" / "registries" / "contract-graph.json"
DOMAIN = ROOT / "src" / "domain"


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
                "path": field("path"),
                "route_group": field("route_group"),
                "retry_class": field("retry_class"),
                "access": field("access"),
            }
        )
    return ops


def domain_modules() -> list[str]:
    if not DOMAIN.is_dir():
        return []
    return sorted(p.stem for p in DOMAIN.glob("*.rs") if p.stem not in {"mod", "common", "README"})


def main() -> int:
    ops = parse_caps(CAPS.read_text(encoding="utf-8"))
    modules = domain_modules()
    nodes = []
    edges = []
    for o in ops:
        oid = o["operation_id"]
        nodes.append({"id": f"op:{oid}", "type": "operation", **o})
        nodes.append({"id": f"profile:{oid}", "type": "OperationSafetyProfile"})
        edges.append({"from": f"op:{oid}", "to": f"profile:{oid}", "kind": "operation-profiled-by"})
        if o.get("access") == "ValidatedFacade":
            edges.append(
                {
                    "from": f"op:{oid}",
                    "to": "tier_s:facade",
                    "kind": "operation-exposed-by-facade",
                }
            )
    nodes.append({"id": "tier_s:facade", "type": "TierS", "modules": modules})
    graph = {
        "version": 1,
        "operation_count": len(ops),
        "domain_modules": modules,
        "nodes": nodes,
        "edges": edges,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(graph, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"wrote {OUT} ops={len(ops)} modules={len(modules)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
