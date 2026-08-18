#!/usr/bin/env python3
"""Fail-closed gate: required Tier-S workflows map to existing example crates (EX-001).

Usage:
  python scripts/check_workflow_examples.py
  python scripts/check_workflow_examples.py --write-md   # refresh docs/rc/workflow-example-matrix.md
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover
    print("PyYAML is required (pip install pyyaml)", file=sys.stderr)
    sys.exit(2)

ROOT = Path(__file__).resolve().parents[1]
REG = ROOT / "docs" / "registries" / "tier-s-workflow-examples.yaml"
MD = ROOT / "docs" / "rc" / "workflow-example-matrix.md"


def load_registry() -> dict:
    data = yaml.safe_load(REG.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or "required_workflows" not in data:
        raise SystemExit(f"invalid registry: {REG}")
    return data


def check(data: dict) -> list[str]:
    examples_dir = ROOT / data.get("examples_dir", "examples")
    errors: list[str] = []
    seen_ids: set[str] = set()
    for wf in data["required_workflows"]:
        wid = wf.get("id")
        if not wid:
            errors.append("workflow missing id")
            continue
        if wid in seen_ids:
            errors.append(f"duplicate workflow id: {wid}")
        seen_ids.add(wid)
        exs = wf.get("examples") or []
        if not exs:
            errors.append(f"{wid}: no examples listed")
        for name in exs:
            path = examples_dir / name
            if not path.is_file():
                errors.append(f"{wid}: missing example {name}")
        for guide in wf.get("guides") or []:
            gpath = ROOT / guide
            if not gpath.is_file():
                errors.append(f"{wid}: missing guide {guide}")
    return errors


def render_md(data: dict) -> str:
    lines = [
        "# Tier-S workflow → example matrix (EX-001)",
        "",
        "Machine source: [`docs/registries/tier-s-workflow-examples.yaml`](../registries/tier-s-workflow-examples.yaml).",
        "Gate: `python scripts/check_workflow_examples.py` (also CI contracts job).",
        "",
        "Examples under `examples/*.rs` are **generated** (`scripts/route_examples.py`); do not hand-edit them.",
        "This matrix asserts required money-path / Tier-S workflows keep at least one compile-checked example",
        "(CI also runs `cargo check --examples --all-features`).",
        "",
        "| Workflow | Title | Example crate(s) | Guide(s) |",
        "| --- | --- | --- | --- |",
    ]
    for wf in data["required_workflows"]:
        ex = ", ".join(f"`{e}`" for e in (wf.get("examples") or []))
        guides = ", ".join(f"`{g}`" for g in (wf.get("guides") or [])) or "—"
        lines.append(f"| `{wf['id']}` | {wf.get('title', '')} | {ex} | {guides} |")
    lines.extend(
        [
            "",
            "## Residual notes",
            "",
            "- Stream APIs (`stream_pages` / `stream_items`) are exercised in unit tests and documented in",
            "  [`pagination.md`](../guides/pagination.md); list examples cover the page entry point.",
            "- Webhook **signature verification** is covered by `docs/guides/handle-signed-webhook.md` and",
            "  library tests (`VerifiedWebhook`); route examples cover webhook CRUD/test endpoints.",
            "- Multi-merchant / OAuth app helpers: see `docs/guides/oauth-connect.md` and `multi-merchant.md`.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--write-md",
        action="store_true",
        help="rewrite docs/rc/workflow-example-matrix.md from the registry",
    )
    args = ap.parse_args()
    if not REG.is_file():
        print(f"missing registry {REG}", file=sys.stderr)
        return 1
    data = load_registry()
    if args.write_md:
        MD.parent.mkdir(parents=True, exist_ok=True)
        MD.write_text(render_md(data), encoding="utf-8")
        print(f"wrote {MD.relative_to(ROOT)}")
    errors = check(data)
    if errors:
        print("workflow example matrix FAILED:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    n = len(data["required_workflows"])
    print(f"workflow example matrix OK ({n} workflows)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
