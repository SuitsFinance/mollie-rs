// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_method`.
//!
//! Route: `GET /methods/{methodId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityMethodGet, Locale, Method, ProfileToken, SequenceType};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_method`.
struct GetMethodExample;

impl RunnableExample for GetMethodExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_method";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /methods/{methodId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let method_id: Method = context
                .options()
                .configured("method_id", from_value::<Method>(json!({}))?)?;
            let locale: Locale = context
                .options()
                .configured("locale", from_value::<Locale>(json!({}))?)?;
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<EntityMethodGet> = context
                .client()
                .get_method(
                    &method_id,
                    Some(context.options().value("currency", "EUR")),
                    Some(context.options().value("include", "issuers")),
                    Some(&locale),
                    profile_id.as_ref(),
                    Some(
                        context
                            .options()
                            .configured("sequence_type", SequenceType::Oneoff)?,
                    ),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetMethodExample).await
}
