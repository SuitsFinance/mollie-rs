#!/usr/bin/env python3
"""Verify generated route capability metadata matches specs-3.0.yaml.

This is a lightweight reproducibility gate: it does not re-run the full OpenAPI
generator (which needs a Rust toolchain + progenitor), but it proves that the
checked-in capability table still describes every operation in the pinned spec.

Exit codes:
  0 — capabilities and operation inventory agree
  1 — drift detected or parse failure
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    print("error: PyYAML is required (pip install pyyaml)", file=sys.stderr)
    raise SystemExit(1) from exc

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "specs-3.0.yaml"
CAPABILITIES = ROOT / "src" / "route_capabilities.rs"


def load_spec_operations(path: Path) -> set[str]:
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    ops: set[str] = set()
    paths = data.get("paths") or {}
    for _path, methods in paths.items():
        if not isinstance(methods, dict):
            continue
        for method, body in methods.items():
            if method.startswith("x-") or not isinstance(body, dict):
                continue
            op_id = body.get("operationId")
            if not op_id:
                continue
            # Generator normalizes to snake_case for Rust.
            ops.add(_to_snake(op_id))
    return ops


def _to_snake(value: str) -> str:
    """Normalize OpenAPI operationId to the crate's snake_case form."""
    value = value.replace("-", "_")
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"__+", "_", value)
    return value.lower()


def load_capability_operations(path: Path) -> set[str]:
    text = path.read_text(encoding="utf-8")
    return set(re.findall(r'operation_id:\s*"([^"]+)"', text))


def main() -> int:
    if not SPEC.is_file():
        print(f"error: missing {SPEC}", file=sys.stderr)
        return 1
    if not CAPABILITIES.is_file():
        print(f"error: missing {CAPABILITIES}", file=sys.stderr)
        return 1

    spec_ops = load_spec_operations(SPEC)
    cap_ops = load_capability_operations(CAPABILITIES)

    missing_in_caps = sorted(spec_ops - cap_ops)
    extra_in_caps = sorted(cap_ops - spec_ops)

    print(f"spec operations:        {len(spec_ops)}")
    print(f"capability operations:  {len(cap_ops)}")

    if missing_in_caps:
        print("missing from route_capabilities.rs:")
        for op in missing_in_caps:
            print(f"  - {op}")
    if extra_in_caps:
        print("extra in route_capabilities.rs (not in specs-3.0.yaml):")
        for op in extra_in_caps:
            print(f"  - {op}")

    if missing_in_caps or extra_in_caps:
        print(
            "\nFAIL: regenerate with scripts/generate_route_capabilities.py "
            "(or the full OpenAPI wrapper) and commit the result.",
            file=sys.stderr,
        )
        return 1

    print("OK: route capabilities match pinned OpenAPI operation inventory.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
