//! Point-of-sale terminals and pairing-code facade.
//!
//! Pairing request/revoke are **NonRetryableWrite** — never invent sticky keys
//! and never auto-retry pairing churn.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::domain::common::{
    client_with_key, next_cursor_from_links, stream_items, stream_pages, validate_page_limit,
};
use crate::pagination::{AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard};
use crate::types::{
    self, EntityPairingCode, EntityTerminal, ListTerminalsResponse,
    TerminalsListPairingCodesResponse, TerminalsRequestPairingCodeBody,
};
use crate::{
    IdempotencyKey, IntoMollieFuture, MollieClient, MollieError, MollieResponse, MollieResult,
    ResponseEnvelope,
};

type TerminalPageFut =
    Pin<Box<dyn Future<Output = MollieResult<Page<types::ListEntityTerminal>>> + Send>>;

/// Terminal and pairing-code operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct TerminalsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the terminals / pairing facade.
    pub fn terminals(&self) -> TerminalsApi<'_> {
        TerminalsApi { client: self }
    }
}

impl TerminalsApi<'_> {
    /// Lists one page of terminals.
    pub async fn list_page(
        &self,
        from: Option<&PageCursor>,
        limit: Option<u32>,
    ) -> MollieResult<Page<types::ListEntityTerminal>> {
        let limit_nz = validate_page_limit(limit)?;
        let from_token = from.map(|c| types::TerminalToken(c.as_str().to_string()));
        let envelope: ResponseEnvelope<ListTerminalsResponse> = self
            .client
            .list_terminals(from_token.as_ref(), limit_nz, None)
            .into_mollie_result()
            .await?;
        Ok(page_from_list_terminals(envelope))
    }

    /// Lists terminals within [`PaginationGuard`] budgets.
    pub async fn list_all(
        &self,
        limit: Option<u32>,
        mut guard: PaginationGuard,
    ) -> MollieResult<Vec<types::ListEntityTerminal>> {
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

    /// Streams terminal pages within [`PaginationGuard`] budgets.
    pub fn stream_pages(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> AsyncPaginator<impl FnMut(Option<PageCursor>) -> TerminalPageFut, types::ListEntityTerminal>
    {
        let client = self.client.clone();
        stream_pages(guard, move |cursor| -> TerminalPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                TerminalsApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Streams terminal items within [`PaginationGuard`] budgets.
    pub fn stream_items(
        &self,
        limit: Option<u32>,
        guard: PaginationGuard,
    ) -> ItemStream<impl FnMut(Option<PageCursor>) -> TerminalPageFut, types::ListEntityTerminal>
    {
        let client = self.client.clone();
        stream_items(guard, move |cursor| -> TerminalPageFut {
            let client = client.clone();
            Box::pin(async move {
                let _ = validate_page_limit(limit)?;
                TerminalsApi { client: &client }
                    .list_page(cursor.as_ref(), limit)
                    .await
            })
        })
    }

    /// Fetches a terminal by id (`term_…`).
    pub async fn get(&self, terminal_id: &str) -> MollieResponse<EntityTerminal> {
        let token = types::TerminalToken(terminal_id.to_string());
        self.client.get_terminal(&token).into_mollie_result().await
    }

    /// Requests a new terminal pairing code for a profile.
    pub async fn request_pairing_code(
        &self,
        profile_id: &str,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityPairingCode> {
        if profile_id.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "terminals pairing profileId must not be empty",
            ));
        }
        let body = TerminalsRequestPairingCodeBody {
            profile_id: profile_id.trim().to_string(),
        };
        client_with_key(self.client, key)
            .terminals_request_pairing_code(None, &body)
            .into_mollie_result()
            .await
    }

    /// Lists pairing codes (one page).
    pub async fn list_pairing_codes(
        &self,
        from: Option<&str>,
        limit: Option<u32>,
        profile_id: Option<&str>,
    ) -> MollieResponse<TerminalsListPairingCodesResponse> {
        let limit_nz = validate_page_limit(limit)?;
        self.client
            .terminals_list_pairing_codes(from, limit_nz, profile_id, None)
            .into_mollie_result()
            .await
    }

    /// Fetches a pairing code by id.
    pub async fn get_pairing_code(
        &self,
        pairing_code_id: &str,
    ) -> MollieResponse<EntityPairingCode> {
        let token = types::TerminalPairingCodeToken(pairing_code_id.to_string());
        self.client
            .terminals_get_pairing_code(&token, None)
            .into_mollie_result()
            .await
    }

    /// Revokes a pairing code (non-retryable write).
    pub async fn revoke_pairing_code(
        &self,
        pairing_code_id: &str,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<EntityPairingCode> {
        if pairing_code_id.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "pairing code id must not be empty",
            ));
        }
        let token = types::TerminalPairingCodeToken(pairing_code_id.trim().to_string());
        client_with_key(self.client, key)
            .terminals_revoke_pairing_code(&token)
            .into_mollie_result()
            .await
    }
}

fn page_from_list_terminals(
    envelope: ResponseEnvelope<ListTerminalsResponse>,
) -> Page<types::ListEntityTerminal> {
    let metadata = envelope.metadata();
    let body = envelope.into_inner();
    let next = next_cursor_from_links(&body.links);
    Page::new(body.embedded.terminals, next, metadata)
}

#[cfg(test)]
mod tests {
    use crate::{operation_safety_profile, RetryClass};

    #[test]
    fn pairing_request_is_non_retryable() {
        let p = operation_safety_profile("terminals_request_pairing_code").unwrap();
        assert_eq!(p.retry_class, RetryClass::NonRetryableWrite);
    }

    #[test]
    fn pairing_revoke_is_profiled() {
        assert!(operation_safety_profile("terminals_revoke_pairing_code").is_some());
    }
}
