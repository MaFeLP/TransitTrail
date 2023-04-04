use super::UrlParameter;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceAdvisory {
    pub key: u32,
    pub priority: Priority,
    pub title: String,
    pub body: String,
    pub category: Category,
    #[serde(rename = "updated-at")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Priority {
    VeryHigh = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    VeryLow = 5,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Transit,
    #[serde(rename = "Handi-Transit")]
    HandiTransit,
    All,
}

impl Default for Category {
    fn default() -> Self {
        Self::All
    }
}

impl From<Priority> for UrlParameter {
    fn from(value: Priority) -> Self {
        Self(format!(
            "&priority={}",
            match value {
                Priority::VeryHigh => 1,
                Priority::High => 2,
                Priority::Medium => 3,
                Priority::Low => 4,
                Priority::VeryLow => 5,
            }
        ))
    }
}

impl From<Option<Priority>> for UrlParameter {
    fn from(value: Option<Priority>) -> Self {
        match value {
            Some(v) => Self::from(v),
            None => Self("".to_string()),
        }
    }
}

impl From<Category> for UrlParameter {
    fn from(value: Category) -> Self {
        Self(format!(
            "&category={}",
            match value {
                Category::Transit => "transit",
                Category::HandiTransit => "handi-transit",
                Category::All => "all",
            }
        ))
    }
}

impl From<Option<Category>> for UrlParameter {
    fn from(value: Option<Category>) -> Self {
        match value {
            Some(v) => Self::from(v),
            None => Self("".to_string()),
        }
    }
}
