use crate::structs::{Stop, StopFeature, StopSchedule, UrlParameter, Usage, TIME_FORMAT};
use chrono::NaiveDateTime;
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
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
        let out: Response = response.json().await?;
        Ok(out.stop)
    }

    pub async fn stop_features(&self, stop: u32, usage: Usage) -> Result<Vec<StopFeature>, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-features")]
            stop_features: Vec<StopFeature>,
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
        let out: Response = response.json().await?;
        Ok(out.stop_features)
    }

    pub async fn stop_schedule(
        &self,
        stop: u32,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
        limit: Option<u32>,
        usage: Usage,
    ) -> Result<StopSchedule, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-schedule")]
            stop_schedule: StopSchedule,
        }

        let response = self
            .client
            .get(format!(
                "{base}/stops/{stop}/schedule.json?api-key={api_key}{usage}{start}{end}{limit}",
                base = self.base_url,
                api_key = self.api_key,
                usage = UrlParameter::from(usage),
                start = match start {
                    Some(t) => format!("&start={}", t.format(TIME_FORMAT).to_string()),
                    None => "".to_string(),
                },
                end = match end {
                    Some(t) => format!("&end={}", t.format(TIME_FORMAT).to_string()),
                    None => "".to_string(),
                },
                limit = match limit {
                    Some(l) => format!("&max-results-per-route={l}"),
                    None => "".to_string(),
                },
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        Ok(out.stop_schedule)
    }

    pub async fn stop_schedule_route_filter(
        &self,
        stop: u32,
        routes: Vec<u32>,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
        max_results_per_route: Option<u32>,
        usage: Usage,
    ) -> Result<StopSchedule, Error> {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "stop-schedule")]
            stop_schedule: StopSchedule,
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
                    Some(t) => format!("&start={}", t.format(TIME_FORMAT).to_string()),
                    None => "".to_string(),
                },
                end = match end {
                    Some(t) => format!("&end={}", t.format(TIME_FORMAT).to_string()),
                    None => "".to_string(),
                },
                limit = match max_results_per_route {
                    Some(l) => format!("&max-results-per-route={l}"),
                    None => "".to_string(),
                },
            ))
            .send()
            .await?;
        let out: Response = response.json().await?;
        Ok(out.stop_schedule)
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    use crate::structs::*;

    #[test]
    fn stop_features() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client =
            crate::TransitClient::new(std::env::var("WPG_TRANSIT_API_KEY").unwrap_or_default());
        let mut actual = rt
            .block_on(client.stop_features(10064, Usage::Normal))
            .unwrap();
        let mut expected = vec![
            StopFeature {
                name: "Bench".to_string(),
                count: 1,
            },
            StopFeature {
                name: "Unheated Shelter".to_string(),
                count: 1,
            },
        ];
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);

        actual = rt
            .block_on(client.stop_features(10172, Usage::Normal))
            .unwrap();
        expected = vec![
            StopFeature {
                name: "BUSwatch Electronic Sign".to_string(),
                count: 1,
            },
            StopFeature {
                name: "Bench".to_string(),
                count: 2,
            },
            StopFeature {
                name: "Unheated Shelter".to_string(),
                count: 1,
            },
        ];
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn stop_info() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client =
            crate::TransitClient::new(std::env::var("WPG_TRANSIT_API_KEY").unwrap_or_default());
        let mut actual = rt.block_on(client.stop_info(10168, Usage::Normal)).unwrap();
        let mut expected = Stop {
            key: 10168,
            name: "Westbound River at Cauchon".to_string(),
            number: 10168,
            direction: StopDirection::Westbound,
            side: StopSide::DirectOpposite,
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
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);

        actual = rt.block_on(client.stop_info(10087, Usage::Normal)).unwrap();
        expected = Stop {
            key: 10087,
            name: "Northbound Stafford at Stafford Loop".to_string(),
            number: 10087,
            direction: StopDirection::Northbound,
            side: StopSide::NA,
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
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn stop_schedule() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client =
            crate::TransitClient::new(std::env::var("WPG_TRANSIT_API_KEY").unwrap_or_default());
        let actual = rt
            .block_on(client.stop_schedule(10064, None, None, Some(3), Usage::Normal))
            .unwrap();

        let expected_stop = Stop {
            key: 10064,
            name: "Northbound Osborne at Glasgow".to_string(),
            number: 10064,
            direction: StopDirection::Northbound,
            side: StopSide::Nearside,
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
        assert_eq!(actual.stop, expected_stop);
        // Can only test length here, as schedule changes live. This still tests the deserialization
        assert_eq!(actual.route_schedules.len(), 1);
        assert_eq!(actual.route_schedules[0].scheduled_stops.len(), 3);
    }

    #[test]
    fn stop_schedule_route_filter() {
        // Read .env file for environment variables
        dotenv::dotenv().unwrap();
        // Create a runtime, to run async functions
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client =
            crate::TransitClient::new(std::env::var("WPG_TRANSIT_API_KEY").unwrap_or_default());
        let actual = rt
            .block_on(client.stop_schedule_route_filter(
                10185,
                vec![18, 60],
                None,
                None,
                Some(3),
                Usage::Normal,
            ))
            .unwrap();

        let expected_stop = Stop {
            key: 10185,
            name: "Southbound Osborne at Wardlaw".to_string(),
            number: 10185,
            direction: StopDirection::Southbound,
            side: StopSide::Nearside,
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
        assert_eq!(actual.stop, expected_stop);
        // Can only test length here, as schedule changes live. This still tests the deserialization
        assert_eq!(actual.route_schedules.len(), 2);
        assert_eq!(actual.route_schedules[0].scheduled_stops.len(), 3);
        assert_eq!(actual.route_schedules[1].scheduled_stops.len(), 3);
    }
}
