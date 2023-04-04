use super::{
    common::{Address, GeoLocation, Intersection, Monument},
    routes::{Route, Variant},
    stops::Bus,
    UrlParameter,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum Filter {
    Date(NaiveDate),
    Time(NaiveTime),
    Mode(Mode),
    WalkSpeed(f32),
    MaxWalkTime(i32),
    MinTransferWait(i32),
    MaxTransferWait(i32),
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

#[derive(Debug)]
pub enum Mode {
    DepartBefore,
    DepartAfter,
    ArriveBefore,
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

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub number: u32,
    pub times: Times,
    pub segments: Vec<Segment>,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Times {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub durations: Durations,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Durations {
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
pub enum Segment {
    #[serde(rename = "walk")]
    Walk(SegmentWalk),
    #[serde(rename = "ride")]
    Ride(SegmentRide),
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SegmentWalk {
    pub bounds: Bounds,
    pub from: TripStop,
    pub times: Times,
    pub to: TripStop,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SegmentRide {
    pub bounds: Bounds,
    pub bus: Bus,
    pub route: Route,
    pub times: Times,
    pub variant: Variant,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    pub maximum: GeoLocation,
    pub minimum: GeoLocation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TripStop {
    #[serde(rename = "origin")]
    Origin(TripLocation),
    #[serde(rename = "stop")]
    Stop(Stop),
    #[serde(rename = "destination")]
    Destination(TripLocation),
}

impl Default for TripStop {
    fn default() -> Self {
        Self::Stop(Stop::default())
    }
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
pub struct Stop {
    pub centre: GeoLocation,
    pub key: u32,
    pub name: String,
}
