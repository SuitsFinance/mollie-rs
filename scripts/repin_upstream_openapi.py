#!/usr/bin/env python3
"""Deliberate upstream OpenAPI repin helper (INV-PROV-01)."""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
import urllib.request
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PIN = ROOT / "specs" / "upstream-pin.toml"
OUT = ROOT / "specs" / "upstream-openapi.yaml"
DEFAULT_URL = "https://raw.githubusercontent.com/mollie/openapi/main/specs.yaml"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", default=DEFAULT_URL)
    parser.add_argument("--write-pin", action="store_true", help="Update specs/upstream-pin.toml sha256")
    args = parser.parse_args()

    print(f"fetching {args.url}")
    with urllib.request.urlopen(args.url, timeout=60) as resp:
        data = resp.read()
    digest = hashlib.sha256(data).hexdigest()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_bytes(data)
    print(f"wrote {OUT} sha256={digest} bytes={len(data)}")

    if args.write_pin:
        text = PIN.read_text(encoding="utf-8")
        text2 = re.sub(r'sha256\s*=\s*"[0-9a-fA-F]+"', f'sha256 = "{digest}"', text, count=1)
        text2 = re.sub(
            r'pinned_date\s*=\s*"[0-9-]+"',
            f'pinned_date = "{date.today().isoformat()}"',
            text2,
            count=1,
        )
        PIN.write_text(text2, encoding="utf-8")
        print(f"updated {PIN}")
    else:
        print("pin not modified (pass --write-pin to update digest)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
