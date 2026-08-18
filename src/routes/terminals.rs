//! Generated terminals route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `terminals` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List terminals
    ///
    /// Retrieve a list of all physical point-of-sale devices.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/terminals`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_terminals<'a>(
        &'a self,
        from: Option<&'a types::TerminalToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListTerminalsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/terminals");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::ListTerminals).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get terminal
    ///
    /// Retrieve a single terminal by its ID.
    ///
    /// Sends a `GET` request to `/terminals/{terminalId}`
    ///
    /// Arguments:
    /// - `terminal_id`: Provide the ID of the related terminal.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_terminal<'a>(
        &'a self,
        terminal_id: &'a types::TerminalToken,
    ) -> Result<ResponseValue<types::EntityTerminal>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/terminals/{}",
            encode_path(&terminal_id.to_string())
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
        let response = self.send(request, routes::Operation::GetTerminal).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List terminal pairing codes
    ///
    /// > ℹ️ **Test mode**
    /// >
    /// > This endpoint currently does not support test mode yet.
    ///
    /// Returns all pairing codes: `active`, `expired`, and `revoked`. Results are paginated.
    ///
    /// Sends a `GET` request to `/terminals/pairing-codes`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to retrieve pairing codes for.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    pub async fn terminals_list_pairing_codes<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a str>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::TerminalsListPairingCodesResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/terminals/pairing-codes");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "profileId",
                &profile_id,
            ))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .build()?;
        let response = self
            .send(request, routes::Operation::TerminalsListPairingCodes)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Request terminal pairing code
    ///
    /// > ℹ️ **Test mode**
    /// >
    /// > This endpoint currently does not support test mode yet.
    ///
    /// Request a pairing code to onboard a point-of-sale terminal.
    ///
    /// The response includes a human-readable `code` for manual entry on the terminal, and a QR Code as a
    /// base64 encoded SVG data URI for scanning if you specify the query parameter `include` with value `details.qrCode`.
    ///
    /// Pairing codes expire after 90 days (see `expiresAt`) and can be used multiple times.
    ///
    /// Sends a `POST` request to `/terminals/pairing-codes`
    ///
    /// Arguments:
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    pub async fn terminals_request_pairing_code<'a>(
        &'a self,
        include: Option<&'a str>,
        body: &'a types::TerminalsRequestPairingCodeBody,
    ) -> Result<ResponseValue<types::EntityPairingCode>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/terminals/pairing-codes");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .json(&body)
            .query(&progenitor_client::QueryParam::new("include", &include))
            .build()?;
        let response = self
            .send(request, routes::Operation::TerminalsRequestPairingCode)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get terminal pairing code
    ///
    /// > ℹ️ **Test mode**
    /// >
    /// > This endpoint currently does not support test mode yet.
    ///
    /// Get a pairing code to onboard a point-of-sale terminal.
    ///
    /// The response includes a human-readable `code` for manual entry on the terminal and, optionally, a QR Code as a
    /// base64 encoded SVG data URI when you use the `include` query parameter with value `details.qrCode`.
    ///
    /// Sends a `GET` request to `/terminals/pairing-codes/{pairingCodeId}`
    ///
    /// Arguments:
    /// - `pairing_code_id`: Provide the ID of the terminal pairing code.
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    pub async fn terminals_get_pairing_code<'a>(
        &'a self,
        pairing_code_id: &'a types::TerminalPairingCodeToken,
        include: Option<&'a str>,
    ) -> Result<ResponseValue<types::EntityPairingCode>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/terminals/pairing-codes/{}",
            encode_path(&pairing_code_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("include", &include))
            .build()?;
        let response = self
            .send(request, routes::Operation::TerminalsGetPairingCode)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Revoke terminal pairing code
    ///
    /// > ℹ️ **Test mode**
    /// >
    /// > This endpoint currently does not support test mode yet.
    ///
    /// Revoke a pairing code, preventing the onboarding of new point-of-sale terminals.
    ///
    /// Terminals that have already paired with this code are not affected.
    ///
    /// Sends a `DELETE` request to `/terminals/pairing-codes/{pairingCodeId}`
    ///
    /// Arguments:
    /// - `pairing_code_id`: Provide the ID of the terminal pairing code.
    pub async fn terminals_revoke_pairing_code<'a>(
        &'a self,
        pairing_code_id: &'a types::TerminalPairingCodeToken,
    ) -> Result<ResponseValue<types::EntityPairingCode>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/terminals/pairing-codes/{}",
            encode_path(&pairing_code_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::TerminalsRevokePairingCode)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
