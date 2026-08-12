//! Handwritten payment-domain facades over the generated route surface.
//!
//! These APIs add validation, idempotency scoping, pagination helpers, and
//! documentation without reimplementing HTTP.

pub mod captures;
mod common;
pub mod connect_balance_transfers;
pub mod mandates;
pub mod oauth;
pub mod payment_links;
pub mod payments;
pub mod payouts;
pub mod refunds;
pub mod sessions;
pub mod subscriptions;
pub mod terminals;
pub mod transfers;
pub mod unmatched_credit_transfers;
pub mod verify_payee;
pub mod webhooks;

pub use captures::CapturesApi;
pub use connect_balance_transfers::ConnectBalanceTransfersApi;
pub use mandates::MandatesApi;
pub use oauth::OAuthApi;
pub use payment_links::PaymentLinksApi;
pub use payments::PaymentsApi;
pub use payouts::PayoutsApi;
pub use refunds::RefundsApi;
pub use sessions::SessionsApi;
pub use subscriptions::SubscriptionsApi;
pub use terminals::TerminalsApi;
pub use transfers::{TransferClientSignature, TransfersApi};
pub use unmatched_credit_transfers::UnmatchedCreditTransfersApi;
pub use verify_payee::VerifyPayeeApi;
pub use webhooks::WebhooksApi;
