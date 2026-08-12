//! Refund-domain facade for payment-scoped refund operations.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, EntityRefundResponse, ListRefundsResponse};
use crate::{
    CreateRefundRequired, EmptyResponse, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, PaymentId, RefundId, ResponseEnvelope,
};

type RefundPageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListEntityRefund>>> + Send>>;

/// Refund operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct RefundsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the refunds domain facade.
    pub fn refunds(&self) -> RefundsApi<'_> {
        RefundsApi { client: self }
    }
}

impl RefundsApi<'_> {
    /// Creates a refund from a **validated** required-fields builder.
    pub async fn create(
        &self,
        payment_id: &PaymentId,
        required: CreateRefundRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityRefundResponse> {
        let body = required.into_request()?;
        self.create_raw(payment_id, &body, key).await
    }

    /// Creates a refund from a generated request body (advanced).
    pub async fn create_raw(
        &self,
        payment_id: &PaymentId,
        body: &types::EntityRefund,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityRefundResponse> {
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        client_with_key(self.client, key)
            .create_refund(&payment, &types::RefundRequest(body.clone()))
            .into_mollie_result()
            .await
    }

    /// Fetches a refund by payment + refund id.
    pub async fn get(
        &self,
        payment_id: &PaymentId,
        refund_id: &RefundId,
    ) -> MollieResponse<EntityRefundResponse> {
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        let refund = types::RefundToken(refund_id.as_str().to_string());
        self.client
            .get_refund(&payment, &refund, None)
            .into_mollie_result()
            .await
    }

    /// Cancels a refund that is still cancelable.
    ///
    /// Mollie may return an empty body; the facade maps that to [`EmptyResponse`].
    pub async fn cancel(
        &self,
        payment_id: &PaymentId,
        refund_id: &RefundId,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EmptyResponse> {
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        let refund = types::RefundToken(refund_id.as_str().to_string());
        client_with_key(self.client, key)
            .cancel_refund(&payment, &refund)
            .into_mollie_result()
            .await
            .map(|envelope| envelope.map(|_| EmptyResponse::new()))
    }

    /// Lists one page of refunds for a payment.
    pub async fn list_page(
        &self,
        payment_id: &PaymentId,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListEntityRefund>> {
        let limit_nz = validate_page_limit(limit)?;
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        let from_token = from.map(|c| types::RefundToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListRefundsResponse> = self
            .client
            .list_refunds(&payment, None, from_token.as_ref(), limit_nz)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_refunds(envelope))
    }

    /// Lists all refunds for a payment within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListEntityRefund>> {
        let mut items = Vec::new();
        let mut cursor: Option<PageCursor> = None;
        loop {
            let page = self.list_page(payment_id, cursor.as_ref(), limit).await?;
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

    /// Streams refund pages for a payment within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> RefundPageFut, types::ListEntityRefund>
    {
        let client = self.client.clone();
        let payment = payment_id.clone();
        stream_pages(guard, move |cursor| -> RefundPageFut {
            let client = client.clone();
            let payment = payment.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                RefundsApi { client: &client }
                    .list_page(&payment, cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams refund items for a payment within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> RefundPageFut, types::ListEntityRefund> {
        let client = self.client.clone();
        let payment = payment_id.clone();
        stream_items(guard, move |cursor| -> RefundPageFut {
            let client = client.clone();
            let payment = payment.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                RefundsApi { client: &client }
                    .list_page(&payment, cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list_refunds(
    envelope: ResponseEnvelope<ListRefundsResponse>,
) -> Page<types::ListEntityRefund> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.refunds, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCount, ListLinks, ListRefundsResponseEmbedded, Url, UrlNullable};
    use crate::Money;
    use reqwest::StatusCode;

    #[test]
    fn maps_refund_list_next_cursor() {
        let response = ListRefundsResponse {
            count: ListCount(0),
            embedded: ListRefundsResponseEmbedded { refunds: vec![] },
            links: ListLinks {
                documentation: Url {
                    href: "https://docs.mollie.com".into(),
                    type_: "text/html".into(),
                },
                next: UrlNullable(Some(types::UrlNullableInner {
                    href: Some(
                        "https://api.mollie.com/v2/payments/tr_x/refunds?from=re_next".into(),
                    ),
                    type_: Some("application/hal+json".into()),
                })),
                previous: UrlNullable(None),
                self_: Url {
                    href: "https://api.mollie.com/v2/payments/tr_x/refunds".into(),
                    type_: "application/hal+json".into(),
                },
            },
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_refunds(env);
        assert_eq!(page.next.as_ref().map(PageCursor::as_str), Some("re_next"));
    }

    #[test]
    fn validated_refund_builder_serializes() {
        let body = CreateRefundRequired::new(Money::new("EUR", "1.00").unwrap(), "Partial")
            .unwrap()
            .into_request()
            .unwrap();
        assert!(!body.amount.value.is_empty());
    }
}
