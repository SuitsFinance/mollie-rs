//! Typed locale helpers for Mollie request payloads.
//!
//! Mollie accepts ISO 15897 locales (`language_TERRITORY`, e.g. `en_US`,
//! `nl_NL`). Hosted payment pages document a preferred set of languages; any
//! well-formed `xx_XX` value is still valid per Mollie docs (unsupported UI
//! languages fall back to the browser language when applicable).
//!
//! Generated wire types remain in [`crate::types`] ([`types::Locale`] /
//! [`types::LocaleInner`] and route-specific locale enums). This module is the
//! application-facing enum and validator.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use crate::{types, MollieError, MollieResult};

/// A Mollie locale in ISO 15897 form (`language_TERRITORY`).
///
/// Mollie documents **possible values** as the set in [`Self::POSSIBLE`]
/// (`en_US`, `nl_NL`, …). Named variants cover that set plus a few extra
/// convenience locales; [`Self::parse`] also accepts any other well-formed
/// `xx_XX` value as [`Self::Other`].
///
/// Conversion into generated OpenAPI enums succeeds only for values present on
/// those generated types (see [`Self::into_generated`]).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Locale {
    /// Catalan (Spain) — `ca_ES`.
    CaEs,
    /// Czech (Czechia) — `cs_CZ` (valid `xx_XX`; not in Mollie possible-values list).
    CsCz,
    /// Danish (Denmark) — `da_DK`.
    DaDk,
    /// German (Austria) — `de_AT`.
    DeAt,
    /// German (Switzerland) — `de_CH`.
    DeCh,
    /// German (Germany) — `de_DE`.
    DeDe,
    /// German (Luxembourg) — `de_LU` (valid `xx_XX`; not in Mollie possible-values list).
    DeLu,
    /// English (Belgium) — `en_BE`.
    EnBe,
    /// English (United Kingdom) — `en_GB`.
    EnGb,
    /// English (Netherlands) — `en_NL`.
    EnNl,
    /// English (United States) — `en_US`.
    EnUs,
    /// Spanish (Spain) — `es_ES`.
    EsEs,
    /// Finnish (Finland) — `fi_FI`.
    FiFi,
    /// French (Belgium) — `fr_BE`.
    FrBe,
    /// French (France) — `fr_FR`.
    FrFr,
    /// French (Luxembourg) — `fr_LU` (valid `xx_XX`; not in Mollie possible-values list).
    FrLu,
    /// Hungarian (Hungary) — `hu_HU`.
    HuHu,
    /// Icelandic (Iceland) — `is_IS`.
    IsIs,
    /// Italian (Italy) — `it_IT`.
    ItIt,
    /// Lithuanian (Lithuania) — `lt_LT`.
    LtLt,
    /// Latvian (Latvia) — `lv_LV`.
    LvLv,
    /// Norwegian Bokmål (Norway) — `nb_NO`.
    NbNo,
    /// Dutch (Belgium) — `nl_BE`.
    NlBe,
    /// Dutch (Netherlands) — `nl_NL`.
    NlNl,
    /// Polish (Poland) — `pl_PL`.
    PlPl,
    /// Portuguese (Portugal) — `pt_PT`.
    PtPt,
    /// Slovak (Slovakia) — `sk_SK` (valid `xx_XX`; not in Mollie possible-values list).
    SkSk,
    /// Swedish (Sweden) — `sv_SE`.
    SvSe,
    /// Any other well-formed ISO 15897 `xx_XX` locale (five ASCII bytes).
    Other([u8; 5]),
}

impl Locale {
    /// Catalan (Spain).
    pub const CA_ES: Self = Self::CaEs;
    /// Czech (Czechia).
    pub const CS_CZ: Self = Self::CsCz;
    /// Danish (Denmark).
    pub const DA_DK: Self = Self::DaDk;
    /// German (Austria).
    pub const DE_AT: Self = Self::DeAt;
    /// German (Switzerland).
    pub const DE_CH: Self = Self::DeCh;
    /// German (Germany).
    pub const DE_DE: Self = Self::DeDe;
    /// German (Luxembourg).
    pub const DE_LU: Self = Self::DeLu;
    /// English (Belgium).
    pub const EN_BE: Self = Self::EnBe;
    /// English (United Kingdom).
    pub const EN_GB: Self = Self::EnGb;
    /// English (Netherlands).
    pub const EN_NL: Self = Self::EnNl;
    /// English (United States).
    pub const EN_US: Self = Self::EnUs;
    /// Spanish (Spain).
    pub const ES_ES: Self = Self::EsEs;
    /// Finnish (Finland).
    pub const FI_FI: Self = Self::FiFi;
    /// French (Belgium).
    pub const FR_BE: Self = Self::FrBe;
    /// French (France).
    pub const FR_FR: Self = Self::FrFr;
    /// French (Luxembourg).
    pub const FR_LU: Self = Self::FrLu;
    /// Hungarian (Hungary).
    pub const HU_HU: Self = Self::HuHu;
    /// Icelandic (Iceland).
    pub const IS_IS: Self = Self::IsIs;
    /// Italian (Italy).
    pub const IT_IT: Self = Self::ItIt;
    /// Lithuanian (Lithuania).
    pub const LT_LT: Self = Self::LtLt;
    /// Latvian (Latvia).
    pub const LV_LV: Self = Self::LvLv;
    /// Norwegian Bokmål (Norway).
    pub const NB_NO: Self = Self::NbNo;
    /// Dutch (Belgium).
    pub const NL_BE: Self = Self::NlBe;
    /// Dutch (Netherlands).
    pub const NL_NL: Self = Self::NlNl;
    /// Polish (Poland).
    pub const PL_PL: Self = Self::PlPl;
    /// Portuguese (Portugal).
    pub const PT_PT: Self = Self::PtPt;
    /// Slovak (Slovakia).
    pub const SK_SK: Self = Self::SkSk;
    /// Swedish (Sweden).
    pub const SV_SE: Self = Self::SvSe;

    /// Mollie-documented possible locale values (ISO 15897 `xx_XX`).
    ///
    /// Order matches common API docs: `en_US`, `en_GB`, `nl_NL`, …
    pub const POSSIBLE: [Self; 24] = [
        Self::EnUs,
        Self::EnGb,
        Self::EnBe,
        Self::EnNl,
        Self::NlNl,
        Self::NlBe,
        Self::FrFr,
        Self::FrBe,
        Self::DeDe,
        Self::DeAt,
        Self::DeCh,
        Self::EsEs,
        Self::CaEs,
        Self::PtPt,
        Self::ItIt,
        Self::NbNo,
        Self::SvSe,
        Self::FiFi,
        Self::DaDk,
        Self::IsIs,
        Self::HuHu,
        Self::PlPl,
        Self::LvLv,
        Self::LtLt,
    ];

    /// Alias for [`Self::POSSIBLE`] (hosted payment-page languages).
    pub const HOSTED: [Self; 24] = Self::POSSIBLE;

    /// Parses a locale string.
    ///
    /// Accepts every value in [`Self::POSSIBLE`] and any other well-formed
    /// ISO 15897 `xx_XX` value. Comparison is case-sensitive for the wire form
    /// (`en_US`, not `en-US` or `EN_US`).
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is not a
    /// five-character `ll_RR` locale (`[a-z]{2}_[A-Z]{2}`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Locale;
    ///
    /// assert_eq!(Locale::parse("nl_NL")?, Locale::NL_NL);
    /// assert!(Locale::parse("ja_JP")?.is_other());
    /// assert!(Locale::parse("en-US").is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        value.as_ref().parse()
    }

    /// Returns the wire locale string (`en_US`, `nl_NL`, …).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Locale;
    ///
    /// assert_eq!(Locale::EN_US.as_str(), "en_US");
    /// assert_eq!(Locale::parse("ja_JP")?.as_str(), "ja_JP");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CaEs => "ca_ES",
            Self::CsCz => "cs_CZ",
            Self::DaDk => "da_DK",
            Self::DeAt => "de_AT",
            Self::DeCh => "de_CH",
            Self::DeDe => "de_DE",
            Self::DeLu => "de_LU",
            Self::EnBe => "en_BE",
            Self::EnGb => "en_GB",
            Self::EnNl => "en_NL",
            Self::EnUs => "en_US",
            Self::EsEs => "es_ES",
            Self::FiFi => "fi_FI",
            Self::FrBe => "fr_BE",
            Self::FrFr => "fr_FR",
            Self::FrLu => "fr_LU",
            Self::HuHu => "hu_HU",
            Self::IsIs => "is_IS",
            Self::ItIt => "it_IT",
            Self::LtLt => "lt_LT",
            Self::LvLv => "lv_LV",
            Self::NbNo => "nb_NO",
            Self::NlBe => "nl_BE",
            Self::NlNl => "nl_NL",
            Self::PlPl => "pl_PL",
            Self::PtPt => "pt_PT",
            Self::SkSk => "sk_SK",
            Self::SvSe => "sv_SE",
            Self::Other(bytes) => {
                std::str::from_utf8(bytes).expect("Other locales are validated as ASCII xx_XX")
            }
        }
    }

    /// Returns true when this locale is in Mollie's documented possible set
    /// ([`Self::POSSIBLE`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Locale;
    ///
    /// assert!(Locale::NL_NL.is_possible());
    /// assert!(Locale::NL_NL.is_hosted());
    /// assert!(!Locale::parse("ja_JP")?.is_possible());
    /// assert!(!Locale::CS_CZ.is_possible());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn is_possible(self) -> bool {
        matches!(
            self,
            Self::EnUs
                | Self::EnGb
                | Self::EnBe
                | Self::EnNl
                | Self::NlNl
                | Self::NlBe
                | Self::FrFr
                | Self::FrBe
                | Self::DeDe
                | Self::DeAt
                | Self::DeCh
                | Self::EsEs
                | Self::CaEs
                | Self::PtPt
                | Self::ItIt
                | Self::NbNo
                | Self::SvSe
                | Self::FiFi
                | Self::DaDk
                | Self::IsIs
                | Self::HuHu
                | Self::PlPl
                | Self::LvLv
                | Self::LtLt
        )
    }

    /// Alias for [`Self::is_possible`].
    pub const fn is_hosted(self) -> bool {
        self.is_possible()
    }

    /// Returns true when this value was parsed as a non-named `xx_XX` locale.
    pub const fn is_other(self) -> bool {
        matches!(self, Self::Other(_))
    }

    /// Returns true when `value` is a well-formed ISO 15897 `xx_XX` locale
    /// (hosted or other).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Locale;
    ///
    /// assert!(Locale::is_valid_format("en_US"));
    /// assert!(Locale::is_valid_format("ja_JP"));
    /// assert!(!Locale::is_valid_format("en-US"));
    /// ```
    pub fn is_valid_format(value: impl AsRef<str>) -> bool {
        is_iso_15897_xx_xx(value.as_ref())
    }

    /// Returns true when `value` is one of Mollie's documented possible locales.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Locale;
    ///
    /// assert!(Locale::is_possible_value("nl_NL"));
    /// assert!(Locale::is_hosted_value("nl_NL"));
    /// assert!(!Locale::is_possible_value("ja_JP"));
    /// ```
    pub fn is_possible_value(value: impl AsRef<str>) -> bool {
        matches!(Self::parse(value), Ok(locale) if locale.is_possible())
    }

    /// Alias for [`Self::is_possible_value`].
    pub fn is_hosted_value(value: impl AsRef<str>) -> bool {
        Self::is_possible_value(value)
    }

    /// Converts into the generated nullable [`types::Locale`] used on payment
    /// bodies, when the locale is present on the generated OpenAPI enum.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the locale is not one of
    /// the generated [`types::LocaleInner`] variants (for example hosted
    /// `cs_CZ` / `sk_SK` / `de_LU` / `fr_LU` until the checked-in spec gains
    /// them, or any [`Self::Other`] value).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, Locale};
    ///
    /// let locale: types::Locale = Locale::EN_US.into_generated()?;
    /// assert_eq!(locale.0, Some(types::LocaleInner::EnUs));
    /// assert!(Locale::parse("ja_JP")?.into_generated().is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn into_generated(self) -> MollieResult<types::Locale> {
        Ok(types::Locale(Some(self.try_into_inner()?)))
    }

    /// Converts into the generated [`types::LocaleInner`] when supported by the
    /// checked-in OpenAPI enum.
    ///
    /// # Errors
    ///
    /// Same conditions as [`Self::into_generated`].
    pub fn try_into_inner(self) -> MollieResult<types::LocaleInner> {
        match self {
            Self::EnUs => Ok(types::LocaleInner::EnUs),
            Self::EnGb => Ok(types::LocaleInner::EnGb),
            Self::EnBe => Ok(types::LocaleInner::EnBe),
            Self::EnNl => Ok(types::LocaleInner::EnNl),
            Self::NlNl => Ok(types::LocaleInner::NlNl),
            Self::NlBe => Ok(types::LocaleInner::NlBe),
            Self::DeDe => Ok(types::LocaleInner::DeDe),
            Self::DeAt => Ok(types::LocaleInner::DeAt),
            Self::DeCh => Ok(types::LocaleInner::DeCh),
            Self::DeLu => Ok(types::LocaleInner::DeLu),
            Self::FrFr => Ok(types::LocaleInner::FrFr),
            Self::FrBe => Ok(types::LocaleInner::FrBe),
            Self::FrLu => Ok(types::LocaleInner::FrLu),
            Self::EsEs => Ok(types::LocaleInner::EsEs),
            Self::CaEs => Ok(types::LocaleInner::CaEs),
            Self::PtPt => Ok(types::LocaleInner::PtPt),
            Self::ItIt => Ok(types::LocaleInner::ItIt),
            Self::NbNo => Ok(types::LocaleInner::NbNo),
            Self::SvSe => Ok(types::LocaleInner::SvSe),
            Self::FiFi => Ok(types::LocaleInner::FiFi),
            Self::DaDk => Ok(types::LocaleInner::DaDk),
            Self::IsIs => Ok(types::LocaleInner::IsIs),
            Self::HuHu => Ok(types::LocaleInner::HuHu),
            Self::PlPl => Ok(types::LocaleInner::PlPl),
            Self::LvLv => Ok(types::LocaleInner::LvLv),
            Self::LtLt => Ok(types::LocaleInner::LtLt),
            Self::CsCz => Ok(types::LocaleInner::CsCz),
            Self::SkSk => Ok(types::LocaleInner::SkSk),
            other => Err(MollieError::invalid_request(format!(
                "locale `{}` is a valid ISO 15897 value but is not present on the generated OpenAPI Locale enum; prefer a hosted locale from the checked-in spec or regenerate types when the Mollie OpenAPI schema adds it",
                other.as_str()
            ))),
        }
    }

    /// Converts into generated list-methods query locales when supported.
    ///
    /// After the 0.7 OpenAPI re-pin, method list endpoints use the shared
    /// [`types::Locale`] query type.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the locale is not on the
    /// generated OpenAPI locale enum.
    pub fn into_list_methods_locale(self) -> MollieResult<types::Locale> {
        self.into_generated()
    }

    /// Converts into generated list-all-methods query locales when supported.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the locale is not on the
    /// generated OpenAPI locale enum.
    pub fn into_list_all_methods_locale(self) -> MollieResult<types::Locale> {
        self.into_generated()
    }

    /// Converts into generated get-method query locales when supported.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the locale is not on the
    /// generated OpenAPI locale enum.
    pub fn into_get_method_locale(self) -> MollieResult<types::Locale> {
        self.into_generated()
    }
}

fn is_iso_15897_xx_xx(value: &str) -> bool {
    let b = value.as_bytes();
    b.len() == 5
        && b[0].is_ascii_lowercase()
        && b[1].is_ascii_lowercase()
        && b[2] == b'_'
        && b[3].is_ascii_uppercase()
        && b[4].is_ascii_uppercase()
}

fn parse_other(value: &str) -> MollieResult<Locale> {
    if !is_iso_15897_xx_xx(value) {
        return Err(MollieError::invalid_request(format!(
            "invalid Mollie locale `{value}`: expected ISO 15897 form `xx_XX` (two lowercase language letters, underscore, two uppercase region letters)"
        )));
    }
    let bytes = value.as_bytes();
    Ok(Locale::Other([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
    ]))
}

impl FromStr for Locale {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ca_ES" => Ok(Self::CaEs),
            "cs_CZ" => Ok(Self::CsCz),
            "da_DK" => Ok(Self::DaDk),
            "de_AT" => Ok(Self::DeAt),
            "de_CH" => Ok(Self::DeCh),
            "de_DE" => Ok(Self::DeDe),
            "de_LU" => Ok(Self::DeLu),
            "en_BE" => Ok(Self::EnBe),
            "en_GB" => Ok(Self::EnGb),
            "en_NL" => Ok(Self::EnNl),
            "en_US" => Ok(Self::EnUs),
            "es_ES" => Ok(Self::EsEs),
            "fi_FI" => Ok(Self::FiFi),
            "fr_BE" => Ok(Self::FrBe),
            "fr_FR" => Ok(Self::FrFr),
            "fr_LU" => Ok(Self::FrLu),
            "hu_HU" => Ok(Self::HuHu),
            "is_IS" => Ok(Self::IsIs),
            "it_IT" => Ok(Self::ItIt),
            "lt_LT" => Ok(Self::LtLt),
            "lv_LV" => Ok(Self::LvLv),
            "nb_NO" => Ok(Self::NbNo),
            "nl_BE" => Ok(Self::NlBe),
            "nl_NL" => Ok(Self::NlNl),
            "pl_PL" => Ok(Self::PlPl),
            "pt_PT" => Ok(Self::PtPt),
            "sk_SK" => Ok(Self::SkSk),
            "sv_SE" => Ok(Self::SvSe),
            other => parse_other(other),
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for Locale {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Locale {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<types::LocaleInner> for Locale {
    fn from(value: types::LocaleInner) -> Self {
        match value {
            types::LocaleInner::EnUs => Self::EnUs,
            types::LocaleInner::EnGb => Self::EnGb,
            types::LocaleInner::EnBe => Self::EnBe,
            types::LocaleInner::EnNl => Self::EnNl,
            types::LocaleInner::NlNl => Self::NlNl,
            types::LocaleInner::NlBe => Self::NlBe,
            types::LocaleInner::DeDe => Self::DeDe,
            types::LocaleInner::DeAt => Self::DeAt,
            types::LocaleInner::DeCh => Self::DeCh,
            types::LocaleInner::DeLu => Self::DeLu,
            types::LocaleInner::FrFr => Self::FrFr,
            types::LocaleInner::FrBe => Self::FrBe,
            types::LocaleInner::FrLu => Self::FrLu,
            types::LocaleInner::EsEs => Self::EsEs,
            types::LocaleInner::CaEs => Self::CaEs,
            types::LocaleInner::PtPt => Self::PtPt,
            types::LocaleInner::ItIt => Self::ItIt,
            types::LocaleInner::CsCz => Self::CsCz,
            types::LocaleInner::SkSk => Self::SkSk,
            types::LocaleInner::NbNo => Self::NbNo,
            types::LocaleInner::SvSe => Self::SvSe,
            types::LocaleInner::FiFi => Self::FiFi,
            types::LocaleInner::DaDk => Self::DaDk,
            types::LocaleInner::IsIs => Self::IsIs,
            types::LocaleInner::HuHu => Self::HuHu,
            types::LocaleInner::PlPl => Self::PlPl,
            types::LocaleInner::LvLv => Self::LvLv,
            types::LocaleInner::LtLt => Self::LtLt,
            types::LocaleInner::Null => Self::EnUs,
        }
    }
}

impl TryFrom<Locale> for types::LocaleInner {
    type Error = MollieError;

    fn try_from(value: Locale) -> Result<Self, Self::Error> {
        value.try_into_inner()
    }
}

impl TryFrom<Locale> for types::Locale {
    type Error = MollieError;

    fn try_from(value: Locale) -> Result<Self, Self::Error> {
        value.into_generated()
    }
}

impl TryFrom<Locale> for Option<types::Locale> {
    type Error = MollieError;

    fn try_from(value: Locale) -> Result<Self, Self::Error> {
        Ok(Some(value.into_generated()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{is_iso_15897_xx_xx, Locale};
    use crate::types;

    #[test]
    fn parse_hosted_and_other_iso_locales() {
        assert_eq!(Locale::parse("nl_NL").unwrap(), Locale::NlNl);
        assert_eq!(Locale::parse("en_US").unwrap(), Locale::EnUs);
        assert_eq!(Locale::parse("en_BE").unwrap(), Locale::EnBe);
        assert_eq!(Locale::parse("en_NL").unwrap(), Locale::EnNl);
        assert!(Locale::EnBe.is_possible());
        assert!(Locale::EnNl.is_possible());
        assert_eq!(
            Locale::EnBe.into_generated().unwrap().0,
            Some(types::LocaleInner::EnBe)
        );
        assert_eq!(
            Locale::EnNl.into_generated().unwrap().0,
            Some(types::LocaleInner::EnNl)
        );
        assert_eq!(Locale::parse("cs_CZ").unwrap(), Locale::CsCz);
        assert_eq!(Locale::parse("de_LU").unwrap(), Locale::DeLu);
        assert_eq!(Locale::parse("fr_LU").unwrap(), Locale::FrLu);
        assert_eq!(Locale::parse("sk_SK").unwrap(), Locale::SkSk);

        let other = Locale::parse("ja_JP").unwrap();
        assert!(other.is_other());
        assert!(!other.is_hosted());
        assert_eq!(other.as_str(), "ja_JP");
        assert_eq!(other.to_string(), "ja_JP");
    }

    #[test]
    fn rejects_invalid_formats() {
        assert!(Locale::parse("en-US").is_err());
        assert!(Locale::parse("EN_US").is_err());
        assert!(Locale::parse("en_us").is_err());
        assert!(Locale::parse("en").is_err());
        assert!(Locale::parse("").is_err());
        assert!(!is_iso_15897_xx_xx("en_USX"));
    }

    #[test]
    fn possible_table_round_trips_and_maps_to_generated() {
        assert_eq!(Locale::POSSIBLE.len(), 24);
        assert_eq!(Locale::HOSTED.len(), 24);
        for locale in Locale::POSSIBLE {
            assert!(locale.is_possible());
            assert!(locale.is_hosted());
            let again = Locale::parse(locale.as_str()).unwrap();
            assert_eq!(again, locale);
            let generated = locale
                .into_generated()
                .expect("possible locales map to OpenAPI");
            assert!(generated.0.is_some());
        }
    }

    #[test]
    fn into_generated_maps_openapi_locales() {
        let locale = Locale::EN_US.into_generated().unwrap();
        assert_eq!(locale.0, Some(types::LocaleInner::EnUs));

        // Extended locales (cs_CZ etc.) are on the re-pinned OpenAPI enum even
        // though they are outside Mollie's documented "possible" hosted set.
        assert!(!Locale::CS_CZ.is_possible());
        assert_eq!(
            Locale::CS_CZ.into_generated().unwrap().0,
            Some(types::LocaleInner::CsCz)
        );
        assert!(Locale::parse("ja_JP").unwrap().into_generated().is_err());
    }

    #[test]
    fn converts_to_method_query_locales() {
        let list = Locale::NL_NL.into_list_methods_locale().unwrap();
        assert_eq!(list.0, Some(types::LocaleInner::NlNl));
        assert_eq!(
            Locale::CS_CZ.into_list_methods_locale().unwrap().0,
            Some(types::LocaleInner::CsCz)
        );
        assert!(Locale::parse("ja_JP")
            .unwrap()
            .into_list_methods_locale()
            .is_err());
    }
}
