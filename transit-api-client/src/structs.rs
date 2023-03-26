use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub(crate) const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

#[derive(Debug, Default)]
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
    Loop,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetLeg {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl Eq for GeoLocation {}

impl<'de> serde::de::Deserialize<'de> for GeoLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = <serde_json::Map<String, serde_json::Value>>::deserialize(deserializer)?;

        // Somehow, this function gets called twice for the deserialization...
        // The first time with a map that contains the "geolocation" key, the second time with the
        // longitude and latitude fields.
        if map.contains_key(&String::from("latitude"))
            && map.contains_key(&String::from("longitude"))
        {
            // the longitude and latitude fields are stored with quotes, so directly asking for
            // them as a float, would error out.
            let latitude: f64 = match map.get("latitude").unwrap().as_str() {
                Some(l) => l.parse().map_err(D::Error::custom)?,
                None => return Err(D::Error::custom("field `latitude` is not of type  `str`")),
            };
            let longitude: f64 = match map.get("longitude").unwrap().as_str() {
                Some(l) => l.parse().map_err(D::Error::custom)?,
                None => return Err(D::Error::custom("field `longitude` is not of type `str`")),
            };

            return Ok(Self {
                latitude,
                longitude,
            });
        }

        let geographic: &serde_json::Value = map
            .get("geographic")
            .ok_or(D::Error::missing_field("geographic"))?;
        let out: GeoLocation =
            serde_json::from_value(geographic.clone()).map_err(D::Error::custom)?;

        Ok(out)
    }
}

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
    pub centre: GeoLocation,
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
    pub centre: GeoLocation,
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
    // Is always set on the 'routes' endpoint, but not set in the 'stops' endpoint
    pub variants: Option<Vec<RouteVariante>>,
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
    pub variants: Option<Vec<RouteVariante>>,
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

// Stops.rs
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub key: u32,
    pub name: String,
    pub number: u32,
    pub direction: StopDirection,
    pub side: StopSide,
    pub street: Street,
    #[serde(rename = "cross-street")]
    pub cross_street: Street,
    pub centre: GeoLocation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StopDirection {
    Northbound,
    Eastbound,
    Southbound,
    Westbound,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StopSide {
    #[serde(rename = "Direct Opposite")]
    DirectOpposite, // stop: 10168
    Farside, //  stop: 10095
    #[serde(rename = "Farside Opposite")]
    FarsideOpposite, // stop: 10081
    Nearside, // stop: 10076
    #[serde(rename = "Nearside Opposite")]
    NearsideOpposite, // stop: 10077
    NA,      // stop: 10087
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StopFeature {
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StopSchedule {
    pub stop: Stop,
    #[serde(rename = "route-schedules")]
    pub route_schedules: Vec<RouteSchedule>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteSchedule {
    pub route: Route,
    #[serde(rename = "scheduled-stops")]
    pub scheduled_stops: Vec<ScheduledStop>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledStop {
    pub key: String,
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub cancelled: bool,
    pub times: ScheduledTimes,
    pub variant: RouteVariante,
    pub bus: Bus,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTimes {
    pub arrival: Time,
    pub departure: Time,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub scheduled: NaiveDateTime,
    pub estimated: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bus {
    pub key: u32,
    #[serde(deserialize_with = "deserialize_string_to_bool", rename = "bike-rack")]
    pub bike_rack: bool,
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub wifi: bool,
}

fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <serde_json::Value>::deserialize(deserializer)?;
    let string_value = value.as_str().ok_or(D::Error::custom("unknown type"))?;
    let bool_value: bool = string_value.parse().map_err(D::Error::custom)?;

    Ok(bool_value)
}
