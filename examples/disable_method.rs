// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::disable_method`.
//!
//! Route: `DELETE /profiles/{profileId}/methods/{methodId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{DisableMethodProfileId, Method, ProfileToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::disable_method`.
struct DisableMethodExample;

impl RunnableExample for DisableMethodExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "disable_method";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /profiles/{profileId}/methods/{methodId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: DisableMethodProfileId = DisableMethodProfileId::from(
                ProfileToken::try_from(
                    context
                        .options()
                        .value("profile_id", "pfl_1234567890")
                        .to_owned(),
                )
                .expect("valid profile token fixture"),
            );
            let method_id: Method = context
                .options()
                .configured("method_id", from_value::<Method>(json!({}))?)?;

            let response: ResponseValue<()> = context
                .client()
                .disable_method(&profile_id, &method_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(DisableMethodExample).await
}
