// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::update_customer`.
//!
//! Route: `PATCH /customers/{customerId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerResponse, CustomerToken, UpdateCustomerBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::update_customer`.
struct UpdateCustomerExample;

impl RunnableExample for UpdateCustomerExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "update_customer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "PATCH /customers/{customerId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let body: UpdateCustomerBody = context.options().body(UpdateCustomerBody::default())?;

            let response: ResponseValue<CustomerResponse> = context
                .client()
                .update_customer(&customer_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(UpdateCustomerExample).await
}
