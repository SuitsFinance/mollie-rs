// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::test_webhook`.
//!
//! Route: `POST /webhooks/{webhookId}/ping`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{TestWebhookBody, WebhookToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::test_webhook`.
struct TestWebhookExample;

impl RunnableExample for TestWebhookExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "test_webhook";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /webhooks/{webhookId}/ping";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let webhook_id: WebhookToken = context
                .options()
                .configured("webhook_id", from_value::<WebhookToken>(json!({}))?)?;
            let body: TestWebhookBody = context.options().body(TestWebhookBody::default())?;

            let response: ResponseValue<()> =
                context.client().test_webhook(&webhook_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(TestWebhookExample).await
}
