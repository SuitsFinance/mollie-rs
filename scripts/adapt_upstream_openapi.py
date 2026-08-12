#!/usr/bin/env python3
"""Adapt upstream Mollie OpenAPI into the local generation pin format.

Upstream (mollie/openapi):
  openapi 3.1.0, server https://api.mollie.com, paths /v2/..., /oauth2/...

Local generation pin (progenitor 0.11 + openapiv3 2.2):
  openapi 3.0.3, server https://api.mollie.com/v2, paths /... under /v2
  oauth keeps /oauth2/* (Client::endpoint rewrites host path)

Also:
  - fixes invalid calendar timestamps
  - converts JSON Schema type arrays / null unions toward OpenAPI 3.0
  - dedupes case-colliding enums for typify
  - rewrites unsupported deepObject query styles to form
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    print("error: PyYAML required", file=sys.stderr)
    raise SystemExit(1) from exc

ROOT = Path(__file__).resolve().parents[1]


def _disable_timestamp_resolver() -> None:
    yaml.SafeLoader.yaml_implicit_resolvers = {
        key: [r for r in resolvers if r[0] != "tag:yaml.org,2002:timestamp"]
        for key, resolvers in yaml.SafeLoader.yaml_implicit_resolvers.items()
    }


def strip_v2_path_keys(text: str) -> str:
    text = text.replace("2024-04-31", "2024-04-30")

    def rewrite_path_key(match: re.Match[str]) -> str:
        path = match.group(1)
        if path.startswith("/v2/"):
            return f"  {path[3:]}:"
        if path == "/v2":
            return "  /:"
        return match.group(0)

    return re.sub(r"(?m)^  (/[^:\n]+):$", rewrite_path_key, text)


def dedupe_enum(values: list) -> list:
    groups: dict[str, list] = {}
    for value in values:
        if value is None:
            continue
        key = re.sub(r"[^A-Za-z0-9]", "", str(value)).lower()
        groups.setdefault(key, []).append(value)
    out: list = []
    for variants in groups.values():
        if len(variants) == 1:
            out.append(variants[0])
            continue
        scored = sorted(
            variants,
            key=lambda item: (
                0 if re.fullmatch(r"[A-Z0-9_]+", str(item)) else 1,
                len(str(item)),
                str(item),
            ),
        )
        out.append(scored[0])
    return out


def convert_schema(node):
    if isinstance(node, list):
        return [convert_schema(item) for item in node]
    if not isinstance(node, dict):
        return node

    out = {key: convert_schema(value) for key, value in node.items()}
    type_value = out.get("type")
    if isinstance(type_value, list):
        non_null = [item for item in type_value if item != "null"]
        has_null = "null" in type_value
        if len(non_null) == 1:
            out["type"] = non_null[0]
            if has_null:
                out["nullable"] = True
        elif not non_null:
            out.pop("type", None)
            out["nullable"] = True
        else:
            out["type"] = non_null[0]
            if has_null:
                out["nullable"] = True
    elif type_value == "null":
        out.pop("type", None)
        out["nullable"] = True

    if "const" in out and "enum" not in out:
        out["enum"] = [out.pop("const")]

    if isinstance(out.get("enum"), list):
        enums = [item for item in out["enum"] if item is not None]
        if len(enums) != len(out["enum"]):
            out["nullable"] = True
        out["enum"] = dedupe_enum(enums)

    for key in ("anyOf", "oneOf"):
        if key in out and isinstance(out[key], list):
            variants = out[key]
            non_null_vars = [
                variant
                for variant in variants
                if not (isinstance(variant, dict) and variant.get("type") == "null")
            ]
            if len(non_null_vars) == 1 and len(variants) >= 2:
                merged = dict(non_null_vars[0]) if isinstance(non_null_vars[0], dict) else {}
                merged["nullable"] = True
                base = {k: v for k, v in out.items() if k != key}
                base.update(merged)
                out = base
            else:
                out[key] = non_null_vars

    if out.get("style") == "deepObject":
        # progenitor 0.11 does not support deepObject; post-processing remains.
        out["style"] = "form"

    return out


def drop_null_defaults(node):
    if isinstance(node, list):
        return [drop_null_defaults(item) for item in node]
    if not isinstance(node, dict):
        return node
    out = {}
    for key, value in node.items():
        if value is None and key in {"default", "example"}:
            continue
        out[key] = drop_null_defaults(value)
    if out.get("type") is None and "type" in out:
        out.pop("type")
    return out


def rewrite_content_types(node):
    """Map HAL JSON content types to application/json for progenitor typing.

    progenitor 0.11 emits ByteStream for non-JSON content types, which breaks
    the SDK's typed ResponseValue normalization.
    """
    if isinstance(node, list):
        return [rewrite_content_types(item) for item in node]
    if not isinstance(node, dict):
        return node
    if "content" in node and isinstance(node["content"], dict):
        content = {}
        for media_type, body in node["content"].items():
            key = media_type
            if media_type in {
                "application/hal+json",
                "application/hal+json; charset=utf-8",
            }:
                key = "application/json"
            # Prefer application/json if both exist.
            if key in content and media_type.startswith("application/hal"):
                continue
            content[key] = rewrite_content_types(body)
        node = {**node, "content": content}
    return {key: rewrite_content_types(value) for key, value in node.items()}


def adapt_document(text: str) -> dict:
    _disable_timestamp_resolver()
    text = strip_v2_path_keys(text)
    data = yaml.safe_load(text)
    data["openapi"] = "3.0.3"
    data["servers"] = [{"url": "https://api.mollie.com/v2"}]
    if isinstance(data.get("info", {}).get("license"), dict):
        data["info"]["license"].pop("identifier", None)
    for key in ("jsonSchemaDialect", "webhooks"):
        data.pop(key, None)
    data = convert_schema(data)
    data = drop_null_defaults(data)
    data = rewrite_content_types(data)
    return data


def count_ops(data: dict) -> int:
    count = 0
    for methods in (data.get("paths") or {}).values():
        if not isinstance(methods, dict):
            continue
        for method, body in methods.items():
            if method.lower() in {"get", "post", "put", "patch", "delete", "head", "options"}:
                if isinstance(body, dict) and body.get("operationId"):
                    count += 1
    return count


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input",
        type=Path,
        default=ROOT / "specs" / "upstream-openapi.yaml",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / "specs-3.0.yaml",
    )
    parser.add_argument("--also-specs-yaml", action="store_true")
    args = parser.parse_args()

    if not args.input.is_file():
        print(
            f"error: missing {args.input}; run scripts/fetch_upstream_openapi.py first",
            file=sys.stderr,
        )
        return 1

    raw = args.input.read_text(encoding="utf-8", errors="replace")
    data = adapt_document(raw)
    ops = count_ops(data)
    with args.output.open("w", encoding="utf-8", newline="\n") as handle:
        yaml.safe_dump(data, handle, sort_keys=False, allow_unicode=True)
    print(f"wrote {args.output} operations={ops} paths={len(data.get('paths') or {})}")
    if args.also_specs_yaml:
        alias = ROOT / "specs.yaml"
        alias.write_text(args.output.read_text(encoding="utf-8"), encoding="utf-8", newline="\n")
        print(f"wrote {alias}")
    if ops < 120:
        print(f"error: expected ~124 operations, got {ops}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
