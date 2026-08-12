#!/usr/bin/env python3
"""Generate machine-readable route capability metadata from Mollie's OpenAPI spec."""

from __future__ import annotations

import argparse
import re
from pathlib import Path
from typing import Any

import yaml


VALIDATED_OPERATIONS = {
    "create_payment",
    "create_refund",
    "create_subscription",
}
HTTP_METHODS = {"get", "post", "put", "patch", "delete"}

# Writes Mollie documents as typically accepting Idempotency-Key and safe to
# multi-attempt when the *same sticky* key is reused by the caller.
IDEMPOTENT_WRITE_OPS = {
    "create_payment",
    "create_refund",
    "create_subscription",
    "create_customer",
    "create_payment_link",
    "create_capture",
    "create_mandate",
    "create_profile",
    "create_webhook",
    "create_sales_invoice",
    "create_client_link",
    "create_connect_balance_transfer",
    "create_payout",
    "create_transfer",
    "create_session",
    "update_payment",
    "update_subscription",
    "update_customer",
    "update_payment_link",
    "update_profile",
    "update_webhook",
    "update_sales_invoice",
    "cancel_payment",
    "cancel_refund",
    "cancel_subscription",
    "cancel_payout",
    "delete_customer",
    "delete_payment_link",
    "delete_profile",
    "delete_webhook",
    "delete_sales_invoice",
    "release_authorization",
}

# Side-effecting writes that must never be auto-retried by the SDK, even with
# a sticky idempotency key (token churn, one-shot provider actions, etc.).
NON_RETRYABLE_WRITE_OPS = {
    "oauth_generate_tokens",
    "oauth_revoke_tokens",
    "test_webhook",
    "request_apple_pay_payment_session",
    "verify_payee",
    "match_unmatched_credit_transfer",
    "return_unmatched_credit_transfer",
    "terminals_request_pairing_code",
    "terminals_revoke_pairing_code",
    "submit_onboarding_data",
    "enable_method",
    "disable_method",
    "enable_method_issuer",
    "disable_method_issuer",
}

# Provider-specific semantics not covered by generic safe-read / idempotent-write.
PROVIDER_DEFINED_OPS = {
    # Reserved for future ops that need explicit Mollie-documented policy.
}


def retry_class_for(method: str, operation_id: str) -> str:
    """Classify by *operation id* first; method is only a coarse fallback."""
    method = method.upper()
    if operation_id in NON_RETRYABLE_WRITE_OPS:
        return "NonRetryableWrite"
    if operation_id in PROVIDER_DEFINED_OPS:
        return "ProviderDefined"
    if operation_id in IDEMPOTENT_WRITE_OPS:
        return "IdempotentWrite"
    if method in {"GET", "HEAD", "OPTIONS"}:
        return "SafeRead"
    # Remaining writes (POST/PUT/PATCH/DELETE not explicitly listed): treat as
    # idempotent-write candidates so sticky-key policy can still apply, rather
    # than silent NonRetryable for every new generated write.
    if method in {"POST", "PUT", "PATCH", "DELETE"}:
        return "IdempotentWrite"
    return "Unknown"


def operation_rows(spec: dict[str, Any]) -> list[dict[str, Any]]:
    """Extract stable capability fields in OpenAPI path order."""

    rows = []
    for path, path_item in spec.get("paths", {}).items():
        for method, operation in path_item.items():
            if method.lower() not in HTTP_METHODS:
                continue
            operation_id = operation["operationId"].replace("-", "_")
            parameters = list(path_item.get("parameters", []))
            parameters.extend(operation.get("parameters", []))
            supports_testmode = any(
                parameter.get("$ref") == "#/components/parameters/testmode"
                or (
                    parameter.get("name") == "testmode"
                    and parameter.get("in") == "query"
                )
                for parameter in parameters
            )
            tag = (operation.get("tags") or ["uncategorized"])[0]
            route_group = re.sub(r"[^a-z0-9]+", "_", tag.lower()).strip("_")
            http_method = method.upper()
            supports_idempotency = http_method != "GET" and http_method != "HEAD"
            paginated = operation_id.startswith("list_") or "list" in operation_id
            requires_profile = any(
                (
                    parameter.get("name") == "profileId"
                    or parameter.get("$ref", "").endswith("/profileId")
                )
                for parameter in parameters
            )
            rows.append(
                {
                    "operation_id": operation_id,
                    "route_group": route_group,
                    "http_method": http_method,
                    "path": path,
                    "supports_testmode": supports_testmode,
                    "supports_idempotency": supports_idempotency,
                    "safe_to_retry": http_method in {"GET", "HEAD"},
                    "retry_class": retry_class_for(http_method, operation_id),
                    "paginated": paginated,
                    "requires_profile_scope": requires_profile,
                    "access": (
                        "ValidatedFacade"
                        if operation_id in VALIDATED_OPERATIONS
                        else "GeneratedClient"
                    ),
                }
            )
    return rows


def render(rows: list[dict[str, Any]]) -> str:
    """Render the checked-in Rust capability module."""

    lines = [
        "//! Machine-readable Mollie route capabilities generated from `specs-3.0.yaml`.",
        "",
        "use crate::transport::RetryClass;",
        "",
        "/// Whether an operation has a dedicated validated request-builder facade.",
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]",
        "pub enum RouteAccess {",
        "    /// The SDK exposes a validated write builder for this operation.",
        "    ValidatedFacade,",
        "    /// Use the generated route method and request model directly.",
        "    GeneratedClient,",
        "}",
        "",
        "/// Stable capability metadata for one Mollie OpenAPI operation.",
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]",
        "pub struct RouteCapability {",
        "    /// The normalized OpenAPI operation id.",
        "    pub operation_id: &'static str,",
        "    /// The OpenAPI tag for this operation.",
        "    pub route_group: &'static str,",
        "    /// The HTTP method declared by Mollie.",
        "    pub http_method: &'static str,",
        "    /// The Mollie API path template.",
        "    pub path: &'static str,",
        "    /// Whether the operation declares Mollie's `testmode` query.",
        "    pub supports_testmode: bool,",
        "    /// Whether sending an `Idempotency-Key` is semantically meaningful.",
        "    pub supports_idempotency: bool,",
        "    /// Whether the SDK may auto-retry without an explicit idempotency key.",
        "    pub safe_to_retry: bool,",
        "    /// Retry classification used by transport policy.",
        "    pub retry_class: RetryClass,",
        "    /// Whether the operation is a list/page style endpoint.",
        "    pub paginated: bool,",
        "    /// Whether the OpenAPI operation documents a `profileId` parameter.",
        "    pub requires_profile_scope: bool,",
        "    /// Whether a dedicated validated request builder exists.",
        "    pub access: RouteAccess,",
        "}",
        "",
        "/// All operations in the checked-in Mollie OpenAPI contract.",
        "pub const ROUTE_CAPABILITIES: &[RouteCapability] = &[",
    ]
    for row in rows:
        lines.extend(
            [
                "    RouteCapability {",
                f'        operation_id: "{row["operation_id"]}",',
                f'        route_group: "{row["route_group"]}",',
                f'        http_method: "{row["http_method"]}",',
                f'        path: "{row["path"]}",',
                f'        supports_testmode: {str(row["supports_testmode"]).lower()},',
                f'        supports_idempotency: {str(row["supports_idempotency"]).lower()},',
                f'        safe_to_retry: {str(row["safe_to_retry"]).lower()},',
                f'        retry_class: RetryClass::{row["retry_class"]},',
                f'        paginated: {str(row["paginated"]).lower()},',
                f'        requires_profile_scope: {str(row["requires_profile_scope"]).lower()},',
                f'        access: RouteAccess::{row["access"]},',
                "    },",
            ]
        )
    lines.extend(
        [
            "];",
            "",
            "/// Finds an operation by its normalized OpenAPI operation id.",
            "#[must_use]",
            "pub fn route_capability(operation_id: &str) -> Option<&'static RouteCapability> {",
            "    ROUTE_CAPABILITIES",
            "        .iter()",
            "        .find(|capability| capability.operation_id == operation_id)",
            "}",
            "",
            "/// Resolves retry class for a generated operation, preferring the registry.",
            "///",
            "/// When the operation id is not in the capability table, falls back to",
            "/// [`crate::transport::classify_http_method`], which never treats unknown",
            "/// writes as auto-retryable.",
            "#[must_use]",
            "pub fn retry_class_for_operation(operation_id: &str, http_method: &str) -> RetryClass {",
            "    route_capability(operation_id)",
            "        .map(|cap| cap.retry_class)",
            "        .unwrap_or_else(|| crate::transport::classify_http_method(http_method))",
            "}",
            "",
            "#[cfg(test)]",
            "mod tests {",
            "    use super::{retry_class_for_operation, route_capability, RouteAccess, ROUTE_CAPABILITIES};",
            "    use crate::transport::RetryClass;",
            "",
            "    #[test]",
            "    fn metadata_covers_every_spec_operation_once() {",
            f"        assert_eq!(ROUTE_CAPABILITIES.len(), {len(rows)});",
            "        for (index, capability) in ROUTE_CAPABILITIES.iter().enumerate() {",
            "            assert!(ROUTE_CAPABILITIES[index + 1..]",
            "                .iter()",
            "                .all(|other| { other.operation_id != capability.operation_id }));",
            "        }",
            "    }",
            "",
            "    #[test]",
            "    fn validated_write_operations_are_explicit() {",
            "        for operation in [\"create_payment\", \"create_refund\", \"create_subscription\"] {",
            "            assert_eq!(",
            "                route_capability(operation).unwrap().access,",
            "                RouteAccess::ValidatedFacade",
            "            );",
            "        }",
            "    }",
            "",
            "    #[test]",
            "    fn list_payments_is_safe_read() {",
            "        let cap = route_capability(\"list_payments\").unwrap();",
            "        assert!(cap.safe_to_retry);",
            "        assert_eq!(cap.retry_class, RetryClass::SafeRead);",
            "        assert!(cap.paginated);",
            "    }",
            "",
            "    #[test]",
            "    fn create_payment_supports_idempotency() {",
            "        let cap = route_capability(\"create_payment\").unwrap();",
            "        assert!(cap.supports_idempotency);",
            "        assert!(!cap.safe_to_retry);",
            "        assert_eq!(cap.retry_class, RetryClass::IdempotentWrite);",
            "    }",
            "",
            "    #[test]",
            "    fn retry_class_prefers_registry_over_method() {",
            "        assert_eq!(",
            "            retry_class_for_operation(\"create_payment\", \"POST\"),",
            "            RetryClass::IdempotentWrite",
            "        );",
            "        assert_eq!(",
            "            retry_class_for_operation(\"list_payments\", \"GET\"),",
            "            RetryClass::SafeRead",
            "        );",
            "        assert_eq!(",
            "            retry_class_for_operation(\"oauth_generate_tokens\", \"POST\"),",
            "            RetryClass::NonRetryableWrite",
            "        );",
            "        // Unknown operation id: method fallback must not upgrade POST.",
            "        assert_eq!(",
            "            retry_class_for_operation(\"totally_unknown_op\", \"POST\"),",
            "            RetryClass::Unknown",
            "        );",
            "    }",
            "}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--spec", type=Path, default=None)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    root = args.root.resolve()
    spec_path = (args.spec or root / "specs-3.0.yaml").resolve()
    output_path = root / "src" / "route_capabilities.rs"
    spec = yaml.safe_load(spec_path.read_text(encoding="utf-8"))
    generated = render(operation_rows(spec))
    if args.check:
        current = output_path.read_text(encoding="utf-8")
        if current != generated:
            raise SystemExit(f"{output_path} is out of date; regenerate route capabilities")
        return
    output_path.write_text(generated, encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
