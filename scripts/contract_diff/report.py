"""Report rendering."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def to_json(report: dict[str, Any], path: str | Path) -> None:
    out = Path(path)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def to_markdown(report: dict[str, Any], path: str | Path) -> None:
    lines = [
        "# Contract diff report",
        "",
        f"- Changes: **{len(report.get('changes', []))}**",
        f"- Blocking: **{report.get('blocking_count', 0)}**",
        f"- Max risk: **{report.get('max_risk', 0)}**",
        "",
        "| Kind | Path | Risk | Blocking |",
        "| --- | --- | ---: | --- |",
    ]
    for c in report.get("changes", []):
        lines.append(
            f"| `{c.get('kind')}` | `{c.get('path')}` | {c.get('risk')} | {c.get('blocking')} |"
        )
    lines.append("")
    out = Path(path)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(lines), encoding="utf-8")
