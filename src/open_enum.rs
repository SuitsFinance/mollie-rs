//! Forward-compatible open enums that preserve unknown provider values (INV-ENUM-01).
//!
//! Prefer this over `#[serde(other)] Unknown` which drops the raw wire string.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

/// Hard ceiling for unknown enum raw strings (fuzz / memory bound).
pub const OPEN_ENUM_MAX_RAW_LEN: usize = 4096;

/// A provider enum that may gain values without breaking decode.
///
/// - [`OpenEnum::known`] when the value maps to `K`
/// - [`OpenEnum::unknown`] when the provider sent a new/unrecognized string
///
/// Serialization always emits the provider raw string.
#[derive(Clone, Debug, Eq)]
pub struct OpenEnum<K> {
    raw: String,
    known: Option<K>,
}

impl<K> OpenEnum<K> {
    /// Creates an open enum from a known typed variant and its wire value.
    pub fn from_known(known: K, raw: impl Into<String>) -> Result<Self, OpenEnumError> {
        let raw = raw.into();
        validate_raw(&raw)?;
        Ok(Self {
            raw,
            known: Some(known),
        })
    }

    /// Creates an open enum that only carries an unknown provider value.
    pub fn unknown(raw: impl Into<String>) -> Result<Self, OpenEnumError> {
        let raw = raw.into();
        validate_raw(&raw)?;
        Ok(Self { raw, known: None })
    }

    /// Provider wire value (always preserved).
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Typed known value when recognized.
    pub fn known(&self) -> Option<&K> {
        self.known.as_ref()
    }

    /// Returns `true` when the value was not recognized as `K`.
    pub fn is_unknown(&self) -> bool {
        self.known.is_none()
    }

    /// Rebuild known mapping with a parse function after construction.
    pub fn reparse_with<F>(self, mut parse: F) -> Self
    where
        F: FnMut(&str) -> Option<K>,
    {
        let known = parse(&self.raw);
        Self {
            raw: self.raw,
            known,
        }
    }
}

impl<K: PartialEq> PartialEq for OpenEnum<K> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<K> Hash for OpenEnum<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<K> fmt::Display for OpenEnum<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

impl<K> AsRef<str> for OpenEnum<K> {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

impl<K: FromStr> OpenEnum<K> {
    /// Parse using `K: FromStr`; unknown values remain preservable.
    pub fn parse_str(raw: impl Into<String>) -> Result<Self, OpenEnumError> {
        let raw = raw.into();
        validate_raw(&raw)?;
        let known = K::from_str(&raw).ok();
        Ok(Self { raw, known })
    }
}

impl<K> Serialize for OpenEnum<K> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.raw)
    }
}

impl<'de, K: FromStr> Deserialize<'de> for OpenEnum<K> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V<K>(PhantomData<K>);
        impl<'de, K: FromStr> Visitor<'de> for V<K> {
            type Value = OpenEnum<K>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string enum value")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                OpenEnum::parse_str(v).map_err(E::custom)
            }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                OpenEnum::parse_str(v).map_err(E::custom)
            }
        }
        deserializer.deserialize_string(V::<K>(PhantomData))
    }
}

/// Error when constructing an [`OpenEnum`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenEnumError {
    /// Empty provider value.
    Empty,
    /// Exceeds [`OPEN_ENUM_MAX_RAW_LEN`].
    TooLong {
        /// Observed length.
        len: usize,
        /// Configured maximum.
        max: usize,
    },
}

impl fmt::Display for OpenEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("open enum value must not be empty"),
            Self::TooLong { len, max } => {
                write!(f, "open enum value length {len} exceeds max {max}")
            }
        }
    }
}

impl std::error::Error for OpenEnumError {}

fn validate_raw(raw: &str) -> Result<(), OpenEnumError> {
    if raw.is_empty() {
        return Err(OpenEnumError::Empty);
    }
    if raw.len() > OPEN_ENUM_MAX_RAW_LEN {
        return Err(OpenEnumError::TooLong {
            len: raw.len(),
            max: OPEN_ENUM_MAX_RAW_LEN,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Color {
        Red,
        Blue,
    }

    impl FromStr for Color {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "red" => Ok(Self::Red),
                "blue" => Ok(Self::Blue),
                _ => Err(()),
            }
        }
    }

    #[test]
    fn known_round_trip() {
        let v: OpenEnum<Color> = serde_json::from_str("\"red\"").unwrap();
        assert_eq!(v.known(), Some(&Color::Red));
        assert_eq!(serde_json::to_string(&v).unwrap(), "\"red\"");
    }

    #[test]
    fn unknown_preserved() {
        let v: OpenEnum<Color> = serde_json::from_str("\"chartreuse\"").unwrap();
        assert!(v.is_unknown());
        assert_eq!(v.as_str(), "chartreuse");
        assert_eq!(serde_json::to_string(&v).unwrap(), "\"chartreuse\"");
        assert_eq!(v.to_string(), "chartreuse");
    }

    #[test]
    fn rejects_empty_and_too_long() {
        assert!(OpenEnum::<Color>::unknown("").is_err());
        let big = "x".repeat(OPEN_ENUM_MAX_RAW_LEN + 1);
        assert!(matches!(
            OpenEnum::<Color>::unknown(big),
            Err(OpenEnumError::TooLong { .. })
        ));
    }
}
