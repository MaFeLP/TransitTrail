use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug)]
pub(crate) struct UrlParameter(String);

impl std::fmt::Display for UrlParameter {
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

// destinations.rs
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Destination {
    pub key: u32,
    pub name: String,
}

// locations.rs
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Street {
    pub key: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub street_type: Option<StreetType>,
    pub leg: Option<StreetLeg>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetType {
    Street,
    Avenue,
    Road,
    Drive,
    Crescent,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetLeg {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeographicLocation {
    pub geographic: GeoLocation,
}

impl From<GeoLocation> for GeographicLocation {
    fn from(value: GeoLocation) -> Self {
        Self { geographic: value }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: String,
    pub longitude: String,
}

impl From<GeographicLocation> for GeoLocation {
    fn from(value: GeographicLocation) -> Self {
        value.geographic
    }
}

impl Eq for GeoLocation {}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    #[serde(rename = "address")]
    Address(Address),
    #[serde(rename = "monument")]
    Monument(Monument),
    #[serde(rename = "intersection")]
    Intersection(Intersection),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub key: u32,
    pub street: Street,
    #[serde(rename = "street-number")]
    pub street_number: u32,
    pub centre: GeographicLocation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Monument {
    pub key: u32,
    pub name: String,
    pub categories: Vec<String>,
    pub address: Address,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Intersection {
    pub key: String,
    pub street: Street,
    #[serde(rename = "cross-street")]
    pub cross_street: Street,
    pub centre: GeographicLocation,
}

// Routes.rs
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteRegular {
    pub key: u32,
    pub number: u32,
    pub name: String,
    #[serde(rename = "customer-type")]
    pub customer_type: RouteCustomer,
    pub coverage: RouteCoverage,
    #[serde(rename = "badge-label")]
    pub badge_label: u32,
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,
    pub variants: Vec<RouteVariante>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteBlue {
    pub key: String,
    pub number: String,
    #[serde(rename = "customer-type")]
    pub customer_type: RouteCustomer,
    pub coverage: RouteCoverage,
    #[serde(rename = "badge-label")]
    pub badge_label: String,
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,
    pub variants: Vec<RouteVariante>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Route {
    Blue(RouteBlue),
    Regular(RouteRegular),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RouteCustomer {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "industrial")]
    Industrial,
    #[serde(rename = "school")]
    School,
    #[serde(rename = "charter")]
    Charter,
    #[serde(rename = "work")]
    Work,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RouteCoverage {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "express")]
    Express,
    #[serde(rename = "super express")]
    SuperExpress,
    #[serde(rename = "rapid transit")]
    RapidTransit,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteVariante {
    pub key: String,
    pub name: Option<String>,
}

pub mod badges {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Style {
        #[serde(rename = "class-names")]
        pub class_names: ClassNames,
        #[serde(rename = "background-color")]
        pub background_color: String,
        #[serde(rename = "border-color")]
        pub border_color: String,
        pub color: String,
    }

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct ClassNames {
        #[serde(rename = "class-name")]
        pub class_name: Vec<String>,
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceAdvisory {
    pub key: u32,
    pub priority: ServiceAdvisoryPriority,
    pub title: String,
    pub body: String,
    pub category: ServiceAdvisoryCategory,
    #[serde(rename = "updated-at")]
    pub updated_at: chrono::NaiveDateTime,
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServiceAdvisoryCategory {
    Transit,
    #[serde(rename = "Handi-Transit")]
    HandiTransit,
    All,
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

impl Default for ServiceAdvisoryCategory {
    fn default() -> Self {
        Self::All
    }
}
