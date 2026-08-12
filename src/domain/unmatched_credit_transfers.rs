//! Unmatched credit transfer (UCT) match / return facade.
//!
//! Match and return are financial settlement actions — they go through the
//! transport kernel with sticky idempotency when supplied.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{
    self, ListUnmatchedCreditTransfersResponse, UnmatchedCreditTransferActionResponse,
};
use crate::{
    IdempotencyKey, IntoMollieFuture, MollieClient, MollieResponse, MollieResult, ResponseEnvelope,
};

type UctPageFut = Pin<
    Box<dyn Future<Output = MollieResult<Page<types::ListEntityUnmatchedCreditTransfer>>> + Send>,
>;

/// Unmatched credit transfer operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct UnmatchedCreditTransfersApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the unmatched credit transfers facade.
    pub fn unmatched_credit_transfers(&self) -> UnmatchedCreditTransfersApi<'_> {
        UnmatchedCreditTransfersApi { client: self }
    }
}

impl UnmatchedCreditTransfersApi<'_> {
    /// Lists one page of unmatched credit transfers.
    pub async fn list_page(
        &self,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListEntityUnmatchedCreditTransfer>> {
        let limit_nz = validate_page_limit(limit)?;
        let from_s = from.map(PageCursor::as_str);
        let envelope: ResponseEnvelope<ListUnmatchedCreditTransfersResponse> = self
            .client
            .list_unmatched_credit_transfers(from_s, limit_nz)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_uct(envelope))
    }

    /// Lists UCTs within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListEntityUnmatchedCreditTransfer>> {
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

    /// Streams UCT pages within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<
        impl FnMut(Option<PageCursor>) -> UctPageFut,
        types::ListEntityUnmatchedCreditTransfer,
    > {
        let client = self.client.clone();
        stream_pages(guard, move |cursor| -> UctPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                UnmatchedCreditTransfersApi { client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams UCT items within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<
        impl FnMut(Option<PageCursor>) -> UctPageFut,
        types::ListEntityUnmatchedCreditTransfer,
    > {
        let client = self.client.clone();
        stream_items(guard, move |cursor| -> UctPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                UnmatchedCreditTransfersApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Fetches a single unmatched credit transfer.
    pub async fn get(&self, id: &str) -> MollieResponse<types::EntityUnmatchedCreditTransfer> {
        let token = types::UnmatchedCreditTransferToken(id.to_string());
        self.client
            .get_unmatched_credit_transfer(&token)
            .into_mollie_result()
            .await
    }

    /// Matches an unmatched credit transfer to payment(s).
    pub async fn match_transfer(
        &self,
        id: &str,
        body: &types::UnmatchedCreditTransferMatchRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<UnmatchedCreditTransferActionResponse> {
        let token = types::UnmatchedCreditTransferToken(id.to_string());
        client_with_key(self.client, key)
            .match_unmatched_credit_transfer(&token, body)
            .into_mollie_result()
            .await
    }

    /// Returns funds for an unmatched credit transfer to the sender.
    pub async fn return_transfer(
        &self,
        id: &str,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<UnmatchedCreditTransferActionResponse> {
        let token = types::UnmatchedCreditTransferToken(id.to_string());
        client_with_key(self.client, key)
            .return_unmatched_credit_transfer(&token)
            .into_mollie_result()
            .await
    }
}

fn page_from_list_uct(
    envelope: ResponseEnvelope<ListUnmatchedCreditTransfersResponse>,
) -> Page<types::ListEntityUnmatchedCreditTransfer> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.unmatched_credit_transfers, next, metadata)
}

#[cfg(test)]
mod tests {
    use crate::operation_safety_profile;

    #[test]
    fn uct_match_and_return_are_profiled() {
        assert!(operation_safety_profile("match_unmatched_credit_transfer").is_some());
        assert!(operation_safety_profile("return_unmatched_credit_transfer").is_some());
    }
}
