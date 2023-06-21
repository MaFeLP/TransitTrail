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


use serde::{Deserialize, Serialize};
use tauri::State;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::PrimitiveDateTime;
use transit_api_client::prelude::*;

use crate::ClientState;

/// Data to be turned into HTML
#[derive(Debug, Serialize, Deserialize)]
struct Html {
    /// The name of the bus
    bus_name: String,

    /// The time the bus is actually arriving
    arrival_time: PrimitiveDateTime,

    /// The time the bus is scheduled to arrive
    scheduled_time: PrimitiveDateTime,

    class_names: Vec<String>,
}

impl Html {
    fn delayed(&self) -> bool {
        self.arrival_time > self.scheduled_time
    }

    fn early(&self) -> bool {
        self.arrival_time < self.scheduled_time
    }
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
    filter: Vec<filters::Stop>,
) -> Result<String, String> {
    // Get the schedule for the stop
    match transit_client
        .0
        .lock()
        .await
        .stop_schedule(stop, filter, Usage::Normal)
        .await
    {
        Ok(Schedule {
            stop,
            route_schedules,
        }) => {
            // Handle the success case
            let mut busses: Vec<Html> = Vec::new();

            for route_schedules in route_schedules {
                for stops in route_schedules.scheduled_stops {
                    if !stops.cancelled {
                        busses.push(Html {
                            bus_name: route_schedules
                                .route
                                .name
                                .clone()
                                .unwrap_or("BLUE".to_string()),
                            arrival_time: stops.times.departure.estimated,
                            scheduled_time: stops.times.departure.scheduled,
                            class_names: route_schedules
                                .route
                                .badge_style
                                .class_names
                                .class_name
                                .clone()
                                .into_iter()
                                .map(|x| x.to_string())
                                .collect(),
                        });
                    }
                }
            }

            busses.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));

            let mut html = String::new();
            html.push_str(&format!(
                r#"
                <div id="stop-info">
                    <span class="number">#{}: </span>
                    <span class="name">{} {} @ {}</a>
                </div>
                <table class="bus-list">
                    <tr class="bus-list-header">
                        <th class="bus-list-header">Departure</th>
                        <th class="bus-list-header">Status</th>
                        <th class="bus-list-header">Scheduled</th>
                        <th class="bus-list-header">Name</th>
                    </tr>
                "#,
                stop.number, stop.direction, stop.street.name, stop.cross_street.name,
            ));
            for bus in busses {
                html.push_str(&bus_to_html(bus));
            }
            html.push_str("</table>");

            Ok(html)
        }
        Err(err) => {
            // Handle the error case
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
fn bus_to_html(data: Html) -> String {
    const DATETIME_FORMAT: &[FormatItem] =
        format_description!("[month]/[day] [hour]:[minute]:[second]");
    const TIME_FORMAT: &[FormatItem] = format_description!("[hour]:[minute]:[second]");
    // <div class="bus">
    //     <div class="bus-name">BLUE</div>
    //     <div class="bus-time">12:00</div>
    // </div>

    let mut html = String::new();
    html.push_str("<tr class=\"bus\">");
    if data.delayed() {
        html.push_str(&format!(
            r#"
            <td class="time late">{}</td>
            <td class="late">LATE</td>
            <td class="scheduled">was {}</td>
            "#,
            data.arrival_time.format(DATETIME_FORMAT).unwrap(),
            data.scheduled_time.format(TIME_FORMAT).unwrap()
        ));
    } else if data.early() {
        html.push_str(&format!(
            r#"
            <td class="time early">{}</td>
            <td class="early">EARLY</td>
            <td class="scheduled">was {}</td>
            "#,
            data.arrival_time.format(DATETIME_FORMAT).unwrap(),
            data.scheduled_time.format(TIME_FORMAT).unwrap()
        ));
    } else {
        html.push_str(&format!(
            r#"
            <td class="time on-time">{}</td>
            <td class="on-time">ON-TIME</td>
            <td class="scheduled"></td>
            "#,
            data.arrival_time.format(DATETIME_FORMAT).unwrap()
        ));
    }
    html.push_str(&format!(
        r#"
        <td class="name {}">{}</td>
        "#,
        data.class_names.join(" "),
        data.bus_name
    ));
    html.push_str("</tr>");
    html
}
