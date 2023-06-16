//!
//! This module holds structs used in the Trip Planner, in individual segments;
//!

use crate::structs::{
    routes::{Route, Variant},
    stops::Bus,
    trip_planner::{Bounds, Times, TripStop},
};
use serde::{Deserialize, Serialize};

/// Segments can either be of type [Walk], [Ride] or [Transfer]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Segment {
    /// The segment is of type [Walk]
    #[serde(rename = "walk")]
    Walk(Walk),

    /// The segment is of type [Ride]
    #[serde(rename = "ride")]
    Ride(Ride),

    /// The segment is of type [Transfer]
    #[serde(rename = "transfer")]
    Transfer(Transfer),
}

/// Information about a walking route
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Walk {
    /// Shows the boundaries of the trip
    pub bounds: Option<Bounds>,

    /// Indicates whether the walk path starts at the origin of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub from: Option<TripStop>,

    /// Individual times for walking and total. Includes default (0) values for all other fields.
    pub times: Times,

    /// Indicates whether the walk path ends at the destination of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub to: Option<TripStop>,
}

/// Information about a riding route
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ride {
    /// Shows the boundaries of the trip
    pub bounds: Option<Bounds>,

    /// Information about the bus servicing this segment.
    /// Typically present in plans for today but omitted for past and future dates.
    pub bus: Option<Bus>,

    /// The route this bus takes
    pub route: Route,

    /// Individual times for walking and total. Includes default (0) values for all other fields.
    pub times: Times,

    /// The variant of the bus that is servicing this route
    pub variant: Variant,
}

/// Information about a transfer
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// Shows the boundaries of the trip
    pub bounds: Option<Bounds>,

    /// Indicates whether the walk path starts at the origin of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub from: TripStop,

    /// Individual times for walking and total. Includes default (0) values for all other fields.
    pub times: Times,

    /// Indicates whether the walk path ends at the destination of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub to: TripStop,
}
