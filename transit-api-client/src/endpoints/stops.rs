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
//! Holds functions to get information about stops from the API
//!

use crate::filters;
use serde::Deserialize;

use crate::structs::common::GeoLocation;
use crate::structs::{
    stops::{Feature, PartialStop, Schedule, Stop},
    Error, UrlParameter, Usage,
};

impl crate::TransitClient {
    /// Get information about a specific stop
    ///
    /// # Arguments
    ///
    /// * `stop`: They stop number to get information about
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Stop, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let stop = client.stop_info(10168, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn stop_info(&self, stop: u32, usage: Usage) -> Result<Stop, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            stop: Stop,
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}.json?api-key={api_key}{usage}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for stop (info; #{stop}): {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for stop (info; #{stop}): {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.stop)
    }

    /// Get information about nearby stops
    ///
    /// # Arguments
    ///
    /// * `location`: The location from which to search stops from.
    /// * `distance`: The distance in meters within the stops must fall.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Stop, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let stops = client
    ///     .stops_nearby(GeoLocation::new(49.895, -97.138), 250, Usage::Normal)
    ///     .await
    ///     .unwrap();
    /// # });
    /// ```
    pub async fn stops_nearby(
        &self,
        location: GeoLocation,
        distance: u32,
        usage: Usage,
    ) -> Result<Vec<Stop>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            stops: Vec<Stop>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops.json?api-key={api_key}{usage}{location}&distance={distance}&walking=true",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
                location = UrlParameter::from(location),
            ))
            .send()
            .await?;
        log::debug!("Got response for nearby stops: {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for nearby stops: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.stops)
    }

    /// Returns information about any features related to the requested stop.
    ///
    /// # Arguments
    ///
    /// * `stop`: They stop number to get information about
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Vec\<Feature\>, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let stop_features = client.stop_features(10168, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn stop_features(&self, stop: u32, usage: Usage) -> Result<Vec<Feature>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-features")]
            stop_features: Vec<Feature>,
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}/features.json?api-key={api_key}{usage}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for stop (features; #{stop}): {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for stop (features; #{stop}): {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.stop_features)
    }

    /// Returns the schedule information for the requested stop.
    ///
    /// # Arguments
    ///
    /// * `stop`: They stop number to get information about
    /// * `start`: The start time. (default: now)
    /// * `end`: The end time. (default: in two hours from now)
    /// * `limit`: The maximum number of scheduled stop times returned for each route stopping
    ///   at this stop.
    /// * `usage`: If the API should yield shorter, longer, or normal names.
    ///
    /// returns: Result<Schedule, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let stop_schedule = client.stop_schedule(10168, vec![], Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn stop_schedule(
        &self,
        stop: u32,
        filters: Vec<filters::Stop>,
        usage: Usage,
    ) -> Result<Schedule, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-schedule")]
            stop_schedule: Schedule,
        }

        let mut url_parameters = String::new();
        for filter in filters {
            let parameter = UrlParameter::from(filter);
            url_parameters.push_str(&parameter.0);
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}/schedule.json?api-key={api_key}{usage}{url_parameters}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
            ))
            .send()
            .await?;
        log::debug!("Got response for stop (schedule; #{stop}): {:?}", &response);
        let text = response.text().await?;
        log::debug!("Response body for stop (schedule; #{stop}): {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");

        Ok(out.stop_schedule)
    }

    /// Returns all stops in Winnipeg, using a non-official API
    ///
    /// The stops are not complete and only include position and an icon style.
    /// Use [PartialStop::try_to_full_stop] to get all information about the stop.
    pub async fn get_all_stops(&self) -> Result<Vec<PartialStop>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            update: Response2,
        }
        #[derive(Debug, Deserialize)]
        struct Response2 {
            stops: Vec<PartialStop>,
        }
        let response = self
            .client
            .get("https://winnipegtransit.com/transit_maps_api/cache/stops?client_version=null")
            .send()
            .await?;
        log::debug!("Got response for destinations: {response:?}");
        let text = response.text().await?;
        log::debug!("Response body for destinations: {text}");
        let out: Response = serde_json::from_str(&text)?;
        log::debug!("Deserialized response: {out:?}");
        Ok(out.update.stops)
    }
}

impl PartialStop {
    /// Tries to convert a partial stop into a normal stop, using the transit API.
    ///
    /// # Arguments
    ///
    /// * `client`: The client used to access the API.
    ///
    /// returns: Result<Stop, Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use transit_api_client::prelude::*;
    ///
    /// # tokio_test::block_on(async {
    /// let client = TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let partial_stops = client.get_all_stops().await.unwrap();
    /// let mut stops: Vec<Stop> = vec![];
    /// // In real life, this will fail due to rate limit of maximum 100 requests per minute.
    /// for stop in partial_stops {
    ///     stop.try_to_full_stop(&client).await.unwrap();
    /// }
    /// # });
    pub async fn try_to_full_stop(&self, client: &crate::TransitClient) -> Result<Stop, Error> {
        client.stop_info(self.id, Usage::Normal).await
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use time::{macros::offset, OffsetDateTime};

    #[tokio::test]
    async fn stop_features() {
        let client = crate::testing_client();
        let mut actual = client.stop_features(10064, Usage::Normal).await.unwrap();
        let mut expected = vec![
            Feature {
                name: "Bench".to_string(),
                count: 1,
            },
            Feature {
                name: "Unheated Shelter".to_string(),
                count: 1,
            },
        ];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);

        actual = client.stop_features(10172, Usage::Normal).await.unwrap();
        expected = vec![
            Feature {
                name: "BUSwatch Electronic Sign".to_string(),
                count: 1,
            },
            Feature {
                name: "Bench".to_string(),
                count: 2,
            },
            Feature {
                name: "Unheated Shelter".to_string(),
                count: 1,
            },
        ];
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn stop_info() {
        let client = crate::testing_client();
        let mut actual = client.stop_info(10168, Usage::Normal).await.unwrap();
        let mut expected = Stop {
            key: 10168,
            name: "Westbound River at Cauchon".to_string(),
            number: 10168,
            distances: None,
            direction: Direction::Westbound,
            side: Side::DirectOpposite,
            street: Street {
                key: 3057,
                name: "River Avenue".to_string(),
                street_type: Some("Avenue".to_string()),
                leg: None,
            },
            cross_street: Street {
                key: 681,
                name: "Cauchon Street".to_string(),
                street_type: Some("Street".to_string()),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.88099,
                longitude: -97.14116,
            },
            internal_name: None,
            sequence_on_street: None,
            icon_style: None,
        };
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);

        actual = client.stop_info(10087, Usage::Normal).await.unwrap();
        expected = Stop {
            key: 10087,
            name: "Northbound Stafford at Stafford Loop".to_string(),
            number: 10087,
            distances: None,
            direction: Direction::Northbound,
            side: Side::NA,
            street: Street {
                key: 50000299,
                name: "Stafford".to_string(),
                street_type: Some("Loop".to_string()),
                leg: None,
            },
            cross_street: Street {
                key: 3465,
                name: "Stafford Street".to_string(),
                street_type: Some("Street".to_string()),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.85741,
                longitude: -97.15236,
            },
            internal_name: None,
            sequence_on_street: None,
            icon_style: None,
        };
        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn stops_nearby() {
        let client = crate::testing_client();
        let actual = client
            .stops_nearby(GeoLocation::new(49.895, -97.138), 100, Usage::Normal)
            .await
            .unwrap();
        let expected = vec![
            Stop {
                key: 10627,
                name: "Northbound Main at Pioneer".to_string(),
                number: 10627,
                direction: Direction::Northbound,
                side: Side::Farside,
                street: Street {
                    key: 2265,
                    name: "Main Street".to_string(),
                    street_type: Some("Street".to_string()),
                    leg: None,
                },
                cross_street: Street {
                    key: 2871,
                    name: "Pioneer Avenue".to_string(),
                    street_type: Some("Avenue".to_string()),
                    leg: None,
                },
                centre: GeoLocation {
                    latitude: 49.89491,
                    longitude: -97.1379,
                },
                internal_name: None,
                sequence_on_street: None,
                distances: Some(Distances {
                    direct: 12.28,
                    walking: 16.31,
                }),
                icon_style: None,
            },
            Stop {
                key: 10761,
                name: "Westbound Pioneer at Main".to_string(),
                number: 10761,
                direction: Direction::Westbound,
                side: Side::Nearside,
                street: Street {
                    key: 2871,
                    name: "Pioneer Avenue".to_string(),
                    street_type: Some("Avenue".to_string()),
                    leg: None,
                },
                cross_street: Street {
                    key: 2265,
                    name: "Main Street".to_string(),
                    street_type: Some("Street".to_string()),
                    leg: None,
                },
                centre: GeoLocation {
                    latitude: 49.89452,
                    longitude: -97.13759,
                },
                internal_name: None,
                sequence_on_street: None,
                distances: Some(Distances {
                    direct: 60.92,
                    walking: 102.52,
                }),
                icon_style: None,
            },
        ];

        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn stop_schedule() {
        let client = crate::testing_client();
        let actual = client
            .stop_schedule(10064, vec![], Usage::Normal)
            .await
            .unwrap();

        let expected_stop = Stop {
            key: 10064,
            name: "Northbound Osborne at Glasgow".to_string(),
            number: 10064,
            distances: None,
            direction: Direction::Northbound,
            side: Side::Nearside,
            street: Street {
                key: 2715,
                name: "Osborne Street".to_string(),
                leg: None,
                street_type: Some("Street".to_string()),
            },
            cross_street: Street {
                key: 1486,
                name: "Glasgow Avenue".to_string(),
                leg: None,
                street_type: Some("Avenue".to_string()),
            },
            centre: GeoLocation {
                latitude: 49.86912,
                longitude: -97.1375,
            },
            internal_name: None,
            sequence_on_street: None,
            icon_style: None,
        };

        log::info!("actual={:?}", &actual);
        assert_eq!(actual.stop, expected_stop);
    }

    #[tokio::test]
    async fn stop_schedule_route_filter() {
        use time::ext::NumericalDuration;
        let client = crate::testing_client();
        let now = OffsetDateTime::now_utc().to_offset(offset!(-7));
        let end = now.clone().checked_add(4.hours()).unwrap();
        let actual = client
            .stop_schedule(
                10185,
                vec![
                    filters::Stop::Routes(vec![18, 60]),
                    filters::Stop::Start((now.time().hour(), now.time().minute())),
                    filters::Stop::End((end.time().hour(), now.time().minute())),
                    filters::Stop::MaxResultsPerRoute(3),
                ],
                Usage::Normal,
            )
            .await
            .unwrap();

        let expected_stop = Stop {
            key: 10185,
            name: "Southbound Osborne at Wardlaw".to_string(),
            number: 10185,
            distances: None,
            direction: Direction::Southbound,
            side: Side::Nearside,
            street: Street {
                key: 2715,
                name: "Osborne Street".to_string(),
                leg: None,
                street_type: Some("Street".to_string()),
            },
            cross_street: Street {
                key: 3781,
                name: "Wardlaw Avenue".to_string(),
                leg: None,
                street_type: Some("Avenue".to_string()),
            },
            centre: GeoLocation {
                latitude: 49.87699,
                longitude: -97.14414,
            },
            internal_name: None,
            sequence_on_street: None,
            icon_style: None,
        };
        log::info!("actual={:?}", &actual);
        assert_eq!(actual.stop, expected_stop);
        // Can only test length here, as schedule changes live. This still tests the deserialization
        // This is only causing trouble................. and changing all the time.........
        //assert_eq!(actual.route_schedules[0].scheduled_stops.len(), 3);
        //assert_eq!(actual.route_schedules[1].scheduled_stops.len(), 3);
    }

    #[tokio::test]
    async fn get_all_stops() {
        let client = crate::testing_client();
        client.get_all_stops().await.unwrap();
    }

    #[tokio::test]
    async fn try_to_full_stop() -> Result<(), crate::structs::Error> {
        let client = crate::testing_client();
        let partial_stop = PartialStop {
            id: 40811,
            position: GeoLocation::new(49.90404, -96.96857),
            icon_style: PartialStopIconStyle::Blue,
        };
        let expected = Stop {
            key: 40811,
            name: "Eastbound McMeans at Corliss North".to_string(),
            number: 40811,
            distances: None,
            direction: Direction::Eastbound,
            side: Side::NearsideOpposite,
            street: Street {
                key: 2430,
                name: "McMeans Avenue".to_string(),
                street_type: Some("Avenue".to_string()),
                leg: Some(StreetLeg::East),
            },
            cross_street: Street {
                key: 873,
                name: "Corliss Crescent".to_string(),
                street_type: Some("Crescent".to_string()),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.90404,
                longitude: -96.96857,
            },
            internal_name: None,
            sequence_on_street: None,
            icon_style: None,
        };
        let actual = partial_stop.try_to_full_stop(&client).await?;
        log::info!("actual={:?},expected={:?}", &actual, &expected);
        assert_eq!(actual, expected);
        Ok(())
    }
}
