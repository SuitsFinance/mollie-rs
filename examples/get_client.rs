// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_client`.
//!
//! Route: `GET /clients/{organizationId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{GetClientResponse, OrganizationToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_client`.
struct GetClientExample;

impl RunnableExample for GetClientExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_client";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /clients/{organizationId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let organization_id: OrganizationToken = context.options().configured(
                "organization_id",
                from_value::<OrganizationToken>(json!({}))?,
            )?;

            let response: ResponseValue<GetClientResponse> = context
                .client()
                .get_client(
                    &organization_id,
                    Some(context.options().value("embed", "payments")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetClientExample).await
}
