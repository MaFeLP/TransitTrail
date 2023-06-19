//!
//! This module holds structs used in the Trip Planner, in individual segments;
//!

use crate::prelude::badges::{ClassNames, Style};
use crate::prelude::{Blue, Regular};
use crate::structs::trip_planner::Durations;
use crate::structs::{
    routes::{Route, Variant},
    stops::Bus,
    trip_planner::{Bounds, Times, TripStop},
};
use google_maps_api_client::{DirectionsStep, TravelMode};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use time::{OffsetDateTime, PrimitiveDateTime};

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

impl From<DirectionsStep> for Segment {
    fn from(step: DirectionsStep) -> Self {
        match step.travel_mode {
            TravelMode::Walking => Self::Walk(Walk {
                times: Times {
                    durations: Durations {
                        walking: (step.duration.value / 60) as u32,
                        total: (step.duration.value / 60) as u32,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                instructions: Some(step.html_instructions),
                ..Default::default()
            }),
            TravelMode::Transit => {
                let transit_details = step
                    .transit_details
                    .expect("Can not display transit details! Missing in API response!");
                let start_time = OffsetDateTime::from_unix_timestamp(
                    transit_details.departure_time.unwrap().value,
                )
                .unwrap()
                .to_offset(offset!(-5));
                let end_time = OffsetDateTime::from_unix_timestamp(
                    transit_details.arrival_time.unwrap().value,
                )
                .unwrap()
                .to_offset(offset!(-5));

                Self::Ride(Ride {
                    route: {
                        let line = transit_details
                            .line
                            .expect("Could not parse line from transit details. Field is missing");
                        let short_name = line.short_name.expect(
                            "Could not parse short name from transit details. Field is missing",
                        );
                        if short_name == *"BLUE".to_string() {
                            Route::Blue(Blue {
                                key: "BLUE".to_string(),
                                number: "BLUE".to_string(),
                                badge_label: "BLUE".to_string(),
                                badge_style: Style {
                                    class_names: ClassNames::default(),
                                    background_color: line.color.expect("Background colour not given!"),
                                    color: line.text_color.clone().expect("Text colour not given!"),
                                    border_color: line.text_color.expect("Border colour not given!"),
                                },
                                variants: Some(vec![Variant {
                                    key: "BLUE".to_string(),
                                    name: Some(format!("BLUE to {}", transit_details.headsign.expect("Could not get the headsign of the line! Is the key wrong?"))),
                                }]),
                                ..Default::default()
                            })
                        } else {
                            let short_name: u32 = short_name.parse().expect(
                                "Could not convert the name to a number. Is the key wrong?",
                            );
                            Route::Regular(Regular {
                                key: short_name,
                                number: short_name,
                                badge_label: short_name,
                                badge_style: Style {
                                    class_names: ClassNames::default(),
                                    background_color: line.color.expect("Background colour not given!"),
                                    color: line.text_color.clone().expect("Text colour not given!"),
                                    border_color: line.text_color.expect("Border colour not given!"),
                                },
                                variants: Some(vec![Variant {
                                    key: short_name.to_string(),
                                    name: Some(format!("{} to {}", short_name, transit_details.headsign.expect("Could not get the headsign of the line! Is the key wrong?"))),
                                }]),
                                ..Default::default()
                            })
                        }
                    },
                    times: Times {
                        start: PrimitiveDateTime::new(start_time.date(), start_time.time()),
                        end: PrimitiveDateTime::new(end_time.date(), end_time.time()),
                        durations: Durations {
                            riding: (step.duration.value / 60) as u32,
                            total: (step.duration.value / 60) as u32,
                            ..Default::default()
                        },
                    },
                    to: Some(
                        transit_details
                            .arrival_stop
                            .expect("No arrival stop was given in the Google API response")
                            .name,
                    ),
                    from: Some(
                        transit_details
                            .departure_stop
                            .expect("No departure stop was given in the Google API response")
                            .name,
                    ),
                    ..Default::default()
                })
            }
            _ => panic!("Unsupported travel mode: {:?}", step.travel_mode),
        }
    }
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

    /// HTML instructions that are present in the google maps API
    pub instructions: Option<String>,
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

    // When using the Google Maps API, these fields are set:
    /// The stop where the bus departs from
    pub to: Option<String>,

    /// The stop where the bus arrives at
    pub from: Option<String>,
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
