// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_webhook`.
//!
//! Route: `POST /webhooks`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CreateWebhook, CreateWebhookBody};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_webhook`.
struct CreateWebhookExample;

impl RunnableExample for CreateWebhookExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_webhook";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /webhooks";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: CreateWebhookBody =
                context
                    .options()
                    .body(from_value::<CreateWebhookBody>(json!({
                        "eventTypes": [
                            "payment-link.paid"
                        ],
                        "name": "Payment links webhook",
                        "url": "https://example.com/webhooks/mollie"
                    }))?)?;

            let response: ResponseValue<CreateWebhook> =
                context.client().create_webhook(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateWebhookExample).await
}
