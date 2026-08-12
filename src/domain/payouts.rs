//! Payout-domain facade (balance → bank settlement).
//!
//! Writes inherit the transport safety kernel: sticky idempotency for retries
//! (`create_payout` / `cancel_payout` are IdempotentWrite). Prefer
//! [`CreatePayoutRequired`] so money and balance ids are validated before send.
#![warn(missing_docs)]

use crate::domain::common::{client_with_key, next_cursor_from_links, validate_page_limit};
use crate::pagination::{Page, PageCursor, PaginationGuard};
use crate::types::{self, EntityPayoutResponse, ListPayoutsResponse};
use crate::{
    BalanceId, CreatePayoutRequired, IdempotencyKey, IntoMollieFuture, MollieClient,
    MollieResponse, MollieResult, ResponseEnvelope,
};

/// Payout operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct PayoutsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the payouts domain facade.
    pub fn payouts(&self) -> PayoutsApi<'_> {
        PayoutsApi { client: self }
    }
}

impl PayoutsApi<'_> {
    /// Requests a payout from a validated builder.
    pub async fn create(
        &self,
        required: CreatePayoutRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityPayoutResponse> {
        let body = required.into_request()?;
        self.create_raw(&body, key).await
    }

    /// Requests a payout from a generated body (advanced).
    pub async fn create_raw(
        &self,
        body: &types::PayoutRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityPayoutResponse> {
        client_with_key(self.client, key)
            .create_payout(body)
            .into_mollie_result()
            .await
    }

    /// Fetches a payout by id (`payout_…`).
    pub async fn get(&self, payout_id: &str) -> MollieResponse<EntityPayoutResponse> {
        self.client.get_payout(payout_id).into_mollie_result().await
    }

    /// Cancels a payout still in `requested` status.
    pub async fn cancel(
        &self,
        payout_id: &str,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityPayoutResponse> {
        client_with_key(self.client, key)
            .cancel_payout(payout_id)
            .into_mollie_result()
            .await
    }

    /// Lists one page of payouts.
    pub async fn list_page(
        &self,
        balance_id: Option<&BalanceId>,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListEntityPayout>> {
        let limit_nz = validate_page_limit(limit)?;
        let balance = balance_id
            .map(|id| types::ListPayoutsBalanceId::try_from(id.as_str()))
            .transpose()
            .map_err(|e| crate::MollieError::invalid_request(e.to_string()))?;
        let from_s = from.map(PageCursor::as_str);
        let envelope: ResponseEnvelope<ListPayoutsResponse> = self
            .client
            .list_payouts(balance.as_ref(), from_s, limit_nz, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_payouts(envelope))
    }

    /// Lists payouts within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        balance_id: Option<&BalanceId>,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListEntityPayout>> {
        let mut items = Vec::new();
        let mut cursor: Option<PageCursor> = None;
        loop {
            let page = self.list_page(balance_id, cursor.as_ref(), limit).await?;
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
}

fn page_from_list_payouts(
    envelope: ResponseEnvelope<ListPayoutsResponse>,
) -> Page<types::ListEntityPayout> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.payouts, next, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operation_safety_profile, Money, RetryClass};

    #[test]
    fn create_payout_is_idempotent_write_in_profile() {
        let p = operation_safety_profile("create_payout").unwrap();
        assert_eq!(p.retry_class, RetryClass::IdempotentWrite);
        assert!(p.supports_idempotency);
    }

    #[test]
    fn create_builder_serializes_write_fields_only() {
        let body = CreatePayoutRequired::with_amount_for_balance_str(
            "bal_gVMhHKqSSRYJyPsuoPNFH",
            Money::new("EUR", "10.00").unwrap(),
        )
        .unwrap()
        .with_description("Weekly settlement")
        .unwrap()
        .into_request()
        .unwrap();
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["balanceId"], "bal_gVMhHKqSSRYJyPsuoPNFH");
        assert_eq!(value["amount"]["value"], "10.00");
        assert_eq!(value["description"], "Weekly settlement");
        assert!(value.get("id").is_none());
        assert!(value.get("status").is_none());
        assert!(value.get("createdAt").is_none());
    }
}
