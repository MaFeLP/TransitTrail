//!
//! Holds functions to get information about stops from the API
//!

use reqwest::Error;
use serde::Deserialize;
use time::{macros::format_description, Time};

use crate::structs::{
    stops::{Feature, Schedule, Stop},
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
}

#[cfg(test)]
mod test {
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
}
