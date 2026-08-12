//! Connect balance-transfer facade (merchant-to-merchant money movement).
//!
//! `create_connect_balance_transfer` is an **IdempotentWrite**. Prefer
//! [`CreateConnectBalanceTransferRequired`] so amounts and organization parties
//! are validated before send. Sticky idempotency is required for safe retries.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{self, EntityBalanceTransferResponse, ListConnectBalanceTransfersResponse};
use crate::{
    CreateConnectBalanceTransferRequired, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieError, MollieResponse, MollieResult, ResponseEnvelope,
};

type PageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<EntityBalanceTransferResponse>>> + Send>>;

/// Connect balance-transfer operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct ConnectBalanceTransfersApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the Connect balance-transfers domain facade.
    pub fn connect_balance_transfers(&self) -> ConnectBalanceTransfersApi<'_> {
        ConnectBalanceTransfersApi { client: self }
    }
}

impl ConnectBalanceTransfersApi<'_> {
    /// Creates a Connect balance transfer from a validated builder.
    pub async fn create(
        &self,
        required: CreateConnectBalanceTransferRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityBalanceTransferResponse> {
        let body = required.into_request()?;
        self.create_raw(&body, key).await
    }

    /// Creates a Connect balance transfer from a generated body (advanced).
    pub async fn create_raw(
        &self,
        body: &types::EntityBalanceTransfer,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityBalanceTransferResponse> {
        client_with_key(self.client, key)
            .create_connect_balance_transfer(body)
            .into_mollie_result()
            .await
    }

    /// Fetches a Connect balance transfer by id (`cbtr_…`).
    pub async fn get(
        &self,
        balance_transfer_id: &str,
    ) -> MollieResponse<EntityBalanceTransferResponse> {
        let id = balance_transfer_id.trim();
        if id.is_empty() {
            return Err(MollieError::invalid_request(
                "connect balance transfer id must not be empty",
            ));
        }
        let token = types::ConnectBalanceTransferToken(id.to_string());
        self.client
            .get_connect_balance_transfer(&token)
            .into_mollie_result()
            .await
    }

    /// Lists one page of Connect balance transfers.
    pub async fn list_page(
        &self,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<EntityBalanceTransferResponse>> {
        let limit_nz = validate_page_limit(limit)?;
        let from_s = from.map(PageCursor::as_str);
        let envelope: ResponseEnvelope<ListConnectBalanceTransfersResponse> = self
            .client
            .list_connect_balance_transfers(from_s, limit_nz, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list(envelope))
    }

    /// Lists Connect balance transfers within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<EntityBalanceTransferResponse>> {
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

    /// Streams pages within [`PaginationGuard`] budgets (never unbounded).
    pub fn stream_pages(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> PageFut, EntityBalanceTransferResponse>
    {
        let client = self.client.clone();
        stream_pages(guard, move |cursor| -> PageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                ConnectBalanceTransfersApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams items within [`PaginationGuard`] budgets (never unbounded).
    pub fn stream_items(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> PageFut, EntityBalanceTransferResponse> {
        let client = self.client.clone();
        stream_items(guard, move |cursor| -> PageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                ConnectBalanceTransfersApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }
}

fn page_from_list(
    envelope: ResponseEnvelope<ListConnectBalanceTransfersResponse>,
) -> Page<EntityBalanceTransferResponse> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.connect_balance_transfers, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operation_safety_profile, Money, RetryClass};

    #[test]
    fn create_connect_is_idempotent_write_validated() {
        let p = operation_safety_profile("create_connect_balance_transfer").unwrap();
        assert_eq!(p.retry_class, RetryClass::IdempotentWrite);
        assert!(matches!(p.access, crate::RouteAccess::ValidatedFacade));
    }

    #[test]
    fn builder_rejects_same_org_and_empty_description() {
        let amount = Money::new("EUR", "1.00").unwrap();
        assert!(CreateConnectBalanceTransferRequired::new(
            amount.clone(),
            "fee",
            "org_a",
            "src",
            "org_a",
            "dst",
        )
        .is_err());
        assert!(CreateConnectBalanceTransferRequired::new(
            amount,
            "   ",
            "org_source",
            "src",
            "org_dest",
            "dst",
        )
        .is_err());
    }

    #[test]
    fn builder_serializes_write_fields_only() {
        let body = CreateConnectBalanceTransferRequired::new(
            Money::new("EUR", "12.50").unwrap(),
            "Invoice fee",
            "org_source",
            "Platform fee",
            "org_dest",
            "Merchant payout share",
        )
        .unwrap()
        .with_category(types::BalanceTransferCategory::OtherFee)
        .into_request()
        .unwrap();
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["amount"]["value"], "12.50");
        assert_eq!(value["description"], "Invoice fee");
        assert_eq!(value["source"]["id"], "org_source");
        assert_eq!(value["destination"]["id"], "org_dest");
        assert_eq!(value["category"], "other_fee");
    }
}
