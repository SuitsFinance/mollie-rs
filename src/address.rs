//! Validated common Mollie address values.
//!
//! Generated address models are intentionally permissive because the same
//! models are used by multiple routes. This facade enforces the common
//! address rules before converting into generated payment or simple address
//! models.
#![warn(missing_docs)]

use crate::{types, CountryCode, MollieError, MollieResult};

/// Countries for which Mollie documents that postal codes may be omitted.
pub const POSTAL_CODE_OPTIONAL_COUNTRIES: &[&str] = &[
    "AE", "AN", "AO", "AW", "BF", "BI", "BJ", "BO", "BS", "BV", "BW", "BZ", "CD", "CF", "CG", "CI",
    "CK", "CM", "DJ", "DM", "ER", "FJ", "GA", "GD", "GH", "GM", "GN", "GQ", "GY", "HK", "JM", "KE",
    "KI", "KM", "KN", "KP", "LC", "ML", "MO", "MR", "MS", "MU", "MW", "NA", "NR", "NU", "PA", "QA",
    "RW", "SB", "SC", "SL", "SO", "SR", "ST", "SY", "TF", "TK", "TL", "TO", "TT", "TV", "UG", "VU",
    "YE", "ZM", "ZW",
];

/// A validated common Mollie address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Address {
    /// The person’s given name, at least two characters and not numeric-only.
    pub given_name: String,
    /// The person’s family name, at least two characters and not numeric-only.
    pub family_name: String,
    /// Street and house number.
    pub street_and_number: String,
    /// Additional address information such as an apartment number.
    pub street_additional: Option<String>,
    /// Postal code, required for countries with a postal-code system.
    pub postal_code: Option<String>,
    /// City name.
    pub city: String,
    /// Optional administrative region.
    pub region: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    pub country: CountryCode,
}

impl Address {
    /// Creates an address with the common required fields.
    pub fn new(
        given_name: impl Into<String>,
        family_name: impl Into<String>,
        street_and_number: impl Into<String>,
        city: impl Into<String>,
        country: impl AsRef<str>,
    ) -> MollieResult<Self> {
        let address = Self {
            given_name: given_name.into(),
            family_name: family_name.into(),
            street_and_number: street_and_number.into(),
            street_additional: None,
            postal_code: None,
            city: city.into(),
            region: None,
            country: CountryCode::parse(country.as_ref())?,
        };
        address.validate_core()?;
        Ok(address)
    }

    /// Sets and validates an optional street addition.
    pub fn with_street_additional(mut self, value: impl Into<String>) -> MollieResult<Self> {
        self.street_additional = Some(non_empty("streetAdditional", value.into())?);
        Ok(self)
    }

    /// Sets and validates the postal code.
    pub fn with_postal_code(mut self, value: impl Into<String>) -> MollieResult<Self> {
        self.postal_code = Some(non_empty("postalCode", value.into())?);
        Ok(self)
    }

    /// Sets and validates the administrative region.
    pub fn with_region(mut self, value: impl Into<String>) -> MollieResult<Self> {
        self.region = Some(non_empty("region", value.into())?);
        Ok(self)
    }

    /// Validates all common address requirements, including postal-code rules.
    pub fn validate(&self) -> MollieResult<()> {
        self.validate_core()?;
        if self.postal_code.is_none()
            && !POSTAL_CODE_OPTIONAL_COUNTRIES.contains(&self.country.as_str())
        {
            return Err(MollieError::invalid_request(format!(
                "postalCode is required for country {}",
                self.country.as_str()
            )));
        }
        Ok(())
    }

    /// Converts the validated address to Mollie’s generated payment-address model.
    pub fn into_payment_address(self) -> MollieResult<types::PaymentAddress> {
        self.validate()?;
        Ok(types::PaymentAddress {
            city: Some(self.city),
            country: Some(self.country.to_string()),
            email: None,
            family_name: Some(self.family_name),
            given_name: Some(self.given_name),
            organization_name: None,
            phone: None,
            postal_code: self.postal_code,
            region: self.region,
            street_additional: self.street_additional,
            street_and_number: Some(self.street_and_number),
            title: None,
        })
    }

    /// Converts the validated address to Mollie’s generated simple address model.
    pub fn into_address(self) -> MollieResult<types::Address> {
        self.validate()?;
        Ok(types::Address {
            city: self.city,
            country: self.country.to_string(),
            postal_code: self.postal_code.unwrap_or_default(),
            street_and_number: self.street_and_number,
        })
    }

    /// Validates the fields required for every address shape.
    fn validate_core(&self) -> MollieResult<()> {
        validate_person_name("givenName", &self.given_name)?;
        validate_person_name("familyName", &self.family_name)?;
        non_empty("streetAndNumber", self.street_and_number.clone())?;
        non_empty("city", self.city.clone())?;
        Ok(())
    }
}

/// Validates a name according to Mollie’s minimum-length and numeric rules.
fn validate_person_name(field: &str, value: &str) -> MollieResult<()> {
    let value = value.trim();
    if value.chars().count() < 2 || value.chars().all(|character| character.is_ascii_digit()) {
        return Err(MollieError::invalid_request(format!(
            "{field} must contain at least two characters and cannot contain only numbers"
        )));
    }
    Ok(())
}

/// Requires a non-empty common address string.
fn non_empty(field: &str, value: String) -> MollieResult<String> {
    if value.trim().is_empty() {
        return Err(MollieError::invalid_request(format!(
            "{field} cannot be empty"
        )));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::Address;

    /// Builds a complete address for a country with postal codes.
    #[test]
    fn validates_and_converts_complete_address() {
        let address = Address::new("Floris", "Xylex", "Main Street 1", "Amsterdam", "NL")
            .expect("valid address")
            .with_postal_code("1012AB")
            .expect("valid postal code");
        let generated = address
            .into_payment_address()
            .expect("valid payment address");

        assert_eq!(generated.country.as_deref(), Some("NL"));
        assert_eq!(generated.postal_code.as_deref(), Some("1012AB"));
    }

    /// Allows omitted postal codes for documented countries such as the UAE.
    #[test]
    fn allows_postal_code_omission_for_supported_country() {
        let address =
            Address::new("Ali", "Khan", "Street 1", "Dubai", "AE").expect("valid address");
        assert!(address.validate().is_ok());
    }

    /// Rejects invalid names and missing postal codes.
    #[test]
    fn rejects_invalid_common_address_values() {
        assert!(Address::new("A", "123", "Street 1", "Amsterdam", "NL").is_err());
        assert!(
            Address::new("Anne", "Example", "Street 1", "Amsterdam", "NL")
                .expect("core fields")
                .validate()
                .is_err()
        );
    }
}
