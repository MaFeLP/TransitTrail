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


//!
//! Holds functions to plan a trip using the Navigo engine
//!

use serde::Deserialize;

use crate::filters;
use crate::prelude::PartialLocation;
use crate::structs::{trip_planner::Plan, Error, UrlParameter, Usage};

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
    /// use time::{OffsetDateTime, macros::offset};
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let now = OffsetDateTime::now_utc().to_offset(offset!(-7));
    /// let trip_plan = client
    ///     .trip_planner(
    ///         PartialLocation::Point(49.86917, -97.1391),
    ///         PartialLocation::Point(49.8327, -97.10887),
    ///         vec![
    ///             // These are all available filters
    ///             filters::TripPlan::Date(now.date()),
    ///             filters::TripPlan::Time(now.time().hour(), now.time().minute()),
    ///             filters::TripPlan::Mode(filters::Mode::DepartAfter),
    ///             filters::TripPlan::WalkSpeed(1.5),
    ///             filters::TripPlan::MaxWalkTime(10),
    ///             filters::TripPlan::MinTransferWait(5),
    ///             filters::TripPlan::MaxTransferWait(10),
    ///             filters::TripPlan::MaxTransfers(2),
    ///         ],
    ///         Usage::Normal,
    ///         )
    ///     .await
    ///     .unwrap();
    /// # });
    /// ```
    pub async fn trip_planner(
        &self,
        origin: PartialLocation<'_>,
        destination: PartialLocation<'_>,
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
        log::debug!("Got response for trip plan: {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for trip plan: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.plans)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use time::{macros::offset, OffsetDateTime};

    #[tokio::test]
    async fn default_trip() {
        let client = crate::testing_client();
        let actual = client
            .trip_planner(
                PartialLocation::Point(49.86917, -97.1391),
                PartialLocation::Point(49.8327, -97.10887),
                Vec::new(),
                Usage::Normal,
            )
            .await
            .unwrap();
        log::info!("actual={:?}", &actual);
    }

    #[tokio::test]
    async fn filters() {
        let client = crate::testing_client();
        let now = OffsetDateTime::now_utc().to_offset(offset!(-7));
        let actual = client
            .trip_planner(
                PartialLocation::Point(49.86917, -97.1391),
                PartialLocation::Point(49.8327, -97.10887),
                vec![
                    filters::TripPlan::Date(now.date()),
                    filters::TripPlan::Time(now.time().hour(), now.time().minute()),
                    filters::TripPlan::Mode(filters::Mode::DepartAfter),
                    filters::TripPlan::WalkSpeed(1.5),
                    filters::TripPlan::MaxWalkTime(10),
                    filters::TripPlan::MinTransferWait(5),
                    filters::TripPlan::MaxTransferWait(10),
                    filters::TripPlan::MaxTransfers(2),
                ],
                Usage::Normal,
            )
            .await
            .unwrap();
        log::info!("actual={:?}", &actual);
    }
}
