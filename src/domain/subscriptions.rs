//! Subscription-domain facade for customer-scoped subscriptions.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, ListSubscriptionsResponse, SubscriptionResponse};
use crate::{
    CreateSubscriptionRequired, CustomerId, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, ResponseEnvelope, SubscriptionId,
};

type SubscriptionPageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListSubscriptionResponse>>> + Send>>;

/// Subscription operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct SubscriptionsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the subscriptions domain facade.
    pub fn subscriptions(&self) -> SubscriptionsApi<'_> {
        SubscriptionsApi { client: self }
    }
}

impl SubscriptionsApi<'_> {
    /// Creates a subscription from a **validated** required-fields builder.
    pub async fn create(
        &self,
        customer_id: &CustomerId,
        required: CreateSubscriptionRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<SubscriptionResponse> {
        let body = required.into_request()?;
        self.create_raw(customer_id, &body, key).await
    }

    /// Creates a subscription from a generated request body (advanced).
    pub async fn create_raw(
        &self,
        customer_id: &CustomerId,
        body: &types::CreateSubscriptionRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<SubscriptionResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        client_with_key(self.client, key)
            .create_subscription(&customer, body)
            .into_mollie_result()
            .await
    }

    /// Fetches a subscription.
    pub async fn get(
        &self,
        customer_id: &CustomerId,
        subscription_id: &SubscriptionId,
    ) -> MollieResponse<SubscriptionResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let subscription = types::SubscriptionToken(subscription_id.as_str().to_string());
        self.client
            .get_subscription(&customer, &subscription)
            .into_mollie_result()
            .await
    }

    /// Updates a subscription.
    pub async fn update(
        &self,
        customer_id: &CustomerId,
        subscription_id: &SubscriptionId,
        body: &types::UpdateSubscriptionBody,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<SubscriptionResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let subscription = types::SubscriptionToken(subscription_id.as_str().to_string());
        client_with_key(self.client, key)
            .update_subscription(&customer, &subscription, body)
            .into_mollie_result()
            .await
    }

    /// Cancels a subscription.
    pub async fn cancel(
        &self,
        customer_id: &CustomerId,
        subscription_id: &SubscriptionId,
        body: Option<&types::CancelSubscriptionBody>,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<SubscriptionResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let subscription = types::SubscriptionToken(subscription_id.as_str().to_string());
        let default_body = types::CancelSubscriptionBody::default();
        let body = body.unwrap_or(&default_body);
        client_with_key(self.client, key)
            .cancel_subscription(&customer, &subscription, body)
            .into_mollie_result()
            .await
    }

    /// Lists one page of subscriptions for a customer.
    pub async fn list_page(
        &self,
        customer_id: &CustomerId,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListSubscriptionResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let from_token = from.map(|c| types::SubscriptionToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListSubscriptionsResponse> = self
            .client
            .list_subscriptions(&customer, from_token.as_ref(), limit_nz, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_subscriptions(envelope))
    }

    /// Lists all subscriptions for a customer within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListSubscriptionResponse>> {
        let mut items = Vec::new();
        let mut cursor: Option<PageCursor> = None;
        loop {
            let page = self.list_page(customer_id, cursor.as_ref(), limit).await?;
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

    /// Streams subscription pages for a customer within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<
        impl FnMut(Option<PageCursor>) -> SubscriptionPageFut,
        types::ListSubscriptionResponse,
    > {
        let client = self.client.clone();
        let customer = customer_id.clone();
        stream_pages(guard, move |cursor| -> SubscriptionPageFut {
            let client = client.clone();
            let customer = customer.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                SubscriptionsApi { client: &client }
                    .list_page(&customer, cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams subscription items for a customer within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<
        impl FnMut(Option<PageCursor>) -> SubscriptionPageFut,
        types::ListSubscriptionResponse,
    > {
        let client = self.client.clone();
        let customer = customer_id.clone();
        stream_items(guard, move |cursor| -> SubscriptionPageFut {
            let client = client.clone();
            let customer = customer.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                SubscriptionsApi { client: &client }
                    .list_page(&customer, cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list_subscriptions(
    envelope: ResponseEnvelope<ListSubscriptionsResponse>,
) -> Page<types::ListSubscriptionResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.subscriptions, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCount, ListLinks, ListSubscriptionsResponseEmbedded, Url, UrlNullable};
    use reqwest::StatusCode;

    #[test]
    fn maps_subscription_list_next_cursor() {
        let response = ListSubscriptionsResponse {
            count: ListCount(0),
            embedded: ListSubscriptionsResponseEmbedded {
                subscriptions: vec![],
            },
            links: ListLinks {
                documentation: Url {
                    href: "https://docs.mollie.com".into(),
                    type_: "text/html".into(),
                },
                next: UrlNullable(Some(types::UrlNullableInner {
                    href: Some(
                        "https://api.mollie.com/v2/customers/cst_x/subscriptions?from=sub_next"
                            .into(),
                    ),
                    type_: Some("application/hal+json".into()),
                })),
                previous: UrlNullable(None),
                self_: Url {
                    href: "https://api.mollie.com/v2/customers/cst_x/subscriptions".into(),
                    type_: "application/hal+json".into(),
                },
            },
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_subscriptions(env);
        assert_eq!(page.next.as_ref().map(PageCursor::as_str), Some("sub_next"));
    }
}
