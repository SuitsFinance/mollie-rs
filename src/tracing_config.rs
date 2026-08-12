//! Tracing subscriber setup for applications and examples.
//!
//! Available when the default `app-helpers` feature is enabled. Library
//! embeddings should usually disable that feature and install their own
//! `tracing-subscriber` (or equivalent) so the SDK does not own global process
//! logging configuration.
//!
//! The SDK emits structured events through the `tracing` crate. Call
//! [`init_tracing`] or [`try_init_tracing`] once at process startup so those
//! events are printed. Log level is controlled by `RUST_LOG` (see
//! [`EnvFilter`](tracing_subscriber::EnvFilter)).
//!
//! # Examples
//!
//! ```rust
//! use mollie_rs::try_init_tracing;
//!
//! let _installed = try_init_tracing();
//! ```
#![warn(missing_docs)]

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{MollieError, MollieResult};

/// Default filter when `RUST_LOG` is unset.
const DEFAULT_ENV_FILTER: &str = "info";

/// Installs a global `tracing-subscriber` fmt layer using `RUST_LOG`.
///
/// When `RUST_LOG` is unset, the filter defaults to `info`.
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when a global subscriber is
/// already installed or the filter directive cannot be parsed.
///
/// # Examples
///
/// ```rust,no_run
/// use mollie_rs::init_tracing;
///
/// # fn main() -> Result<(), mollie_rs::MollieError> {
/// init_tracing()?;
/// # Ok(())
/// # }
/// ```
pub fn init_tracing() -> MollieResult<()> {
    init_with_filter(default_env_filter()?)
}

/// Installs a global `tracing-subscriber` fmt layer with an explicit filter.
///
/// # Errors
///
/// Returns [`MollieError::InvalidConfiguration`] when a global subscriber is
/// already installed or `filter` cannot be parsed.
///
/// # Examples
///
/// ```rust,no_run
/// use mollie_rs::init_tracing_with_filter;
///
/// # fn main() -> Result<(), mollie_rs::MollieError> {
/// init_tracing_with_filter("mollie_rs=debug,info")?;
/// # Ok(())
/// # }
/// ```
pub fn init_tracing_with_filter(filter: impl AsRef<str>) -> MollieResult<()> {
    let filter: EnvFilter = EnvFilter::try_new(filter.as_ref()).map_err(|error| {
        MollieError::invalid_configuration(format!("invalid tracing filter: {error}"))
    })?;
    init_with_filter(filter)
}

/// Attempts to install the default tracing subscriber.
///
/// Returns `true` when this call installed the subscriber, and `false` when a
/// global subscriber was already present (or installation failed for a
/// non-fatal reason). Prefer this in examples and tests so double-init is
/// harmless.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::try_init_tracing;
///
/// let _ = try_init_tracing();
/// // A second call is safe and returns false once a subscriber is installed.
/// let _ = try_init_tracing();
/// ```
pub fn try_init_tracing() -> bool {
    init_tracing().is_ok()
}

/// Attempts to install a tracing subscriber with an explicit filter.
///
/// Returns `true` when this call installed the subscriber.
pub fn try_init_tracing_with_filter(filter: impl AsRef<str>) -> bool {
    init_tracing_with_filter(filter).is_ok()
}

fn default_env_filter() -> MollieResult<EnvFilter> {
    EnvFilter::try_from_default_env().or_else(|_| {
        EnvFilter::try_new(DEFAULT_ENV_FILTER).map_err(|error| {
            MollieError::invalid_configuration(format!("invalid default tracing filter: {error}"))
        })
    })
}

fn init_with_filter(filter: EnvFilter) -> MollieResult<()> {
    let result = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .try_init();

    match result {
        Ok(()) => {
            tracing::debug!("mollie-rs tracing subscriber initialized");
            Ok(())
        }
        Err(error) => Err(MollieError::invalid_configuration(format!(
            "failed to initialize tracing subscriber: {error}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_init_tracing_is_idempotent() {
        // First call may succeed or fail if another test already installed a
        // subscriber; either way a second call must not panic and returns false
        // once a global subscriber is present.
        let _ = try_init_tracing();
        assert!(!try_init_tracing());
    }

    #[test]
    fn init_tracing_with_filter_rejects_invalid_directive() {
        // Only asserts parse errors when init is attempted with a bad filter.
        // If a subscriber is already installed, we still get InvalidConfiguration.
        let error = init_tracing_with_filter("!!!not a filter!!!").unwrap_err();
        assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
    }
}
