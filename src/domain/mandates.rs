//! Mandate-domain facade for customer-scoped mandates.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, ListMandatesResponse, MandateResponse};
use crate::{
    CreateSepaMandateRequired, CustomerId, EmptyResponse, IdempotencyKey, IntoMollieFuture,
    MandateId, MollieClient, MollieResponse, MollieResult, ResponseEnvelope,
};

type MandatePageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListMandateResponse>>> + Send>>;

/// Mandate operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct MandatesApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the mandates domain facade.
    pub fn mandates(&self) -> MandatesApi<'_> {
        MandatesApi { client: self }
    }
}

impl MandatesApi<'_> {
    /// Creates a SEPA Direct Debit mandate from a validated builder.
    pub async fn create_sepa(
        &self,
        customer_id: &CustomerId,
        required: CreateSepaMandateRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<MandateResponse> {
        let body = required.into_request()?;
        self.create(customer_id, &body, key).await
    }

    /// Creates a mandate from a generated request body.
    ///
    /// Prefer [`Self::create_sepa`] for SEPA Direct Debit. Other methods (card,
    /// PayPal, …) keep this advanced path.
    pub async fn create(
        &self,
        customer_id: &CustomerId,
        body: &types::MandateRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<MandateResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        client_with_key(self.client, key)
            .create_mandate(&customer, body)
            .into_mollie_result()
            .await
    }

    /// Fetches a mandate.
    pub async fn get(
        &self,
        customer_id: &CustomerId,
        mandate_id: &MandateId,
    ) -> MollieResponse<MandateResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let mandate = types::MandateToken(mandate_id.as_str().to_string());
        self.client
            .get_mandate(&customer, &mandate)
            .into_mollie_result()
            .await
    }

    /// Revokes a mandate.
    ///
    /// Mollie may return an empty body; the facade maps that to [`EmptyResponse`].
    pub async fn revoke(
        &self,
        customer_id: &CustomerId,
        mandate_id: &MandateId,
        body: Option<&types::RevokeMandateBody>,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EmptyResponse> {
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let mandate = types::MandateToken(mandate_id.as_str().to_string());
        let default_body = types::RevokeMandateBody::default();
        let body = body.unwrap_or(&default_body);
        client_with_key(self.client, key)
            .revoke_mandate(&customer, &mandate, body)
            .into_mollie_result()
            .await
            .map(|envelope| envelope.map(|_| EmptyResponse::new()))
    }

    /// Lists one page of mandates for a customer.
    pub async fn list_page(
        &self,
        customer_id: &CustomerId,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListMandateResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let customer = types::CustomerToken(customer_id.as_str().to_string());
        let from_token = from.map(|c| types::MandateToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListMandatesResponse> = self
            .client
            .list_mandates(&customer, from_token.as_ref(), limit_nz, None, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_mandates(envelope))
    }

    /// Lists all mandates for a customer within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListMandateResponse>> {
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

    /// Streams mandate pages for a customer within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> MandatePageFut, types::ListMandateResponse>
    {
        let client = self.client.clone();
        let customer = customer_id.clone();
        stream_pages(guard, move |cursor| -> MandatePageFut {
            let client = client.clone();
            let customer = customer.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                MandatesApi { client: &client }
                    .list_page(&customer, cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams mandate items for a customer within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        customer_id: &CustomerId,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> MandatePageFut, types::ListMandateResponse>
    {
        let client = self.client.clone();
        let customer = customer_id.clone();
        stream_items(guard, move |cursor| -> MandatePageFut {
            let client = client.clone();
            let customer = customer.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                MandatesApi { client: &client }
                    .list_page(&customer, cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list_mandates(
    envelope: ResponseEnvelope<ListMandatesResponse>,
) -> Page<types::ListMandateResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.mandates, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ListCount, ListLinks, ListMandatesResponseEmbedded, Url, UrlNullable};
    use reqwest::StatusCode;

    #[test]
    fn maps_mandate_list_next_cursor() {
        let response = ListMandatesResponse {
            count: ListCount(0),
            embedded: ListMandatesResponseEmbedded { mandates: vec![] },
            links: ListLinks {
                documentation: Url {
                    href: "https://docs.mollie.com".into(),
                    type_: "text/html".into(),
                },
                next: UrlNullable(Some(types::UrlNullableInner {
                    href: Some(
                        "https://api.mollie.com/v2/customers/cst_x/mandates?from=mdt_next".into(),
                    ),
                    type_: Some("application/hal+json".into()),
                })),
                previous: UrlNullable(None),
                self_: Url {
                    href: "https://api.mollie.com/v2/customers/cst_x/mandates".into(),
                    type_: "application/hal+json".into(),
                },
            },
        };
        let env = ResponseEnvelope::from_parts(response, StatusCode::OK, Default::default());
        let page = page_from_list_mandates(env);
        assert_eq!(page.next.as_ref().map(PageCursor::as_str), Some("mdt_next"));
    }
}
