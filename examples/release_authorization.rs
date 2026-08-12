// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::release_authorization`.
//!
//! Route: `POST /payments/{paymentId}/release-authorization`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentToken, ReleaseAuthorizationBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::release_authorization`.
struct ReleaseAuthorizationExample;

impl RunnableExample for ReleaseAuthorizationExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "release_authorization";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payments/{paymentId}/release-authorization";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: ReleaseAuthorizationBody = context
                .options()
                .body(ReleaseAuthorizationBody::default())?;

            let response: ResponseValue<()> = context
                .client()
                .release_authorization(&payment_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ReleaseAuthorizationExample).await
}
