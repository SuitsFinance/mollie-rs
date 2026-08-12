// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::revoke_mandate`.
//!
//! Route: `DELETE /customers/{customerId}/mandates/{mandateId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, MandateToken, RevokeMandateBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::revoke_mandate`.
struct RevokeMandateExample;

impl RunnableExample for RevokeMandateExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "revoke_mandate";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /customers/{customerId}/mandates/{mandateId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let mandate_id: MandateToken = context.options().token("mandate_id", "mdt_1234567890");
            let body: RevokeMandateBody = context.options().body(RevokeMandateBody::default())?;

            let response: ResponseValue<()> = context
                .client()
                .revoke_mandate(&customer_id, &mandate_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(RevokeMandateExample).await
}
