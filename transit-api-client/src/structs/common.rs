// TransitTrail - Navigate Winnipeg Transit with a different style
// Copyright (C) - 2023 Foxx Azalea Pinkerton, Max Fehlinger
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

//!
//! Structures used in multiple endpoints.
//!

use crate::prelude::Stop;
use crate::structs::UrlParameter;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::fmt::{Display, Formatter};

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

    /// A bus stop
    #[serde(rename = "stop")]
    Stop(Stop),
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
            Self::Stop(s) => write!(f, "stops/{}", s.key),
        }
    }
}

/// Used when creating a trip plan, to not have to query for the whole
/// structure of a Location, but instead only use the key.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum PartialLocation<'a> {
    /// The address of a Location
    Address(&'a str),

    /// The location is a significant point of interest
    Monument(&'a str),

    /// The location is at an intersection of two streets
    Intersection(&'a str),

    /// A geographic point, representing latitude and longitude
    Point(f64, f64),

    /// A stop with its id
    Stop(u32),
}

impl Default for PartialLocation<'_> {
    fn default() -> Self {
        Self::Point(Default::default(), Default::default())
    }
}

impl Display for PartialLocation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address(key) => write!(f, "addresses/{}", key),
            Self::Monument(key) => write!(f, "monuments/{}", key),
            Self::Intersection(key) => write!(f, "intersections/{}", key),
            Self::Point(lat, lon) => write!(f, "geo/{},{}", lat, lon),
            Self::Stop(key) => write!(f, "stops/{}", key),
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
    pub street_type: Option<String>,

    /// If this street is split into more than one parts, a street leg is given
    pub leg: Option<StreetLeg>,
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

impl TryFrom<&str> for StreetLeg {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "north" => Ok(Self::North),
            "east" => Ok(Self::East),
            "south" => Ok(Self::South),
            "west" => Ok(Self::West),
            _ => Err("Not equal to `north`, `east`, `south`, `west`"),
        }
    }
}

impl Display for StreetLeg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreetLeg::North => write!(f, "North"),
            StreetLeg::East => write!(f, "East"),
            StreetLeg::South => write!(f, "South"),
            StreetLeg::West => write!(f, "West"),
        }
    }
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

#[cfg(test)]
mod test {
    use crate::structs::common::StreetLeg;
    use tokio_test::assert_err;

    #[test]
    fn try_from() -> Result<(), &'static str> {
        assert_eq!(StreetLeg::try_from("East")?, StreetLeg::East);
        assert_err!(StreetLeg::try_from("not a valid street leg"));
        Ok(())
    }
}
