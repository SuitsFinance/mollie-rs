// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_customer`.
//!
//! Route: `POST /customers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerResponse, EntityCustomer};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_customer`.
struct CreateCustomerExample;

impl RunnableExample for CreateCustomerExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_customer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /customers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: EntityCustomer = context.options().body(EntityCustomer::default())?;

            let response: ResponseValue<CustomerResponse> =
                context.client().create_customer(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateCustomerExample).await
}
