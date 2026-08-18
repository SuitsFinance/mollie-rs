"""Load OpenAPI documents (YAML/JSON) into plain dicts."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import yaml


def load_spec(path: str | Path) -> dict[str, Any]:
    path = Path(path)
    text = path.read_text(encoding="utf-8")
    if path.suffix.lower() in {".json"}:
        data = json.loads(text)
    else:
        data = yaml.safe_load(text)
    if not isinstance(data, dict):
        raise ValueError(f"OpenAPI root must be a mapping: {path}")
    return data


def schemas(spec: dict[str, Any]) -> dict[str, Any]:
    comps = spec.get("components") or {}
    sch = comps.get("schemas") or {}
    return sch if isinstance(sch, dict) else {}


def paths(spec: dict[str, Any]) -> dict[str, Any]:
    p = spec.get("paths") or {}
    return p if isinstance(p, dict) else {}
