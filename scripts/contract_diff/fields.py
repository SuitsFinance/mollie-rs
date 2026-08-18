"""Field-level schema diffs."""

from __future__ import annotations

from typing import Any

from .schemas import is_nullable, schema_props, schema_required, type_label


def diff_object_fields(
    schema_name: str,
    old: dict[str, Any],
    new: dict[str, Any],
) -> list[dict[str, Any]]:
    changes: list[dict[str, Any]] = []
    op, np_ = schema_props(old), schema_props(new)
    oreq, nreq = schema_required(old), schema_required(new)

    for name in sorted(set(op) | set(np_)):
        path = f"{schema_name}.{name}"
        if name not in op:
            changes.append(
                {
                    "kind": "AdditiveRequestOrResponseField",
                    "path": path,
                    "old": None,
                    "new": type_label(np_[name]) if isinstance(np_.get(name), dict) else None,
                }
            )
            continue
        if name not in np_:
            changes.append(
                {
                    "kind": "FieldRemoved",
                    "path": path,
                    "old": type_label(op[name]) if isinstance(op.get(name), dict) else None,
                    "new": None,
                }
            )
            continue
        o_s, n_s = op[name], np_[name]
        if not isinstance(o_s, dict) or not isinstance(n_s, dict):
            continue
        o_null, n_null = is_nullable(o_s), is_nullable(n_s)
        if o_null != n_null:
            changes.append(
                {
                    "kind": "NullableRelaxation" if n_null and not o_null else "NullableRestriction",
                    "path": path,
                    "old": o_null,
                    "new": n_null,
                }
            )
        o_t, n_t = type_label(o_s), type_label(n_s)
        if o_t != n_t:
            moneyish = "amount" in name.lower() or "money" in o_t.lower() or "money" in n_t.lower()
            kind = "MoneyChange" if moneyish or "Amount" in o_t or "Amount" in n_t else "TypeChange"
            changes.append({"kind": kind, "path": path, "old": o_t, "new": n_t})
        in_o, in_n = name in oreq, name in nreq
        if in_o != in_n:
            changes.append(
                {
                    "kind": "RequirednessChange",
                    "path": path,
                    "old": in_o,
                    "new": in_n,
                }
            )
    return changes
