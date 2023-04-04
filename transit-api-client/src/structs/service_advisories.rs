use super::UrlParameter;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceAdvisory {
    pub key: u32,
    pub priority: ServiceAdvisoryPriority,
    pub title: String,
    pub body: String,
    pub category: ServiceAdvisoryCategory,
    #[serde(rename = "updated-at")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ServiceAdvisoryPriority {
    VeryHigh = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    VeryLow = 5,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServiceAdvisoryCategory {
    Transit,
    #[serde(rename = "Handi-Transit")]
    HandiTransit,
    All,
}

impl Default for ServiceAdvisoryCategory {
    fn default() -> Self {
        Self::All
    }
}

impl From<ServiceAdvisoryPriority> for UrlParameter {
    fn from(value: ServiceAdvisoryPriority) -> Self {
        Self(format!(
            "&priority={}",
            match value {
                ServiceAdvisoryPriority::VeryHigh => 1,
                ServiceAdvisoryPriority::High => 2,
                ServiceAdvisoryPriority::Medium => 3,
                ServiceAdvisoryPriority::Low => 4,
                ServiceAdvisoryPriority::VeryLow => 5,
            }
        ))
    }
}

impl From<Option<ServiceAdvisoryPriority>> for UrlParameter {
    fn from(value: Option<ServiceAdvisoryPriority>) -> Self {
        match value {
            Some(v) => Self::from(v),
            None => Self("".to_string()),
        }
    }
}

impl From<ServiceAdvisoryCategory> for UrlParameter {
    fn from(value: ServiceAdvisoryCategory) -> Self {
        Self(format!(
            "&category={}",
            match value {
                ServiceAdvisoryCategory::Transit => "transit",
                ServiceAdvisoryCategory::HandiTransit => "handi-transit",
                ServiceAdvisoryCategory::All => "all",
            }
        ))
    }
}

impl From<Option<ServiceAdvisoryCategory>> for UrlParameter {
    fn from(value: Option<ServiceAdvisoryCategory>) -> Self {
        match value {
            Some(v) => Self::from(v),
            None => Self("".to_string()),
        }
    }
}
