use mollie_rs::prelude::ResponseEnvelope;
use mollie_rs::{
    try_init_tracing, GeneratedMollieResult, IntoMollieResult, MollieError, ResponseValue,
};
use reqwest::StatusCode;

fn main() -> Result<(), MollieError> {
    let _ = try_init_tracing();

    let generated: GeneratedMollieResult<&str> =
        Ok(ResponseValue::new("ok", StatusCode::OK, Default::default()));

    let envelope: ResponseEnvelope<&str> = generated.into_mollie_result()?;
    tracing::info!(
        status = %envelope.status(),
        data = ?envelope.data(),
        "converted Mollie response envelope"
    );

    Ok(())
}
