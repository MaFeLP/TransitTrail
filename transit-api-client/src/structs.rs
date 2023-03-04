use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Street {
    pub key: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub street_type: Option<StreetType>,
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
