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
//! Filters for various API endpoints, that may have more than one filter option
//!

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::{macros::format_description, Date, Time};

use crate::structs::common::{StreetLeg, StreetType};
use crate::structs::{
    service_advisories::{Category, Priority},
    UrlParameter,
};

/// Filter service advisories
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ServiceAdvisory {
    /// Only return service advisories of this priority or higher. (default: [Priority::VeryLow])
    Priority(Priority),

    /// Only return service advisories of this category (default: [Category::All])
    Category(Category),

    /// Only returns advisories created or updated in the last N days.
    MaxAge(u32),

    /// Only show the top N service advisories -- no more than the given limit.
    Limit(u32),
}

impl From<ServiceAdvisory> for UrlParameter {
    fn from(value: ServiceAdvisory) -> Self {
        match value {
            ServiceAdvisory::Priority(p) => UrlParameter::from(p),
            ServiceAdvisory::Category(c) => UrlParameter::from(c),
            ServiceAdvisory::MaxAge(a) => UrlParameter(format!("&max_age={a}")),
            ServiceAdvisory::Limit(l) => UrlParameter(format!("&limit={l}")),
        }
    }
}

/// Specify filters for the trip planning
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TripPlan {
    /// The date for which to get navigo results. Defaults to today, if not included as a filter
    Date(Date),

    /// The time of the trip. Defaults to now, if not included as a filter.
    ///
    /// What the time means can be customized with a [Mode]
    //TODO change to type (u32, u32) (for hours, minutes)
    Time(Time),

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

impl From<TripPlan> for UrlParameter {
    fn from(value: TripPlan) -> Self {
        Self(match value {
            TripPlan::Date(d) => format!(
                "&date={}",
                d.format(format_description!("[year]-[month]-[day]"))
                    .unwrap()
            ),
            TripPlan::Time(t) => format!(
                "&time={}",
                t.format(format_description!("[hour]:[minute]:[second]"))
                    .unwrap()
            ),
            TripPlan::Mode(m) => format!("&mode={}", m),
            TripPlan::WalkSpeed(s) => format!("&walk-speed={}", s),
            TripPlan::MaxWalkTime(t) => format!("&max-walk-time={}", t),
            TripPlan::MinTransferWait(t) => format!("&min-transfer-wait={}", t),
            TripPlan::MaxTransferWait(t) => format!("&ax-transfer-wait={}", t),
            TripPlan::MaxTransfers(t) => format!("&max-transfers={}", t),
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

/// A filter when searching for streets
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Street<'a> {
    /// Filter for the name of the street
    Name(&'a str),

    /// Filter for the type of the street
    Type(StreetType),

    /// Filter for the leg of the street
    Leg(StreetLeg),
}

impl From<Street<'_>> for UrlParameter {
    fn from(value: Street) -> Self {
        let out = match value {
            Street::Name(n) => format!("&name={n}"),
            Street::Type(t) => format!("&type={t}"),
            Street::Leg(l) => {
                let leg = match l {
                    StreetLeg::North => "N",
                    StreetLeg::East => "E",
                    StreetLeg::South => "S",
                    StreetLeg::West => "W",
                };
                format!("&leg={leg}")
            }
        };
        UrlParameter(out)
    }
}

/// A filter when getting the schedule for a stop
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stop {
    /// Only return results for the specified route
    ///
    /// Defaults to all routes
    Routes(Vec<u32>),

    /// Only return results after this time
    ///
    /// Defaults to now
    Start((u8, u8)),

    /// Only return results before this time
    ///
    /// Defaults to two hours from now
    End((u8, u8)),

    /// Limit the results per returned route
    MaxResultsPerRoute(u32),
}

impl From<Stop> for UrlParameter {
    fn from(value: Stop) -> Self {
        /// Format the time correctly:
        /// Format is: HH:MM:SS
        fn format_time(hours: u8, minutes: u8) -> String {
            let hours = if hours < 10 {
                format!("0{}", hours)
            } else {
                hours.to_string()
            };
            let minutes = if minutes < 10 {
                format!("0{}", minutes)
            } else {
                minutes.to_string()
            };

            format!("{hours}:{minutes}:00")
        }

        let out = match value {
            Stop::Routes(r) => {
                match r.len() {
                    0 => String::new(),
                    1 => format!("&route={}", r[0]),
                    _ => {
                        let mut routes = "&routes=".to_string();
                        for route in r {
                            routes.push_str(route.to_string().as_str());
                            routes.push(',');
                        }
                        routes.pop(); // Remove the last ','
                        routes
                    }
                }
            }
            Stop::Start((hours, minutes)) => {
                format!("&start={}", format_time(hours, minutes))
            }
            Stop::End((hours, minutes)) => {
                format!("&end={}", format_time(hours, minutes))
            }
            Stop::MaxResultsPerRoute(m) => format!("&max-results-per-route={m}"),
        };
        UrlParameter(out)
    }
}
