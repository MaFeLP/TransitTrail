//!
//! Contains data structures exclusively used in the
//! [trip_planner](crate::TransitClient::trip_planner) endpoint
//!

pub mod segment;

use google_maps_api_client::DirectionsLeg;
use serde::{Deserialize, Serialize};
use time::macros::offset;
use time::{OffsetDateTime, PrimitiveDateTime};

use super::{
    common::{Address, GeoLocation, Intersection, Monument},
    datetime_formatter,
};

pub use segment::*;

/// Each plan describes a different trip or path which can be used to get from the origin to
/// the destination.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    /// Contains start and end times of the plan or segment, including the total duration in
    /// minutes. Riding, walking, and waiting totals are also included where appropriate.
    pub times: Times,

    /// Information about how this plan is structured
    pub segments: Vec<Segment>,
}

impl From<DirectionsLeg> for Plan {
    fn from(leg: DirectionsLeg) -> Self {
        let start_time = OffsetDateTime::from_unix_timestamp(leg.departure_time.unwrap().value)
            .unwrap()
            .to_offset(offset!(-5));
        let end_time = OffsetDateTime::from_unix_timestamp(leg.arrival_time.unwrap().value)
            .unwrap()
            .to_offset(offset!(-5));
        let mut times = Times {
            start: PrimitiveDateTime::new(start_time.date(), start_time.time()),
            end: PrimitiveDateTime::new(end_time.date(), end_time.time()),
            durations: Durations::default(),
        };
        let segments: Vec<Segment> = leg.steps.into_iter().map(|step| step.into()).collect();

        for segment in &segments {
            match segment {
                Segment::Walk(walk) => {
                    times.durations.walking += walk.times.durations.walking;
                    times.durations.total += walk.times.durations.total;
                }
                Segment::Ride(ride) => {
                    times.durations.riding += ride.times.durations.riding;
                    times.durations.total += ride.times.durations.total;
                }
                _ => {}
            }
        }

        Plan { times, segments }
    }
}

/// Time information about the [Plan]/[Segment]: when it starts/ends and how much time is
/// spent with what.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Times {
    /// When the ride/walk of the plan/segment starts
    #[serde(with = "datetime_formatter")]
    pub start: PrimitiveDateTime,

    /// When the ride/walk of the plan/segment end
    #[serde(with = "datetime_formatter")]
    pub end: PrimitiveDateTime,

    /// How much time is spent on different transport options (walking, riding, waiting, total time)
    pub durations: Durations,
}

impl Default for Times {
    fn default() -> Self {
        Self {
            start: crate::UNIX_EPOCH,
            end: crate::UNIX_EPOCH,
            durations: Default::default(),
        }
    }
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
    Origin(Location),

    /// The segment starts/ends neither at the start, nor at the end of the [Plan].
    ///
    /// Only includes basic information. Call [stop_info](crate::TransitClient::stop_info)
    /// to retrieve the remaining information of the stop.
    ///
    /// # Example
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// # tokio_test::block_on(async move {
    /// let plans = client.trip_planner(
    ///     PartialLocation::Point(49.86917, -97.1391),
    ///     PartialLocation::Point(49.8327, -97.10887),
    ///     Vec::new(),
    ///     Usage::Normal
    /// ).await.unwrap();
    /// let plan = plans.get(0).unwrap();
    /// let segment = plan.segments.get(0).unwrap();
    ///
    /// match segment {
    ///     trip::Segment::Walk(walk) => {
    ///         if let Some(to) = &walk.to {
    ///             match to {
    ///                 trip::TripStop::Stop(stop) => {
    ///                     // This is what we actually care about:
    ///                     // Get the other required information of the stop
    ///                     let stop_complete = client.stop_info(stop.key, Usage::Normal).await.unwrap();
    ///                     println!("{:?}", stop_complete);
    ///                 },
    ///                 _ => { /* handle other types */ }
    ///             }
    ///        }
    ///     },
    ///     _ => { /* handle other types */ },
    /// }
    /// # });
    /// ```
    #[serde(rename = "stop")]
    Stop(Stop),

    /// The segment ends at the [Plan]'s destination
    #[serde(rename = "destination")]
    Destination(Location),
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
