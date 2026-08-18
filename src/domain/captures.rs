//! Capture-domain facade for payment-scoped captures.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, CaptureResponse, ListCapturesResponse};
use crate::{
    CaptureId, CreateCaptureRequired, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, PaymentId, ResponseEnvelope,
};

type CapturePageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListCaptureResponse>>> + Send>>;

/// Capture operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct CapturesApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the captures domain facade.
    pub fn captures(&self) -> CapturesApi<'_> {
        CapturesApi { client: self }
    }
}

impl CapturesApi<'_> {
    /// Creates a capture from a **validated** required-fields builder.
    pub async fn create(
        &self,
        payment_id: &PaymentId,
        required: CreateCaptureRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<CaptureResponse> {
        let body = required.into_request()?;
        self.create_raw(payment_id, &body, key).await
    }

    /// Creates a capture from a generated request body (advanced).
    pub async fn create_raw(
        &self,
        payment_id: &PaymentId,
        body: &types::EntityCapture,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<CaptureResponse> {
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        client_with_key(self.client, key)
            .create_capture(&payment, body)
            .into_mollie_result()
            .await
    }

    /// Fetches a capture by payment + capture id.
    pub async fn get(
        &self,
        payment_id: &PaymentId,
        capture_id: &CaptureId,
    ) -> MollieResponse<CaptureResponse> {
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        self.client
            .get_capture(
                &payment,
                &types::CaptureToken(capture_id.as_str().to_string()),
                None,
            )
            .into_mollie_result()
            .await
    }

    /// Lists one page of captures for a payment.
    pub async fn list_page(
        &self,
        payment_id: &PaymentId,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListCaptureResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let payment = types::PaymentToken(payment_id.as_str().to_string());
        let from_token = from.map(|c| types::CaptureToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListCapturesResponse> = self
            .client
            .list_captures(&payment, None, from_token.as_ref(), limit_nz)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_captures(envelope))
    }

    /// Lists all captures for a payment within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListCaptureResponse>> {
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

    /// Streams capture pages for a payment within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> CapturePageFut, types::ListCaptureResponse>
    {
        let client = self.client.clone();
        let payment = payment_id.clone();
        stream_pages(guard, move |cursor| -> CapturePageFut {
            let client = client.clone();
            let payment = payment.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                CapturesApi { client: &client }
                    .list_page(&payment, cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams capture items for a payment within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        payment_id: &PaymentId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> CapturePageFut, types::ListCaptureResponse>
    {
        let client = self.client.clone();
        let payment = payment_id.clone();
        stream_items(guard, move |cursor| -> CapturePageFut {
            let client = client.clone();
            let payment = payment.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                CapturesApi { client: &client }
                    .list_page(&payment, cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list_captures(
    envelope: ResponseEnvelope<ListCapturesResponse>,
) -> Page<types::ListCaptureResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.captures, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCapturesResponseEmbedded, ListCount, ListLinks, Url, UrlNullable};
    use reqwest::StatusCode;

    #[test]
    fn maps_capture_list_next_cursor() {
        let response = ListCapturesResponse {
            count: ListCount(0),
            embedded: ListCapturesResponseEmbedded { captures: vec![] },
            links: ListLinks {
                documentation: Url {
                    href: "https://docs.mollie.com".into(),
                    type_: "text/html".into(),
                },
                next: UrlNullable(Some(types::UrlNullableInner {
                    href: Some(
                        "https://api.mollie.com/v2/payments/tr_x/captures?from=cpt_next".into(),
                    ),
                    type_: Some("application/hal+json".into()),
                })),
                previous: UrlNullable(None),
                self_: Url {
                    href: "https://api.mollie.com/v2/payments/tr_x/captures".into(),
                    type_: "application/hal+json".into(),
                },
            },
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_captures(env);
        assert_eq!(page.next.as_ref().map(PageCursor::as_str), Some("cpt_next"));
    }
}
