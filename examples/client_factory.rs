use mollie_rs::{auth::Credential, try_init_tracing, MollieClient, MollieError};

fn main() -> Result<(), MollieError> {
    let _ = try_init_tracing();

    let client: MollieClient = match MollieClient::from_env() {
        Ok(client) => {
            tracing::info!(
                base_url = %client.raw().baseurl(),
                "configured Mollie client from environment"
            );
            client
        }
        Err(error) => {
            tracing::error!(
                error = %error,
                "failed to load Mollie client from environment; using demo credential"
            );
            MollieClient::builder()
                .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
                .build()?
        }
    };

    tracing::info!(base_url = %client.raw().baseurl(), "configured Mollie client");
    Ok(())
}
