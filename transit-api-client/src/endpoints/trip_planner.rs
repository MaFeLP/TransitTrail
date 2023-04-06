use reqwest::Error;
use serde::Deserialize;

use crate::structs::{
    common::Location,
    trip_planner::{Filter, Plan},
    UrlParameter, Usage,
};

impl crate::TransitClient {
    pub async fn trip_planner(
        &self,
        origin: Location,
        destination: Location,
        filters: Vec<Filter>,
        usage: Usage,
    ) -> Result<Vec<Plan>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            plans: Vec<Plan>,
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
    use chrono::offset::Local;

    use crate::structs::{
        common::{GeoLocation, Location},
        trip_planner::{Filter, Mode},
        Usage,
        };

    #[tokio::test]
    async fn default_trip() {
        let client = crate::testing_client();
        client
            .trip_planner(
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
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn filters() {
        let client = crate::testing_client();
        client
            .trip_planner(
                Location::Point(GeoLocation {
                    latitude: 49.86917,
                    longitude: -97.1391,
                }),
                Location::Point(GeoLocation {
                    latitude: 49.8327,
                    longitude: -97.10887,
                }),
                vec![
                    Filter::Date(Local::now().naive_local().date()),
                    Filter::Time(Local::now().naive_local().time()),
                    Filter::Mode(Mode::DepartAfter),
                    Filter::WalkSpeed(1.5),
                    Filter::MaxWalkTime(10),
                    Filter::MinTransferWait(5),
                    Filter::MaxTransferWait(10),
                    Filter::MaxTransfers(2),
                ],
                Usage::Normal,
            )
            .await
            .unwrap();
    }
}
