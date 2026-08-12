//! Shared response decoding for generated route methods.

use crate::{types, Error, ResponseValue};
use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderValue};

pub(crate) const IDEMPOTENCY_KEY_HEADER: &str = "idempotency-key";

/// Decode a response body while accepting the empty body of a `204 No Content` response.
#[allow(clippy::result_large_err)]
fn decode_response_body<T>(
    status: reqwest::StatusCode,
    headers: HeaderMap,
    body: Bytes,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let body = if status == reqwest::StatusCode::NO_CONTENT && body.is_empty() {
        Bytes::from_static(b"null")
    } else {
        body
    };
    let inner = serde_json::from_slice(&body)
        .map_err(|error| Error::InvalidResponsePayload(body, error))?;
    Ok(ResponseValue::new(inner, status, headers))
}

/// Read and decode a generated route response while retaining status and headers.
async fn response_value<T>(
    response: reqwest::Response,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.bytes().await.map_err(Error::ResponseBodyError)?;
    decode_response_body(status, headers, body)
}

/// Preserve a provider idempotency key or synthesize the resolved request key.
#[allow(clippy::result_large_err)]
fn ensure_idempotency_key_header(
    headers: &mut HeaderMap,
    idempotency_key: &str,
) -> Result<(), Error<types::ErrorResponse>> {
    if headers.contains_key(IDEMPOTENCY_KEY_HEADER) {
        return Ok(());
    }

    let header_value = HeaderValue::try_from(idempotency_key).map_err(|error| {
        Error::InvalidRequest(format!("invalid resolved idempotency key: {error}"))
    })?;
    headers.insert(IDEMPOTENCY_KEY_HEADER, header_value);
    Ok(())
}

/// Decode a JSON response according to the route's documented success codes.
///
/// Any non-success status is treated as a Mollie HAL error body
/// ([`types::ErrorResponse`]), including global statuses such as `403` and
/// `429` that are often omitted from per-operation OpenAPI responses.
pub(crate) async fn json<T>(
    response: reqwest::Response,
    success_statuses: &[u16],
    _documented_error_statuses: &[u16],
    idempotency_key: &str,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let mut response = response;
    ensure_idempotency_key_header(response.headers_mut(), idempotency_key)?;
    let status = response.status().as_u16();
    if success_statuses.contains(&status) {
        return response_value(response).await;
    }

    Err(Error::ErrorResponse(response_value(response).await?))
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{decode_response_body, ensure_idempotency_key_header, IDEMPOTENCY_KEY_HEADER};
    use crate::ResponseValueExt;

    /// Adds the resolved key when the provider omits the response header.
    #[test]
    fn synthesizes_missing_idempotency_key() {
        let mut headers = reqwest::header::HeaderMap::new();
        ensure_idempotency_key_header(&mut headers, "resolved-key").expect("valid key");
        let response = crate::ResponseValue::new("ok", reqwest::StatusCode::OK, headers);

        assert_eq!(response.idempotency_key(), Some("resolved-key"));
    }

    /// Does not replace an idempotency key echoed by the provider.
    #[test]
    fn preserves_echoed_idempotency_key() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            IDEMPOTENCY_KEY_HEADER,
            "provider-key".parse().expect("valid key"),
        );
        ensure_idempotency_key_header(&mut headers, "resolved-key").expect("valid key");
        let response = crate::ResponseValue::new("ok", reqwest::StatusCode::OK, headers);

        assert_eq!(response.idempotency_key(), Some("provider-key"));
    }

    /// Decodes a successful empty response as unit data for `204 No Content`.
    #[test]
    fn decodes_empty_no_content_response() {
        let response = decode_response_body::<()>(
            reqwest::StatusCode::NO_CONTENT,
            reqwest::header::HeaderMap::new(),
            Bytes::new(),
        )
        .expect("204 response should decode");

        assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
        response.into_inner();
    }
}
