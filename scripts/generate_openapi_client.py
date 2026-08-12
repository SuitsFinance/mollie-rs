#!/usr/bin/env python3
"""Regenerate the checked-in Mollie OpenAPI client source."""

from __future__ import annotations

import argparse
import copy
import json
import re
import shutil
import subprocess
from pathlib import Path
from typing import Any

import yaml


PROGENITOR_VERSION = "0.11.2"
OPENAPIV3_VERSION = "2.2"

# Mollie documents Balances, Settlements, and Invoices as business-operation
# APIs. They are live-only and must reject sticky testmode before any request
# is sent. Payouts are the documented business-operation exception.
NON_TESTMODE_OPERATIONS = {
    "list_balances",
    "get_balance",
    "get_primary_balance",
    "get_balance_report",
    "list_balance_transactions",
    "list_settlements",
    "get_settlement",
    "get_open_settlement",
    "get_next_settlement",
    "list_settlement_payments",
    "list_settlement_captures",
    "list_settlement_refunds",
    "list_settlement_chargebacks",
    "list_invoices",
    "get_invoice",
    # Business-account / treasury reads are live organization reporting surfaces.
    "list_business_accounts",
    "get_business_account",
    "list_business_account_transactions",
    "get_business_account_transaction",
    "list_payouts",
    "get_payout",
    "list_unmatched_credit_transfers",
    "get_unmatched_credit_transfer",
}

# Matches a rustdoc JSON schema fence produced by progenitor/typify.
JSON_SCHEMA_BLOCK_RE = re.compile(
    r"(/// <details><summary>JSON schema</summary>\n"
    r"///\n"
    r"/// ```json\n)"
    r"(.*?)"
    r"(/// ```\n"
    r"/// </details>)",
    re.DOTALL,
)

MODULE_METHODS: dict[str, tuple[str, ...]] = {
    "balances": (
        "list_balances",
        "get_balance",
        "get_primary_balance",
        "get_balance_report",
        "list_balance_transactions",
    ),
    "settlements": (
        "list_settlements",
        "get_settlement",
        "get_open_settlement",
        "get_next_settlement",
        "list_settlement_payments",
        "list_settlement_captures",
        "list_settlement_refunds",
        "list_settlement_chargebacks",
    ),
    "invoices": ("list_invoices", "get_invoice"),
    "permissions": ("list_permissions", "get_permission"),
    "organizations": ("get_organization", "get_current_organization", "get_partner_status"),
    "profiles": (
        "list_profiles",
        "create_profile",
        "get_profile",
        "delete_profile",
        "update_profile",
        "get_current_profile",
    ),
    "onboarding": ("get_onboarding_status", "submit_onboarding_data"),
    "capabilities": ("list_capabilities",),
    "clients": ("list_clients", "get_client", "create_client_link"),
    "webhooks": (
        "list_webhooks",
        "create_webhook",
        "get_webhook",
        "delete_webhook",
        "update_webhook",
        "test_webhook",
        "get_webhook_event",
    ),
    "connect": (
        "list_connect_balance_transfers",
        "create_connect_balance_transfer",
        "get_connect_balance_transfer",
    ),
    "payments": (
        "list_payments",
        "create_payment",
        "get_payment",
        "cancel_payment",
        "update_payment",
        "release_authorization",
        "payment_list_routes",
        "payment_create_route",
        "payment_get_route",
    ),
    "methods": (
        "list_methods",
        "list_all_methods",
        "get_method",
        "enable_method",
        "disable_method",
        "enable_method_issuer",
        "disable_method_issuer",
    ),
    "refunds": (
        "list_refunds",
        "create_refund",
        "get_refund",
        "cancel_refund",
        "list_all_refunds",
    ),
    "chargebacks": ("list_chargebacks", "get_chargeback", "list_all_chargebacks"),
    "captures": ("list_captures", "create_capture", "get_capture"),
    "wallets": ("request_apple_pay_payment_session",),
    "payment_links": (
        "list_payment_links",
        "create_payment_link",
        "get_payment_link",
        "delete_payment_link",
        "update_payment_link",
        "get_payment_link_payments",
    ),
    "terminals": (
        "list_terminals",
        "get_terminal",
        "terminals_list_pairing_codes",
        "terminals_request_pairing_code",
        "terminals_get_pairing_code",
        "terminals_revoke_pairing_code",
    ),
    "accounts": (
        "list_business_accounts",
        "get_business_account",
        "list_business_account_transactions",
        "get_business_account_transaction",
    ),
    "payouts": (
        "list_payouts",
        "create_payout",
        "get_payout",
        "cancel_payout",
    ),
    "transfers": (
        "create_transfer",
        "get_transfer",
    ),
    "sessions": (
        "create_session",
        "get_session",
    ),
    "unmatched_credit_transfers": (
        "list_unmatched_credit_transfers",
        "get_unmatched_credit_transfer",
        "match_unmatched_credit_transfer",
        "return_unmatched_credit_transfer",
    ),
    "verify_payee": ("verify_payee",),
    "oauth": (
        "oauth_generate_tokens",
        "oauth_revoke_tokens",
    ),
    "customers": (
        "list_customers",
        "create_customer",
        "get_customer",
        "delete_customer",
        "update_customer",
        "list_customer_payments",
        "create_customer_payment",
        "list_mandates",
        "create_mandate",
        "get_mandate",
        "revoke_mandate",
        "list_subscriptions",
        "create_subscription",
        "get_subscription",
        "cancel_subscription",
        "update_subscription",
        "list_all_subscriptions",
        "list_subscription_payments",
    ),
    "sales_invoices": (
        "list_sales_invoices",
        "create_sales_invoice",
        "get_sales_invoice",
        "delete_sales_invoice",
        "update_sales_invoice",
    ),
}


def run(command: list[str], cwd: Path) -> None:
    """Run a command and fail loudly on errors."""

    subprocess.run(command, cwd=cwd, check=True)


def windows_path_from_wsl(path: Path) -> str | None:
    """Convert a WSL /mnt/<drive> path to a Windows path when applicable."""

    parts = path.resolve().parts
    if len(parts) < 3 or parts[0] != "/" or parts[1] != "mnt" or len(parts[2]) != 1:
        return None

    drive = parts[2].upper()
    return drive + ":\\" + "\\".join(parts[3:])


def format_workspace(root: Path) -> None:
    """Format the workspace, with a native fallback for WSL-mounted Windows files."""

    try:
        run(["cargo", "fmt", "--all"], cwd=root)
        return
    except subprocess.CalledProcessError:
        windows_root = windows_path_from_wsl(root)
        if not windows_root or not shutil.which("powershell.exe"):
            raise

    run(
        [
            "powershell.exe",
            "-NoProfile",
            "-Command",
            f"Set-Location -LiteralPath '{windows_root}'; cargo fmt --all",
        ],
        cwd=root,
    )


def pascal_case(value: str) -> str:
    """Convert a snake_case operation id to a Rust enum variant."""

    return "".join(part.capitalize() for part in value.split("_"))


def split_top_level_args(source: str) -> list[str]:
    """Split a Rust macro argument list on top-level commas."""

    args: list[str] = []
    depth = 0
    start = 0
    in_string = False
    escape = False

    for index, char in enumerate(source):
        if in_string:
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            continue

        if char == '"':
            in_string = True
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            part = source[start:index].strip()
            if part:
                args.append(part)
            start = index + 1

    tail = source[start:].strip()
    if tail:
        args.append(tail)
    return args


def find_matching_paren(source: str, open_index: int) -> int:
    """Find the matching closing parenthesis for a Rust macro call."""

    depth = 0
    in_string = False
    escape = False

    for index in range(open_index, len(source)):
        char = source[index]
        if in_string:
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            continue

        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index

    raise ValueError("unclosed parenthesis")


def rewrite_url_building(block: str) -> str:
    """Move route URL construction behind Client::endpoint."""

    output: list[str] = []
    cursor = 0
    marker = "        let url = format!("

    while True:
        start = block.find(marker, cursor)
        if start == -1:
            output.append(block[cursor:])
            return "".join(output)

        output.append(block[cursor:start])
        open_index = block.find("(", start)
        close_index = find_matching_paren(block, open_index)
        end_index = close_index + 2
        if block[close_index + 1 : end_index] != ";":
            raise ValueError("unexpected url format terminator")

        args = split_top_level_args(block[open_index + 1 : close_index])
        if len(args) < 2 or args[1] != "self.baseurl":
            raise ValueError(f"unexpected url format args: {args}")

        route_format = args[0].strip()
        if not (route_format.startswith('"{}/') and route_format.endswith('"')):
            raise ValueError(f"unexpected route format string: {route_format}")

        route_format = '"' + route_format[3:]
        route_args = [arg for arg in args[2:] if arg]
        if route_args:
            replacement = (
                "        let url = self.endpoint(format_args!("
                + ", ".join([route_format, *route_args])
                + "));"
            )
        else:
            replacement = f"        let url = self.endpoint({route_format});"

        output.append(replacement)
        cursor = end_index


def normalize_doc_lines(lines: list[str]) -> list[str]:
    """Normalize generated rustdoc lines.

    Preserves relative indentation inside code fences / JSON schema blocks so
    struct documentation is not flattened or visually \"cut off\". Only the
    common leading indent shared by all non-empty lines is removed.
    """

    # Drop a single leading space from rustdoc-style \" * \" block lines if present.
    cleaned: list[str] = []
    for line in lines:
        if line.startswith(" *"):
            line = line[2:]
        elif line.startswith("*"):
            line = line[1:]
        cleaned.append(line.rstrip())

    nonempty = [line for line in cleaned if line.strip()]
    if nonempty:
        common = min(len(line) - len(line.lstrip(" ")) for line in nonempty)
    else:
        common = 0

    output: list[str] = []
    in_plain_fence = False

    for line in cleaned:
        if not line.strip():
            output.append("///")
            continue

        content = line[common:] if len(line) >= common else line.lstrip(" ")
        stripped = content.strip()
        if stripped == "```":
            if in_plain_fence:
                in_plain_fence = False
                content = "```"
            else:
                # Prefer language tags already present (```json); only bare fences
                # become ```text for rustdoc.
                content = "```text"
                in_plain_fence = True
        output.append(f"/// {content}" if content else "///")

    return output


def convert_doc_block(block: str) -> str:
    """Convert generated block rustdoc into line rustdoc."""

    match = re.match(r"(?s)^    /\*\*(.*?)\*/\n", block)
    if not match:
        return block

    docs = match.group(1).strip("\n").splitlines()
    lines = [f"    {doc}" for doc in normalize_doc_lines(docs)]

    return "\n".join(lines) + "\n" + block[match.end() :]


def convert_all_doc_blocks(source: str) -> str:
    """Convert all generated block rustdoc comments in a source fragment."""

    def replace(match: re.Match[str]) -> str:
        indent = match.group("indent")
        docs = match.group("body").strip("\n").splitlines()
        return "\n".join(f"{indent}{doc}" for doc in normalize_doc_lines(docs))

    return re.sub(
        r"(?ms)^(?P<indent>\s*)/\*\*(?P<body>.*?)\*/",
        replace,
        source,
    )


def strip_idempotency_key_param(block: str) -> str:
    """Remove OpenAPI idempotency-key from method signatures and docs.

    The key is owned on [`Client`] and resolved inside `Client::request`, so it
    must not appear as a per-call parameter (avoids lifetime coupling with bodies).
    """

    # Drop rustdoc argument lines for the header parameter.
    block = re.sub(
        r"    /// - `idempotency_key`:.*\n(?:    /// .*\n)*",
        "",
        block,
    )
    # Drop the method parameter line only (keep commas on neighboring params).
    block = re.sub(
        r"\n        idempotency_key: Option<&'a str>,?",
        "",
        block,
    )
    # Collapse accidental double blank lines left after doc removal.
    block = re.sub(r"(\n    ///\n){2,}", "\n    ///\n", block)
    return block


def strip_testmode_param(block: str) -> str:
    """Remove OpenAPI testmode query from method signatures; bind from Client.

    Sticky `testmode` is owned on [`Client`] and only applied on operations that
    already document the query parameter.
    """

    # Drop the argument bullet and its continuation lines until the next bullet,
    # "Arguments:" section end, or blank doc line followed by `pub async`.
    block = re.sub(
        r"    /// - `testmode`:.*\n"
        r"(?:    /// (?!- ).*\n)*",
        "",
        block,
    )
    block = re.sub(
        r"\n        testmode: Option<bool>,?",
        "",
        block,
    )
    block = block.replace(
        '.query(&progenitor_client::QueryParam::new("testmode", &testmode))',
        '.query(&progenitor_client::QueryParam::new("testmode", &self.testmode()))',
    )
    block = re.sub(r"(\n    ///\n){2,}", "\n    ///\n", block)
    return block


def rewrite_request_building(block: str) -> str:
    """Move generated header construction behind Client::request."""

    block = re.sub(
        r"        let mut header_map = ::reqwest::header::HeaderMap::with_capacity\(2usize\);\n"
        r"(?s:.*?)"
        r"        #\[allow\(unused_mut\)\]\n",
        "        #[allow(unused_mut)]\n",
        block,
        count=1,
    )

    method_match = re.search(r"\n            \.client\n            \.(get|post|patch|delete)\(url\)", block)
    if not method_match:
        raise ValueError("request method not found")

    method = method_match.group(1).upper()
    block = re.sub(
        r"        let mut request = self\n"
        r"            \.client\n"
        r"            \.(get|post|patch|delete)\(url\)\n"
        r"            \.header\(\n"
        r"(?s:.*?)"
        r"            \)",
        (
            "        let (request, resolved_idempotency_key) = self\n"
            f"            .request(::reqwest::Method::{method}, url)?;\n"
            "        #[allow(unused_mut)]\n"
            "        let mut request = request"
        ),
        block,
        count=1,
    )
    block = block.replace("\n            .headers(header_map)", "")
    return block


def rewrite_send(block: str, operation_ids: list[str]) -> str:
    """Move hook metadata and execution behind Client::send."""

    match = re.search(
        r'        let info = OperationInfo \{\n            operation_id: "([A-Za-z_][A-Za-z0-9_]*)",\n        \};\n'
        r"        self\.pre\(&mut request, &info\)\.await\?;\n"
        r"        let result = self\.exec\(request, &info\)\.await;\n"
        r"        self\.post\(&result, &info\)\.await\?;\n"
        r"        let response = result\?;",
        block,
    )
    if not match:
        raise ValueError("operation execution block not found")

    operation = match.group(1)
    if operation not in operation_ids:
        operation_ids.append(operation)

    send = (
        f"        let response = self.send(request, routes::Operation::{pascal_case(operation)}).await?;"
    )
    if operation in NON_TESTMODE_OPERATIONS:
        send = f'        self.reject_testmode_for("{operation}")?;\n' + send

    return block[: match.start()] + send + block[match.end() :]


def rewrite_response(block: str) -> str:
    """Move status response handling behind routes::response."""

    match = re.search(r"        match response\.status\(\)\.as_u16\(\) \{\n(?s:(.*?))\n        \}", block)
    if not match:
        raise ValueError("response match not found")

    body = match.group(1)
    # progenitor 0.11 emits several success shapes depending on content type:
    # - ResponseValue::from_response(response).await
    # - Ok(ResponseValue::from_response(response).await)
    # - Ok(ResponseValue::stream(response))  (when schema typing failed)
    # - Ok(ResponseValue::empty(response))   (204)
    success_codes = re.findall(
        r"^            (\d+)u16 => (?:Ok\()?"
        r"(?:ResponseValue::from_response\(response\)\.await"
        r"|ResponseValue::stream\(response\)"
        r"|ResponseValue::empty\(response\))"
        r",?\)?,",
        body,
        flags=re.MULTILINE,
    )
    error_codes = re.findall(
        r"^            (\d+)u16 => (?:\{\n                )?"
        r"Err\(Error::ErrorResponse",
        body,
        flags=re.MULTILINE,
    )
    if not success_codes:
        raise ValueError("response match has no success status")

    success = ", ".join(f"{code}u16" for code in success_codes)
    errors = ", ".join(f"{code}u16" for code in error_codes)
    replacement = (
        f"        routes::response::json(response, &[{success}], &[{errors}], "
        f"&resolved_idempotency_key).await"
    )
    return block[: match.start()] + replacement + block[match.end() :]


def rewrite_amount_deep_object_query(block: str) -> str:
    """Serialize amount query params as OpenAPI deepObject form.

    progenitor ``QueryParam`` only prefixes scalar values. Struct serialization
    drops the parameter name and emits bare ``currency``/``value``, which Mollie
    rejects with a suggestion to use ``amount.currency`` / ``amount.value``.

    Only ``GET /methods`` and ``GET /methods/all`` use ``style: deepObject`` for
    the amount query parameter in the checked-in Mollie OpenAPI document.
    """

    needle = '.query(&progenitor_client::QueryParam::new("amount", &amount))'
    if needle not in block:
        return block

    replacement = (
        ".query(&progenitor_client::QueryParam::new(\n"
        '                "amount[currency]",\n'
        "                &amount_currency,\n"
        "            ))\n"
        "            .query(&progenitor_client::QueryParam::new(\n"
        '                "amount[value]",\n'
        "                &amount_value,\n"
        "            ))"
    )
    block = block.replace(needle, replacement)

    locals_block = (
        "        // OpenAPI `style: deepObject` for `amount` — progenitor QueryParam drops the\n"
        "        // parameter name for structs and would emit bare `currency`/`value`.\n"
        "        let amount_currency = amount.map(|amount| amount.currency.as_str());\n"
        "        let amount_value = amount.map(|amount| amount.value.as_str());\n"
    )
    marker = "        #[allow(unused_mut)]\n        let mut request = request\n"
    if locals_block not in block and marker in block:
        block = block.replace(marker, locals_block + marker, 1)
    return block


def rewrite_route_block(block: str, operation_ids: list[str]) -> str:
    """Apply all route method normalizations."""

    block = convert_doc_block(block)
    block = strip_idempotency_key_param(block)
    block = strip_testmode_param(block)
    block = rewrite_url_building(block)
    block = rewrite_request_building(block)
    block = rewrite_amount_deep_object_query(block)
    block = rewrite_send(block, operation_ids)
    block = rewrite_response(block)
    # Non-success responses always decode as Mollie's HAL error body, including
    # global 403/429 that the OpenAPI operation may not list.
    block = block.replace("Error<()>", "Error<types::ErrorResponse>")
    return block.strip("\n") + "\n"


def load_openapi_component_schemas(spec_path: Path) -> dict[str, Any]:
    """Load `components.schemas` from an OpenAPI 3 document."""

    document = yaml.safe_load(spec_path.read_text(encoding="utf-8"))
    schemas = document.get("components", {}).get("schemas", {})
    if not isinstance(schemas, dict):
        raise ValueError(f"OpenAPI components.schemas missing in {spec_path}")
    return schemas


def validate_write_schema_contract(components: dict[str, Any]) -> None:
    """Reject response-shaped fields in the generated Mollie write models."""

    forbidden = {
        "resource",
        "id",
        "status",
        "createdAt",
        "updatedAt",
        "canceledAt",
        "nextPaymentDate",
        "timesRemaining",
        "_links",
    }
    expected = {
        "create-payment-request",
        "create-refund-request",
        "create-subscription-request",
    }
    missing = sorted(expected - components.keys())
    if missing:
        raise ValueError(f"missing generated Mollie write schemas: {missing}")

    for name in sorted(expected):
        schema = components[name]
        properties = schema.get("properties", {})
        if not isinstance(properties, dict):
            raise ValueError(f"{name} must define a properties object")
        invalid = sorted(forbidden.intersection(properties))
        if invalid:
            raise ValueError(
                f"{name} contains response-only fields: {', '.join(invalid)}"
            )


def schema_ref_name(ref: str) -> str | None:
    """Return the component schema name for a local `#/components/schemas/...` ref."""

    prefix = "#/components/schemas/"
    if not ref.startswith(prefix):
        return None
    name = ref[len(prefix) :]
    return name or None


def merge_schema_objects(base: dict[str, Any], overlay: dict[str, Any]) -> dict[str, Any]:
    """Merge two JSON Schema object nodes for documentation expansion."""

    merged = copy.deepcopy(base)
    for key, value in overlay.items():
        if key == "properties" and isinstance(value, dict):
            properties = merged.setdefault("properties", {})
            if not isinstance(properties, dict):
                properties = {}
                merged["properties"] = properties
            for prop_name, prop_schema in value.items():
                if prop_name in properties and isinstance(properties[prop_name], dict) and isinstance(prop_schema, dict):
                    properties[prop_name] = merge_schema_objects(properties[prop_name], prop_schema)
                else:
                    properties[prop_name] = copy.deepcopy(prop_schema)
        elif key == "required" and isinstance(value, list):
            required = list(merged.get("required", []))
            for item in value:
                if item not in required:
                    required.append(item)
            if required:
                merged["required"] = required
        elif key in {"allOf", "anyOf", "oneOf"} and isinstance(value, list):
            existing = merged.get(key)
            if isinstance(existing, list):
                merged[key] = existing + copy.deepcopy(value)
            else:
                merged[key] = copy.deepcopy(value)
        elif key not in merged:
            merged[key] = copy.deepcopy(value)
        elif isinstance(merged[key], dict) and isinstance(value, dict):
            merged[key] = merge_schema_objects(merged[key], value)
        else:
            merged[key] = copy.deepcopy(value)
    return merged


def collapse_all_of(schema: dict[str, Any]) -> dict[str, Any]:
    """Merge `allOf` members into a single object schema when possible."""

    members = schema.get("allOf")
    if not isinstance(members, list) or not members:
        return schema

    merged: dict[str, Any] = {key: copy.deepcopy(value) for key, value in schema.items() if key != "allOf"}
    pending: list[Any] = []
    for member in members:
        if not isinstance(member, dict):
            pending.append(member)
            continue
        if any(key in member for key in ("anyOf", "oneOf", "not", "if", "then", "else")):
            pending.append(member)
            continue
        nested_all_of = member.get("allOf")
        if isinstance(nested_all_of, list):
            collapsed = collapse_all_of(member)
            if "allOf" in collapsed:
                pending.append(collapsed)
                continue
            member = collapsed
        merged = merge_schema_objects(merged, member)

    if pending:
        merged["allOf"] = pending
    return merged


def expand_schema(
    node: Any,
    components: dict[str, Any],
    stack: set[str] | None = None,
) -> Any:
    """Recursively expand local `$ref` values using OpenAPI component schemas."""

    if stack is None:
        stack = set()

    if isinstance(node, list):
        return [expand_schema(item, components, stack) for item in node]

    if not isinstance(node, dict):
        return copy.deepcopy(node)

    if "$ref" in node and isinstance(node["$ref"], str):
        ref = node["$ref"]
        name = schema_ref_name(ref)
        siblings = {key: value for key, value in node.items() if key != "$ref"}
        if name is None or name not in components:
            expanded: dict[str, Any] = {"$ref": ref}
            if siblings:
                expanded.update(expand_schema(siblings, components, stack))
            return expanded
        if name in stack:
            # Preserve a cycle edge instead of infinite recursion.
            expanded = {"$ref": ref}
            if siblings:
                expanded.update(expand_schema(siblings, components, stack))
            return expanded

        stack.add(name)
        try:
            target = expand_schema(components[name], components, stack)
        finally:
            stack.remove(name)

        if siblings:
            sibling_expanded = expand_schema(siblings, components, stack)
            if isinstance(target, dict) and isinstance(sibling_expanded, dict):
                target = merge_schema_objects(target, sibling_expanded)
            elif isinstance(target, dict):
                target = {**target, "x-ref-siblings": sibling_expanded}
            else:
                target = {"allOf": [target, sibling_expanded]}
        if isinstance(target, dict):
            target = collapse_all_of(target)
        return target

    expanded_obj = {
        key: expand_schema(value, components, stack) for key, value in node.items()
    }
    if isinstance(expanded_obj, dict):
        expanded_obj = collapse_all_of(expanded_obj)
    return expanded_obj


def rustdoc_json_lines(schema: Any) -> list[str]:
    """Pretty-print a schema as `///`-prefixed JSON lines for rustdoc."""

    pretty = json.dumps(schema, indent=2, ensure_ascii=False, sort_keys=False)
    return [f"///{'' if line == '' else ' ' + line}" for line in pretty.splitlines()]


def expand_json_schemas_in_types(source: str, components: dict[str, Any]) -> str:
    """Replace each type docstring schema fence with a fully expanded JSON schema."""

    def replace(match: re.Match[str]) -> str:
        prefix, body, suffix = match.group(1), match.group(2), match.group(3)
        json_text = "\n".join(
            line[4:] if line.startswith("/// ") else (line[3:] if line.startswith("///") else line)
            for line in body.splitlines()
        )
        try:
            schema = json.loads(json_text)
        except json.JSONDecodeError:
            # Leave progenitor output untouched when a fence is not valid JSON.
            return match.group(0)

        expanded = expand_schema(schema, components)
        rebuilt = "\n".join(rustdoc_json_lines(expanded))
        if not rebuilt.endswith("\n"):
            rebuilt += "\n"
        return prefix + rebuilt + suffix

    return JSON_SCHEMA_BLOCK_RE.sub(replace, source)


def extract_types(raw: str, components: dict[str, Any] | None = None) -> str:
    """Extract the generated types module body."""

    marker = "/// Types used as operation parameters and responses.\n#[allow(clippy::all)]\npub mod types {"
    start = raw.index(marker)
    open_index = raw.index("{", start + len("/// Types used"))
    close_marker = "\n}\n#[derive(Clone, Debug)]"
    close_index = raw.index(close_marker, open_index) + 1
    inner = raw[open_index + 1 : close_index]
    lines = [line[4:] if line.startswith("    ") else line for line in inner.splitlines()]
    body = convert_all_doc_blocks("\n".join(lines).strip())
    if components:
        body = expand_json_schemas_in_types(body, components)
    return "//! Types used as operation parameters and responses.\n\n#![allow(clippy::all)]\n\n" + body + "\n"


def extract_route_blocks(raw: str) -> dict[str, str]:
    """Extract generated route method blocks keyed by method name."""

    marker = "impl ClientHooks<()> for &Client {}\n#[allow(clippy::all)]\nimpl Client {\n"
    start = raw.index(marker) + len(marker)
    prelude = raw.find("\n/// Items consumers will typically use such as the Client.", start)
    body = raw[start:prelude] if prelude != -1 else raw[start:]
    close = body.rfind("\n}")
    if close == -1:
        raise ValueError("generated Client route impl close not found")
    body = body[:close]
    lines = body.splitlines(keepends=True)

    methods: list[tuple[int, int, str]] = []
    for index, line in enumerate(lines):
        match = re.match(r"^    pub async fn ([A-Za-z_][A-Za-z0-9_]*)<'a>\(", line)
        if match:
            start_line = index
            while start_line > 0 and not lines[start_line].startswith("    /**"):
                start_line -= 1
            methods.append((start_line, index, match.group(1)))

    blocks: dict[str, str] = {}
    for position, (start_line, _method_line, method_name) in enumerate(methods):
        end_line = methods[position + 1][0] if position + 1 < len(methods) else len(lines)
        blocks[method_name] = "".join(lines[start_line:end_line])

    return blocks


def write_routes(root: Path, blocks: dict[str, str]) -> None:
    """Write normalized route modules and central helpers."""

    routes_dir = root / "src" / "routes"
    routes_dir.mkdir(parents=True, exist_ok=True)

    expected = {name for names in MODULE_METHODS.values() for name in names}
    missing = sorted(expected - set(blocks))
    extra = sorted(set(blocks) - expected)
    if missing or extra:
        raise SystemExit(f"route method mapping drift: missing={missing} extra={extra}")

    operation_ids: list[str] = []
    rewritten = {
        name: rewrite_route_block(block, operation_ids)
        for name, block in blocks.items()
    }

    mod_lines = [
        "//! Generated Mollie API route groups.\n",
        "//!\n",
        "//! Each public module owns one operation area while the methods remain\n",
        "//! inherent methods on [`crate::Client`].\n",
        "\n",
        "mod operations;\n",
        "pub(crate) mod response;\n",
        "\n",
        "pub(crate) use operations::Operation;\n",
        "\n",
    ]
    mod_lines.extend(f"pub mod {module};\n" for module in MODULE_METHODS)
    (routes_dir / "mod.rs").write_text("".join(mod_lines), encoding="utf-8", newline="\n")

    operation_lines = [
        "//! Central operation metadata for generated route hooks.\n\n",
        "use progenitor_client::OperationInfo;\n\n",
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n",
        "pub(crate) enum Operation {\n",
    ]
    operation_lines.extend(f"    {pascal_case(operation)},\n" for operation in operation_ids)
    operation_lines.extend(
        [
            "}\n\n",
            "impl Operation {\n",
            "    /// Return the OpenAPI operation id for this route.\n",
            "    pub(crate) const fn id(self) -> &'static str {\n",
            "        match self {\n",
        ]
    )
    operation_lines.extend(
        f"            Self::{pascal_case(operation)} => \"{operation}\",\n"
        for operation in operation_ids
    )
    operation_lines.extend(
        [
            "        }\n",
            "    }\n\n",
            "    /// Build the hook metadata expected by progenitor-client.\n",
            "    pub(crate) fn info(self) -> OperationInfo {\n",
            "        OperationInfo {\n",
            "            operation_id: self.id(),\n",
            "        }\n",
            "    }\n",
            "}\n",
        ]
    )
    (routes_dir / "operations.rs").write_text("".join(operation_lines), encoding="utf-8", newline="\n")

    (routes_dir / "response.rs").write_text(
        """//! Shared response decoding for generated route methods.

use bytes::Bytes;
use crate::{types, Error, ResponseValue};
use reqwest::header::{HeaderMap, HeaderValue};

pub(crate) const IDEMPOTENCY_KEY_HEADER: &str = "idempotency-key";

/// Decode a response body while accepting the empty body of a `204 No Content` response.
#[allow(clippy::result_large_err)]
fn decode_response_body<T>(
    status: reqwest::StatusCode,
    headers: HeaderMap,
    body: Bytes,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let body = if status == reqwest::StatusCode::NO_CONTENT && body.is_empty() {
        Bytes::from_static(b"null")
    } else {
        body
    };
    let inner = serde_json::from_slice(&body)
        .map_err(|error| Error::InvalidResponsePayload(body, error))?;
    Ok(ResponseValue::new(inner, status, headers))
}

/// Read and decode a generated route response while retaining status and headers.
async fn response_value<T>(
    response: reqwest::Response,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.bytes().await.map_err(Error::ResponseBodyError)?;
    decode_response_body(status, headers, body)
}

/// Preserve a provider idempotency key or synthesize the resolved request key.
#[allow(clippy::result_large_err)]
fn ensure_idempotency_key_header(
    headers: &mut HeaderMap,
    idempotency_key: &str,
) -> Result<(), Error<types::ErrorResponse>> {
    if headers.contains_key(IDEMPOTENCY_KEY_HEADER) {
        return Ok(());
    }

    let header_value = HeaderValue::try_from(idempotency_key).map_err(|error| {
        Error::InvalidRequest(format!("invalid resolved idempotency key: {error}"))
    })?;
    headers.insert(IDEMPOTENCY_KEY_HEADER, header_value);
    Ok(())
}

/// Decode a JSON response according to the route's documented success codes.
///
/// Any non-success status is treated as a Mollie HAL error body
/// ([`types::ErrorResponse`]), including global statuses such as `403` and
/// `429` that are often omitted from per-operation OpenAPI responses.
pub(crate) async fn json<T>(
    response: reqwest::Response,
    success_statuses: &[u16],
    _documented_error_statuses: &[u16],
    idempotency_key: &str,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let mut response = response;
    ensure_idempotency_key_header(response.headers_mut(), idempotency_key)?;
    let status = response.status().as_u16();
    if success_statuses.contains(&status) {
        return response_value(response).await;
    }

    Err(Error::ErrorResponse(
        response_value(response).await?,
    ))
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{decode_response_body, ensure_idempotency_key_header, IDEMPOTENCY_KEY_HEADER};
    use crate::ResponseValueExt;

    /// Adds the resolved key when the provider omits the response header.
    #[test]
    fn synthesizes_missing_idempotency_key() {
        let mut headers = reqwest::header::HeaderMap::new();
        ensure_idempotency_key_header(&mut headers, "resolved-key").expect("valid key");
        let response = crate::ResponseValue::new(
            "ok",
            reqwest::StatusCode::OK,
            headers,
        );

        assert_eq!(response.idempotency_key(), Some("resolved-key"));
    }

    /// Does not replace an idempotency key echoed by the provider.
    #[test]
    fn preserves_echoed_idempotency_key() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            IDEMPOTENCY_KEY_HEADER,
            "provider-key".parse().expect("valid key"),
        );
        ensure_idempotency_key_header(&mut headers, "resolved-key").expect("valid key");
        let response = crate::ResponseValue::new(
            "ok",
            reqwest::StatusCode::OK,
            headers,
        );

        assert_eq!(response.idempotency_key(), Some("provider-key"));
    }

    /// Decodes a successful empty response as unit data for `204 No Content`.
    #[test]
    fn decodes_empty_no_content_response() {
        let response = decode_response_body::<()>(
            reqwest::StatusCode::NO_CONTENT,
            reqwest::header::HeaderMap::new(),
            Bytes::new(),
        )
        .expect("204 response should decode");

        assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
        assert_eq!(response.into_inner(), ());
    }
}
""",
        encoding="utf-8",
        newline="\n",
    )

    for module, names in MODULE_METHODS.items():
        body = "\n".join(rewritten[name] for name in names)
        imports = ["use crate::{routes, types, Client, Error, ResponseValue};"]
        if "encode_path(" in body:
            imports.insert(0, "use progenitor_client::encode_path;")

        area = module.replace("_", " ")
        text = (
            f"//! Generated {area} route methods.\n\n"
            + "\n".join(imports)
            + "\n\n"
            + f"/// Generated `{area}` route methods on [`crate::Client`].\n"
            + "///\n"
            + "/// Client-owned request policy on [`crate::Client`]:\n"
            + "/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)\n"
            + "/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it\n"
            + "///\n"
            + "/// The resolved idempotency key is returned on the response envelope\n"
            + "/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).\n"
            + "#[allow(clippy::all)]\nimpl Client {\n"
            + body
            + "}\n"
        )
        (routes_dir / f"{module}.rs").write_text(text, encoding="utf-8", newline="\n")


def update_api_version(root: Path, raw: str) -> None:
    """Keep the generated API version in the facade lib."""

    match = re.search(r'fn api_version\(\) -> &\'static str \{\n        "([^"]+)"\n    \}', raw)
    if not match:
        raise ValueError("generated api_version not found")
    version = match.group(1)

    lib_path = root / "src" / "lib.rs"
    lib = lib_path.read_text(encoding="utf-8")
    lib = re.sub(
        r'fn api_version\(\) -> &\'static str \{\n        "[^"]+"\n    \}',
        f'fn api_version() -> &\'static str {{\n        "{version}"\n    }}',
        lib,
        count=1,
    )
    lib_path.write_text(lib, encoding="utf-8", newline="\n")


def temp_manifest(root: Path) -> Path:
    """Create the temporary Cargo manifest used to run the generator source."""

    workdir = root / "target" / "openapi-generator"
    workdir.mkdir(parents=True, exist_ok=True)
    manifest = workdir / "Cargo.toml"
    script_path = (root / "scripts" / "openapi_generator.rs").as_posix()
    manifest.write_text(
        f"""[package]
name = "mollie-openapi-generator"
version = "0.1.0"
edition = "2021"
publish = false

[[bin]]
name = "mollie-openapi-generator"
path = "{script_path}"

[dependencies]
openapiv3 = "{OPENAPIV3_VERSION}"
prettyplease = "0.2"
progenitor = {{ version = "{PROGENITOR_VERSION}", default-features = false }}
serde_yaml = "0.9"
syn = "2"
""",
        encoding="utf-8",
        newline="\n",
    )
    return manifest


def generate_raw(root: Path, spec: Path, raw_output: Path) -> None:
    """Run the Rust OpenAPI generator."""

    raw_output.parent.mkdir(parents=True, exist_ok=True)
    manifest = temp_manifest(root)
    run(
        [
            "cargo",
            "run",
            "--manifest-path",
            str(manifest),
            "--",
            str(spec),
            str(raw_output),
        ],
        cwd=root,
    )


def normalize(root: Path, raw_output: Path, components: dict[str, Any]) -> None:
    """Normalize raw generated code into the SDK layout."""

    raw = raw_output.read_text(encoding="utf-8")
    (root / "src" / "types.rs").write_text(
        extract_types(raw, components),
        encoding="utf-8",
        newline="\n",
    )
    write_routes(root, extract_route_blocks(raw))
    update_api_version(root, raw)


def expand_types_doc_schemas(root: Path, spec: Path) -> None:
    """Expand `$ref` JSON schemas already present in `src/types.rs` docstrings."""

    types_path = root / "src" / "types.rs"
    components = load_openapi_component_schemas(spec)
    validate_write_schema_contract(components)
    source = types_path.read_text(encoding="utf-8")
    expanded = expand_json_schemas_in_types(source, components)
    types_path.write_text(expanded, encoding="utf-8", newline="\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--spec", type=Path, default=None)
    parser.add_argument("--raw-output", type=Path, default=None)
    parser.add_argument("--skip-route-examples", action="store_true")
    parser.add_argument("--skip-fmt", action="store_true")
    parser.add_argument(
        "--expand-doc-schemas-only",
        action="store_true",
        help="Only expand JSON schema blocks in src/types.rs from the OpenAPI components",
    )
    args = parser.parse_args()

    root = args.root.resolve()
    spec = (args.spec or root / "specs-3.0.yaml").resolve()
    raw_output = (args.raw_output or root / "target" / "openapi-client" / "raw-lib.rs").resolve()
    components = load_openapi_component_schemas(spec)

    if args.expand_doc_schemas_only:
        expand_types_doc_schemas(root, spec)
        if not args.skip_fmt:
            format_workspace(root)
        print(f"Expanded type docstring JSON schemas from {spec.relative_to(root)}")
        return

    if not shutil.which("cargo"):
        raise SystemExit("cargo is required to regenerate the OpenAPI client")

    generate_raw(root, spec, raw_output)
    normalize(root, raw_output, components)
    run(
        [
            shutil.which("python") or shutil.which("python3") or "python",
            str(root / "scripts" / "generate_route_capabilities.py"),
            "--root",
            str(root),
            "--spec",
            str(spec),
        ],
        cwd=root,
    )

    if not args.skip_route_examples:
        run([shutil.which("python") or shutil.which("python3") or "python", str(root / "scripts" / "route_examples.py"), "generate", "--root", str(root)], cwd=root)

    if not args.skip_fmt:
        format_workspace(root)

    print(f"Regenerated OpenAPI client from {spec.relative_to(root)}")


if __name__ == "__main__":
    main()
