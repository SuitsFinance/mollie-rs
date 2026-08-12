#!/usr/bin/env python3
"""Rebuild docs/example-support-matrix.md from logs/*.log (offline, no API calls).

The same matrix is also rewritten automatically by examples/support/mod.rs after
every successful append to logs/<example>.log. Use this script when you want to
refresh the roll-up without re-running examples (e.g. after pruning log files).
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


ACCESS_TOKEN_PROFILE_RESTRICTED_KEY = "ACCESS_TOKEN_PROFILE_RESTRICTED"
ACCESS_TOKEN_PROFILE_RESTRICTED_LABEL = "access-token-not-profile-restricted"

OUTCOME_ORDER = {
    "failed": 0,
    "supported": 1,
    "skipped": 2,
    "untested": 3,
}


@dataclass
class Row:
    example: str
    route: str
    outcome: str
    status: str
    code: str
    key: str
    label: str
    summary: str
    updated: str
    log_rel: str


def extract_const_str(source: str, const_name: str) -> str | None:
    needle = f"const {const_name}:"
    for line in source.splitlines():
        if needle not in line:
            continue
        if "=" not in line:
            continue
        after = line.split("=", 1)[1].strip()
        match = re.search(r'"([^"]*)"', after)
        if match:
            return match.group(1)
    return None


def discover_examples(root: Path) -> list[tuple[str, str]]:
    examples_dir = root / "examples"
    out: list[tuple[str, str]] = []
    for path in sorted(examples_dir.glob("*.rs")):
        source = path.read_text(encoding="utf-8")
        if "impl RunnableExample" not in source:
            continue
        name = extract_const_str(source, "NAME") or path.stem
        route = extract_const_str(source, "ROUTE") or "—"
        out.append((name, route))
    return out


def parse_latest_log(example: str, content: str) -> Row | None:
    blocks = [b for b in content.split("========== ") if b.strip()]
    if not blocks:
        return None
    last = blocks[-1]
    if " ==========" not in last:
        return None
    header, rest = last.split(" ==========", 1)
    updated = header.strip()
    body = rest.lstrip("\n")

    route = status = code = key = summary = kind = ""
    for line in body.splitlines():
        if line.startswith("example:"):
            continue
        if not kind and (
            line.startswith("OK ")
            or line.startswith("ERROR ")
            or line.startswith("SKIP ")
        ):
            kind = line
            continue
        if line.startswith("route: "):
            route = line[len("route: ") :].strip()
        elif line.startswith("status: "):
            status = line[len("status: ") :].strip()
        elif line.startswith("code: "):
            code = line[len("code: ") :].strip()
        elif line.startswith("key: "):
            key = line[len("key: ") :].strip()
        elif line.startswith("summary: "):
            summary = line[len("summary: ") :].strip()
        elif line.startswith("body:"):
            break
        elif not summary and line.startswith("error: "):
            summary = line[len("error: ") :].strip()

    if kind.startswith("OK "):
        outcome = "supported"
    elif kind.startswith("SKIP "):
        outcome = "skipped"
    elif kind.startswith("ERROR "):
        outcome = "failed"
    elif "OK response" in body or "OK envelope" in body:
        outcome = "supported"
    elif "SKIP " in body:
        outcome = "skipped"
    elif "ERROR " in body:
        outcome = "failed"
    else:
        outcome = "untested"

    if not summary:
        summary = kind or "see log"

    return Row(
        example=example,
        route=route or "-",
        outcome=outcome,
        status=status or "-",
        code=code or "-",
        key=key or "-",
        label=(
            ACCESS_TOKEN_PROFILE_RESTRICTED_LABEL
            if key == ACCESS_TOKEN_PROFILE_RESTRICTED_KEY
            else "-"
        ),
        summary=summary,
        updated=updated or "-",
        log_rel=f"logs/{example}.log",
    )


def md_cell(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ").replace("\r", "")


def render(rows: list[Row]) -> str:
    supported = sum(1 for r in rows if r.outcome == "supported")
    failed = sum(1 for r in rows if r.outcome == "failed")
    skipped = sum(1 for r in rows if r.outcome == "skipped")
    untested = sum(1 for r in rows if r.outcome == "untested")

    lines = [
        "# Example support matrix",
        "",
        "Auto-generated from the **latest** entry in each `logs/<example>.log` file whenever a route example runs (`examples/support/mod.rs`).",
        "",
        "Do not edit by hand — re-run examples (or delete a log and re-run) to refresh a row.",
        "",
        "Offline rebuild (no API calls):",
        "",
        "```sh",
        "python scripts/rebuild_example_support_matrix.py",
        "```",
        "",
        "## How to read this",
        "",
        "| Support | Meaning |",
        "| --- | --- |",
        "| `supported` | Last run logged `OK response` / `OK envelope` (HTTP success decoded). |",
        "| `failed` | Last run logged `ERROR …` (API error, decode error, or client failure). |",
        "| `skipped` | Missing credentials; example did not call Mollie. |",
        "| `untested` | No `logs/<example>.log` yet (or unparseable). |",
        "",
        "| Label | Meaning |",
        "| --- | --- |",
        "| `access-token-not-profile-restricted` | The endpoint requires an access token that is not restricted to a specific profile. |",
        "",
        f"**Totals:** {len(rows)} examples — **{supported}** supported, **{failed}** failed, **{skipped}** skipped, **{untested}** untested.",
        "",
        "Detail and full bodies stay in the per-example log; this table is the roll-up.",
        "",
        "## Matrix",
        "",
        "| Example | Route | Support | HTTP | Code | Key | Label | Summary | Log | Updated |",
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |",
    ]

    for row in rows:
        lines.append(
            "| `{ex}` | `{route}` | `{outcome}` | {status} | {code} | {key} | {label} | {summary} | `{log}` | {updated} |".format(
                ex=md_cell(row.example),
                route=md_cell(row.route),
                outcome=row.outcome,
                status=md_cell(row.status),
                code=md_cell(row.code),
                key=md_cell(row.key),
                label=md_cell(row.label),
                summary=md_cell(row.summary),
                log=md_cell(row.log_rel),
                updated=md_cell(row.updated),
            )
        )

    lines.append("")
    return "\n".join(lines) + "\n"


def collect(root: Path) -> list[Row]:
    by_name: dict[str, Row] = {}
    for name, route in discover_examples(root):
        by_name[name] = Row(
            example=name,
            route=route,
            outcome="untested",
            status="-",
            code="-",
            key="-",
            label="-",
            summary="no log yet",
            updated="-",
            log_rel="-",
        )

    logs_dir = root / "logs"
    if logs_dir.is_dir():
        for path in sorted(logs_dir.glob("*.log")):
            content = path.read_text(encoding="utf-8")
            parsed = parse_latest_log(path.stem, content)
            if parsed is None:
                continue
            existing = by_name.get(path.stem)
            if existing is not None and (not parsed.route or parsed.route == "-"):
                parsed.route = existing.route
            by_name[path.stem] = parsed

    rows = list(by_name.values())
    rows.sort(key=lambda r: (OUTCOME_ORDER.get(r.outcome, 9), r.example))
    return rows


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="Crate root (default: parent of scripts/)",
    )
    args = parser.parse_args()
    root = args.root.resolve()
    rows = collect(root)
    out_path = root / "docs" / "example-support-matrix.md"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(render(rows), encoding="utf-8", newline="\n")
    print(f"Wrote {out_path.relative_to(root)} ({len(rows)} rows)")


if __name__ == "__main__":
    main()
