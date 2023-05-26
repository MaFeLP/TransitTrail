use transit_api_client::prelude::*;
//use serde_json;

#[tokio::main]
async fn main() {
    let transit_client: TransitClient = TransitClient::new("8LmD2omXO57rTz7cbi9w".to_string());

    let stop:u32 = 20137;
    let filters: Vec<filters::Stop> = vec![];
    let usage:Usage = Usage::Normal;

    match transit_client.stop_schedule(stop, filters, usage).await {
        Ok(schedule) => {
            // Print the raw response body for inspection
            // let response_body = serde_json::to_string(&schedule).unwrap();
            // println!("Raw Response: {}", response_body);

            println!("{:?}", schedule)

            // Deserialize the schedule data
            // ...
        }
        Err(err) => {
            // Handle the error case
            println!("Error: {:?}", err);
        }
    }
}