// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::oauth_generate_tokens`.
//!
//! Route: `POST /oauth2/tokens`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{OauthGenerateTokensBody, OauthGenerateTokensResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::oauth_generate_tokens`.
struct OauthGenerateTokensExample;

impl RunnableExample for OauthGenerateTokensExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "oauth_generate_tokens";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /oauth2/tokens";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: OauthGenerateTokensBody = context
                .options()
                .body(from_value::<OauthGenerateTokensBody>(json!({}))?)?;

            let response: ResponseValue<OauthGenerateTokensResponse> = context
                .client()
                .oauth_generate_tokens(
                    context.options().value("authorization", "example-id"),
                    Some(context.options().value("content_type", "example-id")),
                    &body,
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(OauthGenerateTokensExample).await
}
