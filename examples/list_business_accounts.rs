// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_business_accounts`.
//!
//! Route: `GET /business-accounts/accounts`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BusinessAccountToken, ListBusinessAccountsResponse, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_business_accounts`.
struct ListBusinessAccountsExample;

impl RunnableExample for ListBusinessAccountsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_business_accounts";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /business-accounts/accounts";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<BusinessAccountToken> =
                context.options().optional_configured("from")?;

            let response: ResponseValue<ListBusinessAccountsResponse> = context
                .client()
                .list_business_accounts(
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
    support::run_example(ListBusinessAccountsExample).await
}
