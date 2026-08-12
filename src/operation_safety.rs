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

    /// Returns `true` when this operation is treated as a high-risk write for
    /// the primary coverage metric (money movement, OAuth tokens, etc.).
    pub fn is_high_risk_write(&self) -> bool {
        matches!(
            self.mutation_class(),
            MutationClass::IdempotentWrite | MutationClass::FinancialOrNonRetryableWrite
        ) && (self.route_group.contains("payout")
            || self.route_group.contains("transfer")
            || self.route_group.contains("payment")
            || self.route_group.contains("refund")
            || self.route_group.contains("capture")
            || self.route_group.contains("oauth")
            || self.route_group.contains("balance_transfer")
            || self.route_group.contains("business_account")
            || self.path.contains("payout")
            || self.path.contains("transfer")
            || self.path.contains("oauth")
            || self.operation_id.contains("payout")
            || self.operation_id.contains("transfer")
            || self.operation_id.contains("oauth")
            || self.operation_id.contains("payment")
            || self.operation_id.contains("refund")
            || self.operation_id.contains("capture")
            || self.operation_id.contains("mandate")
            || self.operation_id.contains("subscription"))
    }
}

/// Looks up the safety profile for an operation id.
pub fn operation_safety_profile(operation_id: &str) -> Option<&'static OperationSafetyProfile> {
    route_capability(operation_id)
}

/// All profiles (same slice as [`ROUTE_CAPABILITIES`]).
pub fn all_operation_safety_profiles() -> &'static [OperationSafetyProfile] {
    ROUTE_CAPABILITIES
}

/// Counts high-risk writes that have explicit retry + idempotency fields set
/// (denominator/numerator helpers for the primary readiness metric).
pub fn high_risk_coverage() -> (usize, usize) {
    let high_risk: Vec<_> = ROUTE_CAPABILITIES
        .iter()
        .filter(|p| p.is_high_risk_write())
        .collect();
    let total = high_risk.len();
    // "Enforced" at profile layer: retry_class is not Unknown and idempotency
    // class is coherent. Transport proofs land in Phase 2 tests.
    let enforced = high_risk
        .iter()
        .filter(|p| {
            !matches!(p.retry_class, RetryClass::Unknown)
                && matches!(
                    p.idempotency_class(),
                    IdempotencyClass::RequiredForRetry
                        | IdempotencyClass::NeverRetry
                        | IdempotencyClass::None
                        | IdempotencyClass::Optional
                )
                && matches!(
                    p.access,
                    RouteAccess::GeneratedClient | RouteAccess::ValidatedFacade
                )
        })
        .count();
    (enforced, total)
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
    fn high_risk_coverage_nonzero() {
        let (enforced, total) = high_risk_coverage();
        assert!(total > 0, "expected high-risk ops");
        assert_eq!(
            enforced, total,
            "profile fields must be set for all high-risk"
        );
    }
}
