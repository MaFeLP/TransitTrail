use transit_api_client::prelude::*;
use serde::{Deserialize, Serialize};
use tauri::State;
use time::PrimitiveDateTime;

use crate::ClientState;


/// Data to be turned into HTML
#[derive(Debug, Serialize, Deserialize)]
struct Html {
    /// The name of the bus
    bus_name: String,

    /// The time the bus is scheduled to arrive
    arrival_time: PrimitiveDateTime,
}

/// Gets all the stops for a given stop number
///
/// # Arguments
///
/// * `transit_client`: The transit client to use
/// * `stop`: The stop number to get the schedule for
///
/// returns: Result<String, Error>

#[tauri::command]
pub async fn stop_schedule(
    transit_client: State<'_, ClientState>,
    stop: u32,
)-> Result<String, String> {
    // Get the schedule for the stop
    match transit_client.0.lock().await.stop_schedule(stop, vec![], Usage::Normal).await {
        Ok(Schedule { stop: _stop, route_schedules }) => { // Handle the success case
            let mut busses: Vec<Html> = Vec::new();

            for route_schedules in route_schedules {
                for stops in route_schedules.scheduled_stops {
                    if !stops.cancelled {
                        busses.push(Html {
                            bus_name: route_schedules.route.name.clone().unwrap_or("BLUE".to_string()),
                            arrival_time: stops.times.arrival.estimated.clone(),
                        });
                    }
                }
            }

            let mut html = String::new();
            html.push_str("<div class=\"bus-list\">");
            for bus in busses {
                html.push_str(&bus_to_html(bus));
            }
            html.push_str("</div>");

            Ok(html)
        },
        Err(err) => { // Handle the error case
            println!("Error: {:?}", err);
            Err("Error".to_string())
        }
    }
}


/// Converts a `Html` struct into a string of HTML
///
/// # Arguments
///
/// * `data`: Html - The data to be converted into HTML
///
/// returns: String
fn bus_to_html(data: Html)-> String {
    // <div class="bus">
    //     <div class="bus-name">BLUE</div>
    //     <div class="bus-time">12:00</div>
    // </div>

    let mut html = String::new();
    html.push_str("<div class=\"bus\">");
    html.push_str(&format!("<div class=\"bus-name\">{}</div>", data.bus_name));
    html.push_str(&format!("<div class=\"bus-time\">{}</div>", data.arrival_time));
    html.push_str("</div>");
    html
}
