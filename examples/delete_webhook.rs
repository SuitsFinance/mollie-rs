// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::delete_webhook`.
//!
//! Route: `DELETE /webhooks/{webhookId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{DeleteWebhookBody, WebhookToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::delete_webhook`.
struct DeleteWebhookExample;

impl RunnableExample for DeleteWebhookExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "delete_webhook";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /webhooks/{webhookId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let webhook_id: WebhookToken = context
                .options()
                .configured("webhook_id", from_value::<WebhookToken>(json!({}))?)?;
            let body: DeleteWebhookBody = context.options().body(DeleteWebhookBody::default())?;

            let response: ResponseValue<()> =
                context.client().delete_webhook(&webhook_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(DeleteWebhookExample).await
}
