use crate::structs::{
    common::Location,
    trip_planner::{TripFilter, TripPlan},
    UrlParameter, Usage,
};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
    pub async fn trip_planner(
        &self,
        origin: Location,
        destination: Location,
        filters: Vec<TripFilter>,
        usage: Usage,
    ) -> Result<Vec<TripPlan>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            plans: Vec<TripPlan>,
        }

        let mut filter_parameters = String::new();
        for filter in filters {
            filter_parameters.push_str(&UrlParameter::from(filter).to_string())
        }

        let response = self
            .client
            .get(format!(
                "{base}/trip-planner.json?api-key={api_key}{usage}&origin={origin}&destination={destination}{filter_parameters}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        //dbg!(&response);
        //let text = response.text().await?;
        //let out = serde_json::from_str::<Response>(&text).unwrap();
        let out: Response = response.json().await?;
        Ok(out.plans)
    }
}

#[cfg(test)]
mod test {
    use crate::structs::{
        common::{GeoLocation, Location},
        trip_planner::{TripFilter, TripMode},
        Usage,
    };
    use chrono::offset::Local;

    #[test]
    fn default_trip() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        rt.block_on(client.trip_planner(
            Location::Point(GeoLocation {
                latitude: 49.86917,
                longitude: -97.1391,
            }),
            Location::Point(GeoLocation {
                latitude: 49.8327,
                longitude: -97.10887,
            }),
            Vec::new(),
            Usage::Normal,
        ))
        .unwrap();
    }

    #[test]
    fn filters() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client = crate::TransitClient::new(
            std::env::var("WPG_TRANSIT_API_KEY").unwrap_or(String::from("")),
        );
        rt.block_on(client.trip_planner(
            Location::Point(GeoLocation {
                latitude: 49.86917,
                longitude: -97.1391,
            }),
            Location::Point(GeoLocation {
                latitude: 49.8327,
                longitude: -97.10887,
            }),
            vec![
                TripFilter::Date(Local::now().naive_local().date()),
                TripFilter::Time(Local::now().naive_local().time()),
                TripFilter::Mode(TripMode::DepartAfter),
                TripFilter::WalkSpeed(1.5),
                TripFilter::MaxWalkTime(10),
                TripFilter::MinTransferWait(5),
                TripFilter::MaxTransferWait(10),
                TripFilter::MaxTransfers(2),
            ],
            Usage::Normal,
        ))
        .unwrap();
    }
}
