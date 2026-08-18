//! Operation safety profile — singular policy SSOT for transport and facades.
//!
//! [`OperationSafetyProfile`] is a type alias of [`RouteCapability`]. Derived
//! classes (auth, mutation, idempotency, pagination) are computed from the
//! checked-in capability table so callers never maintain a second parallel
//! registry. INV-PROFILE-01: retry/idempotency/auth attachment decisions read
//! this surface only.

use crate::route_capabilities::{
    route_capability, RouteAccess, RouteCapability, ROUTE_CAPABILITIES,
};
use crate::transport::RetryClass;

/// Singular per-operation safety profile (SSOT).
///
/// Currently identical to [`RouteCapability`]; additional profile fields are
/// exposed as methods so the table can grow without dual cores.
pub type OperationSafetyProfile = RouteCapability;

/// Authentication class implied by the operation contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AuthClass {
    /// Standard Mollie API key or org access token on `/v2`.
    ApiCredential,
    /// OAuth client-credentials / token endpoints (`/oauth2`).
    OAuthClient,
    /// No Authorization expected (rare; webhooks are inbound).
    None,
}

/// Mutation / side-effect class for delivery and retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MutationClass {
    /// Safe read (GET/HEAD-class).
    Read,
    /// Write that may be retried with a sticky idempotency key.
    IdempotentWrite,
    /// Financial or one-shot write that must not auto-retry.
    FinancialOrNonRetryableWrite,
    /// Unknown / unregistered operation — fail closed.
    Unknown,
}

/// Idempotency key expectations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdempotencyClass {
    /// Idempotency-Key not meaningful.
    None,
    /// Optional; auto UUID used when sticky absent (single attempt for writes).
    Optional,
    /// Required for any multi-attempt retry of this write.
    RequiredForRetry,
    /// Never auto-retry even with a key.
    NeverRetry,
}

/// Pagination behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PaginationPolicy {
    /// Not a list endpoint.
    None,
    /// Cursor/`from` list with origin-guarded next links.
    GuardedCursor,
}

/// Testmode query attachment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TestmodePolicy {
    /// Operation documents `testmode`.
    Supported,
    /// Must not attach testmode.
    Unsupported,
}

/// Profile id query attachment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProfileScope {
    /// Operation documents `profileId`.
    RequiredOrSupported,
    /// Must not attach profile id from sticky client context unless per-op.
    Unsupported,
}

/// Financial / security risk class for an operation (derived; not a second SSOT table).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OperationRisk {
    /// Safe read (GET/HEAD-class).
    ReadOnly,
    /// Money-moving or payment-creating write.
    FinancialWrite,
    /// Cancellation of a financial resource.
    FinancialCancellation,
    /// OAuth or credential material mutation.
    CredentialMutation,
    /// Enable/disable payment methods or issuers.
    PaymentCapabilityMutation,
    /// Profile/organization configuration mutation.
    AccountConfigurationMutation,
    /// Terminal pairing / security surface.
    TerminalSecurityMutation,
    /// Session or other PII collection mutation.
    PiiCollectionMutation,
    /// Mutation not yet reviewed into a tighter class.
    Unknown,
}

/// How far the SDK exposes an operation on the stable facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OperationExposure {
    /// Generated client only (reviewed existing surface).
    Generated,
    /// Generated but must not gain retries / Tier-S without review.
    GeneratedQuarantined,
    /// Validated Tier-S facade path.
    ValidatedFacade,
}

impl RouteCapability {
    /// Returns this row as the operation safety profile SSOT view.
    pub const fn safety_profile(&self) -> &OperationSafetyProfile {
        self
    }

    /// Auth class derived from path / route group.
    pub fn auth_class(&self) -> AuthClass {
        if self.path.starts_with("/oauth2") || self.route_group == "oauth_api" {
            AuthClass::OAuthClient
        } else {
            AuthClass::ApiCredential
        }
    }

    /// Mutation class derived from retry classification.
    pub const fn mutation_class(&self) -> MutationClass {
        match self.retry_class {
            RetryClass::SafeRead => MutationClass::Read,
            RetryClass::IdempotentWrite => MutationClass::IdempotentWrite,
            RetryClass::NonRetryableWrite
            | RetryClass::NeverAutoRetry
            | RetryClass::ProviderDefined => MutationClass::FinancialOrNonRetryableWrite,
            RetryClass::Unknown => MutationClass::Unknown,
        }
    }

    /// Idempotency expectations derived from capability flags + retry class.
    pub const fn idempotency_class(&self) -> IdempotencyClass {
        if !self.supports_idempotency {
            return IdempotencyClass::None;
        }
        match self.retry_class {
            RetryClass::IdempotentWrite => IdempotencyClass::RequiredForRetry,
            RetryClass::NonRetryableWrite
            | RetryClass::NeverAutoRetry
            | RetryClass::ProviderDefined
            | RetryClass::Unknown => IdempotencyClass::NeverRetry,
            RetryClass::SafeRead => IdempotencyClass::Optional,
        }
    }

    /// Pagination policy.
    pub const fn pagination_policy(&self) -> PaginationPolicy {
        if self.paginated {
            PaginationPolicy::GuardedCursor
        } else {
            PaginationPolicy::None
        }
    }

    /// Testmode attachment policy.
    pub const fn testmode_policy(&self) -> TestmodePolicy {
        if self.supports_testmode {
            TestmodePolicy::Supported
        } else {
            TestmodePolicy::Unsupported
        }
    }

    /// Profile scope attachment policy.
    pub const fn profile_scope(&self) -> ProfileScope {
        if self.requires_profile_scope {
            ProfileScope::RequiredOrSupported
        } else {
            ProfileScope::Unsupported
        }
    }

    /// Returns `true` when this operation is in the frozen high-risk write set
    /// (INV-TIER-01 denominator; lockstep with
    /// `scripts/check_dangerous_profile_drift.py`).
    pub fn is_high_risk_write(&self) -> bool {
        HIGH_RISK_WRITE_OPERATION_IDS.contains(&self.operation_id)
    }

    /// Returns `true` when profile + access mark this high-risk op as Tier-S
    /// protected (`ValidatedFacade` + coherent write retry class).
    pub fn is_fully_protected_high_risk(&self) -> bool {
        self.is_high_risk_write()
            && matches!(self.access, RouteAccess::ValidatedFacade)
            && !matches!(self.retry_class, RetryClass::Unknown | RetryClass::SafeRead)
    }

    /// Derived operation risk class (INV-DRIFT-03 metadata; single SSOT table).
    pub fn operation_risk(&self) -> OperationRisk {
        let method = self.http_method;
        let is_write = matches!(method, "POST" | "PUT" | "PATCH" | "DELETE");
        if !is_write {
            return OperationRisk::ReadOnly;
        }
        let id = self.operation_id;
        if PAYMENT_CAPABILITY_MUTATION_OPERATION_IDS.contains(&id) {
            return OperationRisk::PaymentCapabilityMutation;
        }
        if id.starts_with("terminals_") || id.contains("pairing") {
            return OperationRisk::TerminalSecurityMutation;
        }
        if id.starts_with("oauth_") {
            return OperationRisk::CredentialMutation;
        }
        if id.contains("session") {
            return OperationRisk::PiiCollectionMutation;
        }
        if id.starts_with("cancel_") {
            return OperationRisk::FinancialCancellation;
        }
        if HIGH_RISK_WRITE_OPERATION_IDS.contains(&id)
            || id.starts_with("create_")
            || id.contains("payment")
            || id.contains("refund")
            || id.contains("payout")
            || id.contains("transfer")
            || id.contains("capture")
            || id.contains("mandate")
            || id.contains("subscription")
        {
            return OperationRisk::FinancialWrite;
        }
        if id.contains("profile") || id.contains("organization") || id.contains("permission") {
            return OperationRisk::AccountConfigurationMutation;
        }
        if matches!(self.retry_class, RetryClass::Unknown) {
            return OperationRisk::Unknown;
        }
        OperationRisk::AccountConfigurationMutation
    }

    /// Derived exposure class for Tier-G vs Tier-S promotion.
    pub fn operation_exposure(&self) -> OperationExposure {
        match self.access {
            RouteAccess::ValidatedFacade => OperationExposure::ValidatedFacade,
            RouteAccess::GeneratedClient => {
                let is_write = matches!(self.http_method, "POST" | "PUT" | "PATCH" | "DELETE");
                if is_write && matches!(self.retry_class, RetryClass::Unknown) {
                    OperationExposure::GeneratedQuarantined
                } else {
                    OperationExposure::Generated
                }
            }
        }
    }
}

/// Payment method / issuer capability mutations (not always in HR denominator).
pub const PAYMENT_CAPABILITY_MUTATION_OPERATION_IDS: &[&str] = &[
    "enable_method",
    "disable_method",
    "enable_method_issuer",
    "disable_method_issuer",
];

/// Frozen high-risk write operation ids (CI denominator).
pub const HIGH_RISK_WRITE_OPERATION_IDS: &[&str] = &[
    "create_payment",
    "cancel_payment",
    "create_refund",
    "cancel_refund",
    "create_capture",
    "create_subscription",
    "cancel_subscription",
    "create_mandate",
    "create_payment_link",
    "create_customer_payment",
    "create_payout",
    "cancel_payout",
    "create_transfer",
    "create_connect_balance_transfer",
    "verify_payee",
    "oauth_generate_tokens",
    "oauth_revoke_tokens",
    "payment_create_route",
    "create_session",
    "terminals_request_pairing_code",
    "terminals_revoke_pairing_code",
    "match_unmatched_credit_transfer",
    "return_unmatched_credit_transfer",
];

/// Looks up the safety profile for an operation id.
pub fn operation_safety_profile(operation_id: &str) -> Option<&'static OperationSafetyProfile> {
    route_capability(operation_id)
}

/// All profiles (same slice as [`ROUTE_CAPABILITIES`]).
pub fn all_operation_safety_profiles() -> &'static [OperationSafetyProfile] {
    ROUTE_CAPABILITIES
}

/// Counts fully protected high-risk writes vs frozen denominator.
///
/// Returns `(fully_protected, total_high_risk)`. Fully protected means
/// [`RouteCapability::is_fully_protected_high_risk`].
pub fn high_risk_coverage() -> (usize, usize) {
    let total = HIGH_RISK_WRITE_OPERATION_IDS.len();
    let fully = ROUTE_CAPABILITIES
        .iter()
        .filter(|p| p.is_fully_protected_high_risk())
        .count();
    (fully, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_cover_all_capabilities() {
        assert_eq!(all_operation_safety_profiles().len(), 124);
        assert!(operation_safety_profile("create_payment").is_some());
    }

    #[test]
    fn oauth_is_oauth_client_auth_and_never_retry() {
        let p = operation_safety_profile("oauth_generate_tokens").unwrap();
        assert_eq!(p.auth_class(), AuthClass::OAuthClient);
        assert_eq!(p.idempotency_class(), IdempotencyClass::NeverRetry);
        assert_eq!(
            p.mutation_class(),
            MutationClass::FinancialOrNonRetryableWrite
        );
    }

    #[test]
    fn create_payment_requires_key_for_retry() {
        let p = operation_safety_profile("create_payment").unwrap();
        assert_eq!(p.mutation_class(), MutationClass::IdempotentWrite);
        assert_eq!(p.idempotency_class(), IdempotencyClass::RequiredForRetry);
        assert_eq!(p.auth_class(), AuthClass::ApiCredential);
    }

    #[test]
    fn high_risk_coverage_is_complete() {
        let (fully, total) = high_risk_coverage();
        assert_eq!(total, HIGH_RISK_WRITE_OPERATION_IDS.len());
        assert_eq!(
            fully, total,
            "every frozen high-risk op must be ValidatedFacade with write retry class"
        );
        for id in HIGH_RISK_WRITE_OPERATION_IDS {
            let p = operation_safety_profile(id).expect(id);
            assert!(p.is_fully_protected_high_risk(), "{id}");
        }
    }

    #[test]
    fn payment_capability_mutations_are_classified() {
        for id in PAYMENT_CAPABILITY_MUTATION_OPERATION_IDS {
            let p = operation_safety_profile(id).expect(id);
            assert_eq!(
                p.operation_risk(),
                OperationRisk::PaymentCapabilityMutation,
                "{id}"
            );
            assert_ne!(p.mutation_class(), MutationClass::Read);
            assert_eq!(p.operation_exposure(), OperationExposure::Generated);
        }
    }

    #[test]
    fn create_payment_risk_and_exposure() {
        let p = operation_safety_profile("create_payment").unwrap();
        assert_eq!(p.operation_risk(), OperationRisk::FinancialWrite);
        assert_eq!(p.operation_exposure(), OperationExposure::ValidatedFacade);
    }
}
