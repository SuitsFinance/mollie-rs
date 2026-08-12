//! Fixture coverage for `list_capabilities` success and global errors.
//!
//! Locks the real Mollie 200 body shape against
//! [`types::ListCapabilitiesResponse`] and the success envelope path.
//!
//! Also locks the **global HTTP 429** HAL body (shown on list_capabilities in
//! Postman and every other route when rate-limited) against the error factory
//! and error envelope.

#[cfg(test)]
mod tests {
    use crate::{
        types::{
            CapabilityRequirementStatus, CapabilityStatus, CapabilityStatusReasonInner,
            ErrorResponse, ListCapabilitiesResponse,
        },
        MollieError, MollieErrorCatalogEntry, MollieErrorKey, MollieSuccessKey, ResponseEnvelope,
    };
    use reqwest::{header::HeaderMap, StatusCode};

    /// Global rate-limit body (same shape on `list_capabilities` and all routes).
    const GLOBAL_429: &str = r#"{
  "status": 429,
  "title": "Too Many Requests",
  "detail": "You have exceeded the rate limit. Please slow down your requests.",
  "_links": {
    "documentation": {
      "href": "https://docs.mollie.com/overview/handling-errors",
      "type": "text/html"
    }
  }
}"#;

    /// Real-world-style `GET /capabilities` 200 body (Postman / live API).
    const LIST_CAPABILITIES_200: &str = r#"{
  "count": 2,
  "_embedded": {
    "capabilities": [
      {
        "resource": "capability",
        "name": "payments",
        "status": "enabled",
        "statusReason": null,
        "requirements": []
      },
      {
        "resource": "capability",
        "name": "settlements",
        "status": "pending",
        "statusReason": "onboarding-information-needed",
        "requirements": [
          {
            "id": "process-first-payment",
            "dueDate": null,
            "status": "requested",
            "_links": {
              "dashboard": {
                "href": "https://my.mollie.com/dashboard/...",
                "type": "text/html"
              }
            }
          },
          {
            "id": "needs-data",
            "dueDate": "2024-05-14T01:29:09+00:00",
            "status": "past-due",
            "_links": {
              "dashboard": {
                "href": "https://my.mollie.com/dashboard/...",
                "type": "text/html"
              }
            }
          }
        ]
      }
    ]
  },
  "_links": {
    "documentation": {
      "href": "https://docs.mollie.com/reference/list-capabilities",
      "type": "text/html"
    }
  }
}"#;

    #[test]
    fn deserializes_list_capabilities_200_body() {
        let body: ListCapabilitiesResponse =
            serde_json::from_str(LIST_CAPABILITIES_200).expect("capabilities JSON should decode");

        assert_eq!(body.count, 2);

        let caps = body.embedded.capabilities.as_slice();
        assert_eq!(caps.len(), 2);

        let payments = &caps[0];
        assert_eq!(payments.resource, "capability");
        assert_eq!(payments.name, "payments");
        assert_eq!(payments.status, CapabilityStatus::Enabled);
        assert!(payments.status_reason.0.is_none());
        assert!(payments.requirements.is_empty());

        let settlements = &caps[1];
        assert_eq!(settlements.name, "settlements");
        assert_eq!(settlements.status, CapabilityStatus::Pending);
        assert_eq!(
            settlements.status_reason.0,
            Some(CapabilityStatusReasonInner::OnboardingInformationNeeded)
        );
        assert_eq!(settlements.requirements.len(), 2);

        let first_req = &settlements.requirements[0];
        assert_eq!(first_req.id, "process-first-payment");
        assert_eq!(first_req.due_date, None);
        assert_eq!(first_req.status, CapabilityRequirementStatus::Requested);
        assert_eq!(
            first_req.links.dashboard.as_ref().map(|d| d.href.as_str()),
            Some("https://my.mollie.com/dashboard/...")
        );

        let second_req = &settlements.requirements[1];
        assert_eq!(second_req.id, "needs-data");
        assert_eq!(
            second_req.due_date.as_deref(),
            Some("2024-05-14T01:29:09+00:00")
        );
        assert_eq!(second_req.status, CapabilityRequirementStatus::PastDue);

        let docs = body.links.documentation.as_ref().map(|d| d.href.as_str());
        assert_eq!(
            docs,
            Some("https://docs.mollie.com/reference/list-capabilities")
        );
    }

    #[test]
    fn list_capabilities_success_envelope() {
        let body: ListCapabilitiesResponse =
            serde_json::from_str(LIST_CAPABILITIES_200).expect("capabilities JSON should decode");

        let success = ResponseEnvelope::from_parts(body, StatusCode::OK, Default::default())
            .to_success_envelope();

        assert!(success.ok);
        assert_eq!(success.status, 200);
        assert_eq!(success.code, 20000);
        assert_eq!(success.key, MollieSuccessKey::Ok);
        assert_eq!(success.message_key, "success.ok");
        assert_eq!(success.data.count, 2);
        assert_eq!(success.data.embedded.capabilities.len(), 2);
    }

    /// Global 429 (also returned by `list_capabilities` when rate-limited).
    #[test]
    fn global_429_classifies_and_matches_error_factory_envelope() {
        let body: ErrorResponse =
            serde_json::from_str(GLOBAL_429).expect("429 HAL JSON should decode");

        assert_eq!(body.status, 429);
        assert_eq!(body.title, "Too Many Requests");
        assert!(body.detail.contains("rate limit"));

        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(entry, MollieErrorCatalogEntry::RATE_LIMIT_EXCEEDED);
        assert_eq!(entry.code(), 42901);
        assert_eq!(entry.key(), MollieErrorKey::RateLimitExceeded);

        let from_body = MollieError::api(StatusCode::TOO_MANY_REQUESTS, HeaderMap::new(), body);
        let envelope = from_body.to_envelope();

        assert!(!envelope.ok);
        assert_eq!(envelope.status, Some(429));
        assert_eq!(envelope.code, 42901);
        assert_eq!(envelope.key.as_str(), "RATE_LIMIT_EXCEEDED");
        assert_eq!(
            envelope.message_key,
            "errors.too_many_requests.rate_limit_exceeded"
        );
        assert_eq!(envelope.title.as_deref(), Some("Too Many Requests"));
        assert_eq!(
            envelope.detail,
            "You have exceeded the rate limit. Please slow down your requests."
        );
        assert_eq!(
            envelope.documentation.as_deref(),
            Some("https://docs.mollie.com/overview/handling-errors")
        );
        assert!(from_body.is_rate_limited());

        // Error factory must produce the same catalog identity + title.
        let factory_envelope = MollieError::rate_limit_exceeded().to_envelope();
        assert_eq!(factory_envelope.title, envelope.title);
        assert_eq!(factory_envelope.code, envelope.code);
        assert_eq!(factory_envelope.key, envelope.key);
        assert_eq!(factory_envelope.status, envelope.status);
        assert!(!factory_envelope.ok);
    }
}
