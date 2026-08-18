//! Shared response decoding for generated route methods.

use crate::{types, Error, ResponseLimits, ResponseValue};
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

/// Buffer a response body with an explicit byte ceiling.
///
/// Rejects oversized `Content-Length` before reading and stops streaming once
/// the cumulative body would exceed `max_bytes`.
pub(crate) async fn read_body_limited(
    mut response: reqwest::Response,
    max_bytes: usize,
) -> Result<Bytes, Error<types::ErrorResponse>> {
    if let Some(declared) = response.content_length() {
        // content_length is u64; compare carefully against usize ceiling.
        if declared > max_bytes as u64 {
            return Err(Error::InvalidRequest(format!(
                "response body Content-Length {declared} exceeds limit of {max_bytes} bytes"
            )));
        }
    }

    let mut buffered: Vec<u8> = Vec::new();
    loop {
        let chunk = response.chunk().await.map_err(Error::ResponseBodyError)?;
        let Some(chunk) = chunk else {
            break;
        };
        let next_len = buffered.len().saturating_add(chunk.len());
        if next_len > max_bytes {
            return Err(Error::InvalidRequest(format!(
                "response body exceeds limit of {max_bytes} bytes"
            )));
        }
        buffered.extend_from_slice(&chunk);
    }

    Ok(Bytes::from(buffered))
}

/// Read and decode a generated route response while retaining status and headers.
async fn response_value<T>(
    response: reqwest::Response,
    max_bytes: usize,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let headers = response.headers().clone();
    let body = read_body_limited(response, max_bytes).await?;
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
///
/// Body buffering is capped by [`ResponseLimits`] (success vs error ceilings).
pub(crate) async fn json<T>(
    response: reqwest::Response,
    success_statuses: &[u16],
    _documented_error_statuses: &[u16],
    idempotency_key: &str,
    limits: ResponseLimits,
) -> Result<ResponseValue<T>, Error<types::ErrorResponse>>
where
    T: serde::de::DeserializeOwned,
{
    let mut response = response;
    ensure_idempotency_key_header(response.headers_mut(), idempotency_key)?;
    let status = response.status().as_u16();
    if success_statuses.contains(&status) {
        return response_value(response, limits.max_json_bytes).await;
    }

    Err(Error::ErrorResponse(
        response_value(response, limits.max_error_body_bytes).await?,
    ))
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::{
        decode_response_body, ensure_idempotency_key_header, json, read_body_limited,
        IDEMPOTENCY_KEY_HEADER,
    };
    use crate::{ResponseLimits, ResponseValueExt};

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

    #[tokio::test]
    async fn accepts_body_exactly_at_limit() {
        let server = MockServer::start().await;
        let body = "x".repeat(8);
        Mock::given(method("GET"))
            .and(path("/at-limit"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body.clone()))
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/at-limit", server.uri()))
            .send()
            .await
            .expect("send");
        let bytes = read_body_limited(response, 8).await.expect("at limit");
        assert_eq!(bytes.as_ref(), body.as_bytes());
    }

    #[tokio::test]
    async fn rejects_body_one_byte_over_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/over"))
            .respond_with(ResponseTemplate::new(200).set_body_string("123456789"))
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/over", server.uri()))
            .send()
            .await
            .expect("send");
        let err = read_body_limited(response, 8)
            .await
            .expect_err("over limit");
        assert!(
            matches!(err, crate::Error::InvalidRequest(ref message) if message.contains("exceeds limit")),
            "unexpected error: {err:?}"
        );
    }

    #[tokio::test]
    async fn rejects_declared_content_length_over_limit() {
        let server = MockServer::start().await;
        // Honest Content-Length larger than the ceiling — reject before buffer.
        let body = "a".repeat(64);
        Mock::given(method("GET"))
            .and(path("/huge-cl"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/huge-cl", server.uri()))
            .send()
            .await
            .expect("send");
        assert_eq!(response.content_length(), Some(64));
        let err = read_body_limited(response, 8)
            .await
            .expect_err("over limit");
        assert!(
            matches!(
                err,
                crate::Error::InvalidRequest(ref message)
                    if message.contains("Content-Length") && message.contains("exceeds limit")
            ),
            "unexpected error: {err:?}"
        );
    }

    #[tokio::test]
    async fn json_uses_error_body_limit_for_non_success() {
        let server = MockServer::start().await;
        let oversized = format!(
            r#"{{"status":400,"title":"x","detail":"{}"}}"#,
            "d".repeat(200)
        );
        Mock::given(method("GET"))
            .and(path("/err"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_raw(oversized.into_bytes(), "application/hal+json"),
            )
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/err", server.uri()))
            .send()
            .await
            .expect("send");
        let limits = ResponseLimits::default().with_max_error_body_bytes(64);
        let err = json::<serde_json::Value>(response, &[200], &[400], "key", limits)
            .await
            .expect_err("error body over limit");
        assert!(
            matches!(err, crate::Error::InvalidRequest(ref message) if message.contains("exceeds limit")),
            "unexpected error: {err:?}"
        );
    }
}
