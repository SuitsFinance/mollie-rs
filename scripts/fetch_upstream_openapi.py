#!/usr/bin/env python3
"""Download and verify the pinned Mollie upstream OpenAPI document.

Reads specs/upstream-pin.toml, fetches the URL, verifies sha256, writes
specs/upstream-openapi.yaml (gitignored), and prints inventory counts.

Exit codes:
  0 — fetch OK and digest matches pin
  1 — pin/config/network/parse error
  3 — digest mismatch (blocking: pin out of date or upstream changed)
"""

from __future__ import annotations

import hashlib
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PIN = ROOT / "specs" / "upstream-pin.toml"
OUT = ROOT / "specs" / "upstream-openapi.yaml"


def parse_pin(text: str) -> dict[str, str]:
    """Minimal TOML subset parser for [upstream] string/int fields."""
    section = None
    data: dict[str, dict[str, str]] = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("[") and line.endswith("]"):
            section = line[1:-1].strip()
            data.setdefault(section, {})
            continue
        if section is None or "=" not in line:
            continue
        key, val = line.split("=", 1)
        key = key.strip()
        val = val.strip()
        if val.startswith('"') and val.endswith('"'):
            val = val[1:-1]
        data[section][key] = val
    return data.get("upstream") or {}


def count_operation_ids(text: str) -> int:
    return len(re.findall(r"^\s+operationId:\s+\S+", text, re.M))


def main() -> int:
    if not PIN.is_file():
        print(f"error: missing pin file {PIN}", file=sys.stderr)
        return 1
    pin = parse_pin(PIN.read_text(encoding="utf-8"))
    url = pin.get("url")
    expected = pin.get("sha256", "").lower()
    if not url or not expected:
        print("error: pin missing url or sha256", file=sys.stderr)
        return 1

    print(f"fetching {url}")
    try:
        with urllib.request.urlopen(url, timeout=90) as response:  # noqa: S310
            body = response.read()
    except Exception as exc:  # pragma: no cover
        print(f"error: fetch failed: {exc}", file=sys.stderr)
        return 1

    digest = hashlib.sha256(body).hexdigest()
    print(f"bytes={len(body)} sha256={digest}")
    if digest != expected:
        print(
            "error: upstream digest mismatch\n"
            f"  expected: {expected}\n"
            f"  actual:   {digest}\n"
            "Update specs/upstream-pin.toml after reviewing upstream changes.",
            file=sys.stderr,
        )
        return 3

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_bytes(body)
    text = body.decode("utf-8", errors="replace")
    ops = count_operation_ids(text)
    expected_ops = int(pin.get("operation_count", "0") or "0")
    print(f"operationId_count={ops} pin_operation_count={expected_ops}")
    if expected_ops and ops != expected_ops:
        print(
            f"error: operation count {ops} != pin operation_count {expected_ops}",
            file=sys.stderr,
        )
        return 3
    print(f"wrote {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
