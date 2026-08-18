"""Enum diffs."""

from __future__ import annotations

from typing import Any

from .schemas import schema_enum


def diff_enums(schema_name: str, old: dict[str, Any], new: dict[str, Any]) -> list[dict[str, Any]]:
    oe, ne = schema_enum(old), schema_enum(new)
    if oe is None and ne is None:
        return []
    changes: list[dict[str, Any]] = []
    if oe is None and ne is not None:
        changes.append(
            {"kind": "AdditiveResponseEnum", "path": schema_name, "old": None, "new": ne}
        )
        return changes
    if oe is not None and ne is None:
        changes.append(
            {"kind": "EnumRemoved", "path": schema_name, "old": oe, "new": None}
        )
        return changes
    assert oe is not None and ne is not None
    so, sn = set(map(str, oe)), set(map(str, ne))
    added = sorted(sn - so)
    removed = sorted(so - sn)
    if added:
        changes.append(
            {
                "kind": "AdditiveResponseEnum",
                "path": schema_name,
                "old": sorted(so),
                "new": added,
            }
        )
    if removed:
        changes.append(
            {
                "kind": "EnumValueRemoved",
                "path": schema_name,
                "old": removed,
                "new": sorted(sn),
            }
        )
    return changes
