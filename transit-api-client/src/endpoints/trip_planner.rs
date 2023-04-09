//!
//! Holds functions to plan a trip using the Navigo engine
//!

use reqwest::Error;
use serde::Deserialize;

use crate::filters;
use crate::structs::{
    common::Location,
    trip_planner::Plan,
    UrlParameter,
    Usage,
};

impl crate::TransitClient {
    /// Uses the Navigo engine to plan optimal trips from an origin to a destination.
    ///
    /// # Arguments
    ///
    /// * `origin`: A location, where the trip should start
    /// * `destination`: A location, where the trip should end
    /// * `filters`: Any potential filters, to personalize the trip
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec<Plan, Global>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chrono::offset::Local;
    /// use transit_api_client::structs::{
    ///     common::{Location, GeoLocation},
    ///     trip_planner::{Filter, Mode},
    ///     Usage
    /// };
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let trip_plan = client
    ///     .trip_planner(
    ///         Location::Point(GeoLocation {
    ///             latitude: 49.86917,
    ///             longitude: -97.1391,
    ///         }),
    ///         Location::Point(GeoLocation {
    ///             latitude: 49.8327,
    ///             longitude: -97.10887,
    ///         }),
    ///         vec![
    ///             // These are all available filters
    ///             Filter::Date(Local::now().naive_local().date()),
    ///             Filter::Time(Local::now().naive_local().time()),
    ///             Filter::Mode(Mode::DepartAfter),
    ///             Filter::WalkSpeed(1.5),
    ///             Filter::MaxWalkTime(10),
    ///             Filter::MinTransferWait(5),
    ///             Filter::MaxTransferWait(10),
    ///             Filter::MaxTransfers(2),
    ///         ],
    ///         Usage::Normal,
    ///         )
    ///     .await
    ///     .unwrap();
    /// # });
    /// ```
    pub async fn trip_planner(
        &self,
        origin: Location,
        destination: Location,
        filters: Vec<filters::TripPlan>,
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

    use crate::{
        filters::{Mode, TripPlan},
        structs::{
            common::{GeoLocation, Location},
            Usage,
        },
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
                    TripPlan::Date(Local::now().naive_local().date()),
                    TripPlan::Time(Local::now().naive_local().time()),
                    TripPlan::Mode(Mode::DepartAfter),
                    TripPlan::WalkSpeed(1.5),
                    TripPlan::MaxWalkTime(10),
                    TripPlan::MinTransferWait(5),
                    TripPlan::MaxTransferWait(10),
                    TripPlan::MaxTransfers(2),
                ],
                Usage::Normal,
            )
            .await
            .unwrap();
    }
}
