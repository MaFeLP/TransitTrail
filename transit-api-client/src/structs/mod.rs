use std::fmt::Display;

use serde::{de::Error, Deserialize};
use serde_json::Value;

pub mod common;
pub mod destinations;
pub mod routes;
pub mod service_advisories;
pub mod stops;
pub mod trip_planner;

pub(crate) const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

#[derive(Debug, Default)]
pub(crate) struct UrlParameter(String);

impl Display for UrlParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Usage {
    Normal,
    Long,
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

pub(crate) fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <Value>::deserialize(deserializer)?;
    let string_value = value.as_str().ok_or(Error::custom("unknown type"))?;
    let bool_value: bool = string_value.parse().map_err(Error::custom)?;

    Ok(bool_value)
}
