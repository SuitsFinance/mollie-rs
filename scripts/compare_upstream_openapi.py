#!/usr/bin/env python3
"""Compare local Mollie pin to fetched upstream OpenAPI inventory.

Uses regex extraction (upstream YAML may contain invalid calendar dates that
break PyYAML SafeLoader).

Exit codes:
  0 — local inventory consistent; no upstream missing/extra (or upstream skipped)
  1 — local pin/capabilities inconsistency
  2 — upstream differs from local (missing or extra operations)
  3 — upstream file missing when required
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOCAL = ROOT / "specs-3.0.yaml"
CAPS = ROOT / "src" / "route_capabilities.rs"
UPSTREAM = ROOT / "specs" / "upstream-openapi.yaml"
REGISTRY = ROOT / "docs" / "registries" / "operation-registry.yaml"


def to_snake(value: str) -> str:
    value = value.strip().strip('"').strip("'")
    value = value.replace("-", "_")
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"__+", "_", value)
    return value.lower()


def load_ops_regex(path: Path) -> dict[str, tuple[str, str]]:
    """Return snake_operation_id -> (METHOD, path)."""
    text = path.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.startswith("paths:"):
            start = i + 1
            break
    if start is None:
        return {}
    end = len(lines)
    for i in range(start, len(lines)):
        if lines[i] and not lines[i].startswith(" ") and not lines[i].startswith("#"):
            end = i
            break
    ops: dict[str, tuple[str, str]] = {}
    cur_path = None
    cur_method = None
    for i in range(start, end):
        line = lines[i]
        m = re.match(r"^  (/[^:\n]+):\s*$", line)
        if m:
            cur_path = m.group(1)
            cur_method = None
            continue
        m = re.match(r"^    (get|post|put|patch|delete|head|options):\s*$", line, re.I)
        if m and cur_path:
            cur_method = m.group(1).upper()
            continue
        m = re.match(r"^      operationId:\s*(\S+)", line)
        if m and cur_path and cur_method:
            oid = to_snake(m.group(1))
            # Normalize /v2 prefix for comparison with local stem paths.
            path = cur_path
            if path.startswith("/v2/"):
                path = path[3:]
            elif path == "/v2":
                path = "/"
            ops[oid] = (cur_method, path)
    return ops


def load_caps(path: Path) -> set[str]:
    return set(re.findall(r'operation_id:\s*"([^"]+)"', path.read_text(encoding="utf-8")))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--require-upstream",
        action="store_true",
        help="Fail if specs/upstream-openapi.yaml is missing",
    )
    parser.add_argument(
        "--write",
        type=Path,
        default=None,
        help="Write markdown report path",
    )
    args = parser.parse_args()

    local = load_ops_regex(LOCAL)
    caps = load_caps(CAPS)
    local_ids = set(local)
    missing_caps = sorted(local_ids - caps)
    extra_caps = sorted(caps - local_ids)

    lines = [
        "# Upstream OpenAPI comparison",
        "",
        f"- Local operations: **{len(local)}**",
        f"- Capability operations: **{len(caps)}**",
    ]

    rc = 0
    if missing_caps or extra_caps:
        rc = 1
        lines.append("")
        lines.append("## Local pin / capability inconsistency")
        for op in missing_caps:
            lines.append(f"- missing capability: `{op}`")
        for op in extra_caps:
            lines.append(f"- extra capability: `{op}`")

    if not UPSTREAM.is_file():
        lines.append("")
        lines.append("## Upstream")
        lines.append("Upstream snapshot not present. Run `python scripts/fetch_upstream_openapi.py`.")
        if args.require_upstream:
            rc = 3 if rc == 0 else rc
        report = "\n".join(lines) + "\n"
        if args.write:
            args.write.write_text(report, encoding="utf-8")
        print(report)
        return rc

    up = load_ops_regex(UPSTREAM)
    # Compare by operation id primarily.
    missing = sorted(set(up) - local_ids)
    extra = sorted(local_ids - set(up))
    lines.append(f"- Upstream operations: **{len(up)}**")
    lines.append(f"- Missing from local: **{len(missing)}**")
    lines.append(f"- Extra in local: **{len(extra)}**")
    lines.append("")
    lines.append("## Missing from local pin")
    if not missing:
        lines.append("_None_")
    for op in missing:
        method, path = up[op]
        lines.append(f"- `{method}` `{path}` — `{op}`")
    lines.append("")
    lines.append("## Extra in local pin")
    if not extra:
        lines.append("_None_")
    for op in extra:
        method, path = local[op]
        lines.append(f"- `{method}` `{path}` — `{op}`")

    if missing or extra:
        if rc == 0:
            rc = 2

    # Dangerous subset: local ops removed upstream
    if extra:
        lines.append("")
        lines.append("## Dangerous: local operations absent upstream")
        for op in extra:
            lines.append(f"- `{op}` may have been removed or renamed upstream")

    report = "\n".join(lines) + "\n"
    if args.write:
        args.write.parent.mkdir(parents=True, exist_ok=True)
        args.write.write_text(report, encoding="utf-8")
        print(f"wrote {args.write}")
    print(report)
    # Registry gap count sanity
    if REGISTRY.is_file():
        reg = REGISTRY.read_text(encoding="utf-8")
        m = re.search(r"missing_operation_count:\s*(\d+)", reg)
        if m and missing and int(m.group(1)) != len(missing):
            print(
                f"warning: registry missing_operation_count={m.group(1)} "
                f"but live missing={len(missing)}",
                file=sys.stderr,
            )
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
