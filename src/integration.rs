//! Application integration boundaries for webhook processing.
//!
//! The SDK verifies signatures and models Mollie resources. Durable storage,
//! queues, and ledger reconciliation remain **application-owned**. These traits
//! document the seams without shipping a database.
//!
//! ## Recommended receive path
//!
//! 1. Receive **raw** request body + signature header  
//! 2. Verify signature ([`crate::WebhookVerifier`]) **before** decoding JSON  
//! 3. Derive a stable event identity (Next-gen event id, or classic resource id)  
//! 4. **Claim** the event ([`WebhookReplayStore::claim_event`])  
//! 5. Enqueue ([`WebhookDispatcher`]) and acknowledge HTTP 2xx quickly  
//! 6. Worker: refetch ([`PaymentStateRefetcher`]) → reconcile → mark done  
//!
//! HMAC verification does **not** prevent replay; durable claim/dedupe does.

use std::future::Future;
use std::pin::Pin;

use crate::error::MollieResult;
use crate::ids::PaymentId;

/// Async trait object helper (avoids pulling `async-trait` for one module).
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Outcome of an atomic claim on a webhook event identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimResult {
    /// This caller now owns processing for the event id.
    Claimed,
    /// Another worker already claimed or completed the event (replay / duplicate).
    AlreadyClaimed,
}

/// Durable replay protection for webhook deliveries.
///
/// Prefer this over check-then-mark races: `claim_event` must be **atomic** in
/// the application store (e.g. `INSERT … ON CONFLICT DO NOTHING` returning
/// whether the row was inserted, or a conditional lease).
///
/// The SDK does not include a database; implementors choose Redis, SQL, etc.
pub trait WebhookReplayStore: Send + Sync {
    /// Attempts to claim exclusive processing rights for `event_id`.
    ///
    /// Return [`ClaimResult::Claimed`] only when this invocation is the first
    /// successful claim. Subsequent deliveries of the same id must return
    /// [`ClaimResult::AlreadyClaimed`] without re-running side effects.
    fn claim_event<'a>(&'a self, event_id: &'a str) -> BoxFuture<'a, MollieResult<ClaimResult>>;

    /// Optional: release a claim after a failed attempt so a later delivery can
    /// retry. Default is a no-op (at-least-once apps often keep the claim and
    /// recover via their own job queue).
    fn release_claim<'a>(&'a self, _event_id: &'a str) -> BoxFuture<'a, MollieResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

/// Legacy two-step dedupe API (check then mark).
///
/// Prefer [`WebhookReplayStore::claim_event`] for new integrations: separate
/// `already_processed` + `mark_processed` invites TOCTOU races under concurrent
/// deliveries.
pub trait WebhookEventStore: Send + Sync {
    /// Returns `true` when this event id was already processed successfully.
    fn already_processed<'a>(&'a self, event_id: &'a str) -> BoxFuture<'a, MollieResult<bool>>;

    /// Marks the event as processed after successful reconciliation.
    fn mark_processed<'a>(&'a self, event_id: &'a str) -> BoxFuture<'a, MollieResult<()>>;
}

/// Adapter: implement [`WebhookReplayStore`] in terms of [`WebhookEventStore`].
///
/// **Not race-safe** under concurrent claims unless the underlying store
/// serializes both calls. Prefer a native atomic claim.
pub struct EventStoreReplayAdapter<S> {
    /// Inner check/mark store.
    pub inner: S,
}

impl<S: WebhookEventStore> WebhookReplayStore for EventStoreReplayAdapter<S> {
    fn claim_event<'a>(&'a self, event_id: &'a str) -> BoxFuture<'a, MollieResult<ClaimResult>> {
        Box::pin(async move {
            if self.inner.already_processed(event_id).await? {
                return Ok(ClaimResult::AlreadyClaimed);
            }
            self.inner.mark_processed(event_id).await?;
            Ok(ClaimResult::Claimed)
        })
    }
}

/// Enqueue verified webhook work for asynchronous handling.
///
/// Acknowledge the HTTP request quickly after enqueue; do not block the
/// provider on capture/refund side effects.
pub trait WebhookDispatcher: Send + Sync {
    /// Enqueues already-verified payload bytes (or a derived job id).
    fn enqueue_verified<'a>(
        &'a self,
        event_id: &'a str,
        raw_body: &'a [u8],
    ) -> BoxFuture<'a, MollieResult<()>>;
}

/// Refetch authoritative Mollie state after a webhook signal.
///
/// Never trust webhook body alone for financial state transitions.
pub trait PaymentStateRefetcher: Send + Sync {
    /// Fetches the current payment resource for reconciliation.
    fn refetch_payment<'a>(&'a self, payment_id: &'a PaymentId) -> BoxFuture<'a, MollieResult<()>>;
}

/// Documentation-only module for the receive → verify → claim → enqueue flow.
pub mod webhook_pipeline {
    #![doc = "See parent module docs for the receive → verify → claim → enqueue → refetch flow."]

    /// Pseudocode workflow applications should implement.
    ///
    /// ```ignore
    /// let body = raw_request_bytes;
    /// verifier.verify_header(body, signature)?;
    /// let event_id = extract_event_id(body)?;
    /// match replay_store.claim_event(&event_id).await? {
    ///     ClaimResult::AlreadyClaimed => return Ok(HttpStatus::OK),
    ///     ClaimResult::Claimed => {
    ///         dispatcher.enqueue_verified(&event_id, body).await?;
    ///         return Ok(HttpStatus::OK);
    ///     }
    /// }
    /// // worker:
    /// //   payment = client.get_payment(...).await?;
    /// //   ledger.apply(payment)?;
    /// ```
    pub const WORKFLOW: &str = "verify → claim_event → enqueue → ack → refetch → reconcile";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Mutex;

    struct MemoryStore {
        seen: Mutex<HashSet<String>>,
    }

    impl WebhookReplayStore for MemoryStore {
        fn claim_event<'a>(
            &'a self,
            event_id: &'a str,
        ) -> BoxFuture<'a, MollieResult<ClaimResult>> {
            Box::pin(async move {
                let mut guard = self.seen.lock().expect("lock");
                if guard.insert(event_id.to_string()) {
                    Ok(ClaimResult::Claimed)
                } else {
                    Ok(ClaimResult::AlreadyClaimed)
                }
            })
        }
    }

    #[tokio::test]
    async fn claim_event_dedupes() {
        let store = MemoryStore {
            seen: Mutex::new(HashSet::new()),
        };
        assert_eq!(
            store.claim_event("evt_1").await.unwrap(),
            ClaimResult::Claimed
        );
        assert_eq!(
            store.claim_event("evt_1").await.unwrap(),
            ClaimResult::AlreadyClaimed
        );
    }
}
