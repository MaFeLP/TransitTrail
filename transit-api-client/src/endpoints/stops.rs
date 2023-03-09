use crate::structs::{Stop, StopFeature, UrlParameter, Usage};
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
        let mut actual = rt
            .block_on(client.stop_info(10168, Usage::Normal))
            .unwrap();
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
            centre: GeographicLocation { geographic: GeoLocation { latitude: "49.88099".to_string(), longitude: "-97.14116".to_string() } },
        };
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);

        actual = rt
            .block_on(client.stop_info(10087, Usage::Normal))
            .unwrap();
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
            centre: GeographicLocation { geographic: GeoLocation { latitude: "49.85741".to_string(), longitude: "-97.15236".to_string() } },
        };
        //dbg!("{:?},{:?}", &actual, &expected);
        assert_eq!(actual, expected);
    }
}

