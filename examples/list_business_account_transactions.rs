// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_business_account_transactions`.
//!
//! Route: `GET /business-accounts/accounts/{businessAccountId}/transactions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    BusinessAccountToken, BusinessAccountTransactionToken, ListBusinessAccountTransactionsResponse,
    Sorting,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_business_account_transactions`.
struct ListBusinessAccountTransactionsExample;

impl RunnableExample for ListBusinessAccountTransactionsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_business_account_transactions";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /business-accounts/accounts/{businessAccountId}/transactions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let business_account_id: BusinessAccountToken = context.options().configured(
                "business_account_id",
                from_value::<BusinessAccountToken>(json!({}))?,
            )?;
            let from: Option<BusinessAccountTransactionToken> =
                context.options().optional_configured("from")?;

            let response: ResponseValue<ListBusinessAccountTransactionsResponse> = context
                .client()
                .list_business_account_transactions(
                    &business_account_id,
                    from.as_ref(),
                    context.options().limit(50),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListBusinessAccountTransactionsExample).await
}
