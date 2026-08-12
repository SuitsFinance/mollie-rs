//! Generated Mollie API route groups.
//!
//! Each public module owns one operation area while the methods remain
//! inherent methods on [`crate::Client`].

mod operations;
pub(crate) mod response;

pub(crate) use operations::Operation;

pub mod accounts;
pub mod balances;
pub mod capabilities;
pub mod captures;
pub mod chargebacks;
pub mod clients;
pub mod connect;
pub mod customers;
pub mod invoices;
pub mod methods;
pub mod oauth;
pub mod onboarding;
pub mod organizations;
pub mod payment_links;
pub mod payments;
pub mod payouts;
pub mod permissions;
pub mod profiles;
pub mod refunds;
pub mod sales_invoices;
pub mod sessions;
pub mod settlements;
pub mod terminals;
pub mod transfers;
pub mod unmatched_credit_transfers;
pub mod verify_payee;
pub mod wallets;
pub mod webhooks;
