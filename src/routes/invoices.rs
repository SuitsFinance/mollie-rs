//! Generated invoices route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `invoices` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List invoices
    ///
    /// Retrieve a list of all your invoices, optionally filtered by year or by
    /// invoice reference.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/invoices`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `reference`: Filter for an invoice with a specific invoice reference, for example
    /// `2024.10000`.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    /// - `year`: Filter for invoices of a specific year, for example `2024`.
    pub async fn list_invoices<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        reference: Option<&'a str>,
        sort: Option<types::Sorting>,
        year: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListInvoicesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/invoices");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("reference", &reference))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("year", &year))
            .build()?;
        self.reject_testmode_for("list_invoices")?;
        let response = self.send(request, routes::Operation::ListInvoices).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get invoice
    ///
    /// Retrieve a single invoice by its ID.
    ///
    /// If you want to retrieve the details of an invoice by its invoice number,
    /// call the [List invoices](list-invoices) endpoint with the `reference` parameter.
    ///
    /// Sends a `GET` request to `/invoices/{invoiceId}`
    ///
    /// Arguments:
    /// - `invoice_id`: Provide the ID of the related invoice.
    pub async fn get_invoice<'a>(
        &'a self,
        invoice_id: &'a types::InvoiceToken,
    ) -> Result<ResponseValue<types::EntityInvoice>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/invoices/{}",
            encode_path(&invoice_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_invoice")?;
        let response = self.send(request, routes::Operation::GetInvoice).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
