use serde::{de::Error, Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Serialize)]
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
        let map: Map<String, Value> = Map::deserialize(deserializer)?;

        // Somehow, this function gets called twice for the deserialization...
        // The first time with a map that contains the "geolocation" key, the second time with the
        // longitude and latitude fields.
        if map.contains_key("latitude") && map.contains_key("longitude") {
            // the longitude and latitude fields are stored with quotes, so directly asking for
            // them as a float, would error out.
            let latitude: f64 = match map.get("latitude").unwrap().as_str() {
                Some(l) => l.parse().map_err(Error::custom)?,
                None => return Err(Error::custom("field `latitude` is not of type  `str`")),
            };
            let longitude: f64 = match map.get("longitude").unwrap().as_str() {
                Some(l) => l.parse().map_err(Error::custom)?,
                None => return Err(Error::custom("field `longitude` is not of type `str`")),
            };

            return Ok(Self {
                latitude,
                longitude,
            });
        }

        if map.contains_key("lat") && map.contains_key("lng") {
            // the longitude and latitude fields are stored with quotes, so directly asking for
            // them as a float, would error out.
            let latitude: f64 = match map.get("lat").unwrap().as_str() {
                Some(l) => l.parse().map_err(Error::custom)?,
                None => return Err(Error::custom("field `lat` is not of type  `str`")),
            };
            let longitude: f64 = match map.get("lng").unwrap().as_str() {
                Some(l) => l.parse().map_err(Error::custom)?,
                None => return Err(Error::custom("field `lng` is not of type `str`")),
            };

            return Ok(Self {
                latitude,
                longitude,
            });
        }

        if map.contains_key("centre") {
            let centre: &Value = map.get("centre").unwrap();
            return serde_json::from_value::<GeoLocation>(centre.clone()).map_err(Error::custom);
        }

        let geographic: &Value = map
            .get("geographic")
            .ok_or(Error::missing_field("geographic"))?;
        let out: GeoLocation = serde_json::from_value(geographic.clone()).map_err(Error::custom)?;

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
    #[serde(rename = "point")]
    Point(GeoLocation),
}

impl Default for Location {
    fn default() -> Self {
        Self::Point(GeoLocation::default())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address(a) => write!(f, "addresses/{}", a.key),
            Self::Monument(m) => write!(f, "monuments/{}", m.key),
            Self::Intersection(i) => write!(f, "intersections/{}", i.key),
            Self::Point(p) => write!(f, "geo/{},{}", p.latitude, p.longitude),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Street {
    pub key: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub street_type: Option<StreetType>,
    pub leg: Option<StreetLeg>,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetType {
    Avenue,
    Boulevard,
    Crescent,
    Drive,
    Loop,
    Road,
    #[default]
    Street,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetLeg {
    North,
    East,
    South,
    West,
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
