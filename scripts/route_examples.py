#!/usr/bin/env python3
"""Generate and verify per-route Mollie SDK examples."""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


GENERATED_MARKER = "// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND."


@dataclass(frozen=True)
class Param:
    """A generated route method parameter."""

    name: str
    typ: str


@dataclass(frozen=True)
class RouteMethod:
    """A generated route method and its example metadata."""

    name: str
    summary: str
    verb: str
    route: str
    params: tuple[Param, ...]
    """Success body type inside `ResponseValue<...>` (e.g. `PaymentResponse`)."""
    response_type: str


@dataclass(frozen=True)
class RustExpression:
    """A Rust expression plus whether it can fail with `?`."""

    code: str
    fallible: bool = False


JSON_FIXTURES: dict[str, str] = {
    "CreatePaymentLinkBody": """json!({
        "amount": {
            "currency": "EUR",
            "value": "10.00"
        },
        "description": "Order #12345"
    })""",
    "CreateWebhookBody": """json!({
        "eventTypes": [
            "payment-link.paid"
        ],
        "name": "Payment links webhook",
        "url": "https://example.com/webhooks/mollie"
    })""",
    "Amount": """json!({
        "currency": "EUR",
        "value": "10.00"
    })""",
    "MandateRequest": """json!({
        "consumerAccount": "NL55INGB0000000000",
        "consumerName": "Jane Doe",
        "method": "directdebit",
        "signatureDate": "2026-01-01"
    })""",
    "CreateRefundRequest": """json!({
        "amount": {
            "currency": "EUR",
            "value": "10.00"
        },
        "description": "Order refund",
        "metadata": {}
    })""",
    "ListInvoicesMonth": """json!("01")""",
    "EntityProfile": """json!({
        "countriesOfActivity": [
            "NL"
        ],
        "description": "Demo webshop for SDK examples.",
        "email": "owner@example.com",
        "name": "Example Shop",
        "phone": "+31201234567",
        "website": "https://example.com"
    })""",
    "CreatePaymentRequest": """json!({
        "amount": {
            "currency": "EUR",
            "value": "10.00"
        },
        "description": "Order #12345",
        "redirectUrl": "https://example.com/return",
        "webhookUrl": "https://example.com/webhooks/mollie"
    })""",
    "RequestApplePayPaymentSessionBody": """json!({
        "domain": "pay.example.com",
        "validationUrl": "https://apple-pay-gateway-cert.apple.com/paymentservices/paymentSession"
    })""",
    "CreateSubscriptionRequest": """json!({
        "amount": {
            "currency": "EUR",
            "value": "10.00"
        },
        "description": "Monthly example subscription",
        "interval": "1 month",
        "startDate": "2026-01-31",
        "webhookUrl": "https://example.com/webhooks/mollie"
    })""",
}

REQUIRED_EXAMPLE_SNIPPETS = (
    "impl RunnableExample for",
    "#[tokio::main]",
    "support::run_example(",
    "support::print_response(Self::ROUTE, &response);",
)

FORBIDDEN_EXAMPLE_SNIPPETS = (
    "UNKNOWN UNKNOWN",
    "unimplemented!",
    "#![allow(dead_code)]",
    "sample::<",
    "async fn example_",
)

OPTION_ARG_PATTERN = re.compile(r"^\s*None,$", flags=re.MULTILINE)
# Matches a generated method call so bare `None,` args can be attributed by position.
METHOD_CALL_ARG_PATTERN = re.compile(
    r"\.(?P<name>[A-Za-z_][A-Za-z0-9_]*)\(\n(?P<body>(?:[ \t]+.+\n)*?)[ \t]+\)",
    flags=re.MULTILINE,
)

TOKEN_EXAMPLES: dict[str, str] = {
    "BalanceToken": "bal_1234567890",
    "CaptureToken": "cpt_1234567890",
    "ChargebackToken": "chb_1234567890",
    "CustomerToken": "cst_1234567890",
    "MandateToken": "mdt_1234567890",
    "PaymentLinkToken": "pl_1234567890",
    "PaymentToken": "tr_1234567890",
    "PermissionToken": "payments.read",
    "ProfileToken": "pfl_1234567890",
    "RefundToken": "re_1234567890",
    "SubscriptionToken": "sub_1234567890",
    "TerminalToken": "term_1234567890",
}

PROFILE_ID_WRAPPERS = {
    "DisableMethodIssuerProfileId",
    "DisableMethodProfileId",
    "EnableMethodIssuerProfileId",
    "EnableMethodProfileId",
}


def route_info(doc_lines: list[str]) -> tuple[str, str]:
    """Extract an API verb and route from generated Rustdoc lines."""

    for index, raw_line in enumerate(doc_lines):
        line = raw_line.strip()
        inline = re.match(r"^Sends a `([A-Z]+)` request to `([^`]+)`$", line)
        if inline:
            return inline.group(1), inline.group(2)

        wrapped = re.match(r"^Sends a `([A-Z]+)` request to$", line)
        if wrapped:
            verb = wrapped.group(1)
            for next_line in doc_lines[index + 1 :]:
                candidate = next_line.strip()
                if not candidate:
                    continue

                path = re.match(r"^`([^`]+)`$", candidate)
                if path:
                    return verb, path.group(1)
                break

    return "UNKNOWN", "UNKNOWN"


def parse_default_types(root: Path) -> set[str]:
    """Return generated type names that implement `Default`."""

    source = (root / "src" / "types.rs").read_text(encoding="utf-8")
    return set(re.findall(r"impl ::std::default::Default for ([A-Za-z_][A-Za-z0-9_]*)", source))


def parse_enum_variants(root: Path) -> dict[str, tuple[str, ...]]:
    """Return generated enum variants keyed by type name."""

    source = (root / "src" / "types.rs").read_text(encoding="utf-8").splitlines()
    variants: dict[str, tuple[str, ...]] = {}

    index = 0
    while index < len(source):
        enum_match = re.match(r"^\s*pub enum ([A-Za-z_][A-Za-z0-9_]*) \{$", source[index])
        if not enum_match:
            index += 1
            continue

        enum_name = enum_match.group(1)
        enum_variants: list[str] = []
        index += 1
        while index < len(source):
            line = source[index]
            if re.match(r"^\s*\}", line):
                break

            variant_match = re.match(r"^\s*([A-Z][A-Za-z0-9_]*),$", line)
            if variant_match:
                enum_variants.append(variant_match.group(1))
            index += 1

        variants[enum_name] = tuple(enum_variants)
        index += 1

    return variants


def parse_methods(root: Path) -> list[RouteMethod]:
    """Parse generated public async route methods from src/routes."""

    route_dir = root / "src" / "routes"
    module_names = re.findall(
        r"^pub mod ([A-Za-z_][A-Za-z0-9_]*);$",
        (route_dir / "mod.rs").read_text(encoding="utf-8"),
        flags=re.MULTILINE,
    )
    route_files = [route_dir / f"{module_name}.rs" for module_name in module_names]
    methods: list[RouteMethod] = []

    for route_file in route_files:
        source = route_file.read_text(encoding="utf-8").splitlines()
        for line_index, line in enumerate(source):
            method = re.match(r"^\s*pub async fn ([A-Za-z_][A-Za-z0-9_]*)<'a>\(", line)
            if not method:
                continue

            name = method.group(1)
            doc_lines: list[str] = []
            doc_index = line_index - 1
            while doc_index >= 0:
                doc = re.match(r"^\s*///(.*)$", source[doc_index])
                if not doc:
                    break
                doc_lines.insert(0, doc.group(1).lstrip())
                doc_index -= 1

            summary = next((item for item in doc_lines if item.strip()), name)
            verb, route = route_info(doc_lines)
            params: list[Param] = []
            response_type = "serde_json::Value"

            signature_index = line_index + 1
            while signature_index < len(source):
                signature_line = source[signature_index]

                # End of parameter list (return type follows on this or later lines).
                if re.match(r"^\s*\)\s*->", signature_line) or re.match(
                    r"^\s*\)\s*->\s*Result<", signature_line
                ):
                    # Collect return-type lines until the function body opens.
                    return_chunk = signature_line
                    while signature_index < len(source) and "{" not in source[signature_index]:
                        signature_index += 1
                        if signature_index < len(source):
                            return_chunk += " " + source[signature_index].strip()

                    ret = re.search(
                        r"ResponseValue<\s*((?:\(\))|(?:::)?(?:[A-Za-z0-9_]+::)*[A-Za-z0-9_]+)\s*>",
                        return_chunk,
                    )
                    if ret:
                        response_type = ret.group(1).replace(" ", "")
                        if response_type.startswith("types::"):
                            response_type = response_type.removeprefix("types::")
                        response_type = normalize_response_type(response_type)
                    break

                param = re.match(
                    r"^\s*([A-Za-z_][A-Za-z0-9_]*):\s*(.+?),\s*$",
                    signature_line,
                ) or re.match(
                    r"^\s*([A-Za-z_][A-Za-z0-9_]*):\s*(.+)\s*$",
                    signature_line,
                )
                if param and param.group(1) != "self":
                    params.append(Param(name=param.group(1), typ=param.group(2).rstrip(",").strip()))

                # Safety: never walk into the function body.
                if signature_line.strip().endswith("{") and "fn " not in signature_line:
                    break

                signature_index += 1

            methods.append(
                RouteMethod(
                    name=name,
                    summary=summary,
                    verb=verb,
                    route=route,
                    params=tuple(params),
                    response_type=response_type,
                )
            )

    return methods


def option_inner_type(param: Param) -> str | None:
    """Extract the inner type from an optional generated parameter."""

    match = re.match(r"^Option<(.+)>$", param.typ)
    if match:
        return match.group(1)

    return None


def referenced_type_name(param: Param) -> str | None:
    """Extract the generated type name from referenced parameters."""

    return referenced_type_name_from_type(param.typ)


def referenced_type_name_from_type(typ: str) -> str | None:
    """Extract the generated type name from a referenced Rust type."""

    if typ.startswith("&'a types::"):
        return typ.removeprefix("&'a types::")

    if typ.startswith("Option<&'a types::") and typ.endswith(">"):
        return typ.removeprefix("Option<&'a types::").removesuffix(">")

    return None


def value_type_name_from_type(typ: str) -> str | None:
    """Extract the generated type name from an owned Rust type."""

    if typ.startswith("types::"):
        return typ.removeprefix("types::")

    if typ.startswith("Option<types::") and typ.endswith(">"):
        return typ.removeprefix("Option<types::").removesuffix(">")

    return None


def optional_vec_element_type(typ: str) -> str | None:
    """Extract element type from ``Option<&'a ::std::vec::Vec<types::T>>``."""

    match = re.match(
        r"^Option<&'a (?:::std::vec::)?Vec<types::([A-Za-z_][A-Za-z0-9_]*)>>$",
        typ,
    )
    if match:
        return match.group(1)
    return None


def pascal_case(value: str) -> str:
    """Convert a snake_case method name to a Rust type identifier."""

    return "".join(part.capitalize() for part in value.split("_"))


def indent_lines(lines: list[str], prefix: str) -> list[str]:
    """Indent possibly multiline generated Rust statements."""

    indented: list[str] = []
    for line in lines:
        for part in line.splitlines():
            if part.startswith("    "):
                part = part[4:]
            indented.append(f"{prefix}{part}")
    return indented


def string_example(param: Param) -> str:
    """Return a concrete string example for a route parameter."""

    if param.name == "billing_country":
        return '"NL"'
    if param.name == "currency":
        return '"EUR"'
    if param.name == "embed":
        return '"payments"'
    if param.name == "include":
        # Methods endpoints accept `issuers` (and sometimes `pricing`); `payments` is not valid.
        return '"issuers"'
    if "month" in param.name:
        return '"2026-01"'
    if "year" in param.name:
        return '"2026"'
    if param.name == "reference":
        return '"INV-12345"'
    if "date" in param.name:
        return '"2026-01-31"'
    if "from" in param.name or "until" in param.name:
        return '"2026-01-01"'
    return '"example-id"'


# Optional query params that must not use generator placeholders in live examples.
# - `from`: Mollie cursor IDs; placeholders return INVALID_CURSOR (40001).
# - `profile_id`: with API keys Mollie rejects profileId entirely ("must not be
#   sent"); only organization OAuth may send a real pfl_* id.
OMITTED_OPTIONAL_QUERY_PARAMS = frozenset({"from", "profile_id"})


def is_omitted_optional_query_param(param: Param) -> bool:
    """True for optional query params examples intentionally pass as ``None``.

    Mollie first-page list calls work without these. Placeholder tokens break
    live runs (`INVALID_CURSOR`, unknown `profileId`, etc.).
    """

    return (
        param.name in OMITTED_OPTIONAL_QUERY_PARAMS
        and option_inner_type(param) is not None
    )


# Back-compat alias used in a few call sites / comments.
def is_optional_pagination_from(param: Param) -> bool:
    """True for optional list pagination cursor params named `from`."""

    return param.name == "from" and option_inner_type(param) is not None


def unexpected_none_arguments(source: str, methods: list[RouteMethod]) -> list[str]:
    """Return labels for bare `None` args outside the allowed omit set."""

    by_name = {method.name: method for method in methods}
    findings: list[str] = []
    for match in METHOD_CALL_ARG_PATTERN.finditer(source):
        method = by_name.get(match.group("name"))
        if method is None:
            continue
        arg_lines = [
            line.strip().removesuffix(",")
            for line in match.group("body").splitlines()
            if line.strip()
        ]
        for index, arg in enumerate(arg_lines):
            if arg != "None":
                continue
            if index >= len(method.params):
                findings.append(f"{method.name}[arg{index}]")
                continue
            param = method.params[index]
            if not is_omitted_optional_query_param(param):
                findings.append(f"{method.name}.{param.name}")
    return findings


def enum_expression(
    type_name_: str,
    enum_variants: dict[str, tuple[str, ...]],
    *,
    qualified: bool = False,
) -> str:
    """Return a concrete generated enum expression."""

    variants = enum_variants.get(type_name_)
    if not variants:
        raise ValueError(f"missing enum variant fixture for types::{type_name_}")

    variant = "Desc" if "Desc" in variants else variants[0]
    prefix = f"types::{type_name_}" if qualified else type_name_
    return f"{prefix}::{variant}"


def argument_expression(
    param: Param,
    enum_variants: dict[str, tuple[str, ...]],
    *,
    qualified_enums: bool = False,
    config_context: str | None = None,
) -> str:
    """Return a compile-checkable argument expression for a parameter."""

    if config_context is not None:
        if optional_vec_element_type(param.typ) is not None:
            return f"Some(&{param.name})"

        if inner_type := option_inner_type(param):
            if inner_type == "&'a str":
                if is_omitted_optional_query_param(param):
                    return f'{config_context}.optional_value("{param.name}")'
                return f'Some({config_context}.value("{param.name}", {string_example(param)}))'
            if inner_type == "::std::num::NonZeroU64":
                return f"{config_context}.limit(50)"
            if inner_type == "bool":
                return f'Some({config_context}.bool_value("{param.name}", true))'
            if referenced_type_name_from_type(param.typ) is not None:
                if is_omitted_optional_query_param(param):
                    return f"{param.name}.as_ref()"
                return f"Some(&{param.name})"
            if type_name_ := value_type_name_from_type(param.typ):
                default = enum_expression(
                    type_name_, enum_variants, qualified=qualified_enums
                )
                return (
                    f'Some({config_context}.configured("{param.name}", '
                    f"{default})?)"
                )

        if param.typ == "&'a str":
            return f'{config_context}.value("{param.name}", {string_example(param)})'

        if referenced_type_name(param) is not None:
            return f"&{param.name}"

        # Owned enum/value params (e.g. `method_id: types::MethodIdWithIssuer`).
        if type_name_ := value_type_name_from_type(param.typ):
            default = enum_expression(
                type_name_, enum_variants, qualified=qualified_enums
            )
            return f'{config_context}.configured("{param.name}", {default})?'

        raise ValueError(
            f"missing configured route example argument fixture for `{param.name}: {param.typ}`"
        )

    if is_omitted_optional_query_param(param):
        return "None"

    if optional_vec_element_type(param.typ) is not None:
        return f"Some(&{param.name})"

    if inner_type := option_inner_type(param):
        if inner_type == "&'a str":
            return f"Some({string_example(param)})"
        if inner_type == "::std::num::NonZeroU64":
            return "::std::num::NonZeroU64::new(50)"
        if inner_type == "bool":
            return "Some(true)"
        if referenced_type_name_from_type(param.typ) is not None:
            return f"Some(&{param.name})"
        if type_name_ := value_type_name_from_type(param.typ):
            return f"Some({enum_expression(type_name_, enum_variants, qualified=qualified_enums)})"

    if param.typ == "&'a str":
        return string_example(param)

    if referenced_type_name(param) is not None:
        return f"&{param.name}"

    # Owned enum params such as `types::MethodIdWithIssuer` / `types::SequenceType`.
    if type_name_ := value_type_name_from_type(param.typ):
        return enum_expression(type_name_, enum_variants, qualified=qualified_enums)

    raise ValueError(f"missing route example argument fixture for `{param.name}: {param.typ}`")


def typed_value_expression(
    type_name_: str,
    default_types: set[str],
    *,
    qualified: bool = False,
) -> RustExpression:
    """Return a concrete Rust expression for a generated type."""

    prefix = f"types::{type_name_}" if qualified else type_name_
    profile = "types::ProfileToken" if qualified else "ProfileToken"

    if type_name_ in PROFILE_ID_WRAPPERS:
        return RustExpression(
            code=(
                f'{prefix}::from({profile}::try_from("pfl_1234567890".to_owned())'
                '.expect("valid profile token fixture"))'
            )
        )

    if token := TOKEN_EXAMPLES.get(type_name_):
        return RustExpression(
            code=(
                f'{prefix}::try_from("{token}".to_owned())'
                '.expect("valid generated token fixture")'
            )
        )

    if fixture := JSON_FIXTURES.get(type_name_):
        return RustExpression(
            code=f"from_value::<{prefix}>({fixture})",
            fallible=True,
        )

    if type_name_ in default_types:
        return RustExpression(code=f"{prefix}::default()")

    return RustExpression(
        code=f"from_value::<{prefix}>(json!({{}}))",
        fallible=True,
    )


def local_binding(
    param: Param,
    default_types: set[str],
    *,
    typed: bool = False,
    qualified: bool = False,
    config_context: str | None = None,
) -> str | None:
    """Return a local binding for referenced generated body/token params.

    When ``typed`` is true, emit cancel_payment-style annotations:
    ``let payment_id: PaymentToken = PaymentToken::from(...);``

    Omitted optional query params (``from``, ``profile_id``) intentionally have
    no binding; the call site passes ``None``.
    """

    if config_context is not None:
        if element := optional_vec_element_type(param.typ):
            # Owned vec so `Some(&name)` is valid across `.await`.
            prefix = f"types::{element}" if qualified else element
            # First enum variant is a stable fixture (parse_enum_variants order).
            # Use Default-style via empty vec when enum parsing is unavailable here.
            return (
                f"    let {param.name}: ::std::vec::Vec<{prefix}> = "
                f"::std::vec::Vec::new();"
            )

        type_name_ = referenced_type_name(param)
        inner_type = option_inner_type(param)

        if inner_type in {"&'a str", "::std::num::NonZeroU64", "bool"}:
            return None
        if inner_type is not None and type_name_ is None:
            return None
        if type_name_ is None:
            return None

        type_ann = f"types::{type_name_}" if qualified else type_name_
        if param.name == "body":
            expression = typed_value_expression(
                type_name_, default_types, qualified=qualified
            )
            default = expression.code + ("?" if expression.fallible else "")
            return f"    let {param.name}: {type_ann} = {config_context}.body({default})?;"

        if inner_type is not None and is_omitted_optional_query_param(param):
            if type_name_ in TOKEN_EXAMPLES:
                expression = f'{config_context}.optional_token("{param.name}")'
            else:
                expression = f'{config_context}.optional_configured("{param.name}")?'
            return f"    let {param.name}: Option<{type_ann}> = {expression};"

        expression = typed_value_expression(
            type_name_, default_types, qualified=qualified
        )
        default = expression.code + ("?" if expression.fallible else "")
        if type_name_ in PROFILE_ID_WRAPPERS:
            profile = "types::ProfileToken" if qualified else "ProfileToken"
            expression_code = (
                f'{type_ann}::from({profile}::try_from('
                f'{config_context}.value("profile_id", "pfl_1234567890").to_owned())'
                '.expect("valid profile token fixture"))'
            )
        elif type_name_ in TOKEN_EXAMPLES:
            expression_code = (
                f'{config_context}.token("{param.name}", '
                f'"{TOKEN_EXAMPLES[type_name_]}")'
            )
        else:
            config_name = (
                "invoice_month"
                if type_name_ == "ListInvoicesMonth"
                else param.name
            )
            expression_code = f'{config_context}.configured("{config_name}", {default})?'
        return f"    let {param.name}: {type_ann} = {expression_code};"

    if is_omitted_optional_query_param(param):
        return None

    if element := optional_vec_element_type(param.typ):
        prefix = f"types::{element}" if qualified else element
        return (
            f"    let {param.name}: ::std::vec::Vec<{prefix}> = "
            f"::std::vec::Vec::new();"
        )

    type_name_ = referenced_type_name(param)
    if type_name_ is not None:
        expression = typed_value_expression(
            type_name_, default_types, qualified=qualified
        )
        suffix = "?" if expression.fallible else ""
        type_ann = f"types::{type_name_}" if qualified else type_name_
        if typed:
            return f"    let {param.name}: {type_ann} = {expression.code}{suffix};"
        return f"    let {param.name} = {expression.code}{suffix};"

    return None


def normalize_response_type(response_type: str) -> str:
    """Normalize a parsed `ResponseValue<T>` success body type."""

    response = response_type.replace(" ", "").lstrip(":")
    if response in {"serde_json::Value", "Value"}:
        return "serde_json::Value"
    return response


def is_json_value_response(response_type: str) -> bool:
    """Return true when the success body is `serde_json::Value`."""

    return normalize_response_type(response_type) == "serde_json::Value"


def collect_type_imports(
    method: RouteMethod,
    default_types: set[str],
    *,
    include_omitted: bool = False,
) -> list[str]:
    """Collect generated type names to import for a route example."""

    names: set[str] = set()
    for param in method.params:
        if is_omitted_optional_query_param(param) and not include_omitted:
            continue
        if type_name_ := referenced_type_name(param):
            names.add(type_name_)
            if type_name_ in PROFILE_ID_WRAPPERS:
                names.add("ProfileToken")
        elif element_type := optional_vec_element_type(param.typ):
            names.add(element_type)
        elif type_name_ := value_type_name_from_type(param.typ):
            names.add(type_name_)

    # Response body type for `ResponseValue<T>` (import plain generated type names only).
    response = normalize_response_type(method.response_type)
    if re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", response):
        names.add(response)

    return sorted(names)


def rust_response_type(method: RouteMethod) -> str:
    """Rust type expression for `ResponseValue<T>` in examples.

    JSON-body-less routes decode as unit; examples annotate the binding as
    ``ResponseValue<()>``.
    """

    response = normalize_response_type(method.response_type)
    if is_json_value_response(response):
        return "Value"
    if response.startswith("serde_json::"):
        return response
    return response


def needs_serde_json_value_import(method: RouteMethod) -> bool:
    """Whether the example should `use serde_json::Value`."""

    return is_json_value_response(method.response_type)


def needs_serde_json_from_value_json_import(
    method: RouteMethod, default_types: set[str]
) -> bool:
    """Whether the example uses `from_value` / `json!` for request fixtures."""

    for param in method.params:
        if is_omitted_optional_query_param(param):
            continue
        type_name_ = referenced_type_name(param)
        if type_name_ is None:
            continue
        if type_name_ in PROFILE_ID_WRAPPERS or type_name_ in TOKEN_EXAMPLES:
            continue
        if type_name_ in JSON_FIXTURES:
            return True
        if type_name_ not in default_types:
            return True
    return False


def serde_json_import_line(method: RouteMethod, default_types: set[str]) -> str | None:
    """Build a single `use serde_json::{...};` line when needed."""

    names: list[str] = []
    if needs_serde_json_from_value_json_import(method, default_types):
        names.extend(["from_value", "json"])
    if needs_serde_json_value_import(method):
        names.append("Value")
    if not names:
        return None
    if len(names) == 1:
        return f"use serde_json::{names[0]};"
    joined = ", ".join(names)
    return f"use serde_json::{{{joined}}};"


def add_generated_call(
    lines: list[str],
    method: RouteMethod,
    indent: str,
    *,
    client_expression: str,
    discard_response: bool,
    propagate_error: bool,
    enum_variants: dict[str, tuple[str, ...]],
    typed_response: bool = False,
    qualified_enums: bool = False,
    config_context: str | None = None,
) -> None:
    """Append a generated route call to a Rust snippet."""

    if typed_response:
        response_ty = rust_response_type(method)
        lines.append(
            f"{indent}let response: ResponseValue<{response_ty}> = {client_expression}"
        )
    else:
        lines.append(f"{indent}let response = {client_expression}")

    if not method.params:
        lines.append(f"{indent}    .{method.name}()")
    else:
        lines.append(f"{indent}    .{method.name}(")
        for param in method.params:
            lines.append(
                f"{indent}        {argument_expression(param, enum_variants, qualified_enums=qualified_enums, config_context=config_context)},"
            )
        lines.append(f"{indent}    )")

    error_suffix = "?" if propagate_error else ""
    lines.append(f"{indent}    .await{error_suffix};")
    lines.append("")
    if discard_response:
        lines.append(f"{indent}let _ = response;")


def markdown_doc(
    methods: list[RouteMethod],
    default_types: set[str],
    enum_variants: dict[str, tuple[str, ...]],
) -> str:
    """Render the markdown route examples document."""

    lines = [
        "# Route Examples",
        "",
        "This file gives one call-shape example for every public async route method in `src/routes`. The API verb and path are copied from the Rustdoc comments, and each example calls the method exactly as it appears on `Client` and `MollieClient`.",
        "",
        "Examples that need generated request bodies or typed token structs use concrete generated values. Required request payloads are created with `Default` when available, token newtypes use their generated `TryFrom<String>` implementations, and the few required non-default request payloads use JSON fixtures.",
        "",
        "Every generated binary accepts shared environment variables and matching Clap options; unknown `--name value` options are also accepted as route/body fixture overrides. See [`docs/example-runtime-config.md`](example-runtime-config.md). CLI values override environment values, and `EXAMPLE_BODY_JSON` / `EXAMPLE_BODY_FILE` can replace a request body.",
        "",
        "Optional pagination `from` cursors and optional `profile_id` filters are always omitted (`None`) so first-page list calls do not send placeholder IDs (Mollie rejects fake cursors as `INVALID_CURSOR`). With API-key credentials Mollie also rejects any `profileId` query param (`must not be sent`); only set `PROFILE_ID` / `--profile-id` when using organization-level OAuth and a real `pfl_*` id.",
        "",
        "Run `powershell -ExecutionPolicy Bypass -File scripts/generate_route_examples.ps1` or `sh scripts/generate_route_examples.sh` after route changes, then run the matching `check_route_examples` script to verify this file and the Rust examples still cover every route method.",
        "",
        "## Methods",
        "",
    ]

    for method in methods:
        type_imports = collect_type_imports(method, default_types)
        response_ty = rust_response_type(method)
        use_types = ""
        if type_imports:
            if len(type_imports) == 1:
                use_types = f"use mollie_rs::types::{type_imports[0]};"
            else:
                joined = ", ".join(type_imports)
                use_types = f"use mollie_rs::types::{{{joined}}};"

        lines.extend(
            [
                f"### `{method.name}`",
                "",
                f"- Summary: {method.summary}",
                f"- Route: `{method.verb} {method.route}`",
                f"- Response: `ResponseValue<{response_ty}>`",
                f"- Rust example: `examples/{method.name}.rs`",
                "",
                "```rust",
                "use mollie_rs::{MollieClient, ResponseValue};",
            ]
        )
        if use_types:
            lines.append(use_types)
        if serde_import := serde_json_import_line(method, default_types):
            lines.append(serde_import)
        lines.extend(
            [
                "",
                "async fn example() -> Result<(), mollie_rs::MollieError> {",
                '    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;',
            ]
        )

        bindings = [
            binding
            for param in method.params
            if (
                binding := local_binding(
                    param, default_types, typed=True, qualified=False
                )
            )
        ]
        if bindings:
            lines.extend(
                [
                    "",
                    *bindings,
                ]
            )

        lines.append("")
        add_generated_call(
            lines,
            method,
            "    ",
            client_expression="client",
            discard_response=True,
            propagate_error=False,
            enum_variants=enum_variants,
            typed_response=True,
            qualified_enums=False,
        )
        lines.extend(["    Ok(())", "}", "```", ""])

    return "\n".join(lines) + "\n"


def rust_example(
    method: RouteMethod,
    default_types: set[str],
    enum_variants: dict[str, tuple[str, ...]],
) -> str:
    """Render one compile-checked Rust example target.

    Shape matches hand-tuned examples such as ``cancel_payment.rs``:
    typed imports, typed locals, and ``ResponseValue<T>`` on the call result.
    """

    type_imports = collect_type_imports(method, default_types, include_omitted=True)
    import_lines: list[str] = []
    if type_imports:
        if len(type_imports) == 1:
            import_lines.append(f"use mollie_rs::types::{type_imports[0]};")
        else:
            joined = ", ".join(type_imports)
            import_lines.append(f"use mollie_rs::types::{{{joined}}};")
    import_lines.append("use mollie_rs::ResponseValue;")
    if serde_import := serde_json_import_line(method, default_types):
        import_lines.append(serde_import)

    lines = [
        GENERATED_MARKER,
        f"//! Runnable example for `Client::{method.name}`.",
        "//!",
        f"//! Route: `{method.verb} {method.route}`.",
        "",
        "#[path = \"support/mod.rs\"]",
        "mod support;",
        "",
        *import_lines,
        "use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};",
        "",
        f"/// Runnable example for `Client::{method.name}`.",
        f"struct {pascal_case(method.name)}Example;",
        "",
        f"impl RunnableExample for {pascal_case(method.name)}Example {{",
        "    /// Generated SDK method name demonstrated by this example.",
        f'    const NAME: &\'static str = "{method.name}";',
        "",
        "    /// HTTP method and path demonstrated by this example.",
        f'    const ROUTE: &\'static str = "{method.verb} {method.route}";',
        "",
        "    /// Runs this route example with the shared example context.",
        "    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {",
        "        Box::pin(async move {",
    ]

    bindings = [
        binding
        for param in method.params
        if (
            binding := local_binding(
                param,
                default_types,
                typed=True,
                qualified=False,
                config_context="context.options()",
            )
        )
    ]
    lines.extend(indent_lines(bindings, "            "))
    if bindings:
        lines.append("")

    add_generated_call(
        lines,
        method,
        "            ",
        client_expression="context\n                .client()",
        discard_response=False,
        propagate_error=True,
        enum_variants=enum_variants,
        typed_response=True,
        qualified_enums=False,
        config_context="context.options()",
    )
    lines.extend(
        [
            "            support::print_response(Self::ROUTE, &response);",
            "            Ok(())",
            "        })",
            "    }",
            "}",
            "",
            "#[tokio::main]",
            "async fn main() -> ExampleResult<()> {",
            f"    support::run_example({pascal_case(method.name)}Example).await",
            "}",
            "",
        ]
    )
    return "\n".join(lines)


def generated_example_files(root: Path) -> list[Path]:
    """Return generated per-route example files currently present."""

    examples_dir = root / "examples"
    files: list[Path] = []
    for path in examples_dir.glob("*.rs"):
        try:
            first_line = path.read_text(encoding="utf-8").splitlines()[0]
        except IndexError:
            continue
        if first_line == GENERATED_MARKER:
            files.append(path)
    return files


def generated_example_sources(root: Path) -> dict[str, tuple[Path, str]]:
    """Return generated per-route example source keyed by method name."""

    return {
        path.stem: (path, path.read_text(encoding="utf-8"))
        for path in generated_example_files(root)
    }


def generate(root: Path) -> None:
    """Generate markdown and one Rust example file per route."""

    methods = parse_methods(root)
    default_types = parse_default_types(root)
    enum_variants = parse_enum_variants(root)
    examples_dir = root / "examples"
    examples_dir.mkdir(exist_ok=True)

    expected = {examples_dir / f"{method.name}.rs" for method in methods}
    for path in generated_example_files(root):
        if path not in expected:
            path.unlink()

    aggregate = examples_dir / "generated_route_methods.rs"
    if aggregate.exists():
        aggregate.unlink()

    (root / "docs" / "route-examples.md").write_text(
        markdown_doc(methods, default_types, enum_variants),
        encoding="utf-8",
        newline="\n",
    )

    for method in methods:
        (examples_dir / f"{method.name}.rs").write_text(
            rust_example(method, default_types, enum_variants),
            encoding="utf-8",
            newline="\n",
        )

    print(f"Generated {len(methods)} markdown route examples and Rust example files.")


def check(root: Path) -> None:
    """Verify markdown and per-route Rust examples match generated methods."""

    methods = parse_methods(root)
    method_names = {method.name for method in methods}
    doc_path = root / "docs" / "route-examples.md"
    examples_dir = root / "examples"

    if not doc_path.exists():
        raise SystemExit("Missing docs/route-examples.md")

    doc = doc_path.read_text(encoding="utf-8")
    if "UNKNOWN UNKNOWN" in doc:
        raise SystemExit("docs/route-examples.md contains UNKNOWN routes. Regenerate route examples.")

    # Bare `None` is allowed only for intentionally omitted optional query params.
    # Other optional params should still use concrete `Some(...)` fixtures.
    unexpected_none = unexpected_none_arguments(doc, methods)
    if unexpected_none:
        allowed = ", ".join(sorted(OMITTED_OPTIONAL_QUERY_PARAMS))
        raise SystemExit(
            "docs/route-examples.md contains unexpected bare None route arguments "
            f"(only optional {allowed} may be None): {', '.join(unexpected_none)}"
        )

    documented = set(re.findall(r"^### `([A-Za-z_][A-Za-z0-9_]*)`$", doc, flags=re.MULTILINE))
    generated_sources = generated_example_sources(root)
    rust_files: set[str] = set()
    example_failures: list[str] = []
    methods_by_name = {method.name: method for method in methods}
    for method_name, (path, content) in generated_sources.items():
        relative_path = path.relative_to(root)
        if (
            f"`Client::{method_name}`" in content
            and f'const NAME: &\'static str = "{method_name}";' in content
            and all(snippet in content for snippet in REQUIRED_EXAMPLE_SNIPPETS)
        ):
            rust_files.add(method_name)

        for snippet in FORBIDDEN_EXAMPLE_SNIPPETS:
            if snippet in content:
                example_failures.append(f"{relative_path} contains forbidden snippet `{snippet}`")

        for snippet in REQUIRED_EXAMPLE_SNIPPETS:
            if snippet not in content:
                example_failures.append(f"{relative_path} is missing `{snippet}`")

        method = methods_by_name.get(method_name)
        if method is not None:
            for bad in unexpected_none_arguments(content, [method]):
                allowed = ", ".join(sorted(OMITTED_OPTIONAL_QUERY_PARAMS))
                example_failures.append(
                    f"{relative_path} has unexpected bare None for `{bad}` "
                    f"(only optional {allowed} may be None)"
                )

    failures: list[str] = []
    for label, found in (("markdown", documented), ("Rust", rust_files)):
        missing = sorted(method_names - found)
        extra = sorted(found - method_names)
        if missing:
            failures.append(f"Missing {label} route examples: {', '.join(missing)}")
        if extra:
            failures.append(f"Unknown {label} route examples: {', '.join(extra)}")

    extra_generated_files = sorted(set(generated_sources) - method_names)
    if extra_generated_files:
        failures.append(f"Unknown generated Rust example files: {', '.join(extra_generated_files)}")

    failures.extend(example_failures)

    aggregate = examples_dir / "generated_route_methods.rs"
    if aggregate.exists():
        failures.append("Remove stale examples/generated_route_methods.rs; examples are generated per method.")

    if failures:
        raise SystemExit("\n".join(failures))

    print(f"docs/route-examples.md and examples/*.rs cover {len(methods)} route methods.")


def main() -> None:
    """Run the route example generator/checker."""

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("generate", "check"))
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()

    root = args.root.resolve()
    if args.command == "generate":
        generate(root)
    else:
        check(root)


if __name__ == "__main__":
    main()
