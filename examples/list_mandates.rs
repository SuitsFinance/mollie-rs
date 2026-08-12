// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_mandates`.
//!
//! Route: `GET /customers/{customerId}/mandates`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, ListMandatesResponse, MandateScopes, MandateToken, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_mandates`.
struct ListMandatesExample;

impl RunnableExample for ListMandatesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_mandates";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers/{customerId}/mandates";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let from: Option<MandateToken> = context.options().optional_token("from");
            let scopes: ::std::vec::Vec<MandateScopes> = ::std::vec::Vec::new();

            let response: ResponseValue<ListMandatesResponse> = context
                .client()
                .list_mandates(
                    &customer_id,
                    from.as_ref(),
                    context.options().limit(50),
                    Some(&scopes),
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
    support::run_example(ListMandatesExample).await
}
