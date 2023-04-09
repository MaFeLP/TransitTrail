//!
//! Filters for various API endpoints, that may have more than one filter option
//!

use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

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

impl From<TripPlan> for UrlParameter {
    fn from(value: TripPlan) -> Self {
        Self(match value {
            TripPlan::Date(d) => format!("&date={}", d.format("%Y-%m-%d")),
            TripPlan::Time(t) => format!("&time={}", t.format("%H:%M:%S")),
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

