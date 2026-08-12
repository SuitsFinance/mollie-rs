//! Shared helpers for domain facades.

use std::future::Future;
use std::num::NonZeroU64;

use crate::pagination::{
    AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard, MAX_PAGE_LIMIT,
};
use crate::types::ListLinks;
use crate::{MollieError, MollieResult};

/// Validates Mollie list `limit` (1..=250).
pub(crate) fn validate_page_limit(limit: Option<u32>) -> MollieResult<Option<NonZeroU64>> {
    match limit {
        None => Ok(None),
        Some(n) if n == 0 || n > MAX_PAGE_LIMIT => Err(MollieError::invalid_request(format!(
            "list limit must be 1..={MAX_PAGE_LIMIT}"
        ))),
        Some(n) => Ok(NonZeroU64::new(u64::from(n))),
    }
}

pub(crate) fn next_cursor_from_links(links: &ListLinks) -> Option<PageCursor> {
    links
        .next
        .0
        .as_ref()
        .and_then(|inner| inner.href.as_ref())
        .and_then(|href| PageCursor::from_list_link(href))
}

/// Scopes a sticky idempotency key onto a short-lived client clone.
pub(crate) fn client_with_key(
    client: &crate::MollieClient,
    key: Option<crate::IdempotencyKey>,
) -> crate::MollieClient {
    match key {
        Some(k) => client.clone().with_idempotency(k),
        None => client.clone(),
    }
}

/// Builds a guarded page stream from a list_page-style fetch closure.
pub(crate) fn stream_pages<F, Fut, T>(guard: PaginationGuard, fetch: F) -> AsyncPaginator<F, T>
where
    F: FnMut(Option<PageCursor>) -> Fut,
    Fut: Future<Output = MollieResult<Page<T>>>,
{
    AsyncPaginator::new(fetch, guard)
}

/// Builds a guarded item stream from a list_page-style fetch closure.
pub(crate) fn stream_items<F, Fut, T>(guard: PaginationGuard, fetch: F) -> ItemStream<F, T>
where
    F: FnMut(Option<PageCursor>) -> Fut,
    Fut: Future<Output = MollieResult<Page<T>>>,
{
    ItemStream::new(AsyncPaginator::new(fetch, guard))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_and_oversized_limits() {
        assert!(validate_page_limit(Some(0)).is_err());
        assert!(validate_page_limit(Some(251)).is_err());
        assert!(validate_page_limit(Some(50)).is_ok());
        assert!(validate_page_limit(None).is_ok());
    }
}
