//!
//! Holds functions to get information about stops from the API
//!

use reqwest::Error;
use serde::Deserialize;
use time::{macros::format_description, Time};

use crate::structs::common::GeoLocation;
use crate::structs::{
    stops::{Feature, PartialStop, Schedule, Stop},
    UrlParameter, Usage,
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
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

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
    /// use transit_api_client::structs::{common::GeoLocation, Usage};
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

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
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

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
    /// use transit_api_client::structs::Usage;
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
    /// let stop_schedule = client.stop_schedule(10168, None, None, None, Usage::Normal).await.unwrap();
    /// # });
    /// ```
    pub async fn stop_schedule(
        &self,
        stop: u32,
        start: Option<Time>,
        end: Option<Time>,
        limit: Option<u32>,
        usage: Usage,
    ) -> Result<Schedule, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-schedule")]
            stop_schedule: Schedule,
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}/schedule.json?api-key={api_key}{usage}{start}{end}{limit}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
                start = match start {
                    Some(t) => format!(
                        "&start={}",
                        t.format(format_description!("[hour]:[minute]:[second]"))
                            .unwrap()
                    ),
                    None => "".to_string(),
                },
                end = match end {
                    Some(t) => format!(
                        "&end={}",
                        t.format(format_description!("[hour]:[minute]:[second]"))
                            .unwrap()
                    ),
                    None => "".to_string(),
                },
                limit = match limit {
                    Some(l) => format!("&max-results-per-route={l}"),
                    None => "".to_string(),
                },
            ))
            .send()
            .await?;
        log::debug!("Got response for stop (schedule; #{stop}): {:?}", &response);
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

        Ok(out.stop_schedule)
    }

    // This function will be deprecated, in favour of a filter vector in stop_schedule
    #[allow(missing_docs)]
    pub async fn stop_schedule_route_filter(
        &self,
        stop: u32,
        routes: Vec<u32>,
        start: Option<Time>,
        end: Option<Time>,
        max_results_per_route: Option<u32>,
        usage: Usage,
    ) -> Result<Schedule, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-schedule")]
            stop_schedule: Schedule,
        }

        let mut routes_param = "&route=".to_string();
        for (i, route) in routes.iter().enumerate() {
            routes_param.push_str(&route.to_string());
            if i + 1 < routes.len() {
                routes_param.push(',');
            }
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}/schedule.json?api-key={api_key}{usage}{start}{end}{limit}{routes_param}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
                start = match start {
                    Some(t) => format!("&start={}", t.format(format_description!("[hour]:[minute]:[second]")).unwrap()),
                    None => "".to_string(),
                },
                end = match end {
                    Some(t) => format!("&end={}", t.format(format_description!("[hour]:[minute]:[second]")).unwrap()),
                    None => "".to_string(),
                },
                limit = match max_results_per_route {
                    Some(l) => format!("&max-results-per-route={l}"),
                    None => "".to_string(),
                },
            ))
            .send()
            .await?;
        log::debug!(
            "Got response for stop (schedule with route filter; #{stop}): {:?}",
            &response
        );
        let out: Response = response.json().await?;
        log::debug!("Response body: {out:?}");

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
        let out: Response = response.json().await?;
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
    /// use transit_api_client::structs::{Usage, stops::Stop};
    ///
    /// # tokio_test::block_on(async {
    /// let client = transit_api_client::TransitClient::new("<YOUR_API_TOKEN>".to_string());
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
    use crate::structs::common::StreetLeg;
    use crate::structs::stops::{Distances, PartialStop, PartialStopIconStyle};
    use crate::structs::{
        common::{GeoLocation, Street, StreetType},
        stops::{Direction, Feature, Side, Stop},
        Usage,
    };

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
                street_type: Some(StreetType::Avenue),
                leg: None,
            },
            cross_street: Street {
                key: 681,
                name: "Cauchon Street".to_string(),
                street_type: Some(StreetType::Street),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.88099,
                longitude: -97.14116,
            },
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
                street_type: Some(StreetType::Loop),
                leg: None,
            },
            cross_street: Street {
                key: 3465,
                name: "Stafford Street".to_string(),
                street_type: Some(StreetType::Street),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.85741,
                longitude: -97.15236,
            },
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
                    street_type: Some(StreetType::Street),
                    leg: None,
                },
                cross_street: Street {
                    key: 2871,
                    name: "Pioneer Avenue".to_string(),
                    street_type: Some(StreetType::Avenue),
                    leg: None,
                },
                centre: GeoLocation {
                    latitude: 49.89491,
                    longitude: -97.1379,
                },
                distances: Some(Distances {
                    direct: 12.28,
                    walking: 16.31,
                }),
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
                    street_type: Some(StreetType::Avenue),
                    leg: None,
                },
                cross_street: Street {
                    key: 2265,
                    name: "Main Street".to_string(),
                    street_type: Some(StreetType::Street),
                    leg: None,
                },
                centre: GeoLocation {
                    latitude: 49.89452,
                    longitude: -97.13759,
                },
                distances: Some(Distances {
                    direct: 60.92,
                    walking: 102.52,
                }),
            },
        ];

        log::info!("actual={:?}, expected:{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn stop_schedule() {
        let client = crate::testing_client();
        let actual = client
            .stop_schedule(10064, None, None, Some(3), Usage::Normal)
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
                street_type: Some(StreetType::Street),
            },
            cross_street: Street {
                key: 1486,
                name: "Glasgow Avenue".to_string(),
                leg: None,
                street_type: Some(StreetType::Avenue),
            },
            centre: GeoLocation {
                latitude: 49.86912,
                longitude: -97.1375,
            },
        };

        log::info!("actual={:?}", &actual);
        assert_eq!(actual.stop, expected_stop);
        // Can only test length here, as schedule changes live. This still tests the deserialization
        assert_eq!(actual.route_schedules.len(), 1);
        assert_eq!(actual.route_schedules[0].scheduled_stops.len(), 3);
    }

    #[tokio::test]
    async fn stop_schedule_route_filter() {
        let client = crate::testing_client();
        let actual = client
            .stop_schedule_route_filter(10185, vec![18, 60], None, None, Some(3), Usage::Normal)
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
                street_type: Some(StreetType::Street),
            },
            cross_street: Street {
                key: 3781,
                name: "Wardlaw Avenue".to_string(),
                leg: None,
                street_type: Some(StreetType::Avenue),
            },
            centre: GeoLocation {
                latitude: 49.87699,
                longitude: -97.14414,
            },
        };
        log::info!("actual={:?}", &actual);
        assert_eq!(actual.stop, expected_stop);
        // Can only test length here, as schedule changes live. This still tests the deserialization
        assert_eq!(actual.route_schedules.len(), 2);
        assert_eq!(actual.route_schedules[0].scheduled_stops.len(), 3);
        assert_eq!(actual.route_schedules[1].scheduled_stops.len(), 3);
    }

    #[tokio::test]
    async fn get_all_stops() {
        let client = crate::testing_client();
        client.get_all_stops().await.unwrap();
    }

    #[tokio::test]
    async fn try_to_full_stop() -> Result<(), reqwest::Error> {
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
                street_type: Some(StreetType::Avenue),
                leg: Some(StreetLeg::East),
            },
            cross_street: Street {
                key: 873,
                name: "Corliss Crescent".to_string(),
                street_type: Some(StreetType::Crescent),
                leg: None,
            },
            centre: GeoLocation {
                latitude: 49.90404,
                longitude: -96.96857,
            },
        };
        let actual = partial_stop.try_to_full_stop(&client).await?;
        log::info!("actual={:?},expected={:?}", &actual, &expected);
        assert_eq!(actual, expected);
        Ok(())
    }
}
