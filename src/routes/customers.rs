//! Generated customers route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `customers` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List customers
    ///
    /// Retrieve a list of all customers.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/customers`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_customers<'a>(
        &'a self,
        from: Option<&'a types::CustomerToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListCustomersResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/customers");
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
        let response = self.send(request, routes::Operation::ListCustomers).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Create customer
    ///
    /// Creates a simple minimal representation of a customer. Payments, recurring mandates, and subscriptions can be linked
    /// to this customer object, which simplifies management of recurring payments.
    ///
    /// Once registered, customers will also appear in your Mollie dashboard.
    ///
    /// Sends a `POST` request to `/customers`
    ///
    /// Arguments:
    pub async fn create_customer<'a>(
        &'a self,
        body: &'a types::EntityCustomer,
    ) -> Result<ResponseValue<types::CustomerResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/customers");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreateCustomer)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get customer
    ///
    /// Retrieve a single customer by its ID.
    ///
    /// Sends a `GET` request to `/customers/{customerId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_customer<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
    ) -> Result<ResponseValue<types::CustomerResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}",
            encode_path(&customer_id.to_string())
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
        let response = self.send(request, routes::Operation::GetCustomer).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Delete customer
    ///
    /// Delete a customer. All mandates and subscriptions created for this customer will be canceled as well.
    ///
    /// Sends a `DELETE` request to `/customers/{customerId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    pub async fn delete_customer<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        body: &'a types::DeleteCustomerBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::DeleteCustomer)
            .await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Update customer
    ///
    /// Update an existing customer.
    ///
    /// For an in-depth explanation of each parameter, refer to the [Create customer](create-customer) endpoint.
    ///
    /// Sends a `PATCH` request to `/customers/{customerId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    pub async fn update_customer<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        body: &'a types::UpdateCustomerBody,
    ) -> Result<ResponseValue<types::CustomerResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::UpdateCustomer)
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

    /// List customer payments
    ///
    /// Retrieve all payments linked to the customer.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/payments`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_customer_payments<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        from: Option<&'a types::PaymentToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListCustomerPaymentsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/customers/{}/payments",
            encode_path(&customer_id.to_string())
        ));
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
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListCustomerPayments)
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

    /// Create customer payment
    ///
    /// Creates a payment for the customer.
    ///
    /// Linking customers to payments enables you to:
    ///
    ///  Keep track of payment preferences for your customers
    ///  Allow your customers to charge a previously used credit card with a single click in our hosted checkout
    ///  Improve payment insights in the Mollie dashboard
    ///  Use recurring payments
    ///
    /// This endpoint is effectively an alias of the [Create payment endpoint](create-payment) with the `customerId`
    /// parameter predefined.
    ///
    /// Sends a `POST` request to `/customers/{customerId}/payments`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    pub async fn create_customer_payment<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        body: &'a types::PaymentRequest,
    ) -> Result<ResponseValue<types::PaymentResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/payments",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreateCustomerPayment)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16, 503u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List mandates
    ///
    /// Retrieve a list of all mandates.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/mandates`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `scopes`: Returns only mandates that include the specified scopes.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_mandates<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        from: Option<&'a types::MandateToken>,
        limit: Option<::std::num::NonZeroU64>,
        scopes: Option<&'a ::std::vec::Vec<types::MandateScopes>>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListMandatesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/mandates",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("scopes", &scopes))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::ListMandates).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Create mandate
    ///
    /// Create a mandate for a specific customer. Mandates allow you to charge a customer's card, PayPal account or bank
    /// account recurrently.
    ///
    /// It is only possible to create mandates for IBANs and PayPal billing agreements with this endpoint. To create
    /// mandates for cards, your customers need to perform a 'first payment' with their card.
    ///
    /// Sends a `POST` request to `/customers/{customerId}/mandates`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    pub async fn create_mandate<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        body: &'a types::MandateRequest,
    ) -> Result<ResponseValue<types::MandateResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/mandates",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateMandate).await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get mandate
    ///
    /// Retrieve a single mandate by its ID. Depending on the type of mandate, the object will contain the customer's bank
    /// account details, card details, or PayPal account details.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/mandates/{mandateId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `mandate_id`: Provide the ID of the related mandate.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_mandate<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        mandate_id: &'a types::MandateToken,
    ) -> Result<ResponseValue<types::MandateResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/mandates/{}",
            encode_path(&customer_id.to_string()),
            encode_path(&mandate_id.to_string())
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
        let response = self.send(request, routes::Operation::GetMandate).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Revoke mandate
    ///
    /// Revoke a customer's mandate. You will no longer be able to charge the customer's bank account or card with this
    /// mandate, and all connected subscriptions will be canceled.
    ///
    /// Sends a `DELETE` request to `/customers/{customerId}/mandates/{mandateId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `mandate_id`: Provide the ID of the related mandate.
    pub async fn revoke_mandate<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        mandate_id: &'a types::MandateToken,
        body: &'a types::RevokeMandateBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/mandates/{}",
            encode_path(&customer_id.to_string()),
            encode_path(&mandate_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::RevokeMandate).await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List customer subscriptions
    ///
    /// Retrieve all subscriptions of a customer.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/subscriptions`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_subscriptions<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        from: Option<&'a types::SubscriptionToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListSubscriptionsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions",
            encode_path(&customer_id.to_string())
        ));
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
        let response = self
            .send(request, routes::Operation::ListSubscriptions)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Create subscription
    ///
    /// With subscriptions, you can schedule recurring payments to take place at regular intervals.
    ///
    /// For example, by simply specifying an `amount` and an `interval`, you can create an endless subscription to charge a
    /// monthly fee, until you cancel the subscription.
    ///
    /// Or, you could use the times parameter to only charge a limited number of times, for example to split a big
    /// transaction in multiple parts.
    ///
    /// A few example usages:
    ///
    /// `amount[currency]="EUR"` `amount[value]="5.00"` `interval="2 weeks"`
    /// Your customer will be charged €5 once every two weeks.
    ///
    /// `amount[currency]="EUR"` `amount[value]="20.00"` `interval="1 day" times=5`
    /// Your customer will be charged €20 every day, for five consecutive days.
    ///
    /// `amount[currency]="EUR"` `amount[value]="10.00"` `interval="1 month"`
    /// `startDate="2018-04-30"`
    /// Your customer will be charged €10 on the last day of each month, starting in April 2018.
    ///
    /// Sends a `POST` request to `/customers/{customerId}/subscriptions`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    pub async fn create_subscription<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        body: &'a types::SubscriptionRequest,
    ) -> Result<ResponseValue<types::SubscriptionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions",
            encode_path(&customer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreateSubscription)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get subscription
    ///
    /// Retrieve a single subscription by its ID and the ID of its parent customer.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/subscriptions/{subscriptionId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `subscription_id`: Provide the ID of the related subscription.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_subscription<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        subscription_id: &'a types::SubscriptionToken,
    ) -> Result<ResponseValue<types::SubscriptionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions/{}",
            encode_path(&customer_id.to_string()),
            encode_path(&subscription_id.to_string())
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
            .send(request, routes::Operation::GetSubscription)
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

    /// Cancel subscription
    ///
    /// Cancel an existing subscription. Canceling a subscription has no effect on the mandates of the customer.
    ///
    /// Sends a `DELETE` request to `/customers/{customerId}/subscriptions/{subscriptionId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `subscription_id`: Provide the ID of the related subscription.
    pub async fn cancel_subscription<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        subscription_id: &'a types::SubscriptionToken,
        body: &'a types::CancelSubscriptionBody,
    ) -> Result<ResponseValue<types::SubscriptionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions/{}",
            encode_path(&customer_id.to_string()),
            encode_path(&subscription_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CancelSubscription)
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

    /// Update subscription
    ///
    /// Update an existing subscription.
    ///
    /// Canceled subscriptions cannot be updated.
    ///
    /// For an in-depth explanation of each parameter, refer to the [Create subscription](create-subscription) endpoint.
    ///
    /// Sends a `PATCH` request to `/customers/{customerId}/subscriptions/{subscriptionId}`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `subscription_id`: Provide the ID of the related subscription.
    pub async fn update_subscription<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        subscription_id: &'a types::SubscriptionToken,
        body: &'a types::UpdateSubscriptionBody,
    ) -> Result<ResponseValue<types::SubscriptionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions/{}",
            encode_path(&customer_id.to_string()),
            encode_path(&subscription_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::UpdateSubscription)
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

    /// List all subscriptions
    ///
    /// Retrieve all subscriptions initiated across all your customers.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/subscriptions`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to retrieve subscriptions for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` is already implied.
    ///
    /// To retrieve all subscriptions across the organization, use an organization-level API credential and omit the
    /// `profileId` parameter.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_all_subscriptions<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListAllSubscriptionsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/subscriptions");
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
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListAllSubscriptions)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List subscription payments
    ///
    /// Retrieve all payments of a specific subscription.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/customers/{customerId}/subscriptions/{subscriptionId}/payments`
    ///
    /// Arguments:
    /// - `customer_id`: Provide the ID of the related customer.
    /// - `subscription_id`: Provide the ID of the related subscription.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_subscription_payments<'a>(
        &'a self,
        customer_id: &'a types::CustomerToken,
        subscription_id: &'a types::SubscriptionToken,
        from: Option<&'a types::PaymentToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListSubscriptionPaymentsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/customers/{}/subscriptions/{}/payments",
            encode_path(&customer_id.to_string()),
            encode_path(&subscription_id.to_string())
        ));
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
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListSubscriptionPayments)
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
}
