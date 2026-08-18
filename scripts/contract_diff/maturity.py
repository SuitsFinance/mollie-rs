"""Maturity / extension marker diffs."""

from __future__ import annotations

from typing import Any


def maturity_label(meta: dict[str, Any]) -> str | None:
    for k, v in (meta or {}).items():
        lk = str(k).lower()
        if "beta" in lk or "maturity" in lk or "stability" in lk:
            return f"{k}={v}"
    tags = meta.get("tags") if isinstance(meta, dict) else None
    if isinstance(tags, list):
        for t in tags:
            if isinstance(t, str) and "beta" in t.lower():
                return t
    return None


def diff_maturity(op_id: str, old_meta: dict[str, Any], new_meta: dict[str, Any]) -> list[dict[str, Any]]:
    o, n = maturity_label(old_meta), maturity_label(new_meta)
    if o == n:
        return []
    return [
        {
            "kind": "MaturityChange",
            "path": op_id,
            "old": o,
            "new": n,
        }
    ]
