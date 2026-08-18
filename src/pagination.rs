//! Pagination primitives and async page iterators for Mollie list endpoints.
//!
//! Mollie uses opaque `from` identifiers with a `limit` (max 250), not classic
//! offset pages. Callers pass a fetch closure; this module never assumes integer
//! cursors.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;

use crate::{MollieError, MollieResult, ResponseMetadata};

/// Default page size when the caller does not specify one.
pub const DEFAULT_PAGE_LIMIT: u32 = 50;
/// Mollie maximum page size.
pub const MAX_PAGE_LIMIT: u32 = 250;

/// Cursor token for the next page (`from` query parameter).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PageCursor(String);

impl PageCursor {
    /// Creates a cursor from a non-empty token.
    pub fn new(token: impl Into<String>) -> Option<Self> {
        let token = token.into();
        if token.is_empty() {
            None
        } else {
            Some(Self(token))
        }
    }

    /// Returns the raw token for the `from` query parameter.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extracts a `from` query parameter from a Mollie list `next` href.
    ///
    /// **Host policy (INV-PAGE-01):** the link must share origin with a known
    /// Mollie API host or loopback mock. Off-origin links are rejected so a
    /// compromised/malicious `next` cannot steer pagination. Prefer
    /// [`Self::from_list_link_for_base`] when the client base URL is known.
    pub fn from_list_link(href: &str) -> Option<Self> {
        Self::from_list_link_for_base(href, None)
    }

    /// Like [`Self::from_list_link`], but also accepts links matching `base_url`'s origin.
    pub fn from_list_link_for_base(href: &str, base_url: Option<&str>) -> Option<Self> {
        let url = reqwest::Url::parse(href).ok()?;
        if !list_link_origin_allowed(&url, base_url) {
            let host = url.host_str().unwrap_or("invalid-host");
            crate::contract_drift::emit_off_origin_pagination_link(host);
            return None;
        }
        let from = url
            .query_pairs()
            .find(|(k, _)| k == "from")
            .map(|(_, v)| v.into_owned())?;
        Self::new(from)
    }
}

/// Returns true when a provider `next` URL is safe to read a cursor from.
fn list_link_origin_allowed(url: &reqwest::Url, base_url: Option<&str>) -> bool {
    if is_official_mollie_api_origin(url) {
        return true;
    }
    if let Some(base) = base_url {
        if let Ok(base_parsed) = reqwest::Url::parse(base) {
            // When the caller supplies a base URL (mock server, custom pin),
            // require same origin — do not accept arbitrary loopback ports.
            return same_http_origin(&base_parsed, url);
        }
        return false;
    }
    // No base: allow loopback for unit tests that only pass absolute mock hrefs.
    is_loopback_http_origin(url)
}

fn is_official_mollie_api_origin(url: &reqwest::Url) -> bool {
    matches!(
        (url.scheme(), url.host_str()),
        ("https", Some(host))
            if host.eq_ignore_ascii_case("api.mollie.com")
                || host.eq_ignore_ascii_case("api.mollie.nl")
    )
}

fn is_loopback_http_origin(url: &reqwest::Url) -> bool {
    matches!(
        (url.scheme(), url.host_str()),
        ("http", Some(host))
            if host.eq_ignore_ascii_case("localhost")
                || host == "127.0.0.1"
                || host == "[::1]"
                || host.eq_ignore_ascii_case("::1")
    )
}

fn same_http_origin(a: &reqwest::Url, b: &reqwest::Url) -> bool {
    a.scheme() == b.scheme()
        && a.host() == b.host()
        && a.port_or_known_default() == b.port_or_known_default()
}

/// One page of list results plus transport metadata.
#[derive(Clone, Debug)]
pub struct Page<T> {
    /// Items on this page.
    pub items: Vec<T>,
    /// Cursor for the next page when more results exist.
    pub next: Option<PageCursor>,
    /// Response metadata for this page request.
    pub metadata: ResponseMetadata,
}

impl<T> Page<T> {
    /// Creates a page.
    pub fn new(items: Vec<T>, next: Option<PageCursor>, metadata: ResponseMetadata) -> Self {
        Self {
            items,
            next,
            metadata,
        }
    }

    /// Returns true when another page may be fetched.
    pub fn has_more(&self) -> bool {
        self.next.is_some()
    }

    /// Number of items on this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether this page is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Guards against runaway pagination loops and cursor cycles.
///
/// Cycle detection uses a bounded set of **seen next-cursors**. Memory is
/// limited by `max_pages` (each cursor string stored at most once).
#[derive(Clone, Debug)]
pub struct PaginationGuard {
    /// Maximum pages to fetch in one iteration (inclusive).
    pub max_pages: u32,
    /// Maximum items across all pages.
    pub max_items: u32,
    pages_seen: u32,
    items_seen: u32,
    /// All previously observed `next` cursors (for arbitrary cycle detection).
    seen_cursors: std::collections::HashSet<String>,
}

impl PaginationGuard {
    /// Creates a guard with page/item budgets.
    pub fn new(max_pages: u32, max_items: u32) -> Self {
        Self {
            max_pages,
            max_items,
            pages_seen: 0,
            items_seen: 0,
            seen_cursors: std::collections::HashSet::with_capacity(max_pages as usize),
        }
    }

    /// Default production-safe budgets.
    pub fn default_safe() -> Self {
        Self::new(100, 10_000)
    }

    /// Records a fetched page; returns an error if budgets or cursor loops trip.
    pub fn observe_page<T>(&mut self, page: &Page<T>) -> Result<(), MollieError> {
        self.pages_seen = self.pages_seen.saturating_add(1);
        if self.pages_seen > self.max_pages {
            return Err(MollieError::invalid_request(format!(
                "pagination exceeded max_pages ({})",
                self.max_pages
            )));
        }
        let page_len = page.len() as u32;
        self.items_seen = self.items_seen.saturating_add(page_len);
        if self.items_seen > self.max_items {
            return Err(MollieError::invalid_request(format!(
                "pagination exceeded max_items ({})",
                self.max_items
            )));
        }
        if let Some(ref next) = page.next {
            let next_s = next.as_str();
            if !self.seen_cursors.insert(next_s.to_string()) {
                return Err(MollieError::invalid_request(
                    "pagination detected cursor cycle (next cursor already observed)",
                ));
            }
            // Bound set size to max_pages + 1 to cap memory even if max_pages is large.
            if self.seen_cursors.len() > (self.max_pages as usize).saturating_add(1) {
                return Err(MollieError::invalid_request(
                    "pagination cursor set exceeded budget",
                ));
            }
        }
        Ok(())
    }
}

/// Async page-by-page iterator driven by a user-supplied fetch function.
///
/// The fetch function receives `None` for the first page and `Some(cursor)` for
/// subsequent pages. Each call performs one network request (documented so
/// iteration cost is never surprising).
pub struct AsyncPaginator<F, T> {
    fetch: F,
    guard: PaginationGuard,
    next_cursor: Option<Option<PageCursor>>,
    done: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<F, Fut, T> AsyncPaginator<F, T>
where
    F: FnMut(Option<PageCursor>) -> Fut,
    Fut: Future<Output = MollieResult<Page<T>>>,
{
    /// Creates a paginator with the given fetch closure and budgets.
    pub fn new(fetch: F, guard: PaginationGuard) -> Self {
        Self {
            fetch,
            guard,
            next_cursor: Some(None),
            done: false,
            _marker: std::marker::PhantomData,
        }
    }

    /// Convenience constructor with [`PaginationGuard::default_safe`].
    pub fn default_safe(fetch: F) -> Self {
        Self::new(fetch, PaginationGuard::default_safe())
    }

    /// Fetches the next page, or `Ok(None)` when exhausted.
    pub async fn next_page(&mut self) -> MollieResult<Option<Page<T>>> {
        if self.done {
            return Ok(None);
        }
        let cursor = match self.next_cursor.take() {
            Some(c) => c,
            None => {
                self.done = true;
                return Ok(None);
            }
        };
        let page = (self.fetch)(cursor).await?;
        self.guard.observe_page(&page)?;
        if let Some(next) = page.next.clone() {
            self.next_cursor = Some(Some(next));
        } else {
            self.next_cursor = None;
            self.done = true;
        }
        Ok(Some(page))
    }
}

/// Streams items across pages using an underlying [`AsyncPaginator`].
pub struct ItemStream<F, T> {
    pages: AsyncPaginator<F, T>,
    buffer: std::vec::IntoIter<T>,
}

impl<F, Fut, T> ItemStream<F, T>
where
    F: FnMut(Option<PageCursor>) -> Fut,
    Fut: Future<Output = MollieResult<Page<T>>>,
{
    /// Wraps a paginator as an item stream.
    pub fn new(pages: AsyncPaginator<F, T>) -> Self {
        Self {
            pages,
            buffer: Vec::new().into_iter(),
        }
    }

    /// Yields the next item, fetching pages as needed.
    pub async fn next_item(&mut self) -> MollieResult<Option<T>> {
        loop {
            if let Some(item) = self.buffer.next() {
                return Ok(Some(item));
            }
            match self.pages.next_page().await? {
                Some(page) => {
                    self.buffer = page.items.into_iter();
                }
                None => return Ok(None),
            }
        }
    }
}

/// Helper for building a pinned box future when storing async closures is hard.
pub type BoxedPageFuture<T> = Pin<Box<dyn Future<Output = MollieResult<Page<T>>> + Send>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_detects_repeated_cursor() {
        let mut guard = PaginationGuard::new(10, 1000);
        let meta = ResponseMetadata::default();
        let page = Page::new(vec![1, 2], PageCursor::new("cur_1"), meta.clone());
        guard.observe_page(&page).unwrap();
        let again = Page::new(vec![3], PageCursor::new("cur_1"), meta);
        assert!(guard.observe_page(&again).is_err());
    }

    #[test]
    fn guard_detects_two_page_cycle() {
        let mut guard = PaginationGuard::new(10, 1000);
        let meta = ResponseMetadata::default();
        guard
            .observe_page(&Page::new(vec![1], PageCursor::new("a"), meta.clone()))
            .unwrap();
        guard
            .observe_page(&Page::new(vec![2], PageCursor::new("b"), meta.clone()))
            .unwrap();
        // next cursor returns to "a"
        assert!(guard
            .observe_page(&Page::new(vec![3], PageCursor::new("a"), meta))
            .is_err());
    }

    #[test]
    fn guard_detects_three_page_cycle() {
        let mut guard = PaginationGuard::new(10, 1000);
        let meta = ResponseMetadata::default();
        guard
            .observe_page(&Page::new(vec![1], PageCursor::new("a"), meta.clone()))
            .unwrap();
        guard
            .observe_page(&Page::new(vec![2], PageCursor::new("b"), meta.clone()))
            .unwrap();
        guard
            .observe_page(&Page::new(vec![3], PageCursor::new("c"), meta.clone()))
            .unwrap();
        // cycle back to b (not only last/earlier)
        assert!(guard
            .observe_page(&Page::new(vec![4], PageCursor::new("b"), meta))
            .is_err());
    }

    #[test]
    fn empty_cursor_rejected() {
        assert!(PageCursor::new("").is_none());
    }

    #[test]
    fn from_list_link_parses_from_query() {
        let c =
            PageCursor::from_list_link("https://api.mollie.com/v2/payments?from=tr_xxx&limit=50")
                .unwrap();
        assert_eq!(c.as_str(), "tr_xxx");
    }

    #[test]
    fn from_list_link_rejects_off_origin_host() {
        assert!(PageCursor::from_list_link(
            "https://evil.example/v2/payments?from=tr_stolen&limit=50"
        )
        .is_none());
    }

    #[test]
    fn from_list_link_allows_matching_client_base() {
        let href = "http://127.0.0.1:9/v2/payments?from=tr_mock&limit=10";
        assert!(PageCursor::from_list_link_for_base(href, Some("http://127.0.0.1:9/v2")).is_some());
        assert!(
            PageCursor::from_list_link_for_base(href, Some("http://127.0.0.1:10/v2")).is_none()
        );
    }

    #[tokio::test]
    async fn async_paginator_walks_two_pages() {
        let mut calls = 0u32;
        let mut pages = AsyncPaginator::new(
            |cursor: Option<PageCursor>| {
                calls += 1;
                let page = match cursor {
                    None => Page::new(
                        vec!["a".to_string()],
                        PageCursor::new("c1"),
                        ResponseMetadata::default(),
                    ),
                    Some(c) if c.as_str() == "c1" => {
                        Page::new(vec!["b".to_string()], None, ResponseMetadata::default())
                    }
                    _ => panic!("unexpected cursor"),
                };
                async move { Ok(page) }
            },
            PaginationGuard::new(10, 100),
        );
        let p1 = pages.next_page().await.unwrap().unwrap();
        assert_eq!(p1.items, vec!["a"]);
        let p2 = pages.next_page().await.unwrap().unwrap();
        assert_eq!(p2.items, vec!["b"]);
        assert!(pages.next_page().await.unwrap().is_none());
        assert_eq!(calls, 2);
    }

    #[tokio::test]
    async fn item_stream_flattens_pages() {
        let pages = AsyncPaginator::new(
            |cursor: Option<PageCursor>| {
                let page = match cursor {
                    None => Page::new(
                        vec![1, 2],
                        PageCursor::new("n"),
                        ResponseMetadata::default(),
                    ),
                    Some(_) => Page::new(vec![3], None, ResponseMetadata::default()),
                };
                async move { Ok(page) }
            },
            PaginationGuard::new(10, 100),
        );
        let mut items = ItemStream::new(pages);
        assert_eq!(items.next_item().await.unwrap(), Some(1));
        assert_eq!(items.next_item().await.unwrap(), Some(2));
        assert_eq!(items.next_item().await.unwrap(), Some(3));
        assert_eq!(items.next_item().await.unwrap(), None);
    }
}
