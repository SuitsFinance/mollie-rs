#!/usr/bin/env python3
"""Generate src/country_code.rs with ISO 3166-1 alpha-2 codes.

Data columns (aligned with ISO 3166 / Wikipedia country tables):
1. Entry — ISO 3166-1 alpha-2 code
2. Country name — English short name used by ISO 3166/MA (title case)
3. Subdivisions — summary of ISO 3166-2 coded subdivisions (empty if none)

Also keeps historically assigned AN (Netherlands Antilles) for integrators
that still receive that transitional code.
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# code -> (english_short_name, subdivisions_summary)
# Source: ISO 3166-1 alpha-2 English short names + ISO 3166-2 subdivision notes.
COUNTRIES: dict[str, tuple[str, str]] = {
    "AD": ("Andorra", "7 parishes"),
    "AE": ("United Arab Emirates", "emirates"),
    "AF": ("Afghanistan", "34 provinces"),
    "AG": ("Antigua and Barbuda", "6 parishes, 2 dependencies"),
    "AI": ("Anguilla", ""),
    "AL": ("Albania", "12 counties"),
    "AM": ("Armenia", "1 city, 10 regions"),
    "AO": ("Angola", "18 provinces"),
    "AQ": ("Antarctica", ""),
    "AR": ("Argentina", "1 city, 23 provinces"),
    "AS": ("American Samoa", ""),
    "AT": ("Austria", "9 states"),
    "AU": ("Australia", "6 states, 2 territories"),
    "AW": ("Aruba", ""),
    "AX": ("Åland Islands", ""),
    "AZ": ("Azerbaijan", "1 autonomous republic, 11 municipalities, 66 rayons"),
    "BA": ("Bosnia and Herzegovina", "entities, 1 district with special status"),
    "BB": ("Barbados", "11 parishes"),
    "BD": ("Bangladesh", "8 divisions, 64 districts"),
    "BE": ("Belgium", "3 regions, 10 provinces"),
    "BF": ("Burkina Faso", "regions, 45 provinces"),
    "BG": ("Bulgaria", "28 regions"),
    "BH": ("Bahrain", "4 governorates"),
    "BI": ("Burundi", "18 provinces"),
    "BJ": ("Benin", "12 departments"),
    "BL": ("Saint Barthélemy", ""),
    "BM": ("Bermuda", ""),
    "BN": ("Brunei Darussalam", "districts"),
    "BO": ("Bolivia, Plurinational State of", "departments"),
    "BQ": ("Bonaire, Sint Eustatius and Saba", "special municipalities"),
    "BR": ("Brazil", "1 federal district, 26 states"),
    "BS": ("Bahamas", "31 districts, 1 island"),
    "BT": ("Bhutan", "20 districts"),
    "BV": ("Bouvet Island", ""),
    "BW": ("Botswana", "10 districts, 4 towns, 2 cities"),
    "BY": ("Belarus", "6 oblasts, 1 city"),
    "BZ": ("Belize", "6 districts"),
    "CA": ("Canada", "10 provinces, 3 territories"),
    "CC": ("Cocos (Keeling) Islands", ""),
    "CD": ("Congo, Democratic Republic of the", "1 city, 25 provinces"),
    "CF": ("Central African Republic", "1 commune, 14 prefectures, 2 economic prefectures"),
    "CG": ("Congo", "12 departments"),
    "CH": ("Switzerland", "26 cantons"),
    "CI": ("Côte d'Ivoire", "districts, 2 autonomous districts"),
    "CK": ("Cook Islands", ""),
    "CL": ("Chile", "16 regions"),
    "CM": ("Cameroon", "10 regions"),
    "CN": (
        "China",
        "4 municipalities, 23 provinces, 5 autonomous regions, 2 special administrative regions",
    ),
    "CO": ("Colombia", "1 capital district, 32 departments"),
    "CR": ("Costa Rica", "provinces"),
    "CU": ("Cuba", "15 provinces, 1 special municipality"),
    "CV": ("Cabo Verde", "geographical regions, 22 municipalities"),
    "CW": ("Curaçao", ""),
    "CX": ("Christmas Island", ""),
    "CY": ("Cyprus", "6 districts"),
    "CZ": ("Czechia", "13 regions, 1 capital city, 76 districts"),
    "DE": ("Germany", "16 states"),
    "DJ": ("Djibouti", "5 regions, 1 city"),
    "DK": ("Denmark", "5 regions"),
    "DM": ("Dominica", "10 parishes"),
    "DO": ("Dominican Republic", "regions, 1 district, 31 provinces"),
    "DZ": ("Algeria", "58 provinces"),
    "EC": ("Ecuador", "24 provinces"),
    "EE": ("Estonia", "15 counties, 64 rural municipalities, 15 urban municipalities"),
    "EG": ("Egypt", "27 governorates"),
    "EH": ("Western Sahara", ""),
    "ER": ("Eritrea", "6 regions"),
    "ES": (
        "Spain",
        "17 autonomous communities, 2 autonomous cities in North Africa, 50 provinces",
    ),
    "ET": ("Ethiopia", "2 administrations, 11 regional states"),
    "FI": ("Finland", "19 regions"),
    "FJ": ("Fiji", "4 divisions, 1 dependency, 14 provinces"),
    "FK": ("Falkland Islands (Malvinas)", ""),
    "FM": ("Micronesia, Federated States of", "states"),
    "FO": ("Faroe Islands", ""),
    "FR": (
        "France",
        "12 metropolitan regions and overseas collectivities/territories; "
        "95 metropolitan departments and related entities",
    ),
    "GA": ("Gabon", "9 provinces"),
    "GB": (
        "United Kingdom of Great Britain and Northern Ireland",
        "countries, province, council areas, counties, districts, unitary authorities, "
        "metropolitan districts, London boroughs, city corporation",
    ),
    "GD": ("Grenada", "6 parishes, 1 dependency"),
    "GE": ("Georgia", "2 autonomous republics, 1 city, 9 regions"),
    "GF": ("French Guiana", ""),
    "GG": ("Guernsey", ""),
    "GH": ("Ghana", "16 regions"),
    "GI": ("Gibraltar", ""),
    "GL": ("Greenland", "5 municipalities"),
    "GM": ("Gambia", "1 city, 5 divisions"),
    "GN": ("Guinea", "7 administrative regions, 1 governorate, 33 prefectures"),
    "GP": ("Guadeloupe", ""),
    "GQ": ("Equatorial Guinea", "regions, 8 provinces"),
    "GR": ("Greece", "13 administrative regions, 1 self-governed part"),
    "GS": ("South Georgia and the South Sandwich Islands", ""),
    "GT": ("Guatemala", "22 departments"),
    "GU": ("Guam", ""),
    "GW": ("Guinea-Bissau", "3 provinces, 1 autonomous sector, 8 regions"),
    "GY": ("Guyana", "10 regions"),
    "HK": ("Hong Kong", ""),
    "HM": ("Heard Island and McDonald Islands", ""),
    "HN": ("Honduras", "18 departments"),
    "HR": ("Croatia", "1 city, 20 counties"),
    "HT": ("Haiti", "10 departments"),
    "HU": ("Hungary", "1 capital city, 19 counties, 23 cities of county right"),
    "ID": (
        "Indonesia",
        "7 geographical units, 36 provinces, 1 capital district, 1 special region",
    ),
    "IE": ("Ireland", "4 provinces, 26 counties"),
    "IL": ("Israel", "6 districts"),
    "IM": ("Isle of Man", ""),
    "IN": ("India", "28 states, 8 union territories"),
    "IO": ("British Indian Ocean Territory", ""),
    "IQ": ("Iraq", "1 region, 18 governorates"),
    "IR": ("Iran, Islamic Republic of", "provinces"),
    "IS": ("Iceland", "8 regions, 64 municipalities"),
    "IT": (
        "Italy",
        "15 regions, 5 autonomous regions, provinces, free municipal consortiums, "
        "metropolitan cities, decentralized regional entities",
    ),
    "JE": ("Jersey", ""),
    "JM": ("Jamaica", "14 parishes"),
    "JO": ("Jordan", "12 governorates"),
    "JP": ("Japan", "47 prefectures"),
    "KE": ("Kenya", "47 counties"),
    "KG": ("Kyrgyzstan", "2 cities, 7 regions"),
    "KH": ("Cambodia", "1 autonomous municipality, 24 provinces"),
    "KI": ("Kiribati", "3 groups of islands"),
    "KM": ("Comoros", "3 islands"),
    "KN": ("Saint Kitts and Nevis", "states, 14 parishes"),
    "KP": (
        "Korea, Democratic People's Republic of",
        "capital city, metropolitan city, special city, 9 provinces",
    ),
    "KR": (
        "Korea, Republic of",
        "metropolitan cities, special cities, special self-governing city, "
        "provinces, special self-governing provinces",
    ),
    "KW": ("Kuwait", "6 governorates"),
    "KY": ("Cayman Islands", ""),
    "KZ": ("Kazakhstan", "3 cities, 17 regions"),
    "LA": ("Lao People's Democratic Republic", "1 prefecture, 17 provinces"),
    "LB": ("Lebanon", "8 governorates"),
    "LC": ("Saint Lucia", "districts"),
    "LI": ("Liechtenstein", "11 communes"),
    "LK": ("Sri Lanka", "provinces, 25 districts"),
    "LR": ("Liberia", "15 counties"),
    "LS": ("Lesotho", "10 districts"),
    "LT": (
        "Lithuania",
        "10 counties, 9 municipalities, 7 city municipalities, 44 district municipalities",
    ),
    "LU": ("Luxembourg", "12 cantons"),
    "LV": ("Latvia", "36 municipalities, 7 state cities"),
    "LY": ("Libya", "22 popularates"),
    "MA": ("Morocco", "12 regions, 62 provinces, 13 prefectures"),
    "MC": ("Monaco", "17 quarters"),
    "MD": (
        "Moldova, Republic of",
        "autonomous territorial unit, 3 cities, 32 districts, 1 territorial unit",
    ),
    "ME": ("Montenegro", "25 municipalities"),
    "MF": ("Saint Martin (French part)", ""),
    "MG": ("Madagascar", "6 provinces"),
    "MH": ("Marshall Islands", "chains of islands, 24 municipalities"),
    "MK": ("North Macedonia", "municipalities"),
    "ML": ("Mali", "1 district, 10 regions"),
    "MM": ("Myanmar", "7 regions, 7 states, 1 union territory"),
    "MN": ("Mongolia", "1 capital city, 21 provinces"),
    "MO": ("Macao", ""),
    "MP": ("Northern Mariana Islands", ""),
    "MQ": ("Martinique", ""),
    "MR": ("Mauritania", "15 regions"),
    "MS": ("Montserrat", ""),
    "MT": ("Malta", "68 local councils"),
    "MU": ("Mauritius", "3 dependencies, 9 districts"),
    "MV": ("Maldives", "19 administrative atolls, 2 cities"),
    "MW": ("Malawi", "3 regions, 28 districts"),
    "MX": ("Mexico", "31 states, 1 federal entity"),
    "MY": ("Malaysia", "3 federal territories, 13 states"),
    "MZ": ("Mozambique", "1 city, 10 provinces"),
    "NA": ("Namibia", "14 regions"),
    "NC": ("New Caledonia", ""),
    "NE": ("Niger", "1 urban community, 7 departments"),
    "NF": ("Norfolk Island", ""),
    "NG": ("Nigeria", "1 capital territory, 36 states"),
    "NI": ("Nicaragua", "15 departments, 2 autonomous regions"),
    "NL": (
        "Netherlands, Kingdom of the",
        "provinces, 3 countries, 3 special municipalities",
    ),
    "NO": ("Norway", "11 counties, 2 arctic regions"),
    "NP": ("Nepal", "7 provinces"),
    "NR": ("Nauru", "14 districts"),
    "NU": ("Niue", ""),
    "NZ": ("New Zealand", "regions, 1 special island authority"),
    "OM": ("Oman", "11 governorates"),
    "PA": ("Panama", "10 provinces, 4 indigenous regions"),
    "PE": ("Peru", "25 regions, 1 municipality"),
    "PF": ("French Polynesia", ""),
    "PG": ("Papua New Guinea", "1 district, 20 provinces, 1 autonomous region"),
    "PH": ("Philippines", "17 regions, 82 provinces"),
    "PK": ("Pakistan", "4 provinces, 2 autonomous territories, 1 federal territory"),
    "PL": ("Poland", "16 voivodships"),
    "PM": ("Saint Pierre and Miquelon", ""),
    "PN": ("Pitcairn", ""),
    "PR": ("Puerto Rico", ""),
    "PS": ("Palestine, State of", "governorates"),
    "PT": ("Portugal", "18 districts, 2 autonomous regions"),
    "PW": ("Palau", "16 states"),
    "PY": ("Paraguay", "1 capital, 17 departments"),
    "QA": ("Qatar", "8 municipalities"),
    "RE": ("Réunion", ""),
    "RO": ("Romania", "41 departments, 1 municipality"),
    "RS": ("Serbia", "2 autonomous provinces, 1 city, 29 districts"),
    "RU": (
        "Russian Federation",
        "republics, administrative territories, administrative regions, "
        "autonomous cities, autonomous region, autonomous districts",
    ),
    "RW": ("Rwanda", "1 town council, 4 provinces"),
    "SA": ("Saudi Arabia", "regions"),
    "SB": ("Solomon Islands", "1 capital territory, 9 provinces"),
    "SC": ("Seychelles", "27 districts"),
    "SD": ("Sudan", "18 states"),
    "SE": ("Sweden", "21 counties"),
    "SG": ("Singapore", "5 districts"),
    "SH": ("Saint Helena, Ascension and Tristan da Cunha", "geographical entities"),
    "SI": ("Slovenia", "200 municipalities and 12 urban municipalities"),
    "SJ": ("Svalbard and Jan Mayen", ""),
    "SK": ("Slovakia", "8 regions"),
    "SL": ("Sierra Leone", "area, 4 provinces"),
    "SM": ("San Marino", "municipalities"),
    "SN": ("Senegal", "14 regions"),
    "SO": ("Somalia", "18 regions"),
    "SR": ("Suriname", "10 districts"),
    "SS": ("South Sudan", "states"),
    "ST": ("Sao Tome and Principe", "1 autonomous region, 6 districts"),
    "SV": ("El Salvador", "departments"),
    "SX": ("Sint Maarten (Dutch part)", ""),
    "SY": ("Syrian Arab Republic", "provinces"),
    "SZ": ("Eswatini", "4 regions"),
    "TC": ("Turks and Caicos Islands", ""),
    "TD": ("Chad", "23 provinces"),
    "TF": ("French Southern Territories", ""),
    "TG": ("Togo", "5 regions"),
    "TH": (
        "Thailand",
        "1 metropolitan administration, 1 special administrative city, 76 provinces",
    ),
    "TJ": (
        "Tajikistan",
        "1 autonomous region, 2 regions, 1 capital territory, "
        "1 district under republic administration",
    ),
    "TK": ("Tokelau", ""),
    "TL": ("Timor-Leste", "12 municipalities, 1 special administrative region"),
    "TM": ("Turkmenistan", "5 regions, 1 city"),
    "TN": ("Tunisia", "24 governorates"),
    "TO": ("Tonga", "5 divisions"),
    "TR": ("Türkiye", "81 provinces"),
    "TT": ("Trinidad and Tobago", "regions, 3 boroughs, 2 cities, 1 ward"),
    "TV": ("Tuvalu", "1 town council, 7 island councils"),
    "TW": ("Taiwan, Province of China", "counties, 3 cities, 6 special municipalities"),
    "TZ": ("Tanzania, United Republic of", "regions"),
    "UA": ("Ukraine", "24 regions, 1 republic, 2 cities"),
    "UG": ("Uganda", "4 geographical regions, 134 districts, 1 city"),
    "UM": ("United States Minor Outlying Islands", "islands, groups of islands"),
    "US": ("United States of America", "states, 1 district, 6 outlying areas"),
    "UY": ("Uruguay", "19 departments"),
    "UZ": ("Uzbekistan", "1 city, 12 regions, 1 republic"),
    "VA": ("Holy See", ""),
    "VC": ("Saint Vincent and the Grenadines", "parishes"),
    "VE": (
        "Venezuela, Bolivarian Republic of",
        "federal dependency, 1 federal district, 23 states",
    ),
    "VG": ("Virgin Islands (British)", ""),
    "VI": ("Virgin Islands (U.S.)", ""),
    "VN": ("Viet Nam", "provinces, 5 municipalities"),
    "VU": ("Vanuatu", "6 provinces"),
    "WF": ("Wallis and Futuna", "administrative precincts"),
    "WS": ("Samoa", "11 districts"),
    "YE": ("Yemen", "1 municipality, 21 governorates"),
    "YT": ("Mayotte", ""),
    "ZA": ("South Africa", "provinces"),
    "ZM": ("Zambia", "10 provinces"),
    "ZW": ("Zimbabwe", "10 provinces"),
    # Historically assigned (deleted from the currently assigned set).
    "AN": (
        "Netherlands Antilles (historically assigned; transitional ISO code)",
        "",
    ),
}

CODES = sorted(COUNTRIES.keys())
assert len(CODES) == 250, len(CODES)
assert "AN" in COUNTRIES
assert COUNTRIES["NL"][0].startswith("Netherlands")
assert COUNTRIES["GB"][0].startswith("United Kingdom")


def variant(code: str) -> str:
    return code[0] + code[1].lower()


def rust_string(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def main() -> None:
    lines: list[str] = []
    lines.append("//! ISO 3166-1 alpha-2 country codes for Mollie request fields.")
    lines.append("//!")
    lines.append("//! Each entry is the ISO 3166-1 **alpha-2** code. English short names")
    lines.append("//! follow the ISO 3166 Maintenance Agency (ISO 3166/MA). Optional")
    lines.append("//! subdivision summaries describe categories coded in **ISO 3166-2**")
    lines.append("//! (not full subdivision code lists).")
    lines.append("//!")
    lines.append("//! Mollie documents country fields (billing country, address `country`,")
    lines.append("//! etc.) as ISO 3166-1 alpha-2. Generated OpenAPI types often keep these")
    lines.append("//! as plain strings; validate with [`CountryCode`] first.")
    lines.append("//!")
    lines.append("//! See `docs/iso/iso-3166-1-alpha-2.md`.")
    lines.append("//!")
    lines.append("//! Regenerated by `scripts/generate_country_code.py`.")
    lines.append("#![warn(missing_docs)]")
    lines.append("")
    lines.append("use std::{fmt, str::FromStr};")
    lines.append("")
    lines.append("use crate::{MollieError, MollieResult};")
    lines.append("")
    lines.append("/// An ISO 3166-1 alpha-2 country code (`NL`, `DE`, `US`, …).")
    lines.append("///")
    lines.append("/// Use [`CountryCode::parse`] to validate strings before sending them as")
    lines.append("/// `billingCountry`, address `country`, or similar Mollie fields.")
    lines.append("///")
    lines.append("/// # Columns")
    lines.append("///")
    lines.append("/// | Method | Content |")
    lines.append("/// | --- | --- |")
    lines.append("/// | [`as_str`](Self::as_str) | Entry (alpha-2 code) |")
    lines.append("/// | [`name`](Self::name) | English short name (ISO 3166/MA) |")
    lines.append(
        "/// | [`subdivisions`](Self::subdivisions) | ISO 3166-2 subdivision categories (if any) |"
    )
    lines.append("#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]")
    lines.append("pub enum CountryCode {")
    for code in CODES:
        name, subs = COUNTRIES[code]
        lines.append(f"    /// {name} — `{code}`.")
        if subs:
            lines.append(f"    ///")
            lines.append(f"    /// ISO 3166-2: {subs}.")
        lines.append(f"    {variant(code)},")
    lines.append("}")
    lines.append("")
    lines.append("impl CountryCode {")
    for code in CODES:
        name, _subs = COUNTRIES[code]
        lines.append(f"    /// {name}.")
        lines.append(f"    pub const {code}: Self = Self::{variant(code)};")
    lines.append("")
    lines.append(
        f"    /// All ISO 3166-1 alpha-2 codes recognized by this crate ({len(CODES)})."
    )
    lines.append(f"    pub const ALL: [Self; {len(CODES)}] = [")
    for code in CODES:
        lines.append(f"        Self::{variant(code)},")
    lines.append("    ];")
    lines.append("")
    lines.append("    /// Parses an ISO 3166-1 alpha-2 country code.")
    lines.append("    ///")
    lines.append("    /// Accepts the uppercase wire form only (`NL`, not `nl` or `NLD`).")
    lines.append("    ///")
    lines.append("    /// # Errors")
    lines.append("    ///")
    lines.append("    /// Returns [`MollieError::InvalidRequest`] when the value is not a")
    lines.append("    /// recognized two-letter uppercase code.")
    lines.append("    ///")
    lines.append("    /// # Examples")
    lines.append("    ///")
    lines.append("    /// ```rust")
    lines.append("    /// use mollie_rs::CountryCode;")
    lines.append("    ///")
    lines.append('    /// assert_eq!(CountryCode::parse("NL")?, CountryCode::NL);')
    lines.append(
        '    /// assert_eq!(CountryCode::NL.name(), "Netherlands, Kingdom of the");'
    )
    lines.append('    /// assert!(CountryCode::parse("nl").is_err());')
    lines.append('    /// assert!(CountryCode::parse("NLD").is_err());')
    lines.append("    /// # Ok::<(), mollie_rs::MollieError>(())")
    lines.append("    /// ```")
    lines.append("    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {")
    lines.append("        value.as_ref().parse()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the uppercase ISO 3166-1 alpha-2 **entry** code.")
    lines.append("    ///")
    lines.append("    /// # Examples")
    lines.append("    ///")
    lines.append("    /// ```rust")
    lines.append("    /// use mollie_rs::CountryCode;")
    lines.append("    ///")
    lines.append('    /// assert_eq!(CountryCode::NL.as_str(), "NL");')
    lines.append("    /// ```")
    lines.append("    pub const fn as_str(self) -> &'static str {")
    lines.append("        match self {")
    for code in CODES:
        lines.append(f'            Self::{variant(code)} => "{code}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append(
        "    /// Returns the English short country name used by ISO 3166/MA (title case)."
    )
    lines.append("    ///")
    lines.append("    /// # Examples")
    lines.append("    ///")
    lines.append("    /// ```rust")
    lines.append("    /// use mollie_rs::CountryCode;")
    lines.append("    ///")
    lines.append('    /// assert_eq!(CountryCode::DE.name(), "Germany");')
    lines.append(
        '    /// assert_eq!(CountryCode::BO.name(), "Bolivia, Plurinational State of");'
    )
    lines.append("    /// ```")
    lines.append("    pub const fn name(self) -> &'static str {")
    lines.append("        match self {")
    for code in CODES:
        name, _ = COUNTRIES[code]
        lines.append(f'            Self::{variant(code)} => "{rust_string(name)}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append(
        "    /// Returns a short summary of ISO 3166-2 subdivision categories for this country,"
    )
    lines.append("    /// when documented.")
    lines.append("    ///")
    lines.append(
        "    /// This is **not** a full list of subdivision codes (e.g. `NL-NH`); it only"
    )
    lines.append("    /// describes how many / which kinds of subdivisions are coded.")
    lines.append("    ///")
    lines.append("    /// # Examples")
    lines.append("    ///")
    lines.append("    /// ```rust")
    lines.append("    /// use mollie_rs::CountryCode;")
    lines.append("    ///")
    lines.append('    /// assert_eq!(CountryCode::AD.subdivisions(), Some("7 parishes"));')
    lines.append("    /// assert_eq!(CountryCode::AQ.subdivisions(), None);")
    lines.append("    /// ```")
    lines.append("    pub const fn subdivisions(self) -> Option<&'static str> {")
    lines.append("        match self {")
    for code in CODES:
        _name, subs = COUNTRIES[code]
        if subs:
            lines.append(
                f'            Self::{variant(code)} => Some("{rust_string(subs)}"),'
            )
        else:
            lines.append(f"            Self::{variant(code)} => None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns true when `value` is a recognized ISO 3166-1 alpha-2 code.")
    lines.append("    ///")
    lines.append("    /// # Examples")
    lines.append("    ///")
    lines.append("    /// ```rust")
    lines.append("    /// use mollie_rs::CountryCode;")
    lines.append("    ///")
    lines.append('    /// assert!(CountryCode::is_valid("DE"));')
    lines.append('    /// assert!(!CountryCode::is_valid("XX"));')
    lines.append("    /// ```")
    lines.append("    pub fn is_valid(value: impl AsRef<str>) -> bool {")
    lines.append("        Self::parse(value).is_ok()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns true when `value` matches the ISO 3166-1 alpha-2 *format*")
    lines.append("    /// (exactly two ASCII uppercase letters), without checking assignment.")
    lines.append("    pub fn is_valid_format(value: impl AsRef<str>) -> bool {")
    lines.append("        let b = value.as_ref().as_bytes();")
    lines.append(
        "        b.len() == 2 && b[0].is_ascii_uppercase() && b[1].is_ascii_uppercase()"
    )
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl FromStr for CountryCode {")
    lines.append("    type Err = MollieError;")
    lines.append("")
    lines.append("    fn from_str(value: &str) -> Result<Self, Self::Err> {")
    lines.append("        match value {")
    for code in CODES:
        lines.append(f'            "{code}" => Ok(Self::{variant(code)}),')
    lines.append("            other => Err(MollieError::invalid_request(format!(")
    lines.append(
        '                "unsupported ISO 3166-1 alpha-2 country code `{other}`"'
    )
    lines.append("            ))),")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl fmt::Display for CountryCode {")
    lines.append("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")
    lines.append("        f.write_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl AsRef<str> for CountryCode {")
    lines.append("    fn as_ref(&self) -> &str {")
    lines.append("        self.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl TryFrom<&str> for CountryCode {")
    lines.append("    type Error = MollieError;")
    lines.append(
        "    fn try_from(value: &str) -> Result<Self, Self::Error> { Self::parse(value) }"
    )
    lines.append("}")
    lines.append("")
    lines.append("impl TryFrom<String> for CountryCode {")
    lines.append("    type Error = MollieError;")
    lines.append(
        "    fn try_from(value: String) -> Result<Self, Self::Error> { Self::parse(value) }"
    )
    lines.append("}")
    lines.append("")
    lines.append("impl From<CountryCode> for String {")
    lines.append(
        "    fn from(value: CountryCode) -> Self { value.as_str().to_string() }"
    )
    lines.append("}")
    lines.append("")
    lines.append("#[cfg(test)]")
    lines.append("mod tests {")
    lines.append("    use super::CountryCode;")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn parse_round_trips_all_codes() {")
    lines.append(f"        assert_eq!(CountryCode::ALL.len(), {len(CODES)});")
    lines.append("        for code in CountryCode::ALL {")
    lines.append("            let again = CountryCode::parse(code.as_str()).unwrap();")
    lines.append("            assert_eq!(again, code);")
    lines.append("            assert_eq!(again.to_string().len(), 2);")
    lines.append("            assert!(!code.name().is_empty());")
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn official_names_and_subdivisions_samples() {")
    lines.append(
        '        assert_eq!(CountryCode::NL.name(), "Netherlands, Kingdom of the");'
    )
    lines.append(
        '        assert_eq!(CountryCode::GB.name(), "United Kingdom of Great Britain and Northern Ireland");'
    )
    lines.append(
        '        assert_eq!(CountryCode::BO.name(), "Bolivia, Plurinational State of");'
    )
    lines.append('        assert_eq!(CountryCode::AD.subdivisions(), Some("7 parishes"));')
    lines.append("        assert_eq!(CountryCode::AQ.subdivisions(), None);")
    lines.append("        assert!(CountryCode::is_valid(\"AN\")); // historical")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn rejects_invalid() {")
    lines.append('        assert!(CountryCode::parse("nl").is_err());')
    lines.append('        assert!(CountryCode::parse("NLD").is_err());')
    lines.append('        assert!(CountryCode::parse("XX").is_err());')
    lines.append('        assert!(CountryCode::parse("").is_err());')
    lines.append('        assert!(CountryCode::is_valid_format("XX"));')
    lines.append('        assert!(!CountryCode::is_valid("XX"));')
    lines.append("    }")
    lines.append("}")
    lines.append("")

    out = ROOT / "src" / "country_code.rs"
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {out} ({len(lines)} lines, {len(CODES)} codes)")


if __name__ == "__main__":
    main()
