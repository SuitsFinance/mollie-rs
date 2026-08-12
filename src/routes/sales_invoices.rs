//! Generated sales invoices route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `sales invoices` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List sales invoices
    ///
    /// Retrieve a list of all sales invoices created through the API.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/sales-invoices`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_sales_invoices<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListSalesInvoicesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/sales-invoices");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListSalesInvoices)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create sales invoice
    ///
    /// With the Sales Invoice API you can generate sales invoices to send to your customers.
    ///
    /// Sends a `POST` request to `/sales-invoices`
    ///
    /// Arguments:
    pub async fn create_sales_invoice<'a>(
        &'a self,
        body: &'a types::SalesInvoiceRequest,
    ) -> Result<ResponseValue<types::SalesInvoiceResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/sales-invoices");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreateSalesInvoice)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get sales invoice
    ///
    /// Retrieve a single sales invoice by its ID.
    ///
    /// Sends a `GET` request to `/sales-invoices/{salesInvoiceId}`
    ///
    /// Arguments:
    /// - `sales_invoice_id`: Provide the ID of the related sales invoice.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_sales_invoice<'a>(
        &'a self,
        sales_invoice_id: &'a types::SalesInvoiceToken,
    ) -> Result<ResponseValue<types::SalesInvoiceResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/sales-invoices/{}",
            encode_path(&sales_invoice_id.to_string())
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
            .send(request, routes::Operation::GetSalesInvoice)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Delete sales invoice
    ///
    /// Sales invoices which are in status `draft` can be deleted. For all other statuses, please use the
    /// [Update sales invoice](update-sales-invoice) endpoint instead.
    ///
    /// Sends a `DELETE` request to `/sales-invoices/{salesInvoiceId}`
    ///
    /// Arguments:
    /// - `sales_invoice_id`: Provide the ID of the related sales invoice.
    pub async fn delete_sales_invoice<'a>(
        &'a self,
        sales_invoice_id: &'a types::SalesInvoiceToken,
        body: &'a types::DeleteValuesSalesInvoice,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/sales-invoices/{}",
            encode_path(&sales_invoice_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::DeleteSalesInvoice)
            .await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Update sales invoice
    ///
    /// Certain details of an existing sales invoice can be updated. For `draft` it is all values listed below, but for
    /// statuses `paid` and `issued` there are certain additional requirements (`paymentDetails` and `emailDetails`,
    /// respectively).
    ///
    /// Sends a `PATCH` request to `/sales-invoices/{salesInvoiceId}`
    ///
    /// Arguments:
    /// - `sales_invoice_id`: Provide the ID of the related sales invoice.
    pub async fn update_sales_invoice<'a>(
        &'a self,
        sales_invoice_id: &'a types::SalesInvoiceToken,
        body: &'a types::UpdateSalesInvoiceBody,
    ) -> Result<ResponseValue<types::SalesInvoiceResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/sales-invoices/{}",
            encode_path(&sales_invoice_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::UpdateSalesInvoice)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
