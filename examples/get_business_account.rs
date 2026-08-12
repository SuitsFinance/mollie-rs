// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_business_account`.
//!
//! Route: `GET /business-accounts/accounts/{businessAccountId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BusinessAccountResponse, BusinessAccountToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_business_account`.
struct GetBusinessAccountExample;

impl RunnableExample for GetBusinessAccountExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_business_account";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /business-accounts/accounts/{businessAccountId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let business_account_id: BusinessAccountToken = context.options().configured(
                "business_account_id",
                from_value::<BusinessAccountToken>(json!({}))?,
            )?;

            let response: ResponseValue<BusinessAccountResponse> = context
                .client()
                .get_business_account(&business_account_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetBusinessAccountExample).await
}
