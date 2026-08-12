//! Payment-link domain facade.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, ListPaymentLinksResponse, PaymentLinkResponse};
use crate::{
    CreatePaymentLinkRequired, EmptyResponse, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, PaymentLinkId, ResponseEnvelope,
};

type PaymentLinkPageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<PaymentLinkResponse>>> + Send>>;

/// Payment-link operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct PaymentLinksApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the payment-links domain facade.
    pub fn payment_links(&self) -> PaymentLinksApi<'_> {
        PaymentLinksApi { client: self }
    }
}

impl PaymentLinksApi<'_> {
    /// Creates a payment link from a **validated** required-fields builder.
    pub async fn create(
        &self,
        required: CreatePaymentLinkRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentLinkResponse> {
        let body = required.into_request()?;
        self.create_raw(&body, key).await
    }

    /// Creates a payment link from a generated request body (advanced).
    pub async fn create_raw(
        &self,
        body: &types::CreatePaymentLinkBody,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<PaymentLinkResponse> {
        client_with_key(self.client, key)
            .create_payment_link(body)
            .into_mollie_result()
            .await
    }

    /// Fetches a payment link by validated id.
    pub async fn get(&self, id: &PaymentLinkId) -> MollieResponse<PaymentLinkResponse> {
        let token = types::PaymentLinkToken(id.as_str().to_string());
        self.client
            .get_payment_link(&token)
            .into_mollie_result()
            .await
    }

    /// Deletes a payment link that has not been used.
    ///
    /// Mollie may return an empty body; the facade maps that to [`EmptyResponse`].
    pub async fn delete(
        &self,
        id: &PaymentLinkId,
        body: Option<&types::DeletePaymentLinkBody>,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EmptyResponse> {
        let token = types::PaymentLinkToken(id.as_str().to_string());
        let default_body = types::DeletePaymentLinkBody::default();
        let body = body.unwrap_or(&default_body);
        client_with_key(self.client, key)
            .delete_payment_link(&token, body)
            .into_mollie_result()
            .await
            .map(|envelope| envelope.map(|_| EmptyResponse::new()))
    }

    /// Lists one page of payment links.
    pub async fn list_page(
        &self,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<PaymentLinkResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let from_token = from.map(|c| types::PaymentLinkToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListPaymentLinksResponse> = self
            .client
            .list_payment_links(from_token.as_ref(), limit_nz)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_payment_links(envelope))
    }

    /// Lists all payment links within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<PaymentLinkResponse>> {
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

    /// Streams payment-link pages within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> PaymentLinkPageFut, PaymentLinkResponse>
    {
        let client = self.client.clone();
        stream_pages(guard, move |cursor| -> PaymentLinkPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                PaymentLinksApi { client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams payment-link items within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> PaymentLinkPageFut, PaymentLinkResponse> {
        let client = self.client.clone();
        stream_items(guard, move |cursor| -> PaymentLinkPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                PaymentLinksApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list_payment_links(
    envelope: ResponseEnvelope<ListPaymentLinksResponse>,
) -> Page<PaymentLinkResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.payment_links, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCount, ListLinks, ListPaymentLinksResponseEmbedded, Url, UrlNullable};
    use reqwest::StatusCode;

    #[test]
    fn maps_payment_link_list_next_cursor() {
        let response = ListPaymentLinksResponse {
            count: ListCount(0),
            embedded: ListPaymentLinksResponseEmbedded {
                payment_links: vec![],
            },
            links: ListLinks {
                documentation: Url {
                    href: "https://docs.mollie.com".into(),
                    type_: "text/html".into(),
                },
                next: UrlNullable(Some(types::UrlNullableInner {
                    href: Some("https://api.mollie.com/v2/payment-links?from=pl_next".into()),
                    type_: Some("application/hal+json".into()),
                })),
                previous: UrlNullable(None),
                self_: Url {
                    href: "https://api.mollie.com/v2/payment-links".into(),
                    type_: "application/hal+json".into(),
                },
            },
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_payment_links(env);
        assert_eq!(page.next.as_ref().map(PageCursor::as_str), Some("pl_next"));
    }
}
