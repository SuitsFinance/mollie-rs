"""Operation inventory extraction."""

from __future__ import annotations

import re
from typing import Any

HTTP_METHODS = {"get", "post", "put", "patch", "delete", "head", "options"}


def to_snake(value: str) -> str:
    value = value.strip().strip('"').strip("'")
    value = value.replace("-", "_")
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"__+", "_", value)
    return value.lower()


def iter_operations(spec: dict[str, Any]) -> dict[str, dict[str, Any]]:
    """operation_id_snake -> metadata."""
    out: dict[str, dict[str, Any]] = {}
    paths = spec.get("paths") or {}
    if not isinstance(paths, dict):
        return out
    for path, item in paths.items():
        if not isinstance(item, dict):
            continue
        for method, op in item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(op, dict):
                continue
            oid = op.get("operationId") or f"{method}_{path}"
            key = to_snake(str(oid))
            security = op.get("security")
            responses = op.get("responses") or {}
            params = op.get("parameters") or []
            out[key] = {
                "operation_id": key,
                "raw_operation_id": oid,
                "method": method.upper(),
                "path": path,
                "security": security,
                "responses": responses if isinstance(responses, dict) else {},
                "parameters": params if isinstance(params, list) else [],
                "request_body": op.get("requestBody"),
                "tags": op.get("tags") or [],
                "x_mollie": {k: v for k, v in op.items() if str(k).startswith("x-")},
            }
    return out


def is_mutation(method: str) -> bool:
    return method.upper() in {"POST", "PUT", "PATCH", "DELETE"}
