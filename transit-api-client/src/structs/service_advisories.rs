//!
//! Structures for the [service_advisories endpoint](crate::endpoints::service_advisories)
//!

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::UrlParameter;

/// A service advisory containing data about the advisory
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceAdvisory {
    /// A unique key to identify the advisory
    pub key: u32,

    /// An indicator of how urgent the advisory is
    pub priority: Priority,

    /// A title ascribed to the advisory.
    pub title: String,

    /// The content of the advisory.
    pub body: String,

    /// Service advisories belong to a category
    pub category: Category,

    /// Timestamp of when the advisory was last updated.
    #[serde(rename = "updated-at")]
    pub updated_at: NaiveDateTime,
}

/// A numerical indicator of how urgent the advisory is. The lower the number, the more urgent it is
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Priority {
    /// Priority of this advisory is very high
    VeryHigh = 1,

    /// Priority of this advisory is high
    High = 2,

    /// Priority of this advisory is medium
    #[default]
    Medium = 3,

    /// Priority of this advisory is low
    Low = 4,

    /// Priority of this advisory is very low
    VeryLow = 5,
}

/// Service advisories belong to a category
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Category {
    /// Only transit vehicles are effected
    Transit,

    /// Only Handi-Transit vehicles are effected
    #[serde(rename = "Handi-Transit")]
    HandiTransit,

    /// Both [Transit](Category::Transit) and [HandiTransit](Category::HandiTransit) are effected
    #[default]
    All,
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
