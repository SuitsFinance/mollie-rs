//! Generated webhooks route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `webhooks` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List all webhooks
    ///
    /// Returns a paginated list of your webhooks. If no webhook endpoints are available, the resulting array will be empty. This request should never throw an error.
    ///
    /// Sends a `GET` request to `/webhooks`
    ///
    /// Arguments:
    /// - `event_types`: Used to filter out only the webhooks that are subscribed to certain types of events.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_webhooks<'a>(
        &'a self,
        event_types: Option<types::WebhookEventTypes>,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListWebhooksResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/webhooks");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "eventTypes",
                &event_types,
            ))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::ListWebhooks).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create a webhook
    ///
    /// A webhook must have a name, an url and a list of event types. You can also create webhooks in the webhooks settings section of the Dashboard.
    ///
    /// Sends a `POST` request to `/webhooks`
    ///
    /// Arguments:
    pub async fn create_webhook<'a>(
        &'a self,
        body: &'a types::CreateWebhookBody,
    ) -> Result<ResponseValue<types::CreateWebhook>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/webhooks");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateWebhook).await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get a webhook
    ///
    /// Retrieve a single webhook object by its ID.
    ///
    /// Sends a `GET` request to `/webhooks/{webhookId}`
    ///
    /// Arguments:
    /// - `webhook_id`: Provide the ID of the related webhook.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_webhook<'a>(
        &'a self,
        webhook_id: &'a types::WebhookToken,
    ) -> Result<ResponseValue<types::EntityWebhook>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/webhooks/{}",
            encode_path(&webhook_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::GetWebhook).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Delete a webhook
    ///
    /// Delete a single webhook object by its webhook ID.
    ///
    /// Sends a `DELETE` request to `/webhooks/{webhookId}`
    ///
    /// Arguments:
    /// - `webhook_id`: Provide the ID of the related webhook.
    pub async fn delete_webhook<'a>(
        &'a self,
        webhook_id: &'a types::WebhookToken,
        body: &'a types::DeleteWebhookBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/webhooks/{}",
            encode_path(&webhook_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::DeleteWebhook).await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Update a webhook
    ///
    /// Updates the webhook. You may edit the name, url and the list of subscribed event types.
    ///
    /// Sends a `PATCH` request to `/webhooks/{webhookId}`
    ///
    /// Arguments:
    /// - `webhook_id`: Provide the ID of the related webhook.
    pub async fn update_webhook<'a>(
        &'a self,
        webhook_id: &'a types::WebhookToken,
        body: &'a types::UpdateWebhookBody,
    ) -> Result<ResponseValue<types::EntityWebhook>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/webhooks/{}",
            encode_path(&webhook_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::UpdateWebhook).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Test a webhook
    ///
    /// Sends a test event to the webhook to verify the endpoint is working as expected.
    ///
    /// Sends a `POST` request to `/webhooks/{webhookId}/ping`
    ///
    /// Arguments:
    /// - `webhook_id`: Provide the ID of the related webhook.
    pub async fn test_webhook<'a>(
        &'a self,
        webhook_id: &'a types::WebhookToken,
        body: &'a types::TestWebhookBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/webhooks/{}/ping",
            encode_path(&webhook_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::TestWebhook).await?;
        routes::response::json(
            response,
            &[202u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get a Webhook Event
    ///
    /// Retrieve a single webhook event object by its event ID.
    ///
    /// Sends a `GET` request to `/events/{webhookEventId}`
    ///
    /// Arguments:
    /// - `webhook_event_id`: Provide the ID of the related webhook event.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_webhook_event<'a>(
        &'a self,
        webhook_event_id: &'a types::WebhookEventToken,
    ) -> Result<ResponseValue<types::EntityWebhookEvent>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/events/{}",
            encode_path(&webhook_event_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::GetWebhookEvent)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
