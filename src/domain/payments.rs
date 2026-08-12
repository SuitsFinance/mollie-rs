//! Payment-domain facade for create / get / list with safe defaults.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, ListPaymentsResponse, PaymentResponse};
use crate::{
    CreatePaymentRequired, CustomerId, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, PaymentId, ResponseEnvelope,
};

type PaymentPageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListPaymentResponse>>> + Send>>;

/// Payment operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct PaymentsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the payments domain facade.
    pub fn payments(&self) -> PaymentsApi<'_> {
        PaymentsApi { client: self }
    }
}

impl PaymentsApi<'_> {
    /// Creates a payment from a **validated** required-fields builder.
    ///
    /// This is the preferred Tier S entry point: money, description, and
    /// redirect URL are validated before any HTTP call.
    pub async fn create(
        &self,
        required: CreatePaymentRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentResponse> {
        let body = required.into_payment_request();
        self.create_raw(&body, key).await
    }

    /// Creates a payment from a generated request body (advanced / full OpenAPI surface).
    ///
    /// Prefer [`Self::create`] unless you need generated-only fields.
    pub async fn create_raw(
        &self,
        body: &types::CreatePaymentRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentResponse> {
        client_with_key(self.client, key)
            .create_payment(None, body)
            .into_mollie_result()
            .await
    }

    /// Fetches a payment by validated id.
    pub async fn get(&self, id: &PaymentId) -> MollieResponse<PaymentResponse> {
        let token = types::PaymentToken(id.as_str().to_string());
        self.client
            .get_payment(&token, None, None)
            .into_mollie_result()
            .await
    }

    /// Cancels a payment that is still cancelable (IdempotentWrite).
    pub async fn cancel(
        &self,
        id: &PaymentId,
        body: Option<&types::CancelPaymentBody>,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentResponse> {
        let token = types::PaymentToken(id.as_str().to_string());
        let default_body = types::CancelPaymentBody::default();
        let body = body.unwrap_or(&default_body);
        client_with_key(self.client, key)
            .cancel_payment(&token, body)
            .into_mollie_result()
            .await
    }

    /// Creates a payment for an existing customer (IdempotentWrite).
    pub async fn create_for_customer(
        &self,
        customer_id: &CustomerId,
        required: CreatePaymentRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentResponse> {
        let body = required.into_payment_request();
        self.create_for_customer_raw(customer_id, &body, key).await
    }

    /// Creates a customer payment from a generated body (advanced).
    pub async fn create_for_customer_raw(
        &self,
        customer_id: &CustomerId,
        body: &types::CreatePaymentRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        client_with_key(self.client, key)
            .create_customer_payment(&customer, body)
            .into_mollie_result()
            .await
    }

    /// Fetches one list page of payments (`from` / `limit` semantics).
    pub async fn list_page(
        &self,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListPaymentResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let from_token = from.map(|c| types::PaymentToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListPaymentsResponse> = self
            .client
            .list_payments(from_token.as_ref(), limit_nz, None, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_payments(envelope))
    }

    /// Lists all payments within [`PaginationGuard`] budgets.
    ///
    /// Prefer [`Self::list_page`] for interactive UIs; use this for bounded
    /// reconciliation exports only.
    pub async fn list_all(
        &self,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListPaymentResponse>> {
        let mut items = Vec::new();
        let mut cursor: Option<PageCursor> = None;
        loop {
            let page = self.list_page(cursor.as_ref(), limit).await?;
            guard.observe_page(&page)?;
            let next = page.next.clone();
            items.extend(page.items);
            match next {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }
        Ok(items)
    }

    /// Streams payment pages within [`PaginationGuard`] budgets (never unbounded).
    pub fn stream_pages(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> PaymentPageFut, types::ListPaymentResponse>
    {
        let client = self.client.clone();
        stream_pages(guard, move |cursor| -> PaymentPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                PaymentsApi { client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams payment items within [`PaginationGuard`] budgets (never unbounded).
    pub fn stream_items(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> PaymentPageFut, types::ListPaymentResponse>
    {
        let client = self.client.clone();
        stream_items(guard, move |cursor| -> PaymentPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                PaymentsApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Creates a delayed Connect route for a payment (IdempotentWrite).
    ///
    /// Prefer this over raw Tier-G access so sticky idempotency is explicit and
    /// payment/org ids are normalized before send.
    pub async fn create_delayed_route(
        &self,
        payment_id: &PaymentId,
        amount: crate::Money,
        organization_id: &str,
        description: Option<&str>,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<types::RouteCreateResponse> {
        if organization_id.trim().is_empty() {
            return Err(crate::MollieError::invalid_request(
                "delayed route organizationId must not be empty",
            ));
        }
        let description = match description {
            Some(value) => Some(
                value
                    .parse::<types::RouteCreateRequestDescription>()
                    .map_err(|error| {
                        crate::MollieError::invalid_request(format!(
                            "delayed route description: {error}"
                        ))
                    })?,
            ),
            None => None,
        };
        let body = types::RouteCreateRequest {
            amount: amount.into_amount(),
            description,
            destination: types::RouteCreateRequestDestination {
                organization_id: types::OrganizationToken(organization_id.trim().to_string()),
                type_: types::RouteDestinationType::Organization,
            },
        };
        let token = types::PaymentToken(payment_id.as_str().to_string());
        client_with_key(self.client, key)
            .payment_create_route(&token, &body)
            .into_mollie_result()
            .await
    }

    /// Fetches a delayed Connect route for a payment.
    pub async fn get_delayed_route(
        &self,
        payment_id: &PaymentId,
        route_id: &str,
    ) -> MollieResponse<types::RouteGetResponse> {
        if route_id.trim().is_empty() {
            return Err(crate::MollieError::invalid_request(
                "delayed route id must not be empty",
            ));
        }
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        let route = types::ConnectRouteToken(route_id.trim().to_string());
        self.client
            .payment_get_route(&payment, &route)
            .into_mollie_result()
            .await
    }
}

fn page_from_list_payments(
    envelope: ResponseEnvelope<ListPaymentsResponse>,
) -> Page<types::ListPaymentResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.payments, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCount, ListLinks, ListPaymentsResponseEmbedded, Url, UrlNullable};
    use crate::Money;
    use reqwest::StatusCode;

    #[test]
    fn page_maps_next_link_from_query() {
        let links = ListLinks {
            documentation: Url {
                href: "https://docs.mollie.com".into(),
                type_: "text/html".into(),
            },
            next: UrlNullable(Some(crate::types::UrlNullableInner {
                href: Some("https://api.mollie.com/v2/payments?from=tr_next&limit=50".into()),
                type_: Some("application/hal+json".into()),
            })),
            previous: UrlNullable(None),
            self_: Url {
                href: "https://api.mollie.com/v2/payments".into(),
                type_: "application/hal+json".into(),
            },
        };
        let response = ListPaymentsResponse {
            count: ListCount(0),
            embedded: ListPaymentsResponseEmbedded { payments: vec![] },
            links,
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_payments(env);
        assert_eq!(page.next.as_ref().map(|c| c.as_str()), Some("tr_next"));
    }

    #[test]
    fn validated_builder_produces_request() {
        let required = CreatePaymentRequired::new(
            "Order #1",
            Money::new("EUR", "10.00").unwrap(),
            "https://example.com/return",
        )
        .unwrap();
        let body = required.into_payment_request();
        assert!(!body.amount.value.is_empty());
        assert!(body.redirect_url.is_some());
    }

    #[test]
    fn delayed_route_create_is_idempotent_write() {
        let p = crate::operation_safety_profile("payment_create_route").unwrap();
        assert_eq!(p.retry_class, crate::RetryClass::IdempotentWrite);
    }
}
