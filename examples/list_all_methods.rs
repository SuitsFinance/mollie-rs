// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_all_methods`.
//!
//! Route: `GET /methods/all`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{Amount, ListAllMethodsResponse, Locale, ProfileToken, SequenceType};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_all_methods`.
struct ListAllMethodsExample;

impl RunnableExample for ListAllMethodsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_all_methods";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /methods/all";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let amount: Amount = context.options().configured(
                "amount",
                from_value::<Amount>(json!({
                    "currency": "EUR",
                    "value": "10.00"
                }))?,
            )?;
            let locale: Locale = context
                .options()
                .configured("locale", from_value::<Locale>(json!({}))?)?;
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListAllMethodsResponse> = context
                .client()
                .list_all_methods(
                    Some(&amount),
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
    support::run_example(ListAllMethodsExample).await
}
