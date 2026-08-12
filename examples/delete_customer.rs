// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::delete_customer`.
//!
//! Route: `DELETE /customers/{customerId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, DeleteCustomerBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::delete_customer`.
struct DeleteCustomerExample;

impl RunnableExample for DeleteCustomerExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "delete_customer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /customers/{customerId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let body: DeleteCustomerBody = context.options().body(DeleteCustomerBody::default())?;

            let response: ResponseValue<()> = context
                .client()
                .delete_customer(&customer_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(DeleteCustomerExample).await
}
