use crate::{error_string, ClientState, SettingsState};
use tauri::State;
use time::Date;
use transit_api_client::filters::{Mode, TripPlan as TripPlanFilters};
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
    date: Date,
    time: (u8, u8),
    mode: Mode,
    client: State<'_, ClientState>,
    settings: State<'_, SettingsState>,
) -> Result<Vec<trip::Plan>, &'static str> {
    let trip_filters = {
        let settings = settings.0.lock().await;
        vec![
            // Specified filters from user
            TripPlanFilters::Date(date),
            TripPlanFilters::Time(time.0, time.1),
            TripPlanFilters::Mode(mode),
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
