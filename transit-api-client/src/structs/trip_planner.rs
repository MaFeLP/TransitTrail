use super::{
    common::{Address, GeoLocation, Intersection, Monument},
    routes::{Route, RouteVariante},
    stops::Bus,
    UrlParameter,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum TripFilter {
    Date(NaiveDate),
    Time(NaiveTime),
    Mode(TripMode),
    WalkSpeed(f32),
    MaxWalkTime(i32),
    MinTransferWait(i32),
    MaxTransferWait(i32),
    MaxTransfers(i32),
}

impl From<TripFilter> for UrlParameter {
    fn from(value: TripFilter) -> Self {
        Self(match value {
            TripFilter::Date(d) => format!("&date={}", d.format("%Y-%m-%d")),
            TripFilter::Time(t) => format!("&time={}", t.format("%H:%M:%S")),
            TripFilter::Mode(m) => format!("&mode={}", m),
            TripFilter::WalkSpeed(s) => format!("&walk-speed={}", s),
            TripFilter::MaxWalkTime(t) => format!("&max-walk-time={}", t),
            TripFilter::MinTransferWait(t) => format!("&min-transfer-wait={}", t),
            TripFilter::MaxTransferWait(t) => format!("&ax-transfer-wait={}", t),
            TripFilter::MaxTransfers(t) => format!("&max-transfers={}", t),
        })
    }
}

#[derive(Debug)]
pub enum TripMode {
    DepartBefore,
    DepartAfter,
    ArriveBefore,
    ArriveAfter,
}

impl Display for TripMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DepartBefore => write!(f, "depart-before"),
            Self::DepartAfter => write!(f, "depart-after"),
            Self::ArriveBefore => write!(f, "arrive-before"),
            Self::ArriveAfter => write!(f, "depart-after"),
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripPlan {
    pub number: u32,
    pub times: TripTimes,
    pub segments: Vec<TripSegment>,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripTimes {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub durations: TripDurations,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripDurations {
    #[serde(default)]
    pub total: u32,
    #[serde(default)]
    pub walking: u32,
    #[serde(default)]
    pub waiting: u32,
    #[serde(default)]
    pub riding: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TripSegment {
    // No field "type"
    #[serde(rename = "walk")]
    Walk(TripSegmentWalk),
    #[serde(rename = "ride")]
    Ride(TripSegmentRide),
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripSegmentWalk {
    pub bounds: TripBounds,
    pub from: TripStop,
    pub times: TripTimes,
    pub to: TripStop,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripSegmentRide {
    pub bounds: TripBounds,
    pub bus: Bus,
    pub route: Route,
    pub times: TripTimes,
    pub variant: RouteVariante,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripBounds {
    pub maximum: GeoLocation,
    pub minimum: GeoLocation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TripStop {
    #[serde(rename = "origin")]
    Origin(TripLocation),
    #[serde(rename = "stop")]
    Stop(TripStopStop),
    #[serde(rename = "destination")]
    Destination(TripLocation),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TripLocation {
    #[serde(rename = "address")]
    Address(Address),
    #[serde(rename = "monument")]
    Monument(Monument),
    #[serde(rename = "intersection")]
    Intersection(Intersection),
    #[serde(rename = "point")]
    Point(GeoLocation), // Can be used in trip planner
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TripStopStop {
    pub centre: GeoLocation,
    pub key: u32,
    pub name: String,
}

impl Default for TripStop {
    fn default() -> Self {
        Self::Stop(TripStopStop::default())
    }
}
