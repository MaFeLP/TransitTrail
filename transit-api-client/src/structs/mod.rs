//!
//! This module holds all the datastructures that can be used in the API and returned from the API.
//!

use std::fmt::Display;

use serde::{de::Error, Deserialize};
use serde_json::Value;

pub mod common;
pub mod destinations;
pub mod routes;
pub mod service_advisories;
pub mod stops;
pub mod trip_planner;

#[derive(Clone, Debug, Default)]
/// A tuple struct that wraps a string. Other types can `impl<T> From<T> for UrlParameter` so that
/// each individual endpoint can easily format and use the structs, without modifying their Display
/// or Debug behaviours.
pub(crate) struct UrlParameter(pub(crate) String);

impl Display for UrlParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
/// If the API should yield shorter, longer, or normal names.
pub enum Usage {
    #[default]
    /// No modification to the length of the outputs
    Normal,

    /// Yields more verbose names
    Long,

    /// Yields terser names
    Short,
}

impl From<Usage> for UrlParameter {
    fn from(value: Usage) -> Self {
        Self(match value {
            Usage::Normal => "".to_string(),
            Usage::Long => "&usage=long".to_string(),
            Usage::Short => "&usage=short".to_string(),
        })
    }
}

/// Wrapper method for deserializing a field in a struct, that holds a boolean, but has quotation
/// marks. (Who came up with this idea?)
///
/// # Arguments
///
/// * `deserializer`: The deserialization object.
///
/// returns: Result<bool, <D as Deserializer>::Error>
///
/// # Examples
///
/// ```
/// # use serde_json::Value;
/// # use serde::de::Error;
/// use serde::Deserialize;
/// #
/// # fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
/// #     where
/// #         D: serde::Deserializer<'de>,
/// # {
/// #     let value = <Value>::deserialize(deserializer)?;
/// #     let string_value = value.as_str().ok_or(Error::custom("unknown type"))?;
/// #     let bool_value: bool = string_value.parse().map_err(Error::custom)?;
/// #
/// #     Ok(bool_value)
/// # }
///
/// #[derive(Debug, Deserialize)]
/// struct Test {
///     #[serde(deserialize_with = "deserialize_string_to_bool")]
///     boolean_string_field: bool,
/// }
///
/// let test: Test = serde_json::from_str(r#"{ "boolean_string_field": "false" }"#).unwrap();
/// ```
pub(crate) fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <Value>::deserialize(deserializer)?;
    let string_value = value.as_str().ok_or(Error::custom("unknown type"))?;
    let bool_value: bool = string_value.parse().map_err(Error::custom)?;

    Ok(bool_value)
}

time::serde::format_description!(
    datetime_formatter,
    PrimitiveDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second]"
);
