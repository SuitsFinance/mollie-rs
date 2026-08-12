// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_webhook_event`.
//!
//! Route: `GET /events/{webhookEventId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityWebhookEvent, WebhookEventToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_webhook_event`.
struct GetWebhookEventExample;

impl RunnableExample for GetWebhookEventExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_webhook_event";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /events/{webhookEventId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let webhook_event_id: WebhookEventToken = context.options().configured(
                "webhook_event_id",
                from_value::<WebhookEventToken>(json!({}))?,
            )?;

            let response: ResponseValue<EntityWebhookEvent> = context
                .client()
                .get_webhook_event(&webhook_event_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetWebhookEventExample).await
}
