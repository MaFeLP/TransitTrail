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


use crate::{error_string, ClientState, GoogleMapsState, SettingsState};
use google_maps_api_client::{GeocodeResult, GoogleMapsClient, TravelMode};
use tauri::State;
use time::macros::{format_description, offset};
use time::{Date, OffsetDateTime};
use tokio::sync::MutexGuard;
use transit_api_client::filters::{Mode, TripPlan as TripPlanFilters};
use transit_api_client::prelude::trip::Plan;
use transit_api_client::prelude::*;

#[tauri::command]
pub async fn search_locations(
    input: &str,
    client: State<'_, ClientState>,
) -> Result<Vec<Location>, &'static str> {
    client
        .0
        .lock()
        .await
        .search_locations(input, Some(10), Usage::Normal)
        .await
        .map_err(|why| error_string(&why, "Could not search for locations"))
}

#[tauri::command]
pub async fn trip_planner(
    origin: PartialLocation<'_>,
    destination: PartialLocation<'_>,
    date: Option<&str>,
    time: Option<(u8, u8)>,
    mode: Mode,
    client: State<'_, ClientState>,
    settings: State<'_, SettingsState>,
) -> Result<Vec<Plan>, &'static str> {
    let trip_filters = {
        let settings = settings.0.lock().await;

        let now = OffsetDateTime::now_utc().to_offset(offset!(-5));
        let now_time = time.unwrap_or((now.time().hour(), now.time().minute()));

        vec![
            // Specified filters from user
            TripPlanFilters::Mode(mode),
            TripPlanFilters::Date(match date {
                Some(d) => Date::parse(d, format_description!("[year]-[month]-[day]"))
                    .map_err(|_| "Invalid date")?,
                None => now.date(),
            }),
            TripPlanFilters::Time(now_time.0, now_time.1),
            // Specified filters from settings
            TripPlanFilters::MaxTransfers(settings.max_transfers),
            TripPlanFilters::MinTransferWait(settings.min_waiting_time),
            TripPlanFilters::MaxTransferWait(settings.max_waiting_time),
            TripPlanFilters::WalkSpeed(settings.walking_speed),
            TripPlanFilters::MaxWalkTime(settings.max_walking_time),
        ]
    };

    client
        .0
        .lock()
        .await
        .trip_planner(origin, destination, trip_filters, Usage::Normal)
        .await
        .map_err(|why| error_string(&why, "Could not get trip plan from the API"))
}

// Google Maps API
async fn get_geocode_from_string(
    client: &GoogleMapsClient,
    address: &str,
) -> Result<Option<GeocodeResult>, &'static str> {
    client
        .geocode(&format!("{}, Winnipeg, MB, Canada", address))
        .await
        .map_err(|why| error_string(&why, "Could not get geocode from address"))
}

async fn partial_location_to_geocode(
    partial_location: PartialLocation<'_>,
    google_client: &GoogleMapsClient,
    transit_client: &TransitClient,
) -> Result<Option<GeocodeResult>, &'static str> {
    match partial_location {
        PartialLocation::Point(lat, lng) => google_client
            .reverse_geocode(lat, lng)
            .await
            .map_err(|why| error_string(&why, "Could not get geocode from point")),
        PartialLocation::Stop(key) => {
            let stop = transit_client
                .stop_info(key, Usage::Normal)
                .await
                .map_err(|why| error_string(&why, "Could not get stop info"))?;

            google_client
                .reverse_geocode(stop.centre.latitude, stop.centre.longitude)
                .await
                .map_err(|why| error_string(&why, "Could not get geocode from point"))
        }
        PartialLocation::Address(address) => get_geocode_from_string(google_client, address).await,
        PartialLocation::Monument(monument) => {
            get_geocode_from_string(google_client, monument).await
        }
        PartialLocation::Intersection(intersection) => {
            get_geocode_from_string(google_client, intersection).await
        }
    }
}

#[tauri::command]
pub async fn google_trip_planner(
    origin: PartialLocation<'_>,
    destination: PartialLocation<'_>,
    google_client: State<'_, GoogleMapsState>,
    transit_client: State<'_, ClientState>,
) -> Result<Vec<Plan>, &'static str> {
    let google_client: MutexGuard<'_, GoogleMapsClient> = google_client.0.lock().await;
    let transit_client: MutexGuard<'_, TransitClient> = transit_client.0.lock().await;

    let origin_geocode = partial_location_to_geocode(origin, &google_client, &transit_client)
        .await?
        .ok_or("Could not get geocode from origin")?;
    let destination_geocode =
        partial_location_to_geocode(destination, &google_client, &transit_client)
            .await?
            .ok_or("Could not get geocode from origin")?;

    let trip_plan = google_client
        .get_directions_from_placeid(
            &origin_geocode.place_id,
            &destination_geocode.place_id,
            Some(TravelMode::Transit),
        )
        .await
        .map_err(|why| error_string(&why, "Could not get trip plan from the API"))?
        .ok_or("Could not get trip plan from the Google Maps API")?;

    let legs = trip_plan.legs;
    let _warnings = trip_plan.warnings.unwrap_or_default();

    let plans = legs.into_iter().map(|leg| leg.into()).collect();

    Ok(plans)
}
