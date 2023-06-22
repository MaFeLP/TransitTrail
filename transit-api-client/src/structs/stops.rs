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
//! Data structures for the [stops endpoint](crate::endpoints::stops)
//!

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::PrimitiveDateTime;

use super::{
    common::{GeoLocation, Street},
    datetime_formatter, deserialize_from_string,
    routes::{Coverage, Variant},
};

/// A stop
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    /// A unique identifier for this stop.
    pub key: u32,

    /// The stop name
    pub name: String,

    /// The stop number
    pub number: u32,

    /// When a location was specified, these are the distances it takes
    /// to get to the stop.
    pub distances: Option<Distances>,

    /// Specifies which direction buses which service the stop are heading.
    pub direction: Direction,

    /// Specifies which side of the intersection the stop lies on.
    pub side: Side,

    /// The street on which the stop is located
    pub street: Street,

    /// The street that intersects the main [Street]
    #[serde(rename = "cross-street")]
    pub cross_street: Street,

    /// A geographical point describing where the stop is located.
    pub centre: GeoLocation,

    /// The internal name use by the API
    #[serde(rename = "internal-name")]
    pub internal_name: Option<String>,

    /// The how many-th stop on the street this stop is
    #[serde(rename = "sequence-on-street")]
    pub sequence_on_street: Option<u32>,

    /// What icon style to use
    #[serde(rename = "icon-style")]
    pub icon_style: Option<String>,
}

/// A Stops that only contains the minimum required.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialStop {
    /// The stop number
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: u32,

    /// Where the stop is located
    pub position: GeoLocation,

    /// What style/image the icon should have
    #[serde(rename = "iconStyle")]
    pub icon_style: PartialStopIconStyle,
}

/// What style a PartialStop has
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartialStopIconStyle {
    /// It is a normal stop, with the Blue Winnipeg Transit Logo
    #[default]
    Blue,

    /// The Partial Stop is a RapidTransit stop, with the rt-logo
    Rt,
}

/// Distances in meters to the stop
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Distances {
    /// The direct distance to the stop
    #[serde(deserialize_with = "deserialize_from_string")]
    pub direct: f32,

    /// The distance it takes to walk there
    #[serde(deserialize_with = "deserialize_from_string")]
    pub walking: f32,
}

impl Eq for Distances {}

/// Specifies which direction buses which service the stop are heading.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    /// The bus is going North
    Northbound,

    /// The bus is going East
    Eastbound,

    /// The bus is going South
    Southbound,

    /// The bus is going West
    Westbound,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Northbound => "Northbound",
                Direction::Eastbound => "Eastbound",
                Direction::Southbound => "Southbound",
                Direction::Westbound => "Westbound",
            }
        )
    }
}
/// Specifies which side of the intersection the stop lies on.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Side {
    /// The stop is directly on the opposite side
    ///
    /// **Example**: `10168`
    #[serde(rename = "Direct Opposite")]
    DirectOpposite,

    /// The stop is on the far side
    ///
    /// **Example**: `10095`
    Farside,

    /// The stop is on the far- and opposite side of the street
    ///
    /// **Example**: `10081`
    #[serde(rename = "Farside Opposite")]
    FarsideOpposite,

    /// The stop is on the nearside of the street
    ///
    /// **Example**: `10076`
    Nearside,

    /// The stop is on the near- and opposite side of the street
    ///
    /// **Example**: `10077`
    #[serde(rename = "Nearside Opposite")]
    NearsideOpposite,

    /// No side of the street available for this stop
    ///
    /// **Example**: `10087`
    #[default]
    NA,
}

/// information about any stop features
///
/// This includes: Benches, (Un-)heated shelters, etc.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    /// The name of the stop feature
    pub name: String,

    ///  The number of occurrences of the feature at this stop
    pub count: u32,
}

/// A schedule of what buses are leaving from this stop
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// The stop which the schedule information is for. See the [Stop] for more details.
    pub stop: Stop,

    /// A route schedule is returned for each route which services the stop.
    #[serde(rename = "route-schedules")]
    pub route_schedules: Vec<RouteSchedule>,
}

/// A route schedule of a route and where it is going.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteSchedule {
    /// Basic route information.
    pub route: FoxxRoute,

    /// Contains information about when a bus on the given route will pass by the stop.
    #[serde(rename = "scheduled-stops")]
    pub scheduled_stops: Vec<ScheduledStop>,
}

/// Contains information about when a bus will pass by the stop.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledStop {
    /// A unique identifier for this scheduled-stop.
    pub key: String,

    /// Boolean field describing whether or not this scheduled stop has been cancelled.
    #[serde(deserialize_with = "deserialize_from_string")]
    pub cancelled: bool,

    /// Times of when the bus is schedules/estimated to arrive/departure
    pub times: ScheduledTimes,

    /// The variant of the route which the passing bus belongs to. See the [Variant]
    /// for more details.
    pub variant: Variant,

    /// Information about the passing bus. Will typically be present in today's schedule results
    /// and omitted for past and future dates.
    pub bus: Option<Bus>,
}

/// Information about the arrival and departure times
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScheduledTimes {
    /// Times of when the bus is scheduled and estimated to arrive
    pub arrival: Time,

    /// Times of when the bus is scheduled and estimated to depart
    pub departure: Time,
}

/// Holds scheduled and estimated times for departure or arrival
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Time {
    /// When the bus is scheduled
    #[serde(with = "datetime_formatter")]
    pub scheduled: PrimitiveDateTime,

    /// When the bus is estimated
    #[serde(with = "datetime_formatter")]
    pub estimated: PrimitiveDateTime,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            scheduled: crate::UNIX_EPOCH,
            estimated: crate::UNIX_EPOCH,
        }
    }
}

/// Information about the passing bus. Will typically be present in today's schedule results
/// and omitted for past and future dates.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bus {
    /// A unique identifier for the bus.
    pub key: u32,

    /// Whether or not the bus has a bike rack
    #[serde(deserialize_with = "deserialize_from_string", rename = "bike-rack")]
    pub bike_rack: bool,

    /// Whether or not the bus has wifi
    #[serde(deserialize_with = "deserialize_from_string")]
    pub wifi: bool,
}

/// A busses route.
///
/// Author: [Foxx](mailto:f.pinkerton@sjsad.ca)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FoxxRoute {
    /// The bus key
    pub key: BusType,

    /// The bus number
    pub number: BusType,

    /// The bus name
    pub name: Option<String>,

    /// The customer's the bus services
    #[serde(rename = "customer-type")]
    pub customer_type: CustomerType,

    /// The bus coverage
    pub coverage: Coverage,

    /// The Badge Label
    #[serde(rename = "badge-label")]
    pub badge_label: BusType,

    /// The Badge Style
    #[serde(rename = "badge-style")]
    pub badge_style: BadgeStyle,

    /// The bus variants
    pub variants: Option<Vec<Variant>>,
}

/// The bus type
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BusType {
    /// The bus is a regular bus
    Regular(u32),

    /// The bus is a BLUE bus
    Blue(String),
}

impl Display for BusType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BusType::Regular(n) => n.to_string(),
                BusType::Blue(s) => s.to_string(),
            }
        )
    }
}

/// The customer's the bus services
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CustomerType {
    /// The bus is a regular bus
    #[serde(rename = "regular")]
    Regular,

    /// The bus is a HandiTransit bus
    HandiTransit,

    /// The bus is a DART bus
    DART,

    /// The bus is a School Charter bus
    School,

    /// The bus is a Community bus
    Community,

    /// The bus is a Express bus
    Express,

    /// The bus is a Rapid Transit bus
    RapidTransit,

    /// The bus is a Unknown bus
    Unknown,
}

/// Styling for the badge
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BadgeStyle {
    /// Classes for the badge
    #[serde(rename = "class-names")]
    pub class_names: ClassNamesHolder,

    /// The background color of the badge
    #[serde(rename = "background-color")]
    pub background_color: String,

    /// The border color of the badge
    #[serde(rename = "border-color")]
    pub border_color: String,

    /// The color of the badge
    pub color: String,
}

/// A Struct just holding the field class_name
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassNamesHolder {
    /// The class name
    #[serde(rename = "class-name")]
    pub class_name: Vec<FoxxClassNames>,
}

/// class names
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FoxxClassNames {
    /// Lable for the badge
    #[serde(rename = "badge-label")]
    BadgeLable,

    /// Express bus
    #[serde(rename = "express")]
    Express,

    /// Regular bus
    #[serde(rename = "regular")]
    Regular,

    /// Rapid Transit bus
    #[serde(rename = "rapid-transit")]
    RapidTransit,

    /// A feeder bus for express and rapid transit busses
    #[serde(rename = "feeder")]
    Feeder,

    /// A feeder bus for express and rapid transit busses, which is used at peak times
    #[serde(rename = "peak-feeder")]
    PeakFeeder,
}

impl Display for FoxxClassNames {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FoxxClassNames::BadgeLable => "badge-label",
                FoxxClassNames::Express => "express",
                FoxxClassNames::Regular => "regular",
                FoxxClassNames::RapidTransit => "rapid-transit",
                FoxxClassNames::Feeder => "feeder",
                FoxxClassNames::PeakFeeder => "peak-feeder",
            }
        )
    }
}
