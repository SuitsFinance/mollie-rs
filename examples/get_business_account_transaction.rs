// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_business_account_transaction`.
//!
//! Route: `GET /business-accounts/accounts/{businessAccountId}/transactions/{transactionId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    BusinessAccountToken, BusinessAccountTransactionToken, TransactionResponse,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_business_account_transaction`.
struct GetBusinessAccountTransactionExample;

impl RunnableExample for GetBusinessAccountTransactionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_business_account_transaction";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str =
        "GET /business-accounts/accounts/{businessAccountId}/transactions/{transactionId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let business_account_id: BusinessAccountToken = context.options().configured(
                "business_account_id",
                from_value::<BusinessAccountToken>(json!({}))?,
            )?;
            let transaction_id: BusinessAccountTransactionToken = context.options().configured(
                "transaction_id",
                from_value::<BusinessAccountTransactionToken>(json!({}))?,
            )?;

            let response: ResponseValue<TransactionResponse> = context
                .client()
                .get_business_account_transaction(&business_account_id, &transaction_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetBusinessAccountTransactionExample).await
}
