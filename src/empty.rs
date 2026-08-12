//! Explicit empty provider responses (204 / empty JSON bodies).
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Marker for successful responses with no meaningful body.
///
/// Prefer this over `serde_json::Value` for cancel/revoke-style routes so callers
/// do not parse JSON nulls or empty objects.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct EmptyResponse;

impl EmptyResponse {
    /// Constructs the unit success marker.
    pub const fn new() -> Self {
        Self
    }
}
