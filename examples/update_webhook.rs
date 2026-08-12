// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::update_webhook`.
//!
//! Route: `PATCH /webhooks/{webhookId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityWebhook, UpdateWebhookBody, WebhookToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::update_webhook`.
struct UpdateWebhookExample;

impl RunnableExample for UpdateWebhookExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "update_webhook";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "PATCH /webhooks/{webhookId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let webhook_id: WebhookToken = context
                .options()
                .configured("webhook_id", from_value::<WebhookToken>(json!({}))?)?;
            let body: UpdateWebhookBody = context.options().body(UpdateWebhookBody::default())?;

            let response: ResponseValue<EntityWebhook> =
                context.client().update_webhook(&webhook_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(UpdateWebhookExample).await
}
