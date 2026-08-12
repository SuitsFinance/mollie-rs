//! Deduped HAL error fixtures harvested from Mollie's Postman collections.
//!
//! One canonical body per HTTP status family (plus 422 split). Asserts
//! classification + consistent error envelope (`ok: false`, code, key, title).

#[cfg(test)]
mod tests {
    use crate::prelude::{
        MollieError, MollieErrorCatalogEntry, MollieErrorEnvelope, MollieErrorKey,
    };
    use crate::types::{ErrorResponse, ErrorResponseLinks, ErrorResponseLinksDocumentation};
    use reqwest::{header::HeaderMap, StatusCode};

    fn docs_links(href: &str) -> ErrorResponseLinks {
        ErrorResponseLinks {
            documentation: ErrorResponseLinksDocumentation {
                href: href.to_string(),
                type_: "text/html".to_string(),
            },
        }
    }

    fn error_body(
        status: i64,
        title: &str,
        detail: &str,
        field: Option<&str>,
        docs_href: &str,
    ) -> ErrorResponse {
        ErrorResponse {
            status,
            title: title.to_string(),
            detail: detail.to_string(),
            field: field.map(str::to_string),
            links: docs_links(docs_href),
        }
    }

    fn classify_envelope(
        status: StatusCode,
        body: ErrorResponse,
    ) -> (MollieErrorCatalogEntry, MollieErrorEnvelope) {
        let entry: MollieErrorCatalogEntry = MollieErrorCatalogEntry::classify_api(&body);
        let envelope: MollieErrorEnvelope =
            MollieError::api(status, HeaderMap::new(), body).to_envelope();
        assert!(!envelope.ok);
        assert_eq!(envelope.status, Some(status.as_u16()));
        assert_eq!(envelope.code, entry.code());
        assert_eq!(envelope.key, entry.key());
        (entry, envelope)
    }

    #[test]
    fn postman_400_invalid_cursor() {
        let body: ErrorResponse = error_body(
            400,
            "Bad Request",
            "Invalid cursor value",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::BAD_REQUEST, body);
        assert_eq!(entry.key(), MollieErrorKey::InvalidCursor);
        assert_eq!(entry.code(), 40001);
        assert_eq!(env.title.as_deref(), Some("Bad Request"));
        assert_eq!(
            MollieError::invalid_cursor().catalog_entry().key(),
            MollieErrorKey::InvalidCursor
        );
    }

    #[test]
    fn postman_403_demo_profiles() {
        let limit: ErrorResponse = error_body(
            403,
            "Forbidden",
            "Profile limit has been reached for demo accounts.",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::FORBIDDEN, limit);
        assert_eq!(entry.key(), MollieErrorKey::DemoProfileLimitReached);
        assert_eq!(env.title.as_deref(), Some("Forbidden"));
        assert_eq!(
            MollieError::demo_profile_limit_reached()
                .catalog_entry()
                .key(),
            MollieErrorKey::DemoProfileLimitReached
        );

        let editable: ErrorResponse = error_body(
            403,
            "Forbidden",
            "This profile cannot be edited because it belongs to a demo account.",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, _env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::FORBIDDEN, editable);
        assert_eq!(entry.key(), MollieErrorKey::DemoProfileNotEditable);
    }

    #[test]
    fn postman_404_entity_not_found() {
        let body: ErrorResponse = error_body(
            404,
            "Not Found",
            "No entity exists with token 'uct_abcDEFghij123456789'",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::NOT_FOUND, body);
        assert_eq!(entry.key(), MollieErrorKey::EntityNotFound);
        assert_eq!(entry.code(), 40401);
        assert_eq!(env.title.as_deref(), Some("Not Found"));
    }

    #[test]
    fn postman_409_payout_not_cancelable() {
        let body: ErrorResponse = error_body(
            409,
            "Conflict",
            "The payout cannot be canceled in its current state.",
            None,
            "https://docs.mollie.com/errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::CONFLICT, body);
        assert_eq!(entry.key(), MollieErrorKey::PayoutNotCancelable);
        assert_eq!(entry.code(), 40901);
        assert_eq!(env.title.as_deref(), Some("Conflict"));
        assert_eq!(
            MollieError::payout_not_cancelable().catalog_entry().key(),
            MollieErrorKey::PayoutNotCancelable
        );
    }

    #[test]
    fn postman_410_profile_deleted() {
        let body: ErrorResponse = error_body(
            410,
            "Gone",
            "Profile with token pfl_QkEhN94Ba has been deleted.",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::GONE, body);
        assert_eq!(entry.key(), MollieErrorKey::ProfileDeleted);
        assert_eq!(entry.code(), 41001);
        assert_eq!(env.title.as_deref(), Some("Gone"));
        assert_eq!(
            MollieError::profile_deleted("pfl_x").catalog_entry().key(),
            MollieErrorKey::ProfileDeleted
        );
    }

    #[test]
    fn postman_422_validation_and_state() {
        let missing: ErrorResponse = error_body(
            422,
            "Unprocessable Entity",
            "The 'description' field is missing",
            Some("description"),
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::UNPROCESSABLE_ENTITY, missing);
        assert_eq!(entry.key(), MollieErrorKey::ValidationError);
        assert_eq!(entry.code(), 42201);
        assert_eq!(env.title.as_deref(), Some("Unprocessable Entity"));
        assert_eq!(env.field.as_deref(), Some("description"));

        let state: ErrorResponse = error_body(
            422,
            "Unprocessable entity",
            "This subscription was already deleted.",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, _env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::UNPROCESSABLE_ENTITY, state);
        assert_eq!(entry.key(), MollieErrorKey::ResourceStateConflict);
        assert_eq!(entry.code(), 42202);
        assert_eq!(
            MollieError::resource_state_conflict("This subscription was already deleted.")
                .catalog_entry()
                .key(),
            MollieErrorKey::ResourceStateConflict
        );
    }

    #[test]
    fn postman_429_rate_limit() {
        let body: ErrorResponse = error_body(
            429,
            "Too Many Requests",
            "You have exceeded the rate limit. Please slow down your requests.",
            None,
            "https://docs.mollie.com/overview/handling-errors",
        );
        let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
            classify_envelope(StatusCode::TOO_MANY_REQUESTS, body);
        assert_eq!(entry.key(), MollieErrorKey::RateLimitExceeded);
        assert_eq!(entry.code(), 42901);
        assert_eq!(env.title.as_deref(), Some("Too Many Requests"));
        assert_eq!(
            env.documentation.as_deref(),
            Some("https://docs.mollie.com/overview/handling-errors")
        );
        // Global: same body + factory for list_clients (/v2/clients), list_capabilities, etc.
        let factory: MollieError = MollieError::rate_limit_exceeded();
        let factory_envelope: MollieErrorEnvelope = factory.to_envelope();
        assert!(factory.is_rate_limited());
        assert_eq!(factory_envelope.key, env.key);
        assert_eq!(factory_envelope.code, env.code);
        assert_eq!(factory_envelope.title, env.title);
    }

    #[test]
    fn postman_503_service_unavailable() {
        let details: [&str; 3] = [
            "An unexpected error occurred while processing the transfer. Please try again later.",
            "An unexpected error occurred while processing the verification request. Please try again later.",
            "Payment platform for this payment method temporarily not available",
        ];
        for detail in details {
            let body: ErrorResponse = error_body(
                503,
                "Service Unavailable",
                detail,
                None,
                "https://docs.mollie.com/overview/handling-errors",
            );
            let (entry, env): (MollieErrorCatalogEntry, MollieErrorEnvelope) =
                classify_envelope(StatusCode::SERVICE_UNAVAILABLE, body);
            assert_eq!(entry.key(), MollieErrorKey::ServiceTemporarilyUnavailable);
            assert_eq!(entry.code(), 50301);
            assert_eq!(env.title.as_deref(), Some("Service Unavailable"));
        }
        assert_eq!(
            MollieError::service_temporarily_unavailable(
                "Payment platform for this payment method temporarily not available"
            )
            .catalog_entry()
            .key(),
            MollieErrorKey::ServiceTemporarilyUnavailable
        );
    }
}
