// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_payments`.
//!
//! Route: `GET /payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListPaymentsResponse, PaymentToken, ProfileToken, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_payments`.
struct ListPaymentsExample;

impl RunnableExample for ListPaymentsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_payments";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<PaymentToken> = context.options().optional_token("from");
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListPaymentsResponse> = context
                .client()
                .list_payments(
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
    support::run_example(ListPaymentsExample).await
}
