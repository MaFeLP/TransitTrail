use super::{
    common::{GeoLocation, Street},
    deserialize_string_to_bool,
    routes::{Route, Variant},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub key: u32,
    pub name: String,
    pub number: u32,
    pub direction: Direction,
    pub side: Side,
    pub street: Street,
    #[serde(rename = "cross-street")]
    pub cross_street: Street,
    pub centre: GeoLocation,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Northbound,
    Eastbound,
    Southbound,
    Westbound,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "Direct Opposite")]
    DirectOpposite, // stop: 10168
    Farside, //  stop: 10095
    #[serde(rename = "Farside Opposite")]
    FarsideOpposite, // stop: 10081
    Nearside, // stop: 10076
    #[serde(rename = "Nearside Opposite")]
    NearsideOpposite, // stop: 10077
    NA,      // stop: 10087
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub stop: Stop,
    #[serde(rename = "route-schedules")]
    pub route_schedules: Vec<RouteSchedule>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteSchedule {
    pub route: Route,
    #[serde(rename = "scheduled-stops")]
    pub scheduled_stops: Vec<ScheduledStop>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledStop {
    pub key: String,
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub cancelled: bool,
    pub times: ScheduledTimes,
    pub variant: Variant,
    pub bus: Bus,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTimes {
    pub arrival: Time,
    pub departure: Time,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub scheduled: NaiveDateTime,
    pub estimated: NaiveDateTime,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bus {
    pub key: u32,
    #[serde(deserialize_with = "deserialize_string_to_bool", rename = "bike-rack")]
    pub bike_rack: bool,
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub wifi: bool,
}
