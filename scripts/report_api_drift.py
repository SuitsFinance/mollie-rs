#!/usr/bin/env python3
"""Produce a machine-readable OpenAPI inventory / drift report for mollie-rs.

By default this reports the **local** pinned contract (`specs-3.0.yaml`) and
compares it to `src/route_capabilities.rs`.

Optional upstream comparison:

  python scripts/report_api_drift.py --upstream path/or/url/to/openapi.yaml

Upstream fetch is never applied automatically to generated sources. Reviewers
must decide exclusions and regenerate via the normal OpenAPI pipeline.

Exit codes:
  0 — report written (or printed); local inventory consistent
  1 — local capability drift vs pinned spec, or fatal parse/fetch error
  2 — upstream provided and differs (report still written when --write is set)
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import sys
import urllib.request
from collections import defaultdict
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover
    print("error: PyYAML is required (pip install pyyaml)", file=sys.stderr)
    raise SystemExit(1) from exc

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "specs-3.0.yaml"
CAPABILITIES = ROOT / "src" / "route_capabilities.rs"


def _to_snake(value: str) -> str:
    value = value.replace("-", "_")
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"__+", "_", value)
    return value.lower()


def load_operations(source: str | Path) -> dict[str, dict[str, Any]]:
    """Return operation_id_snake -> metadata."""
    if isinstance(source, Path):
        text = source.read_text(encoding="utf-8")
    else:
        if source.startswith("http://") or source.startswith("https://"):
            with urllib.request.urlopen(source, timeout=60) as response:  # noqa: S310
                text = response.read().decode("utf-8")
        else:
            text = Path(source).read_text(encoding="utf-8")

    data = yaml.safe_load(text)
    ops: dict[str, dict[str, Any]] = {}
    for path, methods in (data.get("paths") or {}).items():
        if not isinstance(methods, dict):
            continue
        for method, body in methods.items():
            if method.startswith("x-") or not isinstance(body, dict):
                continue
            op_id = body.get("operationId")
            if not op_id:
                continue
            key = _to_snake(op_id)
            tags = body.get("tags") or []
            ops[key] = {
                "operation_id": key,
                "raw_operation_id": op_id,
                "http_method": method.upper(),
                "path": path,
                "tags": tags,
                "deprecated": bool(body.get("deprecated", False)),
                "summary": (body.get("summary") or "").strip(),
            }
    return ops


def load_capability_ops(path: Path) -> set[str]:
    text = path.read_text(encoding="utf-8")
    return set(re.findall(r'operation_id:\s*"([^"]+)"', text))


def render_report(
    local: dict[str, dict[str, Any]],
    caps: set[str],
    upstream: dict[str, dict[str, Any]] | None,
) -> str:
    # Date-only stamp keeps the committed report stable within a day so CI
    # dirty-tree checks are not flaky on wall-clock regeneration.
    now = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%d")
    by_tag: dict[str, list[str]] = defaultdict(list)
    for op_id, meta in sorted(local.items()):
        tag = meta["tags"][0] if meta["tags"] else "(untagged)"
        by_tag[tag].append(op_id)

    missing_caps = sorted(set(local) - caps)
    extra_caps = sorted(caps - set(local))

    lines: list[str] = [
        "# Mollie API drift report",
        "",
        f"Generated: `{now}`",
        "",
        "This file is produced by `scripts/report_api_drift.py`.",
        "It does **not** regenerate client sources.",
        "",
        "## Local pinned contract (`specs-3.0.yaml`)",
        "",
        f"- Operations: **{len(local)}**",
        f"- Route capabilities: **{len(caps)}**",
        f"- Missing from capabilities: **{len(missing_caps)}**",
        f"- Extra in capabilities: **{len(extra_caps)}**",
        "",
    ]

    if missing_caps or extra_caps:
        lines.append("### Local capability inconsistencies")
        lines.append("")
        for op in missing_caps:
            lines.append(f"- missing capability: `{op}`")
        for op in extra_caps:
            lines.append(f"- extra capability: `{op}`")
        lines.append("")
    else:
        lines.append("Local capabilities match the pinned OpenAPI operation inventory.")
        lines.append("")

    lines.append("### Operations by tag")
    lines.append("")
    for tag in sorted(by_tag):
        lines.append(f"- `{tag}`: {len(by_tag[tag])}")
    lines.append("")

    lines.append("### Full local operation inventory")
    lines.append("")
    lines.append("| operation_id | method | path | deprecated |")
    lines.append("| --- | --- | --- | --- |")
    for op_id, meta in sorted(local.items()):
        dep = "yes" if meta["deprecated"] else ""
        lines.append(
            f"| `{op_id}` | `{meta['http_method']}` | `{meta['path']}` | {dep} |"
        )
    lines.append("")

    if upstream is None:
        lines.extend(
            [
                "## Upstream comparison",
                "",
                "No `--upstream` snapshot provided. CI records the local inventory only.",
                "To compare against an authoritative Mollie OpenAPI document:",
                "",
                "```sh",
                "python scripts/report_api_drift.py --upstream path/to/upstream.yaml --write docs/api-drift-report.md",
                "```",
                "",
            ]
        )
    else:
        only_local = sorted(set(local) - set(upstream))
        only_upstream = sorted(set(upstream) - set(local))
        shared = sorted(set(local) & set(upstream))
        path_changed = []
        for op in shared:
            if local[op]["path"] != upstream[op]["path"] or local[op][
                "http_method"
            ] != upstream[op]["http_method"]:
                path_changed.append(op)

        lines.extend(
            [
                "## Upstream comparison",
                "",
                f"- Upstream operations: **{len(upstream)}**",
                f"- Only in local pin: **{len(only_local)}**",
                f"- Only upstream: **{len(only_upstream)}**",
                f"- Shared: **{len(shared)}**",
                f"- Method/path mismatches: **{len(path_changed)}**",
                "",
            ]
        )
        if only_upstream:
            lines.append("### Added upstream (not in local pin)")
            lines.append("")
            for op in only_upstream:
                meta = upstream[op]
                lines.append(
                    f"- `{op}` `{meta['http_method']}` `{meta['path']}`"
                )
            lines.append("")
        if only_local:
            lines.append("### Only in local pin (removed or renamed upstream)")
            lines.append("")
            for op in only_local:
                meta = local[op]
                lines.append(
                    f"- `{op}` `{meta['http_method']}` `{meta['path']}`"
                )
            lines.append("")
        if path_changed:
            lines.append("### Shared ops with path/method drift")
            lines.append("")
            for op in path_changed:
                lines.append(
                    f"- `{op}`: local `{local[op]['http_method']} {local[op]['path']}` "
                    f"vs upstream `{upstream[op]['http_method']} {upstream[op]['path']}`"
                )
            lines.append("")

    lines.extend(
        [
            "## Policy",
            "",
            "- Do **not** auto-publish a regeneration from upstream drift.",
            "- Review Tier G (generated) fallout before merging OpenAPI updates.",
            "- Intentional exclusions should be documented in `docs/route-coverage.md`.",
            "- See `docs/compatibility.md` for stability tiers.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--upstream",
        help="Optional path or https URL to an upstream OpenAPI document",
    )
    parser.add_argument(
        "--write",
        type=Path,
        help="Write markdown report to this path (e.g. docs/api-drift-report.md)",
    )
    args = parser.parse_args()

    local = load_operations(SPEC)
    caps = load_capability_ops(CAPABILITIES)
    upstream = load_operations(args.upstream) if args.upstream else None

    report = render_report(local, caps, upstream)
    if args.write:
        args.write.parent.mkdir(parents=True, exist_ok=True)
        args.write.write_text(report, encoding="utf-8", newline="\n")
        print(f"wrote {args.write}")
    else:
        print(report)

    local_inconsistent = bool(set(local) ^ caps)
    if local_inconsistent:
        print("FAIL: local capabilities disagree with specs-3.0.yaml", file=sys.stderr)
        return 1

    if upstream is not None and (
        set(local) != set(upstream)
        or any(
            local[op]["path"] != upstream[op]["path"]
            or local[op]["http_method"] != upstream[op]["http_method"]
            for op in set(local) & set(upstream)
        )
    ):
        print(
            "UPSTREAM_DRIFT: local pin differs from upstream snapshot "
            "(exit 2; review before regenerating).",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
