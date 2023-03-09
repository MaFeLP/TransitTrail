use crate::structs::{StopFeature, UrlParameter, Usage};
use reqwest::Error;
use serde::Deserialize;

impl crate::TransitClient {
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
}

