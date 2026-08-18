//! Explicit omit / null / value field semantics (INV-NULL-01).
//!
//! Use only for audited provider fields listed in
//! `docs/registries/field-semantics.yaml`. Do not blanket the public API.

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// Three-way field state matching common OpenAPI nullable optional fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum NullableField<T> {
    /// Field absent from the JSON object.
    #[default]
    Omitted,
    /// Field present as JSON `null`.
    Null,
    /// Field present with a value.
    Value(T),
}

impl<T> NullableField<T> {
    /// Returns a reference to the inner value when present.
    pub fn as_value(&self) -> Option<&T> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `true` when the field was omitted.
    pub const fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }

    /// Returns `true` when the field was explicit null.
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl<T: Serialize> Serialize for NullableField<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            // Callers should use skip_serializing_if = "NullableField::is_omitted"
            // on containing structs. Serializing Omitted as null would be wrong.
            Self::Omitted => serializer.serialize_none(),
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NullableField<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V<T>(PhantomData<T>);
        impl<'de, T: Deserialize<'de>> Visitor<'de> for V<T> {
            type Value = NullableField<T>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("null, a value, or absent field")
            }
            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(NullableField::Null)
            }
            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(NullableField::Null)
            }
            fn visit_some<D2: Deserializer<'de>>(self, d: D2) -> Result<Self::Value, D2::Error> {
                T::deserialize(d).map(NullableField::Value)
            }
        }
        // When the field is present, serde calls deserialize on the value.
        // For Option-like presence, containers should use Option<NullableField<T>>
        // or #[serde(default)] on NullableField.
        deserializer.deserialize_option(V::<T>(PhantomData))
    }
}

/// Helper for `skip_serializing_if` on containing structs.
pub fn is_omitted<T>(value: &NullableField<T>) -> bool {
    value.is_omitted()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Row {
        #[serde(default, skip_serializing_if = "NullableField::is_omitted")]
        due_date: NullableField<String>,
    }

    #[test]
    fn omitted_null_value_json() {
        let omitted = Row {
            due_date: NullableField::Omitted,
        };
        assert_eq!(serde_json::to_string(&omitted).unwrap(), "{}");

        let null = Row {
            due_date: NullableField::Null,
        };
        assert_eq!(
            serde_json::to_string(&null).unwrap(),
            r#"{"due_date":null}"#
        );

        let value = Row {
            due_date: NullableField::Value("2026-08-31".into()),
        };
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"{"due_date":"2026-08-31"}"#
        );

        let de_null: Row = serde_json::from_str(r#"{"due_date":null}"#).unwrap();
        assert_eq!(de_null.due_date, NullableField::Null);

        let de_val: Row = serde_json::from_str(r#"{"due_date":"2026-08-31"}"#).unwrap();
        assert_eq!(de_val.due_date, NullableField::Value("2026-08-31".into()));

        let de_omit: Row = serde_json::from_str("{}").unwrap();
        assert_eq!(de_omit.due_date, NullableField::Omitted);
    }
}
