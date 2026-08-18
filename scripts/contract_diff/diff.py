"""Core semantic diff."""

from __future__ import annotations

from typing import Any

from .classify import classify_change
from .enums import diff_enums
from .errors import diff_responses
from .fields import diff_object_fields
from .loader import load_spec, schemas
from .maturity import diff_maturity
from .operations import is_mutation, iter_operations
from .schemas import structural_similarity


def _param_names(params: list[Any]) -> set[str]:
    names: set[str] = set()
    for p in params or []:
        if isinstance(p, dict) and "name" in p:
            names.add(str(p["name"]))
    return names


def _has_idempotency(params: list[Any], request_body: Any) -> bool:
    names = {n.lower() for n in _param_names(params)}
    if "idempotency-key" in names or "idempotency_key" in names:
        return True
    # header params often listed
    for p in params or []:
        if not isinstance(p, dict):
            continue
        if str(p.get("name", "")).lower() in {"idempotency-key", "idempotency_key"}:
            return True
    return False


def _has_testmode(params: list[Any]) -> bool:
    return any(str(n).lower() == "testmode" for n in _param_names(params))


def diff_specs(old_spec: dict[str, Any], new_spec: dict[str, Any]) -> dict[str, Any]:
    changes: list[dict[str, Any]] = []

    old_ops = iter_operations(old_spec)
    new_ops = iter_operations(new_spec)
    old_ids, new_ids = set(old_ops), set(new_ops)

    for oid in sorted(new_ids - old_ids):
        op = new_ops[oid]
        kind = "MutationAdded" if is_mutation(op["method"]) else "OperationAdded"
        changes.append(
            {
                "kind": kind,
                "path": oid,
                "old": None,
                "new": f"{op['method']} {op['path']}",
            }
        )

    for oid in sorted(old_ids - new_ids):
        op = old_ops[oid]
        changes.append(
            {
                "kind": "OperationRemoved",
                "path": oid,
                "old": f"{op['method']} {op['path']}",
                "new": None,
            }
        )

    for oid in sorted(old_ids & new_ids):
        o, n = old_ops[oid], new_ops[oid]
        if o["method"] != n["method"] or o["path"] != n["path"]:
            changes.append(
                {
                    "kind": "OperationRemoved",
                    "path": oid,
                    "old": f"{o['method']} {o['path']}",
                    "new": f"{n['method']} {n['path']}",
                }
            )
        if o.get("security") != n.get("security"):
            changes.append(
                {
                    "kind": "AuthChange",
                    "path": oid,
                    "old": o.get("security"),
                    "new": n.get("security"),
                }
            )
        o_idemp = _has_idempotency(o.get("parameters") or [], o.get("request_body"))
        n_idemp = _has_idempotency(n.get("parameters") or [], n.get("request_body"))
        if o_idemp != n_idemp:
            changes.append(
                {
                    "kind": "IdempotencyChange",
                    "path": oid,
                    "old": o_idemp,
                    "new": n_idemp,
                }
            )
        o_tm, n_tm = _has_testmode(o.get("parameters") or []), _has_testmode(n.get("parameters") or [])
        # also check request body schemas later; parameter-level testmode
        if o_tm != n_tm:
            changes.append(
                {
                    "kind": "TestmodeChange",
                    "path": f"{oid}.parameters.testmode",
                    "old": o_tm,
                    "new": n_tm,
                }
            )
        changes.extend(diff_responses(oid, o.get("responses") or {}, n.get("responses") or {}))
        meta_o = {"tags": o.get("tags"), **(o.get("x_mollie") or {})}
        meta_n = {"tags": n.get("tags"), **(n.get("x_mollie") or {})}
        changes.extend(diff_maturity(oid, meta_o, meta_n))

    old_sch, new_sch = schemas(old_spec), schemas(new_spec)
    old_names, new_names = set(old_sch), set(new_sch)

    removed = sorted(old_names - new_names)
    added = sorted(new_names - old_names)
    matched_replacements: set[str] = set()

    for rname in removed:
        best = None
        best_score = 0.0
        ro = old_sch.get(rname) or {}
        if not isinstance(ro, dict):
            continue
        for aname in added:
            if aname in matched_replacements:
                continue
            ns = new_sch.get(aname) or {}
            if not isinstance(ns, dict):
                continue
            score = structural_similarity(ro, ns)
            if score > best_score:
                best_score = score
                best = aname
        if best is not None and best_score >= 0.5:
            matched_replacements.add(best)
            changes.append(
                {
                    "kind": "SchemaReplacement",
                    "path": f"{rname}->{best}",
                    "old": rname,
                    "new": best,
                    "similarity": round(best_score, 3),
                }
            )
        else:
            changes.append(
                {
                    "kind": "SchemaRemoved",
                    "path": rname,
                    "old": rname,
                    "new": None,
                }
            )

    for aname in added:
        if aname in matched_replacements:
            continue
        changes.append(
            {
                "kind": "SchemaAdded",
                "path": aname,
                "old": None,
                "new": aname,
            }
        )

    for name in sorted(old_names & new_names):
        o_s, n_s = old_sch[name], new_sch[name]
        if not isinstance(o_s, dict) or not isinstance(n_s, dict):
            continue
        changes.extend(diff_enums(name, o_s, n_s))
        changes.extend(diff_object_fields(name, o_s, n_s))
        # testmode property on schemas
        op, np_ = (o_s.get("properties") or {}), (n_s.get("properties") or {})
        if isinstance(op, dict) and isinstance(np_, dict):
            if ("testmode" in np_) != ("testmode" in op):
                changes.append(
                    {
                        "kind": "TestmodeChange",
                        "path": f"{name}.testmode",
                        "old": "testmode" in op,
                        "new": "testmode" in np_,
                    }
                )

    classified = [classify_change(c) for c in changes]
    blocking = [c for c in classified if c.get("blocking")]
    max_risk = max((int(c.get("risk") or 0) for c in classified), default=0)
    return {
        "changes": classified,
        "blocking_count": len(blocking),
        "max_risk": max_risk,
        "blocking": blocking,
    }


def diff_paths(old_path: str, new_path: str) -> dict[str, Any]:
    return diff_specs(load_spec(old_path), load_spec(new_path))
