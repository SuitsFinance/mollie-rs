// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_webhooks`.
//!
//! Route: `GET /webhooks`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListWebhooksResponse, Sorting, WebhookEventTypes};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_webhooks`.
struct ListWebhooksExample;

impl RunnableExample for ListWebhooksExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_webhooks";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /webhooks";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListWebhooksResponse> = context
                .client()
                .list_webhooks(
                    Some(
                        context
                            .options()
                            .configured("event_types", WebhookEventTypes::PaymentPaid)?,
                    ),
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListWebhooksExample).await
}
