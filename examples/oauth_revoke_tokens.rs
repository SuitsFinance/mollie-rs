// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::oauth_revoke_tokens`.
//!
//! Route: `DELETE /oauth2/tokens`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::OauthRevokeTokensBody;
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::oauth_revoke_tokens`.
struct OauthRevokeTokensExample;

impl RunnableExample for OauthRevokeTokensExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "oauth_revoke_tokens";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /oauth2/tokens";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: OauthRevokeTokensBody = context
                .options()
                .body(from_value::<OauthRevokeTokensBody>(json!({}))?)?;

            let response: ResponseValue<()> = context
                .client()
                .oauth_revoke_tokens(
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
    support::run_example(OauthRevokeTokensExample).await
}
