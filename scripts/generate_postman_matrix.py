#!/usr/bin/env python3
"""Regenerate docs/postman-response-matrix.md from harvested Postman fixtures.

Does not truncate detail strings. Uses plain strings only (no accidental escapes).
"""

from __future__ import annotations

import json
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ERRORS = ROOT / "tests" / "fixtures" / "postman_error_responses.json"
SUCCESS = ROOT / "tests" / "fixtures" / "postman_success_response_index.json"
OUT = ROOT / "docs" / "postman-response-matrix.md"


def md_escape(s: str) -> str:
    return (s or "").replace("|", "\\|").replace("\n", " ").replace("\r", "")


def family(status: int, detail: str | None) -> str:
    d = (detail or "").lower()
    if status == 400:
        return "INVALID_CURSOR / factory::invalid_cursor"
    if status == 403:
        if "profile limit" in d:
            return "DEMO_PROFILE_LIMIT_REACHED"
        if "cannot be edited" in d:
            return "DEMO_PROFILE_NOT_EDITABLE"
        return "FORBIDDEN family"
    if status == 404:
        return "ENTITY_NOT_FOUND / factory::entity_not_found"
    if status == 409:
        return "PAYOUT_NOT_CANCELABLE / factory::payout_not_cancelable"
    if status == 410:
        return "PROFILE_DELETED / factory::profile_deleted"
    if status == 422:
        if any(
            x in d
            for x in (
                "already deleted",
                "not allowed",
                "cannot be cancelled",
                "cannot be canceled",
                "cannot be updated",
                "cannot be deleted",
            )
        ):
            return "RESOURCE_STATE_CONFLICT / factory::resource_state_conflict"
        return "VALIDATION_ERROR / factory::validation_error"
    if status == 429:
        return "RATE_LIMIT_EXCEEDED / factory::rate_limit_exceeded (global)"
    if status == 503:
        return "SERVICE_TEMPORARILY_UNAVAILABLE / factory::service_temporarily_unavailable"
    return "status fallback"


def main() -> None:
    errors = json.loads(ERRORS.read_text(encoding="utf-8"))
    success = json.loads(SUCCESS.read_text(encoding="utf-8"))

    lines: list[str] = [
        "# Postman response matrix",
        "",
        "Generated from six Mollie Postman collections. The collections themselves are",
        "copyright Mollie B.V. and are **not** redistributed in this repository (see",
        "[`NOTICE`](../NOTICE)); only the deduplicated response fixtures below are kept.",
        "",
        f"- **Unique error bodies:** {len(errors)} (full HAL in `tests/fixtures/postman_error_responses.json`)",
        f"- **Unique success shapes:** {len(success)} (index in `tests/fixtures/postman_success_response_index.json`)",
        "",
        "Every unique **error** body is exercised by `tests/postman_all_responses.rs` through the shared",
        "error factory / catalog / envelope (`ok: false`, code, key, message_key, title, detail, documentation).",
        "",
        "Global **429** uses a single factory: `factory::rate_limit_exceeded()` / `MollieError::rate_limit_exceeded()`,",
        "including when returned from `list_clients` (`GET /clients`), `list_capabilities`, and every other route.",
        "",
        "## Unique error bodies",
        "",
        "Full `detail` text is kept (**not truncated**). Prefer the JSON fixture for the complete HAL `_links` object.",
        "",
        "| Status | Title | Detail | Example routes | Catalog / factory |",
        "| --- | --- | --- | --- | --- |",
    ]

    for e in sorted(errors, key=lambda x: (x["status"], x.get("detail") or "")):
        routes = e.get("routes") or []
        paths: list[str] = []
        seen: set[str] = set()
        for r in routes:
            p = f"{r['method']} {r['path']}"
            if p not in seen:
                seen.add(p)
                paths.append(p)
            if len(paths) >= 4:
                break
        path_s = "<br>".join(f"`{md_escape(p)}`" for p in paths) or "—"
        detail = md_escape(e.get("detail") or "")
        title = md_escape(e.get("title") or "")
        lines.append(
            f"| {e['status']} | {title} | {detail} | {path_s} | {family(e['status'], e.get('detail'))} |"
        )

    lines.extend(
        [
            "",
            "## Success responses (index)",
            "",
            "Success samples are indexed by method/path/status/top-level keys (not full bodies).",
            "Typed success uses `ResponseEnvelope<T>` + `to_success_envelope()` on existing route methods.",
            "",
            "| Collection | Unique success shapes |",
            "| --- | ---: |",
        ]
    )

    by_coll: dict[str, int] = defaultdict(int)
    for s in success:
        by_coll[s["collection"]] += 1
    for c, n in sorted(by_coll.items()):
        lines.append(f"| {md_escape(c)} | {n} |")

    lines.extend(
        [
            "",
            "### Success shapes by status",
            "",
            "| HTTP | Count |",
            "| ---: | ---: |",
        ]
    )
    by_status: dict[int, int] = defaultdict(int)
    for s in success:
        by_status[int(s["status"])] += 1
    for st, n in sorted(by_status.items()):
        lines.append(f"| {st} | {n} |")

    lines.append("")
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    text = OUT.read_text(encoding="utf-8")
    if "\tests" in text or "\x0cactory" in text or "\tests" in repr(text):
        raise SystemExit("escape corruption detected in matrix output")
    if "ests/fixtures" in text and "tests/fixtures" not in text:
        raise SystemExit("path corruption: missing tests/ prefix")
    print(f"wrote {OUT.relative_to(ROOT)} ({len(lines)} lines)")


if __name__ == "__main__":
    main()
