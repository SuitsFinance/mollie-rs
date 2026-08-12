// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_all_refunds`.
//!
//! Route: `GET /refunds`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListAllRefundsResponse, ProfileToken, RefundToken, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_all_refunds`.
struct ListAllRefundsExample;

impl RunnableExample for ListAllRefundsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_all_refunds";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /refunds";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<RefundToken> = context.options().optional_token("from");
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListAllRefundsResponse> = context
                .client()
                .list_all_refunds(
                    Some(context.options().value("embed", "payments")),
                    from.as_ref(),
                    context.options().limit(50),
                    profile_id.as_ref(),
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
    support::run_example(ListAllRefundsExample).await
}
