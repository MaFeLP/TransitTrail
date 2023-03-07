use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub enum Usage {
    Normal,
    Long,
    Short,
}

impl Usage {
    pub(crate) fn to_url_parameter(&self) -> String {
        match self {
            Usage::Normal => "".to_string(),
            Usage::Long => "&usage=long".to_string(),
            Usage::Short => "&usage=short".to_string(),
        }
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
