// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_webhook`.
//!
//! Route: `GET /webhooks/{webhookId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityWebhook, WebhookToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_webhook`.
struct GetWebhookExample;

impl RunnableExample for GetWebhookExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_webhook";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /webhooks/{webhookId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let webhook_id: WebhookToken = context
                .options()
                .configured("webhook_id", from_value::<WebhookToken>(json!({}))?)?;

            let response: ResponseValue<EntityWebhook> =
                context.client().get_webhook(&webhook_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetWebhookExample).await
}
