//! Environment helpers for process variables and optional dotenv loading.
//!
//! Prefer [`MollieClient::from_env`](crate::MollieClient::from_env),
//! [`ApiKey::from_env`](crate::ApiKey::from_env), or
//! [`OAuthAccessToken::from_env`](crate::OAuthAccessToken::from_env), which
//! load `.env` automatically when the default `app-helpers` feature is enabled.
//! Use [`load_dotenv`] only when you need dotenv before reading other variables
//! yourself.
//!
//! Disable `app-helpers` in library embeddings that must not touch `.env` files;
//! then [`load_dotenv`] is a no-op and only process environment variables apply.
//!
//! # Examples
//!
//! ```rust,no_run
//! use mollie_rs::MollieClient;
//!
//! # fn main() -> Result<(), mollie_rs::MollieError> {
//! let _client = MollieClient::from_env()?;
//! # Ok(())
//! # }
//! ```
#![warn(missing_docs)]

use std::{env, ffi::OsString, path::Path};

use crate::{MollieError, MollieResult};

/// Process environment variable for a Mollie API key.
pub const MOLLIE_API_KEY_ENV: &str = "MOLLIE_API_KEY";

/// Process environment variable for a Mollie OAuth access token.
pub const MOLLIE_OAUTH_ACCESS_TOKEN_ENV: &str = "MOLLIE_OAUTH_ACCESS_TOKEN";

/// Process environment variable for the Mollie OAuth client ID used by Basic Auth.
pub const MOLLIE_OAUTH_CLIENT_ID_ENV: &str = "MOLLIE_OAUTH_CLIENT_ID";

/// Process environment variable for the Mollie OAuth client secret used by Basic Auth.
pub const MOLLIE_OAUTH_CLIENT_SECRET_ENV: &str = "MOLLIE_OAUTH_CLIENT_SECRET";

/// Process environment variable for an optional Mollie API base URL override.
pub const MOLLIE_BASE_URL_ENV: &str = "MOLLIE_BASE_URL";

/// Loads variables from a `.env` file in the current directory into the process
/// environment.
///
/// A missing `.env` file is treated as success so local checkouts without a
/// file keep working. Existing process variables are not overwritten
/// (`dotenvy` default).
///
/// When the `app-helpers` feature is disabled, this is a no-op success so
/// library embeddings never read `.env` files.
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when `.env` exists but cannot
/// be read or parsed (only with `app-helpers`).
///
/// # Examples
///
/// ```rust
/// use mollie_rs::load_dotenv;
///
/// load_dotenv().expect("missing .env is ok");
/// ```
pub fn load_dotenv() -> MollieResult<()> {
    #[cfg(feature = "app-helpers")]
    {
        match dotenvy::dotenv() {
            Ok(_) => Ok(()),
            Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(())
            }
            Err(error) => Err(map_dotenv_error(error)),
        }
    }
    #[cfg(not(feature = "app-helpers"))]
    {
        Ok(())
    }
}

/// Loads variables from the given `.env` path into the process environment.
///
/// Unlike [`load_dotenv`], a missing file is an error.
///
/// Requires the `app-helpers` feature (enabled by default).
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when the file cannot be read
/// or parsed, or when `app-helpers` is disabled.
///
/// # Examples
///
/// ```rust,no_run
/// use mollie_rs::load_dotenv_from;
///
/// # fn main() -> Result<(), mollie_rs::MollieError> {
/// load_dotenv_from(".env")?;
/// # Ok(())
/// # }
/// ```
pub fn load_dotenv_from(path: impl AsRef<Path>) -> MollieResult<()> {
    #[cfg(feature = "app-helpers")]
    {
        dotenvy::from_path(path.as_ref()).map_err(map_dotenv_error)?;
        Ok(())
    }
    #[cfg(not(feature = "app-helpers"))]
    {
        let _ = path;
        Err(MollieError::invalid_configuration(
            "load_dotenv_from requires the `app-helpers` cargo feature",
        ))
    }
}

/// Reads a required process environment variable as UTF-8.
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when the variable is missing
/// or not valid Unicode.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::env::{var, MOLLIE_API_KEY_ENV};
///
/// let _ = var(MOLLIE_API_KEY_ENV);
/// ```
pub fn var(key: impl AsRef<str>) -> MollieResult<String> {
    let key = key.as_ref();
    match env::var(key) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Err(MollieError::missing_env_var(key)),
        Err(env::VarError::NotUnicode(_)) => Err(MollieError::invalid_env_var_encoding(key)),
    }
}

/// Reads an optional process environment variable as UTF-8.
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when the variable is present
/// but not valid Unicode.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::env::{var_optional, MOLLIE_BASE_URL_ENV};
///
/// let _ = var_optional(MOLLIE_BASE_URL_ENV).expect("unicode only");
/// ```
pub fn var_optional(key: impl AsRef<str>) -> MollieResult<Option<String>> {
    let key = key.as_ref();
    match env::var(key) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(MollieError::invalid_env_var_encoding(key)),
    }
}

/// Reads a required process environment variable as an [`OsString`].
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when the variable is missing.
pub fn var_os(key: impl AsRef<str>) -> MollieResult<OsString> {
    let key = key.as_ref();
    env::var_os(key).ok_or_else(|| MollieError::missing_env_var(key))
}

#[cfg(feature = "app-helpers")]
fn map_dotenv_error(error: dotenvy::Error) -> MollieError {
    MollieError::invalid_configuration(format!("failed to load dotenv: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_dotenv_accepts_missing_file() {
        // Running from the crate root may find a real .env; either way the call
        // must not fail for the default missing-file path when cwd has none.
        // We only assert the NotFound mapping via load when we control cwd.
        let result = load_dotenv();
        assert!(result.is_ok(), "load_dotenv should succeed: {result:?}");
    }

    #[test]
    fn var_optional_returns_none_when_missing() {
        let unique = format!("MOLLIE_RS_MISSING_{}", std::process::id());
        std::env::remove_var(&unique);
        assert_eq!(var_optional(&unique).expect("unicode"), None);
    }

    #[cfg(feature = "app-helpers")]
    mod with_app_helpers {
        use super::*;
        use std::io::Write;

        #[test]
        fn load_dotenv_from_missing_path_is_error() {
            let error = load_dotenv_from("definitely-missing-mollie-dotenv-file.env").unwrap_err();
            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn load_dotenv_from_valid_file_succeeds() {
            let dir =
                std::env::temp_dir().join(format!("mollie-rs-dotenv-test-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("temp dir");
            let path = dir.join(".env");
            {
                let mut file = std::fs::File::create(&path).expect("create env file");
                writeln!(file, "MOLLIE_DOTENV_TEST_MARKER=1").expect("write env file");
            }

            load_dotenv_from(&path).expect("dotenv file should load");
            assert_eq!(
                std::env::var("MOLLIE_DOTENV_TEST_MARKER").ok().as_deref(),
                Some("1")
            );

            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    #[cfg(not(feature = "app-helpers"))]
    #[test]
    fn load_dotenv_from_requires_app_helpers_feature() {
        let error = load_dotenv_from(".env").unwrap_err();
        assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        assert!(error.to_string().contains("app-helpers"));
    }
}
