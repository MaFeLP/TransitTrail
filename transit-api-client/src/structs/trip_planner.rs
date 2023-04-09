//!
//! Contains data structures exclusively used in the
//! [trip_planner](crate::TransitClient::trip_planner) endpoint
//!

use std::fmt::Display;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use super::{
    common::{Address, GeoLocation, Intersection, Monument},
    routes::{Route, Variant},
    stops::Bus,
    UrlParameter,
};

/// Specify filters for the trip planning
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Filter {
    /// The date for which to get navigo results. Defaults to today, if not included as a filter
    Date(NaiveDate),

    /// The time of the trip. Defaults to now, if not included as a filter.
    ///
    /// What the time means can be customized with a [Mode]
    Time(NaiveTime),

    /// The mode with which the trip should be planned
    ///
    /// What the time applies to: If the time specifies where to be when, or when to leave
    Mode(Mode),

    /// Walking speed in km/h.
    WalkSpeed(f32),

    /// The maximum number of minutes to spend walking.
    MaxWalkTime(i32),

    /// The minimum number of minutes to spend waiting for a transfer.
    MinTransferWait(i32),

    /// The maximum number of minutes to spend waiting for a transfer.
    MaxTransferWait(i32),

    /// The maximum number of total transfers.
    MaxTransfers(i32),
}

impl From<Filter> for UrlParameter {
    fn from(value: Filter) -> Self {
        Self(match value {
            Filter::Date(d) => format!("&date={}", d.format("%Y-%m-%d")),
            Filter::Time(t) => format!("&time={}", t.format("%H:%M:%S")),
            Filter::Mode(m) => format!("&mode={}", m),
            Filter::WalkSpeed(s) => format!("&walk-speed={}", s),
            Filter::MaxWalkTime(t) => format!("&max-walk-time={}", t),
            Filter::MinTransferWait(t) => format!("&min-transfer-wait={}", t),
            Filter::MaxTransferWait(t) => format!("&ax-transfer-wait={}", t),
            Filter::MaxTransfers(t) => format!("&max-transfers={}", t),
        })
    }
}

/// What the time applies to: If the time specifies where to be when, or when to leave
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Mode {
    /// Depart before the given time.
    DepartBefore,

    /// Depart after the given time.
    #[default]
    DepartAfter,

    /// Arrive before the given time.
    ArriveBefore,

    /// Arrive after the given time.
    ArriveAfter,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DepartBefore => write!(f, "depart-before"),
            Self::DepartAfter => write!(f, "depart-after"),
            Self::ArriveBefore => write!(f, "arrive-before"),
            Self::ArriveAfter => write!(f, "depart-after"),
        }
    }
}

/// Each plan describes a different trip or path which can be used to get from the origin to
/// the destination.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    /// The how many-th plan this is
    pub number: u32,

    /// Contains start and end times of the plan or segment, including the total duration in
    /// minutes. Riding, walking, and waiting totals are also included where appropriate.
    pub times: Times,

    /// Information about how this plan is structured
    pub segments: Vec<Segment>,
}

/// Time information about the [Plan]/[Segment]: when it starts/ends and how much time is
/// spent with what.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Times {
    /// When the ride/walk of the plan/segment starts
    pub start: NaiveDateTime,

    /// When the ride/walk of the plan/segment end
    pub end: NaiveDateTime,

    /// How much time is spent on different transport options (walking, riding, waiting, total time)
    pub durations: Durations,
}

/// Times for how long is spent riding/walking/waiting and total
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Durations {
    /// Total time spent. Defaults to 0
    #[serde(default)]
    pub total: u32,

    /// Total time spent walking. Defaults to 0
    #[serde(default)]
    pub walking: u32,

    /// Total time spent waiting. Defaults to 0
    #[serde(default)]
    pub waiting: u32,

    /// Total time spent riding on buses. Defaults to 0
    #[serde(default)]
    pub riding: u32,
}

/// Segments can either be of type [SegmentWalk], [SegmentRide] or [SegmentTransfer]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Segment {
    // TODO refactor into their own submodule `segment`
    /// The segment is of type [SegmentWalk]
    #[serde(rename = "walk")]
    Walk(SegmentWalk),

    /// The segment is of type [SegmentRide]
    #[serde(rename = "ride")]
    Ride(SegmentRide),

    /// The segment is of type [SegmentTransfer]
    #[serde(rename = "transfer")]
    Transfer(SegmentTransfer),
}

/// Information about a walking route
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SegmentWalk {
    /// Shows the boundaries of the trip
    pub bounds: Bounds,

    /// Indicates whether the walk path starts at the origin of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub from: TripStop,

    /// Individual times for walking and total. Includes default (0) values for all other fields.
    pub times: Times,

    /// Indicates whether the walk path ends at the destination of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub to: TripStop,
}

/// Information about a riding route
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SegmentRide {
    /// Shows the boundaries of the trip
    pub bounds: Bounds,

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
pub struct SegmentTransfer {
    /// Shows the boundaries of the trip
    pub bounds: Bounds,

    /// Indicates whether the walk path starts at the origin of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub from: TripStop,

    /// Individual times for walking and total. Includes default (0) values for all other fields.
    pub times: Times,

    /// Indicates whether the walk path ends at the destination of the trip, or at a stop.
    /// Contains location elements, or point elements which define a geographical point.
    pub to: TripStop,
}

/// The geographic boundaries of the segment/plan
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    /// The maximum point
    pub maximum: GeoLocation,

    /// The minimum point
    pub minimum: GeoLocation,
}

/// Differentiate between stops at the origin, a stop, or the end of the trip
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TripStop {
    /// The segment starts at the origin of the [Plan]
    #[serde(rename = "origin")]
    Origin(TripLocation),

    /// The segment starts/ends neither at the start, nor at the end of the [Plan].
    ///
    /// Only includes basic information. Call [stop_info](crate::TransitClient::stop_info)
    /// to retrieve the remaining information of the stop.
    ///
    /// # Example
    /// ```
    /// # use transit_api_client::{
    /// #     structs::{
    /// #         common::{GeoLocation, Location},
    /// #         trip_planner::{Segment, TripStop},
    /// #         Usage,
    /// #     },
    /// #     TransitClient,
    /// # };
    /// # let client = transit_api_client::testing_client();
    /// // use ...
    /// // let client = ...;
    /// # tokio_test::block_on(async move {
    /// let plans = client.trip_planner(
    ///     Location::Point(GeoLocation::new(49.86917, -97.1391)),
    ///     Location::Point(GeoLocation::new(49.8327, -97.10887)),
    ///     Vec::new(),
    ///     Usage::Normal
    /// ).await.unwrap();
    /// let plan = plans.get(0).unwrap();
    /// let segment = plan.segments.get(0).unwrap();
    ///
    /// match segment {
    ///     Segment::Walk(walk) => {
    ///         match &walk.to {
    ///             TripStop::Stop(stop) => {
    ///                 // This is what we actually care about:
    ///                 // Get the other required information of the stop
    ///                 let stop_complete = client.stop_info(stop.key, Usage::Normal).await.unwrap();
    ///                 println!("{:?}", stop_complete);
    ///             },
    ///             _ => { /* handle other types */ }
    ///         }
    ///     },
    ///     _ => { /* handle other types */ },
    /// }
    /// # });
    /// ```
    #[serde(rename = "stop")]
    Stop(Stop),

    /// The segment ends at the [Plan]'s destination
    #[serde(rename = "destination")]
    Destination(TripLocation),
}

impl Default for TripStop {
    fn default() -> Self {
        Self::Stop(Stop::default())
    }
}

/// A representation of [Location](crate::structs::common::Location), that is serialized and
/// deserialized as an untagged enum.
/// It represents a position or a point on the map that is significant or by address.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TripLocation {
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

/// Basic information about a stop on the Trip.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    /// A unique identifier for this stop.
    pub key: u32,

    /// The stop name
    pub name: String,

    /// A geographical point describing where the stop is located.
    pub centre: GeoLocation,
}
