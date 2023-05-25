//!
//! This module holds all the datastructures that can be used in the API and returned from the API.
//!

use std::fmt::Display;
use std::str::FromStr;

use serde::{de::Error, Deserialize};
use serde_json::Value;

pub mod common;
pub mod destinations;
pub mod routes;
pub mod service_advisories;
pub mod stops;
pub mod trip_planner;
mod datetime_formatter;

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

/// Wrapper method for deserializing a field in a struct, that holds a different type than a
/// [String] but has quotation marks around its value. (Who came up with this idea?)
///
/// # Arguments
///
/// * `deserializer`: The deserialization object.
///
/// returns: Result<T, <D as Deserializer>::Error>
///
/// # Examples
///
/// ```
/// # use std::fmt::Display;
/// # use std::str::FromStr;
/// # use serde_json::Value;
/// # use serde::de::Error;
/// use serde::Deserialize;
/// #
/// # pub(crate) fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
/// #     where
/// #         D: serde::Deserializer<'de>,
/// #         T: FromStr, <T as FromStr>::Err: Display,
/// # {
/// #     let value = <Value>::deserialize(deserializer)?;
/// #     let string_value = value.as_str().ok_or(Error::custom("unknown type"))?;
/// #     let t_value: T = string_value.parse().map_err(Error::custom)?;
/// #
/// #     Ok(t_value)
/// # }
///
/// #[derive(Debug, Deserialize)]
/// struct Test {
///     #[serde(deserialize_with = "deserialize_from_string")]
///     float_string_field: f32,
///     #[serde(deserialize_with = "deserialize_from_string")]
///     integer_string_field: u32,
///     #[serde(deserialize_with = "deserialize_from_string")]
///     bool_string_field: bool,
/// }
///
/// let test: Test = serde_json::from_str(r#"{
///     "float_string_field": "12.34",
///     "integer_string_field": "1234",
///     "bool_string_field": "true"
/// }"#).unwrap();
/// ```
pub(crate) fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let value = <Value>::deserialize(deserializer)?;
    let string_value = value.as_str().ok_or(Error::custom("unknown type"))?;
    let t_value: T = string_value.parse().map_err(Error::custom)?;

    Ok(t_value)
}

