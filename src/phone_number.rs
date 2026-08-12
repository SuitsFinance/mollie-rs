//! E.164 phone number validation for Mollie request fields.
//!
//! Mollie requires phone numbers as strings in
//! [E.164](https://en.wikipedia.org/wiki/E.164) form, for example
//! `+31208202070`. Generated OpenAPI types keep these fields as plain
//! `String`; this module validates before you assign them.
//!
//! See `docs/e.164.md`.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use crate::{MollieError, MollieResult};

/// A phone number in E.164 format.
///
/// Wire form: a leading `+`, then 1–15 digits, with the first digit in `1..=9`
/// (no leading zero on the country code). Example: `+31208202070`.
///
/// Spaces, dashes, parentheses, and national-only forms without `+` are
/// rejected.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Maximum number of digits after the leading `+` (E.164).
    pub const MAX_DIGITS: usize = 15;

    /// Parses and validates an E.164 phone number string.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is not a valid
    /// E.164 number (missing `+`, non-digits, leading zero after `+`, too
    /// short/long, or empty).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PhoneNumber;
    ///
    /// let phone = PhoneNumber::parse("+31208202070")?;
    /// assert_eq!(phone.as_str(), "+31208202070");
    /// assert!(PhoneNumber::parse("0208202070").is_err());
    /// assert!(PhoneNumber::parse("+31 20 820 2070").is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        validate_e164(&value)?;
        Ok(Self(value))
    }

    /// Returns the validated E.164 string (including the leading `+`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PhoneNumber;
    ///
    /// assert_eq!(PhoneNumber::parse("+31208202070")?.as_str(), "+31208202070");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned E.164 string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns true when `value` is a valid E.164 phone number string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PhoneNumber;
    ///
    /// assert!(PhoneNumber::is_valid("+31208202070"));
    /// assert!(!PhoneNumber::is_valid("31208202070"));
    /// ```
    pub fn is_valid(value: impl AsRef<str>) -> bool {
        validate_e164(value.as_ref()).is_ok()
    }

    /// Digits only (without the leading `+`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PhoneNumber;
    ///
    /// assert_eq!(PhoneNumber::parse("+31208202070")?.digits(), "31208202070");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn digits(&self) -> &str {
        &self.0[1..]
    }
}

/// Validates E.164: `+` + 1–15 digits, first digit 1–9.
fn validate_e164(value: &str) -> MollieResult<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return Err(MollieError::invalid_request(
            "phone number cannot be empty; use E.164 form such as `+31208202070`",
        ));
    }
    if bytes[0] != b'+' {
        return Err(MollieError::invalid_request(format!(
            "invalid phone number `{value}`: E.164 requires a leading `+` (example: `+31208202070`)"
        )));
    }
    let digits = &bytes[1..];
    if digits.is_empty() {
        return Err(MollieError::invalid_request(
            "invalid phone number `+`: E.164 requires 1–15 digits after `+`",
        ));
    }
    if digits.len() > PhoneNumber::MAX_DIGITS {
        return Err(MollieError::invalid_request(format!(
            "invalid phone number `{value}`: E.164 allows at most {} digits after `+`",
            PhoneNumber::MAX_DIGITS
        )));
    }
    if !(b'1'..=b'9').contains(&digits[0]) {
        return Err(MollieError::invalid_request(format!(
            "invalid phone number `{value}`: E.164 country code must not start with `0`"
        )));
    }
    if !digits.iter().all(|b| b.is_ascii_digit()) {
        return Err(MollieError::invalid_request(format!(
            "invalid phone number `{value}`: E.164 allows only digits after `+` (no spaces or punctuation)"
        )));
    }
    Ok(())
}

impl FromStr for PhoneNumber {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for PhoneNumber {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PhoneNumber> for String {
    fn from(value: PhoneNumber) -> Self {
        value.into_string()
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneNumber;

    #[test]
    fn accepts_mollie_example() {
        let phone = PhoneNumber::parse("+31208202070").unwrap();
        assert_eq!(phone.as_str(), "+31208202070");
        assert_eq!(phone.digits(), "31208202070");
        assert!(PhoneNumber::is_valid("+12025550123"));
    }

    #[test]
    fn rejects_invalid_forms() {
        assert!(PhoneNumber::parse("").is_err());
        assert!(PhoneNumber::parse("31208202070").is_err());
        assert!(PhoneNumber::parse("+").is_err());
        assert!(PhoneNumber::parse("+031208202070").is_err());
        assert!(PhoneNumber::parse("+31 20 820 2070").is_err());
        assert!(PhoneNumber::parse("+31-20-8202070").is_err());
        assert!(PhoneNumber::parse(format!("+{}", "1".repeat(16))).is_err());
        assert!(PhoneNumber::parse(format!("+{}", "1".repeat(15))).is_ok());
    }
}
