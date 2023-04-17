//!
//! Data structures for the [stops endpoint](crate::endpoints::stops)
//!

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use super::{
    common::{GeoLocation, Street},
    datetime_formatter, deserialize_string_to_bool, deserialize_string_to_float,
    routes::{Route, Variant},
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
}

/// Distances in meters to the stop
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Distances {
    /// The direct distance to the stop
    #[serde(deserialize_with = "deserialize_string_to_float")]
    pub direct: f32,

    /// The distance it takes to walk there
    #[serde(deserialize_with = "deserialize_string_to_float")]
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteSchedule {
    /// Basic route information.
    pub route: Route,

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
    #[serde(deserialize_with = "deserialize_string_to_bool")]
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
    #[serde(deserialize_with = "deserialize_string_to_bool", rename = "bike-rack")]
    pub bike_rack: bool,

    /// Whether or not the bus has wifi
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub wifi: bool,
}
