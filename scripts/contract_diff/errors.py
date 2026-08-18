"""HTTP error / response contract diffs."""

from __future__ import annotations

from typing import Any


def diff_responses(op_id: str, old: dict[str, Any], new: dict[str, Any]) -> list[dict[str, Any]]:
    changes: list[dict[str, Any]] = []
    ok = set(map(str, (old or {}).keys()))
    nk = set(map(str, (new or {}).keys()))
    for code in sorted(nk - ok):
        kind = "ErrorContractChange" if code.startswith(("4", "5")) else "ResponseStatusAdded"
        changes.append(
            {
                "kind": kind,
                "path": f"{op_id}.responses.{code}",
                "old": None,
                "new": code,
            }
        )
    for code in sorted(ok - nk):
        changes.append(
            {
                "kind": "ErrorContractChange",
                "path": f"{op_id}.responses.{code}",
                "old": code,
                "new": None,
            }
        )
    return changes
