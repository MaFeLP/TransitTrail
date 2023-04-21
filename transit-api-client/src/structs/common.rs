//!
//! Structures used in multiple endpoints.
//!

use std::fmt::Display;

use crate::structs::UrlParameter;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{Map, Number, Value};

/// A point on the Earth: A geographic location, represented by longitude and latitude.
///
/// Winnipeg is roughly in the bounds:
///
/// Latitude: North = 49.97; South = 49.75
/// Longitude: East = -96.96; West
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GeoLocation {
    /// The latitude of the point
    pub latitude: f64,

    /// The longitude of the point.
    pub longitude: f64,
}

impl GeoLocation {
    /// Create a new instance of a Geolocation in shorter form.
    ///
    /// Default constructor
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
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
            fn number_to_f64(number: &Number) -> f64 {
                if let Some(n) = number.as_f64() {
                    return n;
                }
                if let Some(n) = number.as_u64() {
                    return n as f64;
                }
                if let Some(n) = number.as_i64() {
                    return n as f64;
                }
                panic!(
                    "[transit-api-client] Something went wrong with converting Number to f64! {:?}",
                    number
                );
            }
            // the longitude and latitude fields are stored with quotes, so directly asking for
            // them as a float, would error out.
            let latitude: f64 = match map.get("lat").unwrap() {
                Value::String(s) => s.parse().map_err(Error::custom)?,
                Value::Number(n) => number_to_f64(n),
                _ => return Err(Error::custom("field `lat` is not of type `str` or `f64`")),
            };
            let longitude: f64 = match map.get("lng").unwrap() {
                Value::String(s) => s.parse().map_err(Error::custom)?,
                Value::Number(n) => number_to_f64(n),
                _ => return Err(Error::custom("field `lng` is not of type `str` or `f64`")),
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

impl From<GeoLocation> for UrlParameter {
    fn from(value: GeoLocation) -> Self {
        Self(format!("&lat={}&lon={}", value.latitude, value.longitude))
    }
}

/// Locations tagged with "type": TYPE in the JSON response. They represent a
/// position or a point on the map that is significant or by address.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    /// The address of a Location
    #[serde(rename = "address")]
    Address(Address),

    /// The location is a significant point of interest
    #[serde(rename = "monument")]
    Monument(Monument),

    /// The location is at an intersection of two streets
    #[serde(rename = "intersection")]
    Intersection(Intersection),

    /// A geographic point
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

/// Represents a Street, as it is returned from the API
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Street {
    /// The unique key of the street
    pub key: u32,

    /// The name of the street.
    ///
    /// Can be more or less verbose, if [Usage](super::Usage) is not set to
    /// [Usage::Normal](super::Usage::Normal) in the request.
    pub name: String,

    /// Optionally a Street Type may be specified, e.g. Road, Boulevard, Street, etc.
    #[serde(rename = "type")]
    pub street_type: Option<StreetType>,

    /// If this street is split into more than one parts, a street leg is given
    pub leg: Option<StreetLeg>,
}

/// What type of street it actually is
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetType {
    /// The street is an avenue (Ave)
    Avenue,

    /// The street is a boulevard (Blvd)
    Boulevard,

    /// The street is a crescent (Cres)
    Crescent,

    /// The street is a drive (Dr)
    Drive,

    /// The street is a bus loop
    Loop,

    /// The street is a road (Rd)
    Road,

    /// The street is a street (St)
    #[default]
    Street,

    /// The street is a way (Wy)
    Way,
}

/// The part of the street if it is split up in more than one parts
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StreetLeg {
    /// The North part of the street (N)
    North,

    /// The East part of the street (E)
    East,

    /// The South part of the street (S)
    South,

    /// The West part of the street (W)
    West,
}

/// A residential address
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The unique key of the address
    pub key: u32,

    /// What street the address is located on
    pub street: Street,

    /// The house number/street number of the address
    #[serde(rename = "street-number")]
    pub street_number: u32,

    /// The geographic centre of the address
    pub centre: GeoLocation,
}

/// A significant point of interest
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Monument {
    /// The unique key of the point of interest
    pub key: u32,

    /// What the point of interest is called
    pub name: String,

    /// Which categories the point of interest has
    pub categories: Vec<String>,

    /// The address of the point of interest
    pub address: Address,
}

/// The intersection of two streets
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Intersection {
    /// The unique key of the intersection. Composed of the unique keys of the two streets
    pub key: String,

    /// The main street of the crossing streets
    pub street: Street,

    /// The street crossing the main street
    #[serde(rename = "cross-street")]
    pub cross_street: Street,

    /// The geographic centre of the intersection
    pub centre: GeoLocation,
}
