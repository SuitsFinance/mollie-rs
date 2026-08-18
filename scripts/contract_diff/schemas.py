"""Schema helpers."""

from __future__ import annotations

from typing import Any


def schema_props(schema: dict[str, Any]) -> dict[str, Any]:
    if not isinstance(schema, dict):
        return {}
    props = schema.get("properties") or {}
    return props if isinstance(props, dict) else {}


def schema_required(schema: dict[str, Any]) -> set[str]:
    if not isinstance(schema, dict):
        return set()
    req = schema.get("required") or []
    if not isinstance(req, list):
        return set()
    return {str(x) for x in req}


def schema_enum(schema: dict[str, Any]) -> list[Any] | None:
    if not isinstance(schema, dict):
        return None
    if "enum" in schema and isinstance(schema["enum"], list):
        return list(schema["enum"])
    # nullable wrapper
    for key in ("allOf", "oneOf", "anyOf"):
        items = schema.get(key)
        if isinstance(items, list):
            for it in items:
                if isinstance(it, dict) and isinstance(it.get("enum"), list):
                    return list(it["enum"])
    return None


def is_nullable(schema: dict[str, Any]) -> bool:
    if not isinstance(schema, dict):
        return False
    if schema.get("nullable") is True:
        return True
    t = schema.get("type")
    if isinstance(t, list) and "null" in t:
        return True
    return False


def type_label(schema: dict[str, Any]) -> str:
    if not isinstance(schema, dict):
        return "unknown"
    if "$ref" in schema:
        return f"ref:{schema['$ref']}"
    t = schema.get("type")
    if isinstance(t, list):
        return "|".join(str(x) for x in t)
    if t:
        fmt = schema.get("format")
        return f"{t}:{fmt}" if fmt else str(t)
    if "enum" in schema:
        return "enum"
    if "properties" in schema:
        return "object"
    return "unknown"


def structural_similarity(a: dict[str, Any], b: dict[str, Any]) -> float:
    pa, pb = set(schema_props(a)), set(schema_props(b))
    if not pa and not pb:
        return 1.0 if type_label(a) == type_label(b) else 0.0
    if not pa or not pb:
        return 0.0
    inter = len(pa & pb)
    union = len(pa | pb)
    return inter / union if union else 0.0
