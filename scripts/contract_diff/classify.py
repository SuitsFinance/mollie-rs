"""Risk classification for contract changes."""

from __future__ import annotations

from typing import Any

# kind -> base risk
RISK_TABLE: dict[str, int] = {
    "DocsOnly": 0,
    "AdditiveResponseField": 2,
    "AdditiveRequestOrResponseField": 3,
    "AdditiveResponseEnum": 2,
    "AdditiveRequestField": 4,
    "NullableRelaxation": 3,
    "NullableRestriction": 7,
    "RequirednessChange": 8,
    "TypeChange": 8,
    "MoneyChange": 9,
    "SchemaAdded": 3,
    "SchemaRemoved": 7,
    "SchemaReplacement": 8,
    "FieldRemoved": 7,
    "EnumRemoved": 8,
    "EnumValueRemoved": 8,
    "OperationAdded": 4,
    "OperationRemoved": 9,
    "MutationAdded": 8,
    "AuthChange": 9,
    "IdempotencyChange": 8,
    "TestmodeChange": 7,
    "ErrorContractChange": 4,
    "ResponseStatusAdded": 3,
    "MaturityChange": 5,
}

BLOCKING_THRESHOLD = 7


def classify_change(change: dict[str, Any]) -> dict[str, Any]:
    kind = str(change.get("kind") or "DocsOnly")
    risk = int(change.get("risk") or RISK_TABLE.get(kind, 5))
    # Requiredness true->false is relaxation (lower); false->true is restriction
    if kind == "RequirednessChange" and change.get("old") is True and change.get("new") is False:
        risk = 4
    out = dict(change)
    out["risk"] = risk
    out["blocking"] = risk >= BLOCKING_THRESHOLD
    return out
