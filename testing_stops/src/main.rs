use transit_api_client::prelude::*;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[tokio::main]
async fn main() {
    let transit_client: TransitClient = TransitClient::new("8LmD2omXO57rTz7cbi9w".to_string());

    const STOP: u32 = 11032;
    const FILTERS: Vec<filters::Stop> = vec![];
    const USAGE: Usage = Usage::Normal;
    match transit_client.stop_schedule(STOP, FILTERS, USAGE).await {
        Ok(schedule) => {

            /// just a quick test to see if I can get the data I want
            #[derive(Debug, Serialize, Deserialize)]
            struct Tester {
                /// The name of the bus
                bus_name: String,

                /// The time the bus is scheduled to arrive
                arrival_time: PrimitiveDateTime,
            }

            let mut tester: Vec<Tester> = Vec::new();

            for route_schedules in schedule.route_schedules {
                for stops in route_schedules.scheduled_stops {
                   if !stops.cancelled {
                        tester.push(Tester {
                            bus_name: route_schedules.route.name.clone().unwrap_or("BLUE".to_string()),
                            arrival_time: stops.times.arrival.estimated.clone(),
                        });
                   }
                }
            }

            for i in tester {
                println!("{:#?}", i);
            }
        }
        Err(err) => {
            // Handle the error case
            println!("Error: {:?}", err);
        }
    }
}